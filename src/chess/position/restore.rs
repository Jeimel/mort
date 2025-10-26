use types::{Castling, PieceType, Square};

#[derive(Clone)]
pub struct RestoreInfo {
    pub rule50_ply: u16,
    pub castling: Castling,
    pub en_passant: Option<Square>,
    pub capture: Option<PieceType>,
}

// TODO: own method for update?
impl RestoreInfo {
    pub const EMPTY: Self = Self {
        rule50_ply: 0,
        castling: Castling::EMPTY,
        en_passant: None,
        capture: None,
    };
}
