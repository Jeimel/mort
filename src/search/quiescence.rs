use crate::{
    evaluation::{DRAW, INF, evaluate, mated_in},
    search::{MAX_DEPTH, picker::MovePicker, worker::Worker},
};

pub fn quiescence(worker: &mut Worker, mut alpha: i32, beta: i32) -> i32 {
    debug_assert!(-INF <= alpha && alpha < beta && beta <= INF);
    debug_assert!(worker.pos.height() < MAX_DEPTH);

    // Again, we only check the constraints on the main search thread
    if worker.main() {
        worker.check_limits();
    }

    let check = worker.pos.check();

    if worker.abort() || worker.pos.draw() {
        return if !check { evaluate(&worker.pos) } else { DRAW };
    }

    // We calculate a static evaluation (stand-pat) as lower bound.
    // We skip stand-pat if we are in check, as we are searching every move in that case
    let mut best_score = if check { -INF } else { evaluate(&worker.pos) };

    // Do we already exceed our higher bound?
    if best_score >= beta {
        return best_score;
    }

    // If the evaluation score is bigger than alpha, we can improve our position
    if best_score > alpha {
        alpha = best_score;
    }

    let mut picker = MovePicker::new(None);
    let mut legal = 0;

    // If we are in check, we have to resolve the threat so the position is not quiet
    picker.set_quiet(check);

    while let Some(mov) = picker.next(&worker.pos) {
        if !worker.pos.legal(mov) {
            continue;
        }

        legal += 1;

        worker.pos.make_move(mov);
        let score = -quiescence(worker, -beta, -alpha);
        worker.pos.unmake_move(mov);

        debug_assert!(-INF < score && score < INF);

        best_score = best_score.max(score);

        if score <= alpha {
            continue;
        }

        if score >= beta {
            break;
        }

        alpha = score;
    }

    worker.update_nodes(legal);

    if legal == 0 && check {
        return mated_in(worker.pos.height());
    }

    debug_assert!(-INF < best_score && best_score < INF);

    best_score
}
