use rust_chess::game::board::*;
use rust_chess::game::rules::*;

struct PieceEvaluation {
    pub king: f32,
    pub queen: f32,
    pub rook: f32,
    pub knignt: f32,
    pub bishop: f32,
    pub pawn: f32,
}
pub struct Evaluator {
    pieces_values: PieceEvaluation,

    pub low_level_eval_called: i32,
}

impl PieceEvaluation {
    pub fn new() -> Self {
        PieceEvaluation {
            king: 1000.0,
            queen: 9.0,
            rook: 5.0,
            knignt: 3.0,
            bishop: 3.0,
            pawn: 1.0,
        }
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            pieces_values: PieceEvaluation::new(),
            low_level_eval_called: 0,
        }
    }

    pub fn get_piece_value(&self, piece: ChessPiece) -> f32 {
        return match piece {
            ChessPiece::None => 0.0,
            ChessPiece::PawnBlack => -self.pieces_values.pawn,
            ChessPiece::PawnWhite => self.pieces_values.pawn,
            ChessPiece::RookBlack => -self.pieces_values.rook,
            ChessPiece::RookWhite => self.pieces_values.rook,
            ChessPiece::KnightBlack => -self.pieces_values.knignt,
            ChessPiece::KnightWhite => self.pieces_values.knignt,
            ChessPiece::BishopBlack => -self.pieces_values.bishop,
            ChessPiece::BishopWhite => self.pieces_values.bishop,
            ChessPiece::QueenBlack => -self.pieces_values.queen,
            ChessPiece::QueenWhite => self.pieces_values.queen,
            ChessPiece::KingBlack => -self.pieces_values.king,
            ChessPiece::KingWhite => self.pieces_values.king,
        };
    }

    pub fn eval(&mut self, board: ChessBoardState, depth: i32, max: bool) -> (ChessMove, f32) {
        let all_moves = board.get_all_moves();
        if all_moves.len() == 0 {
            return (
                ChessMove {
                    mv: Move::new_from_str("a1-a1"),
                    move_type: ChessMoveType::Simple,
                },
                if max { -10000000.0 } else { 10000000.0 },
            );
        };

        let mut best_eval = if depth == 0 {
            (
                all_moves[0],
                self.simple_eval(board.get_new_pos_after_move(all_moves[0])),
            )
        } else {
            self.eval(board.get_new_pos_after_move(all_moves[0]), depth - 1, !max)
        };
        for i in 1..all_moves.len() {
            let eval = if depth == 0 {
                (
                    all_moves[i],
                    self.simple_eval(board.get_new_pos_after_move(all_moves[i])),
                )
            } else {
                (all_moves[i], self.eval(board.get_new_pos_after_move(all_moves[i]), depth - 1, !max).1)
            };

            if max && eval.1 > best_eval.1 || !max && eval.1 > best_eval.1 {
                best_eval = eval;
            }
        }
        return best_eval;
    }

    pub fn simple_eval(&mut self, board: ChessBoardState) -> f32 {
        self.low_level_eval_called += 1;
        let mut eval = 0.0;
        for i in 0..BOARD_ARRAY_SIZE {
            eval += self.get_piece_value(board.board[i]);
        }
        return eval;
    }
}
