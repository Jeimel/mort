use std::{
    io, process,
    slice::Iter,
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

use types::Color;

use crate::{
    Position,
    error::Error,
    evaluation::evaluate,
    search::{self, SearchLimit},
    syntax_error,
    thread::ThreadData,
};

const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub type UciError = String;

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
            "position" => {
                handle_position(&mut pos, commands).unwrap_or_else(|err| println!("{}", err))
            }
            "ucinewgame" => pos = Position::from_fen(START_POS).unwrap(),
            "isready" => println!("readyok"),
            "go" => {
                handle_go(commands, &mut pos, &mut buffer).unwrap_or_else(|err| println!("{}", err))
            }
            "d" => println!("{}", pos),
            "eval" => println!("score cp {}", evaluate(&pos)),
            _ => println!("Unknown command: {}", command),
        };
    }
}

fn identify() {
    println!("id name mort");
    println!("id author jeimel");
    println!("uciok");
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
        match pos
            .gen_moves()
            .iter()
            .find(|mov| &format!("{}", mov) == *str)
        {
            Some(mov) => pos.make_move(*mov),
            None => return Err(Error::Uci(syntax_error!("valid move", str))),
        };
    }

    Ok(())
}

fn handle_go(
    commands: Vec<&str>,
    pos: &Position,
    buffer: &mut Option<String>,
) -> Result<(), Error> {
    let abort = AtomicBool::new(false);

    let (limits, depth) = handle_limits(&mut commands.iter(), pos.stm())?;
    let mut main = ThreadData::new(&abort, pos.clone(), true, limits);

    thread::scope(|s| {
        s.spawn(|| {
            search::go(&mut main, depth);

            println!("bestmove {}", main.best.unwrap());
        });

        *buffer = handle_search_input(&abort);
    });

    Ok(())
}

fn handle_limits(commands: &mut Iter<&str>, stm: Color) -> Result<(SearchLimit, u16), Error> {
    let mut limits = SearchLimit::MAX;
    let mut depth = 64;
    let mut left = [u128::MAX, u128::MAX];
    let mut increment = [0, 0];

    while let Some(key) = commands.next() {
        let mut skip = true;

        match *key {
            "infinite" => depth = u16::MAX,
            "depth" | "nodes" | "wtime" | "btime" | "winc" | "binc" => skip = false,
            _ => continue,
        };

        if skip {
            continue;
        }

        let value = match commands.next() {
            Some(value) => value,
            None => return Err(Error::Uci(format!("Missing value for {}", key))),
        };

        let value: u64 = match value.parse() {
            Ok(value) => value,
            Err(_) => return Err(Error::Uci(format!("Invalid value for {}", key))),
        };

        match *key {
            "depth" => depth = value as u16,
            "nodes" => limits.nodes = value,
            "wtime" => left[Color::White] = value as u128,
            "btime" => left[Color::Black] = value as u128,
            "winc" => increment[Color::White] = value as u128,
            "binc" => increment[Color::Black] = value as u128,
            _ => unreachable!(),
        };
    }

    limits.time = left[stm] / 20 + increment[stm] / 2;

    Ok((limits, depth))
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
