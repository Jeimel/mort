mod fen;
mod hashing;
mod layout;
mod movegen;

pub use fen::FenParseError;

use std::fmt::Display;

use hashing::{Key, zobrist};
use layout::PieceLayout;
use types::{Castling, Color, PieceType, Square, SquareSet};

use crate::error::Error;

#[derive(Clone, Copy)]
pub struct Position {
    pub layout: PieceLayout,
    castling: Castling,
    en_passant: Option<Square>,
    mailbox: [Option<PieceType>; 64],
    stm: Color,
    ply: u16,
    rule50_ply: u8,
    zobrist: Key,
}

impl Display for Position {
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

        write!(f, "{}\n\nKey: {:x}", pos, self.zobrist)
    }
}

impl Position {
    pub fn from_fen(fen: &str) -> Result<Self, Error> {
        Ok(Position::parse_fen(fen)?)
    }

    pub fn stm(&self) -> Color {
        self.stm
    }

    pub fn piece_at(&self, sq: Square) -> PieceType {
        if let Some(piece) = self.mailbox[sq] {
            return piece;
        }

        unreachable!()
    }

    pub fn check(&self) -> bool {
        self.layout.king_attacked(self.stm)
    }

    pub fn zobrist(&self) -> Key {
        let mut zobrist = self.zobrist;

        if let Some(en_passant) = self.en_passant {
            zobrist ^= zobrist::EN_PASSANT[en_passant.file()];
        }

        zobrist ^ zobrist::CASTLING[self.castling]
    }

    pub fn draw(&self) -> bool {
        self.rule50_ply >= 100 || self.insufficient_material()
    }

    pub fn upcoming_repetition(&self) -> bool {
        false
    }

    fn toggle(&mut self, sq: Square, color: Color, piece: PieceType) {
        self.layout.toggle(sq, color, piece);

        self.zobrist ^= zobrist::PIECE[color][piece][sq];

        self.mailbox[sq] = match self.mailbox[sq] {
            Some(_) => None,
            None => Some(piece),
        };
    }

    fn insufficient_material(&self) -> bool {
        const LIGHT_SQUARES: SquareSet = SquareSet(0x55AA55AA55AA55AA);
        const DARK_SQUARES: SquareSet = SquareSet(0xAA55AA55AA55AA55);

        let sufficient_material = self.layout.get(PieceType::Pawn)
            | self.layout.get(PieceType::Rook)
            | self.layout.get(PieceType::Queen);

        // We can still checkmate with any combination of pawns, rooks, and queens
        if !sufficient_material.is_empty() {
            return false;
        }

        // We can't checkmate anymore with only one bishops or knight left on the board
        if (self.layout.color(Color::White) | self.layout.color(Color::Black)).popcnt() <= 3 {
            return true;
        }

        // We can still checkmate with a knight and bishop
        if !self.layout.get(PieceType::Knight).is_empty() {
            return false;
        }

        let bishops = self.layout.get(PieceType::Bishop);

        // We only have bishops on the board, and the game is drawn
        // if all of them are on the same color
        (bishops & LIGHT_SQUARES) == bishops || (bishops & DARK_SQUARES) == bishops
    }
}
