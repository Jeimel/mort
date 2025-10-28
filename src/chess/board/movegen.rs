use types::{Color, Move, MoveFlag, MoveList, PieceType, Rank, Square, SquareSet};

use crate::chess::{attacks, board::Board};

include!(concat!(env!("OUT_DIR"), "/squareset_tables.rs"));

macro_rules! push_loop {
    ($moves:expr, $set:expr, $start:expr, $flag:expr) => {
        for target in $set.iter() {
            $moves.push(Move::new($start, target, $flag));
        }
    };
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
        (self.threat.blockers(color) & start.set()).is_empty()
            || !(LINE[start][target] & self.layout.kings[color].set()).is_empty()
    }

    /// Generatae all pseudo-legal moves given the current position.
    ///
    /// `QUIET` determines whether to include quiet moves or not
    #[inline(always)]
    pub fn generate<const QUIET: bool>(&self, moves: &mut MoveList, color: Color) {
        let checkers = self.threat.checkers();

        // Is our king not in check?
        match checkers.is_empty() {
            true => self.generate_all::<false, QUIET>(moves, color, checkers),
            false => self.generate_all::<true, QUIET>(moves, color, checkers),
        }
    }

    #[inline(always)]
    fn generate_all<const EVADING: bool, const QUIET: bool>(
        &self,
        moves: &mut MoveList,
        color: Color,
        checkers: SquareSet,
    ) {
        let occ = self.layout.all();

        let mut target = SquareSet::EMPTY;

        // We only have to generate non-king moves if our king is not in double check
        if !EVADING || checkers.is_less_two() {
            target = match EVADING {
                true => BETWEEN[self.layout.kings[color]][checkers.index_lsb() as usize],
                false => !self.layout.color(color),
            };

            self.generate_pawns::<QUIET>(moves, color, target, occ);

            self.generate_attacks::<QUIET>(
                moves,
                color,
                PieceType::Knight,
                attacks::knight,
                target,
                occ,
            );
            self.generate_attacks::<QUIET>(
                moves,
                color,
                PieceType::Bishop,
                attacks::bishop,
                target,
                occ,
            );
            self.generate_attacks::<QUIET>(
                moves,
                color,
                PieceType::Rook,
                attacks::rook,
                target,
                occ,
            );
            self.generate_attacks::<QUIET>(
                moves,
                color,
                PieceType::Queen,
                attacks::queen,
                target,
                occ,
            );
        }

        // Is `target` potentially representing the line between our king and the threatening piece?
        if EVADING {
            // Reset `target` so our king is moving away from the threat
            target = !self.layout.color(color);
        }

        self.generate_attacks::<QUIET>(moves, color, PieceType::King, attacks::king, target, occ);

        // We can't castle, if either our king is in check or we already did it
        if QUIET && !EVADING && !self.state.castling.is_empty(color) {
            self.generate_castling(moves, color, occ);
        }
    }

    #[inline(always)]
    fn generate_pawns<const QUIET: bool>(
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
            let captures = captures - PROMOTION_RANK[color];
            push_loop!(moves, captures & target, start, MoveFlag::CAPTURE);

            // Now, we consider promotions. There exist two cases:
            // 1. Quiet promotion: All pawns on the penulimate rank,
            //    which are not blocked by a piece in front.
            // 2. Capture & Promotion: All pawns, which capture one of
            //    their pieces on the last rank.

            // The moves for the first case are generated via single rank shift
            let promo = (set & PRE_PROMOTION_RANK[color]).rotate(ROTATION[color]) - occ;

            // We can promote our pawn to a knight, bishop, rook, or queen. Therefore, we have to
            // generate each possible piece and target square combination
            //
            // NOTE: Currently all promotions are considered non-quiet
            for piece in [
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
            ] {
                let flag = MoveFlag::promotion(piece);

                push_loop!(moves, promo & target, start, flag);
                push_loop!(moves, promo_captures & target, start, flag);
            }

            // There exists an en passant capture if we have a valid target square,
            // which our pawn can attack
            if let Some(target) = self.state.en_passant
                && !(target.set() & attacks::pawn(color, start)).is_empty()
            {
                moves.push(Move::new(start, target, MoveFlag::EN_PASSANT));
            }

            if !QUIET {
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
    fn generate_attacks<const QUIET: bool>(
        &self,
        moves: &mut MoveList,
        color: Color,
        piece: PieceType,
        attacks: fn(Square, SquareSet) -> SquareSet,
        target: SquareSet,
        occ: SquareSet,
    ) {
        let pieces = self.layout.get(piece) & self.layout.color(color);

        for start in pieces.iter() {
            let attacks = attacks(start, occ);

            // The intersection between our attacks and their pieces yields all captures
            let captures = attacks & self.layout.color(!color);
            push_loop!(moves, captures & target, start, MoveFlag::CAPTURE);

            if !QUIET {
                continue;
            }

            // The difference between our attacks and all blockers yields all quiet moves
            let quiets = attacks - occ;
            push_loop!(moves, quiets & target, start, MoveFlag::QUIET);
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
