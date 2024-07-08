pub mod constants;
mod fen;
mod move_list;
mod update;

use {
	crate::network::constants::*,
	constants::pieces::*,
	std::{
		collections::{HashMap, HashSet},
		fmt::{self, Debug, Display},
		iter::FromIterator,
		vec,
	},
};

#[macro_export]
macro_rules! same_color {
	($piece1:expr, $piece2:expr) => {
		if $piece1.is_white() {
			$piece2.is_white()
		} else {
			$piece2.is_black()
		}
	};
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum File {
	A = 0,
	B = 1,
	C = 2,
	D = 3,
	E = 4,
	F = 5,
	G = 6,
	H = 7,
}
use File::*;

#[inline(always)]
pub fn square(file: File, rank: u8) -> usize {
	(file as u8 + 8 * (rank - 1)).into()
}

const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

pub struct Move {
	pub from: usize,
	pub to: usize,
	pub piece: Square,
	pub clear_square: Option<usize>,
	pub rook_square: Option<usize>,
}

impl Debug for Move {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}{}{}{}",
			FILES[self.from % 8],
			self.from / 8 + 1,
			FILES[self.to % 8],
			self.to / 8 + 1
		)
	}
}

impl fmt::Display for Move {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}{}{}{}",
			FILES[self.from % 8],
			self.from / 8 + 1,
			FILES[self.to % 8],
			self.to / 8 + 1
		)
	}
}

pub type Square = [bool; SQUARE_FEATURES];
pub trait SquareProperties {
	fn is_white(&self) -> bool;
	fn is_black(&self) -> bool;
	fn is_pawn(&self) -> bool;
	fn is_knight(&self) -> bool;
	fn is_bishop(&self) -> bool;
	fn is_rook(&self) -> bool;
	fn is_queen(&self) -> bool;
	fn is_king(&self) -> bool;
}
impl SquareProperties for Square {
	fn is_white(&self) -> bool {
		self[FeatureIndex::WHITE as usize]
	}
	fn is_black(&self) -> bool {
		self[FeatureIndex::BLACK as usize]
	}
	fn is_pawn(&self) -> bool {
		self[FeatureIndex::PAWN as usize]
	}
	fn is_knight(&self) -> bool {
		self[FeatureIndex::KNIGHT as usize]
	}
	fn is_bishop(&self) -> bool {
		self[FeatureIndex::BISHOP as usize]
	}
	fn is_rook(&self) -> bool {
		self[FeatureIndex::ROOK as usize]
	}
	fn is_queen(&self) -> bool {
		self[FeatureIndex::QUEEN as usize]
	}
	fn is_king(&self) -> bool {
		self[FeatureIndex::KING as usize]
	}
}

trait ToString {
	fn to_string(&self) -> String;
}
impl ToString for Square {
	fn to_string(&self) -> String {
		let symbol = if self.is_pawn() {
			"p"
		} else if self.is_knight() {
			"n"
		} else if self.is_bishop() {
			"b"
		} else if self.is_rook() {
			"r"
		} else if self.is_queen() {
			"q"
		} else if self.is_king() {
			"k"
		} else {
			"x"
		};
		if self.is_white() {
			symbol.to_uppercase()
		} else {
			symbol.into()
		}
	}
}

#[derive(Clone)]
pub struct Chess {
	pub to_play: Color,
	pub pieces: HashMap<usize, Piece>,
	pub check: Option<Color>,
	pub move_counter: usize,
	pub fifty_move_counter: usize,
}

impl Display for Chess {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			(0..8)
				.rev()
				.flat_map(|j| {
					(0..8)
						.map(move |i| {
							if let Some(piece) = self.pieces.get(&(i + j * 8)) {
								piece.features.to_string()
							} else {
								" ".into()
							}
						})
						.chain(Some("\n".into()))
				})
				.collect::<String>()
		)
	}
}

#[derive(PartialEq, Eq)]
pub enum Result {
	WHITE,
	BLACK,
	// DRAW,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Color {
	WHITE,
	BLACK,
}
impl Color {
	fn is_white(&self) -> bool {
		*self == Self::WHITE
	}

	// fn is_black(&self) -> bool {
	// 	*self == Self::BLACK
	// }
}
use Color::*;

#[derive(PartialEq, Eq)]
pub enum PieceType {
	PAWN,
	ROOK,
	KNIGHT,
	BISHOP,
	QUEEN,
	KING,
}
use PieceType::*;

#[derive(Clone, Debug, Default)]
pub struct Piece {
	pub features: Square,
	pub destinations: HashSet<usize>,
	pub blocking: HashSet<usize>,
	pub can_castle_long: bool,
	pub can_castle_short: bool,
	pub en_passant_left_turn: Option<usize>,
	pub en_passant_right_turn: Option<usize>,
}
impl SquareProperties for Piece {
	fn is_white(&self) -> bool {
		self.features.is_white()
	}
	fn is_black(&self) -> bool {
		self.features.is_black()
	}
	fn is_pawn(&self) -> bool {
		self.features.is_pawn()
	}
	fn is_knight(&self) -> bool {
		self.features.is_knight()
	}
	fn is_bishop(&self) -> bool {
		self.features.is_bishop()
	}
	fn is_rook(&self) -> bool {
		self.features.is_rook()
	}
	fn is_queen(&self) -> bool {
		self.features.is_queen()
	}
	fn is_king(&self) -> bool {
		self.features.is_king()
	}
}

#[allow(non_upper_case_globals)]
impl Piece {
	pub fn new(color: Color, type_: PieceType, destinations: Vec<usize>) -> Self {
		let mut features = [false, false, false, false, false, false, false, false];
		features[color as usize] = true;
		features[type_ as usize + 2] = true;
		Self {
			features,
			destinations: HashSet::from_iter(destinations.iter().cloned()),
			blocking: HashSet::new(),
			can_castle_long: false,
			can_castle_short: false,
			en_passant_left_turn: None,
			en_passant_right_turn: None,
		}
	}
}

#[rustfmt::skip]
fn default_pieces() -> HashMap<usize, Piece> {
    let mut pieces = HashMap::new();
    pieces.insert(square(A, 1), Piece::new(WHITE, ROOK, vec![]));
    pieces.insert(square(B, 1), Piece::new(WHITE, KNIGHT, vec![square(A, 3), square(C, 3)]));
    pieces.insert(square(C, 1), Piece::new(WHITE, BISHOP, vec![]));
    pieces.insert(square(D, 1), Piece::new(WHITE, QUEEN, vec![]));
    pieces.insert(square(E, 1), Piece::new(WHITE, KING, vec![]));
    pieces.insert(square(F, 1), Piece::new(WHITE, BISHOP, vec![]));
    pieces.insert(square(G, 1), Piece::new(WHITE, KNIGHT, vec![square(F, 3), square(H, 3)]));
    pieces.insert(square(H, 1), Piece::new(WHITE, ROOK, vec![]));

    pieces.insert(square(A, 2), Piece::new(WHITE, PAWN, vec![square(A, 3), square(A, 4)]));
    pieces.insert(square(B, 2), Piece::new(WHITE, PAWN, vec![square(B, 3), square(B, 4)]));
    pieces.insert(square(C, 2), Piece::new(WHITE, PAWN, vec![square(C, 3), square(C, 4)]));
    pieces.insert(square(D, 2), Piece::new(WHITE, PAWN, vec![square(D, 3), square(D, 4)]));
    pieces.insert(square(E, 2), Piece::new(WHITE, PAWN, vec![square(E, 3), square(E, 4)]));
    pieces.insert(square(F, 2), Piece::new(WHITE, PAWN, vec![square(F, 3), square(F, 4)]));
    pieces.insert(square(G, 2), Piece::new(WHITE, PAWN, vec![square(G, 3), square(G, 4)]));
    pieces.insert(square(H, 2), Piece::new(WHITE, PAWN, vec![square(H, 3), square(H, 4)]));

    pieces.insert(square(A, 7), Piece::new(BLACK, PAWN, vec![square(A, 6), square(A, 5)]));
    pieces.insert(square(B, 7), Piece::new(BLACK, PAWN, vec![square(B, 6), square(B, 5)]));
    pieces.insert(square(C, 7), Piece::new(BLACK, PAWN, vec![square(C, 6), square(C, 5)]));
    pieces.insert(square(D, 7), Piece::new(BLACK, PAWN, vec![square(D, 6), square(D, 5)]));
    pieces.insert(square(E, 7), Piece::new(BLACK, PAWN, vec![square(E, 6), square(E, 5)]));
    pieces.insert(square(F, 7), Piece::new(BLACK, PAWN, vec![square(F, 6), square(F, 5)]));
    pieces.insert(square(G, 7), Piece::new(BLACK, PAWN, vec![square(G, 6), square(G, 5)]));
    pieces.insert(square(H, 7), Piece::new(BLACK, PAWN, vec![square(H, 6), square(H, 5)]));

    pieces.insert(square(A, 8), Piece::new(BLACK, ROOK, vec![]));
    pieces.insert(square(B, 8), Piece::new(BLACK, KNIGHT, vec![square(A, 6), square(C, 6)]));
    pieces.insert(square(C, 8), Piece::new(BLACK, BISHOP, vec![]));
    pieces.insert(square(D, 8), Piece::new(BLACK, QUEEN, vec![]));
    pieces.insert(square(E, 8), Piece::new(BLACK, KING, vec![]));
    pieces.insert(square(F, 8), Piece::new(BLACK, BISHOP, vec![]));
    pieces.insert(square(G, 8), Piece::new(BLACK, KNIGHT, vec![square(F, 6), square(H, 6)]));
    pieces.insert(square(H, 8), Piece::new(BLACK, ROOK, vec![]));

    pieces
}

impl Chess {
	pub fn new() -> Self {
		Self {
			pieces: default_pieces(),
			to_play: WHITE,
			check: None,
			move_counter: 0,
			fifty_move_counter: 0,
		}
	}
}
