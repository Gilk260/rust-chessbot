use crate::chessboard::Chessboard;

use bitboard::patterns::knight;

use utils::color::Color;
use utils::direction;

use super::*;

const KNIGHT_MOVES: [(i32, fn(u64) -> u64); 8] = [
    (direction::NOEA + direction::NORT, knight::no_no_ea),
    (direction::NOEA + direction::EAST, knight::no_ea_ea),
    (direction::SOEA + direction::EAST, knight::so_ea_ea),
    (direction::SOEA + direction::SOUT, knight::so_so_ea),
    (direction::NOWE + direction::NORT, knight::no_no_we),
    (direction::NOWE + direction::WEST, knight::no_we_we),
    (direction::SOWE + direction::WEST, knight::so_we_we),
    (direction::SOWE + direction::SOUT, knight::so_so_we),
];

pub fn generate_pseudo_moves(
    knights: u64,
    chessboard: &Chessboard,
    color: &Color,
    moves: &mut Vec<Move>,
) {
    let allies: u64 = chessboard.get_colors(color);

    if knights == 0 {
        return;
    }

    for (direction, f) in KNIGHT_MOVES.iter() {
        let targets: u64 = f(knights) & !allies;

        convert_bb_to_moves(chessboard, targets, -*direction, moves);
    }
}

#[cfg(test)]
mod tests {
    use utils::piece::Piece;
    use super::*;

    #[test]
    fn test_generate_pseudo_moves() {
        let chessboard = Chessboard::new("8/8/8/8/8/8/8/8 w - - 0 1".to_string());
        let color = Color::White;
        let knights = chessboard.get_pieces_color(&Piece::Knight, &color);
        let moves = &mut Vec::new();
        generate_pseudo_moves(knights, &chessboard, &color, moves);
        assert_eq!(moves.len(), 0);

        let chessboard = Chessboard::new("8/8/8/3N4/8/8/8/8 w - - 0 1".to_string());
        chessboard.pretty_print();
        let color = Color::White;
        let knights = chessboard.get_pieces_color(&Piece::Knight, &color);
        let moves = &mut Vec::new();
        generate_pseudo_moves(knights, &chessboard, &color, moves);
        assert_eq!(moves.len(), 8);
    }
}
