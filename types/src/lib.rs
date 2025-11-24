//! Centralized repository for chess-relatad types for `Mort`.
//!
//! Its primary role is to support `build.rs` in precomputing black magic bitboards for sliding
//! piece move generation,

#![feature(adt_const_params)]

mod castling;
mod chessmove;
mod color;
mod error;
mod file;
mod piece;
mod rank;
mod slider;
mod square;
mod squareset;

pub use castling::Castling;
pub use chessmove::{Move, MoveFlag};
pub use color::Color;
pub use error::TypeParseError;
pub use file::File;
pub use piece::{Piece, PieceType};
pub use rank::Rank;
pub use slider::{BISHOP, ROOK, Slider, magic};
pub use square::Square;
pub use squareset::{SquareIter, SquareSet, SquareSubsetIter};

/// Compact compile-time `for`-loop
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
