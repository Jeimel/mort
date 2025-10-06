use std::fmt::Display;

use types::{Color, Move, MoveFlag, MoveList, PieceType};

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

    pub fn make_move(&mut self, mov: Move) -> bool {
        let stm = self.stm;

        self.stm = !stm;
        self.ply += 1;
        self.rule50_ply = if self.board.piece(mov.start()) == PieceType::Pawn
            || mov.flag() == MoveFlag::CAPTURE
        {
            0
        } else {
            self.rule50_ply + 1
        };

        self.board.make_move(mov, stm)
    }

    pub fn gen_moves(&self) -> MoveList {
        self.board.gen_moves(self.stm)
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
