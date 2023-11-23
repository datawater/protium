#![allow(dead_code, unused_variables)]

use crate::board::{Board, BitBoard, self, pieces::*, BoardSide};
use super::constants::{*, magics::*};

pub(super) type AttackGeneratorFunction = fn(&Board, u8) -> BitBoard;

pub(super) const PIECE_TO_ATTACK_FUNCTION: [AttackGeneratorFunction; 10] = [
    Board::generate_attacks_king,
    Board::generate_attacks_king,
    Board::generate_attacks_queen,
    Board::generate_attacks_queen,
    Board::generate_attacks_rook,
    Board::generate_attacks_rook,
    Board::generate_attacks_bishop,
    Board::generate_attacks_bishop,
    Board::generate_attacks_knight,
    Board::generate_attacks_knight,
];

impl Board {  
    // NON-SLIDING
    // Note: You can abstract with a general function which's signature would look
    //       something like this: fn(&self, pointer_to_lookup_table: &[BitBoard], square: u8)
    //       But that much abstraction is not necessary and does more harm than good.
    #[inline(always)]
    pub(super) fn generate_attacks_pawn(&self, square: u8, side: board::BoardSide) -> BitBoard {
        if side == board::BoardSide::White {
            BitBoard(PAWN_WHITE_ATTACKS[square as usize])
        } else {
            BitBoard(PAWN_BLACK_ATTACKS[square as usize])
        }
    }

    #[inline(always)]
    pub(super) fn generate_attacks_knight(&self, square: u8) -> BitBoard {
        BitBoard(KNIGHT_ATTACKS[square as usize])
    }

    #[inline(always)]
    pub(super) fn generate_attacks_king(&self, square: u8) -> BitBoard {
        BitBoard(KING_ATTACKS[square as usize])
    }

    // SLIDING

    #[inline(always)]
    fn magic_index(&self, entry: &MagicEntry) -> usize {
        let blockers = self.pieces[ALL_PIECES].0 & entry.mask;
        let hash = blockers.wrapping_mul(entry.magic);
        let index = (hash >> entry.shift) as usize;

        entry.offset as usize + index
    } 

    #[inline(always)]
    pub(super) fn generate_attacks_bishop(&self, square: u8) -> BitBoard { 
        BitBoard(BISHOP_MOVES[self.magic_index(&BISHOP_MAGICS[square as usize])])
    }

    #[inline(always)]
    pub(super) fn generate_attacks_rook(&self, square: u8) -> BitBoard {
        BitBoard(ROOK_MOVES[self.magic_index(&ROOK_MAGICS[square as usize])])
    }

    #[inline(always)]
    pub(super) fn generate_attacks_queen(&self, square: u8) -> BitBoard {
        self.generate_attacks_bishop(square) | self.generate_attacks_rook(square)
    }

    // OVERALL
    #[inline(always)]
    pub fn generate_attacks_piece_on_square(&mut self, piece: usize, square: u8) -> BitBoard {
        // This kinda bothers me but it's one if statement, it shouldn't be that bad
        assert!(piece <= BLACK_PAWN);

        let attacks;

        // If piece is more than 9, so piece is pawn
        if piece > 9 {
            attacks = Board::generate_attacks_pawn(self, square, 
                unsafe { std::mem::transmute((piece % 2) as u8) }
            )
        } else {
            // I use a table of function pointers so the lookup is fast and easy
            attacks = PIECE_TO_ATTACK_FUNCTION[piece](self, square)
        }

        if self.side == BoardSide::White && (self.pieces[WHITE_KING].0 & attacks.0 != 0) && piece % 2 == 1 {
            if self.in_check {self.in_double_check = true}
            self.in_check = true;

            self.check_ray_mask = attacks;
            self.piece_that_checks_loc = 1 << square;
            self.piece_that_checks = piece;
        } else if self.side == BoardSide::Black && (self.pieces[BLACK_KING].0 & attacks.0 != 0) && piece % 2 == 0 {
            if self.in_check {self.in_double_check = true}
            self.in_check = true;

            self.check_ray_mask = attacks;
            self.piece_that_checks_loc = 1 << square;
            self.piece_that_checks = piece;
        }

        attacks
    }

    #[inline(always)]
    pub fn generate_attacks_piece(&mut self, piece: usize) -> BitBoard {
        let mut n = self.pieces[piece].0;
        let mut attacks = BitBoard(0);
    
        while n != 0 {
            let removed_square                = n & (n - 1);
            // Get's the position of the LSB.
            let square_of_rightest_most_piece = (n - removed_square).trailing_zeros() as u8;
            n = removed_square;
            
            attacks |= self.generate_attacks_piece_on_square(piece, square_of_rightest_most_piece);
        }
        
        attacks
    }

    pub fn generate_attacks(&mut self) {
        for (i, n) in self.pieces.clone().iter().enumerate() {
            if i > board::pieces::BLACK_PAWN {break;}

            let attacks = self.generate_attacks_piece(i);

            if i % 2 == 0 {
                self.attacks[board::attacks::WHITE_ATTACKS] |= attacks;
            } else {
                self.attacks[board::attacks::BLACK_ATTACKS] |= attacks;
            }
        }
    }
}