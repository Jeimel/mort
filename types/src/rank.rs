use crate::BitBoard;

#[repr(u8)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    pub const fn new(index: u8) -> Option<Self> {
        if index < 8 {
            // Safety: `index` has a corresponding `Rank` variant
            Some(unsafe { std::mem::transmute(index) })
        } else {
            None
        }
    }

    pub const fn bitboard(self) -> BitBoard {
        BitBoard(0xffu64 << (self as u8 * 8))
    }
}
