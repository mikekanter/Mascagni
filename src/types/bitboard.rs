use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

use super::{Rank, Square};


/// 64-bit unsigned. Each bit indicates a square's occupancy
#[derive(Copy, Clone, PartialEq, Default)]
#[repr(transparent)]
pub struct Bitboard(pub u64);

impl Bitboard {
    /// Checks if bitboard has zero bits set to 1
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    /// Number of bits set to 1
    pub const fn count(self) -> usize {
        self.0.count_ones() as usize
    }
    pub const fn rank(rank: Rank) -> Self {
        todo!();
    }

    /// Returns first set bit in bitboard
    pub const fn lsb(self) -> Square {
        // trailing_zeroes() is the number of trailing zeroes in the binary number
        Square::new(self.0.trailing_zeros() as u8)
    }

    /// Pops / returns least significant set bit in bitboard
    pub fn pop(&mut self) -> Square {
        let lsb = self.lsb();
        self.0 &= self.0 - 1;
        lsb
    }

    /// Sets a specific bit
    pub fn set(&mut self, square: Square) {
        self.0 |= 1 << square as u64;
    }

    /// Clears a specific bit
    pub fn clear(&mut self, square: Square) {
        self.0 &= !(1 << square as u64);
    }
}

impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(self.pop())
        }
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

fn pretty_print(b: Bitboard) {
    let mut a = "-".repeat(64);
    b.for_each(|square| {
        let b = square.index();
        let c = b + 1;
        a.replace_range(b..c, "X")
    })
}
