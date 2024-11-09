use crate::types::{distance, Bitboard, Color, File, Piece, Square};

const A_FILE: u64 = Bitboard::file(File::A).0;
const B_FILE: u64 = A_FILE << 1;
const G_FILE: u64 = A_FILE << 6;
const H_FILE: u64 = A_FILE << 7;
const AB_FILE: u64 = A_FILE | B_FILE;
const GH_FILE: u64 = G_FILE | H_FILE;

/// These are special bitboards that we generate once and then use forever
#[derive(Copy, Clone)]
pub struct StandardBitboards {
    pub knight_attacks: [Bitboard; Square::NUM],
    pub king_attacks: [Bitboard; Square::NUM],
    /// given 2 squares, get the squares between them (useful for blocking sliding pieces)
    pub between: [[Bitboard; Square::NUM]; Square::NUM],
    // TODO: Maybe get line bbs, attacks on empty board, etc.
}

/// Generate the map of betweeners
pub fn generate_betweeners() -> [[Bitboard; Square::NUM]; Square::NUM] {
    let empty = Bitboard(0);
    let mut between_bbs: [[Bitboard; Square::NUM]; Square::NUM]
        = [[Bitboard(0); Square::NUM]; Square::NUM];
    for s1_index in 0..63 {
        let s1 = Square::new(s1_index);
        for s2_index in 0..63 {
            let s2 = Square::new(s2_index);
            // occupancy boards
            let o1 = Bitboard::from(s1);
            let o2 = Bitboard::from(s2);
            let pts = [Piece::Bishop, Piece::Rook];
            for i in pts {
                // attack_boards
                if i == Piece::Bishop {
                    let a1 = generate_bishop_attacks(&s1, &o2);
                    if a1 & o2 > empty {
                        let a2 = generate_bishop_attacks(&s2, &o1);
                        between_bbs[s1][s2] = a1 & a2;
                        break
                    }
                } else {
                    // rook
                    let a1 = generate_rook_attacks(&s1, &o2);
                    if a1 & o2 > empty {
                        let a2 = generate_rook_attacks(&s2, &o1);
                        between_bbs[s1][s2] = a1 & a2;
                        break
                    }
                }
            }
        }
    }
    between_bbs
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
pub fn sliding_attack(square: &Square, occupancy: &Bitboard, offset: i8) -> Bitboard {
    let oc = *occupancy;
    let empty = Bitboard::default();
    let mut dir_bitboard = Bitboard::default();
    let mut current_square = square.clone();
    while safe_destination(&current_square, offset) {
        current_square = current_square.shift(8);
        let cur_bb = Bitboard::from(current_square);
        dir_bitboard |= cur_bb;
        if cur_bb & oc > empty {
            break
        }
    }
    dir_bitboard
}

pub fn safe_destination(square: &Square, step: i8) -> bool {
    let to: Square = square.shift(step);
    let is_valid_square: bool = to >= Square::A1 && to <= Square::H8;
    is_valid_square && distance(square, &to) <= 2
}

/// Returns the rook moves, given a square and occupancy
pub fn generate_rook_attacks(square: &Square, occupancy: &Bitboard) -> Bitboard {
    let north_bitboard = sliding_attack(square, occupancy, 8);
    let south_bitboard = sliding_attack(square, occupancy, -8);

    let east_bitboard = sliding_attack(square, occupancy, 1);
    let west_bitboard = sliding_attack(square, occupancy, -1);

    north_bitboard
        | south_bitboard
        | east_bitboard
        | west_bitboard
}

/// Returns the bishop moves, given a square and occupancy
pub fn generate_bishop_attacks(square: &Square, occupancy: &Bitboard) -> Bitboard {
    let ne_bitboard = sliding_attack(
        square,
        occupancy,
        9,
    );
    let sw_bitboard = sliding_attack(
        square,
        occupancy,
        -9,
    );
    let nw_bitboard = sliding_attack(
        square,
        occupancy,
        7,
    );
    let se_bitboard = sliding_attack(
        square,
        occupancy,
        -7,
    );

    ne_bitboard
        | sw_bitboard
        | nw_bitboard
        | se_bitboard
}

/// Queen moves are simply rook moves and bishop moves
pub fn generate_queen_attacks(square: &Square, occupancy: &Bitboard) -> Bitboard {
    generate_rook_attacks(square, occupancy)
        | generate_bishop_attacks(square, occupancy)
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

impl StandardBitboards {
    pub fn new() -> Self {
        Self {
            knight_attacks: create_map(|square| generate_knight_moves(square)),
            king_attacks: create_map(|square| generate_king_moves(square)),
            between: generate_betweeners(),
        }
    }
}

impl Default for StandardBitboards {
    fn default() -> Self {
        Self {
            knight_attacks: create_map(|square| generate_knight_moves(square)),
            king_attacks: create_map(|square| generate_king_moves(square)),
            between: generate_betweeners(),
        }
    }
}
