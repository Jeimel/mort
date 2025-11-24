use std::fmt::Display;

use crate::chess::FenParseError;

pub type UciError = String;

#[macro_export]
macro_rules! syntax_error {
    ($expected:expr, $found:expr) => {
        format!("expected {}, but found {}", $expected, $found)
    };
}

#[macro_export]
macro_rules! ok_or {
    ($result:expr, $expected:expr, $found:expr) => {
        $result.ok_or_else(|| syntax_error!($expected, $found))?
    };
}

#[macro_export]
macro_rules! unwrap_or {
    ($result:expr) => {
        $result.unwrap_or_else(|err| eprintln!("{}", err))
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
            Error::Uci(error) => write!(f, "Invalid argument: {}", error),
        }
    }
}

impl std::error::Error for Error {}

impl From<FenParseError> for Error {
    fn from(value: FenParseError) -> Self {
        Self::Fen(value)
    }
}
