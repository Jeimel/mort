use types::{Castling, Square, SquareSet};

pub struct Board {
    colors: [SquareSet; 2],
    kings: [Square; 2],
    rooks: SquareSet,
    bishops: SquareSet,
    pawns: SquareSet,
    castling: Castling,
    en_passant: Option<Square>,
}
