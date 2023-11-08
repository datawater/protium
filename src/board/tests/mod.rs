#[cfg(test)]
mod pt_board_tests {
    use crate::board::*;

    #[test]
    fn board_fen_init() {
        let board_fen_starting = Board::from(board::fens::STARTING_FEN);

        let correct_piece_arengment = [16u64, 1152921504606846976u64, 8u64, 576460752303423488u64, 129u64, 9295429630892703744u64, 36u64, 2594073385365405696u64, 66u64, 4755801206503243776u64, 65280u64, 71776119061217280u64, 65535u64, 18446462598732840960u64, 18446462598732906495u64];

        for (i, p) in correct_piece_arengment.iter().enumerate() {
            assert!(board_fen_starting.pieces[i] == BitBoard(*p));
        }

        let board_fen_empty = Board::from(board::fens::EMPTY_FEN);
        assert!(board_fen_empty.pieces.iter().all(|x| x.0 == 0));
    }
}