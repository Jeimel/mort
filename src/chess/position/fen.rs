use types::{Color, File, Piece, Rank, Square};

use crate::{chess::position::restore::RestoreInfo, error::Error, syntax_error};

use super::{Position, layout::PieceLayout, threat::Threat};

macro_rules! ok_or {
    ($result:expr, $expected:expr, $found:expr) => {
        $result.ok_or_else(|| syntax_error!($expected, $found))?
    };
}

pub type FenParseError = String;

impl Position {
    pub fn from_fen(fen: &str) -> Result<Self, Error> {
        Ok(Position::parse_fen(fen)?)
    }

    fn parse_fen(fen: &str) -> Result<Position, FenParseError> {
        let mut pos = Self {
            layout: PieceLayout::EMPTY,
            threat: Threat::EMPTY,
            restore: RestoreInfo::EMPTY,
            stack: Vec::new(),
            stm: Color::White,
            ply: 0,
            zobrist: 0,
        };

        let fields: Vec<&str> = fen.split_ascii_whitespace().collect();
        if fields.len() != 6 {
            return Err(format!("expected 6 fields, but found {}", fields.len()));
        }

        pos.parse_board(fields[0])?;
        pos.parse_castling(fields[2])?;
        pos.parse_en_passant(fields[3])?;

        pos.stm = ok_or!(Color::try_from(fields[1]).ok(), "'w' or 'b'", fields[1]);
        pos.ply = ok_or!(fields[5].parse().ok(), "positive integer", fields[5]);
        pos.restore.rule50_ply = ok_or!(fields[4].parse().ok(), "positive integer", fields[4]);

        pos.threat.set_blockers(Color::White, &pos.layout);
        pos.threat.set_blockers(Color::Black, &pos.layout);

        pos.threat.set_checkers(pos.stm, &pos.layout);

        Ok(pos)
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

            let file = ok_or!(File::new(col), "valid file index", col);
            let rank = ok_or!(Rank::new(row), "valid rank index", row);

            let piece = Piece::try_from(c).map_err(|err| format!("{:?}", err))?;
            let sq = Square::from(file, rank);

            self.toggle(sq, piece.color(), piece.typ());
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
                'K' => self.restore.castling.set_kingside(Color::White),
                'Q' => self.restore.castling.set_queenside(Color::White),
                'k' => self.restore.castling.set_kingside(Color::Black),
                'q' => self.restore.castling.set_queenside(Color::Black),
                _ => return Err(format!("Expected 'KQkq' subset or '-', but found {}", c)),
            };
        }

        Ok(())
    }

    fn parse_en_passant(&mut self, fen: &str) -> Result<(), FenParseError> {
        if fen == "-" {
            return Ok(());
        }

        let mov = fen.as_bytes();

        let file = ok_or!(File::new(mov[0] - b'a'), "valid move", fen);
        let rank = ok_or!(Rank::new(mov[1] - b'1'), "valid", fen);

        self.restore.en_passant = Some(Square::from(file, rank));

        Ok(())
    }
}
