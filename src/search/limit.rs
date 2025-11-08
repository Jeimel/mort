use crate::search::{MAX_DEPTH, info::SearchInfo};

#[derive(Clone)]
pub struct SearchLimit {
    pub perft: u16,
    pub depth: u16,
    pub nodes: u64,
    pub time: u128,
}

impl SearchLimit {
    pub const MAX: Self = Self {
        perft: 0,
        depth: MAX_DEPTH as u16,
        nodes: u64::MAX,
        time: u128::MAX,
    };

    pub fn check(&self, data: &SearchInfo) -> bool {
        data.start.elapsed().as_millis() > self.time || data.nodes > self.nodes
    }
}
