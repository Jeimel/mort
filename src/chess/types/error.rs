use std::fmt::Display;

#[derive(Debug)]
pub enum TypeParseError {
    Piece(char),
    PieceType(char),
    Color(String),
    File(char),
    Rank(char),
}

impl Display for TypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeParseError::Piece(symbol) => write!(f, "Invalid symbol: {}", symbol),
            TypeParseError::PieceType(symbol) => write!(f, "Invalid symbol: {}", symbol),
            TypeParseError::Color(symbol) => write!(f, "Invalid symbol: {}", symbol),
            TypeParseError::File(symbol) => write!(f, "Invalid file: {}", symbol),
            TypeParseError::Rank(symbol) => write!(f, "Invalid rank: {}", symbol),
        }
    }
}

impl std::error::Error for TypeParseError {}
