use::rust_chess::game::board::*;

fn main() {
    let mut board = ChessBoardState::new();
    board.parse_fen("r1b1kbnr/ppp3p1/2n5/1B1qppPp/3P3N/2N1B3/PPP2P1P/R2QK2R w Kq h6 1 38").unwrap();
    board.debug_print();
    let v = board.get_all_moves_from_pos(Pos::new_from_str("c3"));
    let vs: Vec<String> = v.iter().map(|x| board.get_move_string(*x)).collect();
    for s in vs {
        println!("{}", s);
    }
}