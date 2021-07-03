use super::types::{
    castle_move_flags, en_passant_flags, file, moves, piece_color, piece_type, rank, square,
    CastleMoveFlags, EnPassantFlags, Move, Piece,
};

pub const SQUARES_AT: usize = 0;
pub const SQUARES_SIZE: usize = 64;
pub const NOOP_SQUARE_AT: usize = SQUARES_SIZE;
pub const NOOP_SQUARE_SIZE: usize = 1;
pub const TOTAL_SIZE: usize = NOOP_SQUARE_AT + NOOP_SQUARE_SIZE;

pub type BoardArray = [Piece; SQUARES_SIZE];

pub struct Board {
    squares: BoardArray,
    move_castle_flags: CastleMoveFlags,
    en_passant_flags: EnPassantFlags,
}

pub fn make_move(board: &mut Board, mv: Move) {
    // let captured_piece: Square = mv & move_masks::CAPTURED_PIECE_MASK;
}

pub fn unmake_move(board: &mut Board, mv: Move) {}

pub fn from_fen(fen_string: String) -> Board {
    Board
}

pub fn to_fen(board: Board) -> String {
    String("")
}
