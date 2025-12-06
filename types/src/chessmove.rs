use std::{fmt::Display, num::NonZeroU16};

use crate::{PieceType, Square};

const _: () = assert!(std::mem::size_of::<Move>() == 2);
const _: () = assert!(std::mem::size_of::<Move>() == std::mem::size_of::<Option<Move>>());

/// Encodes the type of [`Move`].
///
/// [`MoveFlag`] takes at most the four least siginifcant bits.
#[derive(Clone, Copy, PartialEq)]
pub struct MoveFlag(u8);

impl MoveFlag {
    /// A quiet non-capturing, non-special [`Move`].
    pub const QUIET: Self = Self(0b0000);

    /// A double pawn push from the second [`crate::Rank`].
    pub const DOUBLE_PAWN: Self = Self(0b0001);

    /// Queenside castling.
    pub const QUEEN_CASTLE: Self = Self(0b0010);

    /// Kingside castling.
    pub const KING_CASTLE: Self = Self(0b0011);

    /// A capture.
    pub const CAPTURE: Self = Self(0b0100);

    /// An en-passant capture.
    pub const EN_PASSANT: Self = Self(0b0101);

    /// Internal flag indicating that the move is a promotion.
    const PROMOTION: u8 = 0b1000;
    /// Internal mask selecting the promotion piece bits.
    const PROMOTION_PIECE: u8 = 0b0111;

    /// Constructs a promotion [`MoveFlag`] corresponding to the given [`PieceType`].
    pub fn new_promotion(piece: PieceType) -> Self {
        Self(Self::PROMOTION | piece as u8)
    }

    /// Returns `true` if this [`MoveFlag`] represents a promotion.
    pub fn promotion(&self) -> bool {
        self.0 & Self::PROMOTION != 0
    }

    /// Returns the promoted [`PieceType`] if this [`MoveFlag`] represents a promotion.
    pub fn piece(self) -> Option<PieceType> {
        if !self.promotion() {
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
/// - Bits 12-15: [`MoveFlag`]
#[derive(Clone, Copy)]
pub struct Move(NonZeroU16);

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let flag = match self.flag().piece() {
            Some(piece) => String::from(char::from(piece)),
            None => String::new(),
        };

        write!(f, "{}{}{}", self.start(), self.target(), flag)
    }
}

impl Move {
    /// Bit offset for the encoded start [`Square`].
    const START_OFFSET: u16 = 0;
    /// Bit offset for the encoded target [`Square`].
    const TARGET_OFFSET: u16 = 6;
    /// Bit offset for the encoded [`MoveFlag`].
    const FLAG_OFFSET: u16 = 12;

    /// Creates a new [`Move`] from a start [`Square`], target [`Square`], and [`MoveFlag`].
    pub const fn new(start: Square, target: Square, flag: MoveFlag) -> Self {
        let data = (start as u16) << Self::START_OFFSET
            | (target as u16) << Self::TARGET_OFFSET
            | (flag.0 as u16) << Self::FLAG_OFFSET;

        // Safety: `start` and `target` can't both be zero at the same time
        Move(unsafe { NonZeroU16::new_unchecked(data) })
    }

    /// Returns start [`Square`] of this [`Move`].
    pub fn start(&self) -> Square {
        // Safety: `0b111111` guarantees that the data has a corresponding `Square` variant
        unsafe { std::mem::transmute((self.0.get() >> Self::START_OFFSET) as u8 & 0b111111) }
    }

    /// Returns target [`Square`] of this [`Move`].
    pub fn target(&self) -> Square {
        // Safety: `0b111111` guarantees that the data has a corresponding `Square` variant
        unsafe { std::mem::transmute((self.0.get() >> Self::TARGET_OFFSET) as u8 & 0b111111) }
    }

    /// Returns [`MoveFlag`] of this [`Move`].
    pub fn flag(&self) -> MoveFlag {
        // Safety: `0b1111` guarantees that the data has a corresponding `MoveFlag` variant
        unsafe { std::mem::transmute((self.0.get() >> Self::FLAG_OFFSET) as u8 & 0b1111) }
    }

    /// Returns `true` if this [`Move`] is tactical, which means
    /// it is a capture or promotion.
    pub fn tactical(&self) -> bool {
        let flag = self.flag();
        matches!(flag, MoveFlag::CAPTURE | MoveFlag::EN_PASSANT) || flag.promotion()
    }

    /// Returns the internal bits of this [`Move`].
    pub const fn inner(&self) -> u16 {
        self.0.get()
    }
}
