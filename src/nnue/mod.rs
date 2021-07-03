// https://github.com/asdfjkl/nnue/blob/main/nnue_en.pdf
use crate::constants::{INPUTS, OUTPUTS};

pub struct Nnue {
   pub biases: [f32; OUTPUTS],
   pub weights: [[f32; INPUTS]; OUTPUTS],
   pub cache: [f32; OUTPUTS],
}

impl Nnue {
   pub fn evaluate(&mut self, input: &[u8], indices: Vec<usize>) {
      let mut i = 0;
      for position in indices {
         self.cache[i] += input[position] as f32 * self.weights[i][position];
         i += 1
      }
   }
}
