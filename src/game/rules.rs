// TODO REMOVE
use super::board::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChessMoveType {
    Simple,
    EnPassant,
    CastleLong,
    CastleShort,
    Promotion(ChessPiece),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ChessMove {
    pub mv: Move,
    pub move_type: ChessMoveType,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MoveResult {
    pub new: ChessPiece,
    pub remove: ChessPiece,
}

impl ChessMove {
    pub fn get_move_string(&self) -> String {
        return match self.move_type {
            ChessMoveType::Simple => self.mv.get_str(),
            ChessMoveType::EnPassant => self.mv.get_str() + "e",
            ChessMoveType::CastleLong => "0-0-0".to_string(),
            ChessMoveType::CastleShort => "0-0".to_string(),
            ChessMoveType::Promotion(x) => {
                self.mv.get_str() + &(x.get_piece_u8() as char).to_string()
            }
        };
    }
}

impl MoveResult {
    pub fn capture(p: ChessPiece) -> Self {
        MoveResult {
            new: ChessPiece::None,
            remove: p,
        }
    }
}

impl ChessBoardState {
    fn count_move(&mut self, pawn_move: bool, capture: bool) {
        self.halfmoves_to_draw = if pawn_move || capture {
            0
        } else {
            self.halfmoves_to_draw + 1
        };

        self.move_num += if self.turn == Color::Black { 1 } else { 0 };
        self.turn = if self.turn == Color::White {
            Color::Black
        } else {
            Color::White
        };
        self.en_passant = 0xFF;
    }

    fn update_castle_flags(&mut self, mv: Move) {
        // because once moved from here, it means that king or rook move at least once
        if mv.from.y == 0 && (mv.from.x == 0 || mv.from.x == 4) {
            self.castle_state_flags &= !(CastleStateFlag::WhiteLong as u8);
        }
        if mv.from.y == 0 && (mv.from.x == 7 || mv.from.x == 4) {
            self.castle_state_flags &= !(CastleStateFlag::WhiteShort as u8);
        }
        if mv.from.y == 7 && (mv.from.x == 0 || mv.from.x == 4) {
            self.castle_state_flags &= !(CastleStateFlag::BlackLong as u8);
        }
        if mv.from.y == 7 && (mv.from.x == 7 || mv.from.x == 4) {
            self.castle_state_flags &= !(CastleStateFlag::BlackShort as u8);
        }
    }

    // dont check if move follow the rules and if in board
    fn make_simple_move_force(&mut self, mv: Move) -> MoveResult {
        let captured = self.get_piece_unsafe(mv.to);
        let capture = captured != ChessPiece::None;
        let pawn_move = self.get_piece_unsafe(mv.from) == ChessPiece::PawnBlack
            || self.get_piece_unsafe(mv.from) == ChessPiece::PawnWhite;
        self.count_move(pawn_move, capture);

        self.set_piece_unsafe(mv.to, self.get_piece_unsafe(mv.from));
        self.set_piece_unsafe(mv.from, ChessPiece::None);
        if pawn_move && mv.from.y == 1 && mv.to.y == 3 {
            self.en_passant = Pos { x: mv.from.x, y: 2 }.get_code();
        }
        if pawn_move && mv.from.y == 6 && mv.to.y == 4 {
            self.en_passant = Pos { x: mv.from.x, y: 5 }.get_code();
        }
        self.update_castle_flags(mv);
        MoveResult::capture(captured)
    }

    fn apply_en_passant(&mut self, mv: Move) -> MoveResult {
        let color = self.get_piece_unsafe(mv.from).get_color().unwrap();
        let mut pos = Pos::new_from_code(self.en_passant);
        if color == Color::White {
            pos.y -= 1;
        } else {
            pos.y += 1;
        }
        let captured = self.get_piece_unsafe(pos);
        self.set_piece_unsafe(pos, ChessPiece::None);
        self.make_simple_move_force(mv);
        MoveResult::capture(captured)
    }

    fn apply_castle(&mut self, mv: Move, move_type: ChessMoveType) -> MoveResult {
        let color = self.get_piece_unsafe(mv.from).get_color().unwrap();
        self.make_simple_move_force(mv);
        let to;
        let from;
        match (color, move_type) {
            (Color::White, ChessMoveType::CastleShort) => {
                from = Pos::new_from_coords(7, 0);
                to = Pos::new_from_coords(5, 0);
            }
            (Color::White, ChessMoveType::CastleLong) => {
                from = Pos::new_from_coords(0, 0);
                to = Pos::new_from_coords(3, 0);
            }
            (Color::Black, ChessMoveType::CastleShort) => {
                from = Pos::new_from_coords(7, 7);
                to = Pos::new_from_coords(5, 7);
            }
            (Color::Black, ChessMoveType::CastleLong) => {
                from = Pos::new_from_coords(0, 7);
                to = Pos::new_from_coords(3, 7);
            }
            _ => {
                panic!("Logic err");
            }
        }
        self.set_piece_unsafe(to, self.get_piece_unsafe(from));
        self.set_piece_unsafe(from, ChessPiece::None);
        return MoveResult {
            new: ChessPiece::None,
            remove: ChessPiece::None,
        };
    }

    fn apply_promotion(&mut self, mv: Move, promotion: ChessPiece) -> MoveResult {
        let res = self.make_simple_move_force(mv);
        self.set_piece_unsafe(mv.to, promotion);
        return MoveResult {
            new: promotion,
            remove: res.remove,
        };
    }

    fn apply_move_force(&mut self, mv: ChessMove) -> MoveResult {
        match mv.move_type {
            ChessMoveType::Simple => self.make_simple_move_force(mv.mv),
            ChessMoveType::EnPassant => self.apply_en_passant(mv.mv),
            ChessMoveType::Promotion(x) => self.apply_promotion(mv.mv, x),
            ChessMoveType::CastleLong | ChessMoveType::CastleShort => {
                self.apply_castle(mv.mv, mv.move_type)
            }
        }
    }

    pub fn get_new_pos_after_move(&self, mv: ChessMove) -> ChessBoardState {
        let mut new_board = *self;
        new_board.apply_move_force(mv);
        return new_board;
    }
    pub fn get_new_pos_after_move_for_eval(&self, mv: ChessMove) -> (ChessBoardState, MoveResult) {
        let mut new_board = *self;
        let res = new_board.apply_move_force(mv);
        return (new_board, res);
    }

    // move check

    // does not exclude moves that leave king open
    pub fn get_all_moves(&self) -> Vec<ChessMove> {
        let mut result = vec![];

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let pos = Pos {
                    x: x as u8,
                    y: y as u8,
                };
                let color = self.get_piece_unsafe(pos).get_color();
                if color.is_some() && color.unwrap() == self.turn {
                    result.append(&mut self.get_all_moves_from_pos(pos));
                }
            }
        }

        return result;
    }

    // Filter moves that leave king open to attack
    pub fn get_all_moves_checked(&self) -> Vec<ChessMove> {
        let mut result = vec![];

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let pos = Pos {
                    x: x as u8,
                    y: y as u8,
                };
                let color = self.get_piece_unsafe(pos).get_color();
                if color.is_some() && color.unwrap() == self.turn {
                    result.append(&mut self.get_all_moves_from_pos_filtered(pos));
                }
            }
        }

        return result;
    }

    pub fn get_all_moves_from_pos_filtered(&self, from: Pos) -> Vec<ChessMove> {
        let result = self.get_all_moves_from_pos(from);

        return result
            .into_iter()
            .filter(|mv| !self.is_move_allowed_by_rules(*mv))
            .collect();
    }

    pub fn is_move_allowed_by_rules(&self, mv: ChessMove) -> bool {
        return self.get_new_pos_after_move(mv).get_king_attacked(self.turn);
    }

    pub fn get_king_attacked(&self, color: Color) -> bool {
        let pos = self.get_king_pos(color);
        return self.get_pos_attacked(pos, color);
    }

    // when cheking if smth is attacked color always means who is attacked
    pub fn check_attacked_direction(
        &self,
        from: Pos,
        step_x: i8,
        step_y: i8,
        step_num: usize,
        pieces: &[ChessPiece],
    ) -> bool {
        let mut x = from.x as i8 + step_x;
        let mut y = from.y as i8 + step_y;
        for _ in 0..step_num {
            if !Self::coords_in_bounds(x, y) {
                break;
            }
            let to = Pos::new_from_coords(x, y);
            let piece = self.get_piece_unsafe(to);
            if pieces.contains(&piece) {
                return true;
            } else if piece != ChessPiece::None {
                return false;
            }
            x += step_x;
            y += step_y;
        }
        return false;
    }

    pub fn get_pos_attacked(&self, from: Pos, color: Color) -> bool {
        let rook_dir = if color == Color::White {
            [ChessPiece::RookBlack, ChessPiece::QueenBlack]
        } else {
            [ChessPiece::RookWhite, ChessPiece::QueenWhite]
        };
        let bishop_dir = if color == Color::White {
            [ChessPiece::BishopBlack, ChessPiece::QueenBlack]
        } else {
            [ChessPiece::BishopWhite, ChessPiece::QueenWhite]
        };
        let pawn = if color == Color::White {
            [ChessPiece::PawnBlack]
        } else {
            [ChessPiece::PawnWhite]
        };
        let knight = if color == Color::White {
            [ChessPiece::KnightBlack]
        } else {
            [ChessPiece::KnightWhite]
        };
        let king = if color == Color::White {
            [ChessPiece::KingBlack]
        } else {
            [ChessPiece::KingWhite]
        };
        return self.check_attacked_direction(from, 0, 1, 8, &rook_dir)
            || self.check_attacked_direction(from, 0, -1, 8, &rook_dir)
            || self.check_attacked_direction(from, -1, 0, 8, &rook_dir)
            || self.check_attacked_direction(from, 1, 0, 8, &rook_dir)
            ////
            || self.check_attacked_direction(from, 1, 1, 8, &bishop_dir)
            || self.check_attacked_direction(from, 1, -1, 8, &bishop_dir)
            || self.check_attacked_direction(from, -1, 1, 8, &bishop_dir)
            || self.check_attacked_direction(from, 1, 1, 8, &bishop_dir)
            ////
            || self.check_attacked_direction(from, 1, 0, 1, &king)
            || self.check_attacked_direction(from, -1, 0, 1, &king)
            || self.check_attacked_direction(from, 0, 1, 1, &king)
            || self.check_attacked_direction(from, 0, -1, 1, &king)
            || self.check_attacked_direction(from, 1, 1, 1, &king)
            || self.check_attacked_direction(from, 1, -1, 1, &king)
            || self.check_attacked_direction(from, -1, 1, 1, &king)
            || self.check_attacked_direction(from, -1, -1, 1, &king)
            //// pawns
            || if color == Color::White {
                self.check_attacked_direction(from, 1, 1, 1, &pawn) ||
                self.check_attacked_direction(from, -1, 1, 1, &pawn)
            } else {
                self.check_attacked_direction(from, 1, -1, 1, &pawn) ||
                self.check_attacked_direction(from, -1, -1, 1, &pawn)
            }
            //// knights
            || self.check_attacked_direction(from, 1, 2, 1, &knight)
            || self.check_attacked_direction(from, -1, 2, 1, &knight)
            || self.check_attacked_direction(from, 1, -2, 1, &knight)
            || self.check_attacked_direction(from, -1, -2, 1, &knight)
            || self.check_attacked_direction(from, 2, 1, 1, &knight)
            || self.check_attacked_direction(from, 2, -1, 1, &knight)
            || self.check_attacked_direction(from, -2, 1, 1, &knight)
            || self.check_attacked_direction(from, -2, -1, 1, &knight);
    }

    pub fn get_king_pos(&self, color: Color) -> Pos {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if color == Color::White
                    && self.get_piece_coords_unsafe(x, y) == ChessPiece::KingWhite
                {
                    return Pos::new_from_coords(x as i8, y as i8);
                }
            }
        }
        return Pos::new_from_code(0xFF);
    }

    pub fn is_legal_move(&self, mv: ChessMove) -> bool {
        let piece_opt = self.get_piece(mv.mv.from);
        if piece_opt.is_none() {
            return false;
        }
        let piece_color = piece_opt.unwrap().get_color();
        if piece_color.is_none() || piece_color.unwrap() != self.turn {
            return false;
        }
        let moves = self.get_all_moves_from_pos_filtered(mv.mv.from);
        return moves.iter().find(|m| **m == mv).is_some();
    }

    // get moves

    pub fn get_all_moves_from_pos(&self, from: Pos) -> Vec<ChessMove> {
        let mut res = vec![];

        match self.get_piece_unsafe(from) {
            ChessPiece::None => {}
            ChessPiece::RookBlack | ChessPiece::RookWhite => self.add_rook_moves(from, &mut res),
            ChessPiece::KnightBlack | ChessPiece::KnightWhite => {
                self.add_knight_moves(from, &mut res)
            }
            ChessPiece::BishopBlack | ChessPiece::BishopWhite => {
                self.add_bishop_moves(from, &mut res)
            }
            ChessPiece::QueenBlack | ChessPiece::QueenWhite => self.add_queen_moves(from, &mut res),
            ChessPiece::KingBlack | ChessPiece::KingWhite => self.add_king_moves(from, &mut res),
            ChessPiece::PawnBlack | ChessPiece::PawnWhite => self.add_pawn_moves(from, &mut res),
        }

        return res;
    }

    // utils

    fn add_rook_moves(&self, from: Pos, res: &mut Vec<ChessMove>) {
        self.add_moves_in_direction(from, 1, 0, 8, res);
        self.add_moves_in_direction(from, 0, 1, 8, res);
        self.add_moves_in_direction(from, -1, 0, 8, res);
        self.add_moves_in_direction(from, 0, -1, 8, res);
    }
    fn add_knight_moves(&self, from: Pos, res: &mut Vec<ChessMove>) {
        self.add_moves_in_direction(from, 2, 1, 1, res);
        self.add_moves_in_direction(from, 1, 2, 1, res);
        self.add_moves_in_direction(from, -2, 1, 1, res);
        self.add_moves_in_direction(from, -1, 2, 1, res);
        self.add_moves_in_direction(from, 2, -1, 1, res);
        self.add_moves_in_direction(from, 1, -2, 1, res);
        self.add_moves_in_direction(from, -2, -1, 1, res);
        self.add_moves_in_direction(from, -1, -2, 1, res);
    }
    fn add_bishop_moves(&self, from: Pos, res: &mut Vec<ChessMove>) {
        self.add_moves_in_direction(from, 1, 1, 8, res);
        self.add_moves_in_direction(from, -1, -1, 8, res);
        self.add_moves_in_direction(from, -1, 1, 8, res);
        self.add_moves_in_direction(from, 1, -1, 8, res);
    }
    fn add_queen_moves(&self, from: Pos, res: &mut Vec<ChessMove>) {
        self.add_bishop_moves(from, res);
        self.add_rook_moves(from, res);
    }

    fn check_castle(&self, from: Pos, dir: i8) -> bool {
        if self.get_piece_coords_i8_unsafe(from.x as i8 + dir, from.y as i8) != ChessPiece::None {
            return false;
        }
        if self.get_piece_coords_i8_unsafe(from.x as i8 + 2 * dir, from.y as i8) != ChessPiece::None
        {
            return false;
        }
        if dir == -1
            && self.get_piece_coords_i8_unsafe(from.x as i8 + 3 * dir, from.y as i8)
                != ChessPiece::None
        {
            return false;
        }
        if self.get_pos_attacked(
            Pos::new_from_coords(from.x as i8 + dir, from.y as i8),
            self.get_piece_unsafe(from).get_color().unwrap(),
        ) {
            return false;
        }

        return true;
    }

    fn add_king_moves(&self, from: Pos, res: &mut Vec<ChessMove>) {
        self.add_moves_in_direction(from, 1, 1, 1, res);
        self.add_moves_in_direction(from, -1, 1, 1, res);
        self.add_moves_in_direction(from, 1, -1, 1, res);
        self.add_moves_in_direction(from, -1, -1, 1, res);
        self.add_moves_in_direction(from, 1, 0, 1, res);
        self.add_moves_in_direction(from, -1, 0, 1, res);
        self.add_moves_in_direction(from, 0, 1, 1, res);
        self.add_moves_in_direction(from, 0, -1, 1, res);
        // TODO castle
        let color: Color = self.get_piece_unsafe(from).get_color().unwrap();
        if color == Color::White
            && (self.castle_state_flags & (CastleStateFlag::WhiteShort as u8) != 0)
            && self.check_castle(from, 1)
        {
            res.push(ChessMove {
                mv: Move {
                    from: from,
                    to: Pos {
                        x: from.x + 2,
                        y: from.y,
                    },
                },
                move_type: ChessMoveType::CastleShort,
            });
        }
        if color == Color::White
            && (self.castle_state_flags & (CastleStateFlag::WhiteLong as u8) != 0)
            && self.check_castle(from, -1)
        {
            res.push(ChessMove {
                mv: Move {
                    from: from,
                    to: Pos {
                        x: from.x + 2,
                        y: from.y,
                    },
                },
                move_type: ChessMoveType::CastleLong,
            });
        }
        if color == Color::Black
            && (self.castle_state_flags & (CastleStateFlag::BlackShort as u8) != 0)
            && self.check_castle(from, 1)
        {
            res.push(ChessMove {
                mv: Move {
                    from: from,
                    to: Pos {
                        x: from.x + 2,
                        y: from.y,
                    },
                },
                move_type: ChessMoveType::CastleShort,
            });
        }
        if color == Color::Black
            && (self.castle_state_flags & (CastleStateFlag::BlackLong as u8) != 0)
            && self.check_castle(from, -1)
        {
            res.push(ChessMove {
                mv: Move {
                    from: from,
                    to: Pos {
                        x: from.x + 2,
                        y: from.y,
                    },
                },
                move_type: ChessMoveType::CastleLong,
            });
        }
    }
    fn add_pawn_moves(&self, from: Pos, res: &mut Vec<ChessMove>) {
        let mut result = vec![];
        let piece_color = self.get_piece_unsafe(from).get_color().unwrap();
        let y_dir = if piece_color == Color::White { 1 } else { -1 };

        let step_num = if piece_color == Color::White && from.y == 1
            || piece_color == Color::Black && from.y == 6
        {
            2
        } else {
            1
        };

        self.add_moves_in_direction_general(from, 0, y_dir, step_num, false, false, &mut result);
        self.add_moves_in_direction_general(from, 1, y_dir, 1, true, true, &mut result);
        self.add_moves_in_direction_general(from, -1, y_dir, 1, true, true, &mut result);

        let en_passant = Pos::new_from_code(self.en_passant);

        if (en_passant.y as i8 - from.y as i8) == y_dir
            && [-1 as i8, 1 as i8].contains(&(en_passant.x as i8 - from.x as i8))
        {
            self.add_moves_in_direction_general(
                from,
                en_passant.x as i8 - from.x as i8,
                y_dir,
                1,
                false,
                false,
                &mut result,
            );
            if result.len() > 0 {
                result.last_mut().unwrap().move_type = ChessMoveType::EnPassant;
                // костыль мб както подругому TODO вернуть сколько ходов добавил
            }
        }

        if result.len() > 0 && piece_color == Color::White && from.y == 6
            || result.len() > 0 && piece_color == Color::Black && from.y == 1
        {
            let proms = if piece_color == Color::White {
                [
                    ChessPiece::RookWhite,
                    ChessPiece::QueenWhite,
                    ChessPiece::BishopWhite,
                    ChessPiece::KnightWhite,
                ]
            } else {
                [
                    ChessPiece::RookBlack,
                    ChessPiece::QueenBlack,
                    ChessPiece::BishopBlack,
                    ChessPiece::KnightBlack,
                ]
            };
            for mv in result {
                for prom_piece in proms {
                    res.push(ChessMove {
                        mv: mv.mv,
                        move_type: ChessMoveType::Promotion(prom_piece),
                    });
                }
            }
        } else {
            for mv in result {
                res.push(mv);
            }
        }
    }

    fn add_moves_in_direction_general(
        &self,
        from: Pos,
        step_x: i8,
        step_y: i8,
        step_num: usize,
        can_take: bool,
        must_take: bool,
        res: &mut Vec<ChessMove>,
    ) {
        let color = self.get_piece_unsafe(from).get_color().unwrap();
        let mut x = from.x as i8 + step_x;
        let mut y = from.y as i8 + step_y;
        for _ in 0..step_num {
            if !Self::coords_in_bounds(x, y) {
                break;
            }
            let to = Pos::new_from_coords(x, y);
            let piece_to_color = self.get_piece_unsafe(to).get_color();
            if piece_to_color.is_some_and(|x| x == color) {
                break;
            }
            if must_take && piece_to_color.is_none() {
                break;
            }
            if !can_take && piece_to_color.is_some() {
                break;
            }
            res.push(ChessMove {
                mv: Move { from: from, to: to },
                move_type: ChessMoveType::Simple,
            });
            if piece_to_color.is_some() {
                break;
            }
            x += step_x;
            y += step_y;
        }
    }

    fn add_moves_in_direction(
        &self,
        from: Pos,
        step_x: i8,
        step_y: i8,
        step_num: usize,
        res: &mut Vec<ChessMove>,
    ) {
        self.add_moves_in_direction_general(from, step_x, step_y, step_num, true, false, res);
    }

    pub fn get_move_string(&self, mv: ChessMove) -> String {
        let piece = self.get_piece_unsafe(mv.mv.from);
        return match mv.move_type {
            ChessMoveType::CastleLong => "0-0-0".to_string(),
            ChessMoveType::CastleShort => "0-0".to_string(),
            _ => {
                (piece.get_piece_u8().to_ascii_uppercase() as char).to_string()
                    + &mv.get_move_string()
            }
        };
    }

    pub fn get_chess_move_from_string(&self, move_str: &str) -> ChessMove {
        if move_str == "0-0" && self.turn == Color::White {
            return ChessMove {
                mv: Move {
                    from: Pos { x: 4, y: 0 },
                    to: Pos { x: 6, y: 0 },
                },
                move_type: ChessMoveType::CastleShort,
            };
        }
        if move_str == "0-0" && self.turn == Color::Black {
            return ChessMove {
                mv: Move {
                    from: Pos { x: 4, y: 7 },
                    to: Pos { x: 6, y: 7 },
                },
                move_type: ChessMoveType::CastleShort,
            };
        }
        if move_str == "0-0-0" && self.turn == Color::White {
            return ChessMove {
                mv: Move {
                    from: Pos { x: 4, y: 0 },
                    to: Pos { x: 2, y: 0 },
                },
                move_type: ChessMoveType::CastleLong,
            };
        }
        if move_str == "0-0-0" && self.turn == Color::Black {
            return ChessMove {
                mv: Move {
                    from: Pos { x: 4, y: 7 },
                    to: Pos { x: 2, y: 7 },
                },
                move_type: ChessMoveType::CastleLong,
            };
        }
        let mv = Move::new_from_str(move_str);
        let piece = self.get_piece_unsafe(mv.from);
        if piece == ChessPiece::PawnWhite || piece == ChessPiece::PawnBlack {
            if mv.to.get_code() == self.en_passant {
                return ChessMove {
                    mv: mv,
                    move_type: ChessMoveType::EnPassant,
                };
            }
            if move_str.len() == 6 {
                return ChessMove {
                    mv: mv,
                    move_type: ChessMoveType::Promotion(ChessPiece::new_from_u8(
                        *move_str.as_bytes().last().unwrap(),
                    )),
                };
            }
        }
        return ChessMove {
            mv: mv,
            move_type: ChessMoveType::Simple,
        };
    }
}
