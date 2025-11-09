use crate::chess::Position;

impl Position {
    pub(crate) fn repetition(&self) -> bool {
        let mut repetitions = 0;

        for (distance, state) in self
            .history
            .iter()
            .rev()
            .enumerate()
            // We have to consider all ply until the last irreversible move
            .take(self.board.state.rule50_ply as usize)
            // A repetition can only happen two fullmoves ago
            .skip(3)
            // We only have to consider a position, where it is our turn
            .step_by(2)
        {
            if state.zobrist != self.board.state.zobrist {
                continue;
            }

            if distance < self.ply {
                return true;
            }

            repetitions += 1;

            if repetitions >= 2 {
                return true;
            }
        }

        false
    }
}
