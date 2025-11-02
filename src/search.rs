mod limit;

use crate::{
    evaluation::{DRAW, INF, evaluate},
    thread::ThreadData,
};

pub use limit::SearchLimit;
use types::MoveList;

pub fn go(thread: &mut ThreadData) {
    iterative_deepening(thread, thread.limits.depth);
}

fn iterative_deepening(thread: &mut ThreadData, max_depth: u16) {
    for depth in 1..=max_depth {
        let score = alpha_beta(thread, -INF, INF, depth, 0);

        if thread.abort() {
            break;
        }

        println!(
            "info depth {} score cp {} pv {}",
            depth,
            score,
            thread.best.unwrap()
        );

        thread.score = score;
    }
}

fn alpha_beta(thread: &mut ThreadData, mut alpha: i32, beta: i32, depth: u16, ply: i32) -> i32 {
    if depth <= 0 {
        return evaluate(&thread.pos);
    }

    let root = ply == 0;

    // We only check the time constraint on the main search thread
    if thread.main() {
        thread.check_limits();
    }

    if !root && (thread.abort() || thread.pos.draw() || thread.pos.repetition()) {
        return DRAW;
    }

    let in_check = thread.pos.check();

    let mut best_score = -INF;
    let mut best_move = None;
    let mut legal = 0;

    let mut moves = MoveList::with_capacity(40);
    thread.pos.generate::<true>(&mut moves);

    for mov in moves {
        if !thread.pos.legal(mov) {
            continue;
        }

        legal += 1;

        thread.pos.make_move(mov);
        let score = -alpha_beta(thread, -beta, -alpha, depth - 1, ply + 1);
        thread.pos.unmake_move(mov);

        if score <= best_score {
            continue;
        }

        best_score = score;

        if score <= alpha {
            continue;
        }

        best_move = Some(mov);

        if score >= beta {
            break;
        }

        alpha = score;
    }

    thread.nodes += legal;

    if root {
        thread.best = best_move;
    }

    if legal == 0 {
        return i32::from(in_check) * (ply - INF);
    }

    best_score
}
