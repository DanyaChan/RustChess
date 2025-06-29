#![allow(dead_code)]

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pos {
    pub x: u8,
    pub y: u8,
}

pub struct Move {
    pub from: Pos,
    pub to: Pos,
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
                Some(x) => x - b'a' as u8,
                None => 0,
            },
            y: match pos_str.bytes().nth(1) {
                Some(x) => x - b'1' as u8,
                None => 0,
            },
        }
    }

    pub fn get_code(&self) -> PosCode {
        (self.x & 0x0F) | (self.y << 4 & 0xF0)
    }

    pub fn get_str(&self) -> String {
        if self.x == 0xF && self.y == 0xF {
            return "-".to_string();
        }
        return String::from_utf8([self.x + b'a', self.y + b'1'].to_vec()).unwrap();
    }

    pub fn new_from_code(code: PosCode) -> Self {
        return Pos {
            x: code & 0x0F,
            y: (code & 0xF0) >> 4,
        };
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
    pub fn get_piece_u8(&self) -> u8 {
        match *self {
            ChessPiece::None => b'.',
            ChessPiece::PawnWhite => b'P',
            ChessPiece::PawnBlack => b'p',
            ChessPiece::RookWhite => b'R',
            ChessPiece::RookBlack => b'r',
            ChessPiece::KnightWhite => b'N',
            ChessPiece::KnightBlack => b'n',
            ChessPiece::BishopWhite => b'B',
            ChessPiece::BishopBlack => b'b',
            ChessPiece::KingWhite => b'K',
            ChessPiece::KingBlack => b'k',
            ChessPiece::QueenWhite => b'Q',
            ChessPiece::QueenBlack => b'q',
        }
    }
    pub fn new_from_u8(c: u8) -> Self {
        match c {
            b'P' => ChessPiece::PawnWhite,
            b'p' => ChessPiece::PawnBlack,
            b'R' => ChessPiece::RookWhite,
            b'r' => ChessPiece::RookBlack,
            b'N' => ChessPiece::KnightWhite,
            b'n' => ChessPiece::KnightBlack,
            b'B' => ChessPiece::BishopWhite,
            b'b' => ChessPiece::BishopBlack,
            b'K' => ChessPiece::KingWhite,
            b'k' => ChessPiece::KingBlack,
            b'Q' => ChessPiece::QueenWhite,
            b'q' => ChessPiece::QueenBlack,
            _ => ChessPiece::None,
        }
    }
    pub fn get_color(&self) -> Option<Color> {
        match *self {
            Self::None => None,
            Self::PawnWhite => Some(Color::White),
            Self::PawnBlack => Some(Color::Black),
            Self::RookWhite => Some(Color::White),
            Self::RookBlack => Some(Color::Black),
            Self::KnightWhite => Some(Color::White),
            Self::KnightBlack => Some(Color::Black),
            Self::BishopWhite => Some(Color::White),
            Self::BishopBlack => Some(Color::Black),
            Self::KingWhite => Some(Color::White),
            Self::KingBlack => Some(Color::Black),
            Self::QueenWhite => Some(Color::White),
            Self::QueenBlack => Some(Color::Black),
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

    pub fn new_from_u8(c: u8) -> Option<Self> {
        match c {
            b'w' => Some(Color::White),
            b'W' => Some(Color::White),
            b'b' => Some(Color::Black),
            b'B' => Some(Color::Black),
            _ => None,
        }
    }
}

/*
Such types for MoveCode, CastleStateFlag are required for less memory use when copying state for eval
 */

impl ChessBoardState {
    pub fn get_castle_state_str(&self) -> String {

        if self.castle_state_flags == 0 {
            return "-".to_string();
        }

        let mut res = vec![];

        if (&self.castle_state_flags & CastleStateFlag::WhiteShort as u8) != 0 {
            res.push(b'K');
        }
        if (&self.castle_state_flags & CastleStateFlag::BlackShort as u8) != 0 {
            res.push(b'k');
        }
        if (&self.castle_state_flags & CastleStateFlag::WhiteLong as u8) != 0 {
            res.push(b'Q');
        }
        if (&self.castle_state_flags & CastleStateFlag::BlackLong as u8) != 0 {
            res.push(b'q');
        }

        return String::from_utf8(res).unwrap();
    }

    pub fn get_piece_unsafe(&self, pos: &Pos) -> ChessPiece {
        return self.board[(pos.y as usize) * BOARD_SIZE + pos.x as usize];
    }

    pub fn get_piece(&self, pos: &Pos) -> Option<ChessPiece> {
        if pos.x >= BOARD_SIZE as u8 || pos.y >= BOARD_SIZE as u8 {
            return None;
        }
        return Some(self.board[(pos.y as usize) * BOARD_SIZE + pos.x as usize]);
    }


    pub fn debug_print(&self) {
        for i in 0..self.board.len() {
            if i % 8 == 0 {
                print!("\n");
            }
            print!("{}", self.board[Self::get_display_idx(i)].get_piece_u8() as char)
        }
        print!(
            "\nMove {}, en passant {}, castle {}\n",
            self.cur_move.get_name(),
            Pos::new_from_code(self.en_passant).get_str(),
            self.get_castle_state_str()
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
        let r = res.parse_fen(fen_str);
        if r.is_ok() {
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
                Some(x) => match x {
                    b'/' => {}
                    b'0'..=b'9' => cur_idx += (x - b'0') as usize,
                    b' ' => {
                        return i;
                    }
                    _ => {
                        self.board[Self::get_display_idx(cur_idx)] =
                            ChessPiece::new_from_u8(x);
                        cur_idx += 1;
                    }
                },
            }
        }
        return usize::MAX;
    }

    fn get_display_idx(i: usize) -> usize {
        return 8 * (7 - i / 8) + i % 8;
    }

    pub fn parse_fen(&mut self, fen_str: &str) -> Result<(), String> {
        use scan_fmt::scan_fmt;
        let parsed_str_opt = scan_fmt!(fen_str, "{}{}{}{}{}{}", String, char, String, String, u8, u16);
        if parsed_str_opt.is_err() {
            return Err("Wrong format".to_string());
        }
        let parsed_values = parsed_str_opt.unwrap();
        self.parse_board(&parsed_values.0);
        match Color::new_from_u8(parsed_values.1 as u8) {
            None => {
                return Err("Wrong color".to_string());
            }
            Some(x) => {
                self.cur_move = x;
            }
        }
        for i in parsed_values.2.as_bytes() {
            match i {
                b'K' => {
                    self.castle_state_flags |= CastleStateFlag::WhiteShort as u8;
                }
                b'k' => {
                    self.castle_state_flags |= CastleStateFlag::BlackShort as u8;
                }
                b'Q' => {
                    self.castle_state_flags |= CastleStateFlag::WhiteLong as u8;
                }
                b'q' => {
                    self.castle_state_flags |= CastleStateFlag::BlackLong as u8;
                }
                b'-' => {
                    self.castle_state_flags = 0;
                    break;
                }
                _ => {
                    return Err("Wrong casle format".to_string());
                }
            }
        }
        if parsed_values.3 != "-" {
            self.en_passant = Pos::new_from_str(&parsed_values.3).get_code();
        }

        self.cur_move_to_draw = parsed_values.4;
        self.cur_move_num = parsed_values.5;

        Ok(())
    }
}
