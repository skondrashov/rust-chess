// https://github.com/asdfjkl/nnue/blob/main/nnue_en.pdf
use crate::constants::{INPUT_COLUMNS, INPUT_COLUMN_SIZE, OUTPUTS};

pub struct Nnue {
   pub biases: [f32; OUTPUTS],
   pub weights: [[[f32; INPUT_COLUMN_SIZE]; OUTPUTS]; INPUT_COLUMNS],
   pub input_cache: [[u8; INPUT_COLUMN_SIZE]; INPUT_COLUMNS],
   pub output_cache: [f32; OUTPUTS],
}

impl Nnue {
   pub fn init(&mut self, input: [[u8; INPUT_COLUMN_SIZE]; INPUT_COLUMNS]) {
      self.input_cache = input;
   }

   pub fn evaluate(&mut self, changed_columns: Vec<(usize, [u8; INPUT_COLUMN_SIZE])>) {
      for (i, column) in changed_columns {
         for j in 0..OUTPUTS {
            for k in 0..INPUT_COLUMN_SIZE {
               self.output_cache[j] +=
                  (column[k] - self.input_cache[i][k]) as f32 * self.weights[i][j][k];
            }
         }
         self.input_cache[i] = column;
      }
   }
}
