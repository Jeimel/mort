use types::{Color, SquareSet};

use super::{layout::PieceLayout, movegen::BETWEEN};

#[derive(Clone, Copy)]
pub struct Threat {
    /// Pieces, which block threats to the respective king
    blockers: [SquareSet; 2],
    /// Pieces, which threaten our king
    checkers: SquareSet,
}

impl Threat {
    pub const EMPTY: Self = Self {
        blockers: [SquareSet::EMPTY; 2],
        checkers: SquareSet::EMPTY,
    };

    pub fn blockers(&self, color: Color) -> SquareSet {
        self.blockers[color]
    }

    pub fn checkers(&self) -> SquareSet {
        self.checkers
    }

    pub fn set_blockers(&mut self, color: Color, layout: &PieceLayout) {
        self.blockers[color] = SquareSet::EMPTY;

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
                self.blockers[color] = self.blockers[color] | blocker;
            }
        }
    }

    pub fn set_checkers(&mut self, color: Color, layout: &PieceLayout) {
        self.checkers = layout.attackers(layout.kings[color], color, layout.all())
    }
}
