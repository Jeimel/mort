use crate::search::{MAX_DEPTH, MAX_PLY};

pub const INF: i32 = 30_000;
pub const MATE: i32 = INF - MAX_PLY;
pub const DRAW: i32 = 0;

pub const fn mate_in(ply: usize) -> i32 {
    debug_assert!(ply <= MAX_DEPTH);
    INF - ply as i32
}

pub const fn mated_in(ply: usize) -> i32 {
    debug_assert!(ply <= MAX_DEPTH);
    -INF + ply as i32
}
