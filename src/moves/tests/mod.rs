#[cfg(test)]
mod pt_moves_test {
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