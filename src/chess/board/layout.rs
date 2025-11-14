use std::fmt::Write;

use types::{
    Color, File, Piece,
    PieceType::{self, Bishop, King, Knight, Pawn, Queen, Rook},
    Rank, Square, SquareSet,
};

use crate::chess::attacks;

#[derive(Clone, Copy)]
pub struct PieceLayout {
    // Piece-centric board representation
    colors: [SquareSet; 2],
    kings: [Square; 2],
    pieces: [SquareSet; 6],
    // Square-centric board representation
    mailbox: [Option<Piece>; 64],
}

impl PieceLayout {
    pub const EMPTY: Self = Self {
        colors: [SquareSet::EMPTY; 2],
        kings: [Square::A1; 2],
        pieces: [SquareSet::EMPTY; 6],
        mailbox: [None; 64],
    };

    pub fn all(&self) -> SquareSet {
        self.colors[Color::White] | self.colors[Color::Black]
    }

    pub fn color(&self, color: Color) -> SquareSet {
        self.colors[color]
    }

    pub fn get(&self, piece: PieceType) -> SquareSet {
        self.pieces[piece]
    }

    pub fn at(&self, sq: Square) -> Option<Piece> {
        self.mailbox[sq]
    }

    pub fn unchecked_at(&self, sq: Square) -> PieceType {
        if let Some(piece) = self.mailbox[sq] {
            return piece.typ();
        }

        unreachable!()
    }

    pub fn king(&self, color: Color) -> Square {
        self.kings[color]
    }

    pub fn diagonal(&self) -> SquareSet {
        self.pieces[Bishop] | self.pieces[Queen]
    }

    pub fn orthogonal(&self) -> SquareSet {
        self.pieces[Rook] | self.pieces[Queen]
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
        self.pieces[piece].toggle(sq);

        if piece == King {
            self.kings[color] = sq;
        }

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
        ((self.pieces[Pawn] & attacks::pawn(color, sq))
            | (self.pieces[Knight] & attacks::knight(sq))
            | (self.diagonal() & attacks::bishop(sq, occ))
            | (self.orthogonal() & attacks::rook(sq, occ))
            | (self.pieces[King] & attacks::king(sq)))
            & self.color(!color)
    }

    pub(crate) fn snipers(&self, sq: Square, color: Color) -> SquareSet {
        let rooks = attacks::rook(sq, SquareSet::EMPTY) & self.orthogonal();
        let bishops = attacks::bishop(sq, SquareSet::EMPTY) & self.diagonal();

        (rooks | bishops) & self.color(!color)
    }
}
