mod fen;
mod movegen;

pub use fen::FenParseError;

use std::fmt::Display;

use types::{Castling, Color, PieceType, Square, SquareSet};

use crate::chess::attacks;

#[derive(Clone, Copy)]
pub struct Board {
    colors: [SquareSet; 2],
    kings: [Square; 2],
    rooks: SquareSet,
    bishops: SquareSet,
    pawns: SquareSet,
    castling: Castling,
    en_passant: Option<Square>,
    mailbox: [Option<PieceType>; 64],
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const DELIMITER: &str = concat!("+---+---+---+---+---+---+---+---+", '\n');

        let mut mailbox = [' '; 64];
        for piece in PieceType::iter() {
            let mask = self.get(piece);
            let symbol = char::from(piece);

            for sq in mask.iter() {
                mailbox[sq] = if self.color(Color::White).is_set(sq) {
                    symbol.to_ascii_uppercase()
                } else {
                    symbol
                };
            }
        }

        let mut pos = String::from(DELIMITER);
        for row in (0..8).rev() {
            let start = row * 8;

            let mut rank = String::new();
            for c in &mailbox[start..(start + 8)] {
                rank.push_str(&format!("| {} ", c));
            }

            pos.push_str(&format!("{}| {}\n{}", rank, row + 1, DELIMITER));
        }

        pos.push_str("  a   b   c   d   e   f   g   h");
        write!(f, "{pos}")
    }
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
        mailbox: [None; 64],
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

    pub fn piece_at(&self, sq: Square) -> PieceType {
        if let Some(piece) = self.mailbox[sq] {
            return piece;
        }

        unreachable!()
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

    pub fn in_check(&self, stm: Color) -> bool {
        self.attacked(self.kings[stm], stm, self.all())
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

    fn toggle(&mut self, sq: Square, color: Color, piece: PieceType) {
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
        }

        self.mailbox[sq] = match self.mailbox[sq] {
            Some(_) => None,
            None => Some(piece),
        };
    }

    fn attacked(&self, sq: Square, stm: Color, occ: SquareSet) -> bool {
        macro_rules! attacked_by {
            ($pieces:expr, $attacks:expr) => {
                let pieces = $pieces & self.color(!stm);
                if !($attacks & pieces).is_empty() {
                    return true;
                }
            };
        }

        // We check if there exists an intersection between the given piece-type attacks from `sq`,
        // and their corresponding pieces on the board. We calculate the attacks based on `sq` to
        // avoid iterating over every piece, as we are only interested in pieces, which attack `sq`
        attacked_by!(self.pawns(), attacks::pawn(stm, sq));
        attacked_by!(self.knights(), attacks::knight(sq));
        attacked_by!(self.bishops, attacks::bishop(sq, occ));
        attacked_by!(self.rooks, attacks::rook(sq, occ));
        attacked_by!(self.kings[!stm].set(), attacks::king(sq));

        false
    }
}
