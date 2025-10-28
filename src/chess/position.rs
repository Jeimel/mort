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
        write!(f, "{}", self.board)
    }
}

impl Position {
    pub fn from_fen(fen: &str) -> Result<Self, Error> {
        let (board, stm, ply) = Board::from_fen(fen)?;

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

    pub fn layout(&self) -> &PieceLayout {
        &self.board.layout
    }

    pub fn zobrist(&self) -> Key {
        self.board.zobrist()
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

    pub fn upcoming_repetition(&self) -> bool {
        self.board.upcoming_repetition()
    }
}
