use std::sync::atomic::{AtomicBool, Ordering};

use crate::{
    chess::Position,
    search::{SearchLimit, data::SearchData},
};

pub struct ThreadData<'a> {
    pub pos: Position,
    pub data: SearchData,
    limits: SearchLimit,
    abort: &'a AtomicBool,
    main: bool,
}

impl<'a> ThreadData<'a> {
    pub fn new(pos: Position, limits: SearchLimit, abort: &'a AtomicBool, main: bool) -> Self {
        Self {
            pos,
            data: SearchData::new(),
            limits,
            abort,
            main,
        }
    }

    pub fn abort(&self) -> bool {
        self.abort.load(Ordering::Relaxed)
    }

    pub fn check_limits(&self) {
        if self.limits.check(&self.data) {
            self.abort.store(true, Ordering::Relaxed);
        }
    }

    pub fn main(&self) -> bool {
        self.main
    }
}
