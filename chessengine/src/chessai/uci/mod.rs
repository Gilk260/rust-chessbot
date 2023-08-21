use crate::chessboard::Chessboard;

use super::Ai;

impl Ai {
    pub fn handle_uci_protocol(&mut self) {
        loop {
            let mut buffer = String::new();
            let res = std::io::stdin().read_line(&mut buffer);

            if res.is_err() {
                println!("Error reading from stdin");
                break;
            } else if res.unwrap() == 0 {
                println!("EOF");
                break;
            }

            let buffer = buffer.trim();

            let inputs = buffer.split(' ').collect::<Vec<&str>>();
            let cmd = inputs[0];

            match cmd {
                "uci" => self.handle_uci_cmd(),
                "ucinewgame" => (),
                "isready" => self.handle_isready_cmd(),
                "position" => self.handle_position_cmd(inputs),
                "go" => self.handle_go_cmd(inputs),
                "quit" => break,
                _ => println!("Unknown command: {}", cmd),
            };
        }
    }

    fn handle_uci_cmd(&self) {
        println!("id name GetRusted\nid author Gilk\nuciok");
    }

    fn handle_isready_cmd(&self) {
        println!("readyok");
    }

    fn handle_position_cmd(&mut self, command: Vec<&str>) {
        if command.len() < 2 {
            return;
        }

        let flag = command[1];

        let index_moves = match flag {
            "startpos" => {
                self.chessboard = Chessboard::new(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
                2
            },
            "fen" => {
                self.chessboard =
                    Chessboard::new(String::from(command[2..].join(" ")));
                8
            },
            _ => {
                println!("Unknown position flag: {}", flag);
                return;
            },
        };

        if command.len() < index_moves + 1 {
            return;
        }

        let flag = command[index_moves];

        match flag {
            "moves" => self.handle_moves_cmd(command[index_moves + 1..].to_vec()),
            _ => println!("Expected: moves\nGot: {}", flag),
        };
    }

    fn handle_moves_cmd(&mut self, moves: Vec<&str>) {
        for m in moves {
            let mv = self.chessboard.generate_move_from_string(String::from(m));
            self.chessboard.make_move(&mv);
        }
    }

    fn handle_go_cmd(&mut self, command: Vec<&str>) {
        if command.len() < 3 {
            return;
        }

        let flag = command[1];

        match flag {
            //"infinite" => run_ai(),
            "movetime" => self.handle_movetime_cmd(command[2]),
            //"depth" => run_ai(),
            //"nodes" => run_ai(),
            //"mate" => run_ai(),
            _ => println!("Unknown go command: {}", flag),
        };
    }

    fn handle_movetime_cmd(&mut self, time: &str) {
        let time = time.parse::<u64>();
        if time.is_err() {
            println!("Invalid time: {}", time.unwrap_err());
            return;
        }

        let _time = time.unwrap();

        self.compute_best_move();
    }

    fn compute_best_move(&mut self) {
        let bestmove = self.find_best_move(3);

        if !bestmove.is_none() {
            println!("bestmove {}", bestmove.unwrap().to_string());
        } else {
            println!("bestmove 0000");
        }
    }
}
