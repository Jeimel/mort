use std::ops::{Deref, DerefMut};

use arrayvec::ArrayVec;
use types::Move;

const MAX_MOVES: usize = 218;

#[macro_export]
macro_rules! push_loop {
    ($moves:expr, $set:expr, $start:expr, $flag:expr) => {
        for target in $set.iter() {
            $moves.push(Move::new($start, target, $flag));
        }
    };
}

pub struct MoveListEntry {
    pub mov: Move,
    pub score: u16,
}

pub struct MoveList {
    inner: ArrayVec<MoveListEntry, MAX_MOVES>,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            inner: ArrayVec::new(),
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
