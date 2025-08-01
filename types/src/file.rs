use crate::BitBoard;

#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub const fn new(index: u8) -> Option<Self> {
        if index < 8 {
            // Safety: `index` has a corresponding `File` variant
            Some(unsafe { std::mem::transmute(index) })
        } else {
            None
        }
    }

    pub const fn bitboard(self) -> BitBoard {
        BitBoard(0x101010101010101u64 << (self as u8))
    }
}
