use super::Position;

use types::const_for;

use crate::rng::XorShiftState;

pub type Key = u64;

impl Position {
    pub fn zobrist(&self) -> Key {
        let mut zobrist = self.zobrist;

        if let Some(en_passant) = self.en_passant {
            zobrist ^= EN_PASSANT[en_passant.file()];
        }

        zobrist ^ CASTLING[self.castling]
    }
}

const VALUES: ([[[Key; 64]; 6]; 2], [Key; 8], [Key; 16], Key) = {
    let mut rng = XorShiftState::new(1070372);

    let (mut piece, mut en_passant, mut castling) = ([[[0; 64]; 6]; 2], [0; 8], [0; 16]);

    const_for!(let mut i = 0; i < 2; i += 1; {
        const_for!(let mut j = 0; j < 6; j += 1; {
            const_for!(let mut k = 0; k < 64; k += 1; {
                (rng.state, piece[i][j][k]) = rng.next();
            });
        });
    });

    const_for!(let mut i = 0; i < 8; i += 1; {
        (rng.state, en_passant[i]) = rng.next();
    });

    const_for!(let mut i = 0; i < 16; i += 1; {
        (rng.state, castling[i]) = rng.next();
    });

    (piece, en_passant, castling, rng.next().1)
};

pub const PIECE: &[[[Key; 64]; 6]; 2] = &VALUES.0;
pub const EN_PASSANT: &[Key; 8] = &VALUES.1;
pub const CASTLING: &[Key; 16] = &VALUES.2;
pub const SIDE: &Key = &VALUES.3;
