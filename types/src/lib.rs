mod castling;
mod chessmove;
mod color;
mod error;
mod file;
mod piece;
mod rank;
pub mod slider;
mod square;
mod squareset;

pub use castling::Castling;
pub use chessmove::{Move, MoveFlag};
pub use color::Color;
pub use error::TypeParseError;
pub use file::File;
pub use piece::PieceType;
pub use rank::Rank;
pub use square::Square;
pub use squareset::SquareSet;

pub type MoveList = Vec<Move>;
