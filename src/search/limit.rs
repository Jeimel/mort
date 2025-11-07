use crate::search::MAX_DEPTH;

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
        depth: MAX_DEPTH,
        nodes: u64::MAX,
        time: u128::MAX,
    };
}

