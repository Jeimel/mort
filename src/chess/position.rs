use std::fmt::Display;

use types::{Color, Move, MoveList, PieceType, SquareSet};

use crate::error::Error;

use super::board::Board;

#[derive(Clone)]
pub struct Position {
    /// Board from the point of view of white
    pub(crate) board: Board,
    /// Current side to move
    stm: Color,
    /// Number of half-moves since the beginning
    ply: u16,
    /// Previous board states
    history: Vec<Board>,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
    }
}

impl Position {
    pub fn from_fen(fen: &str) -> Result<Self, Error> {
        let mut board = Board::EMPTY;
        let (stm, ply) = board.from_fen(fen)?;

        Ok(Position {
            board,
            stm,
            ply,
            history: Vec::new(),
        })
    }

    pub fn stm(&self) -> Color {
        self.stm
    }

    pub fn make_move(&mut self, mov: Move) -> bool {
        self.history.push(self.board.clone());

        if self.board.make_move(mov, self.stm) {
            self.board = self.history.pop().unwrap();

            return false;
        }

        self.stm = !self.stm;
        self.ply += 1;

        true
    }

    pub fn unmake_move(&mut self) {
        self.stm = !self.stm;
        self.ply -= 1;
        self.board = self.history.pop().unwrap();
    }

    pub fn gen_moves(&self) -> MoveList {
        self.board.gen_moves(self.stm)
    }

    pub fn check(&self) -> bool {
        self.board.check(self.stm)
    }

    pub fn draw(&self) -> bool {
        self.board.rule50_ply >= 100 || self.insufficient_material()
    }

    pub fn upcoming_repetition(&self) -> bool {
        false
    }

    fn insufficient_material(&self) -> bool {
        const LIGHT_SQUARES: SquareSet = SquareSet(0x55AA55AA55AA55AA);
        const DARK_SQUARES: SquareSet = SquareSet(0xAA55AA55AA55AA55);

        let layout = &self.board.layout;

        let sufficient_material = layout.get(PieceType::Pawn)
            | layout.get(PieceType::Rook)
            | layout.get(PieceType::Queen);

        // We can still checkmate with any combination of pawns, rooks, and queens
        if !sufficient_material.is_empty() {
            return false;
        }

        // We can't checkmate anymore with only one bishops or knight left on the board
        if (layout.color(Color::White) | layout.color(Color::Black)).popcnt() <= 3 {
            return true;
        }

        // We can still checkmate with a knight and bishop
        if !layout.get(PieceType::Knight).is_empty() {
            return false;
        }

        let bishops = layout.get(PieceType::Bishop);

        // We only have bishops on the board, and the game is drawn
        // if all of them are on the same color
        (bishops & LIGHT_SQUARES) == bishops || (bishops & DARK_SQUARES) == bishops
    }
}
