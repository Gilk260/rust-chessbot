use bitboard::bit_scan_forward;
use utils::square::Square;

use crate::chessboard::Chessboard;

use super::Move;

pub mod bishop;
pub mod rook;
pub mod queen;

pub struct SlidingPiece {
    pub mask: u64,
    pub init: u64,
    pub step: usize,
    pub directions: Vec<fn(u64) -> u64>,
}

pub fn convert_bb_to_moves(
    chessboard: &Chessboard,
    bb: u64,
    from: Square,
    moves: &mut Vec<Move>,
) {
    let mut bb: u64 = bb;

    while bb != 0 {
        let square = bit_scan_forward(bb);
        let to = Square::from_u32(square);

        moves.push(chessboard.generate_move(from, to));
        bb ^= 1 << square;
    }
}

impl Chessboard {
    pub fn generate_sliding_piece_moves(&self,
                                        square: usize,
                                        allies: u64,
                                        sliding: &SlidingPiece,
                                        moves: &mut Vec<Move>
                                       ) {
        let mask = sliding.mask;
        let init = sliding.init & !allies;
        let step = sliding.step;
        let directions = &sliding.directions;
        let from = Square::from_u32(square as u32);

        convert_bb_to_moves(self, init, from, moves);

        for direction in directions {
            let mut ray = init & self.empty_board;

            for _ in 0..step {
                ray = direction(ray);

                ray &= mask;

                ray &= !allies;

                if ray != 0 {
                    moves.push(self.convert_bb_to_move(from, ray));
                }

                ray &= self.empty_board;

                if ray == 0 {
                    break;
                }
            }
        }
    }

    pub fn convert_bb_to_move(&self, from: Square, bb: u64) -> Move {
        self.generate_move(from, Square::from_u32(bit_scan_forward(bb)))
    }
}
