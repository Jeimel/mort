use std::sync::atomic::{AtomicU64, Ordering};

use types::Move;

use crate::search::transposition::Bound;

const _: () = assert!(
    std::mem::size_of::<TranspositionEntry>() == std::mem::size_of::<TranspositionInternal>()
);

pub struct TranspositionInternal(AtomicU64);

impl TranspositionInternal {
    pub const EMPTY: Self = Self(AtomicU64::new(0));

    pub fn load(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }

    pub fn store(&self, entry: TranspositionEntry) {
        // Safety: both `TranspositionEntry` and `TranspositionInternal` are of size 8 bytes
        let internal = unsafe { std::mem::transmute(entry) };
        self.0.store(internal, Ordering::Relaxed);
    }
}

#[repr(C)]
pub struct TranspositionEntry {
    pub(super) key: u16,
    pub(super) mov: Option<Move>,
    pub(super) score: i16,
    pub(super) depth: i8,
    pub(super) bound: Bound,
}

impl TranspositionEntry {
    pub const SIZE: usize = std::mem::size_of::<TranspositionEntry>();

    pub fn mov(&self) -> Option<Move> {
        self.mov
    }

    pub fn score(&self) -> i32 {
        i32::from(self.score)
    }

    pub fn depth(&self) -> i32 {
        i32::from(self.depth)
    }

    pub fn bound(&self) -> Bound {
        self.bound
    }
}

impl From<&TranspositionInternal> for TranspositionEntry {
    fn from(value: &TranspositionInternal) -> Self {
        // Safety: `TranspositionEntry` and `TranspositionInternal` are of equal size
        unsafe { std::mem::transmute(value.load()) }
    }
}
