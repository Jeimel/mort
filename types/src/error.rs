use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum TypeParseError {
    InvalidPieceSymbol,
    InvalidColorSymbol,
}

impl Display for TypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for TypeParseError {}
