pub const RANKS: [u64; 8] = [
    0x0000_0000_0000_00FF,
    0x0000_0000_0000_FF00,
    0x0000_0000_00FF_0000,
    0x0000_0000_FF00_0000,
    0x0000_00FF_0000_0000,
    0x0000_FF00_0000_0000,
    0x00FF_0000_0000_0000,
    0xFF00_0000_0000_0000,
];

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    pub fn from_u32(num: u32) -> Rank {
        match num {
            0 => Rank::One,
            1 => Rank::Two,
            2 => Rank::Three,
            3 => Rank::Four,
            4 => Rank::Five,
            5 => Rank::Six,
            6 => Rank::Seven,
            7 => Rank::Eight,
            _ => panic!("Invalid rank: {}", num),
        }
    }

    pub fn from_char(c: char) -> Rank {
        match c {
            '1' => Rank::One,
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            _ => panic!("Invalid rank: {}", c),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Rank::One => '1',
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_from_u32() {
        assert_eq!(Rank::from_u32(0), Rank::One);
        assert_eq!(Rank::from_u32(1), Rank::Two);
        assert_eq!(Rank::from_u32(2), Rank::Three);
        assert_eq!(Rank::from_u32(3), Rank::Four);
        assert_eq!(Rank::from_u32(4), Rank::Five);
        assert_eq!(Rank::from_u32(5), Rank::Six);
        assert_eq!(Rank::from_u32(6), Rank::Seven);
        assert_eq!(Rank::from_u32(7), Rank::Eight);
    }
}
