use std::{
    marker::ConstParamTy,
    ops::{Index, IndexMut},
};

use crate::{Color, TypeParseError};

/// A chess piece, representing both [`PieceType`] and [`Color`].
///
/// The corresponding [`Color`] is stored as least significant bit.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Piece {
    WhitePawn,
    BlackPawn,
    WhiteKnight,
    BlackKnight,
    WhiteBishop,
    BlackBishop,
    WhiteRook,
    BlackRook,
    WhiteQueen,
    BlackQueen,
    WhiteKing,
    BlackKing,
}

impl Piece {
    /// Convert `index` to a [`Piece`].
    pub const fn new(index: u8) -> Option<Self> {
        if index < 12 {
            // Safety: `index` has a corresponding `Piece` variant
            Some(unsafe { std::mem::transmute(index) })
        } else {
            None
        }
    }

    /// Convert a [`Color`] and a [`PieceType`] to a corresponding [`Piece`].
    pub const fn from(color: Color, piece: PieceType) -> Self {
        // Safety: the types `Color` and `PieceType` form a valid `Piece`
        unsafe { std::mem::transmute(((piece as u8) << 1) | color as u8) }
    }

    /// Get represented [`PieceType`].
    pub const fn typ(self) -> PieceType {
        PieceType::new(self as u8 >> 1).unwrap()
    }

    /// Get represented [`Color`].
    pub const fn color(self) -> Color {
        Color::new(self as u8 & 1).unwrap()
    }
}

impl From<Piece> for char {
    fn from(value: Piece) -> Self {
        let piece = char::from(value.typ());

        match value.color() {
            Color::White => piece.to_ascii_uppercase(),
            Color::Black => piece,
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = TypeParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'P' => Ok(Piece::WhitePawn),
            'p' => Ok(Piece::BlackPawn),
            'N' => Ok(Piece::WhiteKnight),
            'n' => Ok(Piece::BlackKnight),
            'B' => Ok(Piece::WhiteBishop),
            'b' => Ok(Piece::BlackBishop),
            'R' => Ok(Piece::WhiteRook),
            'r' => Ok(Piece::BlackRook),
            'Q' => Ok(Piece::WhiteQueen),
            'q' => Ok(Piece::BlackQueen),
            'K' => Ok(Piece::WhiteKing),
            'k' => Ok(Piece::BlackKing),
            _ => Err(TypeParseError::InvalidPieceSymbol(value)),
        }
    }
}

/// A chess piece.
#[repr(u8)]
#[derive(ConstParamTy, Clone, Copy, Eq, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    /// Convert `index` to a [`PieceType`].
    pub const fn new(index: u8) -> Option<Self> {
        if index < 6 {
            // Safety: `index` has a corresponding `PieceType` variant
            Some(unsafe { std::mem::transmute(index) })
        } else {
            None
        }
    }

    /// Get an [`Iterator`] over all [`PieceType`].
    pub fn iter() -> impl Iterator<Item = PieceType> {
        (0..6).map(|index| PieceType::new(index).unwrap())
    }
}

impl<T> Index<PieceType> for [T; 6] {
    type Output = T;

    fn index(&self, index: PieceType) -> &Self::Output {
        // Safety: `index` is in [0, 6)
        unsafe { self.get_unchecked(index as usize) }
    }
}

impl<T> IndexMut<PieceType> for [T; 6] {
    fn index_mut(&mut self, index: PieceType) -> &mut Self::Output {
        // Safety: `index` is in [0, 6)
        unsafe { self.get_unchecked_mut(index as usize) }
    }
}

impl From<PieceType> for char {
    fn from(value: PieceType) -> Self {
        match value {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        }
    }
}

impl TryFrom<char> for PieceType {
    type Error = TypeParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'p' => Ok(PieceType::Pawn),
            'n' => Ok(PieceType::Knight),
            'b' => Ok(PieceType::Bishop),
            'r' => Ok(PieceType::Rook),
            'q' => Ok(PieceType::Queen),
            'k' => Ok(PieceType::King),
            _ => Err(TypeParseError::InvalidPieceSymbol(value)),
        }
    }
}
