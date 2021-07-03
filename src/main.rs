mod constants;
mod nnue;

use {
    constants::{
        chess::{
            pieces::{EMPTY_SQUARE, WHITE_PAWN},
            STARTING_BOARD,
        },
        INPUT_COLUMNS, INPUT_COLUMN_SIZE, OUTPUTS,
    },
    nnue::Nnue,
};

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
use File::*;

fn square(file: File, rank: u8) -> usize {
    (file as u8 + rank) as usize
}
fn main() {
    let mut layer = Nnue {
        biases: [0f32; OUTPUTS],
        weights: [[[0f32; INPUT_COLUMN_SIZE]; OUTPUTS]; INPUT_COLUMNS],
        input_cache: [[0u8; INPUT_COLUMN_SIZE]; INPUT_COLUMNS],
        output_cache: [0f32; OUTPUTS],
    };
    layer.init(STARTING_BOARD);
    layer.evaluate(vec![
        (square(E, 2), EMPTY_SQUARE),
        (square(E, 5), WHITE_PAWN),
    ]);
}
