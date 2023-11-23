
#[cfg(test)]
mod pt_attacks_test {
    use crate::board::{Board, BitBoard, attacks::*};
    
    #[test]
    fn test_knight_attack_generation() {
        let board_1 = Board::from("8/5N2/8/8/8/7N/8/1N6 w - - 0 1");
        assert_eq!(board_1.attacks[WHITE_ATTACKS], BitBoard(9799982666874169408u64));

        let board_2 = Board::from("8/6n1/8/7n/4n3/8/8/8 w - - 0 1");
        assert_eq!(board_2.attacks[BLACK_ATTACKS], BitBoard(1170998455561365504u64));
    }

    #[test]
    fn test_king_attack_generation() {
        let board_1 = Board::from("8/6K1/8/8/8/8/1K6/8 w - - 0 1");
        assert_eq!(board_1.attacks[WHITE_ATTACKS], BitBoard(16186183351374644487));

        let board_2 = Board::from("8/8/8/8/8/8/1K6/K7 w - - 0 1");
        assert_eq!(board_2.attacks[WHITE_ATTACKS], BitBoard(460551));
    }

    #[test]
    fn test_pawn_attack_generation() {
        let board_1 = Board::from("7P/1P6/8/8/8/8/8/8 w - - 0 1");
        assert_eq!(board_1.attacks[WHITE_ATTACKS], BitBoard(360287970189639680));

        let board_2 = Board::from("8/8/8/8/8/8/1p6/7p b - - 0 1");
        assert_eq!(board_2.attacks[BLACK_ATTACKS], BitBoard(5));
    }

    #[test]
    fn test_rook_attack_generation() {
        let board_1 = Board::from("8/8/8/8/8/3p4/2pRp3/3p4 w - - 0 1");
        assert_eq!(board_1.attacks[WHITE_ATTACKS], BitBoard(529416));

        let board_2 = Board::from("8/8/8/8/8/8/3R4/8 w - - 0 1");
        assert_eq!(board_2.attacks[WHITE_ATTACKS], BitBoard(578721382704674568));
    }

    #[test]
    fn test_bishop_attack_generation() {
        let board_1 = Board::from("8/8/8/8/8/4p3/3B4/8 w - - 0 1");
        assert_eq!(board_1.attacks[WHITE_ATTACKS], BitBoard(4329832468));

        let board_2 = Board::from("8/8/8/8/8/8/3B4/8 w - - 0 1");
        assert_eq!(board_2.attacks[WHITE_ATTACKS], BitBoard(141017232965652));   
    }

    // No test for queen, because if both rook and bishop attack generations are working properly,
    // It should too, as it's only an OR of both bitboards
}

// Live, Laugh, Love: http://bernd.bplaced.net/fengenerator/fengenerator.html

#[cfg(test)]
mod pt_moves_test {
    use crate::board::{Board, pieces::*};
    
    #[test]
    fn test_knight_move_generation() {
        let mut moves_vec = Vec::with_capacity(70);
        
        let board_1 = Board::from("2K1b3/2n3pp/1bp2P1P/rPpQPN2/PP5p/Pp1R1pBR/1r3pBP/n3k2q w - - 0 1");
        board_1.generate_moves_piece(WHITE_KNIGHT, &mut moves_vec);
        
        assert_eq!(moves_vec.len(), 6);

        moves_vec.clear();

        let board_2 = Board::from("8/PP4nQ/1q4bR/bpp2pp1/PrPPP1N1/1ppR1Pr1/2P2Bpp/1B1nK2k w - - 0 1");
        board_2.generate_moves_piece(WHITE_KNIGHT, &mut moves_vec);
        
        assert_eq!(moves_vec.len(), 4);
    }

    #[test]
    fn test_king_move_generation() {
        let mut moves_vec = Vec::with_capacity(70);
        
        let board_1 = Board::from("6KR/1NB2p1p/PPPPp2P/p1rPq1k1/Q1b4R/ppP1b1np/1p3r1P/1n3B2 w - - 0 1");
        board_1.generate_moves_piece(WHITE_KING, &mut moves_vec);
        
        assert_eq!(moves_vec.len(), 2);

        moves_vec.clear();

        let board_2 = Board::from("3B1R2/2p1nr1n/pbP2p1k/rNpp4/P1Bp2bP/p1P1R1KP/2PPq1Pp/7Q w - - 0 1");
        board_2.generate_moves_piece(WHITE_KING, &mut moves_vec);
        
        assert_eq!(moves_vec.len(), 2);

        moves_vec.clear();

        // Castling tests
        // White castling
        let board_3 = Board::from("8/8/8/8/8/8/8/R3K2R w KQ - 0 1");
        board_3.generate_moves_piece(WHITE_KING, &mut moves_vec);
        
        assert_eq!(moves_vec.len(), 7);

        moves_vec.clear();

        // Black castling
        let board_4 = Board::from("r3k2r/8/8/8/8/8/8/8 b kq - 0 1");
        board_4.generate_moves_piece(BLACK_KING, &mut moves_vec);
        
        assert_eq!(moves_vec.len(), 7);

        println!("{:#?}", moves_vec);        

        moves_vec.clear();

        // Castling blocked by piece and check
        let board_4 = Board::from("8/8/8/8/5b2/8/8/R3Kn1R w KQ - 0 1");
        board_4.generate_moves_piece(WHITE_KING, &mut moves_vec);
        
        assert_eq!(moves_vec.len(), 4);
    }

    #[test]
    fn test_pawn_move_generation() {
        let mut moves_vec = Vec::with_capacity(70);

        // * White: En passant
        let board_1 = Board::from("8/8/8/3pPp2/8/8/8/8 w - f6 0 3");
        board_1.generate_moves_piece(WHITE_PAWN, &mut moves_vec);
        
        assert_eq!(moves_vec.len(), 2);

        moves_vec.clear();

        // * White: Single and double move
        let board_2 = Board::from("8/8/8/8/8/8/3P4/8 w - - 0 1");
        board_2.generate_moves_piece(WHITE_PAWN, &mut moves_vec);
        
        assert_eq!(moves_vec.len(), 2);

        moves_vec.clear();

        // * White: Capturing
        let board_2 = Board::from("8/8/8/8/2p1p3/3P4/8/8 w - - 0 1");
        board_2.generate_moves_piece(WHITE_PAWN, &mut moves_vec);

        assert_eq!(moves_vec.len(), 3);

        moves_vec.clear();

        // * White: Promotion
        let board_2 = Board::from("8/7P/8/8/8/8/8/8 w - - 0 1");
        board_2.generate_moves_piece(WHITE_PAWN, &mut moves_vec);

        assert_eq!(moves_vec.len(), 4);

        moves_vec.clear();

        // * Black: En passant
        let board_1 = Board::from("8/8/8/8/2Pp4/8/8/8 b - c3 0 3");
        board_1.generate_moves_piece(BLACK_PAWN, &mut moves_vec);
        
        assert_eq!(moves_vec.len(), 2);

        moves_vec.clear();

        // * Black: Single and double move
        let board_2 = Board::from("8/4p3/8/8/8/8/8/8 w - - 0 1");
        board_2.generate_moves_piece(BLACK_PAWN, &mut moves_vec);

        assert_eq!(moves_vec.len(), 2);

        moves_vec.clear();

        // * Black: Capturing
        let board_2 = Board::from("8/8/8/4p3/3P1P2/8/8/8 b - - 0 1");
        board_2.generate_moves_piece(BLACK_PAWN, &mut moves_vec);
        
        assert_eq!(moves_vec.len(), 3);
        
        moves_vec.clear();

        // * Black: Promotion
        let board_2 = Board::from("8/8/8/8/8/8/7p/8 w - - 0 1");
        board_2.generate_moves_piece(BLACK_PAWN, &mut moves_vec);

        assert_eq!(moves_vec.len(), 4);
    }

    #[test]
    fn test_rook_attack_generation() {
        let mut moves_vec = Vec::with_capacity(70);
        
        let board_1 = Board::from("1N3K1Q/1BPp4/pP2P2r/3P1N1k/p1pppr1P/1P1nB1pP/P2np3/2b1qR1b w - - 0 1");
        board_1.generate_moves_piece(WHITE_ROOK, &mut moves_vec);
        
        assert_eq!(moves_vec.len(), 6);

        moves_vec.clear();

        let board_2 = Board::from("6b1/2B2p1p/1P1N1k1P/PnKB1Pb1/pp2r2R/Pp2pr1P/P1Pp2p1/1qnN2Q1 w - - 0 1");
        board_2.generate_moves_piece(WHITE_ROOK, &mut moves_vec);
        
        assert_eq!(moves_vec.len(), 4);
    }

    #[test]
    fn test_bishop_attack_generation() {
        let mut moves_vec = Vec::with_capacity(70);
        
        let board_1 = Board::from("1n5B/1p1pKNp1/P2pbp1b/P1R2P2/PP2r1QP/p1nNp1qp/P1P1R3/k1r5 w - - 0 1");

        board_1.generate_moves_piece(WHITE_BISHOP, &mut moves_vec);
        
        assert_eq!(moves_vec.len(), 1);

        moves_vec.clear();

        let board_2 = Board::from("6N1/1p2RP1k/1qp2QrP/pP2PPp1/nNpPn1p1/2p2RPP/1rp5/b4bBK w - - 0 1");
        board_2.generate_moves_piece(WHITE_BISHOP, &mut moves_vec);
        
        assert_eq!(moves_vec.len(), 3); 
    }

    // No test for queen, because if both rook and bishop attack generations are working properly,
    // It should too, as it's only an OR of both bitboards
}