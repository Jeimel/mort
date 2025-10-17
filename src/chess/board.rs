mod fen;
mod layout;
mod movegen;

pub use fen::FenParseError;
use layout::PieceLayout;

use std::fmt::Display;

use types::{Castling, Color, PieceType, Square};

#[derive(Clone)]
pub struct Board {
    pub layout: PieceLayout,
    pub castling: Castling,
    pub en_passant: Option<Square>,
    pub mailbox: [Option<PieceType>; 64],
    pub rule50_ply: u8,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const DELIMITER: &str = concat!("+---+---+---+---+---+---+---+---+", '\n');

        let mut pos = String::from(DELIMITER);

        for row in (0..8).rev() {
            let start = row * 8;

            let mut rank = String::new();

            for c in &self.mailbox[start..(start + 8)] {
                if let Some(piece) = c {
                    rank.push_str(&format!("| {} ", char::from(*piece)));
                }
            }

            pos.push_str(&format!("{}| {}\n{}", rank, row + 1, DELIMITER));
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

    pub fn clear(&mut self) {
        *self = Self::EMPTY;
    }

    pub fn piece_at(&self, sq: Square) -> PieceType {
        if let Some(piece) = self.mailbox[sq] {
            return piece;
        }

        unreachable!()
    }

    pub fn check(&self, stm: Color) -> bool {
        self.layout
            .attacked(self.layout.kings[stm], stm, self.layout.all())
    }

    fn toggle(&mut self, sq: Square, color: Color, piece: PieceType) {
        self.layout.toggle(sq, color, piece);

        self.mailbox[sq] = match self.mailbox[sq] {
            Some(_) => None,
            None => Some(piece),
        };
    }
}
