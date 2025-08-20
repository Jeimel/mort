use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum TypeParseError {
    InvalidPieceSymbol(char),
    InvalidColorSymbol(String),
}

impl Display for TypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeParseError::InvalidPieceSymbol(symbol) => write!(f, "Invalid symbol: {}", symbol),
            TypeParseError::InvalidColorSymbol(symbol) => write!(f, "Invalid symbol: {}", symbol),
        }
    }
}

impl Error for TypeParseError {}
