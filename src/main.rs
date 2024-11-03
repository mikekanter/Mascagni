use types::Square;

mod types;
mod board;
mod movegen;

fn main() {
    let j = Square::E1;

    println!("Hello, world! E1 is index: {}", j.index());
}
