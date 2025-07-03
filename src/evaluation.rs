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

    pub fn evaluate(
        &mut self,
        board: &ChessBoardState,
        depth: usize,
    ) -> (f32, Vec<(ChessMove, f32)>) {
        let cur_eval = self.simple_eval(board);
        let max = board.turn == Color::White;
        let mut branch = vec![Self::get_base_move(0.0); depth];
        return (self.eval(cur_eval, -1000000.0, 1000000.0, *board, max, depth, &mut branch), branch);
    }

    fn get_base_move(value: f32) -> (ChessMove, f32) {
        (
            ChessMove {
                mv: Move::new_from_str("a1-a1"),
                move_type: ChessMoveType::Simple,
            },
            value
        )
    }

    fn eval(
        &mut self,
        cur_eval: f32,
        mut alpha: f32,
        mut beta: f32,
        board: ChessBoardState,
        max: bool,
        depth: usize,
        branch: &mut Vec<(ChessMove, f32)>
    ) -> f32 {
        if depth == 0 {
            self.low_level_eval_called += 1;
            return cur_eval;
        }
        let all_moves = board.get_all_moves();
        let mut best_eval = Self::get_base_move(if !max { 1000000.0 } else { -1000000.0 });
        for i in 0..all_moves.len() {
            let eval = {
                let (new_board, res) = board.get_new_pos_after_move_for_eval(all_moves[i]);
                if res.remove == ChessPiece::KingBlack || res.remove == ChessPiece::KingWhite {
                    Self::get_base_move(if res.remove == ChessPiece::KingBlack { 1000000.0 } else { -1000000.0 })
                } else {
                let value= self.eval(
                    cur_eval + self.get_result_eval_diff(res),
                    alpha,
                    beta,
                    new_board,
                    !max,
                    depth - 1,
                    branch
                );
                    (all_moves[i], value)
                }
            };

            if max && eval.1 > best_eval.1 || !max && eval.1 < best_eval.1 {
                best_eval = eval.clone();
            }
            if max {
                if eval.1 > beta {
                    break;
                }
                if eval.1 > alpha {
                    alpha = eval.1
                }
            } else {
                if eval.1 < alpha {
                    break;
                }
                if eval.1 < beta {
                    beta = eval.1
                }
            }
        }

        branch[depth - 1] = best_eval;
        return best_eval.1;
    }

    pub fn get_result_eval_diff(&self, move_res: MoveResult) -> f32 {
        return self.get_piece_value(move_res.new) - self.get_piece_value(move_res.remove);
    }

    pub fn simple_eval(&self, board: &ChessBoardState) -> f32 {
        let mut eval = 0.0;
        for i in 0..BOARD_ARRAY_SIZE {
            eval += self.get_piece_value(board.board[i]);
        }
        return eval;
    }
}
