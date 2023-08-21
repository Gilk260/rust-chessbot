use crate::chessai::{Ai, evaluation::{MIN_SCORE, MAX_SCORE}};

impl Ai {
    pub fn alpha_beta(&mut self,
                      depth: u32,
                      mut alpha: i32,
                      mut beta: i32) -> i32 {
        let moves = self.chessboard.generate_legal_moves();
        let color = self.chessboard.turn;

        if moves.len() == 0 {
            //self.chessboard.pretty_print();
            self.is_end_game = true;
        }

        if depth == 0 || self.is_end_game {
            return self.evaluate_board(&color);
        }

        let mut boundary: i32;
        let func: fn(i32, i32) -> i32;
        match self.chessboard.turn == self.color {
            true => {
                boundary = MIN_SCORE;
                func = std::cmp::max;
            },
            false => {
                boundary = MAX_SCORE;
                func = std::cmp::min;
            },
        };

        for mv in moves.iter() {
            self.chessboard.make_move(&mv);
            let score = self.alpha_beta(depth - 1, alpha, beta);
            self.chessboard.unmake_move(&mv);

            boundary = func(boundary, score);

            if self.chessboard.turn == self.color {
                alpha = func(alpha, score);
            } else {
                beta = func(beta, score);
            }

            if beta <= alpha {
                break;
            }
        }
        //println!("{} {}", depth, boundary);

        boundary
    }
}
