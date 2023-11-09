#![allow(dead_code)]
use crate::board;

#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    pub piece:         usize,
    pub from:          board::BitBoard,
    pub to:            board::BitBoard,
    pub is_castle:     bool,
    pub is_en_passant: bool,
    pub promotes_to:   usize,
}

impl Move {
    pub fn new(piece: usize, from: board::BitBoard, to: board::BitBoard, is_castle: bool, is_en_passant: bool, promotes_to: usize) -> Self { 
        Self { piece, from, to, is_castle, is_en_passant, promotes_to } 
    }

    pub fn new_empty() -> Self {
        let mut empty: Self = unsafe { std::mem::zeroed() };
        empty.promotes_to = usize::MAX;
        empty
    }
}

trait ApplyMove {
    fn apply_move(&mut self, move_s: Move);
}