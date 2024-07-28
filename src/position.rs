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
