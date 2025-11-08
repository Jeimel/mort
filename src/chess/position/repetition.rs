use crate::chess::Position;

impl Position {
    pub fn repetition(&self) -> bool {
        self.history
            .iter()
            .rev()
            // We have to consider all plys until the last irreversible move
            .take(self.board.state.rule50_ply as usize + 1)
            // A repetition can only happen two fullmoves ago
            .skip(3)
            // We only have to consider a position, where it is our turn
            .step_by(2)
            .any(|state| state.zobrist == self.board.state.zobrist)
    }
}
