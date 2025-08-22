use std::{error::Error, fmt::Display};

use types::{Color, File, PieceType, Rank, Square, TypeParseError};

use super::Board;

macro_rules! ok_or {
    ($result:expr, $err:ident) => {
        $result.ok_or_else(|| FenParseError::$err)?
    };
}

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
            FenParseError::InvalidLength(len) => {
                write!(f, "Invalid FEN: expected 6 fields, but found {}", len)
            }
            FenParseError::InvalidSymbol(error) => write!(f, "Invalid symbol: {}", error),
            FenParseError::InvalidBoard => {
                write!(f, "Invalid board layout: ranks must sum to 8 squares each")
            }
            FenParseError::InvalidColor => {
                write!(f, "Invalid active color: expected 'w' or 'b'")
            }
            FenParseError::InvalidCastling => {
                write!(f, "Invalid castling rights: must be 'KQkq' subset or '-'")
            }
            FenParseError::InvalidEnPassant => {
                write!(f, "Invalid en passant: must be '-' or valid notation")
            }
            FenParseError::InvalidHalfMove => {
                write!(f, "Invalid halfmove clock: must be a positive integer")
            }
            FenParseError::InvalidFullMove => {
                write!(f, "Invalid fullmove number: must be a positive integer")
            }
        }
    }
}

impl From<TypeParseError> for FenParseError {
    fn from(value: TypeParseError) -> Self {
        Self::InvalidSymbol(value)
    }
}

impl Error for FenParseError {}

impl Board {
    pub fn from_fen(&mut self, fen: &str) -> Result<(Color, u16, u8), FenParseError> {
        self.clear();

        let fields: Vec<&str> = fen.split_ascii_whitespace().collect();
        if fields.len() != 6 {
            return Err(FenParseError::InvalidLength(fields.len()));
        }

        self.parse_board(fields[0])?;
        self.parse_castling(fields[2])?;
        self.parse_en_passant(fields[3])?;

        let stm = ok_or!(Color::try_from(fields[1]).ok(), InvalidColor);
        let ply = ok_or!(fields[5].parse().ok(), InvalidFullMove);
        let rule50_ply = ok_or!(fields[4].parse().ok(), InvalidHalfMove);

        Ok((stm, ply, rule50_ply))
    }

    fn parse_board(&mut self, fen: &str) -> Result<(), FenParseError> {
        let (mut col, mut row) = (0, 7);

        for c in fen.chars() {
            if let Some(delta) = c.to_digit(10) {
                col += delta as u8;

                continue;
            }

            if c == '/' {
                (col, row) = (0, row - 1);

                continue;
            }

            let file = ok_or!(File::new(col), InvalidBoard);
            let rank = ok_or!(Rank::new(row), InvalidBoard);

            let color = Color::from(c.is_ascii_lowercase());
            let piece = PieceType::try_from(c)?;
            let sq = Square::from(file, rank);

            self.set(sq, color, piece);
            col += 1;
        }

        Ok(())
    }

    fn parse_castling(&mut self, fen: &str) -> Result<(), FenParseError> {
        if fen == "-" {
            return Ok(());
        }

        for c in fen.chars() {
            match c {
                'K' => self.castling.set_kingside(Color::White, File::G),
                'Q' => self.castling.set_queenside(Color::White, File::C),
                'k' => self.castling.set_kingside(Color::Black, File::G),
                'q' => self.castling.set_queenside(Color::Black, File::C),
                _ => return Err(FenParseError::InvalidCastling),
            };
        }

        Ok(())
    }

    fn parse_en_passant(&mut self, fen: &str) -> Result<(), FenParseError> {
        if fen == "-" {
            return Ok(());
        }

        let mov = fen.as_bytes();

        let file = ok_or!(File::new(mov[0] - b'a'), InvalidEnPassant);
        let rank = ok_or!(Rank::new(mov[1] - b'1'), InvalidEnPassant);

        self.en_passant = Some(Square::from(file, rank));

        Ok(())
    }
}
