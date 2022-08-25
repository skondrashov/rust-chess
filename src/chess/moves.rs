use {
	super::{
		constants::pieces::*,
		{File::*, *},
	},
	crate::same_color,
};

impl Chess {
	pub fn moves(&self) -> Vec<Move> {
		self
			.pieces
			.iter()
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
							to: (destination, destination_piece),
							clear_square: None,
							rook_square: None,
						})
						.collect::<Vec<Move>>()
					})
					.chain({
						let rank = if piece.is_white() { 1 } else { 8 };
						if piece.can_castle_long {
							Some(Move {
								from: square(E, rank),
								to: (square(C, rank), piece.features),
								clear_square: Some(square(A, rank)),
								rook_square: Some(square(D, rank)),
							})
						} else if piece.can_castle_short {
							Some(Move {
								from: square(E, rank),
								to: (square(G, rank), piece.features),
								clear_square: Some(square(H, rank)),
								rook_square: Some(square(F, rank)),
							})
						} else {
							None
						}
					})
					.chain(if piece.can_en_passant_left {
						Some(Move {
							from: location,
							to: (
								if piece.is_white() {
									location + 7
								} else {
									location - 9
								},
								piece.features,
							),
							clear_square: Some(location - 1),
							rook_square: None,
						})
					} else if piece.can_en_passant_right {
						Some(Move {
							from: location,
							to: (
								if piece.is_white() {
									location + 9
								} else {
									location - 7
								},
								piece.features,
							),
							clear_square: Some(location + 1),
							rook_square: None,
						})
					} else {
						None
					})
			})
			.collect()
	}
}
