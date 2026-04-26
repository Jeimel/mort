use types::const_for;

use crate::util::XorShiftState;

pub type Key = u64;

const VALUES: ([[[Key; 64]; 6]; 2], [Key; 8], [Key; 16], Key) = {
    const SEED: u64 = 1070372;

    let mut rng = XorShiftState::new(SEED);
    let mut zobrist = [0; 64 * 12 + 8 + 16 + 1];

    const_for!(let mut i = 0; i < zobrist.len(); i += 1; {
        (rng.state, zobrist[i]) = rng.next();
    });

    unsafe { std::mem::transmute(zobrist) }
};

pub const PIECE: &[[[Key; 64]; 6]; 2] = &VALUES.0;
pub const EN_PASSANT: &[Key; 8] = &VALUES.1;
pub const CASTLING: &[Key; 16] = &VALUES.2;
pub const SIDE: &Key = &VALUES.3;
