use std::sync::atomic::{AtomicU64, Ordering};

use crate::{chess::Move, search::transposition::Bound};

const _: () = assert!(
    std::mem::size_of::<TranspositionEntry>() == std::mem::size_of::<TranspositionInternal>()
);

pub struct TranspositionInternal(AtomicU64);

impl TranspositionInternal {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const EMPTY: Self = Self(AtomicU64::new(0));

    pub fn load(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }

    pub fn store(&self, entry: TranspositionEntry) {
        // Safety: both `TranspositionEntry` and `TranspositionInternal` are of size 8 bytes
        let internal = unsafe { std::mem::transmute::<TranspositionEntry, u64>(entry) };
        self.0.store(internal, Ordering::Relaxed);
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TranspositionEntry {
    pub(super) key: u16,
    pub(super) mov: Option<Move>,
    pub(super) score: i16,
    pub(super) depth: i8,
    pub(super) meta: u8,
}

impl TranspositionEntry {
    pub(super) const AGE_MASK: u8 = 0b0011_1111;

    const BOUND_BITS: u8 = 2;

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
        match self.meta & 0b11 {
            0 => Bound::Exact,
            1 => Bound::Upper,
            2 => Bound::Lower,
            _ => unreachable!(),
        }
    }

    pub fn age(&self) -> u8 {
        self.meta >> Self::BOUND_BITS
    }

    pub(super) fn pack(age: u8, bound: Bound) -> u8 {
        (age << Self::BOUND_BITS) | bound as u8
    }
}

impl From<&TranspositionInternal> for TranspositionEntry {
    fn from(value: &TranspositionInternal) -> Self {
        // Safety: `TranspositionEntry` and `TranspositionInternal` are of equal size
        unsafe { std::mem::transmute(value.load()) }
    }
}
