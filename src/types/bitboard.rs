use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

use super::Square;

/// 64-bit unsigned. Each bit indicates a square's occupancy
#[derive(Copy, Clone, PartialEq, Default)]
#[repr(transparent)] pub struct Bitboard(pub u64);
impl Bitboard {
    /// Checks if bitboard has zero bits set to 1
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    /// Number of bits set to 1
    pub const fn count(self) -> usize {
        self.0.count_ones() as usize
    }
}

impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl Not for Bitboard {
    type Output= Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
