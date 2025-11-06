mod limit;
mod picker;
mod quiescence;

use crate::{
    evaluation::{DRAW, INF},
    search::{picker::MovePicker, quiescence::quiescence},
    thread::ThreadData,
};

pub use limit::SearchLimit;

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
            "info score cp {} depth {} nodes {}",
            score, depth, thread.nodes,
        );

        thread.score = score;
    }

    if thread.best.is_some() {
        return;
    }

    let mut picker = MovePicker::new();

    // Generate any move if we didn't have enough time to sesarch
    let mov = std::iter::from_fn(|| picker.next(&thread.pos))
        .find(|&mov| thread.pos.legal(mov))
        .expect("Position has no legal moves.");
    thread.best = Some(mov);
}

fn alpha_beta(thread: &mut ThreadData, mut alpha: i32, beta: i32, depth: u16, ply: i32) -> i32 {
    if depth <= 0 {
        return quiescence(thread, alpha, beta, ply);
    }

    let root = ply == 0;

    // We only check the constraints on the main search thread
    if thread.main() {
        thread.check_limits();
    }

    if !root && (thread.abort() || thread.pos.draw() || thread.pos.repetition()) {
        return DRAW;
    }

    let check = thread.pos.check();

    let mut best_score = -INF;
    let mut best_move = None;

    let mut picker = MovePicker::new();
    let mut legal = 0;

    while let Some(mov) = picker.next(&thread.pos) {
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
        return i32::from(check) * (ply - INF);
    }

    best_score
}
