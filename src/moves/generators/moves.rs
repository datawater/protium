#![allow(dead_code)]

use crate::board::attacks::{BLACK_ATTACKS, WHITE_ATTACKS};
use crate::board::castle_masks::*;
use crate::board::{Board, BitBoard, pieces::*, self, 
                   WHITE_CASTLE_KINGSIDE, WHITE_CASTLE_QUEENSIDE, 
                   BLACK_CASTLE_KINGSIDE, BLACK_CASTLE_QUEENSIDE};
use crate::moves::Move;
use super::constants::*;


type MoveGeneratorFunction = fn(&Board, u8, &mut Vec<Move>, usize);

pub(super) const PIECE_TO_MOVE_FUNCTION: [MoveGeneratorFunction; 12] = [
    Board::generate_moves_king,
    Board::generate_moves_king,
    Board::generate_moves_queen,
    Board::generate_moves_queen,
    Board::generate_moves_rook,
    Board::generate_moves_rook,
    Board::generate_moves_bishop,
    Board::generate_moves_bishop,
    Board::generate_moves_knight,
    Board::generate_moves_knight,
    Board::generate_moves_pawn,
    Board::generate_moves_pawn,
];

impl Board {
    #[inline(always)]
    fn generate_moves_general(&self, square: u8, vector: &mut Vec<Move>, piece: usize, mask: u64) {
        assert!(piece <= BLACK_PAWN);

        // TODO(#1): Handle checkmate
        let mut moves = self.generate_attacks_piece_on_square(piece, square).0
                           & !self.pieces[if piece % 2 == 0 {WHITE_PIECES} else {BLACK_PIECES}].0
                           & !self.pieces[WHITE_KING].0
                           & !self.pieces[BLACK_KING].0
                           & !mask;

        let from = 1 << square;

        while moves != 0 {
            let removed = moves & (moves - 1);

            vector.push(Move {
                piece,
                from: BitBoard(from),
                to: BitBoard(moves - removed),
                is_castle: false,
                is_en_passant: false,
                promotes_to: usize::MAX,
            });

            moves = removed;
        }
    }

    // NON-SLIDING
    fn generate_moves_pawn(&self, square: u8, vector: &mut Vec<Move>, piece: usize) {
        let pawn = 1u64 << square;
        let mut moves_bb;
        let mut en_passants = 0u64;

        if piece % 2 == 0 {
            let single_move = pawn << 8     & !self.pieces[ALL_PIECES].0;
            let double_move = ((single_move & RANK_MASK[2]) << 8) & !self.pieces[ALL_PIECES].0;

            moves_bb = single_move | double_move | (PAWN_WHITE_ATTACKS[square as usize] & self.pieces[BLACK_PIECES].0);

            if (pawn & RANK_MASK[4]) != 0 && ((self.en_passant_square.0 & PAWN_WHITE_ATTACKS[square as usize]) != 0) {
                en_passants |= self.en_passant_square.0;
            }
        } else {
            let single_move = pawn >> 8     & !self.pieces[ALL_PIECES].0;
            let double_move = ((single_move & RANK_MASK[5]) >> 8) & !self.pieces[ALL_PIECES].0;

            moves_bb = single_move | double_move | (PAWN_BLACK_ATTACKS[square as usize] & self.pieces[WHITE_PIECES].0);

            if (pawn & RANK_MASK[3]) != 0 && (self.en_passant_square.0 & PAWN_BLACK_ATTACKS[square as usize] != 0) {
                en_passants |= self.en_passant_square.0;
            }
        }

        while moves_bb != 0 {
            let removed = moves_bb & (moves_bb - 1);
            let move_bb = moves_bb - removed;

            if (move_bb & RANK_MASK[0]) != 0 || (move_bb & RANK_MASK[7]) != 0 {
                let start = if piece % 2 == 0 {2} else {3};
                let mut i = start;

                while i < WHITE_PAWN {
                    vector.push(Move {
                        piece,
                        from: BitBoard(pawn),
                        to: BitBoard(move_bb),
                        is_castle: false,
                        is_en_passant: false,
                        promotes_to: i
                    });

                    i += 2;
                }

                moves_bb = removed;
                continue;
            }

            vector.push(Move {
                piece,
                from: BitBoard(pawn),
                to: BitBoard(move_bb),
                is_castle: false,
                is_en_passant: false,
                promotes_to: usize::MAX
            });

            moves_bb = removed;
        }

        while en_passants != 0 {
            let removed = en_passants & (en_passants - 1);
            let move_bb = en_passants - removed;

            vector.push(Move {
                piece,
                from: BitBoard(pawn),
                to: BitBoard(move_bb),
                is_castle: false,
                is_en_passant: true,
                promotes_to: usize::MAX
            });

            en_passants = removed;
        }
    }

    fn generate_moves_knight(&self, square: u8, vector: &mut Vec<Move>, piece: usize) {
        self.generate_moves_general(square, vector, piece, 0)
    }

    fn generate_moves_king(&self, square: u8, vector: &mut Vec<Move>, piece: usize) {
        self.generate_moves_general(square, vector, piece, 
                            if piece % 2 == 0 {self.attacks[BLACK_ATTACKS].0} else {self.attacks[WHITE_ATTACKS].0});

        let ( to_check1, to_check2): (bool, bool);
        let (index1, index2): (usize, usize);
        let (mut to1, mut to2): (u64, u64) = (1, 1);
        let attacks: u64;

        if piece % 2 == 0 {
            to_check1 = self.castle_allowed[WHITE_CASTLE_KINGSIDE];
            to_check2 = self.castle_allowed[WHITE_CASTLE_QUEENSIDE];

            index1 = WHITE_CASTLE_KINGSIDE;
            index2 = WHITE_CASTLE_QUEENSIDE;

            to1 <<= 6;
            to2 <<= 2;

            attacks = self.attacks[BLACK_ATTACKS].0;
        } else {
            to_check1 = self.castle_allowed[BLACK_CASTLE_KINGSIDE];
            to_check2 = self.castle_allowed[BLACK_CASTLE_QUEENSIDE];

            index1 = BLACK_CASTLE_KINGSIDE;
            index2 = BLACK_CASTLE_QUEENSIDE;

            to1 <<= 6 + 8 * 7;
            to2 <<= 2 + 8 * 7;

            attacks = self.attacks[WHITE_ATTACKS].0;
        }

        if to_check1 && ((CASTLE_MASK_CHECK[index1]  & attacks) == 0)
                     && ((CASTLE_MASK_PIECES[index1] & self.pieces[ALL_PIECES].0) == 0) {

                        vector.push(Move {
                            piece,
                            from: BitBoard(1u64 << square),
                            to: BitBoard(to1),
                            is_castle: true,
                            is_en_passant: false,
                            promotes_to: usize::MAX
                        });
        }

        if to_check2 && ((CASTLE_MASK_CHECK[index2]  & attacks) == 0)
                     && ((CASTLE_MASK_PIECES[index2] & self.pieces[ALL_PIECES].0) == 0) {

                        vector.push(Move {
                            piece,
                            from: BitBoard(1u64 << square),
                            to: BitBoard(to2),
                            is_castle: true,
                            is_en_passant: false,
                            promotes_to: usize::MAX
                        });
        }

    }

    // SLIDING
    fn generate_moves_bishop(&self, square: u8, vector: &mut Vec<Move>, piece: usize) {
        self.generate_moves_general(square, vector, piece, 0)
    }

    fn generate_moves_rook(&self, square: u8, vector: &mut Vec<Move>, piece: usize) {
        self.generate_moves_general(square, vector, piece, 0)
    }

    fn generate_moves_queen(&self, square: u8, vector: &mut Vec<Move>, piece: usize) {
        self.generate_moves_general(square, vector, piece, 0)
    }

    #[inline(always)]
    pub fn generate_moves_piece_on_square(&self, piece_i: usize, square: u8, vector: &mut Vec<Move>) {
        PIECE_TO_MOVE_FUNCTION[piece_i](self, square, vector, piece_i);
    }

    #[inline(always)]
    pub fn generate_moves_piece(&self, piece_i: usize, vector: &mut Vec<Move>) {
        let mut piece = self.pieces[piece_i].0;
        
        while piece != 0 {
            let removed = piece & (piece - 1);
            let square = (piece - removed).trailing_zeros() as u8;

            self.generate_moves_piece_on_square(piece_i, square, vector);

            piece = removed;
        }
    }

    // OVERALL
    pub fn generate_moves(&self, generate_for_both_colours: bool) -> Vec<Move> {
        // Interesting source: https://chess.stcackexchange.com/questions/23135/what-is-the-average-number-of-legal-moves-per-turn
        // But this analyses legal moves, not pseudo-legal. Since we can only generate pseudo-legal moves, I'll double it to around 70
        let mut moves_vec: Vec<Move> = Vec::with_capacity(70);

        let start = if generate_for_both_colours
                         || self.side == board::BoardSide::White {0} else {1};

        let skip = if generate_for_both_colours {1} else {2};
        let mut i = start;

        while i <= BLACK_PAWN {
            self.generate_moves_piece(i, &mut moves_vec);
            i += skip;
        }
        moves_vec
    }
}
