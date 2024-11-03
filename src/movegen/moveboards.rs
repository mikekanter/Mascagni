use crate::types::{Bitboard, Color, File, Square};

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

/// Generic function for generating ray bitboards in any direction from any square. This currently
/// actually over-extends. It ***always*** includes the first occupied square in the ray. To deal
/// with this, we should make sure we do an XOr operation with the teammate occupancy bitboard to
/// find the actual pseudo-legal moves.
pub fn ray_bitboard<T: Fn(Square) -> bool>(square: &Square, occupancy: &Bitboard, offset: i8, predicate: T) -> Bitboard {
    let oc = *occupancy;
    let empty = Bitboard::default();
    let mut dir_bitboard = Bitboard::default();
    let mut current_square = square.shift(offset);
    while predicate(current_square) {
        let cur_bb = Bitboard::from(current_square);
        dir_bitboard |= cur_bb;
        if cur_bb & oc > empty {
            break
        }
        current_square = current_square.shift(8);
    }
    dir_bitboard
}

/// Returns the rook moves, given a square and occupancy
pub fn generate_rook_moves(square: &Square, occupancy: &Bitboard) -> Bitboard {
    let north_bitboard = ray_bitboard(square, occupancy, 8, |sq| sq.index() < 64);
    let south_bitboard = ray_bitboard(square, occupancy, -8, |sq| sq.index() < 64);

    let east_bitboard = ray_bitboard(square, occupancy, 1, |sq| sq.index() < 64 && sq.file() != File::A);
    let west_bitboard = ray_bitboard(square, occupancy, -1, |sq| sq.index() < 64 && sq.file() != File::H);

    north_bitboard
        | south_bitboard
        | east_bitboard
        | west_bitboard
}

/// Returns the bishop moves, given a square and occupancy
pub fn generate_bishop_moves(square: &Square, occupancy: &Bitboard) -> Bitboard {
    let ne_bitboard = ray_bitboard(
        square,
        occupancy,
        9,
        |sq| sq.index() < 64 && sq.file() != File::A,
    );
    let sw_bitboard = ray_bitboard(
        square,
        occupancy,
        -9,
        |sq| sq.index() < 64 && sq.file() != File::H,
    );
    let nw_bitboard = ray_bitboard(
        square,
        occupancy,
        7,
        |sq| sq.index() < 64 && sq.file() != File::H,
    );
    let se_bitboard = ray_bitboard(
        square,
        occupancy,
        -7,
        |sq| sq.index() < 64 && sq.file() != File::A,
    );

    ne_bitboard
        | sw_bitboard
        | nw_bitboard
        | se_bitboard
}

/// Queen moves are simply rook moves and bishop moves
pub fn generate_queen_moves(square: &Square, occupancy: &Bitboard) -> Bitboard {
    generate_rook_moves(square, occupancy)
        | generate_bishop_moves(square, occupancy)
}

fn white_pawn_captures(square: Square) -> Bitboard {
    let bb: u64 = 1 << square as u8;
    let answer: u64 = 0
        | (bb & !A_FILE) << 7
        | (bb & !H_FILE) << 9;
    Bitboard(answer)
}

fn black_pawn_captures(square: Square) -> Bitboard {
    let bb: u64 = 1 << square as u8;
    let answer: u64 = 0
        | (bb & !A_FILE) >> 9
        | (bb & !H_FILE) >> 7;
    Bitboard(answer)
}

pub fn pawn_captures(square: Square, color: Color) -> Bitboard {
    match color {
        Color::White => white_pawn_captures(square),
        Color::Black => black_pawn_captures(square),
    }
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
