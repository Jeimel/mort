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

// Index precomputed diagonal attacks for pawns.
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
