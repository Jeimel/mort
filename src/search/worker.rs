use std::{
    fmt::Display,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::{
    chess::{Move, Position},
    search::{
        TimeManagement, history::ButterflyHistory, pv::PrincipalVariation,
        transposition::TranspositionView,
    },
};

struct Info {
    nodes: u64,
    pv: PrincipalVariation,
}

impl Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "score cp {} nodes {} pv {}",
            self.pv.score(),
            self.nodes,
            self.pv
        )
    }
}

impl Info {
    fn new() -> Self {
        Self {
            nodes: 0,
            pv: PrincipalVariation::EMPTY,
        }
    }
}

pub struct Worker<'a> {
    pub(super) pos: Position,
    pub(super) tt: TranspositionView<'a>,
    pub(super) history: ButterflyHistory,
    time: TimeManagement,
    info: Info,
    abort: &'a AtomicBool,
    main: bool,
}

impl<'a> Worker<'a> {
    pub fn new(
        pos: Position,
        tt: TranspositionView<'a>,
        time: TimeManagement,
        abort: &'a AtomicBool,
        main: bool,
    ) -> Self {
        Self {
            pos,
            tt,
            history: ButterflyHistory::EMPTY,
            time,
            info: Info::new(),
            abort,
            main,
        }
    }

    pub fn abort(&self) -> bool {
        self.abort.load(Ordering::Relaxed)
    }

    pub fn check_limits(&self) {
        if self.time.check(self.info.nodes) {
            self.abort.store(true, Ordering::Relaxed);
        }
    }

    pub fn main(&self) -> bool {
        self.main
    }

    pub fn update_nodes(&mut self, nodes: u64) {
        self.info.nodes += nodes;
    }

    pub fn update_pv(&mut self, pv: &PrincipalVariation) {
        self.info.pv = pv.clone();
    }

    pub fn report(&self, depth: i32) {
        println!("info depth {} {}", depth, self.info);
    }

    pub fn result(&self) -> (i32, Option<Move>) {
        self.info.pv.result()
    }
}
