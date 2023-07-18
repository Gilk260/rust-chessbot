use std::{fs, println};

use super::Chessboard;

pub fn run_perft(file_path: &String) {
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let mut chessboard = Chessboard::new(contents);
    //chessboard.pretty_print();
    let nodes = chessboard.perft();
    //chessboard.show_hashmap();
    println!("{}", nodes);
}

impl Chessboard {
    pub fn perft(&mut self) -> u64 {
        //self.pretty_print();
        if self.perft_depth == 0 {
            //self.to_fen();
            return 1;
        }

        let mut nodes = 0;

        let moves = self.generate_moves();
        for m in moves {
            self.make_move(&m);
            if !self.is_in_check() {

                let mut count = self.mv_hashmap.get(&m.to_string());
                if count == None {
                    self.mv_hashmap.insert(m.to_string(), 0);
                    count = Some(&0);
                }
                self.mv_hashmap.insert(m.to_string(), count.unwrap() + 1);

                nodes += self.perft();
            }
            self.unmake_move(&m);
        }

        //self.pretty_print();
        //self.to_fen();
        //println!("{}: {}", self.perft_depth, nodes);
        nodes
    }

    pub fn show_hashmap(&self) {
        let mut sorted = self.mv_hashmap.iter().collect::<Vec<_>>();
        sorted.sort_by_key(|a| a.0);
        for (key, value) in sorted.iter() {
            println!("{}: {}", key, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    fn test_perft(fen: &str, expected: u64, show_debug: bool) {
        let mut chessboard = Chessboard::new(String::from(fen));

        if show_debug {
            chessboard.pretty_print();
        }

        let actual = chessboard.perft();

        if show_debug {
            chessboard.show_hashmap();
        }

        assert_eq!(actual, expected);
    }

    #[test]
    fn base() {
        test_perft("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 0", 1, false);
        test_perft("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 1", 20, false);
        test_perft("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 2", 400, false);
        test_perft("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 3", 8902, false);
        //test_perft("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 4", 197281, false);
        //test_perft("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 5", 4865609, false);
    }

    #[test]
    fn kiwipete() {
        test_perft("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 1", 48, false);
        test_perft("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 2", 2039, false);
        test_perft("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 3", 97862, false);
    }

    #[test]
    fn forbidden_castling() {
        test_perft("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 1", 14, false);
        test_perft("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 2", 191, false);
        test_perft("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 3", 2812, false);
        //test_perft("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 4", 43238, false);
        //test_perft("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 5", 674624, false);
    }

    #[test]
    fn mirrored() {
        test_perft("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1 1", 6, false);
        test_perft("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1 2", 264, false);
        test_perft("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1 3", 9467, false);

        test_perft("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1 1", 6, false);
        test_perft("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1 2", 264, false);
        test_perft("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1 3", 9467, false);
    }

    #[test]
    fn edwards() {
        test_perft("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8 1", 44, false);
        test_perft("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8 2", 1486, false);
        test_perft("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8 3", 62379, false);
    }

    #[test]
    fn edwards_bis() {
        test_perft("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 1", 46, false);
        test_perft("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 2", 2079, false);
        test_perft("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 3", 89890, false);
    }

    #[test]
    fn misc() {
        test_perft("rnbqkbnr/ppppp2pp/8/5p2/8/2P5/PP1PPPPP/RNBQKBNR w KQkq f6 2 2 1", 21, false);
        test_perft("rnbqkbnr/1pppppp1/p7/7p/8/P1P5/1P1PPPPP/RNBQKBNR w KQkq h6 4 3 1", 20, false);
        test_perft("rnbqkbnr/1ppp1ppp/p7/4p3/8/PP6/2PPPPPP/RNBQKBNR w KQkq e6 4 3 1", 19, false);
        test_perft("rnbqk1nr/pppp1ppp/4p3/8/Qb6/2P5/PP1PPPPP/RNB1KBNR w KQkq - 4 3 1", 29, false);
        test_perft("rnbqkbnr/2pppppp/p7/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 4 3 1", 22, false);
        test_perft("r3k3/p1ppqpb1/bn2pnp1/3PN3/1p2P2r/5Q1p/PPPBBPPP/RN2K2R w KQq - 2 2 1", 49, false);
    }
}
