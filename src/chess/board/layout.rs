use std::fmt::Write;

use types::{Color, File, Piece, PieceType, Rank, Square, SquareSet};

use crate::chess::attacks;

#[derive(Clone, Copy)]
pub struct PieceLayout {
    // Piece-centric board representation
    colors: [SquareSet; 2],
    kings: [Square; 2],
    rooks: SquareSet,
    bishops: SquareSet,
    pawns: SquareSet,
    // Square-centric board representation
    mailbox: [Option<Piece>; 64],
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
            PieceType::Queen => self.rooks & self.bishops,
            PieceType::King => self.kings(),
        }
    }

    pub fn piece_at(&self, sq: Square) -> PieceType {
        if let Some(piece) = self.mailbox[sq] {
            return piece.typ();
        }

        unreachable!()
    }

    pub fn king(&self, color: Color) -> Square {
        self.kings[color]
    }

    pub fn pawns(&self) -> SquareSet {
        self.pawns
    }

    pub fn knights(&self) -> SquareSet {
        self.all() - self.pawns - self.bishops - self.rooks - self.kings()
    }

    pub fn bishops(&self) -> SquareSet {
        self.bishops
    }

    pub fn rooks(&self) -> SquareSet {
        self.rooks
    }

    pub fn kings(&self) -> SquareSet {
        self.kings[Color::White].set() | self.kings[Color::Black].set()
    }

    pub fn board(&self) -> Result<String, std::fmt::Error> {
        const DELIMITER: &str = concat!("+---+---+---+---+---+---+---+---+", '\n');

        let mut board = String::from(DELIMITER);

        for row in (0..8).rev() {
            let start = row * 8;

            for piece in &self.mailbox[start..(start + 8)] {
                write!(board, "| {} ", piece.map(|c| char::from(c)).unwrap_or(' '))?;
            }

            write!(board, "| {}\n{}", row + 1, DELIMITER)?;
        }

        write!(board, concat!("  a   b   c   d   e   f   g   h", '\n'))?;

        Ok(board)
    }

    pub fn fen(&self) -> Result<String, std::fmt::Error> {
        let mut fen = String::new();

        for rank in Rank::iter().rev() {
            let mut empty = 0;

            for file in File::iter() {
                let piece = self.mailbox[Square::from(file, rank)];

                if piece.is_none() {
                    empty += 1;

                    continue;
                }

                if empty != 0 {
                    write!(fen, "{}", empty)?;
                    empty = 0;
                }

                write!(fen, "{}", char::from(piece.unwrap()))?;
            }

            if empty != 0 {
                write!(fen, "{}", empty)?;
            }

            if rank != Rank::One {
                write!(fen, "/")?;
            }
        }

        Ok(fen)
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
        debug_assert!(!set.is_empty());
        set.iter().all(|sq| self.attackers(sq, color, occ).is_empty())
    }

    pub(crate) fn attackers(&self, sq: Square, color: Color, occ: SquareSet) -> SquareSet {
        ((self.pawns & attacks::pawn(color, sq))
            | (self.knights() & attacks::knight(sq))
            | (self.bishops & attacks::bishop(sq, occ))
            | (self.rooks & attacks::rook(sq, occ))
            | (self.kings[!color].set() & attacks::king(sq)))
            & self.color(!color)
    }

    pub(crate) fn snipers(&self, sq: Square, color: Color) -> SquareSet {
        let rooks = attacks::rook(sq, SquareSet::EMPTY) & self.rooks;
        let bishops = attacks::bishop(sq, SquareSet::EMPTY) & self.bishops;

        (rooks | bishops) & self.color(!color)
    }
}
