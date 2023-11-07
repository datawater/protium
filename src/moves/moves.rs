#![allow(dead_code)]
use crate::board;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Move {
    pub piece:         usize,
    pub from:          board::Board,
    pub to:            board::Board,
    pub is_castle:     bool,
    pub is_en_passant: bool,
    pub promotes_to:   usize,
}

impl Move {
    pub fn new(piece: usize, from: board::Board, to: board::Board, is_castle: bool, is_en_passant: bool, promotes_to: usize) -> Self { 
        Self { piece, from, to, is_castle, is_en_passant, promotes_to } 
    }

    pub fn new_empty() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

trait ApplyMove {
    fn apply_move(&mut self, move_s: Move);
}