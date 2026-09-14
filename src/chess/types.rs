mod castling;
mod chessmove;
mod color;
mod file;
mod movelist;
mod piece;
mod rank;
mod slider;
mod square;
mod squareset;

pub use castling::Castling;
pub use chessmove::{Move, MoveFlag};
pub use color::Color;
pub use file::File;
pub use movelist::{MoveList, MoveListEntry};
pub use piece::{Piece, PieceType};
pub use rank::Rank;
pub use slider::{BISHOP, ROOK, magic};
pub use square::Square;
pub use squareset::SquareSet;

use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum TypeParseError {
    InvalidPieceSymbol(char),
    InvalidPieceTypeSymbol(char),
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
