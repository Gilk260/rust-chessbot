use std::println;

use bitboard::bit_scan_forward;
use lazy_static::lazy_static;
use utils::{file::{NOT_H_FILE, NOT_A_FILE}, direction::{NORT, EAST}, color::Color, square::Square};

use crate::chessboard::{Chessboard, moves::Move};

use super::convert_bb_to_moves;

lazy_static! {
    pub static ref INIT_ROOK: [u64; 64] = make_init_rook();
    pub static ref MASK_ROOK: [u64; 64] = make_mask_rook();
    pub static ref STEP_ROOK: [usize; 64] = [
        7, 6, 6, 6, 6, 6, 6, 7,
        7, 5, 5, 5, 5, 5, 5, 7,
        7, 5, 4, 4, 4, 4, 5, 7,
        7, 5, 4, 3, 3, 4, 5, 7,
        7, 5, 4, 3, 3, 4, 5, 7,
        7, 5, 4, 4, 4, 4, 5, 7,
        7, 5, 5, 5, 5, 5, 5, 7,
        7, 6, 6, 6, 6, 6, 6, 7,
    ];
    pub static ref INITIATED_ROOK: bool = true;
}

fn make_init_rook() -> [u64; 64] {
    let mut result = [0u64; 64];

    for sq in 0..64 {
        let bb: u64 = 1 << sq;

        result[sq] = bb >> 8;
        result[sq] |= (bb >> 1) & NOT_H_FILE;
        result[sq] |= (bb << 1) & NOT_A_FILE;
        result[sq] |= bb << 8;
    }

    result
}

fn make_mask_rook() -> [u64; 64] {
    let mut result = [0u64; 64];

    for sq in 0..64 {
        let mut i: i32 = sq as i32 - 8;
        while i >= 0 {
            result[sq] |= 1 << i;
            i -= 8;
        }

        i = sq as i32 - 1;
        while i >= 0 && (i & 7) != 7 {
            result[sq] |= 1 << i;
            i -= 1;
        }

        i = sq as i32 + 1;
        while i < 64 && (i & 7) != 0 {
            result[sq] |= 1 << i;
            i += 1;
        }

        i = sq as i32 + 8;
        while i < 64 {
            result[sq] |= 1 << i;
            i += 8;
        }
    }

    result
}

pub fn generate_pseudo_moves(
    rooks: u64,
    chessboard: &Chessboard,
    color: &Color,
) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let mut bb = rooks;
    let allies = chessboard.get_colors(color);
    let empty = chessboard.empty_board;

    while bb != 0 {
        let square = bit_scan_forward(bb) as usize;
        let targets = get_rook_targets(square, empty) & !allies;

        moves.append(&mut convert_bb_to_moves(chessboard, targets, Square::from_u32(square as u32)));

        bb ^= 1 << square;
    }

    moves
}

fn get_rook_targets(
    square: usize,
    empty: u64,
) -> u64 {
    let mask = MASK_ROOK[square];
    let mut bb = INIT_ROOK[square];

    let mut at = bb;
    bb &= empty;

    for _ in 0..STEP_ROOK[square] {
        let mut up = bb;
        up <<= NORT;
        up &= mask;
        at |= up;

        let mut east = bb;
        east <<= EAST;
        east &= NOT_A_FILE;
        east &= mask;
        at |= east;


        let mut down = bb;
        down >>= NORT;
        down &= mask;
        at |= down;

        let mut west = bb;
        west >>= EAST;
        west &= NOT_H_FILE;
        west &= mask;
        at |= west;

        bb = up | east | down | west;
        bb &= empty;
    }

    at
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_rook_targets() {
        let square = 0;
        let empty: u64 = 0xFFFFFFFFFFFFFFFF;
        let expected: u64 = 0x1010101010101fe;
        assert_eq!(get_rook_targets(square, empty), expected);

        let square = 20;
        let empty: u64 = 0xFFFFFFFFFFFFFFFF;
        let expected: u64 = 0x1010101010ef1010;
        assert_eq!(get_rook_targets(square, empty), expected);

        let square = 20;
        let empty: u64 = 0xfffffffffff7ffff;
        let expected: u64 = 0x1010101010e81010;
        assert_eq!(get_rook_targets(square, empty), expected);
    }

    use utils::piece::Piece;

    #[test]
    fn test_generate_pseudo_moves() {
        let piece = Piece::Rook;
        let color = &Color::White;

        let chessboard = Chessboard::new("8/8/8/8/8/8/8/8 w - - 0 1".to_string());
        let rooks = chessboard.get_pieces_color(&piece, color);
        let moves = generate_pseudo_moves(rooks, &chessboard, color);
        assert_eq!(moves.len(), 0);

        let chessboard = Chessboard::new("8/8/8/8/8/8/8/R7 w - - 0 1".to_string());
        let rooks = chessboard.get_pieces_color(&piece, color);
        let moves = generate_pseudo_moves(rooks, &chessboard, color);
        assert_eq!(moves.len(), 14);
        assert!(moves.contains(&Move::new(Square::from_string("a1"), Square::from_string("a2"))));

        let chessboard = Chessboard::new("8/8/2p5/8/8/2R5/8/8 w - - 0 1".to_string());
        let rooks = chessboard.get_pieces_color(&piece, color);
        let moves = generate_pseudo_moves(rooks, &chessboard, color);
        let mut expected = Move::new(Square::from_string("c3"), Square::from_string("c6"));
        expected.capture = Some(Piece::Pawn);
        assert!(moves.contains(&expected));

    }
}
