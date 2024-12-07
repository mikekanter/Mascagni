use std::process::exit;

use types::Square;
use board::Board;

mod types;
mod board;
mod movegen;

fn main() {
    let j = Square::A1;
    let k = j.shift(-1);

    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    match board {
        Ok(b) => b.pretty_print(),
        Err(_) => {
            println!("Error!!!");
            exit(1)
        },
    };

    println!("Hello, world! A1 is index: {}", j.index());
    println!("Hello, world! A1 - 1 is index: {}", k.index());
}
