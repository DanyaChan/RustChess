const BOARD_SIZE: usize = 8;
const BOARD_ARRAY_SIZE: usize = BOARD_SIZE * BOARD_SIZE;
type MoveCode = u16; // array of 4 4bit numbers
type PosCode = u8;

#[repr(u8)]
#[derive(Clone, Copy)]
enum ChessPiece {
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

enum CastleStateFlag {
    WhiteLong = 1,
    WhiteShort = 1 << 1,
    BlackLong = 1 << 2,
    BlackShort = 1 << 3,
}

#[derive(PartialEq)]
enum Color {
    White,
    Black,
}

struct Pos {
    x: u8,
    y: u8,
}

struct Move {
    from: Pos,
    to: Pos,
}

pub struct ChessBoardState {
    cur_move: Color,
    en_passant: PosCode,
    castle_state_flags: u8,
    board: [ChessPiece; BOARD_ARRAY_SIZE],
    cur_move_num: i16,
    cur_move_to_draw: u8,
}

impl Pos {
    fn new_from_str(pos_str: &str) -> Self {
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

    fn get_code(&self) -> PosCode {
        (self.x & 0x0F) | (self.y << 4 & 0xF0)
    }
}

impl Move {
    fn new_from_str(move_str: &str) -> Self {
        Move {
            from: Pos::new_from_str(&move_str[0..2]),
            to: Pos::new_from_str(&move_str[3..5])
        }
    }

    fn get_code(&self) -> MoveCode {
        self.from.get_code() as u16 | ((self.to.get_code() as u16) << 8)
    }
}

impl ChessPiece {
    fn get_piece_char(&self) -> char {
        return '.';
    }
}

impl Color {
    fn get_name(&self) -> &'static str {
        if *self == Color::White {
            "White"
        } else {
            "Black"
        }
    }

    fn new_from_char(c: &char) -> Option<Self> {
        match &c {
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
            castle_state_flags: 0xF0,
            board: [ChessPiece::None; BOARD_ARRAY_SIZE],
            cur_move_num: 1,
            cur_move_to_draw: 0,
        }
    }

    pub fn loadFen(&mut self, fen_str: &str) {}
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_load_pos() {
        let pos1 = Pos::new_from_str("e2");
        let pos2 = Pos::new_from_str("c7");
        let pos3 = Pos::new_from_str("h1");
        let pos4 = Pos::new_from_str("");
        assert!(pos1.x == 4 && pos1.y == 1);
        assert!(pos2.x == 2 && pos2.y == 6);
        assert!(pos3.x == 7 && pos3.y == 0);
        assert!(pos4.x == 0 && pos4.y == 0);

        assert_eq!(pos1.get_code(), 0x14);
        assert_eq!(pos2.get_code(), 0x62);
        assert_eq!(pos3.get_code(), 0x07);
        assert_eq!(pos4.get_code(), 0x00);
    }

    #[test]
    fn run_load_move() {
        let move1 = Move::new_from_str("e2-e4");
        assert_eq!(move1.get_code(), 0x3414);

        let move2 = Move::new_from_str("f4-h3");
        assert_eq!(move2.get_code(), 0x2735);
    }

    #[test]
    fn test_load_fen() {
        let mut fen = ChessBoardState::new();
        fen.loadFen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert!(fen.cur_move == Color::White);
    }
}
