mod info;
mod limit;
mod node;
mod picker;
mod pv;
mod quiescence;
mod thread;
mod transposition;

pub use limit::SearchLimit;
pub use transposition::TranspositionTable;

use std::{iter, sync::atomic::AtomicBool};

use crate::{
    chess::{GenerationType, MoveList, Position},
    evaluation::{DRAW, INF, MATE, mated_in},
    search::{
        node::{NodeType, NonPV, PV, Root},
        picker::MovePicker,
        pv::PrincipalVariation,
        quiescence::quiescence,
        thread::ThreadData,
        transposition::Bound,
    },
};

use types::Move;

pub const MAX_DEPTH: usize = 127;
pub const MAX_PLY: i32 = 127;

pub fn go(
    pos: &Position,
    limits: &SearchLimit,
    tt: &TranspositionTable,
    abort: &AtomicBool,
) -> (i32, Option<Move>) {
    let mut main = ThreadData::new(pos.clone(), limits.clone(), tt.view(), &abort, true);

    main.pos.reset_height();

    iterative_deepening(&mut main, limits.depth);

    let score = main.info.pv.score;
    let mov = main.info.pv.line.first().copied();

    if mov.is_some() {
        return (score, mov);
    }

    let mut picker = MovePicker::new(None);
    let mov = iter::from_fn(|| picker.next(&main.pos)).find(|&mov| main.pos.legal(mov));

    (-INF, mov)
}

fn iterative_deepening(thread: &mut ThreadData, max_depth: u16) {
    let mut pv = PrincipalVariation::EMPTY;

    for depth in 1..=max_depth.min(MAX_DEPTH as u16) {
        alpha_beta::<Root>(thread, &mut pv, -INF, INF, depth as i32);

        // We only consider finished iterations
        if thread.abort() {
            break;
        }

        thread.info.pv = pv.clone();
        thread.info.completed = depth;

        thread.info.report();

        // We can skip further search if we found a forced mate
        if thread.info.pv.score.abs() > MATE {
            break;
        }
    }
}

fn alpha_beta<TYPE: NodeType>(
    thread: &mut ThreadData,
    pv: &mut PrincipalVariation,
    mut alpha: i32,
    beta: i32,
    depth: i32,
) -> i32 {
    debug_assert!(-INF <= alpha && alpha < beta && beta <= INF);
    debug_assert!(thread.pos.height() < MAX_DEPTH);
    debug_assert!(TYPE::PV || (alpha == beta - 1));

    if depth <= 0 {
        return quiescence(thread, alpha, beta);
    }

    debug_assert!(0 < depth && depth < MAX_PLY);

    pv.line.clear();

    // We only check the constraints on the main search thread
    if thread.main() {
        thread.check_limits();
    }

    if !TYPE::ROOT {
        if thread.abort() || thread.pos.draw() {
            return DRAW;
        }
    }

    let zobrist = thread.pos.zobrist();
    let height = thread.pos.height();

    let tt_move = if let Some(entry) = thread.tt.probe(zobrist, height) {
        let illegal = entry
            .mov()
            .is_some_and(|mov| !thread.pos.pseudo_legal(mov) || !thread.pos.legal(mov));

        if !TYPE::PV
            && !illegal
            && entry.depth() >= depth
            && match entry.bound() {
                Bound::Exact => true,
                Bound::Upper => entry.score() <= alpha,
                Bound::Lower => entry.score() >= beta,
            }
        {
            return entry.score();
        }

        if illegal { None } else { entry.mov() }
    } else {
        None
    };

    let mut moves = MoveList::new();
    thread.pos.generate::<{ GenerationType::All }>(&mut moves);

    let check = thread.pos.check();

    let original_alpha = alpha;

    let mut best_score = -INF;
    let mut best_move = None;

    let mut local_pv = PrincipalVariation::EMPTY;
    let mut picker = MovePicker::new(tt_move);

    let mut score = best_score;
    let mut legal = 0;

    while let Some(mov) = picker.next(&thread.pos) {
        if !thread.pos.legal(mov) {
            continue;
        }

        legal += 1;

        thread.pos.make_move(mov);

        if !TYPE::PV || legal > 1 {
            score = -alpha_beta::<NonPV>(thread, &mut local_pv, -(alpha + 1), -alpha, depth - 1);
        }

        if TYPE::PV && (legal == 1 || score > alpha) {
            score = -alpha_beta::<PV>(thread, &mut local_pv, -beta, -alpha, depth - 1);
        }

        thread.pos.unmake_move(mov);

        debug_assert!(-INF < score && score < INF);

        best_score = best_score.max(score);

        if score <= alpha {
            continue;
        }

        best_move = Some(mov);

        if TYPE::PV {
            pv.collect(mov, score, &local_pv);
        }

        if score >= beta {
            break;
        }

        alpha = score;
    }

    thread.info.nodes += legal;

    if legal == 0 {
        return if check { mated_in(height) } else { 0 };
    }

    let bound = if best_score >= beta {
        Bound::Lower
    } else if best_score > original_alpha {
        Bound::Exact
    } else {
        Bound::Upper
    };

    #[rustfmt::skip]
    thread.tt.insert(zobrist, best_move, best_score, depth, bound, height);

    debug_assert!(-INF < best_score && best_score < INF);

    best_score
}
