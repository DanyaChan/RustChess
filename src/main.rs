use ::rust_chess::game::board::*;
use rust_chess::game::rules::{ChessMove, ChessMoveType};
use std::io;
// rnbqkbnr/1ppp2pp/4pp2/8/p1BPP3/2N2Q1N/PPP2PPP/R1B1K2R b KQk - 1 8

pub mod evaluation;
use evaluation::Evaluator;
fn main() {
    let mut board =
        ChessBoardState::new_from_fen("r4k1r/1pp1b1pp/2n1Q2n/3N2N1/8/3q4/PB3PPP/4R1KR w - - 7 26d")
            .unwrap();
    let mut eval = Evaluator::new();
    loop {
        let mut input = String::new();

        // Read from standard input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mv = Move::new_from_str(&input);
        board = board.get_new_pos_after_move(ChessMove {
            mv: mv,
            move_type: ChessMoveType::Simple
        });
        board.debug_print();

        let res = eval.evaluate(&board, 6);
        println!("Computer move {}", board.get_move_string(res.1.last().unwrap().0));

        board = board.get_new_pos_after_move(res.1.last().unwrap().0);

        board.debug_print();
    }
}
