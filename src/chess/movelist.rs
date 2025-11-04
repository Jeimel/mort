use std::ops::{Deref, DerefMut};

use types::Move;

pub struct MoveListEntry {
    pub mov: Move,
    pub score: u16,
}

pub struct MoveList {
    inner: Vec<MoveListEntry>,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            inner: Vec::with_capacity(40),
        }
    }

    pub fn push(&mut self, mov: Move) {
        self.inner.push(MoveListEntry { mov, score: 0 });
    }

    pub fn iter(&self) -> impl Iterator<Item = Move> {
        self.inner.iter().map(|entry| entry.mov)
    }
}

impl Deref for MoveList {
    type Target = [MoveListEntry];

    fn deref(&self) -> &[MoveListEntry] {
        &self.inner
    }
}

impl DerefMut for MoveList {
    fn deref_mut(&mut self) -> &mut [MoveListEntry] {
        &mut self.inner
    }
}
