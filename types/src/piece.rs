use std::ops::Index;

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
