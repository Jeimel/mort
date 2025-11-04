#![feature(adt_const_params)]

mod castling;
mod chessmove;
mod color;
mod error;
mod file;
mod piece;
mod rank;
pub mod slider;
mod square;
pub mod squareset;

pub use castling::Castling;
pub use chessmove::{Move, MoveFlag};
pub use color::Color;
pub use error::TypeParseError;
pub use file::File;
pub use piece::{Piece, PieceType};
pub use rank::Rank;
pub use square::Square;
pub use squareset::SquareSet;

pub type MoveList = Vec<Move>;

#[macro_export]
macro_rules! const_for {
    ($init:stmt; $condition:expr; $next: expr; $body:block) => {
        $init
        while $condition {
            $body;
            $next;
        }
    };
}
