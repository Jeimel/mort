use crate::{Color, File};

/// Compact representation of castling rights.
///
/// **Layout**
/// - Bits 0-11: target file positions (3 bits each, 0–7 for file `a`–`h`)
///     - Bit 0-2: white kingside
///     - Bit 3-5: black kinhside
///     - Bit 6-8: white queenside
///     - Bit 9-11: black queenside
#[repr(transparent)]
pub struct Castling(u16);

impl Castling {
    pub const EMPTY: Self = Self(0);

    pub fn set_kingside(&mut self, color: Color, file: File) {
        const SHIFT: [u8; 2] = [0, 3];

        self.set(SHIFT[color], file as u16);
    }

    pub fn set_queenside(&mut self, color: Color, file: File) {
        const SHIFT: [u8; 2] = [6, 9];

        self.set(SHIFT[color], file as u16);
    }

    fn set(&mut self, shift: u8, data: u16) {
        self.0 |= data << shift;
    }
}
