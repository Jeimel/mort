use types::{Color, Move, MoveFlag, MoveList, PieceType, Rank, SquareSet};

use crate::chess::attacks;

use super::Board;

macro_rules! push_loop {
    ($moves:expr, $set:expr, $start:expr, $flag:expr) => {
        for target in $set.iter() {
            $moves.push(Move::new($start, target, $flag));
        }
    };
}

impl Board {
    pub(in crate::chess) fn make_move(&mut self, mov: Move, stm: Color) -> bool {
        todo!()
    }

    pub(in crate::chess) fn gen_moves(&self, stm: Color) -> MoveList {
        todo!()
    }

    fn add_attacks(&self, piece: PieceType, stm: Color, moves: &mut MoveList) {
        let pieces = self.get(piece) & self.color(stm);
        let occ = self.all();

        for start in pieces.iter() {
            let attacks = match piece {
                PieceType::Knight => attacks::knight(start),
                PieceType::Bishop => attacks::bishop(start, occ),
                PieceType::Rook => attacks::rook(start, occ),
                PieceType::Queen => attacks::rook(start, occ) | attacks::bishop(start, occ),
                PieceType::King => attacks::king(start),
                _ => unreachable!(),
            };

            // The intersection between our attacks and their pieces yields all captures
            let captures = attacks & self.color(!stm);
            push_loop!(moves, captures, start, MoveFlag::CAPTURE);

            // The difference between our attacks and all blockers yields all quiet moves
            let quiets = attacks - occ;
            push_loop!(moves, quiets, start, MoveFlag::QUIET);
        }
    }
}
