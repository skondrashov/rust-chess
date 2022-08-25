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
	pub fn update_pieces(&mut self, move_: &Move) {
		let from_i = move_.from as isize;
		let to_i = move_.to.0 as isize;

		let mut piece = self.pieces.remove(&move_.from).unwrap();
		let piece_features = move_.to.1;
		piece.destinations.clear();

		// castling and en passant remove an extra piece
		if let Some(clear_square) = move_.clear_square {
			let mut cleared_piece = self.pieces.remove(&clear_square).unwrap();
			cleared_piece.destinations.clear();
			self.unblock_knights(clear_square as isize);
			self.unblock_sliders(clear_square as isize, cleared_piece.features);

			self.unblock_knights(from_i);
			self.unblock_sliders(from_i, piece_features);
			self.pieces.insert(move_.to.0, piece);

			// castling also places that removed piece (the rook)
			if let Some(rook_square) = move_.rook_square {
				let cleared_piece_features = cleared_piece.features;
				self.pieces.insert(rook_square, cleared_piece);
				self.block_knights(rook_square as isize, cleared_piece_features);
				self.block_sliders(rook_square as isize, cleared_piece_features);
			}
		} else {
			self.unblock_knights(from_i);
			self.unblock_sliders(from_i, piece_features);
			self.pieces.insert(move_.to.0, piece);
		}
		// TODO: check if castling is possible

		self.block_knights(to_i, piece_features);
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
	}

	#[inline(always)]
	fn unblock_knights(&mut self, i: isize) {
		let (x, y) = (i % 8, i / 8);
		for (dx, dy) in KNIGHT_MOVES {
			let (target_x, target_y) = (x + dx, y + dy);
			if (0..8).contains(&target_x) && (0..8).contains(&target_y) {
				let blocked_i = (target_x + target_y * 8) as usize;
				if let Some(blocked_piece) = self.pieces.get_mut(&blocked_i) {
					if blocked_piece.is_knight() {
						blocked_piece.destinations.insert(i as usize);
					}
				}
			}
		}
	}

	#[inline(always)]
	fn unblock_sliders(&mut self, clear_i: isize, piece_features: [bool; 8]) {
		let (clear_x, clear_y) = (clear_i % 8, clear_i / 8);
		let clear_i = clear_i as usize;
		for (dx, dy) in DIRECTIONS {
			let (mut blocked_x, mut blocked_y) = (clear_x, clear_y);
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
						if let Some(target_piece) = self.pieces.get(&target_i) {
							if !same_color!(target_piece, blocked_piece_features) {
								add_destination!(self.pieces, blocked_i, target_i);
							}
							break;
						} else {
							add_destination!(self.pieces, blocked_i, target_i);
						}
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
							if self.pieces.get(&(clear_i + 8)).is_none() {
								add_destination!(self.pieces, blocked_i, clear_i + 8);
							}
						} else if blocked_y == 6 && clear_y == 5 {
							if self.pieces.get(&(clear_i - 8)).is_none() {
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

	#[inline(always)]
	fn block_knights(&mut self, i: isize, piece_features: [bool; 8]) {
		let (x, y) = (i % 8, i / 8);
		for (dx, dy) in KNIGHT_MOVES {
			let (blocked_x, blocked_y) = (x + dx, y + dy);
			if (0..8).contains(&blocked_x) && (0..8).contains(&blocked_y) {
				let blocked_i = (blocked_x + blocked_y * 8) as usize;
				if self.pieces.contains_key(&blocked_i)
					&& same_color!(self.pieces[&blocked_i].features, piece_features)
				{
					if self.pieces[&blocked_i].features.is_knight() {
						remove_destination!(self.pieces, blocked_i, i as usize);
					}
				} else if piece_features.is_knight() {
					add_destination!(self.pieces, i, blocked_i);
				}
			}
		}
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

	pub fn result(&self) -> Option<Result> {
		Some(Result::DRAW)
	}
}
