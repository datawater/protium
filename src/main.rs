#![allow(clippy::module_inception)]
#![feature(int_roundings)]

mod board;
mod utils;
mod moves;
mod version;

use std::io::{self, BufRead, Write};
use board::Board;

fn main() {
    println!("protium {}.{}.{} Copyright (C) 2023 datawater", version::MAJOR, version::MINOR, version::PATCH);
    println!("The project is currently under development. It only list the legal moves in a given position. (Yet)");

    println!("Input a fen string: "); io::stdout().flush().unwrap();
    // TODO: Fen string validaiton (https://chess.stackexchange.com/questions/1482/how-do-you-know-when-a-fen-position-is-legal)
    let fen = io::stdin().lock().lines().next().unwrap().unwrap();

    let board = Board::from(&fen as &str);
    let moves = board.generate_moves(false);

    for m in moves {
        let from = m.from.0.trailing_zeros();
        let to   = m.to.0  .trailing_zeros();

        let from = [((from % 8)        + ('a' as u32)) as u8 as char,
                     (from.div_floor(8) + ('1' as u32)) as u8 as char].iter().collect::<String>();

        let to = [((to % 8)        + ('a' as u32)) as u8 as char,
                   (to.div_floor(8) + ('1' as u32)) as u8 as char].iter().collect::<String>();

        println!("{}{}", from, to);
    }
}
