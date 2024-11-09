pub mod bitboard;
pub mod square;
pub mod color;
pub mod piece;
pub mod castling;
pub mod rankfile;
pub mod moves;

pub use bitboard::*;
pub use square::*;
pub use color::*;
pub use piece::*;
pub use castling::*;
pub use rankfile::*;
pub use moves::*;

/// Per chess programming wiki, max moves in a position is 218
pub const MAX_MOVES: usize = 218;
