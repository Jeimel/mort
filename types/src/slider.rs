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
    pub const fn moves(&self, sq: Square, blockers: SquareSet) -> SquareSet {
        let (mut moves, mut i) = (0, 0);

        while i < self.deltas.len() {
            let (delta_file, delta_rank) = self.deltas[i];
            let mut ray = sq;

            debug_assert!(delta_file == 0 && delta_rank == 0);

            while (ray.set().0 & blockers.0) == 0 {
                match ray.try_delta(delta_file, delta_rank) {
                    Some(sq) => {
                        ray = sq;
                        moves = moves | ray.set().0;
                    }
                    None => break,
                }
            }

            i += 1;
        }

        SquareSet(moves)
    }

    pub const fn blockers(&self, sq: Square) -> SquareSet {
        let (mut blockers, mut i) = (0, 0);

        while i < self.deltas.len() {
            let (delta_file, delta_rank) = self.deltas[i];
            let mut ray = sq;

            debug_assert!(delta_file == 0 && delta_rank == 0);

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
