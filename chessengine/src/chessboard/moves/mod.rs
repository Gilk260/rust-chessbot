use super::Chessboard;

use utils::color::Color;
use utils::direction::{NORT, SOUT};
use utils::file::File;
use utils::piece::Piece;
use utils::rank::Rank;
use utils::square::Square;

pub mod piece;

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<Piece>,
    pub capture: Option<Piece>,
}

impl Move {
    pub fn new(from: Square, to: Square) -> Move {
        Move {
            from,
            to,
            promotion: None,
            capture: None,
        }
    }

    pub fn to_string(&self) -> String {
        let res: String = format!("{}{}", self.from.to_string(), self.to.to_string());

        if self.promotion.is_some() {
            format!("{}{}", res, self.promotion.unwrap().to_char())
        } else {
            res
        }
    }
}

impl Chessboard {
    pub fn generate_legal_moves(&mut self) -> Vec<Move> {
        let pseudo_moves = self.generate_moves();
        let mut legal_moves = Vec::new();
        self.is_checkmate = false;
        self.is_stalemate = false;

        for mv in pseudo_moves {
            self.make_move(&mv);
            if !self.is_making_check(&self.turn) {
                legal_moves.push(mv.clone());
            }
            self.unmake_move(&mv);
        }
        legal_moves.append(&mut piece::king::generate_castling_moves(self, &self.turn));

        legal_moves
    }

    pub fn generate_moves(&self) -> Vec<Move> {
        let moves = &mut Vec::new();

        let generations: [(fn(u64, &Chessboard, &Color, &mut Vec<Move>), Piece); 6] = [
            (piece::pawn::generate_pseudo_moves, Piece::Pawn),
            (piece::knight::generate_pseudo_moves, Piece::Knight),
            (piece::king::generate_pseudo_moves, Piece::King),
            (piece::sliding_piece::bishop::generate_pseudo_moves, Piece::Bishop),
            (piece::sliding_piece::rook::generate_pseudo_moves, Piece::Rook),
            (piece::sliding_piece::queen::generate_pseudo_moves, Piece::Queen),
        ];

        for (generation, piece) in generations.iter() {
            generation(
                self.get_pieces_color(piece, &self.turn),
                self,
                &self.turn,
                moves);
        }

        moves.to_vec()
    }

    pub fn generate_move_from_string(
        &self,
        mv: String,
    ) -> Move {
        let from = Square::from_string(&mv[0..2]);
        let to = Square::from_string(&mv[2..4]);

        let mut res = self.generate_move(from, to);

        if mv.len() == 5 {
            res.promotion = match mv[4..5].chars().nth(0).unwrap() {
                'q' => Some(Piece::Queen),
                'r' => Some(Piece::Rook),
                'n' => Some(Piece::Bishop),
                'b' => Some(Piece::Knight),
                _ => panic!("Wrong promotion type {message}", message = mv),
            };
        }

        res
    }

    pub fn generate_move(
        &self,
        from: Square,
        to: Square,
    ) -> Move {
        let mut res = Move::new(from, to);

        // Handle capture
        if let Some(capture) = self.get_piece(&res.to) {
            res.capture = Some(capture);
        } else if let Some(piece) = self.get_piece(&res.from) {
            // Handle en passant
            if piece == Piece::Pawn {
                let en_passant = self.en_passant;
                if en_passant != None && res.to == en_passant.unwrap() {
                    res.capture = Some(Piece::Pawn);
                }
            }
        }

        // Handle promotion
        //if let Some(piece) = self.get_piece(&res.from) {
        //    if piece == Piece::Pawn {
        //        self.generate_promotion_moves(&mut moves, res);
        //    }
        //    else {
        //        moves.push(res);
        //    }
        //}
        //else {
        //    moves.push(res);
        //}

        res
    }

    fn generate_promotion_moves(&self, moves: &mut Vec<Move>, mv: Move) {
        let mut mv = mv;

        let rank_promotion = match self.turn {
            Color::White => Rank::Eight,
            Color::Black => Rank::One,
        };

        if mv.to.rank == rank_promotion {
            for promotion in vec![
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen
            ] {
                mv.promotion = Some(promotion);
                moves.push(mv);
            }
        }
        else {
            moves.push(mv);
        }
    }

    fn make_castling(&mut self, mv: &Move) {
        let square_rook_src: Square;
        let square_rook_dst: Square;

        match mv.to.file {
            File::C => {
                square_rook_src = Square::new(File::A, mv.to.rank);
                square_rook_dst = Square::new(File::D, mv.to.rank);
            },
            File::G => {
                square_rook_src = Square::new(File::H, mv.to.rank);
                square_rook_dst = Square::new(File::F, mv.to.rank);
            },
            _ => panic!("Wrong castling move"),
        };

        let square_src_dst = square_rook_src.to_bitboard() | square_rook_dst.to_bitboard();

        self.piece_board[Piece::Rook.to_usize()] ^= square_src_dst;
        self.color_board[self.turn.to_usize()] ^= square_src_dst;
        self.empty_board ^= square_src_dst;
    }

    fn update_castling_rights(&mut self) {
        let square_e1 = self.get_piece(&Square::from_string("e1"));
        if self.white_castle.0 {
            if square_e1 == None || square_e1.unwrap() != Piece::King {
                self.white_castle.0 = false;
                self.white_castle.1 = false;
            }
            let color_h1 = self.get_color(&Square::from_string("h1"));
            if color_h1 != Some(Color::White) {
                self.white_castle.0 = false;
            }
        }
        if self.white_castle.1 {
            if square_e1 == None || square_e1.unwrap() != Piece::King {
                self.white_castle.0 = false;
                self.white_castle.1 = false;
            }
            let color_a1 = self.get_color(&Square::from_string("a1"));
            if color_a1 != Some(Color::White) {
                self.white_castle.1 = false;
            }
        }

        let square_e8 = self.get_piece(&Square::from_string("e8"));
        if self.black_castle.0 {
            if square_e8 == None || square_e8.unwrap() != Piece::King {
                self.black_castle.0 = false;
                self.black_castle.1 = false;
            }
            let color_h8 = self.get_color(&Square::from_string("h8"));
            if color_h8 != Some(Color::Black) {
                self.black_castle.0 = false;
            }
        }
        if self.black_castle.1 {
            let square_e8 = self.get_piece(&Square::from_string("e8"));
            if square_e8 == None || square_e8.unwrap() != Piece::King {
                self.black_castle.0 = false;
                self.black_castle.1 = false;
            }
            let color_a8 = self.get_color(&Square::from_string("a8"));
            if color_a8 != Some(Color::Black) {
                self.black_castle.1 = false;
            }
        }
    }

    pub fn make_move(&mut self, mv: &Move) {
        //self.pretty_print();
        //println!("Move: {}", mv.to_string());
        self.push();
        if self.perft_depth > 0 {
            self.perft_depth -= 1;
        }
        let mut piece = self.get_piece(&mv.from).unwrap();
        let opposite = self.get_opposite_color(&self.turn);
        let mut next_ep = None;

        self.piece_board[piece.to_usize()] ^= mv.from.to_bitboard();
        self.color_board[self.turn.to_usize()] ^= mv.from.to_bitboard();
        self.empty_board ^= mv.from.to_bitboard();

        if piece == Piece::Pawn {
            let diff: i32 = mv.from.to_u32() as i32 - mv.to.to_u32() as i32; 

            if diff.abs() == (2 * NORT) {
                next_ep = Some(Square::from_u32((mv.from.to_u32() as i32 - diff / 2) as u32));
            } else if mv.promotion != None {
                piece = mv.promotion.unwrap();
            }
        } else if piece == Piece::King {
            if ((mv.from.to_u32() as i32 - mv.to.to_u32() as i32) as i32).abs() == 2 {
                self.make_castling(mv);
            }
        }

        let captured = mv.capture;

        // Handle capture
        if captured != None {
            let captured = captured.unwrap();

            // En passant
            if piece == Piece::Pawn
                && captured == Piece::Pawn
                && self.en_passant != None
                && mv.to == self.en_passant.unwrap()
            {
                let real_pos = mv.to.to_u32() as i32 + match self.turn {
                    Color::White => SOUT,
                    Color::Black => NORT,
                };
                let real_square = Square::from_u32(real_pos as u32);
                self.piece_board[captured.to_usize()] ^= real_square.to_bitboard();
                self.color_board[opposite.to_usize()] ^= real_square.to_bitboard();
                self.empty_board ^= real_square.to_bitboard() | mv.to.to_bitboard();
            } else {
                self.piece_board[captured.to_usize()] ^= mv.to.to_bitboard();
                self.color_board[opposite.to_usize()] ^= mv.to.to_bitboard();
            }
        } else {
            self.empty_board ^= mv.to.to_bitboard();
        }

        self.piece_board[piece.to_usize()] |= mv.to.to_bitboard();
        self.color_board[self.turn.to_usize()] |= mv.to.to_bitboard();

        self.update_castling_rights();

        self.turn = opposite;
        //self.halfmove_clock += 1;
        self.en_passant = next_ep;

        if self.turn == Color::White {
            self.fullmove_number += 1;
        }
    }

    pub fn unmake_move(&mut self, mv: &Move) {
        self.perft_depth += 1;
        self.turn = self.get_opposite_color(&self.turn);
        let mut piece = self.get_piece(&mv.to).unwrap();

        self.piece_board[piece.to_usize()] ^= mv.to.to_bitboard();
        self.color_board[self.turn.to_usize()] ^= mv.to.to_bitboard();
        self.empty_board ^= mv.to.to_bitboard();

        if mv.promotion != None {
            piece = Piece::Pawn;
        }

        self.piece_board[piece.to_usize()] |= mv.from.to_bitboard();
        self.color_board[self.turn.to_usize()] |= mv.from.to_bitboard();
        self.empty_board ^= mv.from.to_bitboard();

        self.pop();

        if piece == Piece::King {
            if ((mv.from.to_u32() as i32 - mv.to.to_u32() as i32) as i32).abs() == 2 {
                self.make_castling(mv);
            }
        }

        // Handle capture
        if let Some(capture) = mv.capture {
            if piece == Piece::Pawn
                && capture == Piece::Pawn
                && self.en_passant != None
                && mv.to == self.en_passant.unwrap()
            {
                let real_pos = mv.to.to_u32() as i32 + match self.turn {
                    Color::White => SOUT,
                    Color::Black => NORT,
                };
                let real_square = Square::from_u32(real_pos as u32);
                self.piece_board[capture.to_usize()] |= real_square.to_bitboard();
                self.color_board[self.turn.opposite().to_usize()] |= real_square.to_bitboard();
                self.empty_board ^= real_square.to_bitboard();
            } else {
                self.piece_board[capture.to_usize()] |= mv.to.to_bitboard();
                self.color_board[self.turn.opposite().to_usize()] |= mv.to.to_bitboard();
                self.empty_board ^= mv.to.to_bitboard();
            }
        }

        if self.turn == Color::Black {
            self.fullmove_number -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_chessboard(actual: Chessboard, expected: Chessboard) {
        assert_eq!(actual.get_pieces(&Piece::Pawn), expected.get_pieces(&Piece::Pawn), "Test pawns");
        assert_eq!(actual.get_pieces(&Piece::King), expected.get_pieces(&Piece::King), "Test kings");
        assert_eq!(actual.get_pieces(&Piece::Queen), expected.get_pieces(&Piece::Queen), "Test queens");
        assert_eq!(actual.get_pieces(&Piece::Bishop), expected.get_pieces(&Piece::Bishop), "Test bishops");
        assert_eq!(actual.get_pieces(&Piece::Knight), expected.get_pieces(&Piece::Knight), "Test knights");
        assert_eq!(actual.get_pieces(&Piece::Rook), expected.get_pieces(&Piece::Rook), "Test rooks");
        assert_eq!(actual.get_colors(&Color::White), expected.get_colors(&Color::White), "Test whites");
        assert_eq!(actual.get_colors(&Color::Black), expected.get_colors(&Color::Black), "Test, blacks");
        assert_eq!(actual.empty_board, expected.empty_board, "Test empty board");
        assert_eq!(actual.turn, expected.turn, "Test turn");
        assert_eq!(actual.white_castle, expected.white_castle, "Test white castle");
        assert_eq!(actual.black_castle, expected.black_castle, "Test black castle");
        assert_eq!(actual.en_passant, expected.en_passant, "Test en passant");
        // assert_eq!(actual.halfmove_clock, expected.halfmove_clock, "Test halfmove");
        assert_eq!(actual.fullmove_number, expected.fullmove_number, "Test fullmove");
        assert_eq!(actual.perft_depth, expected.perft_depth, "Test depth");
    }

    #[test]
    fn test_generate_move_from_string() {
        let chessboard = Chessboard::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
        let mv = chessboard.generate_move_from_string("e2e4".to_string());
        let expected: Move = Move {
            from: Square::from_string("e2"),
            to: Square::from_string("e4"),
            capture: None,
            promotion: None,
        };
        assert_eq!(mv, expected);
    }

    fn test_make_move(fen: &str, mv: &str, expected: &str) {
        let mut chessboard = Chessboard::new(fen.to_string());
        let mv = chessboard.generate_move_from_string(mv.to_string());
        chessboard.make_move(&mv);
        let expected = Chessboard::new(expected.to_string());
        compare_chessboard(chessboard, expected);
    }

    #[test]
    fn test_make_move_pawn() {
        // Quiet move
        test_make_move("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 1", "e2e3", "rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 1 0");
        test_make_move("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 1", "e2e4", "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1 0");

        test_make_move("rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 1 1", "e7e6", "rnbqkbnr/pppp1ppp/4p3/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 2 0");
        test_make_move("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1 1", "e7e5", "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2 0");

        // Capture move
        test_make_move("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 2 1", "d4e5", "rnbqkbnr/pppp1ppp/8/4P3/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 2 0");
        test_make_move("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 2 1", "e5d4", "rnbqkbnr/pppp1ppp/8/8/3p4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 3 0");

        // En passant
        test_make_move("rnbqkbnr/pppp1ppp/8/3Pp3/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 2 1", "d5e6", "rnbqkbnr/pppp1ppp/4P3/8/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 2 0");
        test_make_move("rnbqkbnr/pppp1ppp/8/8/3Pp3/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 3 1", "e4d3", "rnbqkbnr/pppp1ppp/8/8/8/3p4/PPP1PPPP/RNBQKBNR w KQkq - 0 4 0");

        // Promotion
        test_make_move("8/P7/8/8/8/8/8/8 w - - 0 1 1", "a7a8q", "Q7/8/8/8/8/8/8/8 b - - 0 1 0");
        test_make_move("8/8/8/8/8/8/p7/8 b - - 0 1 1", "a2a1q", "8/8/8/8/8/8/8/q7 w - - 0 2 0");

    }

    fn test_unmake_move(fen: &str, mv: &str) {
        let mut chessboard = Chessboard::new(fen.to_string());
        let mv = chessboard.generate_move_from_string(mv.to_string());
        chessboard.make_move(&mv);
        chessboard.unmake_move(&mv);
        let expected = Chessboard::new(fen.to_string());
        compare_chessboard(chessboard, expected);
    }

    #[test]
    fn test_unmake_move_pawn() {
        // Quiet move
        test_unmake_move("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 1", "e2e3");
        test_unmake_move("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1 1", "e7e6");

        test_unmake_move("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 1", "e2e4");
        test_unmake_move("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1 1", "e7e5");

        // Capture move
        test_unmake_move("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 2 1", "d4e5");
        test_unmake_move("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 2 1", "e5d4");

        // En passant
        test_unmake_move("rnbqkbnr/pppp1ppp/8/3Pp3/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 2 1", "d5e6");
        test_unmake_move("rnbqkbnr/pppp1ppp/8/8/3Pp3/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 3 1", "e4d3");

        // Promotion
        test_unmake_move("8/P7/8/8/8/8/8/8 w - - 0 1 1", "a7a8q");
        test_unmake_move("8/8/8/8/8/8/p7/8 b - - 0 1 1", "a2a1q");
    }
}
