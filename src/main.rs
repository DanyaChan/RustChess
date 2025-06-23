mod board;

use board::ChessBoardState;
fn main() {
    let board = ChessBoardState::new();
    board.debug_print();
}