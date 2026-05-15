use std::sync::atomic::AtomicBool;

use crate::{
    FEN,
    chess::Position,
    search::{SearchLimit, TimeManagement, TranspositionTable, go},
};

const DEPTH: i32 = 9;

pub fn bench(tt: &TranspositionTable, tokens: &[&str]) {
    let depth = tokens.get(1).and_then(|v| v.parse().ok()).unwrap_or(DEPTH);
    let abort = AtomicBool::new(false);

    for (i, fen) in FEN.iter().enumerate() {
        println!("Position: {}/{} ({})", i + 1, FEN.len(), fen);

        let pos = Position::from_fen(fen).unwrap();

        let limits = TimeManagement::new(SearchLimit::Depth(depth), 0);

        let _ = go(&pos, &limits, tt, &abort);
    }
}
