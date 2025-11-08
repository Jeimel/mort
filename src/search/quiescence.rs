use crate::{
    evaluation::{DRAW, INF, evaluate},
    search::{picker::MovePicker, thread::ThreadData},
};

pub fn quiescence(thread: &mut ThreadData, mut alpha: i32, beta: i32, ply: i32) -> i32 {
    // Again, we only check the constraints on the main search thread
    if thread.main() {
        thread.check_limits();
    }

    let check = thread.pos.check();

    if thread.abort() || thread.pos.draw() || thread.pos.repetition() {
        return if !check { evaluate(&thread.pos) } else { DRAW };
    }

    // We calculate a static evaluation (stand-pat) as lower bound.
    // We skip stand-pat if we are in check, as we are searching every move in that case
    let mut best_score = if check { -INF } else { evaluate(&thread.pos) };

    // Do we already exceed our higher bound?
    if best_score >= beta {
        return best_score;
    }

    // If the evaluation score is bigger than alpha, we can improve our position
    if best_score > alpha {
        alpha = best_score;
    }

    let mut picker = MovePicker::new();
    let mut legal = 0;

    // If we are in check, we have to resolve the threat so the position is not quiet
    picker.set_quiet(check);

    while let Some(mov) = picker.next(&thread.pos) {
        if !thread.pos.legal(mov) {
            continue;
        }

        legal += 1;

        thread.pos.make_move(mov);
        let score = -quiescence(thread, -beta, -alpha, ply + 1);
        thread.pos.unmake_move(mov);

        if score <= best_score {
            continue;
        }

        best_score = score;

        if score <= alpha {
            continue;
        }

        if score >= beta {
            break;
        }

        alpha = score;
    }

    thread.data.nodes += legal;

    if legal == 0 && check {
        return i32::from(check) * (ply - INF);
    }

    best_score
}
