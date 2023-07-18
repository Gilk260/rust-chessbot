use super::file::File;
use super::rank::Rank;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}

impl Square {
    pub fn new(file: File, rank: Rank) -> Square {
        Square {
            file,
            rank,
        }
    }

    pub fn from_u32(num: u32) -> Square {
        let file = File::from_u32(num % 8);
        let rank = Rank::from_u32(num / 8);
        Square::new(file, rank)
    }

    pub fn from_string(string: &str) -> Square {
        let file = File::from_char(string[0..1].chars().collect::<Vec<char>>()[0]);
        let rank = Rank::from_char(string[1..2].chars().collect::<Vec<char>>()[0]);
        Square::new(file, rank)
    }

    pub fn to_u32(&self) -> u32 {
        self.file as u32 + self.rank as u32 * 8
    }

    pub fn to_bitboard(&self) -> u64 {
        1 << self.to_u32()
    }

    pub fn to_string(&self) -> String {
        format!("{}{}", self.file.to_char(), self.rank.to_char())
    }
}
