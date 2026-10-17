mod history;
mod picker;
mod pv;
mod quiescence;
mod stack;
mod time;
mod transposition;
mod worker;

pub use time::{SearchLimit, TimeManagement};
pub use transposition::TranspositionTable;

use std::{
    iter,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use crate::{
    chess::{Move, Position},
    evaluation::{INF, MATE},
    search::{
        picker::MovePicker,
        pv::{PrincipalVariation, pvs},
        quiescence::quiescence,
        worker::Worker,
    },
};

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

pub struct SearchResult {
    #[cfg_attr(not(feature = "datagen"), allow(dead_code))]
    pub score: i32,
    pub best_move: Option<Move>,
    pub nodes: u64,
    pub time: Duration,
}

impl SearchResult {
    pub fn nps(&self) -> u64 {
        time::nps(self.nodes, self.time)
    }
}

pub fn go<const REPORT: bool>(
    pos: &Position,
    time: &TimeManagement,
    tt: &TranspositionTable,
    abort: &AtomicBool,
) -> SearchResult {
    tt.increase_age();
    abort.store(false, Ordering::Relaxed);

    let mut main = Worker::new(pos.clone(), tt.view(), time.clone(), abort, true);

    main.pos.reset_height();

    let depth = if let SearchLimit::Depth(depth) = time.limit() {
        depth
    } else {
        MAX_DEPTH as i32
    };

    iterative_deepening::<REPORT>(&mut main, depth);

    let (mut score, mut best_move) = main.result();

    // If no iteration finished, we fall back to the first legal move
    if best_move.is_none() {
        let mut picker = MovePicker::new(None);

        score = -INF;
        best_move = iter::from_fn(|| picker.next(&main)).find(|&mov| main.pos.legal(mov));
    }

    SearchResult {
        score,
        best_move,
        nodes: main.nodes(),
        time: main.elapsed(),
    }
}

fn iterative_deepening<const REPORT: bool>(worker: &mut Worker, max_depth: i32) {
    let mut pv = PrincipalVariation::EMPTY;

    for depth in 1..=max_depth.min(MAX_PLY - 1) {
        let score = pvs::<Root>(worker, &mut pv, -INF, INF, depth);

        // We only consider finished iterations
        if worker.abort() {
            break;
        }

        worker.update_pv(&pv);

        if REPORT {
            worker.report(depth);
        }

        // We can skip further search if we found a forced mate
        if score.abs() > MATE {
            break;
        }

        // We don't start an iteration, which we most likely can't finish
        if worker.main() && worker.check_soft() {
            break;
        }
    }
}
