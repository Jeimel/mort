use std::{error::Error, fmt::Display};

use types::{Castling, Color, File, PieceType, Rank, Square, TypeParseError};

use super::Board;

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

impl Board {
    pub(in crate::chess) fn from_fen(
        &mut self,
        fen: &str,
    ) -> Result<(Color, u16, u8), FenParseError> {
        macro_rules! ok_or {
            ($result:expr, $err:ident) => {
                $result.ok_or_else(|| FenParseError::$err)?
            };
        }

        self.clear();

        let fields: Vec<&str> = fen.split_ascii_whitespace().collect();
        if fields.len() != 6 {
            return Err(FenParseError::InvalidLength(fields.len()));
        }

        let (mut file, mut rank) = (File::A, Rank::Eight);
        for c in fields[0].chars() {
            if let Some(delta) = c.to_digit(10) {
                file = ok_or!(file.try_delta(delta as i8), InvalidBoard);

                continue;
            }

            if c == '/' {
                file = File::A;
                rank = ok_or!(rank.try_delta(-1), InvalidBoard);

                continue;
            }

            let color = Color::from(c.is_ascii_lowercase());
            let piece = PieceType::try_from(c)?;
            let sq = Square::from(file, rank);

            self.set(sq, color, piece);
            file = ok_or!(file.try_delta(1), InvalidBoard);
        }

        if fields[2] != "-" {
            self.castling = Castling::from_fen(fields[2]);
        }

        if fields[3] != "-" {
            let mov = fields[3].as_bytes();

            let file = ok_or!(File::new(mov[0] - b'a'), InvalidEnPassant);
            let rank = ok_or!(Rank::new(mov[1] - b'1'), InvalidEnPassant);

            self.en_passant = Some(Square::from(file, rank));
        }

        let stm = ok_or!(Color::try_from(fields[1]).ok(), InvalidColor);
        let ply = ok_or!(fields[5].parse().ok(), InvalidFullMove);
        let rule50_ply = ok_or!(fields[4].parse().ok(), InvalidHalfMove);

        Ok((stm, ply, rule50_ply))
    }
}
