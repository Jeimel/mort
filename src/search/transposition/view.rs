use crate::{
    chess::{Key, Move},
    evaluation::MATE,
    search::transposition::{Bound, TranspositionEntry, entry::TranspositionInternal},
};

pub struct TranspositionView<'a> {
    pub(super) table: &'a [TranspositionInternal],
}

impl TranspositionView<'_> {
    pub fn insert(
        &mut self,
        zobrist: Key,
        mov: Option<Move>,
        mut score: i32,
        depth: i32,
        bound: Bound,
        ply: usize,
    ) {
        let index = self.index(zobrist);
        let entry = TranspositionEntry::from(&self.table[index]);
        let key = Self::checksum(zobrist);
        let equal = entry.key == key;

        // We keep the entry, if
        // 1. the entry is occupied by the same position,
        // 2. the new entry is not exact,
        // 3. and the existing entry is much deeper
        if equal && bound != Bound::Exact && entry.depth() >= depth + 4 {
            return;
        }

        // We keep the old move on a fail-low, if it is the same position
        let mov = if equal { mov.or(entry.mov) } else { mov };

        if score.abs() > MATE {
            score += score.signum() * ply as i32;
        }

        let entry = TranspositionEntry {
            key,
            mov,
            score: score.try_into().expect("score must be in range of i16"),
            depth: depth.try_into().expect("depth must be in range of i8"),
            bound,
        };

        self.table[index].store(entry);
    }

    pub fn probe(&self, zobrist: Key, ply: usize) -> Option<TranspositionEntry> {
        let mut entry = TranspositionEntry::from(&self.table[self.index(zobrist)]);

        if entry.key == 0 || entry.key != Self::checksum(zobrist) {
            return None;
        }

        if entry.score().abs() > MATE {
            entry.score -= entry.score.signum() * ply as i16;
        }

        Some(entry)
    }

    fn index(&self, zobrist: Key) -> usize {
        ((u128::from(zobrist) * (self.table.len() as u128)) >> 64) as usize
    }

    fn checksum(zobrist: Key) -> u16 {
        zobrist as u16
    }
}
