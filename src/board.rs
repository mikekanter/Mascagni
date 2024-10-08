use crate::types::{Bitboard, Color, Piece, Square};

use self::parser::FenParseErr;

mod parser;

#[derive(Default, Copy, Clone)]
struct BoardState {
    en_passant: Square,
    halfmove_clock: u8,
    fullmove_number: u16,
}

#[derive(Clone)]
pub struct Board {
    side_to_move: Color,
    colors: [Bitboard; Color::NUM],
    pieces: [Bitboard; Piece::NUM],
    state: BoardState,
}

impl Board {
    pub fn new(fen: &str) -> Result<Self, FenParseErr> {
        print!("{}", fen);
        todo!()
    }

    pub fn add_piece(&mut self, square: Square, color: Color, piece: Piece) {
        // TODO: Update mailbox
        self.pieces[piece].set(square);
        self.colors[color].set(square);
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            side_to_move: Color::White,
            pieces: [Bitboard::default(); Piece::NUM],
            colors: [Bitboard::default(); Color::NUM],
            state: BoardState::default(),
        }
    }
}
