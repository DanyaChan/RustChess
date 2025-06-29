#![allow(dead_code)] // TODO REMOVE
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
    mv: Move,
    move_type: ChessMoveType,
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
    fn make_simple_move_force(&mut self, mv: Move) {
        let from = Self::get_pos_idx(mv.from);
        let to = Self::get_pos_idx(mv.to);

        let capture = self.board[to] != ChessPiece::None;
        let pawn_move =
            self.board[from] == ChessPiece::PawnBlack || self.board[from] == ChessPiece::PawnWhite;
        self.count_move(pawn_move, capture);

        self.board[to] = self.board[from];
        if pawn_move && mv.from.y == 1 && mv.to.y == 3 {
            self.en_passant = Pos { x: mv.from.x, y: 2 }.get_code();
        }
        if pawn_move && mv.from.y == 6 && mv.to.y == 4 {
            self.en_passant = Pos { x: mv.from.x, y: 5 }.get_code();
        }
        self.update_castle_flags(mv);
    }

    fn apply_en_passant(&mut self, mv: Move) {
        let color = self.get_piece_unsafe(mv.from).get_color().unwrap();
        let mut pos = Pos::new_from_code(self.en_passant);
        if color == Color::White { pos.y -= 1; } else {pos.y += 1; }
        self.board[Self::get_pos_idx(pos)] = ChessPiece::None;
        self.make_simple_move_force(mv);
    }

    fn apply_castle(&mut self, mv: Move, move_type: ChessMoveType) {
        let color = self.get_piece_unsafe(mv.from).get_color().unwrap();
        let to;
        let from;
        match (color, move_type) {
            (Color::White, ChessMoveType::CastleShort) => {
                from = Self::get_pos_idx(Pos::new_from_coords(7, 0));
                to = Self::get_pos_idx(Pos::new_from_coords(5, 0));
            }
            (Color::White, ChessMoveType::CastleLong) => {
                from = Self::get_pos_idx(Pos::new_from_coords(0, 0));
                to = Self::get_pos_idx(Pos::new_from_coords(3, 0));
            }
            (Color::Black, ChessMoveType::CastleShort) => {
                from = Self::get_pos_idx(Pos::new_from_coords(7, 7));
                to = Self::get_pos_idx(Pos::new_from_coords(5, 70));
            }
            (Color::Black, ChessMoveType::CastleLong) => {
                from = Self::get_pos_idx(Pos::new_from_coords(0, 7));
                to = Self::get_pos_idx(Pos::new_from_coords(3, 7));
            }
            _ => {
                panic!("Logic err");
            }
        }

        self.board[to] = self.board[from];
        self.make_simple_move_force(mv);
    }

    fn apply_promotion(&mut self, mv: Move, promotion: ChessPiece) {
        self.make_simple_move_force(mv);
        self.board[Self::get_pos_idx(mv.to)] = promotion; 
    }

    fn apply_move_force(&mut self, mv: ChessMove) {
        match mv.move_type {
            ChessMoveType::Simple => self.make_simple_move_force(mv.mv),
            ChessMoveType::EnPassant => self.apply_en_passant(mv.mv),
            ChessMoveType::Promotion(x) => self.apply_promotion(mv.mv, x),
            ChessMoveType::CastleLong | ChessMoveType::CastleShort => self.apply_castle(mv.mv, mv.move_type),
        }
    }

    pub fn get_new_pos_after_move(&self, mv: ChessMove) -> ChessBoardState {
        let mut new_board = *self;
        new_board.apply_move_force(mv);
        return new_board;
    }

    // move check

    // does not exclude moves that leave king open
    pub fn get_all_moves(&self) -> Vec<ChessMove> {
        vec![]
    }

    // Filter moves that leave king open to attack
    pub fn get_all_moves_checked() -> Vec<ChessMove> {
        vec![]
    }

    pub fn get_all_moves_from_pos_filtered(&self, from: Pos) -> Vec<ChessMove> {
        vec![]
    }

    pub fn get_king_attacked(&self, color: Color) -> bool {
        return false;
    }

    pub fn get_pos_attacked(&self, color: Color) -> bool {
        return false;
    }

    pub fn get_king_pos(&self, color: Color) -> Pos {
        return Pos { x: 0, y: 0 };
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
                self.add_knight_moves(from, &mut res)
            }
            ChessPiece::QueenBlack | ChessPiece::QueenWhite => self.add_pawn_moves(from, &mut res),
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
        self.add_moves_in_direction(from, -1, 1, 8, res);
        self.add_moves_in_direction(from, -1, -1, 8, res);
    }
    fn add_queen_moves(&self, from: Pos, res: &mut Vec<ChessMove>) {
        self.add_bishop_moves(from, res);
        self.add_rook_moves(from, res);
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
    }
    fn add_pawn_moves(&self, from: Pos, res: &mut Vec<ChessMove>) {
        let piece_color = self.get_piece_unsafe(from).get_color().unwrap();
        let y_dir = if piece_color == Color::White { 1 } else { -1 };

        let step_num = if piece_color == Color::White && from.y == 1
            || piece_color == Color::Black && from.y == 6
        {
            2
        } else {
            1
        };

        self.add_moves_in_direction_general(from, 0, y_dir, step_num, false, false, res);
        self.add_moves_in_direction_general(from, 1, y_dir, 1, true, true, res);
        self.add_moves_in_direction_general(from, -1, y_dir, 1, true, true, res);

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
                res,
            );
            res.last_mut().unwrap().move_type = ChessMoveType::EnPassant; // костыль мб както подругому
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
            ChessMoveType::Simple => {
                (piece.get_piece_u8().to_ascii_uppercase() as char).to_string() + &mv.mv.get_str()
            }
            ChessMoveType::EnPassant => {
                (piece.get_piece_u8().to_ascii_uppercase() as char).to_string()
                    + &mv.mv.get_str()
                    + "e"
            }
            ChessMoveType::CastleLong => "0-0-0".to_string(),
            ChessMoveType::CastleShort => "0-0".to_string(),
            ChessMoveType::Promotion(x) => {
                (piece.get_piece_u8().to_ascii_uppercase() as char).to_string()
                    + &mv.mv.get_str()
                    + &x.get_piece_u8().to_string()
            }
        };
    }
}
