use std::ops::{BitAnd, BitOr, Not, Sub};

use crate::Square;

/// A `BitBoard` represents a board as array of 64 bits.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const EMPTY: BitBoard = BitBoard(0);

    pub fn iter(self) -> SquareIter {
        SquareIter::new(self)
    }
}

impl Sub<u64> for BitBoard {
    type Output = BitBoard;

    fn sub(self, rhs: u64) -> Self::Output {
        BitBoard(self.0 - rhs)
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

pub struct SquareIter(BitBoard);

impl SquareIter {
    pub const fn new(bb: BitBoard) -> Self {
        Self(bb)
    }
}

impl Iterator for SquareIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == BitBoard::EMPTY {
            return None;
        }

        let lsb = self.0.0.trailing_zeros() as u8;
        self.0 = self.0 & (self.0 - 1);

        Square::new(lsb)
    }
}
