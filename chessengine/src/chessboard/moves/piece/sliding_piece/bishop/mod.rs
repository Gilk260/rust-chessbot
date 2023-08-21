use bitboard::{bit_scan_forward, no_east_one, no_west_one, so_east_one, so_west_one};
use utils::{file::{NOT_A_FILE, NOT_H_FILE}, color::Color, direction::{NOEA, NOWE}};

use lazy_static::lazy_static;

use crate::chessboard::{moves::Move, Chessboard};

lazy_static! {
    pub static ref INIT_BISHOP: [u64; 64] = make_init_bishop();
    pub static ref MASK_BISHOP: [u64; 64] = make_mask_bishop();
    pub static ref STEP_BISHOP: [usize; 64] = [
        6, 5, 4, 3, 3, 4, 5, 6,
        5, 5, 4, 3, 3, 4, 5, 5,
        4, 4, 4, 3, 3, 4, 4, 4,
        3, 3, 3, 3, 3, 3, 3, 3,
        3, 3, 3, 3, 3, 3, 3, 3,
        4, 4, 4, 3, 3, 4, 4, 4,
        5, 5, 4, 3, 3, 4, 5, 5,
        6, 5, 4, 3, 3, 4, 5, 6,
    ];
    pub static ref INITIATED_BISHOP: bool = true;
}

fn make_init_bishop() -> [u64; 64] {
    let mut result: [u64; 64] = [0; 64];

    for sq in 0..64 {
        let bb: u64 = 1 << sq;

        result[sq] = (bb >> 9) & NOT_H_FILE;
        result[sq] |= (bb >> 7) & NOT_A_FILE;
        result[sq] |= (bb << 9) & NOT_A_FILE;
        result[sq] |= (bb << 7) & NOT_H_FILE;
    }

    result
}

fn make_mask_bishop() -> [u64; 64] {
    let mut result: [u64; 64] = [0; 64];

    for sq in 0..64 {
        result[sq] = 0;

        let mut i: i32 = sq as i32 - 9;
        while i >= 0 && i % 8 != 7 {
            result[sq] |= 1 << i;
            i -= 9;
        }

        let mut i: i32 = sq as i32 - 7;
        while i >= 0 && i % 8 != 0 {
            result[sq] |= 1 << i;
            i -= 7;
        }

        let mut i: i32 = sq as i32 + 9;
        while i < 64 && i % 8 != 0 {
            result[sq] |= 1 << i;
            i += 9;
        }

        let mut i: i32 = sq as i32 + 7;
        while i < 64 && i % 8 != 7 {
            result[sq] |= 1 << i;
            i += 7;
        }
    }

    result
}

pub fn generate_pseudo_moves(
    bishops: u64,
    chessboard: &Chessboard,
    color: &Color,
    moves: &mut Vec<Move>,
) {
    let mut bb = bishops;
    let allies = chessboard.get_colors(&color);

    while bb != 0 {
        let square = bit_scan_forward(bb) as usize;

        chessboard.generate_bishop_moves(square, allies, moves);

        bb ^= 1 << square;
    }
}

impl Chessboard {
    pub fn generate_bishop_moves(&self,
                                 square: usize,
                                 allies: u64,
                                 moves: &mut Vec<Move>) {
        let sliding = super::SlidingPiece {
            mask: MASK_BISHOP[square],
            init: INIT_BISHOP[square],
            step: STEP_BISHOP[square],
            directions: [
                no_east_one,
                no_west_one,
                so_east_one,
                so_west_one,
            ].to_vec(),
        };

        self.generate_sliding_piece_moves(square, allies, &sliding, moves);
    }
}


// Compute all square attacked by the bishop
fn get_bishop_targets(
    square: usize,
    empty: u64,
) -> u64 {
    let mask = MASK_BISHOP[square];
    let mut bb = INIT_BISHOP[square];

    let mut at = bb;
    bb &= empty;

    for _ in 0..STEP_BISHOP[square] {
        let mut noea = bb;
        noea <<= NOEA;
        noea &= mask;
        at |= noea;

        let mut nowe = bb;
        nowe <<= NOWE;
        nowe &= mask;
        at |= nowe;

        let mut soea = bb;
        soea >>= NOEA;
        soea &= mask;
        at |= soea;

        let mut sowe = bb;
        sowe >>= NOWE;
        sowe &= mask;
        at |= sowe;

        bb = noea | nowe | soea | sowe;
        bb &= empty;
    }

    at
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bishop_targets() {
        let square = 0;
        let empty: u64 = 0xFF_FF_FF_FF_FF_FF_FF_FF;
        let expected: u64 = 0x8040201008040200;
        assert_eq!(get_bishop_targets(square, empty), expected);

        let square = 59;
        let empty: u64 = 0xFF_FF_FF_FF_FF_FF_FF_FF;
        let expected: u64 = 0x14224180000000;
        assert_eq!(get_bishop_targets(square, empty), expected);

        let square = 59;
        let empty: u64 = 0xFF_FF_FD_FF_FF_FF_FF_FF;
        let expected: u64 = 0x14224080000000;
        assert_eq!(get_bishop_targets(square, empty), expected);

        let square = 59;
        let empty: u64 = 0xf7fffdbfffffffff;
        let expected: u64 = 0x14224000000000;
        assert_eq!(get_bishop_targets(square, empty), expected);
    }

    use utils::{piece::Piece, square::Square};

    #[test]
    fn test_generate_moves() {
        let piece = Piece::Bishop;
        let color = &Color::White;

        let chessboard = Chessboard::new("8/8/8/8/8/8/8/8 w - - 0 1".to_string());
        let bishops: u64 = chessboard.get_pieces_color(&piece, color);
        let moves = &mut Vec::new();
        generate_pseudo_moves(bishops, &chessboard, color, moves);
        assert!(moves.is_empty());

        let chessboard = Chessboard::new("8/8/8/8/8/8/8/B7 w - - 0 1".to_string());
        let bishops: u64 = chessboard.get_pieces_color(&piece, color);
        let moves = &mut Vec::new();
        generate_pseudo_moves(bishops, &chessboard, color, moves);
        assert_eq!(moves.len(), 7, "\n{:?}", moves);
        assert!(moves.contains(&Move::new(Square::from_string("a1"), Square::from_string("b2"))));
        assert!(moves.contains(&Move::new(Square::from_string("a1"), Square::from_string("h8"))));

        let chessboard = Chessboard::new("8/8/8/8/8/8/1b6/B7 w - - 0 1".to_string());
        let bishops: u64 = chessboard.get_pieces_color(&piece, color);
        let moves = &mut Vec::new();
        generate_pseudo_moves(bishops, &chessboard, color, moves);
        let mut expected = Move::new(Square::from_string("a1"), Square::from_string("b2"));
        expected.capture = Some(Piece::Bishop);
        assert_eq!(moves.len(), 1);
        assert!(moves.contains(&expected));

        let chessboard = Chessboard::new("8/8/8/8/8/8/1P6/B7 w - - 0 1".to_string());
        let bishops: u64 = chessboard.get_pieces_color(&piece, color);
        let moves = &mut Vec::new();
        generate_pseudo_moves(bishops, &chessboard, color, moves);
        assert_eq!(moves.len(), 0);

        let chessboard = Chessboard::new("8/8/8/8/4B3/8/8/8 w - - 0 1".to_string());
        let bishops: u64 = chessboard.get_pieces_color(&piece, color);
        let moves = &mut Vec::new();
        generate_pseudo_moves(bishops, &chessboard, color, moves);
        for mv in moves.iter() {
            eprintln!("{}", mv.to_string());
        }
        assert_eq!(moves.len(), 13);
    }
}
