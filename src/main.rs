#![deny(warnings)]
use::rust_chess::game::board::*;
//use::rust_chess::game::rules::*;

fn main() {
    let mut board = ChessBoardState::new();
    board.parse_fen("r1b1kbnr/ppp3p1/2n5/1B1qppPp/3P3N/2N1B3/PPP2P1P/R2QK2R w Kq h6 1 38").unwrap();
    board.debug_print();
}