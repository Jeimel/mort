use types::{Color, PieceType, SquareSet};

use crate::chess::board::Board;

impl Board {
    pub fn draw(&self) -> bool {
        self.state.rule50_ply >= 100 || self.insufficient_material()
    }

    fn insufficient_material(&self) -> bool {
        const LIGHT_SQUARES: SquareSet = SquareSet(0x55AA55AA55AA55AA);
        const DARK_SQUARES: SquareSet = SquareSet(0xAA55AA55AA55AA55);

        let sufficient_material = self.layout.get(PieceType::Pawn)
            | self.layout.get(PieceType::Rook)
            | self.layout.get(PieceType::Queen);

        // We can still checkmate with any combination of pawns, rooks, and queens
        if !sufficient_material.is_empty() {
            return false;
        }

        // We can't checkmate anymore with only one bishops or knight left on the board
        if (self.layout.color(Color::White) | self.layout.color(Color::Black)).popcnt() <= 3 {
            return true;
        }

        // We can still checkmate with a knight and bishop
        if !self.layout.get(PieceType::Knight).is_empty() {
            return false;
        }

        let bishops = self.layout.get(PieceType::Bishop);

        // We only have bishops on the board, and the game is drawn
        // if all of them are on the same color
        (bishops & LIGHT_SQUARES) == bishops || (bishops & DARK_SQUARES) == bishops
    }
}
