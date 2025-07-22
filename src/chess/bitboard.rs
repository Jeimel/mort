/// A `BitBoard` represents a board as array of 64 bits.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct BitBoard(pub u64);

impl BitBoard {}
