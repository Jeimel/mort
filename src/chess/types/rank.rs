use std::fmt::Display;

use crate::chess::{SquareSet, TypeParseError};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
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

impl Display for Rank {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Rank::One => '1',
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
        })
    }
}

impl TryFrom<&u8> for Rank {
    type Error = TypeParseError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            b'1'..=b'8' => Ok(Self::new(value - b'1').unwrap()),
            _ => Err(TypeParseError::Rank(char::from(*value))),
        }
    }
}

impl Rank {
    pub const fn new(index: u8) -> Option<Self> {
        if index < 8 {
            // Safety: `index` has a corresponding `Rank` variant
            Some(unsafe { std::mem::transmute::<u8, Self>(index) })
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
        SquareSet(0xffu64 << (self as u8 * 8))
    }

    pub fn iter() -> impl DoubleEndedIterator<Item = Self> {
        (0..8).map(|index| Self::new(index).unwrap())
    }
}
