use ::rust_chess::game::board::*;
use std::io;
// rnbqkbnr/1ppp2pp/4pp2/8/p1BPP3/2N2Q1N/PPP2PPP/R1B1K2R b KQk - 1 8

pub mod evaluation;
use evaluation::Evaluator;
fn main() {
    let mut board =
        ChessBoardState::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .unwrap();
    let mut eval = Evaluator::new();
    let mut stdin = io::stdin();
    loop {
        
        // Read from standard input
        let mut mv;
        loop {
            print!("test3");
            let mut input = String::new();
            
                stdin.read_line(&mut input)
                .expect("Failed to read line");
            print!("test2 {}", input);
            mv = board.get_chess_move_from_string(&input);
            if board.is_legal_move(mv) {
                print!("test");
                break;
            } else {
                print!("Move {} is illegal", board.get_move_string(mv));
            }
        }
        board = board.get_new_pos_after_move(mv);
        board.debug_print();

        let res = eval.evaluate(&board, 4);
        println!("Computer move {}", board.get_move_string(res.1.last().unwrap().0));

        board = board.get_new_pos_after_move(res.1.last().unwrap().0);

        board.debug_print();
    }
}
