extern crate rand;

mod chess;
mod network;

use {
	chess::Chess,
	network::Network,
	std::io,
	std::sync::atomic::{AtomicBool, Ordering},
};

static DEBUG: AtomicBool = AtomicBool::new(true);
macro_rules! info {
	($($message: expr),*) => {
		println!("info string {}", format!($($message),*))
	};
}
macro_rules! debug {
	($($message: expr),*) => {
		if DEBUG.load(Ordering::Relaxed) {
			info!($($message),*)
		}
	};
}

struct GoSettings {
	_searchmoves: Option<Vec<(usize, usize)>>,
	_ponder: Option<bool>,
	_wtime: Option<usize>,
	_btime: Option<usize>,
	_winc: Option<usize>,
	_binc: Option<usize>,
	_movestogo: Option<usize>,
	_depth: Option<usize>,
	_nodes: Option<usize>,
	_mate: Option<usize>,
	_movetime: Option<usize>,
	infinite: Option<bool>,
}

struct Settings {
	go: GoSettings,
}

struct Engine {
	chess: Chess,
	network: Network,
	settings: Settings,
}

fn main() -> io::Result<()> {
	let mut engine = Engine {
		chess: Chess::new(),
		network: Network::new(true),
		settings: Settings {
			go: GoSettings {
				_searchmoves: None,
				_ponder: None,
				_wtime: None,
				_btime: None,
				_winc: None,
				_binc: None,
				_movestogo: None,
				_depth: None,
				_nodes: None,
				_mate: None,
				_movetime: None,
				infinite: None,
			},
		},
	};

	loop {
		debug!("{}", engine.chess);
		let mut line = String::new();
		io::stdin().read_line(&mut line)?;
		let buffer = &mut line.split_whitespace();

		while let Some(command) = buffer.next() {
			if engine.settings.go.infinite.unwrap_or(false) {
				if command == "stop" {
					engine.settings.go.infinite = None
				}
				continue;
			}

			match command {
				"uci" => {
					println!("id name gm2020");
					println!("id author sasha");
					println!("uciok");
					println!("readyok");
				}
				"debug" => match buffer.next() {
					Some("on") => DEBUG.store(true, Ordering::Relaxed),
					Some("off") => DEBUG.store(false, Ordering::Relaxed),
					_ => info!("'debug' must be followed by 'on' or 'off'"),
				},
				"isready" => println!("readyok"),
				"setoption" => match buffer.next() {
					Some("name") => match buffer.next() {
						Some(_id) => info!("gm2020 doesn't have any options"),
						_ => info!("'setoption name' must be followed by an option name"),
					},
					_ => info!("'setoption' must be followed by 'name'"),
				},
				"ucinewgame" => (),
				"position" => {
					match buffer.next() {
						Some("startpos") => engine.chess = Chess::new(),
						Some("fen") => match buffer.next() {
							Some(fen) => match Chess::from_fen(fen) {
								Ok(chess) => engine.chess = chess,
								_ => info!("Unable to parse FEN string: {}", fen),
							},
							_ => info!("'position fen' must followed by FEN notation"),
						},
						_ => info!("'position' must be followed by 'startpos' or 'fen'"),
					}
					match buffer.next() {
						Some("moves") => while let Some(_move) = buffer.next() {},
						_ => info!("'position' must specify moves"),
					}
				}
				"go" => {
					while let Some(setting) = buffer.next() {
						match setting {
							"searchmoves" => {}
							"ponder" => {}
							"wtime" => {}
							"btime" => {}
							"winc" => {}
							"binc" => {}
							"movestogo" => {}
							"depth" => {}
							"nodes" => {}
							"mate" => {}
							"movetime" => {}
							"infinite" => engine.settings.go.infinite = Some(true),
							setting => {
								info!("{} was not recognized as a subcommand for 'go'", setting)
							}
						};
					}
					let moves = engine.network.policy(&engine.chess);
					debug!("{:?}", moves);
					engine.chess.make_move(&moves[0].1);
					println!("info depth {}", 10);
					println!("bestmove {}", moves[0].1)
				}
				"quit" => {
					return Ok(());
				}
				_ => {
					debug!("Unrecognized command: {}", command)
				}
			}
		}
	}
	// network.playout(playout);
	// network.update_cache(Move {
	//     from: square(E, 2),
	//     to: (square(E, 4), WHITE_PAWN),
	//     clear_square: None,
	//     rook_square: None,
	// });
}

#[cfg(test)]
mod tests {
	impl Chess {
		fn play_and_count(
			&mut self,
			from: (File, u8),
			to: (File, u8),
			piece: [bool; 8],
			num_moves: usize,
		) {
			let moves = self.move_list();
			let move_ = moves
				.iter()
				.find(|move_| {
					move_.from == square(from.0, from.1) && move_.to == square(to.0, to.1)
				})
				.expect(&format!(
					"unable to find move in move list - from: {:?} to: {:?}",
					from, to
				));

			assert!(piece == move_.piece);

			self.make_move(&move_);

			let mut moves = (self.move_list().iter())
				.map(|move_| format!("{}", move_))
				.collect::<Vec<_>>();
			moves.sort();
			println!("position after {}:", move_);
			println!("{}", self);
			println!(
				"{}",
				(moves.iter().enumerate())
					.map(|(i, move_)| format!("\n({}) {}", i, move_))
					.collect::<String>()
			);

			assert!(
				num_moves == moves.len(),
				"expected {} moves but counted {}",
				num_moves,
				moves.len(),
			);
		}
	}

	use super::chess::{constants::pieces::*, File::*, *};

	#[test]
	fn king_basic() {
		let mut game = Chess::from_fen("7k/8/8/8/8/8/8/K7 w - - 0 1").unwrap();
		for (from, to, piece, num_legal_moves) in [
			((A, 1), (B, 2), WHITE_KING, 3),
			((H, 8), (H, 7), BLACK_KING, 8),
			((B, 2), (C, 1), WHITE_KING, 5),
			((H, 7), (G, 8), BLACK_KING, 5),
		] {
			game.play_and_count(from, to, piece, num_legal_moves);
		}
	}

	#[test]
	fn rook_basic() {
		let mut game = Chess::from_fen("7k/8/8/7r/R7/8/8/K7 b - - 0 1").unwrap();
		for (from, to, piece, num_legal_moves) in [
			((H, 5), (H, 4), BLACK_ROOK, 16),
			((A, 4), (D, 4), WHITE_ROOK, 13),
			((H, 4), (E, 4), BLACK_ROOK, 14),
			((D, 4), (D, 5), WHITE_ROOK, 17),
			((E, 4), (D, 4), BLACK_ROOK, 14),
		] {
			game.play_and_count(from, to, piece, num_legal_moves);
		}
	}

	#[test]
	fn bishop_basic() {
		let mut game = Chess::from_fen("7k/8/8/7b/B7/8/8/K7 w - - 0 1").unwrap();
		for (from, to, piece, num_legal_moves) in [
			((A, 4), (C, 6), WHITE_BISHOP, 10),
			((H, 5), (F, 3), BLACK_BISHOP, 12),
			((C, 6), (E, 4), WHITE_BISHOP, 10),
			((F, 3), (E, 2), BLACK_BISHOP, 16),
		] {
			game.play_and_count(from, to, piece, num_legal_moves);
		}
	}

	#[test]
	fn knight_basic() {
		let mut game = Chess::from_fen("7k/8/8/7n/N7/8/8/K7 b - - 0 1").unwrap();
		for (from, to, piece, num_legal_moves) in [
			((H, 5), (F, 4), BLACK_KNIGHT, 7),
			((A, 4), (C, 5), WHITE_KNIGHT, 11),
			((F, 4), (E, 6), BLACK_KNIGHT, 11),
		] {
			game.play_and_count(from, to, piece, num_legal_moves);
		}
	}

	#[test]
	fn blocking() {
		let mut game = Chess::from_fen("7k/3n3b/8/1Bn5/8/R2P2R1/2QPP3/K1N5 b - - 0 1").unwrap();
		for (from, to, piece, num_legal_moves) in [
			((D, 7), (E, 5), BLACK_KNIGHT, 41),
			((G, 3), (E, 3), WHITE_ROOK, 23),
			((E, 5), (D, 3), BLACK_KNIGHT, 42),
			((E, 3), (D, 3), WHITE_ROOK, 15),
			((H, 7), (E, 4), BLACK_BISHOP, 40),
			((D, 3), (D, 5), WHITE_ROOK, 19),
			((E, 4), (F, 5), BLACK_BISHOP, 50),
			((D, 5), (D, 3), WHITE_ROOK, 20),
			((C, 5), (D, 3), BLACK_KNIGHT, 38),
			((E, 2), (E, 3), WHITE_PAWN, 19),
			((D, 3), (E, 5), BLACK_KNIGHT, 43),
		] {
			game.play_and_count(from, to, piece, num_legal_moves);
		}
	}

	#[test]
	fn en_passant() {
		let mut game =
			Chess::from_fen("2N4k/pbpppnp1/Pp3n2/R6r/2PPPP1p/2N2B2/1P4PP/K7 w - - 0 1").unwrap();
		for (from, to, piece, num_legal_moves) in [
			((E, 4), (E, 5), WHITE_PAWN, 35),
			((D, 7), (D, 5), BLACK_PAWN, 38),
			((E, 5), (D, 6), WHITE_PAWN, 41),
			((C, 7), (C, 5), BLACK_PAWN, 37),
			((D, 4), (D, 5), WHITE_PAWN, 33),
			((E, 7), (E, 5), BLACK_PAWN, 32),
			((G, 2), (G, 4), WHITE_PAWN, 30),
			((B, 6), (B, 5), BLACK_PAWN, 31),
			((B, 2), (B, 4), WHITE_PAWN, 29),
		] {
			game.play_and_count(from, to, piece, num_legal_moves);
		}
	}

	#[test]
	fn normal_move_counts() {
		let mut game = Chess::new();
		for (from, to, piece, num_legal_moves) in [
			((E, 2), (E, 4), WHITE_PAWN, 20),
			((C, 7), (C, 5), BLACK_PAWN, 30),
			((E, 1), (E, 2), WHITE_KING, 22),
			((B, 8), (C, 6), BLACK_KNIGHT, 24),
			((B, 2), (B, 4), WHITE_PAWN, 27),
			((C, 5), (C, 4), BLACK_PAWN, 24),
			((C, 2), (C, 3), WHITE_PAWN, 25),
		] {
			game.play_and_count(from, to, piece, num_legal_moves);
		}
	}

	#[test]
	fn castling() {
		let mut game = Chess::from_fen("4k2r/7p/8/8/8/8/8/4KBNR w Kk - 0 1").unwrap();
		for (from, to, piece, num_legal_moves) in [
			((F, 1), (E, 2), WHITE_BISHOP, 10),
			((H, 7), (H, 5), BLACK_PAWN, 19),
			((G, 1), (F, 3), WHITE_KNIGHT, 11),
			((H, 5), (H, 4), BLACK_PAWN, 23),
			((E, 2), (C, 4), WHITE_BISHOP, 11),
		] {
			game.play_and_count(from, to, piece, num_legal_moves);
		}
	}

	// #[test]
	// fn castle_move_counts() {
	// 	let mut game = Chess::new();
	// 	for (from, to, piece, num_legal_moves) in [
	// 		((E, 2), (E, 4), WHITE_PAWN, 20),
	// 		((C, 7), (C, 5), BLACK_PAWN, 30),
	// 		((G, 1), (F, 3), WHITE_KNIGHT, 22),
	// 		((D, 7), (D, 6), BLACK_PAWN, 28),
	// 		((D, 2), (D, 4), WHITE_PAWN, 30),
	// 		((C, 5), (D, 4), BLACK_PAWN, 37),
	// 		((F, 3), (D, 4), WHITE_KNIGHT, 1),
	// 		((G, 8), (F, 6), BLACK_KNIGHT, 1),
	// 		((B, 1), (C, 3), WHITE_KNIGHT, 1),
	// 		((A, 7), (A, 6), BLACK_PAWN, 1),
	// 		((C, 1), (E, 3), WHITE_BISHOP, 1),
	// 		((E, 7), (E, 5), BLACK_PAWN, 1),
	// 		((D, 4), (B, 3), WHITE_KNIGHT, 1),
	// 		((C, 8), (E, 6), BLACK_BISHOP, 1),
	// 		((F, 2), (F, 3), WHITE_PAWN, 1),
	// 		((F, 8), (E, 7), BLACK_BISHOP, 1),
	// 		((D, 1), (D, 2), WHITE_QUEEN, 1),
	// 		((E, 8), (G, 8), BLACK_KING, 1),
	// 		((E, 1), (C, 1), WHITE_KING, 1),
	// 		((B, 8), (D, 7), BLACK_KNIGHT, 1),
	// 	] {
	// 		game.play_and_count(from, to, piece, num_legal_moves);
	// 	}
	// }

	// #[test]
	// fn en_passant_move_counts() {
	// 	let mut game = Chess::new();
	// 	play_and_count(&mut game, square(E, 2), square(E, 4), WHITE_PAWN, 20);
	// 	play_and_count(&mut game, square(D, 7), square(D, 5), BLACK_PAWN, 31);
	// 	play_and_count(&mut game, square(E, 4), square(E, 5), BLACK_PAWN, 30);
	// }

	// #[test]
	// fn promotion_move_counts() {
	// 	let mut game = Chess::new();
	// 	play_and_count(&mut game, square(E, 2), square(E, 4), WHITE_PAWN, 20);
	// 	play_and_count(&mut game, square(D, 7), square(D, 5), BLACK_PAWN, 31);
	// 	play_and_count(&mut game, square(E, 4), square(E, 5), BLACK_PAWN, 30);
	// }
}
