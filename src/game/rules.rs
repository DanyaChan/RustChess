// use super::board::*;

// struct Promotion {
//     pub mv : Move,
//     pub piece : ChessPiece
// }

// pub enum ChessMove {
//     Move(Move),
//     EnPassant,
//     CastleLong,
//     CastleShort,
//     Promotion(Promotion)
// }

// // returns vel of all moves ignoring color, check and castle rules
// pub fn get_all_moves(_board: ChessBoardState, _piece_pos: Pos) -> Vec<ChessMove> {
//     return vec![];
// }

// pub fn get_pawn_moves(_board: ChessBoardState, _piece_pos: &Pos) -> Vec<ChessMove> {
//     let res = vec![];

//     // let color = board.get_piece_unsafe(&piece_pos).get_color();

//     // if color.unwrap() == Color::White {
//     //     let to1 = Pos {
//     //         x: piece_pos.x,
//     //         y: piece_pos.y + 1,
//     //     };
//     //     match board.get_piece(&to1) {
//     //         None => {
//     //             res.push(to1);
//     //         }
//     //         _ => {}
//     //     }
//     //     if piece_pos.y == 1 {
//     //         let to2 = Pos {
//     //             x: piece_pos.x,
//     //             y: piece_pos.y + 2,
//     //         };
//     //         match board.get_piece(&to2) {
//     //             None => {
//     //                 res.push(to2);
//     //             }
//     //             _ => {}
//     //         }
//     //     }

//     // } else {
//     // }

//     return res;
// }
