use types::{Color, File, Piece, Rank, Square};

use crate::{
    chess::{
        board::{Board, zobrist},
        position::GameState,
    },
    syntax_error,
};

use super::layout::PieceLayout;

macro_rules! ok_or {
    ($result:expr, $expected:expr, $found:expr) => {
        $result.ok_or_else(|| syntax_error!($expected, $found))?
    };
}

pub type FenParseError = String;

impl Board {
    pub fn from_fen(fen: &str) -> Result<(Self, Color, u16), FenParseError> {
        let mut board = Self {
            layout: PieceLayout::EMPTY,
            state: GameState::EMPTY,
        };

        let fields: Vec<&str> = fen.split_ascii_whitespace().collect();
        if fields.len() != 6 {
            return Err(format!("expected 6 fields, but found {}", fields.len()));
        }

        let stm = ok_or!(Color::try_from(fields[1]).ok(), "'w' or 'b'", fields[1]);
        let fullmove = ok_or!(fields[5].parse().ok(), "positive integer", fields[5]);

        if stm == Color::Black {
            board.state.zobrist ^= zobrist::SIDE;
        }

        board.parse_board(fields[0])?;
        board.parse_castling(fields[2])?;
        board.parse_en_passant(fields[3])?;

        board.state.rule50_ply = ok_or!(fields[4].parse().ok(), "positive integer", fields[4]);

        board.state.set_blockers(stm, &board.layout);
        board.state.set_checkers(stm, &board.layout);

        Ok((board, stm, fullmove))
    }

    pub fn fen(&self, stm: Color, fullmove: u16) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.layout,
            char::from(stm),
            self.state.castling,
            if let Some(target) = self.state.en_passant {
                format!("{}", target)
            } else {
                "-".to_string()
            },
            self.state.rule50_ply,
            fullmove,
        )
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

            self.toggle::<true>(sq, piece.color(), piece.typ());
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
                'K' => self.state.castling.set_kingside(Color::White),
                'Q' => self.state.castling.set_queenside(Color::White),
                'k' => self.state.castling.set_kingside(Color::Black),
                'q' => self.state.castling.set_queenside(Color::Black),
                _ => return Err(format!("Expected 'KQkq' subset or '-', but found {}", c)),
            };
        }

        self.state.zobrist ^= zobrist::CASTLING[self.state.castling];

        Ok(())
    }

    fn parse_en_passant(&mut self, fen: &str) -> Result<(), FenParseError> {
        if fen == "-" {
            return Ok(());
        }

        let mov = fen.as_bytes();

        let file = ok_or!(File::new(mov[0] - b'a'), "valid move", fen);
        let rank = ok_or!(Rank::new(mov[1] - b'1'), "valid", fen);

        self.state.en_passant = Some(Square::from(file, rank));
        self.state.zobrist ^= zobrist::EN_PASSANT[file];

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::chess::Position;

    const FEN: [&str; 6] = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
    ];

    #[test]
    fn parse() {
        for fen in FEN {
            let pos = Position::from_fen(fen).unwrap();

            assert_eq!(fen, pos.fen());
        }
    }
}
