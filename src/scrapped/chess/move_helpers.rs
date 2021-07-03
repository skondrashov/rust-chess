use super::helpers::*;
use super::types::*;
use crate::chess::board_helpers::bool_to_mask;

#[inline(always)]
pub fn captured_piece(mv: Move) -> Piece {
    ((mv & moves::CAPTURED_PIECE_MASK) >> moves::CAPTURED_PIECE_MASK_SHIFT) as Piece
}

#[inline(always)]
pub fn move_from(mv: Move) -> square::Type {
    ((mv & moves::FROM_MASK) >> moves::FROM_MASK_SHIFT) as square::Type
}

#[inline(always)]
pub fn move_to(mv: Move) -> square::Type {
    ((mv & moves::TO_MASK) >> moves::TO_MASK_SHIFT) as square::Type
}

#[inline(always)]
pub fn move_castle_flags(mv: Move) -> castle_move_flags::Type {
    ((mv & moves::CASTLE_MASK) >> moves::CASTLE_MASK_SHIFT) as castle_move_flags::Type
}

#[inline(always)]
pub fn is_en_passant(mv: Move) -> bool {
    ((mv & moves::EN_PASSANT_MASK) >> moves::EN_PASSANT_MASK_SHIFT) > 0
}

#[inline(always)]
pub fn is_castle(mv: Move) -> bool {
    ((mv & moves::IS_CASTLE_MASK) >> moves::IS_CASTLE_MASK_SHIFT) > 0
}

#[inline(always)]
pub fn en_passant_capture_square(mv: Move) -> square::Type {
    is_ep: bool = is_en_passant(mv);
    is_ep_mask: square::Type = boolean_to_u8_mask(is_ep);
    is_not_ep_mask: square::Type = !is_ep_mask;

    white_move: Move = moves::WHITE_MOVE_COLOR_MASK & mv;
    black_move: Move = moves::BLACK_MOVE_COLOR_MASK & mv;

    is_ep_square: square::Type = (move_to(mv) + (u32_to_bool(white_move) as u8)
        - (u32_to_bool(black_move) as u8))
        & is_ep_mask;
    is_not_ep_square: square::Type = square::NOOP_SQUARE & is_not_ep_mask;
    is_en_square | is_not_ep_square
}

// In order to take a black castling move or flag and turn it into the a move or flag for the same
// kind of castling for white it must be shifted 2 bits to the left. This function figures out the
// number of bits (0 or 2) needed in order to make a castling mask (or flag) appropriate for the
// player whose move is being made.
#[inline(always)]
pub fn black_castle_shift_from_move(mv: Move) -> u8 {
    ((((mv & moves::MOVE_COLOR_MASK) >> (moves::MOVE_COLOR_MASK + 6)) - 1) * 2) as u8
}

// In order to take a white square on the 1st row and turn it into the the corresponding black
// square on 8th row 7 must be added. This function returns either 0 or 7 based on whose move is
// being made. If it's a black move it will return 7, if a white move 0.
#[inline(always)]
pub fn white_square_add_from_move(mv: Move) -> u8 {
    ((((mv & moves::MOVE_COLOR_MASK) >> (moves::MOVE_COLOR_MASK + 6)) - 1) * 7) as u8
}

#[inline(always)]
pub fn new_castle_move_mask(mv: Move) -> castle_move_flags::Type {
    mask = 0b11111111;
    black_left_shift = black_castle_shift_from_move(mv);
    white_square_add = white_square_add_from_move(mv);
    short_castle = castle_move_flags::BLACK_SHORT_CASTLE << black_left_shift;
    long_castle = castle_move_flags::BLACK_LONG_CASTLE << black_left_shift;
    both_castle = castle_move_flags::BLACK_CASTLE_MASK << black_left_shift;
    is_c = is_castle(mv);
    from = move_from(mv);
    is_both_disable = is_c || from == (square::E1 + white_square_add);
    is_short_disable = (square::H1 + white_square_add);
    is_long_disable = (square::A1 + white_square_add);

    mask &= !(boolean_to_u8_mask(is_both_disable) & both_castle);
    mask &= !(boolean_to_u8_mask(is_short_disable) & short_castle);
    mask &= !(boolean_to_u8_mask(is_long_disable) & long_castle);
    mask
}

#[inline(always)]
pub fn new_en_passant_flag(mv: Move) -> en_passant_flags::Type {
    0
}
