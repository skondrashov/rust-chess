extern crate rand;

mod chess;
mod network;

use {chess::Chess, network::Network, std::io};

fn main() -> io::Result<()> {
	let mut game = Chess::new(None);
	println!("{}", game.to_string());
	let mut _network = Network::new();
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
			let moves = game.moves();
			for (i, move_) in moves.iter().enumerate() {
				print!("({}) {}, ", i, move_.to_string());
			}
			println!("");
			let mut buffer = String::new();
			io::stdin().read_line(&mut buffer)?;
			let move_ = &moves[buffer.trim().parse::<usize>().unwrap()];
			println!("\nplaying {}", move_.to_string());
			game.update_pieces(&move_);
			println!("{}", game.to_string());
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
	fn play_and_count(
		game: &mut Chess,
		from: usize,
		to: usize,
		square: [bool; 8],
		num_moves: usize,
	) {
		game.update_pieces(&Move::new(from, to, square));
		let moves = game.moves();
		let mut moves = moves
			.iter()
			.map(|move_| move_.to_string())
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
				.map(|(i, move_)| format!("\n({}) {}", i, move_.to_string()))
				.collect::<String>()
		);
	}

	use super::chess::{constants::pieces::*, File::*, *};

	#[test]
	fn normal_move_counts() {
		let mut game = Chess::new(None);
		play_and_count(&mut game, square(E, 2), square(E, 4), WHITE_PAWN, 20);
		play_and_count(&mut game, square(C, 7), square(C, 5), BLACK_PAWN, 30);
		play_and_count(&mut game, square(E, 1), square(E, 2), WHITE_KING, 22);
		play_and_count(&mut game, square(B, 8), square(C, 6), BLACK_KNIGHT, 24);
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
}
