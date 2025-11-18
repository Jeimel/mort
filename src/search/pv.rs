use std::fmt::Display;

use arrayvec::ArrayVec;
use types::Move;

use crate::{evaluation::DRAW, search::MAX_DEPTH};

#[derive(Clone)]
pub struct PrincipalVariation {
    pub(crate) line: ArrayVec<Move, MAX_DEPTH>,
    pub(crate) score: i32,
}

impl Display for PrincipalVariation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for mov in &self.line {
            write!(f, "{} ", mov)?;
        }

        Ok(())
    }
}

impl PrincipalVariation {
    pub const EMPTY: Self = Self {
        score: DRAW,
        line: ArrayVec::new_const(),
    };

    pub fn collect(&mut self, mov: Move, score: i32, other: &Self) {
        self.score = score;

        self.line.clear();
        self.line.push(mov);
        self.line
            .try_extend_from_slice(&other.line)
            .expect("PV can't be longer than `MAX_DEPTH`");
    }
}
