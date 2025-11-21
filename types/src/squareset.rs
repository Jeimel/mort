use std::ops::{BitAnd, BitOr, Not, Sub};

use crate::Square;

/// Represents a board as array of 64 bits.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SquareSet(pub u64);

impl SquareSet {
    /// The empty [`SquareSet`].
    pub const EMPTY: Self = Self(0);

    /// Returns `true` if this [`SquareSet`] contains no squares.
    pub fn is_empty(self) -> bool {
        self == Self::EMPTY
    }

    /// Returns `true` if the given [`Square`] is contained in this [`SquareSet`].
    pub fn is_set(&self, sq: Square) -> bool {
        (self.0 & sq.set().0) != 0
    }

    /// Returns `true` if this [`SquareSet`] contains zero or one square.
    pub fn is_less_two(&self) -> bool {
        self.reset_lsb().is_empty()
    }

    /// Toggles given [`Square`] in this [`SquareSet`].
    pub fn toggle(&mut self, sq: Square) {
        self.0 = self.0 ^ sq.set().0;
    }

    /// Get [`SquareIter`] over all squares in this [`SquareSet`].
    pub fn iter(self) -> SquareIter {
        SquareIter::new(self)
    }

    /// Get [`SquareSubsetIter`] over all subsets of this [`SquareSet`].
    pub fn iter_subset(self) -> SquareSubsetIter {
        SquareSubsetIter::new(self)
    }

    /// Performs a wrapping subtraction of another [`SquareSet`].
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0))
    }

    /// Rotates this [`SquareSet`] by `n` bits.
    pub fn rotate(self, n: u32) -> Self {
        Self(self.0.rotate_left(n))
    }

    /// Get number of set bits in this [`SquareSet`].
    pub fn popcnt(self) -> u32 {
        self.0.count_ones()
    }

    /// Get the index of the least significant set bit as a [`u8`].
    pub fn index_lsb(self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    /// Get this [`SquareSet`] with its least significant set bit removed.
    pub fn reset_lsb(self) -> Self {
        self & self.wrapping_sub(Self(1))
    }
}

impl Sub for SquareSet {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 & !rhs.0)
    }
}

impl BitOr for SquareSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAnd for SquareSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl Not for SquareSet {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

/// Iterator over all [`Square`] in a [`SquareSet`].
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

        debug_assert!(lsb < 64);
        debug_assert!((self.set & SquareSet(1 << lsb)).is_empty());

        Square::new(lsb)
    }
}

/// Iterator over all subsets of a [`SquareSet`].
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
