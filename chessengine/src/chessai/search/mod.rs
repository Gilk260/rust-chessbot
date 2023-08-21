use utils::piece::Piece;

use crate::chessboard::{moves::Move, Chessboard};

use super::{Ai, evaluation::{MIN_SCORE, MAX_SCORE, basic::material::piece_value}};

pub mod negamax;
pub mod alpha_beta;

impl Ai {
    pub fn find_best_move(&mut self, flag: u32) -> Option<super::Move> {
        let mut best_move: Option<Move> = None;
        let mut best_move_score = MIN_SCORE;

        let mut moves = self.chessboard.generate_legal_moves();
        self.chessboard.order_moves(&mut moves);

        for mv in moves {
            self.chessboard.make_move(&mv);
            //let score = self.negamax(&color_to_maximize, flag);
            let score = self.alpha_beta(flag, MIN_SCORE, MAX_SCORE);
            self.chessboard.unmake_move(&mv);
            eprintln!("{} {:?}", score, mv);

            if score > best_move_score {
                best_move = Some(mv);
                best_move_score = score;
            }
        }

        best_move
    }
}

impl Chessboard {
    pub fn order_moves(&mut self, moves: &mut Vec<Move>) {
        moves.sort_by(|a, b| self.see(b).cmp(&self.see(a)));
    }

    pub fn see(&mut self, mv: &Move) -> i32 {
        if mv.capture == None || mv.promotion == None {
            return 0;
        }

        let piece: Piece = if mv.promotion != None {
            Piece::from(mv.promotion.unwrap())
        } else {
            self.get_piece(&mv.from).unwrap()
        };

        let value = piece_value(&piece);
        let mut opponent_responses: Vec<i32> = Vec::new();

        self.make_move(mv);

        let opponents_moves = self.generate_legal_moves();
        for opponent_move in opponents_moves {
            if opponent_move.to == mv.to {
                let opponent_response = -self.see(&opponent_move);
                opponent_responses.push(opponent_response);
            }
        }

        let best_opponent_response = opponent_responses.iter().cloned().fold(i32::MAX, i32::min);

        self.unmake_move(mv);

        value - best_opponent_response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_find_best_move(fen: &str, expected: &str) {
        let mut chessai = Ai::new(fen.to_string());
        let best_move = chessai.find_best_move(4).unwrap();
        let expected_best_move = chessai.chessboard.generate_move_from_string(expected.to_string());
        assert_eq!(best_move, expected_best_move);
    }

    #[test]
    #[ignore]
    fn test_negamax() {
        test_find_best_move("rnbqkbnr/1p1ppppp/2p5/p6Q/2B5/8/8/4K3 w - - 0 1 1", "c4f7");
        test_find_best_move("rnbqkbnr/1ppppppp/8/p7/2B5/4P3/PPPP1PPP/RNBQK1NR w KQkq - 0 1 1", "d1h5")
    }
}
