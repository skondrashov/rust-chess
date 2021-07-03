mod constants;
mod nnue;

use {
    constants::{INPUTS, OUTPUTS},
    nnue::Nnue,
};

const T: u8 = 1;
const F: u8 = 0;

fn main() {
    // W, B, P, R, N, B, Q, K
    let board: [u8; INPUTS] = [
        // a
        T, F, F, T, F, F, F, F, // 1
        T, F, T, F, F, F, F, F, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        F, T, T, F, F, F, F, F, // 7
        F, T, F, T, F, F, F, F, // 8
        // b
        T, F, F, F, T, F, F, F, // 1
        T, F, T, F, F, F, F, F, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        F, T, T, F, F, F, F, F, // 7
        F, T, F, F, T, F, F, F, // 8
        // c
        T, F, F, F, F, T, F, F, // 1
        T, F, T, F, F, F, F, F, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        F, T, T, F, F, F, F, F, // 7
        F, T, F, F, F, T, F, F, // 8
        // d
        T, F, F, F, F, F, T, F, // 1
        T, F, T, F, F, F, F, F, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        F, T, T, F, F, F, F, F, // 7
        F, T, F, F, F, F, T, F, // 8
        // e
        T, F, F, F, F, F, F, T, // 1
        T, F, T, F, F, F, F, F, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        F, T, T, F, F, F, F, F, // 7
        F, T, F, F, F, F, F, T, // 8
        // f
        T, F, F, F, F, T, F, F, // 1
        T, F, T, F, F, F, F, F, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        F, T, T, F, F, F, F, F, // 7
        F, T, F, F, F, T, F, F, // 8
        // g
        T, F, F, F, T, F, F, F, // 1
        T, F, T, F, F, F, F, F, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        F, T, T, F, F, F, F, F, // 7
        F, T, F, F, T, F, F, F, // 8
        // h
        T, F, F, T, F, F, F, F, // 1
        T, F, T, F, F, F, F, F, // 2
        F, F, F, F, F, F, F, F, // 3
        F, F, F, F, F, F, F, F, // 4
        F, F, F, F, F, F, F, F, // 5
        F, F, F, F, F, F, F, F, // 6
        F, T, T, F, F, F, F, F, // 7
        F, T, F, T, F, F, F, F, // 8
    ];
    let mut layer = Nnue {
        biases: [0f32; OUTPUTS],
        weights: [[0f32; INPUTS]; OUTPUTS],
        cache: [0f32; OUTPUTS],
    };
    layer.evaluate(&board, vec![1]);
}
