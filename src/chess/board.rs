mod draw;
mod fen;
mod layout;
mod legal;
mod movegen;
mod zobrist;

pub use fen::FenParseError;
pub use layout::PieceLayout;
pub use movegen::GenerationType;
pub use zobrist::Key;

use std::fmt::Display;

use types::{
    Color, Move, MoveFlag,
    PieceType::{self, Bishop, King, Knight, Pawn, Queen, Rook},
    Rank, Square, SquareSet,
};

use crate::chess::position::GameState;

include!(concat!(env!("OUT_DIR"), "/squareset_tables.rs"));

#[derive(Clone)]
pub struct Board {
    pub layout: PieceLayout,
    pub state: GameState,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nKey: {:x}", self.layout.board()?, self.state.zobrist)
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

        let piece = self.layout.unchecked_at(start);

        debug_assert!(!(start.set() & self.layout.color(color)).is_empty());

        self.state.zobrist ^= zobrist::SIDE;

        if let Some(target) = self.state.en_passant {
            self.state.zobrist ^= zobrist::EN_PASSANT[target.file()];
        }

        self.state.en_passant = None;
        self.state.rule50_ply += 1;
        self.state.capture = None;

        // The fifty move counter is resetted on a pawn move
        if piece == PieceType::Pawn {
            self.state.rule50_ply = 0;
        }

        self.state.zobrist ^= zobrist::CASTLING[self.state.castling];
        self.state.castling.remove(start, target);
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
                self.toggle::<true>(Self::KING_CASTLE_START[color], color, Rook);
                self.toggle::<true>(Self::KING_CASTLE_TARGET[color], color, Rook);
            }
            // Place rook on queenside castle target, which is either D1 or D8
            MoveFlag::QUEEN_CASTLE => {
                self.toggle::<true>(Self::QUEEN_CASTLE_START[color], color, Rook);
                self.toggle::<true>(Self::QUEEN_CASTLE_TARGET[color], color, Rook);
            }
            // Remove their piece from the board, and reset the fifty move counter
            MoveFlag::CAPTURE => {
                let capture = self.layout.unchecked_at(target);

                debug_assert!(capture != King);

                self.toggle::<true>(target, !color, capture);

                self.state.rule50_ply = 0;
                self.state.capture = Some(capture);
            }
            // Remove their captured pawn
            MoveFlag::EN_PASSANT => self.toggle::<true>(
                Square::from(target.file(), Self::EN_PASSANT_CAPTURE[!color]),
                !color,
                Pawn,
            ),
            _ => {}
        };

        self.toggle::<true>(start, color, piece);

        // Determine which piece must be placed on the target square
        let piece = match flag.promotion_piece() {
            // We promote our piece
            Some(piece) => {
                debug_assert!(matches!(piece, Knight | Bishop | Rook | Queen));
                piece
            }
            // We just move our piece to the target square
            None => piece,
        };

        // We captured their piece to promote ours
        if self.layout.all().is_set(target) {
            let capture = self.layout.unchecked_at(target);

            self.toggle::<true>(target, !color, capture);

            self.state.capture = Some(capture)
        }

        // Add our new piece back on the board
        self.toggle::<true>(target, color, piece);

        debug_assert!(!(target.set() & self.layout.color(color)).is_empty());

        // We have to update for the next side to move only
        self.state.set_blockers(!color, &self.layout);
        self.state.set_checkers(!color, &self.layout);

        #[rustfmt::skip]
        debug_assert!(
            self.layout.attackers(self.layout.king(color), color, self.layout.all()).is_empty()
        );
    }

    pub fn unmake_move(&mut self, mov: Move, color: Color, state: GameState) {
        let start = mov.start();
        let target = mov.target();
        let flag = mov.flag();

        let piece = self.layout.unchecked_at(target);

        debug_assert!(!(target.set() & self.layout.color(color)).is_empty());

        self.toggle::<false>(target, color, piece);

        match flag {
            MoveFlag::KING_CASTLE => {
                self.toggle::<false>(Self::KING_CASTLE_START[color], color, Rook);
                self.toggle::<false>(Self::KING_CASTLE_TARGET[color], color, Rook);
            }
            MoveFlag::QUEEN_CASTLE => {
                self.toggle::<false>(Self::QUEEN_CASTLE_START[color], color, Rook);
                self.toggle::<false>(Self::QUEEN_CASTLE_TARGET[color], color, Rook);
            }
            MoveFlag::EN_PASSANT => self.toggle::<false>(
                Square::from(target.file(), Self::EN_PASSANT_CAPTURE[!color]),
                !color,
                PieceType::Pawn,
            ),
            _ => {}
        };

        let piece = match flag.promotion_piece() {
            Some(_) => {
                debug_assert!(matches!(piece, Knight | Bishop | Rook | Queen));
                Pawn
            }
            None => piece,
        };

        if let Some(capture) = self.state.capture {
            debug_assert!((target.set() & self.layout.color(!color)).is_empty());
            debug_assert!(capture != King);

            self.toggle::<false>(target, !color, capture);
        }

        self.toggle::<false>(start, color, piece);

        debug_assert!(!(start.set() & self.layout.color(color)).is_empty());

        self.state = state;
    }

    fn toggle<const ZOBRIST: bool>(&mut self, sq: Square, color: Color, piece: PieceType) {
        self.layout.toggle(sq, color, piece);

        if ZOBRIST {
            self.state.zobrist ^= zobrist::PIECE[color][piece][sq];
        }
    }
}
