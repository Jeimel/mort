use crate::search::MAX_PLY;

pub const INF: i32 = 30_000;
pub const MATE: i32 = INF - MAX_PLY;
pub const DRAW: i32 = 0;

pub const fn mate_in(ply: i32) -> i32 {
    debug_assert!(ply <= MAX_PLY);
    INF - ply
}

pub const fn mated_in(ply: i32) -> i32 {
    debug_assert!(ply <= MAX_PLY);
    -INF + ply
}
