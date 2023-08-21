use utils::color::Color;

use crate::chessboard::{Chessboard, moves::Move};

use super::{bishop, rook};

pub fn generate_pseudo_moves(
    queens: u64,
    chessboard: &Chessboard,
    color: &Color,
    moves: &mut Vec<Move>,
) {
    bishop::generate_pseudo_moves(queens, chessboard, color, moves);
    rook::generate_pseudo_moves(queens, chessboard, color, moves);
}
