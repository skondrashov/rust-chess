use {
	super::{constants::pieces::*, File::*, *},
	crate::same_color,
	constants::{DIRECTIONS, KNIGHT_MOVES},
};

impl Chess {
	pub fn move_list(&self) -> Vec<Move> {
		(self.pieces.iter())
			.filter(|(_, piece)| same_color!(self.to_play, piece))
			.flat_map(|(&location, piece)| {
				piece
					.destinations
					.iter()
					.flat_map(move |&destination| {
						// expand pawn moves onto first/last rank into all possible promotions
						if piece.is_pawn() && destination > square(H, 7) {
							vec![WHITE_QUEEN, WHITE_KNIGHT, WHITE_BISHOP, WHITE_ROOK]
						} else if piece.is_pawn() && destination < square(A, 2) {
							vec![BLACK_QUEEN, BLACK_KNIGHT, BLACK_BISHOP, BLACK_ROOK]
						} else {
							vec![piece.features]
						}
						.iter()
						.map(|&destination_piece| Move {
							from: location,
							to: destination,
							piece: destination_piece,
							clear_square: None,
							rook_square: None,
						})
						.collect::<Vec<Move>>()
					})
					.chain({
						let rank = if piece.is_white() { 1 } else { 8 };

						if self.check.is_some() {
							None
						} else if piece.can_castle_long
							&& self.pieces.get(&square(B, rank)).is_none()
							&& self.pieces.get(&square(C, rank)).is_none()
							&& !self.is_square_threatened_by_opponent(square(B, rank))
							&& !self.is_square_threatened_by_opponent(square(C, rank))
						{
							Some(Move {
								from: square(E, rank),
								to: square(C, rank),
								piece: piece.features,
								clear_square: Some(square(A, rank)),
								rook_square: Some(square(D, rank)),
							})
						} else if piece.can_castle_short
							&& self.pieces.get(&square(F, rank)).is_none()
							&& self.pieces.get(&square(G, rank)).is_none()
							&& !self.is_square_threatened_by_opponent(square(F, rank))
							&& !self.is_square_threatened_by_opponent(square(G, rank))
						{
							Some(Move {
								from: square(E, rank),
								to: square(G, rank),
								piece: piece.features,
								clear_square: Some(square(H, rank)),
								rook_square: Some(square(F, rank)),
							})
						} else {
							None
						}
					})
					.chain(if let Some(turn) = piece.en_passant_left_turn {
						if turn == self.move_counter {
							Some(Move {
								from: location,
								to: if piece.is_white() {
									location + 7
								} else {
									location - 9
								},
								piece: piece.features,
								clear_square: Some(location - 1),
								rook_square: None,
							})
						} else {
							None
						}
					} else if let Some(turn) = piece.en_passant_right_turn {
						println!("b b b b ");
						if turn == self.move_counter {
							Some(Move {
								from: location,
								to: if piece.is_white() {
									location + 9
								} else {
									location - 7
								},
								piece: piece.features,
								clear_square: Some(location + 1),
								rook_square: None,
							})
						} else {
							None
						}
					} else {
						None
					})
			})
			.collect()
	}

	fn is_square_threatened_by_opponent(&self, i: usize) -> bool {
		let square_i = i as isize;
		let (square_x, square_y) = (square_i % 8, square_i / 8);
		for (target_x, target_y) in KNIGHT_MOVES
			.iter()
			.map(|&(dx, dy)| (square_x + dx, square_y + dy))
			.filter(|&(target_x, target_y)| {
				(0..8).contains(&target_x) && (0..8).contains(&target_y)
			}) {
			if let Some(threatening_piece) = self.pieces.get(&((target_x + target_y * 8) as usize))
			{
				if threatening_piece.is_knight() && !same_color!(self.to_play, threatening_piece) {
					return true;
				}
			}
		}
		for (dx, dy) in DIRECTIONS {
			let (mut target_x, mut target_y) = (square_x + dx, square_y + dy);
			while (0..8).contains(&target_x) && (0..8).contains(&target_y) {
				let blocked_i = (target_x + target_y * 8) as usize;
				if !self.pieces.contains_key(&blocked_i) {
					target_x += dx;
					target_y += dy;
					continue;
				}

				let threatening_piece_features = self.pieces[&blocked_i].features;
				if same_color!(self.to_play, threatening_piece_features) {
					break;
				}

				if threatening_piece_features.is_queen()
					|| if dx * dy == 0 {
						threatening_piece_features.is_rook()
					} else {
						threatening_piece_features.is_bishop()
					} || threatening_piece_features.is_king()
					&& (target_x - square_x).abs() < 2
					&& (target_y - square_y).abs() < 2
					|| threatening_piece_features.is_pawn()
						&& (square_x - target_x).abs() == 1
						&& (threatening_piece_features.is_white() && target_y == square_y - 1
							|| threatening_piece_features.is_black() && target_y == square_y + 1)
				{
					return true;
				}

				break;
			}
		}
		false
	}
}
