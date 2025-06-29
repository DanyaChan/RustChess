use::rust_chess::game::board::*;

fn main() {
    let mut board = ChessBoardState::new();
    board.parse_fen("r3k1n1/pPp3p1/Q1n2r2/1B1qppPp/1b1P3N/2N1B1Pb/PP5P/R3K2R b KQ h6 0 1").unwrap();
    let v = board.get_all_moves_from_pos_filtered(Pos::new_from_str("b7"));
    let vs: Vec<String> = v.iter().map(|x| board.get_move_string(*x)).collect();
    for s in vs {
        println!("{}", s);
    }
}