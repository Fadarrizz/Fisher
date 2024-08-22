use std::io::{stdin, stdout, Write};

use chess::Board;

fn input(prompt: impl std::fmt::Display) -> String {
    let mut s = String::new();

    print!("{}", prompt);

    let _ = stdout().flush();
    
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct move");

    return s;
}

fn main() {
    let board = Board::default();

    println!("{}", board);

    let s = input(">>> ");
    let s = s.trim().to_string();

    dbg!(s);
}
