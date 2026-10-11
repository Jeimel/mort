use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use crate::{
    chess::{Move, Position},
    evaluation::uci_score,
    search::{
        MAX_DEPTH, TimeManagement, history::ButterflyHistory, pv::PrincipalVariation,
        stack::StackEntry, time::nps, transposition::TranspositionView,
    },
};

struct Info {
    nodes: u64,
    pv: PrincipalVariation,
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
    pub(super) stack: [StackEntry; MAX_DEPTH],
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
            stack: [StackEntry::EMPTY; MAX_DEPTH],
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

    pub fn check_hard(&self) {
        if self.time.hard(self.info.nodes) {
            self.abort.store(true, Ordering::Relaxed);
        }
    }

    pub fn check_soft(&self) -> bool {
        self.time.soft()
    }

    pub fn main(&self) -> bool {
        self.main
    }

    pub fn increment_nodes(&mut self) {
        self.info.nodes += 1;
    }

    pub fn update_pv(&mut self, pv: &PrincipalVariation) {
        self.info.pv = pv.clone();
    }

    pub fn report(&self, depth: i32) {
        let (nodes, time) = (self.info.nodes, self.elapsed());

        println!(
            "info depth {} score {} nodes {} nps {} time {} pv {}",
            depth,
            uci_score(self.info.pv.score()),
            nodes,
            nps(nodes, time),
            time.as_millis(),
            self.info.pv
        );
    }

    pub fn result(&self) -> (i32, Option<Move>) {
        self.info.pv.result()
    }

    pub fn nodes(&self) -> u64 {
        self.info.nodes
    }

    pub fn elapsed(&self) -> Duration {
        self.time.elapsed()
    }
}
