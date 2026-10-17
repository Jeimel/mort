use std::sync::atomic::{AtomicU8, Ordering};

use crate::search::transposition::{TranspositionView, bucket::Bucket};

pub struct TranspositionTable {
    table: Vec<Bucket>,
    age: AtomicU8,
}

impl TranspositionTable {
    const MEGABYTE: usize = 1024 * 1024;

    pub const fn new() -> Self {
        Self {
            table: Vec::new(),
            age: AtomicU8::new(0),
        }
    }

    pub fn view(&self) -> TranspositionView<'_> {
        TranspositionView {
            table: &self.table,
            age: self.age.load(Ordering::Relaxed),
        }
    }

    pub fn increase_age(&self) {
        self.age.fetch_add(1, Ordering::Relaxed);
    }

    pub fn resize(&mut self, mb: usize) {
        let size = mb * Self::MEGABYTE / Bucket::SIZE;

        self.table.clear();
        self.table.resize_with(size, || Bucket::EMPTY);
    }

    pub fn clear(&mut self) {
        for entry in self.table.iter_mut() {
            *entry = Bucket::EMPTY;
        }
    }
}
