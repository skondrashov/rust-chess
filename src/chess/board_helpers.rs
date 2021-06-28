use super::board::BoardArray;
use super::helpers::*;
use super::types::*;

#[inline(always)]
pub fn set_square(board_array: &mut BoardArray, pce: Piece, sq: square::Type) {
    board_array[sq] = pce
}

#[inline(always)]
pub fn clear_square(board_array: &mut BoardArray, sq: square::Type) {
    board_array[sq] = piece::NONE
}

#[inline(always)]
pub fn flip_to_move(cmflags: castle_move_flags::Type) -> castle_move_flags::Type {
    cmflags ^ castle_move_flags::SWAP_MOVE_FLAGS_MASK
}

#[inline(always)]
pub fn bool_to_mask(boolean: bool) -> u8 {
    !((boolean as u8) - 1)
}
