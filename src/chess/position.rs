use types::Color;

use super::board::Board;

pub struct Position {
    /// Board fron the point of view of white
    board: Board,
    /// Current side to move
    stm: Color,
    /// Number of half-moves since the beginning
    ply: u16,
    /// Number of half-moves since the last capture or pawn move
    rule50_ply: u8,
}
