use rust_chess::game::board::*;
use rust_chess::game::rules::*;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use rust_chess::utils::thread_pool::{Task, ThreadPoolWrap};

#[derive(Debug, Clone, Copy)]
struct EvaluationCandidate {
    mv: ChessMove,
    value: f32,
}

impl PartialEq for EvaluationCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.mv == other.mv
    }
}
impl Eq for EvaluationCandidate {}
impl Ord for EvaluationCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
         self.value.partial_cmp(&other.value).unwrap()
    }
}
impl PartialOrd for EvaluationCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl EvaluationCandidate {
    fn new(mv: ChessMove, val: f32) -> Self {
        EvaluationCandidate { mv, value: val }
    }
}

#[derive(Debug, Clone, Copy)]
struct PieceEvaluation {
    pub king: f32,
    pub queen: f32,
    pub rook: f32,
    pub knight: f32,
    pub bishop: f32,
    pub pawn: f32,
}
#[derive(Debug, Clone, Copy)]
pub struct Evaluator {
    pieces_values: PieceEvaluation,
    castle_value: f32,
    pawn_pos_value: [f32; 8],
    center_pos_value: [f32; 8],

    pub low_level_eval_called: i32,
}
#[derive(Debug, Clone, Copy)]
struct AsyncEvalCandidate {
    evaluator: Evaluator,
    mv: ChessMove,
    max: bool,
    board: ChessBoardState,
    cur_eval: f32,
    depth: usize,
}

#[derive(Debug, Clone)]
struct AsyncEvalResult {
    mv: ChessMove,
    value: f32,
    branch: Vec<EvaluationCandidate>
}

fn async_evaluate_impl(mut data: AsyncEvalCandidate) -> AsyncEvalResult {
    let ret = data.evaluator.eval_async_branch_impl(data.cur_eval,
                                                    data.board,
                                                    data.max,
                                                    data.depth);
    println!("{}", data.evaluator.low_level_eval_called);
    AsyncEvalResult {
        mv: data.mv,
        value: ret.0,
        branch: ret.1,
    }
}

impl PieceEvaluation {
    pub fn new() -> Self {
        PieceEvaluation {
            king: 1000.0,
            queen: 9.0,
            rook: 5.0,
            knight: 3.0,
            bishop: 3.0,
            pawn: 1.0,
        }
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            pieces_values: PieceEvaluation::new(),
            castle_value: 0.3,
            pawn_pos_value: [0.0, 0.0, 0.05, 0.1, 0.1, 0.3, 1.0, 0.0],
            center_pos_value: [0.0, 0.02, 0.1, 0.2, 0.2, 0.1, 0.02, 0.0],
            low_level_eval_called: 0,
        }
    }

    pub fn get_piece_value(&self, piece: ChessPiece) -> f32 {
        match piece {
            ChessPiece::None => 0.0,
            ChessPiece::PawnBlack => -self.pieces_values.pawn,
            ChessPiece::PawnWhite => self.pieces_values.pawn,
            ChessPiece::RookBlack => -self.pieces_values.rook,
            ChessPiece::RookWhite => self.pieces_values.rook,
            ChessPiece::KnightBlack => -self.pieces_values.knight,
            ChessPiece::KnightWhite => self.pieces_values.knight,
            ChessPiece::BishopBlack => -self.pieces_values.bishop,
            ChessPiece::BishopWhite => self.pieces_values.bishop,
            ChessPiece::QueenBlack => -self.pieces_values.queen,
            ChessPiece::QueenWhite => self.pieces_values.queen,
            ChessPiece::KingBlack => -self.pieces_values.king,
            ChessPiece::KingWhite => self.pieces_values.king,
        }
    }

    pub fn evaluate(
        &mut self,
        board: &ChessBoardState,
        depth: usize,
    ) -> (f32, Vec<(ChessMove, f32)>) {
        self.low_level_eval_called = 0;
        let cur_eval = self.simple_eval(board);
        let max = board.turn == Color::White;
        let mut branch = vec![Self::get_base_move(0.0); depth];
        (
            self.eval(
                cur_eval,
                -1000000.0,
                1000000.0,
                *board,
                max,
                depth,
                &mut branch,
            ),
            branch.iter().map(|x| (x.mv, x.value)).collect(), //TODO refactor
        )
    }
    pub fn evaluate_async(
        &mut self,
        board: &ChessBoardState,
        depth: usize,
    ) -> (f32, Vec<(ChessMove, f32)>) {
        self.low_level_eval_called = 0;
        let cur_eval = self.simple_eval(board);
        let max = board.turn == Color::White;
        let tpw = ThreadPoolWrap::new(12);
        let mut tasks = VecDeque::new();
        let all_moves = board.get_all_moves();

        for mv in all_moves {
            let (new_board, res) = board.get_new_pos_after_move_for_eval(mv);
            let value = if max {
                cur_eval + self.get_result_eval_diff(&new_board, res, mv)
            } else {
                -cur_eval - self.get_result_eval_diff(&new_board, res, mv)
            };
            tasks.push_back(Task {
                data: AsyncEvalCandidate {
                    evaluator: *self,
                    cur_eval: value,
                    mv,
                    max: !max,
                    board: new_board,
                    depth: depth - 1
                },
                task: |x| async_evaluate_impl(x)
            });
        }
        tpw.put_tasks_into_queue(tasks);
        tpw.await_all_tasks();
        let tasks_res = tpw.get_all_tasks_and_clear();
        if tasks_res.is_empty() {
            return (0.0, vec![]);
        }
        let mut best_eval_idx = 0;
        for i in 1..tasks_res.len() {
            if max && tasks_res[i].value > tasks_res[best_eval_idx].value {
                best_eval_idx = i;
            }
            if !max && tasks_res[i].value < tasks_res[best_eval_idx].value {
                best_eval_idx = i;
            }
        }
        let best_eval = &tasks_res[best_eval_idx];
        let mut res_branch: Vec<(ChessMove, f32)> = best_eval.branch.iter().map(|x| (x.mv, x.value)).collect();
        res_branch.push((best_eval.mv, best_eval.value));
        (best_eval.value, res_branch)
    }

    fn get_base_move(value: f32) -> EvaluationCandidate {
        EvaluationCandidate {
            mv: ChessMove {
                mv: Move::from_str("a1-a1"),
                move_type: ChessMoveType::Simple,
            },
            value,
        }
    }

    // TODO refactor config in Evaluator 
    #[warn(dead_code)]
    fn get_depth(_: usize, cur: usize, depth: usize) -> usize {
        if depth >= 1 && cur < 6 {
            return depth - 1;
        }
        if depth >= 3 {
            return depth - 3;
        }
        (depth + 1) % 2
    }
    fn eval_async_branch_impl(
        &mut self,
        cur_eval: f32,
        board: ChessBoardState,
        max: bool,
        depth: usize
    ) -> (f32, Vec<EvaluationCandidate>) {
        let mut branch = vec![Self::get_base_move(0.0); depth];
        let res = self.eval(
            cur_eval,
            -1000000.0,
            1000000.0,
            board,
            max,
            depth,
            &mut branch,
        );
        (res, branch)
    }

    fn eval(
        &mut self,
        cur_eval: f32,
        mut alpha: f32,
        mut beta: f32,
        board: ChessBoardState,
        max: bool,
        depth: usize,
        branch: &mut Vec<EvaluationCandidate>,
    ) -> f32 {
        if depth == 0 {
            self.low_level_eval_called += 1;
            return cur_eval;
        }
        let all_moves = board.get_all_moves();
        if all_moves.is_empty() {
            return if !max { 1000000.0 } else { -1000000.0 };
        }
        let mut moves_queue = BinaryHeap::<EvaluationCandidate>::new();
        for mv in all_moves {
            let (new_board, res) = board.get_new_pos_after_move_for_eval(mv); // TODO optimise even more dont make new board twice
            let value = if max {
                cur_eval + self.get_result_eval_diff(&new_board, res, mv)
            } else {
                -cur_eval - self.get_result_eval_diff(&new_board, res, mv)
            };
            moves_queue.push(EvaluationCandidate {
                mv,
                value,
            });
        }
        let mut best_eval = Self::get_base_move(if !max { 10000000.0 } else { -10000000.0 });
        let moves_num = moves_queue.len();
        for i in 0..moves_num {
            let mv = moves_queue.pop().unwrap();
            let eval = {
                let (new_board, res) = board.get_new_pos_after_move_for_eval(mv.mv);
                if res.remove == ChessPiece::KingBlack || res.remove == ChessPiece::KingWhite {
                    Self::get_base_move(-Self::get_piece_value(&self, res.remove))
                } else {
                    let new_depth = Self::get_depth(moves_queue.len(), i, depth);
                    let value = self.eval(
                        cur_eval + self.get_result_eval_diff(&new_board, res, mv.mv),
                        alpha,
                        beta,
                        new_board,
                        !max,
                        new_depth,
                        branch,
                    );
                    EvaluationCandidate::new(mv.mv, value)
                }
            };

            if max && eval.value > best_eval.value || !max && eval.value < best_eval.value {
                best_eval = eval.clone();
            }
            if max {
                if eval.value > beta {
                    break;
                }
                if eval.value > alpha {
                    alpha = eval.value
                }
            } else {
                if eval.value < alpha {
                    break;
                }
                if eval.value < beta {
                    beta = eval.value
                }
            }
        }

        branch[depth - 1] = best_eval;
        best_eval.value
    }

    fn get_result_eval_diff(
        &self,
        board: &ChessBoardState,
        move_res: MoveResult,
        mv: ChessMove,
    ) -> f32 {
        let piece = board.get_piece_unsafe(mv.mv.to);
        let color = piece.get_color().unwrap();
        let mut sum = 0.0;
        let mult = if color == Color::White { 1.0 } else { -1.0 };
        if mv.move_type == ChessMoveType::CastleLong || mv.move_type == ChessMoveType::CastleLong {
            sum += self.castle_value * mult;
        }
        sum += self.get_piece_value_from_pos(piece, mv.mv.to)
            - self.get_piece_value_from_pos(piece, mv.mv.from);
        sum += self.get_piece_value(move_res.new) - self.get_piece_value(move_res.remove);
        sum
    }

    fn get_piece_value_from_pos(&self, piece: ChessPiece, pos: Pos) -> f32 {
        let color = piece.get_color().unwrap();
        let mult = if color == Color::White { 1.0 } else { -1.0 };
        if piece == ChessPiece::PawnWhite {
            return self.pawn_pos_value[pos.y as usize] + (self.center_pos_value[pos.x as usize] + self.center_pos_value[pos.y as usize]) / 4.0;
        }
        if piece == ChessPiece::PawnBlack {
            return -(self.center_pos_value[pos.x as usize] * self.center_pos_value[pos.y as usize]) * 10.0;
            // return -self.pawn_pos_value[7 - pos.y as usize] - (self.center_pos_value[pos.x as usize] + self.center_pos_value[pos.y as usize]) * 10.0;
        }
        mult
            * (self.center_pos_value[pos.x as usize] + self.center_pos_value[pos.y as usize])
            / 2.0
    }

    fn simple_eval(&self, board: &ChessBoardState) -> f32 {
        let mut eval = 0.0;
        for x in 0..BOARD_SIZE as i8 {
            for y in 0..BOARD_SIZE as i8 {
                let piece = board.get_piece_unsafe(Pos::from_coords(x, y));
                eval += self.get_piece_value(piece);
            }
        }
        eval
    }
}

#[cfg(test)]
mod tests {
    use::rust_chess::game::board::*;
    use super::*;
    #[test]
    fn test_eval_plus_time_diff() {
        let board =
            ChessBoardState::from_fen("r1b1kbnr/ppp3p1/2n5/1B1qppPp/3P3N/2N1B3/PPP2P1P/R2QK2R b Kq - 0 1")
                .unwrap();
        let mut eval = Evaluator::new();
        {
            let start = std::time::SystemTime::now();
            let _ = eval.evaluate(&board, 10);
            println!("Eval sync time {} {}", eval.low_level_eval_called ,start.elapsed().unwrap().as_millis());
        }
        {
            let start = std::time::SystemTime::now();
            let _ = eval.evaluate_async(&board, 10);
            println!("Eval async time {}", start.elapsed().unwrap().as_millis());
        }
        assert!(false);
    }
}