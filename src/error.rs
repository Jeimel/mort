use std::fmt::Display;

use crate::{chess::FenParseError, uci::UciError};

#[macro_export]
macro_rules! syntax_error {
    ($expected:expr, $found:expr) => {
        format!("expected {}, but found {}", $expected, $found)
    };
}

#[derive(Debug)]
pub enum Error {
    Fen(FenParseError),
    Uci(UciError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Fen(error) => write!(f, "Invalid FEN: {}", error),
            Error::Uci(error) => write!(f, "Invalid command: {}", error),
        }
    }
}

impl std::error::Error for Error {}

impl From<FenParseError> for Error {
    fn from(value: FenParseError) -> Self {
        Self::Fen(value)
    }
}
