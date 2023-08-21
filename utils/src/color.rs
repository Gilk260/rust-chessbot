#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn to_char(&self) -> char {
        match self {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 1,
        }
    }

    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
