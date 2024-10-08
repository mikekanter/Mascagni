use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub const NUM: usize = 6;

    pub const fn new(value: usize) -> Self {
        unsafe { std::mem::transmute(value as u8) }
    }
}

impl TryFrom<char> for Piece {
    type Error = ();

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            'p' | 'P' => Ok(Self::Pawn),
            'n' | 'N' => Ok(Self::Knight),
            'b' | 'B' => Ok(Self::Bishop),
            'r' | 'R' => Ok(Self::Rook),
            'q' | 'Q' => Ok(Self::Queen),
            'k' | 'K' => Ok(Self::King),
            _ => Err(()),
        }
    }
}

impl<T> Index<Piece> for [T] {
    type Output = T;

    fn index(&self, piece: Piece) -> &Self::Output {
        &self[piece as usize]
    }
}

impl<T> IndexMut<Piece> for [T] {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        &mut self[piece as usize]
    }
}
