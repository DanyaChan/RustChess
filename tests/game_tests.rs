
mod tests {
    use::rust_chess::game::board::*;
    #[test]
    fn test_all_moves() {
        let board = ChessBoardState::new_from_fen("r3k1n1/ppp3pP/Q1n2r2/1B1qpp1p/1b1P1P1N/2N1B2b/PPP4P/R3K2R w KQ - 0 1").unwrap();
        let result = board.get_all_moves();
        let expected = [
            "Pa2-a3"
        ];
        // for mv in &result {
        //     assert!(expected.contains(&board.get_move_string(*mv).as_str()));
        // }
        assert_eq!(expected.len(), result.len());
    }

    #[test]
    fn test_all_moves_black() {
        let board = ChessBoardState::new_from_fen("r3k1n1/ppp3pP/Q1n2r2/1B1qpp1p/1b1P1P1N/2N1B2b/PPP4P/R3K2R b KQ - 0 1").unwrap();
        let result = board.get_all_moves();
        let expected = [
            "Pa2-a3"
        ];
        // for mv in &result {
        //     assert!(expected.contains(&board.get_move_string(*mv).as_str()));
        // }
        assert_eq!(expected.len(), result.len());
    }

    #[test]
    fn test_all_moves_filetered() {
        let board = ChessBoardState::new_from_fen("r3k1n1/ppp3pP/Q1n2r2/1B1qpp1p/1b1P1P1N/2N1B2b/PPP4P/R3K2R w KQ - 0 1").unwrap();
        let result = board.get_all_moves_checked();
        let expected = [
            "Pa2-a3"
        ];
        // for mv in &result {
        //     assert!(expected.contains(&board.get_move_string(*mv).as_str()));
        // }
        assert_eq!(expected.len(), result.len());
    }

    #[test]
    fn test_all_moves_filetered_black() {
        let board = ChessBoardState::new_from_fen("r3k1n1/ppp3pP/Q1n2r2/1B1qpp1p/1b1P1P1N/2N1B2b/PPP4P/R3K2R b KQ - 0 1").unwrap();
        let result = board.get_all_moves_checked();
        let expected = [
            "Pa2-a3"
        ];
        // for mv in &result {
        //     assert!(expected.contains(&board.get_move_string(*mv).as_str()));
        // }
        assert_eq!(expected.len(), result.len());
    }
}