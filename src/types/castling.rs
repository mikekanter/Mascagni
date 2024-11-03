use std::ops::Index;

use super::{Bitboard, Square};

pub trait CastlingKind {
    /// The mask of the catling kind
    const MASK: u8;
    /// The path mask = squares that must be empty to legally castle
    const PATH_MASK: Bitboard;
    /// The check squares are the squares that must not be in check for the castling to occur
    const CHECK_SQUARES: [Square; 2];
    // The move associated with such a castling kind.
    // const CASTLING_MOVE
}

macro_rules! impl_castling_kind {
    ($($kind: ident => $mask:expr, $start_square:expr, $through_square:expr, $target_square: expr,)*) => {
        $(
            pub struct $kind;

            impl CastlingKind for $kind {
                const MASK: u8 = $mask;
                const PATH_MASK: Bitboard = Bitboard((1 << ($start_square.index() + 1)) | (1 << ($through_square.index() + 1)));
                const CHECK_SQUARES: [Square; 2] = [$start_square, $through_square];
            }
        )*
    };
}

impl_castling_kind! {
    WhiteKingside => 1, Square::E1, Square::F1, Square::G1,
    WhiteQueenside => 2, Square::E1, Square::D1, Square::C1,
    BlackKingside => 4, Square::E8, Square::F8, Square::G8,
    BlackQueenside => 4, Square::E8, Square::D8, Square::C8,
}

#[derive(Copy, Clone, Default)]
#[repr(transparent)]
pub struct Castling {
    raw: u8
}

impl Castling {
    /// We represent castling _rights_ as 4 bits, where:
    /// first bit: 1 White, Kingside castling
    /// second bit: 2 white, queenside castling
    /// third bit: 4 black kingside castling
    /// fourth bit: 8 black queenside castling.
    /// So, 1001 would mean white kingside castling and black queenside castling rights are still
    /// intact
    /// So, when a square is moved from a given square, the rights update
    const UPDATES: [u8; Square::NUM] = [
        13, 15, 15, 15, 12, 15, 15, 14,
        15, 15, 15, 15, 15, 15, 15, 15,
        15, 15, 15, 15, 15, 15, 15, 15,
        15, 15, 15, 15, 15, 15, 15, 15,
        15, 15, 15, 15, 15, 15, 15, 15,
        15, 15, 15, 15, 15, 15, 15, 15,
        15, 15, 15, 15, 15, 15, 15, 15,
        7, 15, 15, 15, 3, 15, 15, 11,
    ];

    /// This updates the castling rights for a given move.
    /// It is important to include the target square because if a rook is captured, the player
    /// should not be allowed to castle to that side.
    pub fn update(&mut self, start: Square, target: Square) {
        self.raw &= Self::UPDATES[start] & Self::UPDATES[target];
    }

    pub const fn is_allowed<Kind: CastlingKind>(self) -> bool {
        self.raw & Kind::MASK != 0
    }
}

impl From<&str> for Castling {
    fn from(value: &str) -> Self {
        let mut castling = Self::default();
        for c in value.chars() {
            castling.raw |= match c {
                'K' => WhiteKingside::MASK,
                'Q' => WhiteQueenside::MASK,
                'k' => BlackKingside::MASK,
                'q' => BlackQueenside::MASK,
                _ => continue,
            };
        }
        castling
    }
}

impl<T> Index<Castling> for [T] {
    type Output = T;

    fn index(&self, index: Castling) -> &Self::Output {
        &self[index.raw as usize]
    }
}
