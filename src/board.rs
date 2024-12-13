use std::str::FromStr;

use crate::types::{Bitboard, BlackKingside, BlackQueenside, Castling, CastlingKind, Color, File, MoveList, MoveType, Piece, Rank, Square, WhiteKingside, WhiteQueenside};

use self::movegen::{black_pawn_advances, generate_bishop_attacks, generate_queen_attacks, generate_rook_attacks, white_pawn_advances, StandardBitboards};

use self::parser::FenParseErr;

mod parser;
mod movegen;
mod makemove;

/// Basic state of the board
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
    // basic board state
    pub side_to_move: Color,
    state: BoardState,
    // pieces / mailboxes
    colors: [Bitboard; Color::NUM],
    pieces: [Bitboard; Piece::NUM],
    mailbox: [Piece; Square::NUM],
    standard_bitboards: StandardBitboards,
    // calculated board state
    checking_state: CheckState,
    pinning_state: [Bitboard; Square::NUM],
    pub legal_moves: MoveList,
}


impl Board {
    // pub fn new(fen: &str) -> Result<Self, FenParseErr> {
    //     print!("{}", fen);
    //     todo!()
    // }
    /// Create a new board from a given fen
    pub fn new(fen: String) -> Result<Self, FenParseErr> {
        match Self::from_str(&fen) {
            Ok(mut board) => {
                board.analyze_board();
                Ok(board)
            }
            Err(err) => {
                println!("There was an error: {}", err);
                Err(err)
            }
        }
    }

    /// Should be run on board creation and after each move. This function generates legal moves
    /// and updates other basic state about the board.
    pub fn analyze_board(&mut self) {
        self.checking_state = self.calculate_check_state();
        self.pinning_state = self.calculate_pin_state();
        self.legal_moves = self.generate_legal_moves();
    }

    /// determine if the current side to move is checkmated.
    pub fn is_checkmate(&self) -> bool {
        if self.legal_moves.len == 0 && self.checking_state.checks[0].checking_piece != Square::None {
            true
        } else {
            false
        }
    }

    /// determine if the current position is a stalemate
    pub fn is_stalemate(&self) -> bool {
        if self.legal_moves.len == 0 && self.checking_state.checks[0].checking_piece == Square::None {
            true
        } else {
            false
        }
    }

    pub fn piece_on(&self, square: Square) -> Piece {
        self.mailbox[square]
    }

    pub fn add_piece(&mut self, square: Square, color: Color, piece: Piece) {
        // TODO: Update mailbox?
        self.pieces[piece].set(square);
        self.colors[color].set(square);
        self.mailbox[square] = piece;
    }

    pub fn remove_piece(&mut self, square: Square, color: Color, piece: Piece) {
        self.pieces[piece].clear(square);
        self.colors[color].clear(square);
        self.mailbox[square] = Piece::None;
    }

    pub fn our(&self, piece: Piece) -> Bitboard {
        self.pieces[piece] & self.colors[self.side_to_move]
    }

    pub fn their(&self, piece: Piece) -> Bitboard {
        self.pieces[piece] & self.colors[!self.side_to_move]
    }

    /// Returns an array of bitboards which describes the absolute pin state of the current
    /// position. For any square `sq`, we can retrieve the bitboard `b` at `sq`'s index. If one of
    /// our pieces is on `sq`, then its moves are, at most, restricted to the squares that are set
    /// on `b`. Note that `b` is naive as to the type of piece on `sq`.
    /// example:
    /// ```rust
    /// let sq = Square::E4;
    /// // (assume one of our pieces is on E4)
    /// let pins = self.calculate_pin_state();
    /// let pins_for_sq = pins[sq];
    /// // The piece on E4 cannot legally move away from the pins_for_sq bitboard
    /// ```
    pub fn calculate_pin_state(&self) -> [Bitboard; Square::NUM] {
        let their_sliders = self.their(Piece::Rook)
            | self.their(Piece::Bishop)
            | self.their(Piece::Queen);
        let our_king_bb = self.our(Piece::King);
        let our_king_square = our_king_bb.lsb();
        let universal_bitboard = Bitboard::universal();
        let mut pin_spaces: [Bitboard; Square::NUM] = [universal_bitboard.clone(); Square::NUM];
        for square in their_sliders {
            let attack_space = match self.mailbox[square] {
                Piece::Bishop => generate_bishop_attacks(&square, &our_king_bb),
                Piece::Rook => generate_rook_attacks(&square, &our_king_bb),
                Piece::Queen => generate_queen_attacks(&square, &our_king_bb),
                _ => Bitboard(0),
            };
            let between_space = attack_space
                    & self.standard_bitboards.between[square][our_king_square];
            let our_betweeners = between_space & self.colors[self.side_to_move];
            let their_betweeners = between_space & self.colors[!self.side_to_move];
            if our_betweeners.count() == 1 && their_betweeners.count() == 0 {
                // there is exactly one piece between their slider and our king
                // and that piece is of our color
                let mut pin_bb = between_space.clone();
                pin_bb.set(square);
                let lsb = our_betweeners.lsb();
                pin_spaces[lsb] = pin_bb;
            }
        }
        pin_spaces
    }

    /// Get the check state for the current position
    pub fn calculate_check_state(&self) -> CheckState {
        let our_nonkings = self.colors[self.side_to_move] ^ self.pieces[Piece::King];
        let our_king = self.our(Piece::King);

        // This occupancy bitboard contains everything except our kings
        let occupancy_bitboard = self.colors[!self.side_to_move] | our_nonkings;
        let their_pieces = self.colors[!self.side_to_move].clone();
        let mut answer = CheckState::default();
        for square in their_pieces {
            let sq_bb = Bitboard::from(square);
            let piece = self.mailbox[square];
            let check_space: Bitboard = match piece {
                Piece::Pawn => self.standard_bitboards.pawn_captures[!self.side_to_move][square],
                Piece::Knight => self.standard_bitboards.knight_attacks[square],
                Piece::Bishop => generate_bishop_attacks(&square, &occupancy_bitboard),
                Piece::Rook => generate_rook_attacks(&square, &occupancy_bitboard),
                Piece::Queen => generate_queen_attacks(&square, &occupancy_bitboard),
                Piece::King => self.standard_bitboards.king_attacks[square],
                Piece::None => Bitboard::default(),
            };
            answer.check_space |= check_space;
            if !((check_space & our_king).is_empty()) {
                // this piece checks our king
                let mut index = 0;
                if answer.checks[0].checking_piece != Square::None {
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

    pub fn generate_king_moves(&self, check_state: &CheckState) -> Bitboard {
        let our_king = self.our(Piece::King);
        let mut bb = Bitboard(0);
        for king_square in our_king {
            let reachable_squares = !check_state.check_space
                & self.standard_bitboards.king_attacks[king_square]
                & !self.colors[self.side_to_move];
            bb |= reachable_squares;
        }
        bb
    }

    pub fn generate_legal_moves(&self) -> MoveList {
        // First, let's find out if we are in check.
        let mut mvs = MoveList::default();
        let check_state = self.checking_state;
        let pin_state = self.pinning_state;
        let full_occupancy = self.colors[self.side_to_move] | self.colors[!self.side_to_move];
        let our_pieces = self.colors[self.side_to_move];
        let their_pieces = self.colors[!self.side_to_move];
        let our_king = self.our(Piece::King);
        let king_reachable_squares = self.generate_king_moves(&check_state);
        for king_square in our_king {
            let captures = king_reachable_squares & self.colors[!self.side_to_move];
            let non_captures = king_reachable_squares & !captures;
            mvs.add_many(king_square, captures, MoveType::Capture);
            mvs.add_many(king_square, non_captures, MoveType::Quiet);
        }
        if check_state.checks[1].checking_piece != Square::None {
            // we are double-checked. can only move king.
            return mvs
        }
        let in_check = check_state.checks[0].checking_piece != Square::None;
        // single-checked. Can only move king, or block / capture checker.
        let relevant_check = check_state.checks[0];

        // king moves have already been handled
        // let us take a look at non-king moves

        let our_non_kings = self.colors[self.side_to_move] & !our_king;

        for square in our_non_kings {
            let piece = self.mailbox[square];
            // legal moves for the piece, taking into account absolute pins (but not yet taking
            // into account checks)
            let mut reachable_squares = match piece {
                Piece::Rook => generate_rook_attacks(&square, &full_occupancy) & !our_pieces,
                Piece::Bishop => generate_bishop_attacks(&square, &full_occupancy) & !our_pieces,
                Piece::Queen => generate_queen_attacks(&square, &full_occupancy) & !our_pieces,
                Piece::Knight => self.standard_bitboards.knight_attacks[square] & !our_pieces,
                Piece::Pawn => {
                    let en_passant_bb = match self.state.en_passant {
                        Square::None => Bitboard(0),
                        _ => Bitboard::from(self.state.en_passant),
                    };
                    let captures = self.standard_bitboards.pawn_captures[self.side_to_move][square] & (their_pieces | en_passant_bb);

                    // TODO: Define pawn advances
                    let advances = match self.side_to_move {
                        Color::White => white_pawn_advances(&square, &full_occupancy),
                        Color::Black => black_pawn_advances(&square, &full_occupancy),
                    };

                    captures | advances
                },
                Piece::None => Bitboard(0),
                // this is for king (which shan't be possible).
                _ => Bitboard(0),
            } & pin_state[square];

            if in_check {
                // if in check, our only choices are moving the king, capturing the checking piece,
                // or blocking the capture. we have already calculated king moves, these are the
                // blocks / captures.
                reachable_squares &= relevant_check.block_space;
            }
            // now, we add each move
            for target in reachable_squares {
                match piece {
                    Piece::Pawn => {
                        let promotion_rank: Rank = match self.side_to_move {
                            Color::White => Rank::R8,
                            Color::Black => Rank::R1
                        };
                        // pawns have a ton of edge cases
                        let is_advance = target.file() == square.file();
                        if target.rank() == promotion_rank {
                            if is_advance {
                                mvs.add(square, target, MoveType::PromotionToRook);
                                mvs.add(square, target, MoveType::PromotionToQueen);
                                mvs.add(square, target, MoveType::PromotionToBishop);
                                mvs.add(square, target, MoveType::PromotionToKnight);
                            } else {
                                mvs.add(square, target, MoveType::PromotionCaptureToRook);
                                mvs.add(square, target, MoveType::PromotionCaptureToQueen);
                                mvs.add(square, target, MoveType::PromotionCaptureToBishop);
                                mvs.add(square, target, MoveType::PromotionCaptureToKnight);
                            }
                        } else if !is_advance {
                            if target == self.state.en_passant {
                                mvs.add(square, target, MoveType::EnPassant)
                            } else {
                                mvs.add(square, target, MoveType::Capture)
                            }
                        } else if target == square.shift(16) || target == square.shift(-16) {
                            mvs.add(square, target, MoveType::DoublePawnPush);
                        } else {
                            mvs.add(square, target, MoveType::Quiet);
                        }
                    },
                    _ => {
                        // we don't have to check if the piece is the right color, since we know it
                        // is able to move there
                        let piece = self.mailbox[target];
                        if piece == Piece::None {
                            mvs.add(square, target, MoveType::Quiet)
                        } else {
                            mvs.add(square, target, MoveType::Capture)
                        }
                    }
                };
            }
        }
        if self.side_to_move == Color::White {
            self.generate_castling::<WhiteKingside>(&mut mvs);
            self.generate_castling::<WhiteQueenside>(&mut mvs);
        } else {
            self.generate_castling::<BlackKingside>(&mut mvs);
            self.generate_castling::<BlackQueenside>(&mut mvs);
        }
        mvs
    }

    pub fn generate_castling<Kind: CastlingKind>(&self, list: &mut MoveList) {
        if !self.state.castling.is_allowed::<Kind>() {
            // lost castling rights
            return
        }
        let full_occupancies = self.colors[self.side_to_move] | self.colors[!self.side_to_move];
        if !((Kind::PATH_MASK & full_occupancies).is_empty()) {
            // cant castle because there is a piece between the king and the rooks
            return
        }
        // if check_space includes the check_squares, we can't do it
        for sq in Kind::CHECK_SQUARES {
            let bb = Bitboard::from(sq);
            if !((self.checking_state.check_space & bb).is_empty()) {
                return
            }
        }
        list.push(Kind::CASTLING_MOVE);
    }

    pub fn pretty_print(&self) {
        let mut cur_square = Square::A1;
        let mut lines: [String; 8] = [(); 8].map(|_| String::new());
        let mut current_line = 0;

        while cur_square <= Square::H8 {
            let piece = self.mailbox[cur_square];
            let mut new_char: String = match piece {
                Piece::King => "k",
                Piece::Queen => "q",
                Piece::Knight => "n",
                Piece::Bishop => "b",
                Piece::Rook => "r",
                Piece::Pawn => "p",
                _ => "-",
            }.to_owned();

            let is_white = !(self.colors[Color::White] & Bitboard::from(cur_square)).is_empty();

            new_char = match is_white {
                true => new_char.to_uppercase(),
                false => new_char
            };

            lines[current_line] = lines[current_line].to_owned() + &new_char;

            if cur_square.file() == File::H {
                current_line += 1;
            }
            cur_square = cur_square.shift(1);
        }
        lines.reverse();
        println!("{}", lines.join("\n"));
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
            checking_state: CheckState::default(),
            pinning_state: [Bitboard(u64::MAX); Square::NUM],
            legal_moves: MoveList::default(),
        }
    }
}
