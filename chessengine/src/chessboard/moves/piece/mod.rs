pub mod king;
pub mod knight;
pub mod pawn;
pub mod sliding_piece;

use crate::chessboard::Chessboard;
use super::Move;

use utils::{square::Square, piece::Piece};

pub fn convert_bb_to_moves(
    chessboard: &Chessboard,
    bb: u64,
    relation: i32,
    moves: &mut Vec<Move>) {
    let mut bb = bb;

    while bb != 0 {
        let sq = bb.trailing_zeros();
        let to = Square::from_u32(sq);
        let from = Square::from_u32((sq as i32 + relation) as u32);

        let mv = chessboard.generate_move(from, to);

        // Handle promotion
        if let Some(piece) = chessboard.get_piece(&mv.from) {
            if piece == Piece::Pawn {
                chessboard.generate_promotion_moves(moves, mv);
            }
            else {
                moves.push(mv);
            }
        }
        else {
            moves.push(mv);
        }

        bb ^= 1 << sq;
    }
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
        let moves = &mut Vec::new();
        convert_bb_to_moves(&chessboard, bb, relation, moves);
        assert_eq!(moves.len(), 0);

        let chessboard = Chessboard::new("8/8/8/8/8/8/8/8 w - - 0 1".to_string());
        let bb = 0b00000001_00000000_00000000;
        let relation = direction::SOUT;
        let moves = &mut Vec::new();
        convert_bb_to_moves(&chessboard, bb, relation, moves);
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0], Move::new(Square::from_u32(8), Square::from_u32(16)));

        let chessboard = Chessboard::new("8/8/8/8/8/8/8/8 w - - 0 1".to_string());
        let bb = 0b10000001_00000000_00000000;
        let relation = direction::SOUT;
        let moves = &mut Vec::new();
        convert_bb_to_moves(&chessboard, bb, relation, moves);
        assert_eq!(moves.len(), 2);
        assert_eq!(moves[0], Move::new(Square::from_u32(8), Square::from_u32(16)));
        assert_eq!(moves[1], Move::new(Square::from_u32(15), Square::from_u32(23)));
    }
}
