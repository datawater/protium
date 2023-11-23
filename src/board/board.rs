#![allow(dead_code)]
use std::ops::*;
use std::fmt;

pub use super::consts::*;
use super::consts::pieces::*;
use crate::pt_error;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct BitBoard(pub u64);
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BoardSide {
    #[default]
    White, 
    Black
}

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub pieces:  [BitBoard; 15],
    pub attacks: [BitBoard; 2],
    pub side:    BoardSide,
    pub en_passant_square: BitBoard,

    pub castle_allowed: [bool; 4],
    pub in_check: bool,
    pub in_double_check: bool,
    pub check_ray_mask: BitBoard,
    pub piece_that_checks_loc: u64,
    pub piece_that_checks: usize,

    print_using: bool,
}

#[inline]
fn mirror_horizontal(mut x: u64) -> u64 {
    let k1 = 0x5555555555555555u64;
    let k2 = 0x3333333333333333u64;
    let k4 = 0x0f0f0f0f0f0f0f0fu64;
    x = ((x >> 1) & k1) | ((x & k1) << 1);
    x = ((x >> 2) & k2) | ((x & k2) << 2);
    x = ((x >> 4) & k4) | ((x & k4) << 4);
    
    x
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        let pieces_eq = self.pieces .iter().zip(other.pieces .iter()).all(|(a,b)| a == b);
        let attack_eq = self.attacks.iter().zip(other.attacks.iter()).all(|(a,b)| a == b);
        
        let castle_eq = self.castle_allowed.iter().zip(other.castle_allowed.iter()).all(|(a,b)| a == b);

        pieces_eq && attack_eq && castle_eq && self.side == other.side && self.en_passant_square == other.en_passant_square
    }
}

impl Default for Board {
    fn default() -> Self {
        Self { 
            pieces: Default::default(),
            attacks: Default::default(), 
            side: Default::default(), 
            en_passant_square: 
            Default::default(),
            castle_allowed: Default::default(), 
            in_check: Default::default(), 
            in_double_check: Default::default(), 
            check_ray_mask: BitBoard(u64::MAX), 
            print_using: Default::default(),
            piece_that_checks_loc: 0,
            piece_that_checks: NULL as usize
        }
    }
}

impl Eq for Board {}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {BitBoard(self.0 | rhs.0)}
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {self.0 |= rhs.0}
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {BitBoard(self.0 & rhs.0)}
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {self.0 &= rhs.0}
}

impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: Self) -> Self::Output {BitBoard(self.0 ^ rhs.0)}
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {self.0 ^= rhs.0}
}

impl From<&str> for Board {
    /// A function that takes in a FEN (https://en.wikipedia.org/wiki/Forsyth–Edwards_Notation) string and returns an initialized struct Board
    fn from(_string: &str) -> Board {
        let string = _string.as_bytes();
        let mut rank = 7;
        let mut file = 0;
        let mut to_return: Self = unsafe { std::mem::zeroed() };

        let mut i = 0;
        
        while i < _string.len() {
            let c = string[i];

            match c {
                b'/' => {
                    rank -= 1; file = 0; i += 1; continue;
                }

                b'0'..=b'9' => {
                    let as_number = c - b'1';
                    file += as_number;
                }

                b' ' => {
                    i += 1;
                    to_return.side = if string[i] == b'w' {BoardSide::White} else {BoardSide::Black};
                    
                    i += 2;

                    while string[i] != b' ' {
                        let char = string[i];
                        
                        if char == b'-' {
                            i += 1;
                            break;
                        }

                        if char == b'K' {to_return.castle_allowed[WHITE_CASTLE_KINGSIDE]  = true;}
                        if char == b'k' {to_return.castle_allowed[BLACK_CASTLE_KINGSIDE]  = true;}
                        if char == b'Q' {to_return.castle_allowed[WHITE_CASTLE_QUEENSIDE] = true;}
                        if char == b'q' {to_return.castle_allowed[BLACK_CASTLE_QUEENSIDE] = true;}

                        i += 1;
                    }

                    i += 1;

                    if string[i] != b'-' {
                        to_return.en_passant_square = BitBoard(
                            1 << (
                            (string[i + 1] as u64 - '1' as u64) * 8
                          +  string[i]     as u64 - 'a' as u64));
                    } else {
                        to_return.en_passant_square = BitBoard(0);
                    }

                    break;
                }

                b'a'..=b'z' | b'A'..=b'Z' => {
                    let square = 1u64 << (rank * 8 + file);

                    to_return.pieces[CHAR_TO_PIECE[&(c as char).to_string()]] |= BitBoard(square);
                    to_return.pieces[WHITE_PIECES + (c as char).is_lowercase() as usize] |= BitBoard(square);
                    to_return.pieces[ALL_PIECES] |= BitBoard(square);
                }

                _ => {
                    pt_error!("Invalid charachter {} in fen string specified.", c);
                }                
            }

            file += 1;
            i += 1;
        }
        
        to_return.generate_attacks();

        to_return
    }
}

impl Board {
    #[inline]
    pub fn print_using_chars(&mut self) {self.print_using = false;}

    #[inline]
    pub fn print_using_unicode(&mut self) {self.print_using = true;}
}

// Originally implemented by https://github.com/Ellipse0934 in c++ for an old project of mine.
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = vec![". ".to_string(); 64];

        for piece in if !self.print_using {CHAR_TO_PIECE.entries()} else {ICON_TO_PIECE.entries()} {
            let piece_char = piece.0;
            
            // Mirroring is necessary for it to properly display.
            let piece_map = mirror_horizontal(self.pieces[*piece.1].0);

            for i in 0..64 {
                string[64 - i - 1] = if ((piece_map >> i) & 1) != 0 {piece_char.to_owned().to_string() + " "} else {string[64 - i - 1].to_string()};
            }
        }

        for (i, c) in string.iter().enumerate() {
            write!(f, "{}", c)?;
            if (i + 1) % 8 == 0 {writeln!(f)?;}
        }
        
        writeln!(f)
    }
}
