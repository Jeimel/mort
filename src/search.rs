mod history;
mod picker;
mod pv;
mod quiescence;
mod time;
mod transposition;
mod worker;

pub use time::{SearchLimit, TimeManagement};
pub use transposition::TranspositionTable;

use std::{iter, sync::atomic::AtomicBool};

use crate::{
    chess::Position,
    evaluation::{INF, MATE},
    search::{
        picker::MovePicker,
        pv::{PrincipalVariation, pvs},
        quiescence::quiescence,
        worker::Worker,
    },
};

use types::Move;

pub const MAX_DEPTH: usize = 127;
pub const MAX_PLY: i32 = 127;

pub trait NodeType {
    const PV: bool;
    const ROOT: bool;
}

pub struct Root {}

impl NodeType for Root {
    const PV: bool = true;
    const ROOT: bool = true;
}

pub struct PV {}

impl NodeType for PV {
    const PV: bool = true;
    const ROOT: bool = false;
}

pub struct NonPV {}

impl NodeType for NonPV {
    const PV: bool = false;
    const ROOT: bool = false;
}

pub fn go(
    pos: &Position,
    time: &TimeManagement,
    tt: &TranspositionTable,
    abort: &AtomicBool,
) -> (i32, Option<Move>) {
    let mut main = Worker::new(pos.clone(), tt.view(), time.clone(), &abort, true);

    main.pos.reset_height();

    let depth = if let SearchLimit::Depth(depth) = time.limit() {
        depth
    } else {
        MAX_DEPTH as i32
    };

    iterative_deepening(&mut main, depth);

    let (score, mov) = main.result();

    if mov.is_some() {
        return (score, mov);
    }

    let mut picker = MovePicker::new(None);
    let mov = iter::from_fn(|| picker.next(&main)).find(|&mov| main.pos.legal(mov));

    (-INF, mov)
}

fn iterative_deepening(worker: &mut Worker, max_depth: i32) {
    let mut pv = PrincipalVariation::EMPTY;

    for depth in 1..=max_depth.min(MAX_PLY) {
        let score = pvs::<Root>(worker, &mut pv, -INF, INF, depth);

        // We only consider finished iterations
        if worker.abort() {
            break;
        }

        worker.update_pv(&pv);
        worker.report(depth);

        // We can skip further search if we found a forced mate
        if score.abs() > MATE {
            break;
        }
    }
}
