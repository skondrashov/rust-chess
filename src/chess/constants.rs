use crate::network::constants::SQUARE_FEATURES;

pub const BOARD_SIZE: usize = 64;

#[rustfmt::skip]
#[allow(non_upper_case_globals)]
pub mod pieces {
 	use super::SQUARE_FEATURES;

	const o: bool = false;
	const X: bool = true;
	pub enum FeatureIndex {
		WHITE,
		BLACK,
		PAWN,
		ROOK,
		KNIGHT,
		BISHOP,
		QUEEN,
		KING,
	}

	pub const EMPTY_SQUARE: [bool; SQUARE_FEATURES] = [o, o, o, o, o, o, o, o];
	pub const WHITE_PAWN:   [bool; SQUARE_FEATURES] = [X, o, X, o, o, o, o, o];
	pub const WHITE_ROOK:   [bool; SQUARE_FEATURES] = [X, o, o, X, o, o, o, o];
	pub const WHITE_KNIGHT: [bool; SQUARE_FEATURES] = [X, o, o, o, X, o, o, o];
	pub const WHITE_BISHOP: [bool; SQUARE_FEATURES] = [X, o, o, o, o, X, o, o];
	pub const WHITE_QUEEN:  [bool; SQUARE_FEATURES] = [X, o, o, o, o, o, X, o];
	pub const WHITE_KING:   [bool; SQUARE_FEATURES] = [X, o, o, o, o, o, o, X];
	pub const BLACK_PAWN:   [bool; SQUARE_FEATURES] = [o, X, X, o, o, o, o, o];
	pub const BLACK_ROOK:   [bool; SQUARE_FEATURES] = [o, X, o, X, o, o, o, o];
	pub const BLACK_KNIGHT: [bool; SQUARE_FEATURES] = [o, X, o, o, X, o, o, o];
	pub const BLACK_BISHOP: [bool; SQUARE_FEATURES] = [o, X, o, o, o, X, o, o];
	pub const BLACK_QUEEN:  [bool; SQUARE_FEATURES] = [o, X, o, o, o, o, X, o];
	pub const BLACK_KING:   [bool; SQUARE_FEATURES] = [o, X, o, o, o, o, o, X];
}
use pieces::*;

pub const STARTING_BOARD: [[bool; SQUARE_FEATURES]; BOARD_SIZE] = [
	// a
	WHITE_ROOK,   // 1
	WHITE_PAWN,   // 2
	EMPTY_SQUARE, // 3
	EMPTY_SQUARE, // 4
	EMPTY_SQUARE, // 5
	EMPTY_SQUARE, // 6
	BLACK_PAWN,   // 7
	BLACK_ROOK,   // 8
	// b
	WHITE_KNIGHT, // 1
	WHITE_PAWN,   // 2
	EMPTY_SQUARE, // 3
	EMPTY_SQUARE, // 4
	EMPTY_SQUARE, // 5
	EMPTY_SQUARE, // 6
	BLACK_PAWN,   // 7
	BLACK_KNIGHT, // 8
	// c
	WHITE_BISHOP, // 1
	WHITE_PAWN,   // 2
	EMPTY_SQUARE, // 3
	EMPTY_SQUARE, // 4
	EMPTY_SQUARE, // 5
	EMPTY_SQUARE, // 6
	BLACK_PAWN,   // 7
	BLACK_BISHOP, // 8
	// d
	WHITE_QUEEN,  // 1
	WHITE_PAWN,   // 2
	EMPTY_SQUARE, // 3
	EMPTY_SQUARE, // 4
	EMPTY_SQUARE, // 5
	EMPTY_SQUARE, // 6
	BLACK_PAWN,   // 7
	BLACK_QUEEN,  // 8
	// e
	WHITE_KING,   // 1
	WHITE_PAWN,   // 2
	EMPTY_SQUARE, // 3
	EMPTY_SQUARE, // 4
	EMPTY_SQUARE, // 5
	EMPTY_SQUARE, // 6
	BLACK_PAWN,   // 7
	BLACK_KING,   // 8
	// f
	WHITE_BISHOP, // 1
	WHITE_PAWN,   // 2
	EMPTY_SQUARE, // 3
	EMPTY_SQUARE, // 4
	EMPTY_SQUARE, // 5
	EMPTY_SQUARE, // 6
	BLACK_PAWN,   // 7
	BLACK_BISHOP, // 8
	// g
	WHITE_KNIGHT, // 1
	WHITE_PAWN,   // 2
	EMPTY_SQUARE, // 3
	EMPTY_SQUARE, // 4
	EMPTY_SQUARE, // 5
	EMPTY_SQUARE, // 6
	BLACK_PAWN,   // 7
	BLACK_KNIGHT, // 8
	// h
	WHITE_ROOK,   // 1
	WHITE_PAWN,   // 2
	EMPTY_SQUARE, // 3
	EMPTY_SQUARE, // 4
	EMPTY_SQUARE, // 5
	EMPTY_SQUARE, // 6
	BLACK_PAWN,   // 7
	BLACK_ROOK,   // 8
];

pub const KNIGHT_MOVES: [(isize, isize); 8] = [
	(1, 2),
	(2, 1),
	(2, -1),
	(1, -2),
	(-1, -2),
	(-2, -1),
	(-2, 1),
	(-1, 2),
];

// pub const DIAGONAL_DIRECTIONS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
// pub const ORTHOGONAL_DIRECTIONS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
pub const DIRECTIONS: [(isize, isize); 8] = [
	(1, 1),
	(1, -1),
	(-1, 1),
	(-1, -1),
	(1, 0),
	(0, 1),
	(-1, 0),
	(0, -1),
];
