
use::rust_chess::game::board::*;
use::rust_chess::game::rules::*;

fn f(s : String) {

}

fn main() {
    let board = ChessBoardState::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    board.debug_print();
    let v = get_all_moves(board, Pos{x:0, y:0});
    print!("{}", v.len());
}