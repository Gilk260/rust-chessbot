use utils::color::Color;

use crate::chessboard::{Chessboard, moves::Move};

pub mod evaluation;
mod search;
mod uci;

pub struct Ai {
    pub chessboard: Chessboard,
    pub color: Color,
    pub is_end_game: bool,
}

impl Ai {
    pub fn new(fen: String) -> Ai {
        let chessboard = Chessboard::new(fen);
        let color = chessboard.turn.clone();
        Ai {
            chessboard,
            color,
            is_end_game: false,
        }
    }
}

pub fn run_ai(){
    let mut ai: Ai = Ai::new(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));

    ai.handle_uci_protocol();
}
