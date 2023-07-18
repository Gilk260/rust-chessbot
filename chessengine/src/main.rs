use utils;

mod chessboard;

fn main() {
    let args = utils::parse_args(std::env::args());
    match args.get_flag() {
        utils::Flag::Ai => {
            println!("Launch AI.");
        },
        utils::Flag::Help => {
            utils::print_help();
        },
        utils::Flag::Perft => {
            chessboard::perft::run_perft(args.get_file_path());
        },
        utils::Flag::Invalid => {
            println!("Invalid flag: {}", args.get_file_path());
            utils::print_help();
        },
    }

}
