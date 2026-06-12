use std::{
    collections::VecDeque,
    io::stdin,
    process,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, channel},
    },
    thread,
};

use types::Color;

use crate::{
    chess::{All, MoveList, Position},
    error::Error,
    evaluation::evaluate,
    ok_or,
    search::{self, SearchLimit, TimeManagement, TranspositionTable},
    syntax_error, unwrap_or,
    util::{bench, perft},
};

const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

mod default {
    pub const TT_SIZE: usize = 16;
    pub const OVERHEAD: u64 = 10;
}

fn spawn(abort: Arc<AtomicBool>) -> Receiver<String> {
    let (sender, receiver) = channel();

    thread::spawn(move || {
        loop {
            let mut input = String::new();

            // Check for EOF
            if stdin().read_line(&mut input).unwrap_or(0) == 0 {
                let _ = sender.send("quit".to_string());
                break;
            };

            match input.as_str() {
                "isready" => println!("readyok"),
                "stop" => abort.store(true, Ordering::Relaxed),
                "quit" => {
                    let _ = sender.send("quit".to_string());
                    break;
                }
                _ => {
                    let _ = sender.send(input);
                }
            }
        }
    });

    receiver
}

pub fn run(mut buffer: VecDeque<String>) {
    let mut pos = Position::from_fen(START_POS).unwrap();
    let mut tt = TranspositionTable::new();
    let mut overhead = default::OVERHEAD;

    let abort = Arc::new(AtomicBool::new(false));

    tt.resize(default::TT_SIZE);

    let receiver = spawn(abort.clone());

    loop {
        let input = match buffer.pop_front() {
            Some(input) => input,
            None => match receiver.recv() {
                Ok(input) => input,
                Err(_) => continue,
            },
        };

        let commands: Vec<_> = input.split_ascii_whitespace().collect();

        match commands.as_slice() {
            ["quit"] => process::exit(0),
            ["uci"] => uci(),
            ["setoption", tokens @ ..] => unwrap_or!(option(tokens, &mut tt, &mut overhead)),
            ["position", tokens @ ..] => unwrap_or!(position(&mut pos, tokens)),
            ["ucinewgame"] => newgame(&mut pos, &mut tt),
            ["isready"] => println!("readyok"),
            ["go", tokens @ ..] => unwrap_or!(go(&pos, &tt, overhead, &abort, tokens)),
            ["bench", tokens @ ..] => bench(&tt, tokens),
            ["d"] => println!("{}", pos),
            ["eval"] => println!("score cp {}", evaluate(&pos)),
            [] => (),
            _ => eprintln!("Unknown command: {}", input),
        };
    }
}

fn uci() {
    println!(concat!(
        "id name mort-",
        env!("CARGO_PKG_VERSION"),
        '\n',
        "id author jeimel",
        '\n',
        "option name Hash type spin default 16 min 1 max 1024",
        '\n',
        "option name Clear Hash type button",
        '\n',
        "option name Overhead type spin default 10 min 0 max 5000",
        '\n',
        "uciok",
    ));
}

fn option(tokens: &[&str], tt: &mut TranspositionTable, overhead: &mut u64) -> Result<(), Error> {
    match tokens {
        ["name", "Hash", "value", x] => tt.resize(ok_or!(x.parse().ok(), "integer", x)),
        ["name", "Clear", "Hash"] => tt.clear(),
        ["name", "Overhead", "value", x] => *overhead = ok_or!(x.parse().ok(), "integer", x),
        #[rustfmt::skip]
        _ => return Err(Error::Uci(syntax_error!("name <id> value <x>", tokens.join(" ")))),
    };

    Ok(())
}

fn position(pos: &mut Position, tokens: &[&str]) -> Result<(), Error> {
    let mut parts = tokens.splitn(2, |&t| t == "moves");

    let fen = match parts.next() {
        Some(["startpos"]) => START_POS,
        Some(["fen", tokens @ ..]) => &tokens.join(" "),
        #[rustfmt::skip]
        _ => return Err(Error::Uci(syntax_error!("fen or startpos", tokens.join(" ")))),
    };

    *pos = Position::from_fen(fen)?;

    for str in parts.next().unwrap_or_default() {
        let mut moves = MoveList::new();
        pos.generate::<All>(&mut moves);

        match moves.iter().find(|mov| format!("{}", mov) == *str) {
            Some(mov) => pos.make_move(mov),
            None => return Err(Error::Uci(syntax_error!("valid move", str))),
        };
    }

    Ok(())
}

fn newgame(pos: &mut Position, tt: &mut TranspositionTable) {
    *pos = Position::from_fen(START_POS).unwrap();
    tt.clear();
}

fn go(
    pos: &Position,
    tt: &TranspositionTable,
    overhead: u64,
    abort: &Arc<AtomicBool>,
    tokens: &[&str],
) -> Result<(), Error> {
    abort.store(false, Ordering::Relaxed);

    let limit = parse_limits(tokens, pos.stm())?;
    if let SearchLimit::Perft(depth) = limit {
        return Ok(perft::<true>(&mut pos.clone(), depth)).map(|_| ());
    }

    let time = TimeManagement::new(limit, overhead);

    thread::scope(|s| {
        s.spawn(|| {
            let (_, mov) = search::go(&pos, &time, &tt, &abort);

            match mov {
                Some(mov) => println!("bestmove {}", mov),
                _ => panic!("{}", Error::Internal("No move found".to_string())),
            };
        });

        Ok(())
    })
}

fn parse_limits(tokens: &[&str], stm: Color) -> Result<SearchLimit, Error> {
    if let ["infinite"] = tokens {
        return Ok(SearchLimit::Infinite);
    }

    let mut main = None;
    let mut increment = None;

    for chunk in tokens.chunks(2) {
        let [name, value] = *chunk else {
            return Err(Error::Uci(syntax_error!("<name> <value>", chunk.join(" "))));
        };

        let Ok(value) = value.parse::<u64>() else {
            return Err(Error::Uci(syntax_error!("integer", value)));
        };

        match name {
            "perft" => return Ok(SearchLimit::Perft(value as u16)),
            "depth" => return Ok(SearchLimit::Depth(value as i32)),
            "nodes" => return Ok(SearchLimit::Nodes(value)),
            "movetime" => return Ok(SearchLimit::Time(value)),
            "wtime" if stm == Color::White => main = Some(value),
            "btime" if stm == Color::Black => main = Some(value),
            "winc" if stm == Color::White => increment = Some(value),
            "binc" if stm == Color::Black => increment = Some(value),
            _ => (),
        }
    }

    if main.is_none() && increment.is_none() {
        return Ok(SearchLimit::Infinite);
    }

    Ok(SearchLimit::Bonus(
        main.unwrap_or_default(),
        increment.unwrap_or_default(),
    ))
}
