pub mod bitboard;
pub mod square;
pub mod color;
pub mod piece;
pub mod castling;

pub use bitboard::*;
pub use square::*;
pub use color::*;
pub use piece::*;
pub use castling::*;

/// Per chess programming wiki, max moves in a position is 218
pub const MAX_MOVES: usize = 218;

pub enum Rank { R1, R2, R3, R4, R5, R6, R7, R8 }
