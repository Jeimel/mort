use std::fmt::Display;

use types::{Color, Move, MoveFlag, MoveList, PieceType, SquareSet};

use crate::error::Error;

use super::board::Board;

#[derive(Clone, Copy)]
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

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
    }
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

    pub fn board(&self) -> Board {
        self.board
    }

    pub fn stm(&self) -> Color {
        self.stm
    }

    pub fn make_move(&mut self, mov: Move) -> bool {
        let stm = self.stm;

        self.stm = !stm;
        self.ply += 1;
        self.rule50_ply += 1;

        // We reset the fifty move counter if either a pawn moved or a capture has been made
        if self.board.piece_at(mov.start()) == PieceType::Pawn || mov.flag() == MoveFlag::CAPTURE {
            self.rule50_ply = 0;
        }

        self.board.make_move(mov, stm)
    }

    pub fn gen_moves(&self) -> MoveList {
        self.board.gen_moves(self.stm)
    }

    pub fn draw(&self) -> bool {
        const LIGHT_SQUARES: SquareSet = SquareSet(0x55AA55AA55AA55AA);
        const DARK_SQUARES: SquareSet = SquareSet(0xAA55AA55AA55AA55);

        if self.rule50_ply >= 100 {
            return true;
        }

        let sufficient_material = self.board.get(PieceType::Pawn)
            | self.board.get(PieceType::Rook)
            | self.board.get(PieceType::Queen);

        // We can still checkmate with any combination of pawns, rooks, and queens
        if !(sufficient_material).is_empty() {
            return false;
        }

        // We can't checkmate anymore with only one bishops or knight left on the board
        if (self.board.color(Color::White) | self.board.color(Color::Black)).popcnt() <= 3 {
            return true;
        }

        // We can still checkmate with a knight and bishop
        if !self.board.get(PieceType::Knight).is_empty() {
            return false;
        }

        let bishops = self.board.get(PieceType::Bishop);

        // We only have bishops on the board, and the game is drawn
        // if all of them are on the same color
        (bishops & LIGHT_SQUARES) == bishops || (bishops & DARK_SQUARES) == bishops
    }

    pub fn perft<const DEBUG: bool>(&self, depth: usize) -> usize {
        perft::<DEBUG>(&self.board, self.stm, depth)
    }
}

fn perft<const ROOT: bool>(board: &Board, stm: Color, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;

    for mov in board.gen_moves(stm) {
        let mut new_board = *board;

        if new_board.make_move(mov, stm) {
            continue;
        }

        let child_nodes = perft::<false>(&new_board, !stm, depth - 1);
        nodes += child_nodes;

        if ROOT {
            println!("{}: {}", mov, child_nodes);
        }
    }

    if ROOT {
        println!("\nNodes searched: {}", nodes);
    }

    nodes
}
