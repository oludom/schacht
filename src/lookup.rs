use super::*;

// pub fn make_bishop_map(index: u8) -> String {
//     let mut map: u64 = 0;
//     let bishop = 1 << index;
//     let right = index % WIDTH;
//     let left = WIDTH - 1 - right;

//     // right
//     for i in 0..right {
//         map |= bishop << 7 * (i + 1);
//         map |= bishop >> 9 * (i + 1);
//     }
//     // left
//     for i in 0..left {
//         map |= bishop << 9 * (i + 1);
//         map |= bishop >> 7 * (i + 1);
//     }
//     from_u64(map)
// }

// fn from_bit_string(s: String, reverse: bool) -> String {
//     // if reverse {
//     //     // reverse
//     //     let s: &str = &s.chars().rev().collect::<String>();
//     // }
//     let mut ss = String::new();
//     for i in 0..WIDTH as usize {
//         ss.push_str("\n    0b");
//         let idx = i * WIDTH as usize;
//         let end = idx + WIDTH as usize;
//         ss.push_str(&s[idx..end]);
//         ss.push_str(&format!(" << RANK{} ", WIDTH as usize - i));
//         if i != WIDTH as usize - 1 {
//             ss.push('|');
//         }
//     }
//     ss.push(',');
//     ss.push('\n');
//     ss
// }

// fn from_u64(u: u64) -> String {
//     from_bit_string(format!("{:064b}", u), false)
// }