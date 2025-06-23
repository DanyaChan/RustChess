

const BOARD_SIZE : usize = 8;
const BOARD_ARRAY_SIZE : usize = BOARD_SIZE * BOARD_SIZE;
type MoveCode = u16; // array of 4 4bit numbers
type PosCode = u8;

#[repr(u8)]
#[derive(Clone)]
#[derive(Copy)]
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

impl ChessPiece {
    fn get_piece_char(&self) -> char {
        return '.';
    }
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

impl Color {
    fn get_name(&self) -> &'static str  {
        if *self == Color::White { "White" } else { "Black" } 
    }
}

/*
Such types for MoveCode, CastleStateFlag are required for less memory use when copying state for eval
 */
pub struct ChessBoardState {
    cur_move: Color,
    en_passant: PosCode,
    castle_state_flags: u8,
    board: [ChessPiece; BOARD_ARRAY_SIZE],
    cur_move_num: i16,
    cur_move_to_draw: u8,
}

impl ChessBoardState {
    pub fn debug_print(&self) {
        for i in 0..self.board.len() {
            if i % 8 == 0 {
                print!("\n");
            }
            print!("{}", self.board[i].get_piece_char())
        }
        print!("\nMove {}, en passant TODO, castle TODO", self.cur_move.get_name());
    }

    pub fn new() -> Self {
        ChessBoardState {
            cur_move : Color::White,
            en_passant: 0xFF,
            castle_state_flags: 0xF0,
            board: [ChessPiece::None; BOARD_ARRAY_SIZE],
            cur_move_num: 1,
            cur_move_to_draw: 0,
        }
    }

    pub fn loadFen(&mut self, fen_str : &str) {

    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
   #[test]
   fn test_load_fen() {
        let mut fen = ChessBoardState::new();
        fen.loadFen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert!(fen.cur_move == Color::White);
   }
}

