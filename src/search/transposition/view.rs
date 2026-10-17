use crate::{
    chess::{Key, Move},
    evaluation::MATE,
    search::transposition::{Bound, TranspositionEntry, bucket::Bucket},
};

pub struct TranspositionView<'a> {
    pub(super) table: &'a [Bucket],
    pub(super) age: u8,
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
        let bucket = &self.table[self.index(zobrist)];
        let key = Self::checksum(zobrist);

        // We use the slot of the same position, otherwise we replace the least valuable one.
        let (slot, old) = bucket
            .iter()
            .map(|slot| (slot, TranspositionEntry::from(slot)))
            .min_by_key(|(_, entry)| (entry.key != key, self.value(*entry)))
            .expect("bucket should not be empty");

        let equal = old.key == key;

        // We keep the entry, if
        // 1. the entry is occupied by the same position,
        // 2. the entry is from the current search,
        // 3. the new entry is not exact,
        // 4. and the existing entry is much deeper
        if equal && self.relative_age(old) == 0 && bound != Bound::Exact && old.depth() >= depth + 4
        {
            return;
        }

        // We keep the old move on a fail-low, if it is the same position
        let mov = if equal { mov.or(old.mov) } else { mov };

        if score.abs() > MATE {
            score += score.signum() * ply as i32;
        }

        let entry = TranspositionEntry {
            key,
            mov,
            score: score.try_into().expect("score must be in range of i16"),
            depth: depth.try_into().expect("depth must be in range of i8"),
            meta: TranspositionEntry::pack(self.age, bound),
        };

        slot.store(entry);
    }

    pub fn probe(&self, zobrist: Key, ply: usize) -> Option<TranspositionEntry> {
        let bucket = &self.table[self.index(zobrist)];

        for slot in bucket.iter() {
            let mut entry = TranspositionEntry::from(slot);

            if entry.key == 0 || entry.key != Self::checksum(zobrist) {
                continue;
            }

            if entry.score().abs() > MATE {
                entry.score -= entry.score.signum() * ply as i16;
            }

            return Some(entry);
        }

        None
    }

    fn index(&self, zobrist: Key) -> usize {
        ((u128::from(zobrist) * (self.table.len() as u128)) >> 64) as usize
    }

    fn checksum(zobrist: Key) -> u16 {
        zobrist as u16
    }

    fn value(&self, entry: TranspositionEntry) -> i32 {
        entry.depth() - 8 * self.relative_age(entry)
    }

    fn relative_age(&self, entry: TranspositionEntry) -> i32 {
        i32::from(self.age.wrapping_sub(entry.age()) & TranspositionEntry::AGE_MASK)
    }
}
