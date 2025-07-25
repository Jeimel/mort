use std::ops::Index;

use super::BitBoard;

#[repr(transparent)]
pub struct Castling(u8);

#[repr(u8)]
pub enum Color {
    White,
    Black,
}

impl<T> Index<Color> for [T; 2] {
    type Output = T;

    fn index(&self, index: Color) -> &Self::Output {
        unsafe { self.get_unchecked(index as usize) }
    }
}

#[repr(u8)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
}

impl<T> Index<PieceType> for [T; 4] {
    type Output = T;

    fn index(&self, index: PieceType) -> &Self::Output {
        unsafe { self.get_unchecked(index as usize) }
    }
}

#[rustfmt::skip]
#[repr(u8)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub const fn bitboard(self) -> BitBoard {
        BitBoard(1u64 << (self as u8))
    }
}

impl<T> Index<Square> for [T; 64] {
    type Output = T;

    fn index(&self, index: Square) -> &Self::Output {
        unsafe { self.get_unchecked(index as usize) }
    }
}

#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub const fn bitboard(self) -> BitBoard {
        BitBoard(0x101010101010101u64 << (self as u8))
    }
}

#[repr(u8)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    pub const fn bitboard(self) -> BitBoard {
        BitBoard(0xffu64 << (self as u8 * 8))
    }
}
