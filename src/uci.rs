use std::{
    io, process,
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

use crate::{Position, error::Error, search, syntax_error};

const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub type UciError = String;

fn handle_input<F: FnMut(&str, Vec<&str>)>(abort: &AtomicBool, mut f: F) {
    loop {
        let mut input = String::new();

        let bytes_read = match io::stdin().read_line(&mut input) {
            Ok(bytes_read) => bytes_read,
            Err(_) => continue,
        };

        // We received EOF
        if bytes_read == 0 {
            process::exit(0);
        }

        let commands: Vec<_> = input.split_ascii_whitespace().collect();
        let command = match commands.first() {
            Some(command) => *command,
            None => continue,
        };

        f(command, commands);

        if abort.load(Ordering::Relaxed) {
            break;
        }
    }
}

pub fn run() {
    let mut pos = Position::from_fen(START_POS).unwrap();

    handle_input(&AtomicBool::new(false), |command, commands| match command {
        "quit" => process::exit(0),
        "uci" => identify(),
        "position" => handle_position(&mut pos, commands).unwrap_or_else(|err| println!("{}", err)),
        "ucinewgame" => {}
        "isready" => println!("readyok"),
        "go" => handle_go(commands, &mut pos),
        _ => println!("Unknown command: {}", command),
    });
}

fn identify() {
    println!("id name mort");
    println!("id author jeimel");
    println!("uciok");
}

fn handle_position(pos: &mut Position, commands: Vec<&str>) -> Result<(), Error> {
    let mut commands = commands.iter().skip(1);

    let fen = match *commands.next().unwrap() {
        "fen" => commands.next().unwrap(),
        "startpos" => START_POS,
        command => return Err(Error::Uci(syntax_error!("[ fen | startpos ]", command))),
    };

    *pos = match Position::from_fen(fen) {
        Ok(pos) => pos,
        Err(err) => return Err(err),
    };

    if commands.next().is_none() {
        return Ok(());
    }

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

fn handle_go(_: Vec<&str>, pos: &mut Position) {
    let abort = AtomicBool::new(false);

    thread::scope(|s| {
        s.spawn(|| {
            let (mov, _) = search::go(pos);

            println!("bestmove {}", mov);
        });

        handle_input(&abort, |command, _| match command {
            "quit" => process::exit(0),
            "isready" => println!("readyok"),
            "stop" => abort.store(true, Ordering::Relaxed),
            _ => println!("Unknown command: {}", command),
        });
    });
}
