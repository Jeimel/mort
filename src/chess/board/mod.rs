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

    pub fn all(&self) -> SquareSet {
        self.colors[Color::White] | self.colors[Color::Black]
    }

    pub fn color(&self, color: Color) -> SquareSet {
        self.colors[color]
    }

    pub fn get(&self, piece: PieceType) -> SquareSet {
        match piece {
            PieceType::Pawn => self.pawns(),
            PieceType::Knight => self.knights(),
            PieceType::Bishop => self.bishops(),
            PieceType::Rook => self.rooks(),
            PieceType::Queen => self.queens(),
            PieceType::King => self.kings(),
        }
    }

    fn pawns(&self) -> SquareSet {
        self.pawns
    }

    fn knights(&self) -> SquareSet {
        self.all() - self.pawns - self.bishops - self.rooks - self.kings()
    }

    fn bishops(&self) -> SquareSet {
        self.bishops - self.rooks
    }

    fn rooks(&self) -> SquareSet {
        self.rooks - self.bishops
    }

    fn queens(&self) -> SquareSet {
        self.rooks & self.bishops
    }

    fn kings(&self) -> SquareSet {
        self.kings[Color::White].set() | self.kings[Color::Black].set()
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
