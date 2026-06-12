mod butterfly;

pub use butterfly::ButterflyHistory;

use crate::{chess::Move, search::worker::Worker};

impl Worker<'_> {
    pub fn update_quiet_history(&mut self, mov: Move, depth: i16) {
        self.history[self.pos.stm()].update(mov, depth * depth);
    }
}
