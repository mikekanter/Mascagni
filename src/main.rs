use types::Square;

mod types;
mod board;
mod movegen;

fn main() {
    let j = Square::A1;
    let k = j.shift(-1);

    println!("Hello, world! A1 is index: {}", j.index());
    println!("Hello, world! A1 - 1 is index: {}", k.index());
}
