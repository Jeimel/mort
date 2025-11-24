use std::{error::Error, fmt::Display};

/// The error type, which is returned from converting symbols into a type.
#[derive(Debug)]
pub enum TypeParseError {
    /// The symbol was not a valid [`crate::Piece`].
    InvalidPieceSymbol(char),
    /// The symbol was not a valid [`crate::PieceType`].
    InvalidPieceTypeSymbol(char),
    /// The symbol was not a valid [`crate::Color`].
    InvalidColorSymbol(String),
}

impl Display for TypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeParseError::InvalidPieceSymbol(symbol) => write!(f, "Invalid symbol: {}", symbol),
            TypeParseError::InvalidColorSymbol(symbol) => write!(f, "Invalid symbol: {}", symbol),
            TypeParseError::InvalidPieceTypeSymbol(symbol) => {
                write!(f, "Invalid symbol: {}", symbol)
            }
        }
    }
}

impl Error for TypeParseError {}
