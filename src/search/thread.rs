use std::sync::atomic::{AtomicBool, Ordering};

use crate::{
    chess::Position,
    search::{SearchLimit, info::SearchInfo},
};

pub struct ThreadData<'a> {
    pub pos: Position,
    pub info: SearchInfo,
    limits: SearchLimit,
    abort: &'a AtomicBool,
    main: bool,
}

impl<'a> ThreadData<'a> {
    pub fn new(pos: Position, limits: SearchLimit, abort: &'a AtomicBool, main: bool) -> Self {
        Self {
            pos,
            info: SearchInfo::new(),
            limits,
            abort,
            main,
        }
    }

    pub fn abort(&self) -> bool {
        self.abort.load(Ordering::Relaxed)
    }

    pub fn check_limits(&self) {
        if self.limits.check(&self.info) {
            self.abort.store(true, Ordering::Relaxed);
        }
    }

    pub fn main(&self) -> bool {
        self.main
    }
}
