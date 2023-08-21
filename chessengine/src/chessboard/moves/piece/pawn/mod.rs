use crate::chessboard::Chessboard;

use bitboard::patterns::pawn;

use utils::color::Color;
use utils::direction;

use super::*;

pub fn generate_pseudo_moves(
    pawns: u64,
    chessboard: &Chessboard,
    color: &Color,
    moves: &mut Vec<Move>,
) {
    let enemies = chessboard.get_colors(&chessboard.turn.opposite());

    moves.append(&mut generate_single_push_moves(pawns, chessboard, color));
    moves.append(&mut generate_double_push_moves(pawns, chessboard, color));

    let ep: u64 = match chessboard.en_passant {
        Some(ep) => ep.to_bitboard(),
        None => 0,
    };

    moves.append(&mut generate_capture_moves(pawns, &(enemies | ep), chessboard, color));
}

fn generate_single_push_moves(
    pawns: u64,
    chessboard: &Chessboard,
    color: &Color,
) -> Vec<Move> {
    let mut moves = Vec::new();
    let single_push_targets: u64 = pawn::single_push_targets(pawns, chessboard.empty_board, color);
    // println!("single_push_targets: {:b}", single_push_targets);
    let direction = if color == &Color::White { direction::SOUT } else { direction::NORT };

    convert_bb_to_moves(chessboard, single_push_targets, direction, &mut moves);

    moves
}

fn generate_double_push_moves(
    pawns: u64,
    chessboard: &Chessboard,
    color: &Color,
) -> Vec<Move> {
    let mut moves = Vec::new();
    let double_push_targets: u64 = pawn::double_push_targets(pawns, chessboard.empty_board, color);
    let direction = if color == &Color::White { direction::SOUT } else { direction::NORT };

    convert_bb_to_moves(chessboard, double_push_targets, direction * 2, &mut moves);

    moves
}

pub fn generate_capture_moves(
    pawns: u64,
    enemies: &u64,
    chessboard: &Chessboard,
    color: &Color,
) -> Vec<Move> {
    let west_attack_targets: u64 = pawn::west_attack_targets(pawns, color) & enemies;
    let east_attack_targets: u64 = pawn::east_attack_targets(pawns, color) & enemies;

    let west_direction = if color == &Color::White { direction::SOEA } else { direction::NOEA };
    let east_direction = if color == &Color::White { direction::SOWE } else { direction::NOWE };

    let mut moves = Vec::new();

    convert_bb_to_moves(chessboard, west_attack_targets, west_direction, &mut moves);
    convert_bb_to_moves(chessboard, east_attack_targets, east_direction, &mut moves);

    moves
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    #[test]
    fn test_generate_single_push_moves() {
        use super::*;
        use utils::piece::Piece;

        let chessboard = Chessboard::new("8/8/8/8/8/8/8/8 w - - 0 1".to_string());
        let color = Color::White;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let moves = generate_single_push_moves(pawns, &chessboard, &color);
        assert_eq!(moves.len(), 0);

        let chessboard = Chessboard::new("8/8/8/8/8/8/P7/8 w - - 0 1".to_string());
        let color = Color::White;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let moves = generate_single_push_moves(pawns, &chessboard, &color);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0], Move::new(Square::from_u32(8), Square::from_u32(16)));

        let chessboard = Chessboard::new("8/8/8/8/8/p7/P7/8 w - - 0 1".to_string());
        let color = Color::White;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let moves = generate_single_push_moves(pawns, &chessboard, &color);
        assert_eq!(moves.len(), 0);

        let chessboard = Chessboard::new("8/p7/8/8/8/8/8/8 w - - 0 1".to_string());
        let color = Color::Black;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let moves = generate_single_push_moves(pawns, &chessboard, &color);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0], Move::new(Square::from_u32(48), Square::from_u32(40)));

        let chessboard = Chessboard::new("rnbqkbnr/1ppppppp/8/p7/P7/8/1PPPPPPP/RNBQKBNR w - a6 0 1".to_string());
        let color = Color::White;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let moves = generate_single_push_moves(pawns, &chessboard, &color);
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn test_generate_double_push_moves() {
        use super::*;
        use utils::piece::Piece;

        let chessboard = Chessboard::new("8/8/8/8/8/8/8/8 w - - 0 1".to_string());
        let color = Color::White;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let moves = generate_double_push_moves(pawns, &chessboard, &color);
        assert_eq!(moves.len(), 0);

        let chessboard = Chessboard::new("8/8/8/8/8/8/P7/8 w - - 0 1".to_string());
        let color = Color::White;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let moves = generate_double_push_moves(pawns, &chessboard, &color);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0], Move::new(Square::from_u32(8), Square::from_u32(24)));

        let chessboard = Chessboard::new("8/8/8/8/8/p7/P7/8 w - - 0 1".to_string());
        let color = Color::White;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let moves = generate_double_push_moves(pawns, &chessboard, &color);
        assert_eq!(moves.len(), 0);

        let chessboard = Chessboard::new("rnbqkbnr/1ppppppp/8/p7/P7/8/1PPPPPPP/RNBQKBNR w - a6 0 1".to_string());
        let color = Color::White;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let moves = generate_double_push_moves(pawns, &chessboard, &color);
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn test_generate_capture_moves() {
        use super::*;
        use utils::piece::Piece;

        let chessboard = Chessboard::new("8/8/8/8/8/8/8/8 w - - 0 1".to_string());
        let color = Color::White;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let enemies = chessboard.get_colors(&Color::Black);
        let moves = generate_capture_moves(pawns, &enemies, &chessboard, &color);
        assert_eq!(moves.len(), 0);

        let chessboard = Chessboard::new("8/8/8/8/8/p7/P7/8 w - - 0 1".to_string());
        let color = Color::White;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let enemies = chessboard.get_colors(&Color::Black);
        let moves = generate_capture_moves(pawns, &enemies, &chessboard, &color);
        assert_eq!(moves.len(), 0);

        let chessboard = Chessboard::new("8/8/8/8/8/1p6/P7/8 w - - 0 1".to_string());
        let color = Color::White;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let enemies = chessboard.get_colors(&Color::Black);
        let moves = generate_capture_moves(pawns, &enemies, &chessboard, &color);
        assert_eq!(moves.len(), 1);
        let mut expected = Move::new(Square::from_u32(8), Square::from_u32(17));
        expected.capture = Some(Piece::Pawn);
        assert_eq!(moves[0], expected);

        let chessboard = Chessboard::new("8/8/8/8/8/p1p5/1P6/8 w - - 0 1".to_string());
        let color = Color::White;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let enemies = chessboard.get_colors(&Color::Black);
        let moves = generate_capture_moves(pawns, &enemies, &chessboard, &color);
        assert_eq!(moves.len(), 2);
        let mut expected = Move::new(Square::from_u32(9), Square::from_u32(16));
        expected.capture = Some(Piece::Pawn);
        assert!(moves.contains(&expected));
        let mut expected = Move::new(Square::from_u32(9), Square::from_u32(18));
        expected.capture = Some(Piece::Pawn);
        assert!(moves.contains(&expected));

        let chessboard = Chessboard::new("rnbqkbnr/8/8/p7/P7/8/8/RNBQKBNR w - - 0 1".to_string());
        let color = Color::White;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let enemies = chessboard.get_colors(&Color::Black);
        let moves = generate_capture_moves(pawns, &enemies, &chessboard, &color);
        assert_eq!(moves.len(), 0);

        let chessboard = Chessboard::new("rnbqkbnr/1ppppppp/8/p7/P7/8/1PPPPPPP/RNBQKBNR w - a6 0 1".to_string());
        let color = Color::White;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let enemies = chessboard.get_colors(&Color::Black);
        let moves = generate_capture_moves(pawns, &enemies, &chessboard, &color);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn test_generate_capture_ep_moves() {
        use super::*;
        use utils::piece::Piece;

        let chessboard = Chessboard::new("8/8/8/pP6/8/8/8/8 w - a6 0 1".to_string());
        let color = Color::White;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let mut enemies = chessboard.get_colors(&Color::Black);
        enemies |= chessboard.en_passant.unwrap().to_bitboard();
        let moves = generate_capture_moves(pawns, &enemies, &chessboard, &color);
        assert_eq!(moves.len(), 1);
        let mut expected = Move::new(Square::from_u32(33), Square::from_u32(40));
        expected.capture = Some(Piece::Pawn);
        assert_eq!(moves[0], expected);
    }

    fn test_pawns_generate_pseudo_move(fen: &str, expected: Vec<&str>) {
        use super::*;
        use utils::piece::Piece;

        let chessboard = Chessboard::new(fen.to_string());

        let mut expected_moves = Vec::new();

        for mv in expected {
            expected_moves.push(chessboard.generate_move_from_string(mv.to_string()));
        }
        expected_moves.sort();

        let color = chessboard.turn;
        let pawns = chessboard.get_pieces_color(&Piece::Pawn, &color);
        let enemies = chessboard.get_colors(&chessboard.get_opposite_color(&color));
        let mut moves = Vec::new();
        generate_pseudo_moves(pawns, &chessboard, &color, &mut moves);
        moves.sort();
        assert_eq!(moves.len(), expected_moves.len());
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn test_pawns_moves_generation() {
        test_pawns_generate_pseudo_move("rnbqkbnr/2pppppp/p7/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 4 3 1",
                                        vec!["b2b3", "b2b4", "c2c3", "c2c4", "d2d3", "d2d4", "e2e3", "e2e4", "f2f3",
                                             "f2f4", "g2g3", "g2g4", "h2h3", "h2h4", "a5b6"]);
    }
}
