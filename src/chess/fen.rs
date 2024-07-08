// adapted from https://github.com/ucarion/fen

//! This module provides a pretty straightfoward interface for converting
//! [Forsyth-Edwards notation][1] (FEN) into the state of a game of chess and
//! back.
//!
//! FEN is a way of representing a board a string. This crate provides one such
//! representation, `fen::BoardState`. If you want to be able to read FEN, you
//! will to need to create a way to convert `BoardState` to your own board
//! representation.  If you want to export FEN, you will need to convert your
//! own board representation to `BoardState`.
//!
//! [1]: https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation

use {
	super::{square, Chess, Color, File::*, Piece, PieceType, SquareProperties},
	std::collections::HashMap,
};

pub type FenResult<'a, T> = Result<T, FenError<'a>>;

#[derive(Debug, Eq, PartialEq)]
pub enum FenError<'a> {
	NotEnoughParts,
	BadPlacement(&'a str),
	TooManyPieces(&'a str),
	UnknownPiece(char),
	NoSuchSide(&'a str),
	BadEnPassant(&'a str),
	BadCastling(&'a str),
	BadHalfmove(&'a str),
	BadFullmove(&'a str),
}

impl Chess {
	pub fn from_fen<'a>(fen: &'a str) -> FenResult<'a, Self> {
		let parts: Vec<_> = fen.split(' ').collect();
		if parts.len() != 6 {
			return Err(FenError::NotEnoughParts);
		}

		let mut pieces = Self::parse_placement(parts[0])?;
		let to_play = Self::parse_side_to_play(parts[1])?;
		Self::parse_castling(parts[2], &mut pieces)?;
		Self::parse_en_passant(parts[3], &mut pieces)?;
		let fifty_move_counter = Self::parse_halfmove(parts[4])?;
		Self::parse_fullmove(parts[5])?;

		let mut game = Self {
			pieces,
			to_play,
			check: None,
			move_counter: 0,
			fifty_move_counter,
		};

		game.generate_semilegal_moves();

		Ok(game)
	}

	fn parse_placement(placement_str: &str) -> FenResult<HashMap<usize, Piece>> {
		let mut result_pieces = HashMap::new();
		let lines: Vec<_> = placement_str.split('/').collect();

		if lines.len() != 8 {
			return Err(FenError::BadPlacement(placement_str));
		}

		for (rank, pieces) in lines.iter().rev().enumerate() {
			let mut file = 0usize;

			for piece_char in pieces.chars() {
				match piece_char.to_digit(10) {
					// parse a number indicating n empty squares
					Some(n) => {
						file += n as usize;
						if file > 8 {
							return Err(FenError::TooManyPieces(pieces));
						}
					}

					// parse a letter indicating a piece
					None => match Piece::from_char(piece_char) {
						Some(piece) => {
							result_pieces.insert(rank * 8 + file, piece);
							file += 1;
						}

						None => return Err(FenError::UnknownPiece(piece_char)),
					},
				}
			}
		}

		Ok(result_pieces)
	}

	fn parse_side_to_play(side_to_play: &str) -> FenResult<Color> {
		match side_to_play {
			"w" => Ok(Color::WHITE),
			"b" => Ok(Color::BLACK),
			_ => Err(FenError::NoSuchSide(side_to_play)),
		}
	}

	fn parse_castling<'a>(
		castling: &'a str,
		pieces: &mut HashMap<usize, Piece>,
	) -> FenResult<'a, ()> {
		if castling == "-" {
			return Ok(());
		}

		if castling
			.chars()
			.any(|c| !matches!(c, 'K' | 'Q' | 'k' | 'q'))
		{
			return Err(FenError::BadCastling(castling));
		}

		if castling.contains('K') {
			if let Some(king) = pieces.get_mut(&square(E, 1)) {
				king.can_castle_short = true
			}
		}
		if castling.contains('Q') {
			if let Some(king) = pieces.get_mut(&square(E, 1)) {
				king.can_castle_long = true
			}
		}
		if castling.contains('k') {
			if let Some(king) = pieces.get_mut(&square(E, 8)) {
				king.can_castle_short = true
			}
		}
		if castling.contains('q') {
			if let Some(king) = pieces.get_mut(&square(E, 8)) {
				king.can_castle_long = true
			}
		}

		Ok(())
	}

	fn parse_en_passant<'a>(
		en_passant: &'a str,
		pieces: &mut HashMap<usize, Piece>,
	) -> FenResult<'a, ()> {
		if en_passant == "-" {
			return Ok(());
		}

		if en_passant.len() != 2 {
			return Err(FenError::BadEnPassant(en_passant));
		}

		let chars: Vec<_> = en_passant.chars().collect();
		let (file, rank) = (chars[0], chars[1]);

		let file = match file {
			'a' => A,
			'b' => B,
			'c' => C,
			'd' => D,
			'e' => E,
			'f' => F,
			'g' => G,
			'h' => H,
			_ => return Err(FenError::BadEnPassant(en_passant)),
		};

		match rank {
			'3' => {
				if !matches!(file, A) {
					if let Some(piece) = pieces.get_mut(&(square(file, 4) - 1)) {
						if piece.is_pawn() && piece.is_black() {
							piece.en_passant_right_turn = Some(0);
						}
					}
				}
				if !matches!(file, H) {
					if let Some(piece) = pieces.get_mut(&(square(file, 4) + 1)) {
						if piece.is_pawn() && piece.is_black() {
							piece.en_passant_left_turn = Some(0);
						}
					}
				}
			}
			'6' => {
				if !matches!(file, A) {
					if let Some(piece) = pieces.get_mut(&(square(file, 5) - 1)) {
						if piece.is_pawn() && piece.is_white() {
							piece.en_passant_right_turn = Some(0);
						}
					}
				}
				if !matches!(file, H) {
					if let Some(piece) = pieces.get_mut(&(square(file, 5) + 1)) {
						if piece.is_pawn() && piece.is_white() {
							piece.en_passant_left_turn = Some(0);
						}
					}
				}
			}
			_ => return Err(FenError::BadEnPassant(en_passant)),
		};

		Ok(())
	}

	fn parse_halfmove(halfmove: &str) -> FenResult<usize> {
		match halfmove.parse() {
			Ok(n) => Ok(n),
			Err(_) => Err(FenError::BadHalfmove(halfmove)),
		}
	}

	fn parse_fullmove(fullmove: &str) -> FenResult<u64> {
		match fullmove.parse() {
			Ok(n) => Ok(n),
			Err(_) => Err(FenError::BadFullmove(fullmove)),
		}
	}
}

impl Piece {
	pub fn from_char(piece_char: char) -> Option<Piece> {
		let (color, kind) = match piece_char {
			'P' => (Color::WHITE, PieceType::PAWN),
			'N' => (Color::WHITE, PieceType::KNIGHT),
			'B' => (Color::WHITE, PieceType::BISHOP),
			'R' => (Color::WHITE, PieceType::ROOK),
			'Q' => (Color::WHITE, PieceType::QUEEN),
			'K' => (Color::WHITE, PieceType::KING),
			'p' => (Color::BLACK, PieceType::PAWN),
			'n' => (Color::BLACK, PieceType::KNIGHT),
			'b' => (Color::BLACK, PieceType::BISHOP),
			'r' => (Color::BLACK, PieceType::ROOK),
			'q' => (Color::BLACK, PieceType::QUEEN),
			'k' => (Color::BLACK, PieceType::KING),
			_ => return None,
		};
		Some(Piece::new(color, kind, vec![]))
	}
}

//     /// Convert a BoardState into a string in Forsyth-Edwards notation.
//     ///
//     /// ```
//     /// let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
//     /// let board = fen::BoardState::from_fen(fen).unwrap();
//     /// assert_eq!(fen, board.to_fen());
//     /// ```
//     pub fn to_fen(&self) -> String {
//         let placement = self.make_placement();
//         let side_to_play = self.make_side_to_play();
//         let castling = self.make_castling();
//         let en_passant = self.make_en_passant();
//         let halfmove = self.make_halfmove();
//         let fullmove = self.make_fullmove();

//         let parts = [placement, side_to_play, castling,
//                      en_passant, halfmove, fullmove];
//         parts.connect(" ")
//     }

//     fn make_placement(&self) -> String {
//         let mut placement = String::new();

//         for rank in (0..8).rev() {
//             let mut blanks = 0;

//             for file in 0..8 {
//                 match self.pieces[rank * 8 + file] {
//                     Some(ref piece) => {
//                         if blanks != 0 {
//                             placement.push_str(&blanks.to_string());
//                             blanks = 0;
//                         }

//                         placement.push_str(&piece.to_string());
//                     }

//                     None => { blanks += 1 }
//                 }
//             }

//             if blanks != 0 {
//                 placement.push_str(&blanks.to_string());
//             }

//             // rank = 0 is the last rank displayed, and does not need a '/'
//             // separator
//             if rank != 0 {
//                 placement.push('/');
//             }
//         }

//         placement
//     }

//     fn make_side_to_play(&self) -> String {
//         match self.side_to_play {
//             Color::White => "w",
//             Color::Black => "b"
//         }.to_owned()
//     }

//     fn make_castling(&self) -> String {
//         let mut castling = String::new();
//         if self.white_can_oo  { castling.push('K') }
//         if self.white_can_ooo { castling.push('Q') }
//         if self.black_can_oo  { castling.push('k') }
//         if self.black_can_ooo { castling.push('q') }

//         if castling != "" {
//             castling
//         } else {
//             "-".to_owned()
//         }
//     }

//     fn make_en_passant(&self) -> String {
//         match self.en_passant_square {
//             Some(en_passant) => {
//                 let file = match en_passant % 8 {
//                     0 => 'a',
//                     1 => 'b',
//                     2 => 'c',
//                     3 => 'd',
//                     4 => 'e',
//                     5 => 'f',
//                     6 => 'g',
//                     7 => 'h',
//                     _ => unreachable!()
//                 };

//                 let rank = match en_passant / 8 {
//                     0 => '1',
//                     1 => '2',
//                     2 => '3',
//                     3 => '4',
//                     4 => '5',
//                     5 => '6',
//                     6 => '7',
//                     7 => '8',
//                     _ => unreachable!()
//                 };

//                 format!("{}{}", file, rank)
//             },

//             None => "-".to_owned()
//         }
//     }

//     fn make_halfmove(&self) -> String {
//         self.halfmove_clock.to_string()
//     }

//     fn make_fullmove(&self) -> String {
//         self.fullmove_number.to_string()
//     }
// }

// impl ToString for Piece {
//     fn to_string(&self) -> String {
//         match (&self.color, &self.kind) {
//             (&Color::White, &PieceKind::Pawn) => "P",
//             (&Color::White, &PieceKind::Knight) => "N",
//             (&Color::White, &PieceKind::Bishop) => "B",
//             (&Color::White, &PieceKind::Rook) => "R",
//             (&Color::White, &PieceKind::Queen) => "Q",
//             (&Color::White, &PieceKind::King) => "K",
//             (&Color::Black, &PieceKind::Pawn) => "p",
//             (&Color::Black, &PieceKind::Knight) => "n",
//             (&Color::Black, &PieceKind::Bishop) => "b",
//             (&Color::Black, &PieceKind::Rook) => "r",
//             (&Color::Black, &PieceKind::Queen) => "q",
//             (&Color::Black, &PieceKind::King) => "k"
//         }.to_owned()
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     fn test_valid_fen(fen: &str) {
//         let board = BoardState::from_fen(fen).unwrap();
//         assert_eq!(fen, board.to_fen());
//     }

//     #[test]
//     fn test_to_and_from_fen() {
//         test_valid_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
//         test_valid_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
//         test_valid_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2");
//         test_valid_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
//         test_valid_fen("4k3/8/8/8/8/8/4P3/4K3 w - - 5 39");
//     }

//     #[test]
//     fn test_too_short() {
//         let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0";
//         let board = BoardState::from_fen(fen);
//         assert_eq!(Err(FenError::NotEnoughParts), board);
//     }

//     #[test]
//     fn test_bad_placement() {
//         let placement = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/8";
//         let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/8 w KQkq - 0 1";
//         let board = BoardState::from_fen(fen);
//         assert_eq!(Err(FenError::BadPlacement(placement)), board);
//     }

//     #[test]
//     fn test_too_many_pieces() {
//         let fen = "rnbqkbnr/pppppppp/9/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
//         let board = BoardState::from_fen(fen);
//         assert_eq!(Err(FenError::TooManyPieces("9")), board);
//     }

//     #[test]
//     fn test_unknown_piece() {
//         let fen = "rnbqkbnr/ppppTppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
//         let board = BoardState::from_fen(fen);
//         assert_eq!(Err(FenError::UnknownPiece('T')), board);
//     }

//     #[test]
//     fn test_no_such_side() {
//         let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR n KQkq - 0 1";
//         let board = BoardState::from_fen(fen);
//         assert_eq!(Err(FenError::NoSuchSide("n")), board);
//     }

//     #[test]
//     fn test_bad_en_passant() {
//         let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq h9 0 1";
//         let board = BoardState::from_fen(fen);
//         assert_eq!(Err(FenError::BadEnPassant("h9")), board);
//     }

//     #[test]
//     fn test_bad_halfmove() {
//         let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - zero 1";
//         let board = BoardState::from_fen(fen);
//         assert_eq!(Err(FenError::BadHalfmove("zero")), board);
//     }

//     #[test]
//     fn test_bad_fullmove() {
//         let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 one";
//         let board = BoardState::from_fen(fen);
//         assert_eq!(Err(FenError::BadFullmove("one")), board);
//     }
// }
