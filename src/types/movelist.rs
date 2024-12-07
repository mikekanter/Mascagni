use super::{Bitboard, Move, MoveType, Square, MAX_MOVES};

/// collects move objects
/// possibly replace with a vec?
#[derive(Clone, Copy)]
pub struct MoveList {
    pub moves: [Move; MAX_MOVES],
    pub len: usize,
}

impl MoveList {
    pub fn push(&mut self, m: Move) {
        self.moves[self.len] = m;
        self.len += 1;
    }

    pub fn add(&mut self, start: Square, target: Square, move_type: MoveType) {
        self.push(Move::new(start, target, move_type))
    }

    pub fn add_many(&mut self, start: Square, targets: Bitboard, move_type: MoveType) {
        for t in targets {
            self.add(start, t, move_type)
        }
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self {
            moves: [Move::default(); MAX_MOVES],
            len: 0,
        }
    }
}
