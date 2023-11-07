#![allow(dead_code, unused_variables)]

use crate::board::{Board, BitBoard, self};

type AttackGeneratorFunction = fn(Board, u8) -> BitBoard;

// NON SLIDING

fn generate_attacks_pawn(board: Board, square: u8) -> BitBoard {
    todo!();
}

fn generate_attacks_knight(board: Board, square: u8) -> BitBoard {
    todo!();
}

fn generate_attacks_king(board: Board, square: u8) -> BitBoard {
    todo!();
}

// SLIDING

fn generate_attacks_bishop(board: Board, square: u8) -> BitBoard {
    todo!();
}

fn generate_attacks_rook(board: Board, square: u8) -> BitBoard {
    todo!();
}

fn generate_attacks_queen(board: Board, square: u8) -> BitBoard {
    todo!();
}

// OVERALL

const PIECE_TO_ATTACK_FUNCTION: [AttackGeneratorFunction; 12] = [
    generate_attacks_king,
    generate_attacks_king,
    generate_attacks_queen,
    generate_attacks_queen,
    generate_attacks_rook,
    generate_attacks_rook,
    generate_attacks_bishop,
    generate_attacks_bishop,
    generate_attacks_knight,
    generate_attacks_knight,
    generate_attacks_pawn,
    generate_attacks_pawn,
];

impl Board {
    pub fn generate_attacks(&mut self) {
        for (i, n) in self.pieces.iter().enumerate() {
            if i > board::pieces::BLACK_PAWN {break;}

            let mut n = n.0;
            
            while n != 0 {
                let removed_square                = n & (n - 1);
                let square_of_rightest_most_piece = (n - removed_square).leading_ones();
                n = removed_square;

                let attacks = PIECE_TO_ATTACK_FUNCTION[i](*self, square_of_rightest_most_piece as u8);

                if self.side == board::BoardSide::White {
                    self.attacks[board::attacks::WHITE_ATTACKS] |= attacks;
                } else {
                    self.attacks[board::attacks::BLACK_ATTACKS] |= attacks;
                }
            }
        }
    }
}