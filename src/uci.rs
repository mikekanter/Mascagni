use crate::{board::Board, types::{FullMove, Move, MoveType, Piece}};

pub struct AlgebraicMove {
    pub full_move: FullMove,
    /// Algebraic notation
    pub algebraic: String,
}

impl AlgebraicMove {
    pub const fn new(full_move: &FullMove, algebraic: String) -> Self {
        Self {
            full_move: *full_move,
            algebraic,
        }
    }
}

pub fn move_to_full(mv: Move, board: &Board) -> FullMove {
    let piece = board.piece_on(mv.start());
    let captured = match mv.is_en_passant() {
        true => Piece::Pawn,
        false => board.piece_on(mv.target()),
    };
    FullMove::new(piece, captured, mv)
}

/// Take a board and return a list of full, algebraic moves.
pub fn collect_algebraic_moves(board: &Board) -> Vec<AlgebraicMove> {
    let mut full_moves_vec: Vec<FullMove> = vec![];
    let total_moves = board.legal_moves.len;
    for i in 0..total_moves {
        let lm = board.legal_moves.moves[i];
        full_moves_vec.push(move_to_full(lm, board));
    }

    let mut alg_moves_vec: Vec<AlgebraicMove> = vec![];

    for i in 0..total_moves {
        let fm = full_moves_vec[i];

        let kind = fm.inner_move.kind();
        // let's handle castles first
        if kind == MoveType::KingsideCastle {
            alg_moves_vec.push(AlgebraicMove::new(&fm, String::from("O-O")));
            continue
        } else if kind == MoveType::QueensideCastle {
            alg_moves_vec.push(AlgebraicMove::new(&fm, String::from("O-O-O")));
            continue
        } else {
            let start = fm.inner_move.start();
            let target = fm.inner_move.target();
            let piece = board.piece_on(start);
            let is_promotion = fm.inner_move.is_promotion();
            let is_capture = fm.inner_move.is_capture();

            let target_segment = target.to_string();

            // This covers all non-promoting pawn advances
            if piece == Piece::Pawn {
                let (start_segment, capture_segment) =
                    if is_capture {
                        (start.file().to_string(), String::from("x"))
                    } else {
                        (String::from(""), String::from(""))
                    };
                let (promo_segment, promo_piece_segment) =
                    if is_promotion {
                        (String::from("="), match kind {
                            MoveType::PromotionToRook => String::from("R"),
                            MoveType::PromotionToQueen => String::from("Q"),
                            MoveType::PromotionToBishop => String::from("B"),
                            MoveType::PromotionToKnight => String::from("N"),
                            MoveType::PromotionCaptureToRook => String::from("R"),
                            MoveType::PromotionCaptureToQueen => String::from("Q"),
                            MoveType::PromotionCaptureToBishop => String::from("B"),
                            MoveType::PromotionCaptureToKnight => String::from("N"),
                            _ => String::from(""),
                        })
                    } else {
                        (String::from(""), String::from(""))
                    };

                let alg_notation = format!(
                    "{}{}{}{}{}",
                    start_segment,
                    capture_segment,
                    target_segment,
                    promo_segment,
                    promo_piece_segment,
                );
                alg_moves_vec.push(AlgebraicMove::new(&fm, alg_notation));
                continue
            } else {
                // All non-pawn, non-castling moves moves
                // 4 parts
                //  - piece segment.
                //  - location verification segment
                //  - capture segment.
                //  - target segment.
                let piece_segment = match piece {
                    Piece::Rook => String::from("R"),
                    Piece::Bishop => String::from("B"),
                    Piece::King => String::from("K"),
                    Piece::Knight => String::from("N"),
                    Piece::Queen => String::from("Q"),
                    _ => String::from(""),
                };

                let same_piece_and_target_moves_vec = full_moves_vec
                    .iter()
                    .filter(|m| m.piece == piece)
                    .filter(|m| m.inner_move.target() == target)
                    .map(|m| m.clone())
                    .collect::<Vec<FullMove>>();

                // TODO: Capture location verification
                let location_segment: String = match same_piece_and_target_moves_vec.len() {
                    1 => String::from(""),
                    2 => {
                        let v0 = same_piece_and_target_moves_vec[0];
                        let v1 = same_piece_and_target_moves_vec[1];
                        if v0.inner_move.start().file() == v1.inner_move.start().file() {
                            start.rank().to_string()
                        } else {
                            start.file().to_string()
                        }
                    },
                    _ => start.to_string(),
                };

                let capture_segment = if is_capture { String::from("x") } else { String::from("") };
                let target_segment = target.to_string();
                let alg_notation = format!(
                    "{}{}{}{}",
                    piece_segment,
                    location_segment,
                    capture_segment,
                    target_segment,
                );
                alg_moves_vec.push(AlgebraicMove::new(&fm, alg_notation));
                continue
            }
        }
    }
    alg_moves_vec
}
