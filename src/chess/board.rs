mod fen;
mod layout;
mod movegen;

use std::fmt::Display;

use layout::PieceLayout;
use types::{Castling, Color, PieceType, Square};

pub use fen::FenParseError;

#[derive(Clone)]
pub struct Board {
    pub layout: PieceLayout,
    castling: Castling,
    en_passant: Option<Square>,
    mailbox: [Option<PieceType>; 64],
    rule50_ply: u8,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const DELIMITER: &str = concat!("+---+---+---+---+---+---+---+---+", '\n');

        let mut pos = String::from(DELIMITER);

        for row in (0..8).rev() {
            let start = row * 8;

            for c in &self.mailbox[start..(start + 8)] {
                pos.push_str(&format!("| {} ", c.map(|c| char::from(c)).unwrap_or(' ')));
            }

            pos.push_str(&format!("| {}\n{}", row + 1, DELIMITER));
        }

        pos.push_str("  a   b   c   d   e   f   g   h");
        write!(f, "{pos}")
    }
}

impl Board {
    pub(super) const EMPTY: Board = Self {
        layout: PieceLayout::EMPTY,
        castling: Castling::EMPTY,
        en_passant: None,
        mailbox: [None; 64],
        rule50_ply: 0,
    };

    pub(crate) fn clear(&mut self) {
        *self = Self::EMPTY;
    }

    pub(crate) fn check(&self, stm: Color) -> bool {
        self.layout
            .attacked(self.layout.kings[stm], stm, self.layout.all())
    }

    pub(crate) fn rule50_ply(&self) -> u8 {
        self.rule50_ply
    }

    pub fn piece_at(&self, sq: Square) -> PieceType {
        if let Some(piece) = self.mailbox[sq] {
            return piece;
        }

        unreachable!()
    }

    fn toggle(&mut self, sq: Square, color: Color, piece: PieceType) {
        self.layout.toggle(sq, color, piece);

        self.mailbox[sq] = match self.mailbox[sq] {
            Some(_) => None,
            None => Some(piece),
        };
    }
}
