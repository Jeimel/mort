mod data;
mod limit;
mod picker;
mod pv;
mod quiescence;
mod thread;

pub use limit::SearchLimit;

use std::sync::atomic::AtomicBool;

use crate::{
    chess::Position,
    evaluation::{DRAW, INF},
    search::{
        picker::MovePicker, pv::PrincipalVariation, quiescence::quiescence, thread::ThreadData,
    },
};

use types::Move;

pub const MAX_DEPTH: usize = 128;

pub fn go(pos: &Position, limits: &SearchLimit, abort: &AtomicBool) -> (i32, Option<Move>) {
    let mut main = ThreadData::new(pos.clone(), limits.clone(), &abort, true);

    iterative_deepening(&mut main, limits.depth);

    let score = main.data.pv.score;
    let mov = main.data.pv.line.first().copied();

    if mov.is_some() {
        return (score, mov);
    }

    let mut picker = MovePicker::new();
    let mov = std::iter::from_fn(|| picker.next(&main.pos)).find(|&mov| main.pos.legal(mov));

    (-INF, mov)
}

fn iterative_deepening(thread: &mut ThreadData, max_depth: u16) {
    let mut pv = PrincipalVariation::EMPTY;

    for depth in 1..=max_depth {
        alpha_beta(thread, &mut pv, -INF, INF, depth as i16, 0);

        // Did we finish the last iteration?
        if thread.abort() {
            break;
        }

        thread.data.pv = pv.clone();
        thread.data.completed = depth;

        println!(
            "info score cp {} depth {} nodes {} pv {}",
            pv.score, depth, thread.data.nodes, pv,
        );
    }
}

fn alpha_beta(
    thread: &mut ThreadData,
    pv: &mut PrincipalVariation,
    mut alpha: i32,
    beta: i32,
    depth: i16,
    ply: i32,
) -> i32 {
    if depth <= 0 {
        return quiescence(thread, alpha, beta, ply);
    }

    // We only check the constraints on the main search thread
    if thread.main() {
        thread.check_limits();
    }

    let root = ply == 0;

    if !root && (thread.abort() || thread.pos.draw() || thread.pos.repetition()) {
        return DRAW;
    }

    let check = thread.pos.check();

    let mut best_score = -INF;
    let mut local_pv = PrincipalVariation::EMPTY;

    let mut picker = MovePicker::new();
    let mut legal = 0;

    while let Some(mov) = picker.next(&thread.pos) {
        if !thread.pos.legal(mov) {
            continue;
        }

        legal += 1;

        thread.pos.make_move(mov);
        let score = -alpha_beta(thread, &mut local_pv, -beta, -alpha, depth - 1, ply + 1);
        thread.pos.unmake_move(mov);

        if score <= best_score {
            continue;
        }

        best_score = score;

        if score <= alpha {
            continue;
        }

        pv.collect(mov, score, &local_pv);

        if score >= beta {
            break;
        }

        alpha = score;
    }

    thread.data.nodes += legal;

    if legal == 0 {
        return i32::from(check) * (ply - INF);
    }

    best_score
}
