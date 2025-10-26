mod draw;
mod fen;
mod layout;
mod movegen;
mod restore;
mod threat;
mod zobrist;

pub use fen::FenParseError;

use threat::Threat;

use std::fmt::Display;

use layout::PieceLayout;
use types::{Color, Move, MoveFlag, PieceType, Rank, Square};

use crate::chess::position::{restore::RestoreInfo, zobrist::Key};

#[derive(Clone)]
pub struct Position {
    pub layout: PieceLayout,
    threat: Threat,

    restore: RestoreInfo,
    stack: Vec<RestoreInfo>,

    stm: Color,
    ply: u16,

    zobrist: Key,
}

impl Display for Position {
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

        write!(f, "{}\n\nKey: {:x}", pos, self.zobrist)
    }
}

impl Position {
    const EN_PASSANT_TARGET: [Rank; 2] = [Rank::Three, Rank::Six];
    const EN_PASSANT_CAPTURE: [Rank; 2] = [Rank::Four, Rank::Five];

    const KING_CASTLE_TARGET: [Square; 2] = [Square::H1, Square::H8];
    const KING_CASTLE_START: [Square; 2] = [Square::F1, Square::F8];

    const QUEEN_CASTLE_TARGET: [Square; 2] = [Square::A1, Square::A8];
    const QUEEN_CASTLE_START: [Square; 2] = [Square::D1, Square::D8];

    pub fn stm(&self) -> Color {
        self.stm
    }

    pub fn check(&self) -> bool {
        !self.threat.checkers().is_empty()
    }

    pub fn make_move(&mut self, mov: Move) {
        let stm = self.stm;
        let start = mov.start();
        let target = mov.target();
        let flag = mov.flag();

        let piece = self.layout.piece_at(start);

        // We always flip the value for the side to move
        self.zobrist ^= zobrist::SIDE;

        self.stm = !self.stm;
        self.ply += 1;

        // We store all information to revert back to this state
        self.stack.push(self.restore.clone());

        self.restore.rule50_ply += 1;
        self.restore.castling.remove(start, target);
        self.restore.en_passant = None;
        self.restore.capture = None;

        // The fifty move counter is resetted on a pawn move
        if piece == PieceType::Pawn {
            self.restore.rule50_ply = 0;
        }

        match flag {
            // Store en passant target square for next turn
            MoveFlag::DOUBLE_PAWN => {
                self.restore.en_passant =
                    Some(Square::from(start.file(), Self::EN_PASSANT_TARGET[stm]))
            }
            // Place rook on kinsgide castle target, which is either F1 or F8
            MoveFlag::KING_CASTLE => {
                self.toggle(Self::KING_CASTLE_START[stm], stm, PieceType::Rook);
                self.toggle(Self::KING_CASTLE_TARGET[stm], stm, PieceType::Rook);
            }
            // Place rook on queenside castle target, which is either D1 or D8
            MoveFlag::QUEEN_CASTLE => {
                self.toggle(Self::QUEEN_CASTLE_START[stm], stm, PieceType::Rook);
                self.toggle(Self::QUEEN_CASTLE_TARGET[stm], stm, PieceType::Rook);
            }
            // Remove their piece from the board, and reset the fifty move counter
            MoveFlag::CAPTURE => {
                let capture = self.layout.piece_at(target);

                self.toggle(target, !stm, capture);

                self.restore.rule50_ply = 0;
                self.restore.capture = Some(capture);
            }
            // Remove their captured pawn
            MoveFlag::EN_PASSANT => self.toggle(
                Square::from(target.file(), Self::EN_PASSANT_CAPTURE[!stm]),
                !stm,
                PieceType::Pawn,
            ),
            _ => {}
        }

        self.toggle(start, stm, piece);

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

            self.toggle(target, !stm, capture);
            self.restore.capture = Some(capture);
        }

        // Add our new piece back on the board
        self.toggle(target, stm, piece);

        self.threat.set_blockers(Color::White, &self.layout);
        self.threat.set_blockers(Color::Black, &self.layout);

        // We have to update the checkers for the next side to move only
        self.threat.set_checkers(self.stm, &self.layout);
    }

    pub fn unmake_move(&mut self, mov: Move) {
        self.zobrist ^= zobrist::SIDE;

        self.stm = !self.stm;
        self.ply -= 1;

        let stm = self.stm;
        let start = mov.start();
        let target = mov.target();

        let piece = self.layout.piece_at(target);

        self.toggle(target, self.stm, piece);

        match mov.flag() {
            MoveFlag::KING_CASTLE => {
                self.toggle(Self::KING_CASTLE_START[stm], stm, PieceType::Rook);
                self.toggle(Self::KING_CASTLE_TARGET[stm], stm, PieceType::Rook);
            }
            MoveFlag::QUEEN_CASTLE => {
                self.toggle(Self::QUEEN_CASTLE_START[stm], stm, PieceType::Rook);
                self.toggle(Self::QUEEN_CASTLE_TARGET[stm], stm, PieceType::Rook);
            }
            MoveFlag::EN_PASSANT => self.toggle(
                Square::from(target.file(), Self::EN_PASSANT_CAPTURE[!stm]),
                !stm,
                PieceType::Pawn,
            ),
            _ => {}
        }

        let piece = match mov.flag().promotion_piece() {
            Some(_) => PieceType::Pawn,
            None => piece,
        };

        if let Some(capture) = self.restore.capture {
            self.toggle(target, !stm, capture);
        }

        self.toggle(start, self.stm, piece);

        // TODO: store in restore
        self.threat.set_blockers(Color::White, &self.layout);
        self.threat.set_blockers(Color::Black, &self.layout);

        self.threat.set_checkers(!stm, &self.layout);

        self.restore = self.stack.pop().unwrap();
    }

    fn toggle(&mut self, sq: Square, color: Color, piece: PieceType) {
        self.layout.toggle(sq, color, piece);

        self.zobrist ^= zobrist::PIECE[color][piece][sq];
    }
}
