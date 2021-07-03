mod chess;
mod constants;
mod nnue;

use {
    constants::{INPUTS, OUTPUTS},
    nnue::Nnue,
};

const T: u8 = 1;
const F: u8 = 0;

fn main() {
    let board: [u8; INPUTS] = [
        // White
        T, T, T, T, T, T, T, T, // 1
        T, T, T, T, T, T, T, T, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        F, F, F, F, F, F, F, F, // 7
        F, F, F, F, F, F, F, F, // 8
        // Black
        F, F, F, F, F, F, F, F, // 1
        F, F, F, F, F, F, F, F, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        T, T, T, T, T, T, T, T, // 7
        T, T, T, T, T, T, T, T, // 8
        // Pawn
        F, F, F, F, F, F, F, F, // 1
        T, T, T, T, T, T, T, T, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        T, T, T, T, T, T, T, T, // 7
        F, F, F, F, F, F, F, F, // 8
        // Rook
        T, F, F, F, F, F, F, T, // 1
        F, F, F, F, F, F, F, F, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        F, F, F, F, F, F, F, F, // 7
        T, F, F, F, F, F, F, T, // 8
        // Knight
        F, T, F, F, F, F, T, F, // 1
        F, F, F, F, F, F, F, F, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        F, F, F, F, F, F, F, F, // 7
        F, T, F, F, F, F, T, F, // 8
        // Bishop
        F, F, T, F, F, T, F, F, // 1
        F, F, F, F, F, F, F, F, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        F, F, F, F, F, F, F, F, // 7
        F, F, T, F, F, T, F, F, // 8
        // Queen
        F, F, F, T, F, F, F, F, // 1
        F, F, F, F, F, F, F, F, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        F, F, F, F, F, F, F, F, // 7
        F, F, F, T, F, F, F, F, // 8
        // King
        F, F, F, F, T, F, F, F, // 1
        F, F, F, F, F, F, F, F, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        F, F, F, F, F, F, F, F, // 7
        F, F, F, F, T, F, F, F, // 8
    ];
    let mut layer = Nnue {
        biases: [0f32; OUTPUTS],
        weights: [[0f32; INPUTS]; OUTPUTS],
        cache: [0f32; OUTPUTS],
    };
    layer.evaluate(&board, vec![1]);
}
