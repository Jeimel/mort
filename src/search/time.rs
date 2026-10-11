use std::time::{Duration, Instant};

pub fn nps(nodes: u64, time: Duration) -> u64 {
    (nodes as u128 * 1_000_000 / time.as_micros().max(1)) as u64
}

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
    soft: Duration,
    hard: Duration,
}

impl TimeManagement {
    pub fn new(limit: SearchLimit, overhead: u64) -> Self {
        let (soft, hard) = match limit {
            SearchLimit::Time(millis) => (millis, millis),
            SearchLimit::Bonus(main, inc) => {
                let soft = main / 20 + inc / 2;
                let hard = (3 * soft).min(main / 2).saturating_sub(overhead).max(1);

                (soft.saturating_sub(overhead).max(1).min(hard), hard)
            }
            _ => (u64::MAX, u64::MAX),
        };

        Self {
            limit,
            start: Instant::now(),
            soft: Duration::from_millis(soft),
            hard: Duration::from_millis(hard),
        }
    }

    pub fn limit(&self) -> SearchLimit {
        self.limit.clone()
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn hard(&self, nodes: u64) -> bool {
        match self.limit {
            SearchLimit::Infinite | SearchLimit::Depth(_) => false,
            SearchLimit::Nodes(max) => nodes > max,
            _ => nodes & 2047 == 0 && self.start.elapsed() >= self.hard,
        }
    }

    pub fn soft(&self) -> bool {
        self.start.elapsed() >= self.soft
    }
}
