use std::process::exit;

use board::Board;
use uci::collect_algebraic_moves;

use std::io::{stdin, stdout, Write};

mod types;
mod board;
mod uci;

fn main() {
    // let mut s = String::new();
    // print!("Paste a fen to start your game: ");
    // let _ = stdout().flush();
    // stdin().read_line(&mut s).expect("Did not enter a valid string.");
    // if s == String::from("") {
    //     s = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    // }
    // if let Some('\n')=s.chars().next_back() {
    //     s.pop();
    // }
    // if let Some('\r')=s.chars().next_back() {
    //     s.pop();
    // }

    let mut stdout = stdout();

    let board = Board::new(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
    match board {
        Ok(mut b) => {
            while b.legal_moves.len > 0 {
                let alg_moves = collect_algebraic_moves(&b);
                b.pretty_print();
                stdout.write_all(format!("\n{} to move: ", b.side_to_move.to_string()).as_bytes()).unwrap();
                let mut new_st = String::new();
                stdout.flush().unwrap();
                stdin().read_line(&mut new_st).expect("Did not enter a valid string.");
                let mut made_move = false;
                for m in alg_moves.iter() {
                    if m.algebraic.trim() == new_st.trim() {
                        b.make_move(m.full_move.inner_move);
                        made_move = true;
                        break
                    }
                }
                if !made_move {
                    println!("Not a valid move!!!!");
                }

            }
            if b.is_checkmate() {
                println!("{} wins by checkmate", (!b.side_to_move).to_string());
            }
            if b.is_stalemate() {
                println!("stalemate!!");
            }
        },
        Err(e) => {
            println!("Error!!! {}", e);
            exit(1)
        },
    };
}
