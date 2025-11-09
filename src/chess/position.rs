mod repetition;
mod state;

pub use state::GameState;

use std::fmt::Display;

use types::{Color, Move};

use crate::{
    chess::{
        MoveList,
        board::{Board, GenerationType, Key, PieceLayout},
    },
    error::Error,
};

#[derive(Clone)]
pub struct Position {
    board: Board,
    stm: Color,
    ply: usize,
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

    #[allow(dead_code)]
    pub fn zobrist(&self) -> Key {
        self.board.state.zobrist
    }

    pub fn generate<const TYPE: GenerationType>(&self, moves: &mut MoveList) {
        self.board.generate::<TYPE>(moves, self.stm);
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
        self.board.draw() || self.repetition() && self.ply != 0
    }
}
