use std::{fmt::Display, ops::{Add, AddAssign}};

use crate::board::Board;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct PerftResult {
    nodes: u64,
    captures: u64,
    en_passants: u64,
    castles: u64,
    promotions: u64,
    checks: u64,
    checkmates: u64,
}

impl Add for PerftResult {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            nodes: self.nodes + rhs.nodes,
            captures: self.captures + rhs.captures,
            en_passants: self.en_passants + rhs.en_passants,
            castles: self.castles + rhs.castles,
            promotions: self.promotions + rhs.promotions,
            checks: self.checks + rhs.checks,
            checkmates: self.checkmates + rhs.checkmates,
        }
    }
}

impl AddAssign for PerftResult {
    fn add_assign(&mut self, rhs: Self) {
        self.nodes += rhs.nodes;
        self.captures += rhs.captures;
        self.en_passants += rhs.en_passants;
        self.castles += rhs.castles;
        self.promotions += rhs.promotions;
        self.checks += rhs.checks;
        self.checkmates += rhs.checkmates;
    }
}

impl Display for PerftResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {}, {}, {}, {})", self.nodes, self.captures, self.en_passants, self.castles, self.promotions, self.checks, self.checkmates)
    }
}

pub fn perft(board: &mut Board, depth: usize) -> PerftResult {
    let mut nodes = PerftResult{
        nodes: 0,
        captures: 0,
        en_passants: 0,
        castles: 0,
        promotions: 0,
        checks: 0,
        checkmates: 0,
    };
    if depth == 0 {
        let len = board.move_stack.len();
        if len == 0 {
            return PerftResult {
                nodes: 1,
                captures: 0,
                en_passants: 0,
                castles: 0,
                promotions: 0,
                checks: 0,
                checkmates: 0,
            }
        } else {
            let mv = board.move_stack[len - 1].inner_move;
            return PerftResult {
                nodes: 1,
                captures: mv.is_capture() as u64,
                en_passants: mv.is_en_passant() as u64,
                castles: mv.is_castling() as u64,
                promotions: mv.is_promotion() as u64,
                checks: board.is_check() as u64,
                checkmates: board.is_checkmate() as u64,
            }
        }
    }
    for i in 0..board.legal_moves.len {
        let mv = board.legal_moves.moves[i];
        board.make_move(mv);
        nodes += perft(board, depth - 1);
        // the move has been made
        board.undo_move();
        // the move has been unmade
    }
    return nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        let mut board = Board::new( "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string()).unwrap();
        let result = perft(&mut board, 1);
        // assert_eq!(result.nodes, 44 as u64);
        assert_eq!(result, PerftResult {
            nodes: 48,
            captures: 8,
            en_passants: 0,
            castles: 2,
            promotions: 0,
            checks: 0,
            checkmates: 0,
        });
    }

    #[test]
    fn two() {
        let mut board = Board::new( "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string()).unwrap();
        let result = perft(&mut board, 2);
        // assert_eq!(result.nodes, 1486 as u64);
        assert_eq!(result, PerftResult {
            nodes: 2039,
            captures: 351,
            en_passants: 1,
            castles: 91,
            promotions: 0,
            checks: 3,
            checkmates: 0,
        });
    }

    #[test]
    fn three() {
        let mut board = Board::new( "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string()).unwrap();
        let result = perft(&mut board, 3);
        // assert_eq!(result.nodes, 63279 as u64);
        assert_eq!(result, PerftResult {
            nodes: 97862,
            captures: 17102,
            en_passants: 45,
            castles: 3162,
            promotions: 0,
            checks: 993,
            checkmates: 1,
        });
    }

    // SHOULDN"T TAKE THIS LONG

    // #[test]
    // fn four() {
    //     let mut board = Board::new( "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".to_string()).unwrap();
    //     let result = perft(&mut board, 4);
    //     //assert_eq!(result.nodes, 2103487 as u64);
    //     assert_eq!(result, PerftResult {
    //         nodes: 4085603,
    //         captures: 757163,
    //         en_passants: 1929,
    //         castles: 128013,
    //         promotions: 15172,
    //         checks: 25523,
    //         checkmates: 43,
    //     });
    // }

    // #[test]
    // fn five() {
    //     let mut board = Board::new( "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()).unwrap();
    //     let result = perft(&mut board, 5);
    //     assert_eq!(result, PerftResult {
    //         nodes: 4865609,
    //         captures: 82719,
    //         en_passants: 258,
    //         castles: 0,
    //         promotions: 0,
    //         checks: 27351,
    //         checkmates: 347,
    //     });
    // }
}
