use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

use types::Move;

use crate::{
    chess::Position,
    search::{SearchLimit, pv::PrincipalVariation, transposition::TranspositionView},
};

struct Info {
    start: Instant,
    nodes: u64,
    completed: i32,
    pv: PrincipalVariation,
}

impl Info {
    fn new() -> Self {
        Self {
            start: Instant::now(),
            nodes: 0,
            completed: 0,
            pv: PrincipalVariation::EMPTY,
        }
    }

    fn elapsed(&self) -> u128 {
        self.start.elapsed().as_millis()
    }

    fn report(&self) {
        println!(
            "info depth {} score cp {} nodes {} pv {}",
            self.completed,
            self.pv.score(),
            self.nodes,
            self.pv
        )
    }
}

pub struct Worker<'a> {
    pub pos: Position,
    pub tt: TranspositionView<'a>,
    limits: SearchLimit,
    info: Info,
    abort: &'a AtomicBool,
    main: bool,
}

impl<'a> Worker<'a> {
    pub fn new(
        pos: Position,
        tt: TranspositionView<'a>,
        limits: SearchLimit,
        abort: &'a AtomicBool,
        main: bool,
    ) -> Self {
        Self {
            pos,
            tt,
            limits,
            info: Info::new(),
            abort,
            main,
        }
    }

    pub fn abort(&self) -> bool {
        self.abort.load(Ordering::Relaxed)
    }

    pub fn check_limits(&self) {
        if self.limits.check(self.info.elapsed(), self.info.nodes) {
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

    pub fn report(&self) {
        self.info.report();
    }

    pub fn result(&self) -> (i32, Option<Move>) {
        self.info.pv.result()
    }
}
