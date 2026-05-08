use std::sync::atomic::AtomicBool;

use crate::{
    FEN,
    chess::Position,
    search::{SearchLimit, TranspositionTable, go},
};

const DEPTH: u16 = 9;

pub fn bench(tt: &TranspositionTable, args: Vec<&str>) {
    let depth = args.get(1).and_then(|v| v.parse().ok()).unwrap_or(DEPTH);
    let abort = AtomicBool::new(false);

    for (i, fen) in FEN.iter().enumerate() {
        println!("Position: {}/{} ({})", i + 1, FEN.len(), fen);

        let pos = Position::from_fen(fen).unwrap();

        let mut limits = SearchLimit::MAX;
        limits.depth = depth;

        let _ = go(&pos, &limits, tt, &abort);
    }
}
