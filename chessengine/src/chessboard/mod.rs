use std::collections::HashMap;

use utils::color::Color;
use utils::piece::Piece;

use utils::rank::Rank;
use utils::file::File;
use utils::square::Square;

use self::moves::Move;

pub mod moves;
pub mod perft;

#[derive(PartialEq, Eq, Debug)]
pub struct Chessboard {
    piece_board: HashMap<Piece, u64>,
    color_board: HashMap<Color, u64>,
    pub empty_board: u64,
    pub turn: Color,
    pub white_castle: (bool, bool),
    pub black_castle: (bool, bool),
    pub en_passant: Option<Square>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
    pub perft_depth: u8,
    wc_stack: Vec<(bool, bool)>,
    bc_stack: Vec<(bool, bool)>,
    ep_stack: Vec<Option<Square>>,
    hm_stack: Vec<u8>,
    pub mv_hashmap: HashMap<String, u32>,
}

impl Chessboard {
    pub fn new(perft_string: String) -> Chessboard {
        let perft_parts: Vec<&str> = perft_string.split(' ').collect();

        // Parse the piece board.
        let piece_placement = perft_parts[0];
        let mut piece_board = HashMap::new();
        piece_board.insert(Piece::Pawn, 0);
        piece_board.insert(Piece::Knight, 0);
        piece_board.insert(Piece::Bishop, 0);
        piece_board.insert(Piece::Rook, 0);
        piece_board.insert(Piece::Queen, 0);
        piece_board.insert(Piece::King, 0);
        let mut color_board = HashMap::new();
        color_board.insert(Color::White, 0);
        color_board.insert(Color::Black, 0);
        let mut empty_board = 0xff_ff_ff_ff_ff_ff_ff_ff;
        let mut rank = Rank::Eight;
        let mut file = File::A;

        for c in piece_placement.chars() {
            if c.is_numeric() {
                let skip_count = c.to_digit(10).unwrap() as u32;
                let new_file = file as u32 + skip_count;
                if new_file == 8 {
                    continue;
                }
                file = File::from_u32(new_file);
            } else {
                let piece = match c.to_ascii_lowercase() {
                    'p' => Piece::Pawn,
                    'n' => Piece::Knight,
                    'b' => Piece::Bishop,
                    'r' => Piece::Rook,
                    'q' => Piece::Queen,
                    'k' => Piece::King,
                    '/' => {
                        rank = Rank::from_u32(rank as u32 - 1);
                        file = File::A;
                        continue;
                    },
                    _ => panic!("Invalid piece {} {message}", "fancy", message = c),
                };

                let color = if c.is_ascii_lowercase() {
                    Color::Black
                } else {
                    Color::White
                };

                *piece_board.entry(piece).or_insert(0) |= 1 << (rank as u32 * 8 + file as u32);
                *color_board.entry(color).or_insert(0) |= 1 << (rank as u32 * 8 + file as u32);
                empty_board &= !(1 << (rank as u32 * 8 + file as u32));

                if file != File::H {
                    file = File::from_u32(file as u32 + 1);
                }
            }
        }

        // Parse the turn board.
        let turn = match perft_parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid turn"),
        };

        // Parse castling rights.
        let white_castle = (
            perft_parts[2].contains('K'),
            perft_parts[2].contains('Q'),
        );

        let black_castle = (
            perft_parts[2].contains('k'),
            perft_parts[2].contains('q'),
        );

        // Parse the en passant square.
        let en_passant = if perft_parts[3] == "-" {
            None
        } else {
            let file = File::from_char(perft_parts[3].chars().nth(0).unwrap());
            let rank = Rank::from_char(perft_parts[3].chars().nth(1).unwrap());
            Some(Square::new(file, rank))
        };

        // Parse the halfmove clock.
        let halfmove_clock = perft_parts[4].parse().unwrap();

        // Parse the fullmove number.
        let fullmove_number = perft_parts[5].parse().unwrap();

        // Parse the perft depth.
        // If no depth is given, default to 0.
        let perft_depth: u8 = if perft_parts.len() > 6 {
            let depth = perft_parts[6].replace("\n", "");
            depth.parse().unwrap()
        } else {
            0
        };

        let wc_stack = Vec::new();
        let bc_stack = Vec::new();
        let ep_stack = Vec::new();
        let hm_stack = Vec::new();
        let mv_hashmap = HashMap::new();

        Chessboard {
            piece_board,
            color_board,
            empty_board,
            turn,
            white_castle,
            black_castle,
            en_passant,
            halfmove_clock,
            fullmove_number,
            perft_depth,
            wc_stack,
            bc_stack,
            ep_stack,
            hm_stack,
            mv_hashmap,
        }
    }

    pub fn to_fen(&self) {
        let mut fen = String::new();
        let mut empty_count = 0;
        let mut rank = Rank::Eight;
        let mut file = File::A;

        for _ in 0..8 {
            for _ in 0..8 {
                let square = Square::new(file.clone(), rank.clone());
                let piece = self.get_piece(&square);
                let color = self.get_color(&square);

                if piece == None {
                    empty_count += 1;
                } else {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }

                    let piece_char = match piece.unwrap() {
                        Piece::Pawn => "p",
                        Piece::Knight => "n",
                        Piece::Bishop => "b",
                        Piece::Rook => "r",
                        Piece::Queen => "q",
                        Piece::King => "k",
                    };

                    let piece = match color.unwrap() {
                        Color::White => piece_char.to_uppercase(),
                        Color::Black => piece_char.to_lowercase(),
                    };

                    fen.push_str(&piece);
                }

                if file != File::H {
                    file = File::from_u32(file as u32 + 1);
                }
            }

            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
                empty_count = 0;
            }

            if rank != Rank::One {
                fen.push('/');
            }

            if rank != Rank::One {
                rank = Rank::from_u32(rank as u32 - 1);
            }
            file = File::A;
        }

        fen.push(' ');

        let turn_char = match self.turn {
            Color::White => "w",
            Color::Black => "b",
        };

        fen.push_str(turn_char);
        fen.push(' ');

        let mut castle_string = String::new();

        if self.white_castle.0 {
            castle_string.push('K');
        }

        if self.white_castle.1 {
            castle_string.push('Q');
        }

        if self.black_castle.0 {
            castle_string.push('k');
        }

        if self.black_castle.1 {
            castle_string.push('q');
        }

        if castle_string == "" {
            castle_string.push('-');
        }

        fen.push_str(&castle_string);
        fen.push(' ');

        let ep_string = match self.en_passant {
            Some(square) => square.to_string(),
            None => "-".to_string(),
        };

        fen.push_str(&ep_string);
        fen.push(' ');

        fen.push_str(&self.halfmove_clock.to_string());
        fen.push(' ');

        fen.push_str(&self.fullmove_number.to_string());
        fen.push(' ');

        fen.push('1');

        println!("{}", fen);
    }

    pub fn pretty_print(&self) {
        let icons = [
            ["♟︎", "♞", "♝", "♜", "♛", "♚"],
            ["♙", "♘", "♗", "♖", "♕", "♔"],
        ];
        let mut rank = Rank::Eight;
        let mut file = File::A;

        for _ in 0..8 {
            for _ in 0..8 {
                let square = Square::new(file.clone(), rank.clone());
                let piece = self.get_piece(&square);
                let color = self.get_color(&square);

                if piece == None {
                    print!(".");
                } else {
                    let icon = icons[color.unwrap() as usize][piece.unwrap() as usize];
                    print!("{}", icon);
                }

                if file != File::H {
                    print!(" ");
                    file = File::from_u32(file as u32 + 1);
                }
            }

            println!();
            if rank != Rank::One {
                rank = Rank::from_u32(rank as u32 - 1);
                file = File::A;
            }
        }
    }

    pub fn get_pieces_color(&self, piece: &Piece, color: &Color) -> u64 {
        self.get_pieces(piece)& self.color_board[&color]
    }

    pub fn get_pieces(&self, piece: &Piece) -> u64 {
        self.piece_board[&piece]
    }

    pub fn get_colors(&self, color: &Color) -> u64 {
        self.color_board[&color]
    }

    pub fn get_opposite_color(&self, color: &Color) -> Color {
        match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn get_piece(&self, square: &Square) -> Option<Piece> {
        if self.get_pieces(&Piece::Pawn) & square.to_bitboard() != 0 {
            Some(Piece::Pawn)
        } else if self.get_pieces(&Piece::Knight) & square.to_bitboard() != 0 {
            Some(Piece::Knight)
        } else if self.get_pieces(&Piece::Bishop) & square.to_bitboard() != 0 {
            Some(Piece::Bishop)
        } else if self.get_pieces(&Piece::Rook) & square.to_bitboard() != 0 {
            Some(Piece::Rook)
        } else if self.get_pieces(&Piece::Queen) & square.to_bitboard() != 0 {
            Some(Piece::Queen)
        } else if self.get_pieces(&Piece::King) & square.to_bitboard() != 0 {
            Some(Piece::King)
        } else {
            None
        }
    }

    pub fn get_color(&self, square: &Square) -> Option<Color> {
        if self.get_colors(&Color::White) & square.to_bitboard() != 0 {
            Some(Color::White)
        } else if self.get_colors(&Color::Black) & square.to_bitboard() != 0 {
            Some(Color::Black)
        } else {
            None
        }
    }

    fn is_attacked(&self, pos: u64, color: &Color, enemies: u64, gen: fn(u64, &Chessboard, &Color) -> Vec<Move>) -> bool {
        let moves = gen(pos, &self, color);

        for mv in moves.iter() {
            if enemies & mv.to.to_bitboard() != 0 {
                return true;
            }
        }

        return false;
    }

    pub fn is_attacked_square(&self, square: u64, color: &Color) -> bool {
        let opposite = self.get_opposite_color(color);

        let knights = self.get_pieces_color(&Piece::Knight, &opposite);
        let bishops = self.get_pieces_color(&Piece::Bishop, &opposite);
        let rooks = self.get_pieces_color(&Piece::Rook, &opposite);
        let queens = self.get_pieces_color(&Piece::Queen, &opposite);
        let king = self.get_pieces_color(&Piece::King, &opposite);

        let pawns = self.get_pieces_color(&Piece::Pawn, &opposite);
        let targets = moves::piece::pawn::generate_capture_moves(square, &pawns, &self, color);
        if targets.len() > 0 {
            return true;
        }

        self.is_attacked(square, color, knights, moves::piece::knight::generate_pseudo_moves) ||
        self.is_attacked(square, color, bishops, moves::piece::sliding_piece::bishop::generate_pseudo_moves) ||
        self.is_attacked(square, color, rooks, moves::piece::sliding_piece::rook::generate_pseudo_moves) ||
        self.is_attacked(square, color, queens, moves::piece::sliding_piece::queen::generate_pseudo_moves) ||
        self.is_attacked(square, color, king, moves::piece::king::generate_pseudo_moves)
    }

    pub fn is_in_check(&self) -> bool {
        let opposite = self.get_opposite_color(&self.turn);
        let opposite_king = self.get_pieces_color(&Piece::King, &opposite);

        self.is_attacked_square(opposite_king, &opposite)
    }

    pub fn push(&mut self) {
        self.wc_stack.push(self.white_castle);
        self.bc_stack.push(self.black_castle);
        self.ep_stack.push(self.en_passant);
        self.hm_stack.push(self.halfmove_clock);
    }

    pub fn pop(&mut self) {
        self.white_castle = self.wc_stack.pop().unwrap();
        self.black_castle = self.bc_stack.pop().unwrap();
        self.en_passant = self.ep_stack.pop().unwrap();
        self.halfmove_clock = self.hm_stack.pop().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use utils::{color::Color, square::Square};

    use super::Chessboard;

    fn test_is_attacked_square(fen: &str, square: Square, color: &Color, expected: bool) {
        let chessboard = Chessboard::new(fen.to_string());
        assert_eq!(chessboard.is_attacked_square(square.to_bitboard(), color), expected);
    }

    fn test_is_in_check(fen: &str, expected: bool) {
        let chessboard = Chessboard::new(fen.to_string());
        assert_eq!(chessboard.is_in_check(), expected);
    }

    #[test]
    fn test_check() {
        test_is_attacked_square("8/8/8/8/8/8/3p4/4K3 w - - 0 1 1", Square::from_string("e1"), &Color::White, true);
        test_is_attacked_square("8/8/8/8/8/8/2n5/4K3 w - - 0 1 1", Square::from_string("e1"), &Color::White, true);
        test_is_attacked_square("8/4r3/8/8/8/8/8/4K3 w - - 0 1 1", Square::from_string("e1"), &Color::White, true);

        test_is_in_check("8/8/4q3/8/8/8/8/4K3 b - - 0 1 1", true);
    }
}
