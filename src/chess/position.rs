use types::Color;

use crate::uci::Error;

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

impl Position {
    pub fn from_fen(fen: &str) -> Result<Self, Error> {
        let mut board = Board::EMPTY;
        let (stm, ply, rule50_ply) = board.from_fen(fen)?;

        Ok(Position {
            board,
            stm,
            ply,
            rule50_ply,
        })
    }
}
