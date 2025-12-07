mod butterfly;

pub use butterfly::ButterflyHistory;

use types::Move;

use crate::search::worker::Worker;

impl Worker<'_> {
    pub fn update_quiet_history(&mut self, mov: Move, depth: i16) {
        self.history[self.pos.stm()].update(mov, depth * depth);
    }
}
