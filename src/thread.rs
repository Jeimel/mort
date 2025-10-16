use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

use types::Move;

use crate::search::SearchLimit;

pub struct ThreadData<'a> {
    // Search
    start: Instant,
    abort: &'a AtomicBool,
    limits: SearchLimit,

    // Debug
    pub nodes: u64,
    pub best: Option<Move>,
}

impl<'a> ThreadData<'a> {
    pub fn new(abort: &'a AtomicBool, limits: SearchLimit) -> Self {
        Self {
            start: Instant::now(),
            abort,
            limits,
            nodes: 0,
            best: None,
        }
    }

    pub fn abort(&self) -> bool {
        if self.abort.load(Ordering::Relaxed) {
            return true;
        }

        if self.start.elapsed().as_millis() < self.limits.time && self.nodes < self.limits.nodes {
            return false;
        }

        !self.abort.swap(true, Ordering::Relaxed)
    }
}
