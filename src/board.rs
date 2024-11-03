use std::str::FromStr;

use crate::types::{Bitboard, Castling, Color, Piece, Square};

use self::parser::FenParseErr;

mod parser;

#[derive(Default, Copy, Clone)]
struct BoardState {
    en_passant: Square,
    halfmove_clock: u8,
    castling: Castling,
    fullmove_number: u16,
}

#[derive(Clone)]
pub struct Board {
    side_to_move: Color,
    colors: [Bitboard; Color::NUM],
    pieces: [Bitboard; Piece::NUM],
    state: BoardState,
    mailbox: [Piece; Square::NUM],
}

impl Board {
    // pub fn new(fen: &str) -> Result<Self, FenParseErr> {
    //     print!("{}", fen);
    //     todo!()
    // }
    pub fn new(fen: &str) -> Result<Self, FenParseErr> {
        Self::from_str(fen)
    }
    pub fn add_piece(&mut self, square: Square, color: Color, piece: Piece) {
        // TODO: Update mailbox
        self.pieces[piece].set(square);
        self.colors[color].set(square);
        self.mailbox[square] = piece;
    }

    pub fn our(self, piece: Piece) -> Bitboard {
        self.pieces[piece] & self.colors[self.side_to_move]
    }

    pub fn their(self, piece: Piece) -> Bitboard {
        self.pieces[piece] & self.colors[!self.side_to_move]
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            side_to_move: Color::White,
            pieces: [Bitboard::default(); Piece::NUM],
            colors: [Bitboard::default(); Color::NUM],
            state: BoardState::default(),
            mailbox: [Piece::None; Square::NUM],
        }
    }
}
