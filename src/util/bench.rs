use std::{sync::atomic::AtomicBool, time::Duration};

use crate::{
    FEN,
    chess::Position,
    error::{Error, Result},
    search::{SearchLimit, SearchResult, TimeManagement, TranspositionTable, go},
};

const DEPTH: i32 = 10;

pub fn bench(tt: &mut TranspositionTable, tokens: &[&str]) -> Result<()> {
    let depth = tokens.first().and_then(|v| v.parse().ok()).unwrap_or(DEPTH);
    let abort = AtomicBool::new(false);

    let mut total = SearchResult {
        score: 0,
        best_move: None,
        nodes: 0,
        time: Duration::ZERO,
    };

    for (i, fen) in FEN.iter().enumerate() {
        let pos = Position::from_fen(fen)?;
        let limits = TimeManagement::new(SearchLimit::Depth(depth), 0);

        println!("Position: {}/{} ({})", i + 1, FEN.len(), fen);

        tt.clear();

        let result = go::<true>(&pos, &limits, tt, &abort);

        match result.best_move {
            Some(mov) => println!("bestmove {}\n", mov),
            _ => return Err(Error::Internal("No move found".to_string())),
        };

        total.nodes += result.nodes;
        total.time += result.time;
    }

    println!(
        "Time: {} ms\nNodes: {}\nNodes/Seconds: {}",
        total.time.as_millis(),
        total.nodes,
        total.nps()
    );

    Ok(())
}
