use crate::{
    chess::{All, File, Move, MoveFlag, MoveList, PieceType, Position, Rank, Square},
    error::{Error, Result},
};

pub fn parse(pgn: &str) -> Result<(Vec<Move>, f32)> {
    let (moves, outcome) = parse_movetext(pgn)?;

    let (mut pos, mut result) = (Position::default(), Vec::new());

    for str in moves {
        let mut moves = MoveList::new();
        pos.generate::<All>(&mut moves);

        let legal = moves.iter().filter(|&mov| pos.legal(mov));
        let mov = parse_san(&pos, legal, &str)?;

        result.push(mov);
        pos.make_move(mov);
    }

    Ok((result, outcome))
}

fn parse_movetext(movetext: &str) -> Result<(Vec<String>, f32)> {
    let mut moves = Vec::new();
    let mut outcome = None;

    for token in movetext.split_whitespace() {
        match token.as_bytes() {
            [b'1', b'-', b'0'] => outcome = Some(1.0),
            [b'0', b'-', b'1'] => outcome = Some(0.0),
            [b'1', b'/', b'2', b'-', b'1', b'/', b'2'] => outcome = Some(0.5),
            [_, b'.'] | [_, _, b'.'] | [_, _, _, b'.'] => {}
            [..] => moves.push(token.to_string()),
        }
    }

    let Some(outcome) = outcome else {
        return Err(Error::Internal(String::from("Result missing")));
    };

    Ok((moves, outcome))
}

fn parse_san(pos: &Position, mut moves: impl Iterator<Item = Move>, str: &str) -> Result<Move> {
    let error = || Error::Internal(String::from("Invalid SAN"));
    let san = str.trim_end_matches(['+', '#']);

    let castling = match san {
        "O-O" => Some(MoveFlag::KING_CASTLE),
        "O-O-O" => Some(MoveFlag::QUEEN_CASTLE),
        _ => None,
    };

    if let Some(flag) = castling {
        return moves.find(|mov| mov.flag() == flag).ok_or_else(error);
    }

    let (san, promotion) = match san.split_once('=') {
        Some((san, piece)) => match piece.as_bytes() {
            [piece] => (san, Some(PieceType::try_from(piece)?)),
            _ => return Err(error()),
        },
        None => (san, None),
    };

    let (piece, rest) = match san.as_bytes() {
        [piece @ (b'N' | b'B' | b'R' | b'Q' | b'K'), rest @ ..] => {
            (PieceType::try_from(piece)?, rest)
        }
        rest => (PieceType::Pawn, rest),
    };

    let [hint @ .., file, rank] = rest else {
        return Err(error());
    };
    let target = Square::from(File::try_from(file)?, Rank::try_from(rank)?);

    let (from_file, from_rank) = match hint.strip_suffix(b"x").unwrap_or(hint) {
        [] => (None, None),
        [file @ b'a'..=b'h'] => (Some(File::try_from(file)?), None),
        [rank @ b'1'..=b'8'] => (None, Some(Rank::try_from(rank)?)),
        [file, rank] => (Some(File::try_from(file)?), Some(Rank::try_from(rank)?)),
        _ => return Err(error()),
    };

    moves
        .find(|mov| {
            mov.target() == target
                && pos.layout().unchecked_at(mov.start()) == piece
                && from_file.is_none_or(|file| mov.start().file() == file)
                && from_rank.is_none_or(|rank| mov.start().rank() == rank)
                && mov.flag().piece() == promotion
        })
        .ok_or_else(error)
}
