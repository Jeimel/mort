use types::{Color, PieceType, Square, SquareSet};

use crate::chess::attacks;

#[derive(Clone)]
pub struct PieceLayout {
    pub colors: [SquareSet; 2],
    pub kings: [Square; 2],
    pub rooks: SquareSet,
    pub bishops: SquareSet,
    pub pawns: SquareSet,
}

impl PieceLayout {
    pub const EMPTY: Self = Self {
        colors: [SquareSet::EMPTY; 2],
        kings: [Square::A1; 2],
        rooks: SquareSet::EMPTY,
        bishops: SquareSet::EMPTY,
        pawns: SquareSet::EMPTY,
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

    pub(crate) fn attacked(&self, sq: Square, stm: Color, occ: SquareSet) -> bool {
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
        }
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
