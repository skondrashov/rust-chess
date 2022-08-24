mod chess;
mod network;

use {
    chess::{constants::pieces::*, square, Chess, File::*, Move},
    network::Network,
};

fn main() {
    let mut game = Chess::new(None);
    for move_ in game.moves() {
        println!("{}", move_.to_string())
    }
    println!("-----------");
    game.update_pieces(&Move {
        from: square(B, 1),
        to: (square(A, 3), WHITE_KNIGHT),
        clear_square: None,
        rook_square: None,
    });
    for move_ in game.moves() {
        println!("{}", move_.to_string())
    }
    let mut _network = Network::new();
    // network.playout(playout);
    // network.update_cache(Move {
    //     from: square(E, 2),
    //     to: (square(E, 4), WHITE_PAWN),
    //     clear_square: None,
    //     rook_square: None,
    // });
}
