use types::{Castling, PieceType, Square};

#[derive(Clone)]
pub struct GameState {
    pub rule50_ply: u16,
    pub castling: Castling,
    pub en_passant: Option<Square>,
    pub capture: Option<PieceType>,
}

impl GameState {
    pub const EMPTY: Self = Self {
        rule50_ply: 0,
        castling: Castling::EMPTY,
        en_passant: None,
        capture: None,
    };
}
