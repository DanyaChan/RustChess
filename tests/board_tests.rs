


#[cfg(test)]
mod tests {
    use::rust_chess::game::board::*;
    #[test]
    fn test_load_pos() {
        let pos1 = Pos::from_str("e2");
        let pos2 = Pos::from_str("c7");
        let pos3 = Pos::from_str("h1");
        let pos4 = Pos::from_str("");

        assert_eq!(pos1, Pos{x:4, y:1});
        assert_eq!(pos2, Pos{x:2, y:6});
        assert_eq!(pos3, Pos{x:7, y:0});
        assert_eq!(pos4, Pos{x:0xf, y:0xf});
    }

    #[test]
    fn run_load_move() {
        let move1 = Move::from_str("e2-e4");
        assert_eq!(move1.get_code(), 0x3414);

        let move2 = Move::from_str("f4-h3");
        assert_eq!(move2.get_code(), 0x2735);
    }

    #[test]
    fn test_load_fen_default() {
        let fen = ChessBoardState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(fen.turn, Color::White);
        assert_eq!(fen.en_passant, 0xFF);
        assert_eq!(fen.castle_state_flags, 0x0F);
        assert_eq!(fen.move_num, 1);
        assert_eq!(fen.halfmoves_to_draw, 0);
        let expected = [
            ChessPiece::RookWhite, ChessPiece::KnightWhite, ChessPiece::BishopWhite, ChessPiece::QueenWhite, ChessPiece::KingWhite, ChessPiece::BishopWhite, ChessPiece::KnightWhite, ChessPiece::RookWhite,
            ChessPiece::PawnWhite, ChessPiece::PawnWhite, ChessPiece::PawnWhite, ChessPiece::PawnWhite, ChessPiece::PawnWhite, ChessPiece::PawnWhite, ChessPiece::PawnWhite, ChessPiece::PawnWhite,
            ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None,
            ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None,
            ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None,
            ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None,
            ChessPiece::PawnBlack, ChessPiece::PawnBlack, ChessPiece::PawnBlack, ChessPiece::PawnBlack, ChessPiece::PawnBlack, ChessPiece::PawnBlack, ChessPiece::PawnBlack, ChessPiece::PawnBlack,
            ChessPiece::RookBlack, ChessPiece::KnightBlack, ChessPiece::BishopBlack, ChessPiece::QueenBlack, ChessPiece::KingBlack, ChessPiece::BishopBlack, ChessPiece::KnightBlack, ChessPiece::RookBlack,
        ];
        for i in 0..BOARD_ARRAY_SIZE {
            assert_eq!(fen.board[i], expected[i]);
        }
    }

    #[test]
    fn test_load_fen() {
        let fen = ChessBoardState::from_fen("r1b1kbnr/ppp3p1/2n5/1B1qppPp/3P3N/2N1B3/PPP2P1P/R2QK2R w Kq h6 1 38").unwrap();
        assert_eq!(fen.turn, Color::White);
        assert_eq!(fen.en_passant, 0x57);
        assert_eq!(fen.castle_state_flags, 0x00 as u8 | CastleStateFlag::BlackLong as u8 | CastleStateFlag::WhiteShort as u8);
        assert_eq!(fen.move_num, 38);
        assert_eq!(fen.halfmoves_to_draw, 1);
        let expected = [
            ChessPiece::RookWhite, ChessPiece::None, ChessPiece::None, ChessPiece::QueenWhite, ChessPiece::KingWhite, ChessPiece::None, ChessPiece::None, ChessPiece::RookWhite,
            ChessPiece::PawnWhite, ChessPiece::PawnWhite, ChessPiece::PawnWhite, ChessPiece::None, ChessPiece::None, ChessPiece::PawnWhite, ChessPiece::None, ChessPiece::PawnWhite,
            ChessPiece::None, ChessPiece::None, ChessPiece::KnightWhite, ChessPiece::None, ChessPiece::BishopWhite, ChessPiece::None, ChessPiece::None, ChessPiece::None,
            ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::PawnWhite, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::KnightWhite,
            ChessPiece::None, ChessPiece::BishopWhite, ChessPiece::None, ChessPiece::QueenBlack, ChessPiece::PawnBlack, ChessPiece::PawnBlack, ChessPiece::PawnWhite, ChessPiece::PawnBlack,
            ChessPiece::None, ChessPiece::None, ChessPiece::KnightBlack, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::None,
            ChessPiece::PawnBlack, ChessPiece::PawnBlack, ChessPiece::PawnBlack, ChessPiece::None, ChessPiece::None, ChessPiece::None, ChessPiece::PawnBlack, ChessPiece::None,
            ChessPiece::RookBlack, ChessPiece::None, ChessPiece::BishopBlack, ChessPiece::None, ChessPiece::KingBlack, ChessPiece::BishopBlack, ChessPiece::KnightBlack, ChessPiece::RookBlack,
        ];
        for i in 0..BOARD_ARRAY_SIZE {
            assert_eq!(fen.board[i], expected[i]);
        }
    }
}