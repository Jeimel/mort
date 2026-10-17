use std::ops::Deref;

use crate::search::transposition::entry::TranspositionInternal;

#[repr(C, align(32))]
pub struct Bucket([TranspositionInternal; 4]);

impl Bucket {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const EMPTY: Self = Self([TranspositionInternal::EMPTY; 4]);

    pub const SIZE: usize = std::mem::size_of::<Bucket>();
}

impl Deref for Bucket {
    type Target = [TranspositionInternal];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
