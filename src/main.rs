use clap::{arg, command, value_parser, ArgAction, Command};
use schacht::{
    board::{self, Board, DebugDisplay},
};

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

    println!("BLACK: \n{}", b.get_black().debug_print());
    println!("WHITE: \n{}", b.get_white().debug_print());
    println!("EMPTY: \n{}", b.get_empty_squares().debug_print());


    // for i in 0..64 {
    //     println!("index: {} \n{}", i, &make_bishop_map(i));
    // }
}
