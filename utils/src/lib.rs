use std::env;

pub mod color;
pub mod direction;
pub mod file;
pub mod piece;
pub mod rank;
pub mod square;

#[derive(PartialEq, Debug)]
pub enum Flag {
    Ai,
    Help,
    Perft,
    Invalid,
}

pub struct Args {
    flag: Flag,
    file_path: String,
}

impl Args {
    fn new(flag: Flag, file_path: String) -> Args {
        Args {
            flag,
            file_path,
        }
    }

    pub fn get_flag(&self) -> &Flag {
        &self.flag
    }

    pub fn get_file_path(&self) -> &String {
        &self.file_path
    }
}

pub fn parse_args(
    mut args: impl Iterator<Item = String>
) -> Args {
    args.next(); // Skip the first argument, which is the program name.

    match args.next() {
        Some(arg) => match arg.as_str() {
            "-h" => Args::new(Flag::Help, String::from("")),
            "--perft" => Args::new(Flag::Perft, args.next().unwrap().to_string()),
            _ => Args::new(Flag::Invalid, String::from(arg)),
        },
        None => Args::new(Flag::Ai, String::from("")),
    }
}

pub fn print_help() {
    let args: Vec<String> = env::args().collect();
    println!("Usage: {} [flag] [file]", args[0]);
    println!("Flags:");
    println!("  -h: Print this help message.");
    println!("  --perft [depth]: Run a perft test on the given file.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_no_args() {
        let args = vec![
            String::from("chessengine"),
        ];
        let parsed_args = parse_args(args.into_iter());
        assert_eq!(parsed_args.get_flag(), &Flag::Ai);
        assert_eq!(parsed_args.get_file_path(), &String::from(""));
    }

    #[test]
    fn test_parse_args_help() {
        let args = vec![
            String::from("chessengine"),
            String::from("-h"),
        ];
        let parsed_args = parse_args(args.into_iter());
        assert_eq!(parsed_args.get_flag(), &Flag::Help);
        assert_eq!(parsed_args.get_file_path(), &String::from(""));
    }

    #[test]
    fn test_parse_args_perft() {
        let args = vec![
            String::from("chessengine"),
            String::from("--perft"),
            String::from("test.perft"),
        ];
        let parsed_args = parse_args(args.into_iter());
        assert_eq!(parsed_args.get_flag(), &Flag::Perft);
        assert_eq!(parsed_args.get_file_path(), &String::from("test.perft"));
    }

    #[test]
    fn test_parse_args_invalid() {
        let args = vec![
            String::from("chessengine"),
            String::from("--invalid"),
        ];
        let parsed_args = parse_args(args.into_iter());
        assert_eq!(parsed_args.get_flag(), &Flag::Invalid);
        assert_eq!(parsed_args.get_file_path(), &String::from("--invalid"));
    }
}
