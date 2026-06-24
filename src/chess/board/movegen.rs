use types::{Color, Move, MoveFlag, PieceType, Rank, Square, SquareSet};

use crate::{
    chess::{
        MoveList, attacks,
        board::{BETWEEN, Board},
    },
    push_loop,
};

pub trait GenerationType {
    const QUIET: bool;
    const CAPTURE: bool;
}

pub struct Quiet {}

impl GenerationType for Quiet {
    const QUIET: bool = true;
    const CAPTURE: bool = false;
}

pub struct Capture {}

impl GenerationType for Capture {
    const QUIET: bool = false;
    const CAPTURE: bool = true;
}

pub struct All {}

impl GenerationType for All {
    const QUIET: bool = true;
    const CAPTURE: bool = true;
}

impl Board {
    pub const PAWN_ROTATION: [u32; 2] = [8, 56];

    pub const DOUBLE_PUSH: [SquareSet; 2] = [Rank::Two.set(), Rank::Seven.set()];

    /// Generatae all pseudo-legal moves given the current position.
    #[inline(always)]
    pub fn generate<TYPE: GenerationType>(&self, moves: &mut MoveList, color: Color) {
        let checkers = self.state.checkers;

        // Is our king not in check?
        match checkers.is_empty() {
            true => self.generate_all::<false, TYPE>(moves, color, checkers),
            false => self.generate_all::<true, TYPE>(moves, color, checkers),
        }
    }

    #[inline(always)]
    fn generate_all<const EVADING: bool, TYPE: GenerationType>(
        &self,
        moves: &mut MoveList,
        color: Color,
        checkers: SquareSet,
    ) {
        let occ = self.layout.all();

        let mut target = SquareSet::EMPTY;

        // We only have to generate non-king moves if our king is not in double check
        if !EVADING || !checkers.many() {
            debug_assert!(checkers.popcnt() < 2);

            target = match EVADING {
                true => BETWEEN[self.layout.king(color)][checkers.index_lsb() as usize],
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
        if TYPE::QUIET && !EVADING && !self.state.castling.is_empty(color) {
            self.generate_castling(moves, color, occ);
        }
    }

    #[inline(always)]
    fn generate_pawns<TYPE: GenerationType>(
        &self,
        moves: &mut MoveList,
        color: Color,
        target: SquareSet,
        occ: SquareSet,
    ) {
        const PROMOTION_RANK: [SquareSet; 2] = [Rank::Eight.set(), Rank::One.set()];

        for start in (self.layout.get(PieceType::Pawn) & self.layout.color(color)).iter() {
            let set = start.set();

            // The intersection between our attacks and their pieces yields all captures
            let captures = attacks::pawn(color, start) & self.layout.color(!color);

            // Captures and promotions within a single move are a special case, which we can filter
            // through the intersection between all captures and the respective last rank
            let promo_captures = captures & PROMOTION_RANK[color];

            // We don't consider captures and promotions here, so we remove
            // all captures on the respective last rank
            if TYPE::CAPTURE {
                let captures = captures - PROMOTION_RANK[color];
                push_loop!(moves, captures & target, start, MoveFlag::CAPTURE);
            }

            // Now, we consider promotions. There exist two cases:
            // 1. Quiet promotion: All pawns on the penulimate rank,
            //    which are not blocked by a piece in front.
            // 2. Capture & Promotion: All pawns, which capture one of
            //    their pieces on the last rank.

            // The moves for the first case are generated via single rank shift
            let promo = (set & Self::DOUBLE_PUSH[!color]).rotate(Self::PAWN_ROTATION[color]) - occ;

            // We consider both quiet and capture promotions for queens in quiescence search
            let promo_flag = MoveFlag::new_promotion(PieceType::Queen);
            push_loop!(moves, promo & target, start, promo_flag);
            push_loop!(moves, promo_captures & target, start, promo_flag);

            // We have to generate each possible piece and target square combination
            for piece in [PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                let flag = MoveFlag::new_promotion(piece);

                if TYPE::CAPTURE {
                    push_loop!(moves, promo_captures & target, start, flag);
                }

                if TYPE::QUIET {
                    push_loop!(moves, promo & target, start, flag);
                }
            }

            // There exists an en passant capture if we have a valid target square,
            // which our pawn can attack
            if TYPE::CAPTURE
                && let Some(target) = self.state.en_passant
                && !(target.set() & attacks::pawn(color, start)).is_empty()
            {
                moves.push(Move::new(start, target, MoveFlag::EN_PASSANT));
            }

            if !TYPE::QUIET {
                continue;
            }

            // We have to exclude all promoting pawns, as those are already calculated.
            // To calculate the push, we just rotate the square set in the corresponding
            // direction, and remove all pushes, which advance to an occupied square
            let single = (set - Self::DOUBLE_PUSH[!color]).rotate(Self::PAWN_ROTATION[color]) - occ;
            push_loop!(moves, single & target, start, MoveFlag::QUIET);

            // If the path for a single push is blocked, we cannot double push our pawn
            if single.is_empty() {
                continue;
            }

            // A pawn can only advance two squares, if present on the enemey penultimate
            // promotion rank, which we can filter out. Next, we double the shift, as we
            // advance two squares, and again, remove occupied squares
            let double = (set & Self::DOUBLE_PUSH[color]).rotate(2 * Self::PAWN_ROTATION[color]);
            push_loop!(moves, (double - occ) & target, start, MoveFlag::DOUBLE_PAWN);
        }
    }

    #[inline(always)]
    fn generate_attacks<TYPE: GenerationType, const PIECE: PieceType>(
        &self,
        moves: &mut MoveList,
        color: Color,
        target: SquareSet,
        occ: SquareSet,
    ) {
        let pieces = self.layout.get(PIECE) & self.layout.color(color);

        for start in pieces.iter() {
            let attacks = attacks::const_by_type::<PIECE>(start, occ);

            // The intersection between our attacks and their pieces yields all captures
            if TYPE::CAPTURE {
                let captures = attacks & self.layout.color(!color);
                push_loop!(moves, captures & target, start, MoveFlag::CAPTURE);
            }

            // The difference between our attacks and all blockers yields all quiet moves
            if TYPE::QUIET {
                let quiets = attacks - occ;
                push_loop!(moves, quiets & target, start, MoveFlag::QUIET);
            }
        }
    }

    #[inline(always)]
    fn generate_castling(&self, moves: &mut MoveList, color: Color, occ: SquareSet) {
        const KING_TARGET: [Square; 2] = [Square::G1, Square::G8];
        const QUEEN_TARGET: [Square; 2] = [Square::C1, Square::C8];

        let king = self.layout.king(color);

        if self.state.castling.pseudo_kingside(color, occ) {
            moves.push(Move::new(king, KING_TARGET[color], MoveFlag::KING_CASTLE));
        }

        if self.state.castling.pseudo_queenside(color, occ) {
            moves.push(Move::new(king, QUEEN_TARGET[color], MoveFlag::QUEEN_CASTLE));
        }
    }
}
