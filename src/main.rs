// use clap::{arg, command, value_parser, ArgAction, Command};
use schacht::board::Board;

fn main() {
    let b = Board::default();
    println!("{}", b);
}
