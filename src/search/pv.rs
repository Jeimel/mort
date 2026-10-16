use std::fmt::Display;

use crate::{
    chess::{MAX_MOVES, Move},
    evaluation::{DRAW, INF, MATE, evaluate, mate_in, mated_in},
    search::{
        MAX_DEPTH, MAX_PLY, NodeType, NonPV, PV, picker::MovePicker, quiescence,
        transposition::Bound, worker::Worker,
    },
    util::ArrayVec,
};

#[derive(Clone)]
pub struct PrincipalVariation {
    line: ArrayVec<Move, MAX_DEPTH>,
    score: i32,
}

impl Display for PrincipalVariation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for mov in self.line.iter() {
            write!(f, "{} ", mov)?;
        }

        Ok(())
    }
}

impl PrincipalVariation {
    pub const EMPTY: Self = Self {
        score: DRAW,
        line: ArrayVec::new(),
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
        self.line.extend(&other.line);
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
        worker.check_hard();
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

    let tt_move = if let Some(entry) = worker.tt.probe(zobrist, height)
        && !entry
            .mov()
            .is_some_and(|mov| !worker.pos.pseudo_legal(mov) || !worker.pos.legal(mov))
    {
        if !TYPE::PV
            && entry.depth() >= depth
            && match entry.bound() {
                Bound::Exact => true,
                Bound::Upper => entry.score() <= alpha,
                Bound::Lower => entry.score() >= beta,
            }
        {
            return entry.score();
        }

        entry.mov()
    } else {
        None
    };

    let check = worker.pos.check();
    let eval = if check { -INF } else { evaluate(&worker.pos) };
    let margin = 80 * depth;

    worker.stack[height].eval = eval;

    if !TYPE::PV && !check && depth <= 8 && beta.abs() < MATE - MAX_PLY && eval >= beta + margin {
        return eval;
    }

    let mut local_pv = PrincipalVariation::EMPTY;

    if !TYPE::PV
        && !check
        && depth >= 3
        && eval >= beta
        && worker.stack[height - 1].mov.is_some()
        && worker.pos.non_pawn_material()
    {
        let reduction = 3 + depth / 3;

        worker.stack[height].mov = None;

        worker.pos.make_null_move();
        let score = -pvs::<NonPV>(worker, &mut local_pv, -beta, -beta + 1, depth - reduction);
        worker.pos.unmake_null_move();

        // A mate found after passing our turn is not proven, so we don't return it
        if score >= beta {
            return if score >= MATE { beta } else { score };
        }
    }

    let mut best_score = -INF;
    let mut best_move = None;

    let mut quiets: ArrayVec<Move, MAX_MOVES> = ArrayVec::new();

    let mut picker = MovePicker::new(tt_move);

    let mut score = best_score;
    let mut legal = 0;

    while let Some(mov) = picker.next(worker) {
        if !worker.pos.legal(mov) {
            continue;
        }

        legal += 1;

        worker.stack[height].mov = Some(mov);

        worker.increment_nodes();
        worker.pos.make_move(mov);

        if !TYPE::PV || legal > 1 {
            score = -pvs::<NonPV>(worker, &mut local_pv, -(alpha + 1), -alpha, depth - 1);
        }

        if TYPE::PV && (legal == 1 || score > alpha) {
            score = -pvs::<PV>(worker, &mut local_pv, -beta, -alpha, depth - 1);
        }

        worker.pos.unmake_move(mov);

        if worker.abort() {
            return DRAW;
        }

        debug_assert!(-INF < score && score < INF);

        if !mov.tactical() {
            quiets.push(mov);
        }

        best_score = best_score.max(score);

        if score <= alpha {
            continue;
        }

        best_move = Some(mov);
        alpha = score;

        if TYPE::PV {
            pv.collect(mov, score, &local_pv);
        }

        if alpha < beta {
            continue;
        }

        break;
    }

    if legal == 0 {
        return if check { mated_in(height) } else { 0 };
    }

    if let Some(best_move) = best_move
        && !best_move.tactical()
        && best_score >= beta
    {
        worker.update_quiet_history(best_move, &quiets, depth as i16);
    }

    let bound = if best_score >= beta {
        Bound::Lower
    } else if TYPE::PV && best_move.is_some() {
        Bound::Exact
    } else {
        Bound::Upper
    };

    #[rustfmt::skip]
    worker.tt.insert(zobrist, best_move, best_score, depth, bound, height);

    debug_assert!(-INF < best_score && best_score < INF);

    best_score
}
