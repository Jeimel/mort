use std::ops::{Index, IndexMut};

use crate::{Color, Square, SquareSet};

/// Compact representation of castling rights.
///
/// **Layout**
/// - Bits 0-3: indicator for each possible castling
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Castling(u8);

impl Castling {
    pub const EMPTY: Self = Self(0);

    const KING_MASK: [u8; 2] = [0b0001, 0b0010];
    const QUEEN_MASK: [u8; 2] = [0b0100, 0b1000];

    const COLOR_MASK: [u8; 2] = [0b0101, 0b1010];

    pub fn is_empty(&self, color: Color) -> bool {
        self.0 & Self::COLOR_MASK[color] == 0
    }

    pub fn remove(&mut self, start: Square, target: Square) {
        // We can return early if both sides can't castle anymore
        if (self.0 & (Self::COLOR_MASK[Color::White] | Self::COLOR_MASK[Color::Black])) == 0 {
            return;
        }

        self.0 &= Self::mask(start) & Self::mask(target);
    }

    pub fn kingside(&self, color: Color) -> bool {
        self.is_set(Self::KING_MASK[color])
    }

    pub fn queenside(&self, color: Color) -> bool {
        self.is_set(Self::QUEEN_MASK[color])
    }

    pub fn set_kingside(&mut self, color: Color) {
        self.set(Self::KING_MASK[color]);
    }

    pub fn set_queenside(&mut self, color: Color) {
        self.set(Self::QUEEN_MASK[color]);
    }

    fn is_set(&self, mask: u8) -> bool {
        self.0 & mask != 0
    }

    fn set(&mut self, mask: u8) {
        self.0 |= mask;
    }

    fn mask(sq: Square) -> u8 {
        const RELEVANT: SquareSet = SquareSet(10448351135499550865);

        // We can skip the comparison if the square is not relevant for the update
        if (sq.set() & RELEVANT).is_empty() {
            return Self::COLOR_MASK[Color::White] | Self::COLOR_MASK[Color::Black];
        }

        match sq {
            // The white queenside rook moved or got captured
            Square::A1 => !Self::QUEEN_MASK[Color::White],
            // The white king moved
            Square::E1 => !Self::COLOR_MASK[Color::White],
            // The white kingside rook moved or got captured
            Square::H1 => !Self::KING_MASK[Color::White],
            // The black queenside rook moved or got captured
            Square::A8 => !Self::QUEEN_MASK[Color::Black],
            // The black king moved
            Square::E8 => !Self::COLOR_MASK[Color::Black],
            // The black kingside rook moved or got captured
            Square::H8 => !Self::KING_MASK[Color::Black],
            _ => unreachable!(),
        }
    }
}

impl<T> Index<Castling> for [T; 16] {
    type Output = T;

    fn index(&self, index: Castling) -> &Self::Output {
        unsafe { self.get_unchecked(index.0 as usize) }
    }
}

impl<T> IndexMut<Castling> for [T; 16] {
    fn index_mut(&mut self, index: Castling) -> &mut Self::Output {
        unsafe { self.get_unchecked_mut(index.0 as usize) }
    }
}
