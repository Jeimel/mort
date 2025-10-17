use std::ops::{Index, IndexMut, Not};

use crate::TypeParseError;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub const fn new(index: u8) -> Option<Self> {
        if index < 2 {
            // Safety: `index` has a corresponding `Color` variant
            Some(unsafe { std::mem::transmute(index) })
        } else {
            None
        }
    }
}

impl<T> Index<Color> for [T; 2] {
    type Output = T;

    fn index(&self, index: Color) -> &Self::Output {
        // Safety: index is either 0 or 1
        unsafe { self.get_unchecked(index as usize) }
    }
}

impl<T> IndexMut<Color> for [T; 2] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        // Safety: index is either 0 or 1
        unsafe { self.get_unchecked_mut(index as usize) }
    }
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl TryFrom<&str> for Color {
    type Error = TypeParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err(TypeParseError::InvalidColorSymbol(value.to_string())),
        }
    }
}

impl From<bool> for Color {
    fn from(value: bool) -> Self {
        match value {
            true => Color::Black,
            false => Color::White,
        }
    }
}
