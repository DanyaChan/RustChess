use::rust_chess::game::board::*;

use crate::evaluation::Evaluator;
pub mod evaluation;

fn main() {
    let mut board = ChessBoardState::new();
    board.parse_fen("3r1k2/3B3p/b1p1PBp1/p2P1pnq/Pp6/6P1/P2Q1PKP/2RR4 b - - 2 28").unwrap();
    let mut eval = Evaluator::new();
    let best_move = eval.evaluate(&board, 5);
    print!("eval {} {}\n", best_move.0, eval.low_level_eval_called);

    for mv in best_move.1 {
        println!("Move: {} {}", board.get_move_string(mv.0), mv.1);
    }
}