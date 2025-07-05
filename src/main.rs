use ::rust_chess::game::board::*;
use std::io::{self, Write};
// rnbqkbnr/1ppp2pp/4pp2/8/p1BPP3/2N2Q1N/PPP2PPP/R1B1K2R b KQk - 1 8
// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1

pub mod evaluation;
use evaluation::Evaluator;
fn main() {
    let mut board =
        ChessBoardState::from_fen("2r1kb1r/p3pp2/2p2np1/7p/1P2b3/5B1P/PRP2PP1/2B1R1K1 w - - 0 1")
            .unwrap();
    let mut eval = Evaluator::new();
    board.debug_print();
    loop {
        let mut mv;
        loop {
            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Failed to read line");
            mv = board.get_chess_move_from_string(&input[0..input.len() - 1]);

            if mv.is_none() {
                println!("Wrong move format");
            } else if board.is_legal_move(mv.unwrap()) {
                break;
            } else {
                println!("Move {} is illegal", board.get_move_string(mv.unwrap()));
                io::stdout().flush().expect("Unable To Flush");
            }
        }
        board = board.get_new_pos_after_move(mv.unwrap());
        board.debug_print();

        let res = eval.evaluate(&board, 6);
        println!(
            "Computer move {}",
            board.get_move_string(res.1.last().unwrap().0)
        );

        board = board.get_new_pos_after_move(res.1.last().unwrap().0);

        board.debug_print();
    }
}
