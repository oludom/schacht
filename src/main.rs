use clap::{arg, command, value_parser, ArgAction, Command};
use schacht::board::Board;

fn main() {
    let matches = command!()
        .arg(
            arg!(-p --position <FEN> "starting position in FEN notation.")
                .default_value("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
        )
        .get_matches();

    // has default so can unwrap() here
    let position: &str = matches.get_one::<String>("position").unwrap();

    let b = Board::from(position);
    println!("{}", b);
}
