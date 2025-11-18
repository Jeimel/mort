mod entry;
mod table;
mod view;

pub use entry::TranspositionEntry;
pub use table::TranspositionTable;
pub use view::TranspositionView;

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Bound {
    Exact,
    Upper,
    Lower,
}
