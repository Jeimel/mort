use std::fmt::Display;

use types::{Color, Move, MoveList};

use crate::{
    chess::{
        board::{Board, Key, PieceLayout},
        state::GameState,
    },
    error::Error,
};

#[derive(Clone)]
pub struct Position {
    board: Board,
    stm: Color,
    ply: u16,
    history: Vec<GameState>,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nFen: {}", self.board, self.fen())
    }
}

impl Position {
    pub fn from_fen(fen: &str) -> Result<Self, Error> {
        let (board, stm, ply) = Board::from_fen(fen)?;

        Ok(Position {
            board,
            stm,
            ply: (ply - 1) * 2 + if stm == Color::White { 0 } else { 1 },
            history: Vec::new(),
        })
    }

    pub fn fen(&self) -> String {
        self.board.fen(self.stm, (self.ply + 2) / 2)
    }

    pub fn stm(&self) -> Color {
        self.stm
    }

    pub fn layout(&self) -> &PieceLayout {
        &self.board.layout
    }

    pub fn zobrist(&self) -> Key {
        self.board.state.zobrist
    }

    pub fn generate<const QUIET: bool>(&self, moves: &mut MoveList) {
        self.board.generate::<QUIET>(moves, self.stm);
    }

    pub fn legal(&self, mov: Move) -> bool {
        self.board.legal(mov, self.stm)
    }

    pub fn make_move(&mut self, mov: Move) {
        self.history.push(self.board.state.clone());

        self.board.make_move(mov, self.stm);

        self.stm = !self.stm;
        self.ply += 1;
    }

    pub fn unmake_move(&mut self, mov: Move) {
        self.stm = !self.stm;
        self.ply -= 1;

        self.board
            .unmake_move(mov, self.stm, self.history.pop().unwrap());
    }

    pub fn check(&self) -> bool {
        !self.board.state.checkers.is_empty()
    }

    pub fn draw(&self) -> bool {
        self.board.draw()
    }

    pub fn repetition(&self) -> bool {
        self.history
            .iter()
            .rev()
            // We have to consider all plys until the last irreversible move
            .take(self.board.state.rule50_ply as usize + 1)
            // A repetition can only happen two fullmoves ago
            .skip(3)
            // We only have to consider a position, where it is our turn
            .step_by(2)
            .any(|state| state.zobrist == self.board.state.zobrist)
    }
}
