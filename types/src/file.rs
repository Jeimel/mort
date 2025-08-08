use crate::SquareSet;

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

    pub const fn try_delta(self, delta: i8) -> Option<Self> {
        let index = self as i8 + delta;
        if index < 0 || index >= 8 {
            return None;
        }

        Self::new(index as u8)
    }

    pub const fn set(self) -> SquareSet {
        SquareSet(0x101010101010101u64 << (self as u8))
    }
}
