use crate::types::{Color, FullMove, Move, MoveType, Piece, Square};

use super::Board;

impl Board {
    pub fn make_move(&mut self, mv: Move) {
        self.state_stack.push(self.state);
        let start = mv.start();
        let target = mv.target();
        let piece = self.piece_on(start);

        if piece == Piece::None {
            // this is no good
            return
        }

        if start == target {
            // this is no good
            return
        }


        let kind = mv.kind();

        //
        // Update pieces on the board
        //

        let mut captured_piece = Piece::None;

        // Remove any captured piece
        if mv.is_capture() {
            // we have to capture
            let capture_square = if mv.is_en_passant() {
                match self.side_to_move {
                    Color::White => target.shift(-8),
                    Color::Black => target.shift(8),
                }
            } else {
                target
            };

            captured_piece = self.piece_on(capture_square);

            self.remove_piece(capture_square, !self.side_to_move, captured_piece);
        }

        // remove the moved piece from its own square
        self.remove_piece(start, self.side_to_move, piece);

        // place the moved piece (or promoted piece) on target square.
        let placed_piece: Piece = match kind {
            MoveType::PromotionToKnight => Piece::Knight,
            MoveType::PromotionToBishop => Piece::Bishop,
            MoveType::PromotionToRook => Piece::Rook,
            MoveType::PromotionToQueen => Piece::Queen,
            MoveType::PromotionCaptureToKnight => Piece::Knight,
            MoveType::PromotionCaptureToBishop => Piece::Bishop,
            MoveType::PromotionCaptureToRook => Piece::Rook,
            MoveType::PromotionCaptureToQueen => Piece::Queen,
            _ => piece,
        };
        self.add_piece(target, self.side_to_move, placed_piece);

        // Do castling
        if mv.is_castling() {
            // TODO: edit this to make compatible with chess 960
            let rook_square = match kind {
                MoveType::KingsideCastle => start.shift(3),
                MoveType::QueensideCastle => start.shift(-4),
                _ => Square::None,
            };
            // remove the relevant rook
            self.remove_piece(rook_square, self.side_to_move, Piece::Rook);

            // add rook to the square between king's start and target squares.
            let between_square = self.standard_bitboards.between[start][target].lsb();
            self.add_piece(between_square, self.side_to_move, Piece::Rook);
        }

        //
        // Update the non-piece-related board state
        //

        let full_move = FullMove::new(piece, captured_piece, mv);

        self.state.captured_piece = captured_piece;

        // set halfmove clock
        if piece == Piece::Pawn || mv.is_capture() {
            self.state.halfmove_clock = 0;
        } else {
            self.state.halfmove_clock += 1;
        }

        // set en passant square
        if kind == MoveType::DoublePawnPush {
            self.state.en_passant = self.standard_bitboards.between[start][target].lsb();
        } else {
            self.state.en_passant = Square::None;
        }

        // Set castling rights
        self.state.castling.update(start, target);

        // change side_to_move
        self.side_to_move = !self.side_to_move;

        // Update the move stack
        self.move_stack.push(full_move);

        // TODO: update the zobrist hashes

        self.analyze_board()
    }

    pub fn undo_move(&mut self) {
        let full_move = self.move_stack.pop();
        match full_move {
            Some(fm) => {
                let start = fm.inner_move.start();
                let target = fm.inner_move.target();
                // the piece that was moved
                let moved_piece = fm.piece;
                // the piece AFTER it was moved
                let placed_piece = if fm.inner_move.is_promotion() {
                    fm.inner_move.promo_piece()
                } else {
                    moved_piece
                };
                self.side_to_move = !self.side_to_move;
                // remove the piece that was placed
                self.remove_piece(target, self.side_to_move, placed_piece);
                // replace the piece that was captured
                if fm.inner_move.is_capture() {
                    let captured_piece = fm.captured;
                    let captured_piece_square = match fm.inner_move.is_en_passant() {
                        true => {
                            if self.side_to_move == Color::White {
                                target.shift(-8)
                            } else {
                                target.shift(8)
                            }
                        },
                        false => target,
                    };
                    self.add_piece(captured_piece_square, !self.side_to_move, captured_piece);
                }
                if fm.inner_move.is_castling() {
                    let (rook_start, rook_end) = get_rook_move_for_king_target(target);
                    self.remove_piece(rook_end, self.side_to_move, Piece::Rook);
                    self.add_piece(rook_start, self.side_to_move, Piece::Rook);
                }
                // re-add the piece that was moved
                self.add_piece(start, self.side_to_move, moved_piece);
                self.state = self.state_stack.pop().unwrap();
                self.analyze_board();
            },
            None => {
            },
        }
    }
}


/// Returns (rook_start, rook_end) square for a given castle move
const fn get_rook_move_for_king_target(king_target: Square) -> (Square, Square) {
    match king_target {
        Square::G1 => (Square::H1, Square::F1),
        Square::C1 => (Square::A1, Square::D1),
        Square::G8 => (Square::H8, Square::F8),
        Square::C8 => (Square::A8, Square::D8),
        _ => panic!("Not a valid king target for castling.")
    }
}
