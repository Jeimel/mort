use std::{
    io, process,
    slice::Iter,
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

use types::Color;

use crate::{
    chess::{All, MoveList, Position, perft},
    error::Error,
    evaluation::evaluate,
    ok_or,
    search::{SearchLimit, TranspositionTable, go},
    syntax_error, unwrap_or,
};

const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

mod default {
    pub const TT_SIZE: usize = 16;
    pub const OVERHEAD: u16 = 10;
}

fn read() -> Option<String> {
    let mut input = String::new();

    let bytes_read = match io::stdin().read_line(&mut input) {
        Ok(bytes_read) => bytes_read,
        Err(_) => return None,
    };

    // We received EOF
    if bytes_read == 0 {
        process::exit(0);
    }

    Some(input)
}

pub fn run() {
    let mut pos = Position::from_fen(START_POS).unwrap();
    let mut tt = TranspositionTable::new();
    let mut overhead = default::OVERHEAD;

    tt.resize(default::TT_SIZE);

    let mut buffer = None;

    loop {
        let input = match buffer.clone().or_else(read) {
            Some(input) => input,
            None => continue,
        };

        buffer = None;

        let commands: Vec<_> = input.split_ascii_whitespace().collect();
        let command = match commands.first() {
            Some(command) => *command,
            None => continue,
        };

        match command {
            "quit" => process::exit(0),
            "uci" => identify(),
            "setoption" => unwrap_or!(handle_option(commands, &mut tt, &mut overhead)),
            "position" => unwrap_or!(handle_position(&mut pos, commands)),
            "ucinewgame" => {
                pos = Position::from_fen(START_POS).unwrap();
                tt.clear();
            }
            "isready" => println!("readyok"),
            "go" => unwrap_or!(handle_go(&pos, &tt, overhead, commands, &mut buffer)),
            "d" => println!("{}", pos),
            "eval" => println!("score cp {}", evaluate(&pos)),
            _ => eprintln!("Unknown command: {}", command),
        };
    }
}

fn identify() {
    println!(concat!(
        "id name mort",
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

fn handle_option(
    commands: Vec<&str>,
    tt: &mut TranspositionTable,
    overhead: &mut u16,
) -> Result<(), Error> {
    match commands[1..] {
        ["name", "Hash", "value", x] => tt.resize(ok_or!(x.parse().ok(), "integer", x)),
        ["name", "Clear", "Hash"] => tt.clear(),
        ["name", "Overhead", "value", x] => *overhead = ok_or!(x.parse().ok(), "integer", x),
        #[rustfmt::skip]
        _ => return Err(Error::Uci(syntax_error!("name <id> value <x>", commands[1..].join(" ")))),
    };

    Ok(())
}

fn handle_position(pos: &mut Position, commands: Vec<&str>) -> Result<(), Error> {
    let mut commands = commands.iter().peekable().skip(1);

    let mut fen = match *commands.next().unwrap() {
        "fen" | "startpos" => String::new(),
        command => return Err(Error::Uci(syntax_error!("[fen|startpos]", command))),
    };

    // We iterate until we are either empty, or we found the "moves" token, which we will consume
    while let Some(command) = commands.next()
        && *command != "moves"
    {
        fen.push_str(&format!("{} ", command));
    }

    let fen = if fen.is_empty() {
        START_POS
    } else {
        fen.as_str()
    };

    *pos = match Position::from_fen(fen) {
        Ok(pos) => pos,
        Err(err) => return Err(err),
    };

    // We already skipped the "moves" token earlier, so we can directly play the moves if any
    while let Some(str) = commands.next() {
        let mut moves = MoveList::new();
        pos.generate::<All>(&mut moves);

        match moves.iter().find(|mov| &format!("{}", mov) == *str) {
            Some(mov) => pos.make_move(mov),
            None => return Err(Error::Uci(syntax_error!("valid move", str))),
        };
    }

    Ok(())
}

fn handle_go(
    pos: &Position,
    tt: &TranspositionTable,
    overhead: u16,
    commands: Vec<&str>,
    buffer: &mut Option<String>,
) -> Result<(), Error> {
    let abort = AtomicBool::new(false);

    let limits = handle_limits(&mut commands.iter(), pos.stm(), overhead)?;

    if limits.perft != 0 {
        perft::<true>(&mut pos.clone(), limits.perft);

        return Ok(());
    }

    thread::scope(|s| {
        s.spawn(|| {
            let (_, mov) = go(&pos, &limits, &tt, &abort);

            match mov {
                Some(mov) => println!("bestmove {}", mov),
                _ => eprintln!("Internal error: No move found"),
            };
        });

        *buffer = handle_search_input(&abort);

        Ok(())
    })
}

fn handle_limits(
    commands: &mut Iter<&str>,
    stm: Color,
    overhead: u16,
) -> Result<SearchLimit, Error> {
    macro_rules! parse {
        (match ($commands:expr, $key:ident) { $($value:literal => $var:expr),* $(,)? }) => {
            match *$key {
                $($value => $var = parse!($key, $commands),)*
                _ => {},
            }
        };

        ($key:expr, $commands:expr) => {{
            let value = match $commands.next() {
                Some(value) => value,
                None => return Err(Error::Uci(format!("Missing value for {}", $key))),
            };

            match value.parse() {
                Ok(value) => value,
                Err(_) => return Err(Error::Uci(format!("Invalid value for {}", $key))),
            }
        }};
    }

    let mut limits = SearchLimit::MAX;
    let mut left = [u128::MAX, u128::MAX];
    let mut increment = [0, 0];

    while let Some(key) = commands.next() {
        if *key == "infinite" {
            continue;
        }

        parse!(match (commands, key) {
            "perft" => limits.perft,
            "depth" => limits.depth,
            "nodes" => limits.nodes,
            "wtime" => left[Color::White],
            "btime" => left[Color::Black],
            "winc" => increment[Color::White],
            "binc" => increment[Color::Black],
        });
    }

    // We want to have a search time greater than zero
    limits.time = (left[stm] / 20 + increment[stm] / 2)
        .saturating_sub(u128::from(overhead))
        .max(1);

    Ok(limits)
}

fn handle_search_input(abort: &AtomicBool) -> Option<String> {
    loop {
        let input = match read() {
            Some(input) => input,
            None => continue,
        };

        let command = match input.split_ascii_whitespace().next() {
            Some(command) => command,
            None => continue,
        };

        match command {
            "quit" => process::exit(0),
            "isready" => println!("readyok"),
            "stop" => abort.store(true, Ordering::Relaxed),
            _ => return Some(input),
        }

        if abort.load(Ordering::Relaxed) {
            break;
        }
    }

    None
}
