use super::*;

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
