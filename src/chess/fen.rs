use std::{error::Error, fmt::Display};

use types::TypeParseError;

#[derive(Debug)]
pub enum FenParseError {
    InvalidLength(usize),
    InvalidSymbol(TypeParseError),
    InvalidBoard,
    InvalidColor,
    InvalidCastling,
    InvalidEnPassant,
    InvalidHalfMove,
    InvalidFullMove,
}

impl Display for FenParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FenParseError::InvalidLength(len) => write!(f, "Invalid length: {}", len),
            FenParseError::InvalidSymbol(error) => write!(f, "{}", error),
            FenParseError::InvalidBoard => todo!(),
            FenParseError::InvalidColor => todo!(),
            FenParseError::InvalidCastling => todo!(),
            FenParseError::InvalidEnPassant => todo!(),
            FenParseError::InvalidHalfMove => todo!(),
            FenParseError::InvalidFullMove => todo!(),
        }
    }
}

impl From<TypeParseError> for FenParseError {
    fn from(value: TypeParseError) -> Self {
        Self::InvalidSymbol(value)
    }
}

impl Error for FenParseError {}
