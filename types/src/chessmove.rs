use std::fmt::Display;

use crate::{PieceType, Square};

#[derive(Default, Copy, Clone, PartialEq)]
pub struct MoveFlag(u8);

impl MoveFlag {
    pub const QUIET: Self = Self(0b0000);
    pub const DOUBLE_PAWN: Self = Self(0b0001);
    pub const QUEEN_CASTLE: Self = Self(0b0010);
    pub const KING_CASTLE: Self = Self(0b0011);
    pub const CAPTURE: Self = Self(0b0100);
    pub const EN_PASSANT: Self = Self(0b0101);

    const PROMOTION: u8 = 0b1000;
    const PROMOTION_PIECE: u8 = 0b0111;

    pub fn promotion(piece: PieceType) -> Self {
        Self(Self::PROMOTION | piece as u8)
    }

    pub fn promotion_piece(self) -> Option<PieceType> {
        if self.0 & Self::PROMOTION == 0 {
            return None;
        }

        PieceType::new(self.0 & Self::PROMOTION_PIECE)
    }
}

/// A `Move` represents the transformation of a chess board into a new state.
///
/// **Layout**
/// - Bits 0-5: start square
/// - Bits 6-11: target square
/// - Bits 12-15: type of move (See: [`MoveFlag`])
#[derive(Clone, Copy)]
pub struct Move(u16);

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let flag = self
            .flag()
            .promotion_piece()
            .map_or(char::default(), char::from);

        write!(f, "{}{}{}", self.start(), self.target(), flag)
    }
}

impl Move {
    const START_OFFSET: u16 = 0;
    const TARGET_OFFSET: u16 = 6;
    const FLAG_OFFSET: u16 = 12;

    pub const fn new(start: Square, target: Square, flag: MoveFlag) -> Self {
        Move(
            (start as u16) << Self::START_OFFSET
                | (target as u16) << Self::TARGET_OFFSET
                | (flag.0 as u16) << Self::FLAG_OFFSET,
        )
    }

    pub fn start(&self) -> Square {
        // Safety: `0b111111` guarantees that the data has a corresponding `Square` variant
        unsafe { std::mem::transmute((self.0 >> Self::START_OFFSET) as u8 & 0b111111) }
    }

    pub fn target(&self) -> Square {
        // Safety: `0b111111` guarantees that the data has a corresponding `Square` variant
        unsafe { std::mem::transmute((self.0 >> Self::TARGET_OFFSET) as u8 & 0b111111) }
    }

    pub fn flag(&self) -> MoveFlag {
        // Safety: `0b1111` guarantees that the data has a corresponding `MoveFlag` variant
        unsafe { std::mem::transmute((self.0 >> Self::FLAG_OFFSET) as u8 & 0b1111) }
    }
}
