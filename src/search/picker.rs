use types::{Move, MoveFlag, PieceType};

use crate::{
    chess::{Capture, GenerationType, MoveList, MoveListEntry, PieceLayout, Quiet},
    search::worker::Worker,
};

// We sort our moves in stages to limit the amount of move generation
#[derive(PartialEq)]
enum Stage {
    TranspositionMove,
    GenerateCaptures,
    YieldCaptures,
    GenerateQuiets,
    YieldQuiets,
    Done,
}

pub struct MovePicker {
    moves: MoveList,
    tt: Option<Move>,
    stage: Stage,
    index: usize,
    quiet: bool,
}

impl MovePicker {
    pub fn new(mov: Option<Move>) -> Self {
        Self {
            moves: MoveList::new(),
            tt: mov,
            stage: Stage::TranspositionMove,
            index: 0,
            quiet: true,
        }
    }

    pub fn next(&mut self, worker: &Worker) -> Option<Move> {
        if self.stage == Stage::TranspositionMove {
            self.stage = Stage::GenerateCaptures;

            if self.tt.is_some() {
                return self.tt;
            }
        }

        if self.stage == Stage::GenerateCaptures {
            self.stage = Stage::YieldCaptures;

            self.extend::<Capture>(&worker);
        }

        if self.stage == Stage::YieldCaptures {
            if let Some(entry) = self.moves.get(self.index) {
                self.index += 1;
                return Some(entry.mov);
            }

            self.stage = Stage::GenerateQuiets;
        }

        if !self.quiet {
            self.stage = Stage::Done;
        }

        if self.stage == Stage::GenerateQuiets {
            self.stage = Stage::YieldQuiets;

            self.extend::<Quiet>(&worker);
        }

        if self.stage == Stage::YieldQuiets {
            if let Some(entry) = self.moves.get(self.index) {
                self.index += 1;
                return Some(entry.mov);
            }

            self.stage = Stage::Done;
        }

        None
    }

    pub fn set_quiet(&mut self, quiet: bool) {
        self.quiet = quiet;
    }

    fn score_captures(layout: &PieceLayout, moves: &mut [MoveListEntry]) {
        const VALUE: [i16; 6] = [1, 2, 3, 4, 5, 6];

        for entry in moves {
            let (start, target, flag) = (entry.mov.start(), entry.mov.target(), entry.mov.flag());

            let piece = layout.unchecked_at(start);
            let capture = match flag {
                MoveFlag::CAPTURE => layout.unchecked_at(target),
                MoveFlag::EN_PASSANT => PieceType::Pawn,
                _ => PieceType::Queen,
            };

            debug_assert!(capture != PieceType::King);

            entry.score = 100 * VALUE[capture] - VALUE[piece];
        }
    }

    fn score_quiets(worker: &Worker, moves: &mut [MoveListEntry]) {
        let us = worker.pos.stm();

        for entry in moves {
            entry.score = worker.history[us][entry.mov];
        }
    }

    fn extend<TYPE: GenerationType>(&mut self, worker: &Worker) {
        if TYPE::CAPTURE && !TYPE::QUIET {
            worker.pos.generate::<Capture>(&mut self.moves);
            MovePicker::score_captures(worker.pos.layout(), &mut self.moves[self.index..]);
        }

        if !TYPE::CAPTURE && TYPE::QUIET {
            worker.pos.generate::<Quiet>(&mut self.moves);
            MovePicker::score_quiets(worker, &mut self.moves[self.index..]);
        }

        self.moves[self.index..].sort_unstable_by(|a, b| b.score.cmp(&a.score));
    }
}
