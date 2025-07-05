mod tests {
    use ::rust_chess::game::board::*;
    #[test]
    fn test_all_moves() {
        let board = ChessBoardState::from_fen(
            "r3k1n1/pPp3p1/Q1n2r2/1B1qppPp/1b1P3N/2N1B1Pb/PP5P/R3K2R w KQ h6 0 1",
        )
        .unwrap();
        let result = board.get_all_moves();
        let expected = [
            "Pa2-a3", "Pa2-a4", "Pb2-b3", "Ra1-b1", "Ra1-c1", "Ra1-d1", "Rh1-g1", "Rh1-f1",
            "Ke1-f1", "Ke1-d1", "Ke1-d2", "Ke1-e2", "Ke1-f2", "0-0-0", "Pg3-g4", "Pg5-h6e",
            "Pg5-g6", "Pg5-f6", "Nh4-g6", "Nh4-f5", "Nh4-f3", "Nh4-g2", "Be3-f4", "Be3-f2",
            "Be3-g1", "Be3-d2", "Be3-c1", "Pd4-e5", "Nc3-d5", "Nc3-e4", "Nc3-e2", "Nc3-d1",
            "Nc3-b1", "Nc3-a4", "Bb5-c6", "Bb5-c4", "Bb5-d3", "Bb5-e2", "Bb5-f1", "Bb5-a4",
            "Qa6-a5", "Qa6-a4", "Qa6-a3", "Qa6-b6", "Qa6-c6", "Qa6-a7", "Pb7-b8Q", "Pb7-b8R",
            "Pb7-b8B", "Pb7-b8N", "Pb7-a8Q", "Pb7-a8R", "Pb7-a8B", "Pb7-a8N",
        ];
        let mut v = vec![];
        for mv in &result {
            if !expected.contains(&board.get_move_string(*mv).as_str()) {
                print!("{}", &board.get_move_string(*mv).as_str());
            }
            v.push(board.get_move_string(*mv));
            assert!(expected.contains(&board.get_move_string(*mv).as_str()));
        }
        for mv in expected {
            if !v.contains(&mv.to_string()) {
                print!("mv {}", mv);
            }
        }
        assert_eq!(expected.len(), result.len());
    }

    // #[test]
    // fn test_all_moves_black() {
    //     let board = ChessBoardState::new_from_fen("r3k1n1/pPp3p1/Q1n2r2/1B1qppPp/1b1P3N/2N1B1Pb/PP5P/R3K2R b KQ h6 0 1").unwrap();
    //     let result = board.get_all_moves();
    //     let expected = [
    //         "Pa2-a3",
    //         "Pa2-a3",
    //     ];
    //     // for mv in &result {
    //     //     assert!(expected.contains(&board.get_move_string(*mv).as_str()));
    //     // }
    //     assert_eq!(expected.len(), result.len());
    // }

    #[test]
    fn test_all_moves_filetered() {
        let board = ChessBoardState::from_fen(
            "r3k1n1/pPp3p1/Q1n2r2/1B1qppPp/1b1P3N/2N1B1Pb/PP5P/R3K2R w KQ h6 0 1",
        )
        .unwrap();
        let result = board.get_all_moves_checked();
        let expected = [
            "Pa2-a3", "Pa2-a4", "Pb2-b3", "Ra1-b1", "Ra1-c1", "Ra1-d1", "Rh1-g1", "Rh1-f1",
            "Ke1-f2", "Ke1-d1", "Ke1-d2", "Ke1-e2", "0-0-0", "Pg3-g4", "Pg5-h6e",
            "Pg5-g6", "Pg5-f6", "Nh4-g6", "Nh4-f5", "Nh4-f3", "Nh4-g2", "Be3-f4", "Be3-f2",
            "Be3-g1", "Be3-d2", "Be3-c1", "Pd4-e5", "Bb5-c6", "Bb5-c4", "Bb5-d3", "Bb5-e2", "Bb5-f1", "Bb5-a4",
            "Qa6-a5", "Qa6-a4", "Qa6-a3", "Qa6-b6", "Qa6-c6", "Qa6-a7", "Pb7-b8Q", "Pb7-b8R",
            "Pb7-b8B", "Pb7-b8N", "Pb7-a8Q", "Pb7-a8R", "Pb7-a8B", "Pb7-a8N",
        ];
        let mut v = vec![];
        for mv in &result {
            if !expected.contains(&board.get_move_string(*mv).as_str()) {
                print!("lacks {}", &board.get_move_string(*mv).as_str());
            }
            v.push(board.get_move_string(*mv));
            assert!(expected.contains(&board.get_move_string(*mv).as_str()));
        }
        for mv in expected {
            if !v.contains(&mv.to_string()) {
                print!("more {}", mv);
            }
        }
        assert_eq!(expected.len(), result.len());
    }

    // #[test]
    // fn test_all_moves_filetered_black() {
    //     let board = ChessBoardState::new_from_fen("r3k1n1/pPp3p1/Q1n2r2/1B1qppPp/1b1P3N/2N1B1Pb/PP5P/R3K2R b KQ h6 0 1").unwrap();
    //     let result = board.get_all_moves_checked();
    //     let expected = [
    //         "Pa2-a3"
    //     ];
    //     // for mv in &result {
    //     //     assert!(expected.contains(&board.get_move_string(*mv).as_str()));
    //     // }
    //     assert_eq!(expected.len(), result.len());
    // }
}
