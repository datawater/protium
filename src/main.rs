#![allow(clippy::module_inception)]

mod board;
mod utils;
mod moves;

fn main() {
    println!("Hello, world!");

    let board = board::Board::from(board::fens::STARTING_FEN);
    println!("{:#?}", board);
}