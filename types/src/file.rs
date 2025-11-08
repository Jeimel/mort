use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::SquareSet;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
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

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        };

        write!(f, "{}", file)
    }
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

    pub fn iter() -> impl DoubleEndedIterator<Item = Self> {
        (0..8).map(|index| Self::new(index).unwrap())
    }
}

impl<T> Index<File> for [T; 8] {
    type Output = T;

    fn index(&self, index: File) -> &Self::Output {
        // Safety: `index` is in [0, 8)
        unsafe { self.get_unchecked(index as usize) }
    }
}

impl<T> IndexMut<File> for [T; 8] {
    fn index_mut(&mut self, index: File) -> &mut Self::Output {
        // Safety: `index` is in [0, 8)
        unsafe { self.get_unchecked_mut(index as usize) }
    }
}
