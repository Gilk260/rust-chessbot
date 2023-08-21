pub mod patterns;

use utils::{file, direction::{NORT, EAST, NOEA, NOWE}};

pub fn is_empty(bb: u64) -> bool {
    bb == 0
}

// Get index of least significant bit
pub fn bit_scan_reverse(bb: u64) -> u32 {
    bb.trailing_zeros()
}

// Get index of most significant bit
pub fn bit_scan_forward(bb: u64) -> u32 {
    63 - bb.leading_zeros()
}

pub fn flip_vertical(bb: u64) -> u64 {
    bb.swap_bytes()
}

pub fn count_bits(bb: u64) -> u32 {
    bb.count_ones()
}

pub fn south_one(b: u64) -> u64 {
    b >> NORT
}

pub fn north_one(b: u64) -> u64 {
    b << NORT
}

pub fn east_one(b: u64) -> u64 {
    (b << EAST) & file::NOT_A_FILE
}

pub fn no_east_one(b: u64) -> u64 {
    (b << NOEA) & file::NOT_A_FILE
}

pub fn so_east_one(b: u64) -> u64 {
    (b >> NOWE) & file::NOT_A_FILE
}

pub fn west_one(b: u64) -> u64 {
    (b >> EAST) & file::NOT_H_FILE
}

pub fn no_west_one(b: u64) -> u64 {
    (b << NOWE) & file::NOT_H_FILE
}

pub fn so_west_one(b: u64) -> u64 {
    (b >> NOEA) & file::NOT_H_FILE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        assert!(is_empty(0));
        assert!(!is_empty(1));
    }

    #[test]
    fn test_bit_scan_reverse() {
        assert_eq!(bit_scan_reverse(0b01), 0);
        assert_eq!(bit_scan_reverse(0b1_000), 3);
    }

    #[test]
    fn test_bit_scan_forward() {
        assert_eq!(bit_scan_forward(0b01), 0);
        assert_eq!(bit_scan_forward(0b1_000), 3);
    }

    #[test]
    fn test_flip_vertical() {
        assert_eq!(
            flip_vertical(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000000),
                          0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
        );
        assert_eq!(
            flip_vertical(
                0b01111000_01000100_01000100_01001000_01110000_01010000_01001000_01000100),
                0b01000100_01001000_01010000_01110000_01001000_01000100_01000100_01111000
        );
    }
}

