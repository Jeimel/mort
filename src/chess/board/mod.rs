mod fen;

pub use fen::FenParseError;

use types::{Castling, Color, PieceType, Square, SquareSet};

pub struct Board {
    colors: [SquareSet; 2],
    kings: [Square; 2],
    rooks: SquareSet,
    bishops: SquareSet,
    pawns: SquareSet,
    castling: Castling,
    en_passant: Option<Square>,
}

impl Board {
    pub(super) const EMPTY: Board = Self {
        colors: [SquareSet::EMPTY; 2],
        kings: [Square::A1; 2],
        rooks: SquareSet::EMPTY,
        bishops: SquareSet::EMPTY,
        pawns: SquareSet::EMPTY,
        castling: Castling::EMPTY,
        en_passant: None,
    };

    pub fn clear(&mut self) {
        *self = Self::EMPTY;
    }

    fn set(&mut self, sq: Square, color: Color, piece: PieceType) {
        self.colors[color].set(sq);

        if matches!(piece, PieceType::Bishop | PieceType::Queen) {
            self.bishops.set(sq);
        }
        if matches!(piece, PieceType::Rook | PieceType::Queen) {
            self.rooks.set(sq);
        }

        match piece {
            PieceType::Pawn => self.pawns.set(sq),
            PieceType::King => self.kings[color] = sq,
            _ => {}
        }
    }
}
