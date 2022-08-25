pub mod constants;

use {
	crate::chess::{
		constants::{
			pieces::{BLACK_ROOK, EMPTY_SQUARE, WHITE_ROOK},
			BOARD_SIZE, STARTING_BOARD,
		},
		Chess,
		Color::*,
		Move, Result, Square, SquareProperties,
	},
	constants::*,
};

// https://github.com/asdfjkl/nnue/blob/main/nnue_en.pdf
struct Nnue<const INPUT_COLUMNS: usize, const INPUT_COLUMN_SIZE: usize, const OUTPUTS: usize> {
	pub weights: [[[f32; INPUT_COLUMN_SIZE]; OUTPUTS]; INPUT_COLUMNS],
	pub biases: [f32; OUTPUTS],
	pub output_cache: [f32; OUTPUTS],
}

struct FullyConnected<const INPUTS: usize, const OUTPUTS: usize> {
	pub weights: [[f32; INPUTS]; OUTPUTS],
	pub biases: [f32; OUTPUTS],
}

pub struct Network {
	input: [Square; BOARD_SIZE],
	nnue: Nnue<64, { SQUARE_FEATURES }, { NNUE_OUTPUTS }>,
	fc1: FullyConnected<{ NNUE_OUTPUTS }, { FC1_OUTPUTS }>,
	win_evaluation: [f32; FC1_OUTPUTS],
}

#[inline(always)]
fn relu(n: f32) -> f32 {
	if n < 0.0 {
		0.0
	} else {
		n
	}
}

fn bool_dot(a: &[bool], b: &[f32]) -> f32 {
	a.iter()
		.zip(b.iter())
		.fold(0.0, |value, (&v, w)| if v { value + w } else { value })
}

fn dot(a: &[f32], b: &[f32]) -> f32 {
	a.iter()
		.zip(b.iter())
		.fold(0.0, |value, (v, w)| value + v * w)
}

impl Network {
	pub fn new() -> Self {
		Self {
			input: STARTING_BOARD,
			nnue: Nnue {
				weights: [[[0.0; SQUARE_FEATURES]; NNUE_OUTPUTS]; BOARD_SIZE],
				biases: [0.0; NNUE_OUTPUTS],
				output_cache: [0.0; NNUE_OUTPUTS],
			},
			fc1: FullyConnected {
				weights: [[0.0; FC1_OUTPUTS]; NNUE_OUTPUTS],
				biases: [0.0; FC1_OUTPUTS],
			},
			win_evaluation: [0.0; FC1_OUTPUTS],
		}
	}

	pub fn evaluate(&mut self, move_: Option<&Move>) -> f32 {
		let old_cache = if let Some(move_) = move_ {
			let old_cache = self.nnue.output_cache.clone();
			self.update_cache(&move_);
			Some(old_cache)
		} else {
			None
		};

		let evaluation = self
			.fc1
			.weights
			.iter()
			.zip(self.fc1.biases.iter())
			.zip(self.win_evaluation.iter())
			.fold(0.0, |evaluation, ((input, bias), weight)| {
				evaluation + relu(dot(input, &self.nnue.output_cache) + bias) * weight
			});

		if let Some(old_cache) = old_cache {
			self.nnue.output_cache = old_cache;
		}

		evaluation
	}

	pub fn move_evaluations(&mut self, game: &Chess) -> Vec<(f32, Move)> {
		game
			.moves()
			.into_iter()
			.map(|move_| (self.evaluate(Some(&move_)), move_))
			.collect()
	}

	pub fn policy(&mut self, game: &Chess) -> Vec<(f32, Move)> {
		let mut moves = self.move_evaluations(game);
		moves.sort_by(|(eval1, _), (eval2, _)| eval1.partial_cmp(&eval2).unwrap());
		moves
	}

	#[allow(non_snake_case)]
	pub fn update_cache(&mut self, move_: &Move) {
		let A = self.input;
		let W = self.nnue.weights;
		let B = self.nnue.biases;
		let i_from = move_.from;
		let i_to = move_.to.0;

		for i in 0..NNUE_OUTPUTS {
			self.nnue.output_cache[i] += (relu(B[i_from])
				- relu(bool_dot(&A[i_from], &W[i_from][i]) + B[i_from]))
				+ (relu(bool_dot(&move_.to.1, &W[i_to][i]) + B[i_to])
					- relu(bool_dot(&A[i_to], &W[i_to][i]) + B[i_to]));
			if let Some(i_clear) = move_.clear_square {
				self.nnue.output_cache[i] +=
					relu(B[i_clear]) - relu(bool_dot(&A[i_clear], &W[i_clear][i]) + B[i_clear])
			}
			if let Some(i_rook) = move_.rook_square {
				let rook = if move_.to.1.is_white() {
					WHITE_ROOK
				} else {
					BLACK_ROOK
				};
				self.nnue.output_cache[i] +=
					relu(bool_dot(&rook, &W[i_rook][i]) + B[i_rook]) - relu(B[i_rook])
			}
		}
	}

	pub fn playout(&mut self, mut game: Chess) -> Result {
		loop {
			if let Some(result) = game.result() {
				return result;
			}
			let move_ = self
				.move_evaluations(&game)
				.into_iter()
				.max_by(|(eval1, _), (eval2, _)| eval1.partial_cmp(&eval2).unwrap())
				.unwrap()
				.1;

			self.input[move_.from] = EMPTY_SQUARE;
			self.input[move_.to.0] = move_.to.1;
			if let Some(clear_square) = move_.clear_square {
				self.input[clear_square] = EMPTY_SQUARE;
			}
			if let Some(rook_square) = move_.rook_square {
				self.input[rook_square] = if game.to_play == WHITE {
					WHITE_ROOK
				} else {
					BLACK_ROOK
				};
			}

			game.update_pieces(&move_);
			self.update_cache(&move_);
		}
	}
}
