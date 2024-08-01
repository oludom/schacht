use std::fs::File;
use std::io::Write;
use std::ops::Index;
use std::path::Path;

const WIDTH: u8 = 8;
const HEIGHT: u8 = 8;

fn make_rook_map(row: u8, col: u8) -> u64 {
    let mut map = 0xFF << row * 8;
    let col_mask = 1 << col;
    map |= col_mask << 0
    | col_mask << 8
    | col_mask << 16
    | col_mask << 24
    | col_mask << 32
    | col_mask << 40
    | col_mask << 48
    | col_mask << 56;
    map &= !(1 << (row * 8 + col));
    map
}

fn make_bishop_map(index: u8) -> u64 {
    let mut map: u64 = 0;
    let bishop = 1 << index;
    let right = index % WIDTH;
    let left = WIDTH - 1 - right;

    // right
    for i in 0..right {
        map |= bishop << 7 * (i + 1);
        map |= bishop >> 9 * (i + 1);
    }
    // left
    for i in 0..left {
        map |= bishop << 9 * (i + 1);
        map |= bishop >> 7 * (i + 1);
    }
    map
}

fn make_king_map(index: u8) -> String {
    let mut map: u64 = 0;

    let king: u64 = 1 << index;
    map |= king << 8
    | king >> 8;

    if !(index % WIDTH == 0) {
        map |= king << 7
        | king >> 9
        | king >> 1
    }
    if !(index % WIDTH == 7) {
        map |= king << 9
        | king << 1
        | king >> 7
    }

    from_u64(map)
}

fn make_knight_map(index: u8) -> String {
    let mut map: u64 = 0;
    let knight: u64 = 1 << index;
    let row = index % WIDTH;

    // left side
    if !(row > 5) {
        map |= knight << 10
        | knight >> 6;
    }
    if !(row > 6) {
        map |= knight << 17
        | knight >> 15;
    }

    // right side
    if !(row < 2) {
        map |= knight << 6
        | knight >> 10;
    }
    if !(row < 1) {
        map |= knight << 15
        | knight >> 17;
    }

    from_u64(map)
}

fn from_bit_string(s: String, reverse: bool) -> String {
    // if reverse {
    //     // reverse
    //     let s: &str = &s.chars().rev().collect::<String>();
    // }
    let mut ss = String::new();
    for i in 0..WIDTH as usize {
        ss.push_str("\n    0b");
        let idx = i * WIDTH as usize;
        let end = idx + WIDTH as usize;
        ss.push_str(&s[idx..end]);
        ss.push_str(&format!(" << RANK{} ", WIDTH as usize - i));
        if i != WIDTH as usize - 1 {
            ss.push('|');
        }
    }
    ss.push(',');
    ss.push('\n');
    ss
}

fn from_u64(u: u64) -> String {
    println!("{:064b}", u);
    from_bit_string(format!("{:064b}", u), false)
}

// build script's entry point
fn main() {
    let lookup_out_file_path = Path::new("./src/lookup_generated.rs");
    let mut lookup_out_file = File::create(&lookup_out_file_path).unwrap();

    let mut rl = String::new();
    let mut bl = String::new();
    let mut bishop_lookup: [u64; 64] = [0; 64];
    let mut rook_lookup: [u64; 64] = [0; 64];

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let map = make_rook_map(i, j);
            rook_lookup[(i*WIDTH + j) as usize] = map;
            rl.push_str(&from_u64(map));
        }
    }
    
    let mut kl = String::new();
    let mut knl = String::new();
    let mut ql = String::new();
    
    for i in 0..(WIDTH*HEIGHT) {
        kl.push_str(&make_king_map(i));
        knl.push_str(&make_knight_map(i));
        let map = make_bishop_map(i);
        bishop_lookup[i as usize] = map;
        bl.push_str(&from_u64(map));
        ql.push_str(&from_u64(bishop_lookup[i as usize] | rook_lookup[i as usize]));
    }

    write!(
        lookup_out_file,
        include_str!("./src/lookup_generated.rs.template"),
        rook_list = rl,
        bishop_list = bl,
        king_list = kl,
        knight_list = knl,
        queen_list = ql,
    )
    .unwrap();
}
