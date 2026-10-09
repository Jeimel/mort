mod pgn;

use std::{
    collections::VecDeque,
    fs::OpenOptions,
    io::{BufWriter, Read, Write},
    sync::atomic::AtomicBool,
};

use crate::{
    chess::Position,
    error::{Error, Result},
    search::{self, SearchLimit, TimeManagement, TranspositionTable},
};

const TT_SIZE: usize = 16;
const NODES: u64 = 5000;
const SKIP: usize = 15;
const MAX_SCORE: i32 = 10000;
const BUFFER_SIZE: usize = 10 << 20;

pub fn run(mut buffer: VecDeque<String>) -> Result<()> {
    buffer.pop_front().unwrap();

    let (input, output) = match buffer.make_contiguous() {
        [input, output] => (std::mem::take(input), std::mem::take(output)),
        _ => return Err(Error::Internal(String::from("Paths missing"))),
    };

    let mut data = String::new();
    {
        let mut file = std::fs::File::open(input).unwrap();
        file.read_to_string(&mut data).unwrap();
    }

    let mut writer = BufWriter::with_capacity(
        BUFFER_SIZE,
        OpenOptions::new().create(true).append(true).open(output)?,
    );

    let mut tt = TranspositionTable::new();
    tt.resize(TT_SIZE);

    let abort = AtomicBool::new(false);

    for pgn in data.split("\n\n").skip(1).step_by(2) {
        let (moves, outcome) = pgn::parse(pgn)?;

        let mut pos = Position::default();
        tt.clear();

        let mut moves = moves.into_iter();

        for mov in moves.by_ref().take(SKIP) {
            pos.make_move(mov);
        }

        for mov in moves {
            pos.make_move(mov);

            if pos.check() {
                continue;
            }

            let time = TimeManagement::new(SearchLimit::Nodes(NODES), 0);
            let (score, best) = search::go::<false>(&pos, &time, &tt, &abort);

            if !(score.abs() <= MAX_SCORE && best.is_some_and(|best| best.flag().normal())) {
                continue;
            }

            let score = [score, -score][pos.stm()];

            writeln!(writer, "{} | {} | {:.1}", pos.fen()?, score, outcome)?;
        }
    }

    writer.flush()?;

    Ok(())
}
