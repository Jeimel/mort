use std::fmt::Display;

use arrayvec::ArrayVec;
use types::Move;

use crate::{
    chess::{All, MoveList},
    evaluation::{DRAW, INF, mate_in, mated_in},
    search::{
        MAX_DEPTH, MAX_PLY, NodeType, NonPV, PV, picker::MovePicker, quiescence,
        thread::ThreadData, transposition::Bound,
    },
};

#[derive(Clone)]
pub struct PrincipalVariation {
    pub(crate) line: ArrayVec<Move, MAX_DEPTH>,
    pub(crate) score: i32,
}

impl Display for PrincipalVariation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for mov in &self.line {
            write!(f, "{} ", mov)?;
        }

        Ok(())
    }
}

impl PrincipalVariation {
    pub const EMPTY: Self = Self {
        score: DRAW,
        line: ArrayVec::new_const(),
    };

    pub fn collect(&mut self, mov: Move, score: i32, other: &Self) {
        self.score = score;

        self.line.clear();
        self.line.push(mov);
        self.line
            .try_extend_from_slice(&other.line)
            .expect("PV can't be longer than `MAX_DEPTH`");
    }
}

pub fn pvs<TYPE: NodeType>(
    thread: &mut ThreadData,
    pv: &mut PrincipalVariation,
    mut alpha: i32,
    mut beta: i32,
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

    let height = thread.pos.height();

    if !TYPE::ROOT {
        if thread.abort() || thread.pos.draw() {
            return DRAW;
        }

        alpha = alpha.max(mated_in(height));
        beta = beta.min(mate_in(height + 1));

        if alpha >= beta {
            return alpha;
        }
    }

    let zobrist = thread.pos.zobrist();

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
    thread.pos.generate::<All>(&mut moves);

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
            score = -pvs::<NonPV>(thread, &mut local_pv, -(alpha + 1), -alpha, depth - 1);
        }

        if TYPE::PV && (legal == 1 || score > alpha) {
            score = -pvs::<PV>(thread, &mut local_pv, -beta, -alpha, depth - 1);
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
