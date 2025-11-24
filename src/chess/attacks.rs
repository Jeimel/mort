use types::{
    Color, File, PieceType, Square, SquareSet,
    magic::{bishop_magic_index, rook_magic_index},
};

include!(concat!(env!("OUT_DIR"), "/sliding_moves.rs"));

macro_rules! gen_lookup {
    (| $set:ident | $($intern:expr)+) => {{
        let mut $set = 1u64;
        let mut index = 0;
        let mut attacks = [$($intern)+; 64];

        while index != 63  {
            $set = $set << 1;
            index += 1;

            attacks[index] = $($intern)+;
        }

        attacks
    }};
}

pub fn by_type(piece: PieceType, sq: Square, occ: SquareSet) -> SquareSet {
    match piece {
        PieceType::Knight => knight(sq),
        PieceType::Bishop => bishop(sq, occ),
        PieceType::Rook => rook(sq, occ),
        PieceType::Queen => rook(sq, occ) | bishop(sq, occ),
        PieceType::King => king(sq),
        _ => unreachable!(),
    }
}

pub fn const_by_type<const PIECE: PieceType>(sq: Square, occ: SquareSet) -> SquareSet {
    match PIECE {
        PieceType::Knight => knight(sq),
        PieceType::Bishop => bishop(sq, occ),
        PieceType::Rook => rook(sq, occ),
        PieceType::Queen => rook(sq, occ) | bishop(sq, occ),
        PieceType::King => king(sq),
        _ => unreachable!(),
    }
}

/// Index precomputed diagonal attacks for pawns
pub fn pawn(stm: Color, sq: Square) -> SquareSet {
    const ATTACKS: [[SquareSet; 64]; 2] = [
        gen_lookup!(|set| SquareSet(
            ((set << 9) & !File::A.set().0) | ((set << 7) & !File::H.set().0)
        )),
        gen_lookup!(|set| SquareSet(
            ((set >> 7) & !File::A.set().0) | ((set >> 9) & !File::H.set().0)
        )),
    ];

    ATTACKS[stm][sq]
}

/// Index precomputed attacks for knights
pub fn knight(sq: Square) -> SquareSet {
    const ATTACKS: [SquareSet; 64] = gen_lookup!(|set| {
        let l1 = (set >> 1) & !File::H.set().0;
        let l2 = (set >> 2) & !File::H.set().0 & !File::G.set().0;
        let r1 = (set << 1) & !File::A.set().0;
        let r2 = (set << 2) & !File::A.set().0 & !File::B.set().0;

        let h1 = l1 | r1;
        let h2 = l2 | r2;

        SquareSet((h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8))
    });

    ATTACKS[sq]
}

/// Index shared precomputed attacks for bishops based on magic index
pub fn bishop(sq: Square, blockers: SquareSet) -> SquareSet {
    SLIDING_MOVES[bishop_magic_index(sq, blockers)]
}

/// Index shared precomputed attacks for rooks based on magic index
pub fn rook(sq: Square, blockers: SquareSet) -> SquareSet {
    SLIDING_MOVES[rook_magic_index(sq, blockers)]
}

/// Index precomputed attacks in all eight directions for the king
pub fn king(sq: Square) -> SquareSet {
    const ATTACKS: [SquareSet; 64] = gen_lookup!(|set| {
        let attacks = ((set << 1) & !File::A.set().0) | ((set >> 1) & !File::H.set().0);
        let king = set | attacks;

        SquareSet(attacks | (king << 8) | (king >> 8))
    });

    ATTACKS[sq]
}
