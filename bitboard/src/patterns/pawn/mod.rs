use utils::color::Color;
use utils::rank;

use crate::north_one;
use crate::south_one;
use crate::no_east_one;
use crate::no_west_one;
use crate::so_east_one;
use crate::so_west_one;

pub fn single_push_targets(pawns: u64, empty: u64, color: &Color) -> u64 {
    if color == &Color::White {
        north_one(pawns) & empty
    } else {
        south_one(pawns) & empty
    }
}

#[test]
fn test_single_push_targets() {
    let pawns = 0b10000000_00000000;
    let empty = !pawns;
    assert_eq!(single_push_targets(pawns, empty, &Color::White), 0b10000000_00000000_00000000);
    let empty = 0;
    assert_eq!(single_push_targets(pawns, empty, &Color::Black), 0);
}

pub fn double_push_targets(pawns: u64, empty: u64, color: &Color) -> u64 {
    let single_pushes = single_push_targets(pawns, empty, color);
    if color == &Color::White {
        north_one(single_pushes) & empty & rank::RANKS[rank::Rank::Four as usize]
    } else {
        south_one(single_pushes) & empty & rank::RANKS[rank::Rank::Five as usize]
    }
}

#[test]
fn test_double_push_targets() {
    let pawns = 0b10000000_00000000;
    let empty = !pawns;
    assert_eq!(double_push_targets(pawns, empty, &Color::White), 0b10000000_00000000_00000000_00000000);
    let empty = 0;
    assert_eq!(double_push_targets(pawns, empty, &Color::Black), 0);
}

pub fn west_attack_targets(pawns: u64, color: &Color) -> u64 {
    if color == &Color::White {
        no_west_one(pawns)
    } else {
        so_west_one(pawns)
    }
}

#[test]
fn test_west_attack_targets() {
    let pawns = 0b00000001_00000000;
    assert_eq!(west_attack_targets(pawns, &Color::White), 0b00000000_00000000);

    let pawns = 0b00000001_00000000_00000000;
    assert_eq!(west_attack_targets(pawns, &Color::Black), 0b00000000_00000000);
}

pub fn east_attack_targets(pawns: u64, color: &Color) -> u64 {
    if color == &Color::White {
        no_east_one(pawns)
    } else {
        so_east_one(pawns)
    }
}

#[test]
fn test_east_attack_targets() {
    let pawns = 0b00000001_00000000;
    assert_eq!(east_attack_targets(pawns, &Color::White), 0b00000010_00000000_00000000);

    let pawns = 0b00000001_00000000_00000000;
    assert_eq!(east_attack_targets(pawns, &Color::Black), 0b00000010_00000000);
}
