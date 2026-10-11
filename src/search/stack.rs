use crate::chess::Move;

pub struct StackEntry {
    pub eval: i32,
    pub mov: Option<Move>,
}

impl StackEntry {
    pub const EMPTY: Self = Self { eval: 0, mov: None };
}
