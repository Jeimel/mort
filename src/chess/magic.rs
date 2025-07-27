use super::{BitBoard, types::Square};

const ROOK_SHIFT_BITS: u8 = 64 - 12;

const BISHOP_SHIFT_BITS: u8 = 64 - 9;

const ROOK_MAGICS: &[BlackMagicEntry; 64] = todo!();

const BISHOP_MAGICS: &[BlackMagicEntry; 64] = todo!();

struct BlackMagicEntry {
    neg_mask: BitBoard,
    magic: u64,
    offset: u32,
}

fn magic_index(
    magics: &[BlackMagicEntry; 64],
    shift_bits: u8,
    sq: Square,
    blockers: BitBoard,
) -> usize {
    let entry = &magics[sq];
    let hash = (blockers | entry.neg_mask).0.wrapping_mul(entry.magic);

    entry.offset as usize + (hash >> shift_bits) as usize
}

pub fn rook_magic_index(sq: Square, blockers: BitBoard) -> usize {
    magic_index(ROOK_MAGICS, ROOK_SHIFT_BITS, sq, blockers)
}

pub fn bishop_magic_index(sq: Square, blockers: BitBoard) -> usize {
    magic_index(BISHOP_MAGICS, BISHOP_SHIFT_BITS, sq, blockers)
}
