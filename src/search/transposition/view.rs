use types::Move;

use crate::{
    chess::Key,
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
        ply: i32,
    ) {
        let index = self.index(zobrist);
        let entry = TranspositionEntry::from(&self.table[index]);
        let key = Self::checksum(zobrist);

        // We keep the entry, if
        // 1. the entry is occupied by the same position,
        // 2. and the existing entry is at least as deep
        if entry.key == key && entry.depth() >= depth {
            return;
        }

        if score.abs() > MATE {
            score += score.signum() * ply;
        }

        let entry = TranspositionEntry {
            key,
            mov,
            score: score.try_into().expect("`score` must be in range of i16"),
            depth: depth.try_into().expect("`depth` must be in range of i8"),
            bound,
        };

        self.table[index].store(entry);
    }

    pub fn probe(&self, zobrist: Key, ply: i32) -> Option<TranspositionEntry> {
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
        (zobrist as usize) & (self.table.len() - 1)
    }

    fn checksum(zobrist: Key) -> u16 {
        const KEY_SHIFT: u16 = 48;

        (zobrist >> KEY_SHIFT) as u16
    }
}
