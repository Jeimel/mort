mod butterfly;

pub use butterfly::ButterflyHistory;

use crate::{chess::Move, search::worker::Worker};

impl Worker<'_> {
    pub fn update_quiet_history(&mut self, best_move: Move, quiets: &[Move], depth: i16) {
        let entry = &mut self.history[self.pos.stm()];
        let bonus = depth * depth;

        for &mov in quiets {
            entry.update(mov, if mov == best_move { bonus } else { -bonus });
        }
    }
}
