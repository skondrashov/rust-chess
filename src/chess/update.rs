use {super::*, crate::same_color, constants::*};

// println!(
// 	"AAAAAAa {} {} {} | {} {} {}",
// 	FILES[(i % 8) as usize],
// 	i / 8 + 1,
// 	piece_features.to_string(),
// 	FILES[blocked_x as usize],
// 	blocked_y + 1,
// 	blocked_piece_features.to_string()
// );

macro_rules! add_destination {
	($pieces:expr, $source:expr, $destination:expr) => {
		$pieces
			.get_mut(&($source as usize))
			.unwrap()
			.destinations
			.insert(($destination as usize))
	};
}

macro_rules! remove_destination {
	($pieces:expr, $source:expr, $destination:expr) => {
		$pieces
			.get_mut(&$source)
			.unwrap()
			.destinations
			.remove(&($destination as usize))
	};
}

impl Chess {
	pub fn update_pieces(&mut self, move_: &Move) -> Option<Result> {
		let to_i = move_.to.0 as isize;

		let mut piece = self.pieces.remove(&move_.from).unwrap();
		let piece_features = move_.to.1;
		piece.destinations.clear();

		// castling and en passant remove an extra piece
		let cleared_piece = if let Some(clear_square) = move_.clear_square {
			self.pieces.remove(&clear_square)
		} else {
			None
		};

		for &i in [Some(move_.from), move_.clear_square].iter().flatten() {
			let clear_i = i as isize;
			let (clear_x, clear_y) = (clear_i % 8, clear_i / 8);
			for (target_x, target_y) in KNIGHT_MOVES
				.iter()
				.map(|&(dx, dy)| (clear_x + dx, clear_y + dy))
				.filter(|&(target_x, target_y)| {
					(0..8).contains(&target_x) && (0..8).contains(&target_y)
				}) {
				if let Some(blocked_piece) =
					self.pieces.get_mut(&((target_x + target_y * 8) as usize))
				{
					if blocked_piece.is_knight() {
						blocked_piece.destinations.insert(i);
					}
				}
			}
			for (dx, dy) in DIRECTIONS {
				let (mut blocked_x, mut blocked_y) = (clear_x + dx, clear_y + dy);
				while (0..8).contains(&blocked_x) && (0..8).contains(&blocked_y) {
					let blocked_i = (blocked_x + blocked_y * 8) as usize;
					if !self.pieces.contains_key(&blocked_i) {
						blocked_x += dx;
						blocked_y += dy;
						continue;
					}

					let blocked_piece_features = self.pieces[&blocked_i].features;
					if blocked_piece_features.is_queen()
						|| if dx * dy == 0 {
							blocked_piece_features.is_rook()
						} else {
							blocked_piece_features.is_bishop()
						} {
						let (mut target_x, mut target_y) = (blocked_x - dx, blocked_y - dy);
						while (0..8).contains(&target_x) && (0..8).contains(&target_y) {
							let target_i = (target_x + target_y * 8) as usize;
							if target_i == move_.to.0 || Some(target_i) == move_.rook_square {
								break;
							}
							if let Some(target_piece) = self.pieces.get(&target_i) {
								if !same_color!(target_piece, blocked_piece_features) {
									if target_piece.is_king()
										&& same_color!(self.to_play, target_piece)
									{
										return Some(self.loss());
									}
									add_destination!(self.pieces, blocked_i, target_i);
								}
								break;
							}
							add_destination!(self.pieces, blocked_i, target_i);
							target_x -= dx;
							target_y -= dy;
						}
					} else if blocked_piece_features.is_king()
						&& (blocked_x - clear_x).abs() < 2
						&& (blocked_y - clear_y).abs() < 2
					{
						add_destination!(self.pieces, blocked_i, clear_i as usize);
					} else if blocked_piece_features.is_pawn()
						&& (blocked_piece_features.is_white() && blocked_y < clear_y
							|| blocked_piece_features.is_black() && blocked_y > clear_y)
					{
						if clear_x == blocked_x {
							if (clear_y - blocked_y).abs() == 1
								|| (clear_y - blocked_y).abs() == 2
									&& (blocked_y == 1 || blocked_y == 6)
							{
								add_destination!(self.pieces, blocked_i, clear_i as usize);
							}

							// check for a piece behind the piece if we moved out of the way of a pawn that can move 2 squares
							if blocked_y == 1 && clear_y == 2 {
								if self.pieces.get(&(i + 8)).is_none() {
									add_destination!(self.pieces, blocked_i, clear_i + 8);
								}
							} else if blocked_y == 6 && clear_y == 5 {
								if self.pieces.get(&(i - 8)).is_none() {
									add_destination!(self.pieces, blocked_i, clear_i - 8);
								}
							} else {
							}
						} else if !same_color!(blocked_piece_features, piece_features)
							&& (clear_x - blocked_x).abs() == 1
							&& (clear_y - blocked_y).abs() == 1
						{
							remove_destination!(self.pieces, blocked_i, clear_i);
						}
					}
					break;
				}
			}
		}

		self.pieces.insert(move_.to.0, piece);
		// castling also places the removed piece (the rook)
		if let Some(mut rook) = cleared_piece {
			let rook_features = rook.features;
			if let Some(i) = move_.rook_square {
				rook.destinations.clear();
				self.pieces.insert(i, rook);
				let rook_i = i as isize;
				let (rook_x, rook_y) = (rook_i % 8, rook_i / 8);
				for blocked_i in KNIGHT_MOVES
					.iter()
					.map(|&(dx, dy)| ((rook_x + dx) + (rook_y + dy) * 8) as usize)
				{
					if let Some(blocked_piece) = self.pieces.get_mut(&blocked_i) {
						if blocked_piece.is_knight() && same_color!(blocked_piece, piece_features) {
							blocked_piece.destinations.insert(i);
						}
					}
				}

				self.block_sliders(rook_i, rook_features);
			}
		}

		// TODO: check if castling is possible

		let (to_x, to_y) = (to_i % 8, to_i / 8);
		for (dx, dy) in KNIGHT_MOVES {
			let (blocked_x, blocked_y) = (to_x + dx, to_y + dy);
			if (0..8).contains(&blocked_x) && (0..8).contains(&blocked_y) {
				let blocked_i = (blocked_x + blocked_y * 8) as usize;
				if self.pieces.contains_key(&blocked_i) {
					let blocked_piece_features = self.pieces[&blocked_i].features;
					if same_color!(blocked_piece_features, piece_features) {
						if blocked_piece_features.is_knight() {
							remove_destination!(self.pieces, blocked_i, to_i);
						}
					} else if piece_features.is_king() && blocked_piece_features.is_knight() {
						return Some(self.loss());
					} else if piece_features.is_knight() {
						add_destination!(self.pieces, to_i, blocked_i);
					}
				} else if piece_features.is_knight() {
					add_destination!(self.pieces, to_i, blocked_i);
				}
			}
		}
		self.block_sliders(to_i, piece_features);
		// en passantable pawn
		//     if from_piece.is_pawn()
		//     && !from_piece.same_color(blocked_piece)
		//     && (from_i - blocked_i).abs() == 1
		// {
		//     blocked_piece.destinations.insert(to_i)
		// }

		self.to_play = if self.to_play.is_white() {
			BLACK
		} else {
			WHITE
		};
		None
	}

	#[inline(always)]
	fn block_knights(&mut self, i: isize, piece_features: [bool; 8]) -> Option<()> {
		Some(())
	}

	#[inline(always)]
	fn block_sliders(&mut self, i: isize, piece_features: [bool; 8]) {
		let (x, y) = (i % 8, i / 8);
		for (dx, dy) in DIRECTIONS {
			let (mut blocked_x, mut blocked_y) = (x + dx, y + dy);
			while (0..8).contains(&blocked_x) && (0..8).contains(&blocked_y) {
				let blocked_i = (blocked_x + blocked_y * 8) as usize;
				if !self.pieces.contains_key(&blocked_i) {
					if piece_features.is_queen()
						|| if dx * dy == 0 {
							piece_features.is_rook()
						} else {
							piece_features.is_bishop()
						} || piece_features.is_king()
						&& (x - blocked_x).abs() < 2
						&& (y - blocked_y).abs() < 2
						|| piece_features.is_pawn()
							&& x == blocked_x && (piece_features.is_white() && y - blocked_y == -1
							|| piece_features.is_black() && y - blocked_y == 1)
					{
						add_destination!(self.pieces, i, blocked_i);
					}
					blocked_x += dx;
					blocked_y += dy;
					continue;
				}

				let blocked_piece_features = self.pieces[&blocked_i].features;
				if !same_color!(piece_features, blocked_piece_features)
					&& (piece_features.is_queen()
						|| if dx * dy == 0 {
							piece_features.is_rook()
						} else {
							piece_features.is_bishop()
						} || piece_features.is_king()
						&& (x - blocked_x).abs() < 2
						&& (y - blocked_y).abs() < 2
						|| piece_features.is_pawn()
							&& (x - blocked_x).abs() == 1
							&& (piece_features.is_white() && y - blocked_y == -1
								|| piece_features.is_black() && y - blocked_y == 1))
				{
					add_destination!(self.pieces, i, blocked_i);
				}

				if blocked_piece_features.is_queen()
					|| if dx * dy == 0 {
						blocked_piece_features.is_rook()
					} else {
						blocked_piece_features.is_bishop()
					} {
					if same_color!(blocked_piece_features, piece_features) {
						remove_destination!(self.pieces, blocked_i, i);
					}
					let (mut target_x, mut target_y) = (x - dx, y - dy);
					while (0..8).contains(&target_x)
						&& (0..8).contains(&target_y)
						&& remove_destination!(self.pieces, blocked_i, target_x + target_y * 8)
					{
						target_x -= dx;
						target_y -= dy;
					}
				} else if blocked_piece_features.is_king()
					&& (blocked_x - x).abs() < 2
					&& (blocked_y - y).abs() < 2
					&& same_color!(blocked_piece_features, piece_features)
				{
					remove_destination!(self.pieces, blocked_i, i);
				} else if blocked_piece_features.is_pawn()
					&& (blocked_piece_features.is_white() && blocked_y < y
						|| blocked_piece_features.is_black() && blocked_y > y)
				{
					if x == blocked_x {
						if (y - blocked_y).abs() == 1 {
							remove_destination!(self.pieces, blocked_i, i - 8);
							remove_destination!(self.pieces, blocked_i, i);
							remove_destination!(self.pieces, blocked_i, i + 8);
						} else if (y - blocked_y).abs() == 2 && (blocked_y == 1 || blocked_y == 6) {
							remove_destination!(self.pieces, blocked_i, i);
						}
					} else if !same_color!(blocked_piece_features, piece_features)
						&& (x - blocked_x).abs() == 1
						&& (y - blocked_y).abs() == 1
					{
						add_destination!(self.pieces, blocked_i, i);
					}
				}
				break;
			}
		}
	}

	fn loss(&self) -> Result {
		if self.to_play.is_white() {
			Result::BLACK
		} else {
			Result::WHITE
		}
	}
}
