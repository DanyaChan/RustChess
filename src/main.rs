use::rust_chess::game::board::*;

use crate::evaluation::Evaluator;
pub mod evaluation;

fn main() {
    let mut board = ChessBoardState::new();
    board.parse_fen("k7/6R1/8/7P/8/8/8/5K2 w - - 0 1").unwrap();
    let mut eval = Evaluator::new();
    let best_move = eval.eval(board, 6);
    //print!("move {} eval {} {}\n", board.get_move_string(best_move.0), best_move.1, eval.low_level_eval_called);

    for mv in best_move.2 {
        println!("Move: {}", mv)
    }
}