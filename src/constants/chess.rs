use super::{INPUT_COLUMNS, INPUT_COLUMN_SIZE};

#[rustfmt::skip]
#[allow(non_upper_case_globals)]
pub mod pieces {
    const o: u8 = 0;
    const X: u8 = 1;
    pub const EMPTY_SQUARE: [u8; 8] = [o, o, o, o, o, o, o, o];
    pub const WHITE_PAWN:   [u8; 8] = [X, o, X, o, o, o, o, o];
    pub const WHITE_ROOK:   [u8; 8] = [X, o, o, X, o, o, o, o];
    pub const WHITE_KNIGHT: [u8; 8] = [X, o, o, o, X, o, o, o];
    pub const WHITE_BISHOP: [u8; 8] = [X, o, o, o, o, X, o, o];
    pub const WHITE_QUEEN:  [u8; 8] = [X, o, o, o, o, o, X, o];
    pub const WHITE_KING:   [u8; 8] = [X, o, o, o, o, o, o, X];
    pub const BLACK_PAWN:   [u8; 8] = [o, X, X, o, o, o, o, o];
    pub const BLACK_ROOK:   [u8; 8] = [o, X, o, X, o, o, o, o];
    pub const BLACK_KNIGHT: [u8; 8] = [o, X, o, o, X, o, o, o];
    pub const BLACK_BISHOP: [u8; 8] = [o, X, o, o, o, X, o, o];
    pub const BLACK_QUEEN:  [u8; 8] = [o, X, o, o, o, o, X, o];
    pub const BLACK_KING:   [u8; 8] = [o, X, o, o, o, o, o, X];
}
use pieces::*;

// W, B, P, R, N, B, Q, K
pub const STARTING_BOARD: [[u8; INPUT_COLUMN_SIZE]; INPUT_COLUMNS] = [
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
