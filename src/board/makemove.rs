use crate::types::{Color, FullMove, Move, MoveType, Piece, Square};

use super::Board;

impl Board {
    pub fn make_move(&mut self, mv: Move) {
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

        let full_move = FullMove::new(piece, mv);

        let kind = mv.kind();

        //
        // Update pieces on the board
        //

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

            let captured_piece = self.piece_on(capture_square);

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

        self.analyze_board()
    }
}
