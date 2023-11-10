#![allow(dead_code)]

use phf::{Map, phf_map};

pub const WHITE_CASTLE_KINGSIDE:  usize = 0;
pub const WHITE_CASTLE_QUEENSIDE: usize = 1;
pub const BLACK_CASTLE_KINGSIDE:  usize = 2;
pub const BLACK_CASTLE_QUEENSIDE: usize = 3;

pub mod pieces {
    pub const WHITE_KING: usize = 0;
    pub const BLACK_KING: usize = 1;
    pub const WHITE_QUEEN: usize = 2;
    pub const BLACK_QUEEN: usize = 3;
    pub const WHITE_ROOK: usize = 4;
    pub const BLACK_ROOK: usize = 5;
    pub const WHITE_BISHOP: usize = 6;
    pub const BLACK_BISHOP: usize = 7;
    pub const WHITE_KNIGHT: usize = 8;
    pub const BLACK_KNIGHT: usize = 9;
    pub const WHITE_PAWN: usize = 10;
    pub const BLACK_PAWN: usize = 11;
    pub const WHITE_PIECES: usize = 12;
    pub const BLACK_PIECES: usize = 13;
    pub const ALL_PIECES: usize = 14;
    pub const NULL: usize = 15;
}

pub mod attacks {
    pub const WHITE_ATTACKS: usize = 0;
    pub const BLACK_ATTACKS: usize = 1;
}

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const EMPTY_FEN:    &str = "8/8/8/8/8/8/8/8 w - - 0 1";

pub(super) const CHAR_TO_PIECE: Map<&str, usize> = phf_map! {
    "K" => pieces::WHITE_KING,
    "k" => pieces::BLACK_KING,
    "Q" => pieces::WHITE_QUEEN,
    "q" => pieces::BLACK_QUEEN,
    "R" => pieces::WHITE_ROOK,
    "r" => pieces::BLACK_ROOK,
    "B" => pieces::WHITE_BISHOP,
    "b" => pieces::BLACK_BISHOP,
    "N" => pieces::WHITE_KNIGHT,
    "n" => pieces::BLACK_KNIGHT,
    "P" => pieces::WHITE_PAWN,
    "p" => pieces::BLACK_PAWN,
};

pub(super) const ICON_TO_PIECE: Map<&str, usize> = phf_map! {
    "♚" => pieces::WHITE_KING,
    "♔" => pieces::BLACK_KING,
    "♛" => pieces::WHITE_QUEEN,
    "♕" => pieces::BLACK_QUEEN,
    "♜" => pieces::WHITE_ROOK,
    "♖" => pieces::BLACK_ROOK,
    "♝" => pieces::WHITE_BISHOP,
    "♗" => pieces::BLACK_BISHOP,
    "♞" => pieces::WHITE_KNIGHT,
    "♘" => pieces::BLACK_KNIGHT,
    "♟︎" => pieces::WHITE_PAWN,
    "♙" => pieces::BLACK_PAWN,
};

pub mod castle_masks {
    pub const CASTLE_MASK_CHECK: [u64; 4] = [
        96, 12, 6917529027641081856, 864691128455135232
    ];

    pub const CASTLE_MASK_PIECES: [u64; 4] = [
        96, 14, 6917529027641081856, 1008806316530991104
    ];
}
