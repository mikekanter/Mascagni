use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Add, Sub, Mul};

use super::{Rank, File, Square};


/// 64-bit unsigned. Each bit indicates a square's occupancy
#[derive(Copy, Clone, PartialEq, PartialOrd, Default)]
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
        Self(255 << (rank as u8))
    }

    pub const fn file(file: File) -> Self {
        let index = file as usize;
        Self(0x101010101010101 << index)
    }

    /// lsb = least set bit? Returns first set bit in bitboard
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

impl From<Square> for Bitboard {
    fn from(square: Square) -> Self {
        Self(1 << square.index())
    }
}

impl From<File> for Bitboard {
    fn from(file: File) -> Self {
        Self(0x101010101010101 << file.index())
    }
}

impl From<Rank> for Bitboard {
    fn from(rank: Rank) -> Self {
        Self(255 << rank.index())
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

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Add for Bitboard {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Bitboard {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Bitboard {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Not for Bitboard {
    type Output= Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
