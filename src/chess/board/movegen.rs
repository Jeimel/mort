use std::marker::ConstParamTy;

use types::{Color, Move, MoveFlag, PieceType, Rank, Square, SquareSet};

use crate::chess::{MoveList, attacks, board::Board};

include!(concat!(env!("OUT_DIR"), "/squareset_tables.rs"));

macro_rules! push_loop {
    ($moves:expr, $set:expr, $start:expr, $flag:expr) => {
        for target in $set.iter() {
            $moves.push(Move::new($start, target, $flag));
        }
    };
}

#[derive(ConstParamTy, PartialEq, Eq)]
pub enum GenerationType {
    Quiet,
    Capture,
    All,
}

impl Board {
    #[inline(always)]
    pub fn legal(&self, mov: Move, color: Color) -> bool {
        const KING_PATH: [SquareSet; 2] = [SquareSet(0b0110_0000), SquareSet(0b0110_0000 << 56)];
        const QUEEN_PATH: [SquareSet; 2] = [SquareSet(0b0000_1100), SquareSet(0b0000_1100 << 56)];

        let occ = self.layout.all();

        if mov.flag() == MoveFlag::KING_CASTLE {
            return self.layout.attacked(KING_PATH[color], color, occ);
        }

        if mov.flag() == MoveFlag::QUEEN_CASTLE {
            return self.layout.attacked(QUEEN_PATH[color], color, occ);
        }

        let start = mov.start();
        let target = mov.target();

        if mov.flag() == MoveFlag::EN_PASSANT {
            let king = self.layout.kings[color];

            let capture = target.set().rotate([56, 8][color]);
            let occ = (self.layout.all() - start.set() - capture) | target.set();

            let rooks = attacks::rook(king, occ) & self.layout.rooks;
            let bishops = attacks::bishop(king, occ) & self.layout.bishops;

            // Is our king in check after making the en passant capture?
            return ((rooks | bishops) & self.layout.color(!color)).is_empty();
        }

        // If the king moves, we must check if the target square is being attacked or not
        if self.layout.piece_at(start) == PieceType::King {
            #[rustfmt::skip]
            return self.layout.attackers(target, color, occ - start.set()).is_empty();
        }

        // The start square must either not be a blocker of our king,
        // or the piece moves towards the threat
        (self.state.blockers & start.set()).is_empty()
            || !(LINE[start][target] & self.layout.kings[color].set()).is_empty()
    }

    /// Generatae all pseudo-legal moves given the current position.
    ///
    /// `QUIET` determines whether to include quiet moves or not
    #[inline(always)]
    pub fn generate<const TYPE: GenerationType>(&self, moves: &mut MoveList, color: Color) {
        let checkers = self.state.checkers;

        // Is our king not in check?
        match checkers.is_empty() {
            true => self.generate_all::<false, TYPE>(moves, color, checkers),
            false => self.generate_all::<true, TYPE>(moves, color, checkers),
        }
    }

    #[inline(always)]
    fn generate_all<const EVADING: bool, const TYPE: GenerationType>(
        &self,
        moves: &mut MoveList,
        color: Color,
        checkers: SquareSet,
    ) {
        let occ = self.layout.all();

        let mut target = SquareSet::EMPTY;

        // We only have to generate non-king moves if our king is not in double check
        if !EVADING || checkers.is_less_two() {
            debug_assert!(checkers.popcnt() < 2);

            target = match EVADING {
                true => BETWEEN[self.layout.kings[color]][checkers.index_lsb() as usize],
                false => !self.layout.color(color),
            };

            self.generate_pawns::<TYPE>(moves, color, target, occ);

            self.generate_attacks::<TYPE, { PieceType::Knight }>(moves, color, target, occ);
            self.generate_attacks::<TYPE, { PieceType::Bishop }>(moves, color, target, occ);
            self.generate_attacks::<TYPE, { PieceType::Rook }>(moves, color, target, occ);
            self.generate_attacks::<TYPE, { PieceType::Queen }>(moves, color, target, occ);
        }

        // Is `target` potentially representing the line between our king and the threatening piece?
        if EVADING {
            // Reset `target` so our king is moving away from the threat
            target = !self.layout.color(color);
        }

        self.generate_attacks::<TYPE, { PieceType::King }>(moves, color, target, occ);

        // We can't castle, if either our king is in check or we already did it
        if matches!(TYPE, GenerationType::All | GenerationType::Quiet)
            && !EVADING
            && !self.state.castling.is_empty(color)
        {
            self.generate_castling(moves, color, occ);
        }
    }

    #[inline(always)]
    fn generate_pawns<const TYPE: GenerationType>(
        &self,
        moves: &mut MoveList,
        color: Color,
        target: SquareSet,
        occ: SquareSet,
    ) {
        const ROTATION: [u32; 2] = [8, 56];

        const PROMOTION_RANK: [SquareSet; 2] = [Rank::Eight.set(), Rank::One.set()];
        const PRE_PROMOTION_RANK: [SquareSet; 2] = [Rank::Seven.set(), Rank::Two.set()];

        for start in (self.layout.get(PieceType::Pawn) & self.layout.color(color)).iter() {
            let set = start.set();

            // The intersection between our attacks and their pieces yields all captures
            let captures = attacks::pawn(color, start) & self.layout.color(!color);

            // Captures and promotions within a single move are a special case, which we can filter
            // through the intersection between all captures and the respective last rank
            let promo_captures = captures & PROMOTION_RANK[color];

            // We don't consider captures and promotions here, so we remove
            // all captures on the respective last rank
            if matches!(TYPE, GenerationType::All | GenerationType::Capture) {
                let captures = captures - PROMOTION_RANK[color];
                push_loop!(moves, captures & target, start, MoveFlag::CAPTURE);
            }

            // Now, we consider promotions. There exist two cases:
            // 1. Quiet promotion: All pawns on the penulimate rank,
            //    which are not blocked by a piece in front.
            // 2. Capture & Promotion: All pawns, which capture one of
            //    their pieces on the last rank.

            // The moves for the first case are generated via single rank shift
            let promo = (set & PRE_PROMOTION_RANK[color]).rotate(ROTATION[color]) - occ;

            // We consider both quiet and capture promotions for queens in quiescence search
            let promo_flag = MoveFlag::promotion(PieceType::Queen);
            push_loop!(moves, promo & target, start, promo_flag);
            push_loop!(moves, promo_captures & target, start, promo_flag);

            // We have to generate each possible piece and target square combination
            for piece in [PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                let flag = MoveFlag::promotion(piece);

                if matches!(TYPE, GenerationType::All | GenerationType::Capture) {
                    push_loop!(moves, promo_captures & target, start, flag);
                }

                if matches!(TYPE, GenerationType::All | GenerationType::Quiet) {
                    push_loop!(moves, promo & target, start, flag);
                }
            }

            // There exists an en passant capture if we have a valid target square,
            // which our pawn can attack
            if matches!(TYPE, GenerationType::All | GenerationType::Capture)
                && let Some(target) = self.state.en_passant
                && !(target.set() & attacks::pawn(color, start)).is_empty()
            {
                moves.push(Move::new(start, target, MoveFlag::EN_PASSANT));
            }

            if TYPE == GenerationType::Capture {
                continue;
            }

            // We have to exclude all promoting pawns, as those are already calculated.
            // To calculate the push, we just rotate the square set in the corresponding
            // direction, and remove all pushes, which advance to an occupied square
            let single = (set - PRE_PROMOTION_RANK[color]).rotate(ROTATION[color]) - occ;
            push_loop!(moves, single & target, start, MoveFlag::QUIET);

            // If the path for a single push is blocked, we cannot double push our pawn
            if single.is_empty() {
                continue;
            }

            // A pawn can only advance two squares, if present on the enemey penultimate
            // promotion rank, which we can filter out. Next, we double the shift, as we
            // advance two squares, and again, remove occupied squares
            let double = (set & PRE_PROMOTION_RANK[!color]).rotate(2 * ROTATION[color]) - occ;
            push_loop!(moves, double & target, start, MoveFlag::DOUBLE_PAWN);
        }
    }

    #[inline(always)]
    fn generate_attacks<const TYPE: GenerationType, const PIECE: PieceType>(
        &self,
        moves: &mut MoveList,
        color: Color,
        target: SquareSet,
        occ: SquareSet,
    ) {
        let pieces = self.layout.get(PIECE) & self.layout.color(color);

        for start in pieces.iter() {
            let attacks = match PIECE {
                PieceType::Knight => attacks::knight(start),
                PieceType::Bishop => attacks::bishop(start, occ),
                PieceType::Rook => attacks::rook(start, occ),
                PieceType::Queen => attacks::rook(start, occ) | attacks::bishop(start, occ),
                PieceType::King => attacks::king(start),
                _ => unreachable!(),
            };

            // The intersection between our attacks and their pieces yields all captures
            if matches!(TYPE, GenerationType::All | GenerationType::Capture) {
                let captures = attacks & self.layout.color(!color);
                push_loop!(moves, captures & target, start, MoveFlag::CAPTURE);
            }

            // The difference between our attacks and all blockers yields all quiet moves
            if matches!(TYPE, GenerationType::All | GenerationType::Quiet) {
                let quiets = attacks - occ;
                push_loop!(moves, quiets & target, start, MoveFlag::QUIET);
            }
        }
    }

    #[inline(always)]
    fn generate_castling(&self, moves: &mut MoveList, color: Color, occ: SquareSet) {
        const KING_MASK: [SquareSet; 2] = [SquareSet(0b0110_0000), SquareSet(0b0110_0000 << 56)];
        const QUEEN_MASK: [SquareSet; 2] = [SquareSet(0b0000_1110), SquareSet(0b0000_1110 << 56)];

        const KING_TARGET: [Square; 2] = [Square::G1, Square::G8];
        const QUEEN_TARGET: [Square; 2] = [Square::C1, Square::C8];

        let king = self.layout.kings[color];

        if self.state.castling.kingside(color) && (occ & KING_MASK[color]).is_empty() {
            moves.push(Move::new(king, KING_TARGET[color], MoveFlag::KING_CASTLE));
        }

        if self.state.castling.queenside(color) && (occ & QUEEN_MASK[color]).is_empty() {
            moves.push(Move::new(king, QUEEN_TARGET[color], MoveFlag::QUEEN_CASTLE));
        }
    }
}
