use core::fmt;
use std::arch::x86_64::_MM_FROUND_CUR_DIRECTION;

pub enum Piece {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    pub fn to_string(&self) -> &str {
        use Piece::*;
        match self.get_color() {
            Color::Black => match self {
                King(_) => "♔",
                Queen(_) => "♕",
                Rook(_) => "♖",
                Knight(_) => "♘",
                Bishop(_) => "♗",
                Pawn(_) => "♙",
            },
            Color::White => match self {
                King(_) => "♚",
                Queen(_) => "♛",
                Rook(_) => "♜",
                Knight(_) => "♞",
                Bishop(_) => "♝",
                Pawn(_) => "♟",
            },
        }
    }

    #[inline]
    pub fn get_color(&self) -> Color {
        use Piece::*;
        match self {
            Pawn(c) | Rook(c) | Knight(c) | Bishop(c) | Queen(c) | King(c) => *c,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Position {
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
}

impl Position {
    pub fn get(&self) -> (u8, u8) {
        match self {
            Position::A1 => (0, 0),
            Position::A2 => (0, 1),
            Position::A3 => (0, 2),
            Position::A4 => (0, 3),
            Position::A5 => (0, 4),
            Position::A6 => (0, 5),
            Position::A7 => (0, 6),
            Position::A8 => (0, 7),
            Position::B1 => (1, 0),
            Position::B2 => (1, 1),
            Position::B3 => (1, 2),
            Position::B4 => (1, 3),
            Position::B5 => (1, 4),
            Position::B6 => (1, 5),
            Position::B7 => (1, 6),
            Position::B8 => (1, 7),
            Position::C1 => (2, 0),
            Position::C2 => (2, 1),
            Position::C3 => (2, 2),
            Position::C4 => (2, 3),
            Position::C5 => (2, 4),
            Position::C6 => (2, 5),
            Position::C7 => (2, 6),
            Position::C8 => (2, 7),
            Position::D1 => (3, 0),
            Position::D2 => (3, 1),
            Position::D3 => (3, 2),
            Position::D4 => (3, 3),
            Position::D5 => (3, 4),
            Position::D6 => (3, 5),
            Position::D7 => (3, 6),
            Position::D8 => (3, 7),
            Position::E1 => (4, 0),
            Position::E2 => (4, 1),
            Position::E3 => (4, 2),
            Position::E4 => (4, 3),
            Position::E5 => (4, 4),
            Position::E6 => (4, 5),
            Position::E7 => (4, 6),
            Position::E8 => (4, 7),
            Position::F1 => (5, 0),
            Position::F2 => (5, 1),
            Position::F3 => (5, 2),
            Position::F4 => (5, 3),
            Position::F5 => (5, 4),
            Position::F6 => (5, 5),
            Position::F7 => (5, 6),
            Position::F8 => (5, 7),
            Position::G1 => (6, 0),
            Position::G2 => (6, 1),
            Position::G3 => (6, 2),
            Position::G4 => (6, 3),
            Position::G5 => (6, 4),
            Position::G6 => (6, 5),
            Position::G7 => (6, 6),
            Position::G8 => (6, 7),
            Position::H1 => (7, 0),
            Position::H2 => (7, 1),
            Position::H3 => (7, 2),
            Position::H4 => (7, 3),
            Position::H5 => (7, 4),
            Position::H6 => (7, 5),
            Position::H7 => (7, 6),
            Position::H8 => (7, 7),
        }
    }

    pub fn from(file: u8, rank: u8) -> Self {
        match (file, rank) {
            (0, 0) => Position::A1,
            (0, 1) => Position::A2,
            (0, 2) => Position::A3,
            (0, 3) => Position::A4,
            (0, 4) => Position::A5,
            (0, 5) => Position::A6,
            (0, 6) => Position::A7,
            (0, 7) => Position::A8,
            (1, 0) => Position::B1,
            (1, 1) => Position::B2,
            (1, 2) => Position::B3,
            (1, 3) => Position::B4,
            (1, 4) => Position::B5,
            (1, 5) => Position::B6,
            (1, 6) => Position::B7,
            (1, 7) => Position::B8,
            (2, 0) => Position::C1,
            (2, 1) => Position::C2,
            (2, 2) => Position::C3,
            (2, 3) => Position::C4,
            (2, 4) => Position::C5,
            (2, 5) => Position::C6,
            (2, 6) => Position::C7,
            (2, 7) => Position::C8,
            (3, 0) => Position::D1,
            (3, 1) => Position::D2,
            (3, 2) => Position::D3,
            (3, 3) => Position::D4,
            (3, 4) => Position::D5,
            (3, 5) => Position::D6,
            (3, 6) => Position::D7,
            (3, 7) => Position::D8,
            (4, 0) => Position::E1,
            (4, 1) => Position::E2,
            (4, 2) => Position::E3,
            (4, 3) => Position::E4,
            (4, 4) => Position::E5,
            (4, 5) => Position::E6,
            (4, 6) => Position::E7,
            (4, 7) => Position::E8,
            (5, 0) => Position::F1,
            (5, 1) => Position::F2,
            (5, 2) => Position::F3,
            (5, 3) => Position::F4,
            (5, 4) => Position::F5,
            (5, 5) => Position::F6,
            (5, 6) => Position::F7,
            (5, 7) => Position::F8,
            (6, 0) => Position::G1,
            (6, 1) => Position::G2,
            (6, 2) => Position::G3,
            (6, 3) => Position::G4,
            (6, 4) => Position::G5,
            (6, 5) => Position::G6,
            (6, 6) => Position::G7,
            (6, 7) => Position::G8,
            (7, 0) => Position::H1,
            (7, 1) => Position::H2,
            (7, 2) => Position::H3,
            (7, 3) => Position::H4,
            (7, 4) => Position::H5,
            (7, 5) => Position::H6,
            (7, 6) => Position::H7,
            (7, 7) => Position::H8,
            _ => panic!("impossible position requested."),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White,
    Black,
}

impl core::ops::Not for Color {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &BLACK => write!(f, "Black"),
            &WHITE => write!(f, "White"),
        }
    }
}

pub const WHITE: Color = Color::White;
pub const BLACK: Color = Color::Black;

type BitBoard = u64;

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
                    BLACK => WIDTH - col - 1,
                    WHITE => col,
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
