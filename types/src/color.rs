use std::ops::{Index, IndexMut};

use crate::TypeParseError;

#[repr(u8)]
#[derive(Copy, Clone)]
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
        unsafe { self.get_unchecked(index as usize) }
    }
}

impl<T> IndexMut<Color> for [T; 2] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        unsafe { self.get_unchecked_mut(index as usize) }
    }
}

impl TryFrom<&str> for Color {
    type Error = TypeParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err(TypeParseError::InvalidColorSymbol),
        }
    }
}

impl From<bool> for Color {
    fn from(value: bool) -> Self {
        match value {
            true => Color::White,
            false => Color::Black,
        }
    }
}
