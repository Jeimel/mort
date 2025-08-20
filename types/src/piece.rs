use crate::TypeParseError;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub const fn new(index: u8) -> Option<Self> {
        if index < 6 {
            // Safety: `index` has a corresponding `PieceType` variant
            Some(unsafe { std::mem::transmute(index) })
        } else {
            None
        }
    }

    pub fn iter() -> impl Iterator<Item = PieceType> {
        (0..6).map(|index| PieceType::new(index).unwrap())
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
            _ => Err(TypeParseError::InvalidPieceSymbol),
        }
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
