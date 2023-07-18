pub const FILES: [u64; 8] = [
    0x0101010101010101,
    0x0202020202020202,
    0x0404040404040404,
    0x0808080808080808,
    0x1010101010101010,
    0x2020202020202020,
    0x4040404040404040,
    0x8080808080808080,
];

pub const NOT_A_FILE: u64 = 0xfefefefefefefefe;
pub const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn from_u32(num: u32) -> File {
        match num {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("Invalid file: {}", num)
        }
    }

    pub fn from_char(c: char) -> File {
        match c {
            'a' => File::A,
            'b' => File::B,
            'c' => File::C,
            'd' => File::D,
            'e' => File::E,
            'f' => File::F,
            'g' => File::G,
            'h' => File::H,
            _ => panic!("Invalid file: {}", c)
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_from_u32() {
        assert_eq!(File::from_u32(0), File::A);
        assert_eq!(File::from_u32(1), File::B);
        assert_eq!(File::from_u32(2), File::C);
        assert_eq!(File::from_u32(3), File::D);
        assert_eq!(File::from_u32(4), File::E);
        assert_eq!(File::from_u32(5), File::F);
        assert_eq!(File::from_u32(6), File::G);
        assert_eq!(File::from_u32(7), File::H);
    }
}
