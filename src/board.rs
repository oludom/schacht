use super::*;

pub trait DebugDisplay {
    fn debug_print(&self) -> String;
    fn is_set(&self, pos: u8) -> bool;
    fn print(&self) {
        println!("{}", self.debug_print())
    }
}

type BitBoard = u64;

impl DebugDisplay for BitBoard {
    fn debug_print(&self) -> String {
        let mut out = String::new();
        let abc = "   abcdefgh\n";
        out.push_str(abc);

        out.push_str("  ╔════════╗");
        for r in 0..HEIGHT {
            let rank = HEIGHT - 1 - r;
            out.push_str("\n");
            out.push_str(&format!("{} ║", rank + 1));

            for f in 0..WIDTH {
                let file = WIDTH - 1 - f;
                let bit_pos = rank * HEIGHT + file;
                if self.is_set(bit_pos) {
                    out.push('▓');
                } else {
                    out.push('░');
                }
            }
            out.push_str("║");
        }

        out.push_str("\n  ╚════════╝\n");
        out.push_str(abc);
        out
    }

    fn is_set(&self, pos: u8) -> bool {
        (self >> pos) & 1 > 0
    }
}

pub struct Board {
    b_pawn: BitBoard,
    b_rook: BitBoard,
    b_knight: BitBoard,
    b_bishop: BitBoard,
    b_queen: BitBoard,
    b_king: BitBoard,
    w_pawn: BitBoard,
    w_rook: BitBoard,
    w_knight: BitBoard,
    w_bishop: BitBoard,
    w_queen: BitBoard,
    w_king: BitBoard,
    enpassant: BitBoard,
    castling: BitBoard,
    current_to_move: Color,
}

const WIDTH: u8 = 8;
const HEIGHT: u8 = 8;

const RANK8: u8 = 56;
const RANK7: u8 = 48;
const RANK6: u8 = 40;
const RANK5: u8 = 32;
const RANK4: u8 = 24;
const RANK3: u8 = 16;
const RANK2: u8 = 8;
const RANK1: u8 = 0;
const POSITION_CASTLING_KING: u8 = 1;
const POSITION_CASTLING_QUEEN: u8 = 6;

// 0b00000000 00000000
const START_PAWN: BitBoard = 0b11111111;
const START_ROOK: BitBoard = 0b10000001;
const START_KNIGHT: BitBoard = 0b01000010;
const START_BISHOP: BitBoard = 0b00100100;
const START_QUEEN: BitBoard = 0b00010000;
const START_KING: BitBoard = 0b00001000;
const START_CASTLING: BitBoard = 0b00100010;

const START_B_PAWN: BitBoard = START_PAWN << RANK7;
const START_B_ROOK: BitBoard = START_ROOK << RANK8;
const START_B_QUEEN: BitBoard = START_QUEEN << RANK8;
const START_B_KING: BitBoard = START_KING << RANK8;
const START_B_BISHOP: BitBoard = START_BISHOP << RANK8;
const START_B_KNIGHT: BitBoard = START_KNIGHT << RANK8;

const START_W_QUEEN: BitBoard = START_QUEEN;
const START_W_KING: BitBoard = START_KING;
const START_W_KNIGHT: BitBoard = START_KNIGHT;
const START_W_PAWN: BitBoard = START_PAWN << RANK2;
const START_W_BISHOP: BitBoard = START_BISHOP;
const START_W_ROOK: BitBoard = START_ROOK;

impl Board {
    pub fn default() -> Self {
        Self {
            b_pawn: START_B_PAWN,
            b_rook: START_B_ROOK,
            b_knight: START_B_KNIGHT,
            b_bishop: START_B_BISHOP,
            b_queen: START_B_QUEEN,
            b_king: START_B_KING,
            w_pawn: START_W_PAWN,
            w_rook: START_W_ROOK,
            w_knight: START_W_KNIGHT,
            w_bishop: START_W_BISHOP,
            w_queen: START_W_QUEEN,
            w_king: START_W_KING,
            enpassant: 0,
            castling: START_CASTLING | (START_CASTLING << RANK8),
            current_to_move: Color::White,
        }
    }

    pub fn empty() -> Self {
        Self {
            b_pawn: 0,
            b_rook: 0,
            b_knight: 0,
            b_bishop: 0,
            b_queen: 0,
            b_king: 0,
            w_pawn: 0,
            w_rook: 0,
            w_knight: 0,
            w_bishop: 0,
            w_queen: 0,
            w_king: 0,
            enpassant: 0,
            castling: 0,
            current_to_move: Color::White,
        }
    }

    pub fn get_piece(&self, p: Position) -> Option<Piece> {
        let (file, rank) = p.get();
        if Self::is_set(file, rank, self.b_pawn) {
            Some(Piece::Pawn(Color::Black))
        } else if Self::is_set(file, rank, self.b_rook) {
            Some(Piece::Rook(Color::Black))
        } else if Self::is_set(file, rank, self.b_knight) {
            Some(Piece::Knight(Color::Black))
        } else if Self::is_set(file, rank, self.b_bishop) {
            Some(Piece::Bishop(Color::Black))
        } else if Self::is_set(file, rank, self.b_queen) {
            Some(Piece::Queen(Color::Black))
        } else if Self::is_set(file, rank, self.b_king) {
            Some(Piece::King(Color::Black))
        } else if Self::is_set(file, rank, self.w_pawn) {
            Some(Piece::Pawn(Color::White))
        } else if Self::is_set(file, rank, self.w_rook) {
            Some(Piece::Rook(Color::White))
        } else if Self::is_set(file, rank, self.w_knight) {
            Some(Piece::Knight(Color::White))
        } else if Self::is_set(file, rank, self.w_bishop) {
            Some(Piece::Bishop(Color::White))
        } else if Self::is_set(file, rank, self.w_queen) {
            Some(Piece::Queen(Color::White))
        } else if Self::is_set(file, rank, self.w_king) {
            Some(Piece::King(Color::White))
        } else {
            None
        }
    }

    fn is_set(file: u8, rank: u8, piece: BitBoard) -> bool {
        (piece >> (file + (8 * rank))) & 1 > 0
    }

    #[inline]
    fn set(v: &mut BitBoard, pos: u8) {
        *v |= 1 << pos;
    }

    pub fn get_white(&self) -> BitBoard {
        self.w_king | self.w_queen | self.w_bishop | self.w_knight | self.w_rook | self.w_pawn
    }

    pub fn get_black(&self) -> BitBoard {
        self.b_king | self.b_queen | self.b_bishop | self.b_knight | self.b_rook | self.b_pawn
    }

    pub fn get_empty_squares(&self) -> BitBoard {
        !self.get_occupied()
    }
    pub fn get_occupied(&self) -> BitBoard {
        self.get_white() | self.get_black()
    }
}

// FEN notation
/*
starting position
rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR

 */

impl From<&str> for Board {
    // expects FEN notation
    fn from(value: &str) -> Self {
        let mut b_pawn: BitBoard = 0;
        let mut b_rook: BitBoard = 0;
        let mut b_knight: BitBoard = 0;
        let mut b_bishop: BitBoard = 0;
        let mut b_queen: BitBoard = 0;
        let mut b_king: BitBoard = 0;
        let mut w_pawn: BitBoard = 0;
        let mut w_rook: BitBoard = 0;
        let mut w_knight: BitBoard = 0;
        let mut w_bishop: BitBoard = 0;
        let mut w_queen: BitBoard = 0;
        let mut w_king: BitBoard = 0;
        let mut current_to_move = WHITE;
        let mut castling: BitBoard = 0;
        let mut enpassant: BitBoard = 0;

        let mut current_position: u8 = 64;
        let mut itt = value.chars();

        for c in itt.by_ref() {
            if current_position > 0 {
                current_position -= 1;
            }
            match c {
                'r' => Self::set(&mut b_rook, current_position),
                'R' => Self::set(&mut w_rook, current_position),
                'n' => Self::set(&mut b_knight, current_position),
                'N' => Self::set(&mut w_knight, current_position),
                'b' => Self::set(&mut b_bishop, current_position),
                'B' => Self::set(&mut w_bishop, current_position),
                'q' => Self::set(&mut b_queen, current_position),
                'Q' => Self::set(&mut w_queen, current_position),
                'k' => Self::set(&mut b_king, current_position),
                'K' => Self::set(&mut w_king, current_position),
                'p' => Self::set(&mut b_pawn, current_position),
                'P' => Self::set(&mut w_pawn, current_position),
                '/' => {
                    current_position += 1;
                    continue;
                }
                ' ' => break,
                n if n.is_numeric() => current_position -= n.to_digit(10).unwrap() as u8 - 1,
                _ => panic!("unrecognized FEN char for position"),
            }
        }

        // match color
        for c in itt.by_ref() {
            match c {
                'w' => current_to_move = WHITE,
                'b' => current_to_move = BLACK,
                ' ' => break,
                s => panic!("unrecognized color {}", s),
            }
        }

        // retrieve castling rights
        for c in itt.by_ref() {
            match c {
                'K' => Self::set(&mut castling, POSITION_CASTLING_KING),
                'k' => Self::set(&mut castling, POSITION_CASTLING_KING + RANK8),
                'Q' => Self::set(&mut castling, POSITION_CASTLING_QUEEN),
                'q' => Self::set(&mut castling, POSITION_CASTLING_QUEEN + RANK8),
                ' ' => break,
                '-' => continue,
                _ => panic!("unrecognized FEN char for castling rights"),
            }
        }

        let mut rank = 0;
        let mut file = 0;
        // retrieve enpassant
        for c in itt.by_ref() {
            match c {
                r @ 'a'..='h' => rank = r as u8 - 'a' as u8,
                n @ '1'..='8' => file = n as u8 - '1' as u8,
                '-' => continue,
                ' ' => break,
                _ => panic!("unrecognized FEN char for enpassant"),
            }
        }
        Self::set(&mut enpassant, file + (8 * rank));

        // for now skip number of moves

        Self {
            b_pawn,
            b_rook,
            b_knight,
            b_bishop,
            b_queen,
            b_king,
            w_pawn,
            w_rook,
            w_knight,
            w_bishop,
            w_queen,
            w_king,
            enpassant,
            castling,
            current_to_move,
        }
    }
}

impl core::fmt::Display for Board {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        let abc = if self.current_to_move == WHITE {
            "abcdefgh"
        } else {
            "hgfedcba"
        };

        write!(f, "   {}\n  ╔════════╗", abc)?;
        let mut square_color = !self.current_to_move;

        for row in 0..HEIGHT {
            writeln!(f)?;

            let print_row = match self.current_to_move {
                WHITE => HEIGHT - row - 1,
                BLACK => row,
            };
            write!(f, "{} ║", print_row + 1)?;

            for col in 0..WIDTH {
                let print_col = match self.current_to_move {
                    BLACK => col,
                    WHITE => WIDTH - col - 1,
                };

                let pos = Position::from(print_col, print_row);

                let s = if let Some(piece) = self.get_piece(pos) {
                    piece.to_string().to_owned()
                } else {
                    String::from(match square_color {
                        WHITE => "░",
                        BLACK => "▓",
                    })
                };

                write!(f, "{}", s)?;

                square_color = !square_color;
            }
            write!(f, "║")?;

            if row == 3 {
                write!(f, " {} to move", self.current_to_move)?;
            }

            square_color = !square_color;
        }

        write!(f, "\n  ╚════════╝\n   {}\n", abc)
    }
}
