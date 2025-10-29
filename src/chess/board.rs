mod draw;
mod fen;
mod layout;
mod movegen;
mod zobrist;

pub use fen::FenParseError;
pub use layout::PieceLayout;
pub use movegen::BETWEEN;
pub use zobrist::Key;

use std::fmt::Display;

use types::{Color, Move, MoveFlag, PieceType, Rank, Square};

use crate::chess::state::GameState;

#[derive(Clone)]
pub struct Board {
    pub layout: PieceLayout,
    pub state: GameState,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const DELIMITER: &str = concat!("+---+---+---+---+---+---+---+---+", '\n');

        let mut pos = String::from(DELIMITER);

        for row in (0..8).rev() {
            let start = row * 8;

            for c in &self.layout.mailbox[start..(start + 8)] {
                pos.push_str(&format!("| {} ", c.map(|c| char::from(c)).unwrap_or(' ')));
            }

            pos.push_str(&format!("| {}\n{}", row + 1, DELIMITER));
        }

        pos.push_str("  a   b   c   d   e   f   g   h");

        write!(f, "{}\n\nKey: {:x}", pos, self.state.zobrist)
    }
}

impl Board {
    const EN_PASSANT_TARGET: [Rank; 2] = [Rank::Three, Rank::Six];
    const EN_PASSANT_CAPTURE: [Rank; 2] = [Rank::Four, Rank::Five];

    const KING_CASTLE_START: [Square; 2] = [Square::F1, Square::F8];
    const KING_CASTLE_TARGET: [Square; 2] = [Square::H1, Square::H8];

    const QUEEN_CASTLE_START: [Square; 2] = [Square::D1, Square::D8];
    const QUEEN_CASTLE_TARGET: [Square; 2] = [Square::A1, Square::A8];

    pub fn make_move(&mut self, mov: Move, color: Color) {
        let start = mov.start();
        let target = mov.target();
        let flag = mov.flag();

        let piece = self.layout.piece_at(start);

        self.state.zobrist ^= zobrist::SIDE;
        self.state.zobrist ^= zobrist::CASTLING[self.state.castling];

        if let Some(target) = self.state.en_passant {
            self.state.zobrist ^= zobrist::EN_PASSANT[target.file()];
        }

        self.state.castling.remove(start, target);
        self.state.en_passant = None;
        self.state.rule50_ply += 1;
        self.state.capture = None;

        // The fifty move counter is resetted on a pawn move
        if piece == PieceType::Pawn {
            self.state.rule50_ply = 0;
        }

        self.state.zobrist ^= zobrist::CASTLING[self.state.castling];

        match flag {
            // Store en passant target square for next turn
            MoveFlag::DOUBLE_PAWN => {
                let file = start.file();

                self.state.en_passant = Some(Square::from(file, Self::EN_PASSANT_TARGET[color]));
                self.state.zobrist ^= zobrist::EN_PASSANT[file];
            }
            // Place rook on kinsgide castle target, which is either F1 or F8
            MoveFlag::KING_CASTLE => {
                self.toggle::<true>(Self::KING_CASTLE_START[color], color, PieceType::Rook);
                self.toggle::<true>(Self::KING_CASTLE_TARGET[color], color, PieceType::Rook);
            }
            // Place rook on queenside castle target, which is either D1 or D8
            MoveFlag::QUEEN_CASTLE => {
                self.toggle::<true>(Self::QUEEN_CASTLE_START[color], color, PieceType::Rook);
                self.toggle::<true>(Self::QUEEN_CASTLE_TARGET[color], color, PieceType::Rook);
            }
            // Remove their piece from the board, and reset the fifty move counter
            MoveFlag::CAPTURE => {
                let capture = self.layout.piece_at(target);

                self.toggle::<true>(target, !color, capture);

                self.state.rule50_ply = 0;
                self.state.capture = Some(capture);
            }
            // Remove their captured pawn
            MoveFlag::EN_PASSANT => self.toggle::<true>(
                Square::from(target.file(), Self::EN_PASSANT_CAPTURE[!color]),
                !color,
                PieceType::Pawn,
            ),
            _ => {}
        };

        self.toggle::<true>(start, color, piece);

        // Determine which piece must be placed on the target square
        let piece = match flag.promotion_piece() {
            // We promote our piece
            Some(piece) => piece,
            // We just move our piece to the target square
            None => piece,
        };

        // We captured their piece to promote ours
        if self.layout.all().is_set(target) {
            let capture = self.layout.piece_at(target);

            self.toggle::<true>(target, !color, capture);

            self.state.capture = Some(capture)
        }

        // Add our new piece back on the board
        self.toggle::<true>(target, color, piece);

        // We have to update for the next side to move only
        self.state.set_blockers(!color, &self.layout);
        self.state.set_checkers(!color, &self.layout);
    }

    pub fn unmake_move(&mut self, mov: Move, color: Color, state: GameState) {
        let start = mov.start();
        let target = mov.target();
        let flag = mov.flag();

        let piece = self.layout.piece_at(target);

        self.toggle::<false>(target, color, piece);

        match flag {
            MoveFlag::KING_CASTLE => {
                self.toggle::<false>(Self::KING_CASTLE_START[color], color, PieceType::Rook);
                self.toggle::<false>(Self::KING_CASTLE_TARGET[color], color, PieceType::Rook);
            }
            MoveFlag::QUEEN_CASTLE => {
                self.toggle::<false>(Self::QUEEN_CASTLE_START[color], color, PieceType::Rook);
                self.toggle::<false>(Self::QUEEN_CASTLE_TARGET[color], color, PieceType::Rook);
            }
            MoveFlag::EN_PASSANT => self.toggle::<false>(
                Square::from(target.file(), Self::EN_PASSANT_CAPTURE[!color]),
                !color,
                PieceType::Pawn,
            ),
            _ => {}
        };

        let piece = match flag.promotion_piece() {
            Some(_) => PieceType::Pawn,
            None => piece,
        };

        if let Some(capture) = self.state.capture {
            self.toggle::<false>(target, !color, capture);
        }

        self.toggle::<false>(start, color, piece);

        self.state = state;
    }

    fn toggle<const ZOBRIST: bool>(&mut self, sq: Square, color: Color, piece: PieceType) {
        self.layout.toggle(sq, color, piece);

        if ZOBRIST {
            self.state.zobrist ^= zobrist::PIECE[color][piece][sq];
        }
    }
}
