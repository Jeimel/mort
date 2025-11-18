// Taken from: https://vigna.di.unimi.it/ftp/papers/xorshift.pdf
pub struct XorShiftState {
    pub state: u64,
}

impl XorShiftState {
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub const fn next(&self) -> (u64, u64) {
        debug_assert!(self.state != 0);

        let mut state = self.state;

        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;

        (state, state.wrapping_mul(2685821657736338717))
    }
}
