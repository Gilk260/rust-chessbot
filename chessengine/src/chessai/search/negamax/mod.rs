use utils::color::Color;

use crate::chessai::Ai;

impl Ai {
    pub fn negamax(&mut self, color_to_max: &Color, depth: u32) -> i32 {
        let modifier = if self.chessboard.turn == *color_to_max { 1 } else { -1 };
        let moves = self.chessboard.generate_legal_moves();

        if moves.len() == 0 {
            self.is_end_game = true;
        }

        if depth == 0 {
            //chessboard.pretty_print();
            return modifier * self.evaluate_board(color_to_max);
        }

        let mut max = std::i32::MIN;
        for mv in moves.iter() {
            self.chessboard.make_move(&mv);
            let score = - modifier * self.negamax(color_to_max, depth - 1);
            self.chessboard.unmake_move(&mv);

            if score > max {
                max = score;
            }
        }

        //println!("{} {}", depth, max);

        max
    }
}
