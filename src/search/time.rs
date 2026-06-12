use std::time::{Duration, Instant};

#[derive(Clone)]
pub enum SearchLimit {
    Infinite,
    Perft(u16),
    Depth(i32),
    Nodes(u64),
    Time(u64),
    Bonus(u64, u64),
}

#[derive(Clone)]
pub struct TimeManagement {
    limit: SearchLimit,
    start: Instant,
    bound: Duration,
}

impl TimeManagement {
    pub fn new(limit: SearchLimit, overhead: u64) -> Self {
        let millis = match limit {
            SearchLimit::Time(millis) => millis,
            SearchLimit::Bonus(main, inc) => (main / 20 + inc / 2).saturating_sub(overhead).max(1),
            _ => u64::MAX,
        };

        Self {
            limit,
            start: Instant::now(),
            bound: Duration::from_millis(millis),
        }
    }

    pub fn limit(&self) -> SearchLimit {
        self.limit.clone()
    }

    pub fn check(&self, nodes: u64) -> bool {
        match self.limit {
            SearchLimit::Infinite | SearchLimit::Depth(_) => false,
            SearchLimit::Nodes(max) => nodes > max,
            _ => nodes & 2047 == 2047 && self.start.elapsed() >= self.bound,
        }
    }
}
