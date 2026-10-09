use crate::chess::{
    All, Color, Move, MoveFlag, MoveList, PieceType, SquareSet, attacks,
    board::{BETWEEN, Board, LINE},
};

impl Board {
    pub fn pseudo_legal(&self, mov: Move, color: Color) -> bool {
        let flag = mov.flag();

        if !flag.normal() {
            let mut moves = MoveList::new();
            self.generate::<All>(&mut moves, color);

            return moves.iter().any(|other| other.inner() == mov.inner());
        }

        let start = mov.start();

        // Does the piece even exists?
        let Some(piece) = self.layout.at(start) else {
            return false;
        };

        let target = mov.target();

        // Is the piece our color and is `target` not occupied by a friendly piece?
        if piece.color() != color || self.layout.color(color).is_set(target) {
            return false;
        }

        let checkers = self.state.checkers;

        if piece.typ() != PieceType::King && !checkers.is_empty() {
            // Only the king can evade a double check
            if !checkers.is_less_two() {
                return false;
            }

            // We either block or capture the checker
            if !BETWEEN[self.layout.king(color)][checkers.index_lsb() as usize].is_set(target) {
                return false;
            }
        }

        let capture = self.layout.at(target);

        if piece.typ() != PieceType::Pawn {
            return match capture {
                Some(_) => flag == MoveFlag::CAPTURE,
                None => flag == MoveFlag::QUIET,
            } && attacks::by_type(piece.typ(), start, self.layout.all()).is_set(target);
        }

        // A pawn on the last rank always promotes, and promotions took the shortcut
        if Self::PROMOTION_RANK[color].is_set(target) {
            return false;
        }

        let push = start.set().rotate(Self::PAWN_ROTATION[color]);

        // A single forward push onto an empty square
        if push.is_set(target) {
            return capture.is_none() && flag == MoveFlag::QUIET;
        }

        // A double push from the starting rank with both squares empty
        if flag == MoveFlag::DOUBLE_PAWN {
            return capture.is_none()
                && Self::DOUBLE_PUSH[color].is_set(start)
                && (push & self.layout.all()).is_empty()
                && push.rotate(Self::PAWN_ROTATION[color]).is_set(target);
        }

        // Otherwise a diagonal capture onto an enemy piece
        flag == MoveFlag::CAPTURE && capture.is_some() && attacks::pawn(color, start).is_set(target)
    }

    #[inline(always)]
    pub fn legal(&self, mov: Move, color: Color) -> bool {
        const KING_PATH: [SquareSet; 2] = [SquareSet(0b0110_0000), SquareSet(0b0110_0000 << 56)];
        const QUEEN_PATH: [SquareSet; 2] = [SquareSet(0b0000_1100), SquareSet(0b0000_1100 << 56)];

        let occ = self.layout.all();

        if mov.flag() == MoveFlag::KING_CASTLE {
            return self.layout.attacked(KING_PATH[color], color, occ);
        }

        if mov.flag() == MoveFlag::QUEEN_CASTLE {
            #[rustfmt::skip]
            return self.layout.attacked(QUEEN_PATH[color], color, occ);
        }

        let start = mov.start();
        let target = mov.target();

        if mov.flag() == MoveFlag::EN_PASSANT {
            let king = self.layout.king(color);

            let capture = target.set().rotate([56, 8][color]);
            let occ = (self.layout.all() - start.set() - capture) | target.set();

            let rooks = attacks::rook(king, occ) & self.layout.orthogonal();
            let bishops = attacks::bishop(king, occ) & self.layout.diagonal();

            // Is our king in check after making the en passant capture?
            return ((rooks | bishops) & self.layout.color(!color)).is_empty();
        }

        // If the king moves, we must check if the target square is being attacked or not
        if self.layout.unchecked_at(start) == PieceType::King {
            return self
                .layout
                .attackers(target, color, occ - start.set())
                .is_empty();
        }

        // The start square must either not be a blocker of our king,
        // or the piece moves towards the threat
        (self.state.blockers & start.set()).is_empty()
            || !(LINE[start][target] & self.layout.king(color).set()).is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        FEN,
        chess::{All, MoveList, Position},
    };

    #[test]
    fn pseudo_legal() {
        for fen in FEN {
            let pos = Position::from_fen(fen).unwrap();

            let mut moves = MoveList::new();
            pos.generate::<All>(&mut moves);

            assert!(moves.iter().all(|mov| pos.pseudo_legal(mov)));
        }
    }
}
