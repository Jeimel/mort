use std::time::Instant;

use crate::search::PrincipalVariation;

pub struct SearchData {
    pub start: Instant,
    pub nodes: u64,
    pub completed: u16,
    pub pv: PrincipalVariation,
}

impl SearchData {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            nodes: 0,
            completed: 0,
            pv: PrincipalVariation::EMPTY,
        }
    }
}
