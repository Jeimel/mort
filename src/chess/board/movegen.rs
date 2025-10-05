use types::{Color, Move, MoveFlag, MoveList, PieceType, Rank, Square, SquareSet};

use crate::chess::attacks;

use super::Board;

macro_rules! push_loop {
    ($moves:expr, $set:expr, $start:expr, $flag:expr) => {
        for target in $set.iter() {
            $moves.push(Move::new($start, target, $flag));
        }
    };
}

impl Board {
    pub(in crate::chess) fn make_move(&mut self, mov: Move, stm: Color) -> bool {
        const PAWN_SHIFT: [i8; 2] = [1, -1];

        const CASTLING_QUEEN_START: [Square; 2] = [Square::A1, Square::A8];
        const CASTLING_KING_START: [Square; 2] = [Square::H1, Square::H8];

        const CASTLING_QUEEN_TARGET: [Square; 2] = [Square::D1, Square::D8];
        const CASTLING_KING_TARGET: [Square; 2] = [Square::F1, Square::F8];

        let start = mov.start();
        let target = mov.target();
        let flag = mov.flag();

        let piece = self.piece(start);

        self.en_passant = None;
        self.castling.remove(start, target);

        match flag {
            // Store en passant target square for next turn
            MoveFlag::DOUBLE_PAWN => self.en_passant = start.try_delta_rank(PAWN_SHIFT[stm]),
            MoveFlag::QUEEN_CASTLE => {
                self.toggle(CASTLING_QUEEN_START[stm], stm, PieceType::Rook);
                self.toggle(CASTLING_QUEEN_TARGET[stm], stm, PieceType::Rook);
            }
            MoveFlag::KING_CASTLE => {
                self.toggle(CASTLING_KING_START[stm], stm, PieceType::Rook);
                self.toggle(CASTLING_KING_TARGET[stm], stm, PieceType::Rook);
            }
            // Remove their piece from the board
            MoveFlag::CAPTURE => self.toggle(target, !stm, self.piece(target)),
            // Remove their captured pawn
            MoveFlag::EN_PASSANT => self.toggle(
                target.try_delta_rank(PAWN_SHIFT[!stm]).unwrap(),
                !stm,
                PieceType::Pawn,
            ),
            _ => {}
        };

        // Remove our piece from the current square
        self.toggle(start, stm, piece);

        // Determine which piece must be placed on the target square
        let piece = match mov.flag().promotion_piece() {
            // We promote our piece
            Some(piece) => piece,
            // We just move our piece to the target square
            None => piece,
        };

        // We captured their piece to promote ours
        if self.all().is_set(target) {
            self.toggle(target, !stm, self.piece(target));
        }

        // Add our new piece back on the board
        self.toggle(target, stm, piece);

        self.in_check(stm)
    }

    pub(in crate::chess) fn gen_moves(&self, stm: Color) -> MoveList {
        // We set the capacity to the average branching factor, which reduces uncessary allocations
        // without occupying too much memory
        let mut moves = MoveList::with_capacity(35);

        self.add_pawns(stm, &mut moves);

        for piece in [
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Rook,
            PieceType::Queen,
            PieceType::King,
        ] {
            self.add_attacks(piece, stm, &mut moves);
        }

        moves
    }

    fn add_pawns(&self, color: Color, moves: &mut MoveList) {
        const ROTATION: [u32; 2] = [8, 56];

        const PROMOTION_RANK: [SquareSet; 2] = [Rank::Eight.set(), Rank::One.set()];
        const PRE_PROMOTION_RANK: [SquareSet; 2] = [Rank::Seven.set(), Rank::Two.set()];

        let occ = self.all();
        let pawns = self.get(PieceType::Pawn) & self.color(color);

        for start in pawns.iter() {
            // The intersection between our attacks and their pieces yields all captures
            let captures = attacks::pawn(color, start) & self.color(!color);

            // Captures and promotions within a single move are a special case, which we can filter
            // through the intersection between all captures and the respective last rank
            let promo_captures = captures & PROMOTION_RANK[color];

            // We don't consider captures and promotions here, so we remove
            // all captures on the respective last rank
            let captures = captures - PROMOTION_RANK[color];
            push_loop!(moves, captures, start, MoveFlag::CAPTURE);

            // Now, we consider promotions. There exist two cases:
            // 1. Quiet promotion: All pawns on the penulimate rank,
            //    which are not blocked by a piece in front.
            // 2. Capture & Promotion: All pawns, which capture one of
            //    their pieces on the last rank.

            // The moves for the first case are generated via single rank shift
            let promo = (start.set() & PRE_PROMOTION_RANK[color]).rotate(ROTATION[color]) - occ;

            // We can promote our pawn to a knight, bishop, rook, or queen. Therefore, we have to
            // generate each possible piece and target square combination
            for piece in [
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
            ] {
                let flag = MoveFlag::promotion(piece);

                push_loop!(moves, promo, start, flag);
                push_loop!(moves, promo_captures, start, flag);
            }

            // There exists an en passant capture if we have a valid target square,
            // which our pawn can attack
            if let Some(target) = self.en_passant
                && !(target.set() & attacks::pawn(color, start)).is_empty()
            {
                moves.push(Move::new(start, target, MoveFlag::EN_PASSANT));
            }

            // We have to exclude all promotions pawns, as those are already calculated. To
            // calculate the push, we just rotate the square set in the corresponding direction,
            // and remove all pushes, which advance on a occupied squaure
            let single = (start.set() - PRE_PROMOTION_RANK[color]).rotate(ROTATION[color]) - occ;
            push_loop!(moves, single, start, MoveFlag::QUIET);

            // We can early return if there exists no single push,
            // because it can be seen as precondition for a double push
            if single.is_empty() {
                continue;
            }

            // A pawn can only advance two squares, if present on the enemey penultimate promotion
            // rank, which we can filter out. Next, we double the shift, as we advance two
            // squares, and again, remove occupied squares
            let double =
                (start.set() & PRE_PROMOTION_RANK[!color]).rotate(2 * ROTATION[color]) - occ;
            push_loop!(moves, double, start, MoveFlag::DOUBLE_PAWN);
        }
    }

    fn add_attacks(&self, piece: PieceType, color: Color, moves: &mut MoveList) {
        let pieces = self.get(piece) & self.color(color);
        let occ = self.all();

        for start in pieces.iter() {
            let attacks = match piece {
                PieceType::Knight => attacks::knight(start),
                PieceType::Bishop => attacks::bishop(start, occ),
                PieceType::Rook => attacks::rook(start, occ),
                PieceType::Queen => attacks::rook(start, occ) | attacks::bishop(start, occ),
                PieceType::King => attacks::king(start),
                _ => unreachable!(),
            };

            // The intersection between our attacks and their pieces yields all captures
            let captures = attacks & self.color(!color);
            push_loop!(moves, captures, start, MoveFlag::CAPTURE);

            // The difference between our attacks and all blockers yields all quiet moves
            let quiets = attacks - occ;
            push_loop!(moves, quiets, start, MoveFlag::QUIET);
        }
    }
}
