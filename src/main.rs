mod constants;
mod nnue;

use {
    constants::{INPUT_COLUMNS, INPUT_COLUMN_SIZE, OUTPUTS},
    nnue::Nnue,
};

#[rustfmt::skip]
#[allow(non_upper_case_globals)]
mod pieces {
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

pub enum File {
    A = 8 * 0,
    B = 8 * 1,
    C = 8 * 2,
    D = 8 * 3,
    E = 8 * 4,
    F = 8 * 5,
    G = 8 * 6,
    H = 8 * 7,
}
fn square(file: File, rank: u8) -> usize {
    (file as u8 + rank) as usize
}

use {pieces::*, File::*};

fn main() {
    // W, B, P, R, N, B, Q, K
    let board: [[u8; INPUT_COLUMN_SIZE]; INPUT_COLUMNS] = [
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
    let mut layer = Nnue {
        biases: [0f32; OUTPUTS],
        weights: [[[0f32; INPUT_COLUMN_SIZE]; OUTPUTS]; INPUT_COLUMNS],
        input_cache: [[0u8; INPUT_COLUMN_SIZE]; INPUT_COLUMNS],
        output_cache: [0f32; OUTPUTS],
    };
    layer.init(board);
    layer.evaluate(vec![
        (square(E, 2), EMPTY_SQUARE),
        (square(E, 5), WHITE_PAWN),
    ]);
}
