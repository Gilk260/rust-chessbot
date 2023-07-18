use crate::bit_scan_reverse;

use utils::square::Square;

pub mod knight;
pub mod pawn;

pub fn bb_to_squares(bb: u64) -> Vec<Square> {
    let mut squares = Vec::new();
    let mut bb = bb;
    while bb != 0 {
        let sq = bit_scan_reverse(bb);
        squares.push(Square::from_u32(sq));
        bb ^= 1 << sq;
    }
    squares
}

#[test]
fn test_bb_to_squares() {
    let bb = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001;
    let squares = bb_to_squares(bb);
    assert_eq!(squares.len(), 1);
    assert_eq!(squares[0], Square::from_u32(0));

    let bb = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000001;
    let squares = bb_to_squares(bb);
    assert_eq!(squares.len(), 2);
    assert_eq!(squares[0], Square::from_u32(0));
    assert_eq!(squares[1], Square::from_u32(8));

    let bb = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
    let squares = bb_to_squares(bb);
    assert_eq!(squares.len(), 0);
}
