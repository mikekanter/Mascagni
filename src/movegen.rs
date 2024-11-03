// use crate::types::{Bitboard, Color, Square};
//
//
// pub struct MoveGenerator {
//     /// This thing fucks so hard
//     king: [Bitboard; Square::NUM],
//     knight: [Bitboard; Square::NUM],
//     pawns: [[Bitboard; Square::Num]; Color::NUM],
//     rook: Vec<Bitboard>,
//     bishop: Vec<Bitboard>,
// }
//
// const FIRST_RANK: Bitboard = Bitboard{ 0: u64::pow(0b10, 0b1000) - 0b1 as u64 };
//
// impl MoveGenerator {
//     pub fn new() -> Self {
//         Self {
//             king: [Bitboard::default(); Square::NUM],
//             knight: [Bitboard::default(); Square::NUM],
//             pawns: [[Bitboard::default(); Square::Num]; Color::NUM],
//             rook: vec![Bitboard::default()],
//             bishop: Vec![Bitboard::default()],
//         }
//     }
//
//     /// Oh fuck yeah brother
//     pub fn get_that_nut(self) {
//         self.king
//     }
//
//     pub fn generate_knight_moves(&mut self, square: Square) {
//         let mut bitboard = Bitboard::default();
//
//         let index = square.index();
//         let target = index + 10;
//     }
//
// }
//
// impl Default for MoveGenerator {
//     fn default() -> Self {
//         todo!()
//     }
// }
mod moveboards;
