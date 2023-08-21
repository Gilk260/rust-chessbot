use crate::chessboard::Chessboard;

use utils::color::Color;
use utils::direction;
use utils::file::File;
use utils::rank::Rank;

use super::*;

const KING_MOVES: [(i32, fn(u64) -> u64); 8] = [
    (direction::NORT, bitboard::north_one),
    (direction::NOEA, bitboard::no_east_one),
    (direction::EAST, bitboard::east_one),
    (direction::SOEA, bitboard::so_east_one),
    (direction::SOUT, bitboard::south_one),
    (direction::SOWE, bitboard::so_west_one),
    (direction::WEST, bitboard::west_one),
    (direction::NOWE, bitboard::no_west_one),
];

pub fn generate_pseudo_moves(
    king: u64,
    chessboard: &Chessboard,
    color: &Color,
    moves: &mut Vec<Move>,
) {
    let allies = chessboard.get_colors(&color);

    if king == 0 {
        return;
    }

    for (direction, f) in KING_MOVES.iter() {
        let targets = f(king) & !allies;

        convert_bb_to_moves(chessboard, targets, -*direction, moves);
    }
}

fn generate_castle_move(chessboard: &Chessboard, square: &Square, color: &Color, queen_side: bool) -> Vec<Move> {
    let castle_ability = match color {
        Color::White => match queen_side {
            true => chessboard.white_castle.1,
            false => chessboard.white_castle.0,
        },
        Color::Black => match queen_side {
            true => chessboard.black_castle.1,
            false => chessboard.black_castle.0,
        },
    };

    if !castle_ability {
        return Vec::new()
    }

    let direction = match queen_side {
        true => direction::WEST,
        false => direction::EAST,
    };

    let dest_square = Square::from_u32((square.to_u32() as i32 + 2 * direction) as u32);
    let dest_square_to_check = match queen_side {
        true => Square::from_u32((dest_square.to_u32() as i32 + direction) as u32),
        false => dest_square,
    };

    let mut distance = 1;
    // Check if squares between king and rook are empty
    loop {
        let square_to_check = &Square::from_u32((square.to_u32() as i32 + direction * distance) as u32);
        if chessboard.get_piece(square_to_check) != None {
            return Vec::new()
        } else if square_to_check == &dest_square_to_check {
            break
        }
        distance += 1;
    }

    // Check if the squares between the king and the rook are not attacked
    for i in 0..3 {
        let square = Square::from_u32((square.to_u32() as i32 + i * direction) as u32);
        if chessboard.is_attacked_square(square.to_bitboard(), color) {
            return Vec::new()
        }
    }

    return vec![Move::new(*square, dest_square)]
}

pub fn generate_castling_moves(
    chessboard: &Chessboard,
    color: &Color,
) -> Vec<Move> {
    let mut moves = Vec::new();
    let file_king = File::E;
    let rank_king = match color {
        Color::White => Rank::One,
        Color::Black => Rank::Eight,
    };
    let square_king = Square::new(file_king, rank_king);

    moves.append(&mut generate_castle_move(chessboard, &square_king, color, false));
    moves.append(&mut generate_castle_move(chessboard, &square_king, color, true));

    moves
}

#[cfg(test)]
mod tests {
    use utils::piece::Piece;
    use super::*;

    #[test]
    fn test_generate_pseudo_moves() {
        let chessboard = Chessboard::new("8/8/8/8/8/8/8/8 w - - 0 1".to_string());
        let color = Color::White;
        let king = chessboard.get_pieces_color(&Piece::King, &color);
        let moves = &mut Vec::new();
        generate_pseudo_moves(king, &chessboard, &color, moves);
        assert_eq!(moves.len(), 0);

        let chessboard = Chessboard::new("8/8/8/3K4/8/8/8/8 w - - 0 1".to_string());
        let color = Color::White;
        let king = chessboard.get_pieces_color(&Piece::King, &color);
        let moves = &mut Vec::new();
        generate_pseudo_moves(king, &chessboard, &color, moves);
        assert_eq!(moves.len(), 8);

        let chessboard = Chessboard::new("8/8/8/8/8/8/8/KP6 w - - 0 1".to_string());
        let color = Color::White;
        let king = chessboard.get_pieces_color(&Piece::King, &color);
        let moves = &mut Vec::new();
        generate_pseudo_moves(king, &chessboard, &color, moves);
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&Move::new(Square::from_u32(0), Square::from_u32(8))));
        assert!(moves.contains(&Move::new(Square::from_u32(0), Square::from_u32(9))));
    }

    fn test_generate_castling_moves(fen: &str, color: &Color, expected_moves: usize) {
        let chessboard = Chessboard::new(fen.to_string());
        let moves = generate_castling_moves(&chessboard, &color);
        assert_eq!(moves.len(), expected_moves);
    }

    #[test]
    fn test_castling_moves() {
        test_generate_castling_moves("8/8/8/8/8/8/8/RN2K3 w Q - 0 1", &Color::White, 0);
        test_generate_castling_moves("8/8/8/8/8/8/8/4KB1R w K - 0 1", &Color::White, 0);
        test_generate_castling_moves("8/8/8/8/8/8/8/R1B1K3 w Q - 0 1", &Color::White, 0);

        test_generate_castling_moves("8/8/8/8/8/8/7p/4K2R w K - 0 1", &Color::White, 0);
        test_generate_castling_moves("8/8/8/8/8/4q3/8/4K2R w K - 0 1", &Color::White, 0);
        test_generate_castling_moves("8/8/8/4r3/8/8/8/4K2R w K - 0 1", &Color::White, 0);

        test_generate_castling_moves("8/8/8/8/8/8/8/4K2R w K - 0 1", &Color::White, 1);
        test_generate_castling_moves("8/8/8/8/8/8/8/R3K3 w Q - 0 1", &Color::White, 1);
        test_generate_castling_moves("8/8/8/8/8/8/8/4K2R w KQ - 0 1", &Color::White, 2);
        test_generate_castling_moves("4k2r/8/8/8/8/8/8/8 b k - 0 1", &Color::Black, 1);
        test_generate_castling_moves("r3k3/8/8/8/8/8/8/8 b q - 0 1", &Color::Black, 1);
        test_generate_castling_moves("4k2r/8/8/8/8/8/8/8 b kq - 0 1", &Color::Black, 2);

        test_generate_castling_moves("8/8/8/8/4b3/8/8/R3K3 w Q - 0 1", &Color::White, 1);
    }
}
