use utils::{color::Color, piece::Piece};

use crate::chessboard::Chessboard;

pub fn material_balance(chessboard: &Chessboard, color: &Color) -> i32 {
    let mut score: i32 = 0;

    score += compute_balance_piece(chessboard, &Piece::Pawn, color);
    score += compute_balance_piece(chessboard, &Piece::Knight, color);
    score += compute_balance_piece(chessboard, &Piece::Bishop, color);
    score += compute_balance_piece(chessboard, &Piece::Rook, color);
    score += compute_balance_piece(chessboard, &Piece::Queen, color);
    score += compute_balance_piece(chessboard, &Piece::King, color);

    score
}

fn compute_balance_piece(chessboard: &Chessboard, piece: &Piece, color: &Color) -> i32 {
    let opposite = chessboard.get_opposite_color(&color);

    let allies = chessboard.get_pieces_color(&piece, &color);
    let enemies = chessboard.get_pieces_color(&piece, &opposite);

    (allies.count_ones() as i32 - enemies.count_ones() as i32) * piece_value(piece)
}

pub fn piece_value(piece: &Piece) -> i32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 350,
        Piece::Bishop => 350,
        Piece::Rook => 525,
        Piece::Queen => 1000,
        Piece::King => 2000,
    }
}
