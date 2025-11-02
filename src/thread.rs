use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

use types::Move;

use crate::{Position, evaluation::DRAW, search::SearchLimit};

pub struct ThreadData<'a> {
    // Search
    start: Instant,
    abort: &'a AtomicBool,
    pub limits: SearchLimit,

    // State
    pub pos: Position,
    pub score: i32,
    main: bool,

    // Debug
    pub nodes: u64,
    pub best: Option<Move>,
}

impl<'a> ThreadData<'a> {
    pub fn new(abort: &'a AtomicBool, pos: Position, main: bool, limits: SearchLimit) -> Self {
        Self {
            start: Instant::now(),
            abort,
            limits,
            pos,
            score: DRAW,
            main,
            nodes: 0,
            best: None,
        }
    }

    pub fn abort(&self) -> bool {
        self.abort.load(Ordering::Relaxed)
    }

    pub fn check_limits(&self) {
        if self.start.elapsed().as_millis() < self.limits.time && self.nodes < self.limits.nodes {
            return;
        }

        self.abort.store(true, Ordering::Relaxed);
    }

    pub fn main(&self) -> bool {
        self.main
    }
}
