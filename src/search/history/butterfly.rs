use std::ops::{Deref, DerefMut, Index};

use types::Move;

const BUTTERFLY_SIZE: usize = u16::MAX as usize + 1;

pub struct ButterflyBoard<const MAX: i16>([i16; BUTTERFLY_SIZE]);

impl<const MAX: i16> ButterflyBoard<MAX> {
    pub const EMPTY: Self = Self([0; BUTTERFLY_SIZE]);

    pub fn update(&mut self, mov: Move, bonus: i16) {
        let bonus = bonus.clamp(-MAX, MAX);

        self.0[mov.inner() as usize] = bonus;
    }
}

impl<const MAX: i16> Index<Move> for ButterflyBoard<MAX> {
    type Output = i16;

    fn index(&self, mov: Move) -> &Self::Output {
        &self.0[mov.inner() as usize]
    }
}

pub struct ButterflyHistory([ButterflyBoard<{ Self::MAX_VALUE }>; 2]);

impl ButterflyHistory {
    pub const EMPTY: Self = Self([ButterflyBoard::EMPTY; 2]);

    const MAX_VALUE: i16 = 16384;
}

impl Deref for ButterflyHistory {
    type Target = [ButterflyBoard<{ Self::MAX_VALUE }>; 2];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ButterflyHistory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
