use types::{Castling, Color, PieceType, Square, SquareSet};

use crate::chess::board::{BETWEEN, Key, PieceLayout};

#[derive(Clone)]
pub struct GameState {
    pub rule50_ply: u8,
    pub castling: Castling,
    pub en_passant: Option<Square>,
    pub capture: Option<PieceType>,
    /// Pieces, which block threats to our king
    pub blockers: SquareSet,
    /// Pieces, which threaten our king
    pub checkers: SquareSet,
    pub zobrist: Key,
}

impl GameState {
    pub const EMPTY: Self = Self {
        rule50_ply: 0,
        castling: Castling::EMPTY,
        en_passant: None,
        capture: None,
        blockers: SquareSet::EMPTY,
        checkers: SquareSet::EMPTY,
        zobrist: 0,
    };

    pub fn set_blockers(&mut self, color: Color, layout: &PieceLayout) {
        self.blockers = SquareSet::EMPTY;

        let king = layout.kings[color];

        // Determine all sliders, which potentially attack our king
        let snipers = layout.snipers(king, color);
        let occ = layout.all() - snipers;

        for sniper in snipers.iter() {
            // Determine occupied squares between the sniper and our king
            let blocker = BETWEEN[king][sniper] & occ;

            // If we have no blocker our king is already in check, and
            // if we have more than one blocker, the sniper is not a threat
            if !blocker.is_empty() && blocker.is_less_two() {
                self.blockers = self.blockers | blocker;
            }
        }
    }

    pub fn set_checkers(&mut self, color: Color, layout: &PieceLayout) {
        self.checkers = layout.attackers(layout.kings[color], color, layout.all())
    }
}
