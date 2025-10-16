pub struct SearchLimit {
    pub time: u128,
    pub nodes: u64,
}

impl SearchLimit {
    pub const MAX: Self = Self {
        time: u128::MAX,
        nodes: u64::MAX,
    };
}
