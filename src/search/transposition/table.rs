use crate::search::transposition::{
    TranspositionEntry, TranspositionView, entry::TranspositionInternal,
};

pub struct TranspositionTable {
    table: Vec<TranspositionInternal>,
}

impl TranspositionTable {
    const MEGABYTE: usize = 1024 * 1024;

    pub const fn new() -> Self {
        Self { table: Vec::new() }
    }

    pub fn view(&self) -> TranspositionView<'_> {
        TranspositionView { table: &self.table }
    }

    pub fn resize(&mut self, mb: usize) {
        let size = mb * Self::MEGABYTE / TranspositionEntry::SIZE;

        self.table.clear();
        self.table
            .resize_with(size, || TranspositionInternal::EMPTY);
    }

    pub fn clear(&mut self) {
        for entry in self.table.iter_mut() {
            *entry = TranspositionInternal::EMPTY;
        }
    }
}
