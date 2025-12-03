use std::fmt::Display;

use arrayvec::ArrayVec;
use types::Move;

use crate::{
    chess::{All, MoveList},
    evaluation::{DRAW, INF, mate_in, mated_in},
    search::{
        MAX_DEPTH, MAX_PLY, NodeType, NonPV, PV, picker::MovePicker, quiescence,
        transposition::Bound, worker::Worker,
    },
};

#[derive(Clone)]
pub struct PrincipalVariation {
    line: ArrayVec<Move, MAX_DEPTH>,
    score: i32,
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

    pub fn result(&self) -> (i32, Option<Move>) {
        (self.score, self.line.first().copied())
    }

    pub fn score(&self) -> i32 {
        self.score
    }

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
    worker: &mut Worker,
    pv: &mut PrincipalVariation,
    mut alpha: i32,
    mut beta: i32,
    depth: i32,
) -> i32 {
    debug_assert!(-INF <= alpha && alpha < beta && beta <= INF);
    debug_assert!(worker.pos.height() < MAX_DEPTH);
    debug_assert!(TYPE::PV || (alpha == beta - 1));

    if depth <= 0 {
        return quiescence(worker, alpha, beta);
    }

    debug_assert!(0 < depth && depth < MAX_PLY);

    pv.line.clear();

    // We only check the constraints on the main search thread
    if worker.main() {
        worker.check_limits();
    }

    let height = worker.pos.height();

    if !TYPE::ROOT {
        if worker.abort() || worker.pos.draw() {
            return DRAW;
        }

        alpha = alpha.max(mated_in(height));
        beta = beta.min(mate_in(height + 1));

        if alpha >= beta {
            return alpha;
        }
    }

    let zobrist = worker.pos.zobrist();

    let tt_move = if let Some(entry) = worker.tt.probe(zobrist, height) {
        let illegal = entry
            .mov()
            .is_some_and(|mov| !worker.pos.pseudo_legal(mov) || !worker.pos.legal(mov));

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
    worker.pos.generate::<All>(&mut moves);

    let check = worker.pos.check();

    let original_alpha = alpha;

    let mut best_score = -INF;
    let mut best_move = None;

    let mut local_pv = PrincipalVariation::EMPTY;
    let mut picker = MovePicker::new(tt_move);

    let mut score = best_score;
    let mut legal = 0;

    while let Some(mov) = picker.next(&worker.pos) {
        if !worker.pos.legal(mov) {
            continue;
        }

        legal += 1;

        worker.pos.make_move(mov);

        if !TYPE::PV || legal > 1 {
            score = -pvs::<NonPV>(worker, &mut local_pv, -(alpha + 1), -alpha, depth - 1);
        }

        if TYPE::PV && (legal == 1 || score > alpha) {
            score = -pvs::<PV>(worker, &mut local_pv, -beta, -alpha, depth - 1);
        }

        worker.pos.unmake_move(mov);

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

    worker.update_nodes(legal);

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
    worker.tt.insert(zobrist, best_move, best_score, depth, bound, height);

    debug_assert!(-INF < best_score && best_score < INF);

    best_score
}
