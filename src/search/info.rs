use std::time::Instant;

use crate::search::PrincipalVariation;

pub struct SearchInfo {
    pub start: Instant,
    pub nodes: u64,
    pub completed: i32,
    pub pv: PrincipalVariation,
}

impl SearchInfo {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            nodes: 0,
            completed: 0,
            pv: PrincipalVariation::EMPTY,
        }
    }

    pub fn report(&self) {
        println!(
            "info depth {} score cp {} nodes {} pv {}",
            self.completed, self.pv.score, self.nodes, self.pv
        )
    }
}
