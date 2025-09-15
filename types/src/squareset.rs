use std::ops::{BitAnd, BitOr, Sub};

use crate::Square;

/// A `SquareSet` represents a board as array of 64 bits.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct SquareSet(pub u64);

impl SquareSet {
    pub const EMPTY: SquareSet = Self(0);

    pub fn toggle(&mut self, sq: Square) {
        self.0 = self.0 ^ sq.set().0;
    }

    pub fn is_set(&self, sq: Square) -> bool {
        (self.0 & sq.set().0) != 0
    }

    pub fn iter(self) -> SquareIter {
        SquareIter::new(self)
    }

    pub fn iter_subset(self) -> SquareSubsetIter {
        SquareSubsetIter::new(self)
    }

    pub fn wrapping_sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0))
    }

    pub fn is_empty(self) -> bool {
        self == Self::EMPTY
    }

    pub fn index_lsb(self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    pub fn reset_lsb(self) -> Self {
        self & self.wrapping_sub(Self(1))
    }
}

/// Calculates the difference between two square sets.
impl Sub for SquareSet {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 & !rhs.0)
    }
}

/// Calculates the union between two square sets.
impl BitOr for SquareSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

/// Calculates the intersection between two square sets.
impl BitAnd for SquareSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

pub struct SquareIter {
    set: SquareSet,
}

impl SquareIter {
    pub const fn new(set: SquareSet) -> Self {
        Self { set }
    }
}

impl Iterator for SquareIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.set == SquareSet::EMPTY {
            return None;
        }

        let lsb = self.set.index_lsb();
        self.set = self.set.reset_lsb();

        Square::new(lsb)
    }
}

pub struct SquareSubsetIter {
    set: SquareSet,
    subset: SquareSet,
    finished: bool,
}

impl SquareSubsetIter {
    pub const fn new(set: SquareSet) -> Self {
        Self {
            set,
            subset: SquareSet::EMPTY,
            finished: false,
        }
    }
}

impl Iterator for SquareSubsetIter {
    type Item = SquareSet;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let current = self.subset;
        self.subset = self.subset.wrapping_sub(self.set) & self.set;
        self.finished = self.subset.is_empty();

        Some(current)
    }
}
