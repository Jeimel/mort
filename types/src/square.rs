use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::{File, Rank, SquareSet};

#[rustfmt::skip]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

impl Square {
    pub const fn new(index: u8) -> Option<Self> {
        if index < 64 {
            // Safety: `index` has a corresponding `Square` variant
            Some(unsafe { std::mem::transmute(index) })
        } else {
            None
        }
    }

    pub const fn from(file: File, rank: Rank) -> Self {
        // Safety: the types `File` and `Rank` form a valid `Square`
        unsafe { std::mem::transmute(rank as u8 * 8 + file as u8) }
    }

    pub fn iter() -> impl Iterator<Item = Square> {
        (0..64).map(|index| Square::new(index).unwrap())
    }

    pub const fn try_delta(self, delta_file: i8, delta_rank: i8) -> Option<Self> {
        let file = self.file().try_delta(delta_file);
        let rank = self.rank().try_delta(delta_rank);

        match (file, rank) {
            (Some(file), Some(rank)) => Some(Self::from(file, rank)),
            _ => None,
        }
    }

    pub const fn try_delta_file(self, delta: i8) -> Option<Self> {
        let file = self.file().try_delta(delta);

        match file {
            Some(file) => Some(Self::from(file, self.rank())),
            _ => None,
        }
    }

    pub const fn try_delta_rank(self, delta: i8) -> Option<Self> {
        let rank = self.rank().try_delta(delta);

        match rank {
            Some(rank) => Some(Self::from(self.file(), rank)),
            _ => None,
        }
    }

    pub const fn set(self) -> SquareSet {
        SquareSet(1u64 << (self as u8))
    }

    pub const fn file(self) -> File {
        File::new(self as u8 & 7).unwrap()
    }

    pub const fn rank(self) -> Rank {
        Rank::new(self as u8 >> 3).unwrap()
    }
}

impl<T> Index<Square> for [T; 64] {
    type Output = T;

    fn index(&self, index: Square) -> &Self::Output {
        unsafe { self.get_unchecked(index as usize) }
    }
}

impl<T> IndexMut<Square> for [T; 64] {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        unsafe { self.get_unchecked_mut(index as usize) }
    }
}
