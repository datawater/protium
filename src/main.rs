#![allow(clippy::module_inception)]

mod board;
mod utils;
mod moves;

fn main() {
    println!("Hello, world!");

    let board = board::Board::from("8/8/8/8/8/3p4/2pRP3/3P4 w - - 0 1");
    println!("{:#?}", board);
}