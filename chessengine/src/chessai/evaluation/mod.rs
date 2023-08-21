pub mod basic;

use utils::color::Color;

use self::basic::material::material_balance;

use super::Ai;

pub const MAX_SCORE: i32 = std::i32::MAX;
pub const MIN_SCORE: i32 = std::i32::MIN;

impl Ai {
    pub fn evaluate_board(&mut self, turn: &Color) -> i32 {
        if self.is_end_game {
            self.is_end_game = false;
            //println!("End game: {:?}", turn);
            //self.chessboard.pretty_print();

            if self.chessboard.is_in_check(turn) {
                //println!("Checkmate: {:?}", self.color);
                if *turn == self.color {
                    return MIN_SCORE;
                } else {
                    return MAX_SCORE;
                }
            }
            else if *turn == self.color {
                return 0;
            }
        }

        material_balance(&self.chessboard, turn)
    }
}
