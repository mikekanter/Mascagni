use std::str::FromStr;

use crate::types::{Bitboard, Castling, Color, Move, Piece, Square, MAX_MOVES};

use crate::movegen::{generate_bishop_attacks, generate_queen_attacks, generate_rook_attacks, pawn_captures, StandardBitboards};

use self::parser::FenParseErr;

mod parser;

#[derive(Default, Copy, Clone)]
struct BoardState {
    en_passant: Square,
    halfmove_clock: u8,
    castling: Castling,
    fullmove_number: u16,
}

#[derive(Default, Copy, Clone)]
struct Check {
    checking_piece: Square,
    /// The squares that a piece can enter to block this check (including captures)
    block_space: Bitboard,
}

#[derive(Default, Copy, Clone)]
pub struct CheckState {
    check_space: Bitboard,
    /// TODO: Potentially change this to a slice
    checks: [Check; 2],
}

#[derive(Clone)]
pub struct Board {
    side_to_move: Color,
    colors: [Bitboard; Color::NUM],
    pieces: [Bitboard; Piece::NUM],
    state: BoardState,
    mailbox: [Piece; Square::NUM],
    standard_bitboards: StandardBitboards,
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

    pub fn our(&self, piece: Piece) -> Bitboard {
        self.pieces[piece] & self.colors[self.side_to_move]
    }

    pub fn their(&self, piece: Piece) -> Bitboard {
        self.pieces[piece] & self.colors[!self.side_to_move]
    }

    /// Get the check state for the current position
    pub fn check_state(&self) -> CheckState {
        let empty = Bitboard(0);
        // need way to iterate through pieces
        let our_nonkings = self.colors[self.side_to_move] ^ self.pieces[Piece::King];
        let our_king = self.our(Piece::King);

        // This occupancy bitboard contains everything except our kings
        let occupancy_bitboard = self.colors[!self.side_to_move] | our_nonkings;
        let their_pieces = self.colors[!self.side_to_move].clone();
        let mut iterated_pieces = their_pieces.clone();
        let mut answer = CheckState::default();
        while iterated_pieces > empty {
            let square = iterated_pieces.lsb();
            let sq_bb = Bitboard::from(square);
            iterated_pieces.clear(square);
            let piece = self.mailbox[square];
            let check_space: Bitboard = match piece {
                Piece::Pawn => pawn_captures(square, !self.side_to_move),
                Piece::Knight => self.standard_bitboards.knight_attacks[square],
                Piece::Bishop => generate_bishop_attacks(&square, &occupancy_bitboard),
                Piece::Rook => generate_rook_attacks(&square, &occupancy_bitboard),
                Piece::Queen => generate_queen_attacks(&square, &occupancy_bitboard),
                Piece::King => self.standard_bitboards.king_attacks[square],
                Piece::None => Bitboard::default(),
            };

            answer.check_space |= check_space;
            if !(check_space & our_king).is_empty() {
                // this piece checks our king
                let mut index = 0;
                if answer.checks[0].checking_piece == Square::None {
                    index = 1;
                }
                answer.checks[index].checking_piece = square;
                let king_square = our_king.lsb();
                // TODO. FIGURE OUT BLOCKING SPACE
                answer.checks[index].block_space = match piece {
                    // Non-ray attacks -> can only capture
                    Piece::Pawn => sq_bb,
                    Piece::Knight => sq_bb,
                    // TODO: Get squares in-between
                    Piece::Bishop => self.standard_bitboards.between[square][king_square] | sq_bb,
                    Piece::Rook => self.standard_bitboards.between[square][king_square] | sq_bb,
                    Piece::Queen => self.standard_bitboards.between[square][king_square] | sq_bb,
                    // Not possible that a check comes from one of the below
                    Piece::King => Bitboard(0),
                    Piece::None => Bitboard(0),
                };
            }
        }
        answer
    }

    pub fn generate_legal_moves(self) -> [Move; MAX_MOVES] {
        // First, let's find out if we are in check.
        let ms = [Move::default(); MAX_MOVES];
        let check_state = self.check_state();
        let mut total_checks: usize = 0;
        if check_state.checks[1].checking_piece != Square::None {
            // we are double-checked. can only move king.
            total_checks = 2;
        } else if check_state.checks[0].checking_piece != Square::None {
            // single-checked. Can only move king, or block / capture checker.
            total_checks = 1;
        }
        //
        // calculate their attack_space
        //
        let our_nonkings = self.colors[self.side_to_move] ^ self.pieces[Piece::King];
        let m = Move::default();
        todo!()
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
            standard_bitboards: StandardBitboards::new(),
        }
    }
}
