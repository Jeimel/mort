mod score;
mod tables;

pub use score::{DRAW, INF, MATE, mate_in, mated_in};

use tables::{ENDGAME_TABLE, MIDGAME_TABLE};
use types::Color;

use crate::chess::Position;

const MIDGAME_VALUE: [i32; 6] = [82, 337, 365, 477, 1025, 0];
const ENDGAME_VALUE: [i32; 6] = [94, 281, 297, 512, 936, 0];

const PHASE: [i32; 6] = [0, 1, 1, 2, 4, 0];

pub fn evaluate(pos: &Position) -> i32 {
    let (layout, stm) = (pos.layout(), pos.stm());

    let (mut midgame, mut endgame, mut phase) = ([0, 0], [0, 0], 0);

    for color in [Color::Black, Color::White] {
        for sq in layout.color(color).iter() {
            let piece = layout.piece_at(sq);

            let sq = if color == Color::White { sq.flip() } else { sq };
            midgame[color] += MIDGAME_TABLE[piece][sq] + MIDGAME_VALUE[piece];
            endgame[color] += ENDGAME_TABLE[piece][sq] + ENDGAME_VALUE[piece];

            phase += PHASE[piece];
        }
    }

    phase = phase.min(24);

    debug_assert!(phase <= 24);

    ((midgame[stm] - midgame[!stm]) * phase + (endgame[stm] - endgame[!stm]) * (24 - phase)) / 24
}
