use std::println;

use utils::color::Color;

use crate::chessboard::{Chessboard, moves::Move};

use super::{bishop, rook};

pub fn generate_pseudo_moves(
    queens: u64,
    chessboard: &Chessboard,
    color: &Color,
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    moves.append(&mut bishop::generate_pseudo_moves(queens, chessboard, color));
    moves.append(&mut rook::generate_pseudo_moves(queens, chessboard, color));

    moves
}
