use std::sync::atomic::{AtomicBool, Ordering};

use crate::{
    chess::Position,
    search::{SearchLimit, info::SearchInfo, transposition::TranspositionView},
};

pub struct ThreadData<'a> {
    pub pos: Position,
    pub info: SearchInfo,
    limits: SearchLimit,
    pub tt: TranspositionView<'a>,
    abort: &'a AtomicBool,
    main: bool,
}

impl<'a> ThreadData<'a> {
    pub fn new(
        pos: Position,
        limits: SearchLimit,
        tt: TranspositionView<'a>,
        abort: &'a AtomicBool,
        main: bool,
    ) -> Self {
        Self {
            pos,
            info: SearchInfo::new(),
            limits,
            tt,
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
