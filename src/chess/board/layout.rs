use types::{Color, Piece, PieceType, Square, SquareSet};

use crate::chess::attacks;

#[derive(Clone, Copy)]
pub struct PieceLayout {
    // Piece-centric board representation
    pub(super) colors: [SquareSet; 2],
    pub(super) kings: [Square; 2],
    pub(super) rooks: SquareSet,
    pub(super) bishops: SquareSet,
    pub(super) pawns: SquareSet,
    // Square-centric board representation
    pub(super) mailbox: [Option<Piece>; 64],
}

impl PieceLayout {
    pub const EMPTY: Self = Self {
        colors: [SquareSet::EMPTY; 2],
        kings: [Square::A1; 2],
        rooks: SquareSet::EMPTY,
        bishops: SquareSet::EMPTY,
        pawns: SquareSet::EMPTY,
        mailbox: [None; 64],
    };

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

    pub fn piece_at(&self, sq: Square) -> PieceType {
        if let Some(piece) = self.mailbox[sq] {
            return piece.typ();
        }

        unreachable!()
    }

    pub(crate) fn toggle(&mut self, sq: Square, color: Color, piece: PieceType) {
        self.colors[color].toggle(sq);

        if matches!(piece, PieceType::Bishop | PieceType::Queen) {
            self.bishops.toggle(sq);
        }
        if matches!(piece, PieceType::Rook | PieceType::Queen) {
            self.rooks.toggle(sq);
        }

        match piece {
            PieceType::Pawn => self.pawns.toggle(sq),
            PieceType::King => self.kings[color] = sq,
            _ => {}
        };

        self.mailbox[sq] = match self.mailbox[sq] {
            Some(_) => None,
            None => Some(Piece::from(color, piece)),
        };
    }

    #[rustfmt::skip]
    pub(crate) fn attacked(&self, set: SquareSet, color: Color, occ: SquareSet) -> bool {
        set.iter().all(|sq| self.attackers(sq, color, occ).is_empty())
    }

    pub(crate) fn attackers(&self, sq: Square, color: Color, occ: SquareSet) -> SquareSet {
        ((self.pawns & attacks::pawn(color, sq))
            | (self.knights() & attacks::knight(sq, SquareSet::EMPTY))
            | (self.bishops & attacks::bishop(sq, occ))
            | (self.rooks & attacks::rook(sq, occ))
            | (self.kings[!color].set() & attacks::king(sq, SquareSet::EMPTY)))
            & self.color(!color)
    }

    pub(crate) fn snipers(&self, sq: Square, color: Color) -> SquareSet {
        let rooks = attacks::rook(sq, SquareSet::EMPTY) & self.rooks;
        let bishops = attacks::bishop(sq, SquareSet::EMPTY) & self.bishops;

        (rooks | bishops) & self.color(!color)
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
}
