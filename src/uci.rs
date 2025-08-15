use std::fmt::Display;

use crate::chess::FenParseError;

#[derive(Debug)]
pub enum Error {
    Fen(FenParseError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Fen(error) => write!(f, "Invalid FEN: {}", error),
        }
    }
}

impl std::error::Error for Error {}

impl From<FenParseError> for Error {
    fn from(value: FenParseError) -> Self {
        Self::Fen(value)
    }
}
