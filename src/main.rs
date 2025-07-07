use ::rust_chess::game::board::*;
use std::io::{self, Write};
// rnbqkbnr/1ppp2pp/4pp2/8/p1BPP3/2N2Q1N/PPP2PPP/R1B1K2R b KQk - 1 8
// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1

pub mod evaluation;
use evaluation::Evaluator;
fn main() {
    let board =
        ChessBoardState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .unwrap();
    let mut eval = Evaluator::new();
    let e = eval.evaluate(&board, 8);
    print!("Move {}", e.1.last().unwrap().0.get_move_string());
}
