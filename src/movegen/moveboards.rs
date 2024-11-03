use crate::types::{Bitboard, File, Square};

const A_FILE: u64 = Bitboard::file(File::A).0;
const B_FILE: u64 = A_FILE << 1;
const G_FILE: u64 = A_FILE << 6;
const H_FILE: u64 = A_FILE << 7;
const AB_FILE: u64 = A_FILE | B_FILE;
const GH_FILE: u64 = G_FILE | H_FILE;

struct Moveboards {
    knight: [Bitboard; Square::NUM],
    king: [Bitboard; Square::NUM],
    // TODO: get Bishops, Rooks, and Queens (which is just bishop | rook for a given square)
}

/// Generates the knight attacks from a given square
pub fn generate_knight_moves(square: Square) -> Bitboard {
    let bb = 1 << square as u64;

    let moves = (bb & !A_FILE) >> 17
        | (bb & !H_FILE) << 17
        | (bb & !A_FILE) << 15
        | (bb & !H_FILE) >> 15
        | (bb & !AB_FILE) >> 10
        | (bb & !GH_FILE) << 10
        | (bb & !AB_FILE) << 6
        | (bb & !GH_FILE) >> 6;
    Bitboard(moves)
}

/// Generates the king attacks from a given square
pub fn generate_king_moves(square: Square) -> Bitboard {
    let bb: u64 = 1 << square as u8;

    let answer = (bb >> 8 | bb << 8)
        | (bb & !A_FILE) >> 9
        | (bb & !A_FILE) >> 1
        | (bb & !A_FILE) << 7
        | (bb & !H_FILE) >> 7
        | (bb & !H_FILE) << 1
        | (bb & !H_FILE) << 9;
    Bitboard(answer)
}

/// Returns the rook moves (on an empty board)
/// TODO: refine this
pub fn generate_rook_moves(square: Square) -> Bitboard {
}

/// Takes an attack generator function and returns a map from squares to attack bitboards
pub fn create_map<T: Fn(Square) -> Bitboard>(generator: T) -> [Bitboard; Square::NUM] {
    let mut map: [Bitboard; Square::NUM] = [Bitboard(0); Square::NUM];
    for i in 0..Square::NUM {
        map[i as usize] = generator(Square::new(i as u8));
    }
    map
}

impl Moveboards {
}

impl Default for Moveboards {
    fn default() -> Self {
        Self {
            knight: create_map(|square| generate_knight_moves(square)),
            king: create_map(|square| generate_king_moves(square)),
        }
    }
}
