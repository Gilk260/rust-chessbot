pub mod king;
pub mod knight;
pub mod pawn;
pub mod sliding_piece;

use crate::chessboard::Chessboard;
use super::Move;

use utils::square::Square;

pub fn convert_bb_to_moves(
    chessboard: &Chessboard,
    bb: u64,
    relation: i32,
) -> Vec<Move> {
    let mut moves = Vec::new();
    let mut bb = bb;

    while bb != 0 {
        let sq = bb.trailing_zeros();
        let to = Square::from_u32(sq);
        let from = Square::from_u32((sq as i32 + relation) as u32);

        moves.append(&mut chessboard.generate_move(from, to));

        bb ^= 1 << sq;
    }

    moves
}

#[cfg(test)]
mod test {
    #[test]
    fn test_convert_bb_to_moves() {
        use utils::direction;
        use super::*;

        let chessboard = Chessboard::new("8/8/8/8/8/8/8/8 w - - 0 1".to_string());
        let bb = 0b00000000;
        let relation = 0;
        let moves = convert_bb_to_moves(&chessboard, bb, relation);
        assert_eq!(moves.len(), 0);

        let chessboard = Chessboard::new("8/8/8/8/8/8/8/8 w - - 0 1".to_string());
        let bb = 0b00000001_00000000_00000000;
        let relation = direction::SOUT;
        let moves = convert_bb_to_moves(&chessboard, bb, relation);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0], Move::new(Square::from_u32(8), Square::from_u32(16)));

        let chessboard = Chessboard::new("8/8/8/8/8/8/8/8 w - - 0 1".to_string());
        let bb = 0b10000001_00000000_00000000;
        let relation = direction::SOUT;
        let moves = convert_bb_to_moves(&chessboard, bb, relation);
        assert_eq!(moves.len(), 2);
        assert_eq!(moves[0], Move::new(Square::from_u32(8), Square::from_u32(16)));
        assert_eq!(moves[1], Move::new(Square::from_u32(15), Square::from_u32(23)));
    }
}
