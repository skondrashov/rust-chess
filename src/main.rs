extern crate rand;

mod chess;
mod network;

use {chess::Chess, network::Network, std::io};

fn main() -> io::Result<()> {
	let mut game = Chess::new(None);
	println!("{}", game);
	let mut network = Network::new();
	let mut buffer = String::new();
	while buffer != "quit" {
		buffer = String::new();
		io::stdin().read_line(&mut buffer)?;
		let buffer = buffer.split_whitespace().collect::<Vec<_>>();
		let mut buffer = buffer.iter();
		let &command = buffer.next().unwrap_or(&"");
		if command == "uci" {
			println!("id name gm2020");
			println!("id author sasha");
			println!("uciok");
			println!("readyok");
		} else if command == "isready" {
			println!("readyok");
		} else if command == "ucinewgame" {
			game = Chess::new(None);
		} else if command == "position" {
			let &position = buffer.next().unwrap_or(&"");
			if position == "startpos" {
				game = Chess::new(None);
			}
		} else if command == "go" {
			let moves = network.policy(&game);
			println!("{:?}", moves);
			game.update_pieces(&moves[0].1);
		} else if command == "force" {
			let moves = game.moves();
			for (i, move_) in moves.iter().enumerate() {
				print!("({}) {:?}, ", i, move_);
			}
			println!();
			let mut buffer = String::new();
			io::stdin().read_line(&mut buffer)?;
			let move_ = &moves[buffer.trim().parse::<usize>().unwrap()];
			println!("\nplaying {:?}", move_);
			game.update_pieces(move_);
			println!("{}", game);
		}
	}

	// network.playout(playout);
	// network.update_cache(Move {
	//     from: square(E, 2),
	//     to: (square(E, 4), WHITE_PAWN),
	//     clear_square: None,
	//     rook_square: None,
	// });
	Ok(())
}

#[cfg(test)]
mod tests {
	impl Chess {
		fn play_and_count(&mut self, from: usize, to: usize, square: [bool; 8], num_moves: usize) {
			self.update_pieces(&Move::new(from, to, square));
			let moves = self.moves();
			let mut moves = moves
				.iter()
				.map(|move_| format!("{:?}", move_))
				.collect::<Vec<_>>();
			moves.sort();
			assert!(
				moves.len() == num_moves,
				"{} != {}: {}",
				moves.len(),
				num_moves,
				moves
					.iter()
					.enumerate()
					.map(|(i, move_)| format!("\n({}) {}", i, move_))
					.collect::<String>()
			);
		}
	}

	impl Move {
		pub fn new(from: usize, to: usize, piece: Square) -> Self {
			Move {
				from,
				to: (to, piece),
				clear_square: None,
				rook_square: None,
			}
		}
	}

	use super::chess::{constants::pieces::*, File::*, *};

	#[test]
	fn normal_move_counts() {
		let mut game = Chess::new(None);
		game.play_and_count(square(E, 2), square(E, 4), WHITE_PAWN, 20);
		game.play_and_count(square(C, 7), square(C, 5), BLACK_PAWN, 30);
		game.play_and_count(square(E, 1), square(E, 2), WHITE_KING, 22);
		game.play_and_count(square(B, 8), square(C, 6), BLACK_KNIGHT, 24);
		game.play_and_count(square(B, 2), square(B, 4), WHITE_PAWN, 27);
		game.play_and_count(square(C, 5), square(C, 4), BLACK_PAWN, 24);
		game.play_and_count(square(C, 2), square(C, 3), WHITE_PAWN, 25);
	}

	// #[test]
	// fn castle_move_counts() {
	// 	let mut game = Chess::new(None);
	// 	play_and_count(&mut game, square(E, 2), square(E, 3), WHITE_PAWN, 20);
	// 	play_and_count(&mut game, square(D, 7), square(D, 5), BLACK_PAWN, 20);
	// 	play_and_count(&mut game, square(E, 4), square(E, 5), BLACK_PAWN, 30);
	// }

	// #[test]
	// fn en_passant_move_counts() {
	// 	let mut game = Chess::new(None);
	// 	play_and_count(&mut game, square(E, 2), square(E, 4), WHITE_PAWN, 20);
	// 	play_and_count(&mut game, square(D, 7), square(D, 5), BLACK_PAWN, 31);
	// 	play_and_count(&mut game, square(E, 4), square(E, 5), BLACK_PAWN, 30);
	// }

	// #[test]
	// fn promotion_move_counts() {
	// 	let mut game = Chess::new(None);
	// 	play_and_count(&mut game, square(E, 2), square(E, 4), WHITE_PAWN, 20);
	// 	play_and_count(&mut game, square(D, 7), square(D, 5), BLACK_PAWN, 31);
	// 	play_and_count(&mut game, square(E, 4), square(E, 5), BLACK_PAWN, 30);
	// }
}
