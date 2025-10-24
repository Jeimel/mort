pub struct SearchLimit {
    pub perft: u16,
    pub depth: u16,
    pub nodes: u64,
    pub time: u128,
}

impl SearchLimit {
    pub const MAX: Self = Self {
        perft: 0,
        depth: 64,
        nodes: u64::MAX,
        time: u128::MAX,
    };
}
