use super::{
    BitBoard,
    types::{Color, File, Square},
};

macro_rules! gen_lookup {
    (| $bb:ident | $($intern:expr)+) => {{
        let mut $bb = 1u64;
        let mut index = 0;
        let mut attacks = [$($intern)+; 64];

        while index != 63  {
            $bb = $bb << 1;
            index += 1;

            attacks[index] = $($intern)+;
        }

        attacks
    }};
}

/// Index precomputed diagonal attacks for pawns.
pub fn pawn(stm: Color, sq: Square) -> BitBoard {
    const ATTACKS: [[BitBoard; 64]; 2] = [
        gen_lookup!(|bb| BitBoard(
            ((bb << 9) & !File::A.bitboard().0) | ((bb << 7) & !File::H.bitboard().0)
        )),
        gen_lookup!(|bb| BitBoard(
            ((bb >> 7) & !File::A.bitboard().0) | ((bb >> 9) & !File::H.bitboard().0)
        )),
    ];

    ATTACKS[stm][sq]
}

/// Index precomputed attacks for knights.
pub fn knight(sq: Square) -> BitBoard {
    const ATTACKS: [BitBoard; 64] = gen_lookup!(|bb| {
        let l1 = (bb >> 1) & !File::A.bitboard().0;
        let l2 = (bb >> 2) & !File::A.bitboard().0 & !File::B.bitboard().0;
        let r1 = (bb << 1) & !File::H.bitboard().0;
        let r2 = (bb << 2) & !File::H.bitboard().0 & !File::G.bitboard().0;

        let h1 = l1 | r1;
        let h2 = l2 | r2;

        BitBoard((h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8))
    });

    ATTACKS[sq]
}

/// Index precomputed attacks in all eight directions for the king.
pub fn king(sq: Square) -> BitBoard {
    const ATTACKS: [BitBoard; 64] = gen_lookup!(|bb| {
        let attacks = ((bb << 1) & !File::A.bitboard().0) | ((bb >> 1) & !File::H.bitboard().0);
        let king = bb | attacks;

        BitBoard(attacks | (king << 8) | (king >> 8))
    });

    ATTACKS[sq]
}
