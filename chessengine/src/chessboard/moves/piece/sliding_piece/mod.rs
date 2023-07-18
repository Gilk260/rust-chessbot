use bitboard::bit_scan_forward;
use utils::square::Square;

use crate::chessboard::Chessboard;

use super::Move;

pub mod bishop;
pub mod rook;
pub mod queen;

pub fn convert_bb_to_moves(
    chessboard: &Chessboard,
    bb: u64,
    from: Square,
) -> Vec<Move> {
    let mut bb: u64 = bb;
    let mut moves: Vec<Move> = Vec::new();

    while bb != 0 {
        let square = bit_scan_forward(bb);
        let to = Square::from_u32(square);

        moves.append(&mut chessboard.generate_move(from, to));
        bb ^= 1 << square;
    }

    moves
}
