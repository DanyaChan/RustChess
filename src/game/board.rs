#![allow(dead_code)]

use std::usize; //todo Remove
pub const BOARD_SIZE: usize = 8;
pub const BOARD_ARRAY_SIZE: usize = BOARD_SIZE * BOARD_SIZE;
pub type MoveCode = u16; // array of 4 4bit numbers
pub type PosCode = u8;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ChessPiece {
    None = 0,
    PawnWhite,
    PawnBlack,
    RookWhite,
    RookBlack,
    KnightWhite,
    KnightBlack,
    BishopWhite,
    BishopBlack,
    KingWhite,
    KingBlack,
    QueenWhite,
    QueenBlack,
}

pub enum CastleStateFlag {
    WhiteLong = 1,
    WhiteShort = 1 << 1,
    BlackLong = 1 << 2,
    BlackShort = 1 << 3,
}

#[derive(PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq)]
pub struct Pos {
    pub x: u8,
    pub y: u8,
}

pub struct Move {
    from: Pos,
    to: Pos,
}

pub struct ChessBoardState {
    pub cur_move: Color,
    pub en_passant: PosCode,
    pub castle_state_flags: u8,
    pub board: [ChessPiece; BOARD_ARRAY_SIZE],
    pub cur_move_num: u16,
    pub cur_move_to_draw: u8,
}

impl Pos {
    pub fn new_from_str(pos_str: &str) -> Self {
        Pos {
            x: match pos_str.bytes().nth(0) {
                Some(x) => x - 'a' as u8,
                None => 0,
            },
            y: match pos_str.bytes().nth(1) {
                Some(x) => x - '1' as u8,
                None => 0,
            },
        }
    }

    pub fn get_code(&self) -> PosCode {
        (self.x & 0x0F) | (self.y << 4 & 0xF0)
    }
}

impl Move {
    pub fn new_from_str(move_str: &str) -> Self {
        Move {
            from: Pos::new_from_str(&move_str[0..2]),
            to: Pos::new_from_str(&move_str[3..5]),
        }
    }

    pub fn get_code(&self) -> MoveCode {
        self.from.get_code() as u16 | ((self.to.get_code() as u16) << 8)
    }
}

impl ChessPiece {
    pub fn get_piece_char(&self) -> char {
        match *self {
            ChessPiece::None => '.',
            ChessPiece::PawnWhite => 'P',
            ChessPiece::PawnBlack => 'p',
            ChessPiece::RookWhite => 'R',
            ChessPiece::RookBlack => 'r',
            ChessPiece::KnightWhite => 'N',
            ChessPiece::KnightBlack => 'n',
            ChessPiece::BishopWhite => 'B',
            ChessPiece::BishopBlack => 'b',
            ChessPiece::KingWhite => 'K',
            ChessPiece::KingBlack => 'k',
            ChessPiece::QueenWhite => 'Q',
            ChessPiece::QueenBlack => 'q',
        }
    }
    pub fn new_from_char(c: char) -> Self {
        match c {
            'P' => ChessPiece::PawnWhite,
            'p' => ChessPiece::PawnBlack,
            'R' => ChessPiece::RookWhite,
            'r' => ChessPiece::RookBlack,
            'N' => ChessPiece::KnightWhite,
            'n' => ChessPiece::KnightBlack,
            'B' => ChessPiece::BishopWhite,
            'b' => ChessPiece::BishopBlack,
            'K' => ChessPiece::KingWhite,
            'k' => ChessPiece::KingBlack,
            'Q' => ChessPiece::QueenWhite,
            'q' => ChessPiece::QueenBlack,
            _ => ChessPiece::None,
        }
    }
}

impl Color {
    pub fn get_name(&self) -> &'static str {
        if *self == Color::White {
            "White"
        } else {
            "Black"
        }
    }

    pub fn new_from_char(c: char) -> Option<Self> {
        match c {
            'w' => Some(Color::White),
            'W' => Some(Color::White),
            'b' => Some(Color::Black),
            'B' => Some(Color::Black),
            _ => None,
        }
    }
}

/*
Such types for MoveCode, CastleStateFlag are required for less memory use when copying state for eval
 */

impl ChessBoardState {
    pub fn debug_print(&self) {
        for i in 0..self.board.len() {
            if i % 8 == 0 {
                print!("\n");
            }
            print!("{}", self.board[i].get_piece_char())
        }
        print!(
            "\nMove {}, en passant TODO, castle TODO",
            self.cur_move.get_name()
        );
    }

    pub fn new() -> Self {
        ChessBoardState {
            cur_move: Color::White,
            en_passant: 0xFF,
            castle_state_flags: 0x00,
            board: [ChessPiece::None; BOARD_ARRAY_SIZE],
            cur_move_num: 1,
            cur_move_to_draw: 0,
        }
    }

    pub fn new_from_fen(fen_str: &str) -> Option<Self> {
        let mut res = Self::new();
        let r = res.load_fen(fen_str);
        if r == 0 {
            Some(res)
        } else {
            None
        }
    }

    fn parse_board(&mut self, fen_str: &str) -> usize {
        let mut cur_idx: usize = 0;
        for i in 0..fen_str.len() {
            let cur_byte = fen_str.bytes().nth(i);
            match cur_byte {
                None => {
                    return usize::MAX;
                }
                Some(x) => match x as char {
                    '/' => {}
                    '0'..='9' => cur_idx += (x - '0' as u8) as usize,
                    ' ' => {
                        return i;
                    }
                    _ => {
                        self.board[Self::get_fen_idx(cur_idx)] = ChessPiece::new_from_char(x as char);
                        cur_idx += 1;
                    }
                },
            }
        }
        return usize::MAX;
    }

    fn get_fen_idx(i: usize) -> usize {
        return 8 * (7 - i / 8) + i % 8;
    }

    // todo result error
    pub fn load_fen(&mut self, fen_str: &str) -> i8 {
        use scan_fmt::scan_fmt;
        let board_end = self.parse_board(fen_str);
        if board_end == usize::MAX {
            return 1;
        }
        let mut cur_parse = board_end + 1;
        if cur_parse >= fen_str.len() {
            return 2;
        }
        match Color::new_from_char(fen_str.bytes().nth(cur_parse).unwrap() as char) {
            None => {
                return 3;
            }
            Some(x) => {
                self.cur_move = x;
            }
        }
        cur_parse += 2;

        if cur_parse >= fen_str.len() {
            return 4;
        }

        for i in cur_parse..cur_parse + 5 {
            if i == fen_str.len() {
                return 5;
            }
            match fen_str.bytes().nth(i).unwrap() as char {
                'K' => {
                    self.castle_state_flags |= CastleStateFlag::WhiteShort as u8;
                }
                'k' => {
                    self.castle_state_flags |= CastleStateFlag::BlackShort as u8;
                }
                'Q' => {
                    self.castle_state_flags |= CastleStateFlag::WhiteLong as u8;
                }
                'q' => {
                    self.castle_state_flags |= CastleStateFlag::BlackLong as u8;
                }
                _ => {
                    cur_parse = i + 1;
                    break;
                }
            }
        }
        if cur_parse >= fen_str.len() {
            return 6;
        }
        if fen_str.chars().nth(cur_parse).unwrap() == '-' {
            cur_parse += 2;
        } else {
            self.en_passant = Pos::new_from_str(&fen_str[cur_parse..cur_parse+2]).get_code();
            cur_parse += 3;
        }
        if cur_parse >= fen_str.len() {
            return 7;
        }

        let res = scan_fmt!(&fen_str[cur_parse..], "{} {}", u8, u16);
        if res.is_err() {
            return 8;
        }

        let ok_res = res.unwrap();
        self.cur_move_num = ok_res.1;
        self.cur_move_to_draw = ok_res.0;
        return 0;
    }
}
