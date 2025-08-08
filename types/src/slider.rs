pub mod magic;

use crate::{Square, SquareSet};

pub struct Slider {
    deltas: [(i8, i8); 4],
}

pub const ROOK: Slider = Slider {
    deltas: [(1, 0), (0, -1), (-1, 0), (0, 1)],
};

pub const BISHOP: Slider = Slider {
    deltas: [(1, 1), (1, -1), (-1, -1), (-1, 1)],
};

impl Slider {
    pub fn moves(&self, sq: Square, blockers: SquareSet) -> SquareSet {
        let mut moves = SquareSet::EMPTY;

        for &(delta_file, delta_rank) in &self.deltas {
            let mut ray = sq;

            while (ray.set() & blockers) == SquareSet::EMPTY {
                match ray.try_delta(delta_file, delta_rank) {
                    Some(sq) => {
                        ray = sq;
                        moves = moves | ray.set();
                    }
                    None => break,
                }
            }
        }

        moves
    }

    pub const fn blockers(&self, sq: Square) -> SquareSet {
        let (mut blockers, mut i) = (0, 0);

        while i < self.deltas.len() {
            let (delta_file, delta_rank) = self.deltas[i];
            let mut ray = sq;

            while let Some(sq) = ray.try_delta(delta_file, delta_rank) {
                blockers = blockers | ray.set().0;
                ray = sq;
            }

            i += 1;
        }

        blockers = blockers & !sq.set().0;

        SquareSet(blockers)
    }
}
