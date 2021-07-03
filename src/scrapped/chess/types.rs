pub mod file {
    pub type Type = u8;

    pub const A: Type = 0;
    pub const B: Type = 1;
    pub const C: Type = 2;
    pub const D: Type = 3;
    pub const E: Type = 4;
    pub const F: Type = 5;
    pub const G: Type = 6;
    pub const H: Type = 7;
}

pub mod rank {
    pub type Type = u8;

    pub const R1: Type = 0;
    pub const R2: Type = 1;
    pub const R3: Type = 2;
    pub const R4: Type = 3;
    pub const R5: Type = 4;
    pub const R6: Type = 5;
    pub const R7: Type = 6;
    pub const R8: Type = 7;
}

pub mod square {
    pub type Type = u8;

    pub const A1: Type = 0;
    pub const A2: Type = 1;
    pub const A3: Type = 2;
    pub const A4: Type = 3;
    pub const A5: Type = 4;
    pub const A6: Type = 5;
    pub const A7: Type = 6;
    pub const A8: Type = 7;
    pub const B1: Type = 8;
    pub const B2: Type = 9;
    pub const B3: Type = 10;
    pub const B4: Type = 11;
    pub const B5: Type = 12;
    pub const B6: Type = 13;
    pub const B7: Type = 14;
    pub const B8: Type = 15;
    pub const C1: Type = 16;
    pub const C2: Type = 17;
    pub const C3: Type = 18;
    pub const C4: Type = 19;
    pub const C5: Type = 20;
    pub const C6: Type = 21;
    pub const C7: Type = 22;
    pub const C8: Type = 23;
    pub const D1: Type = 24;
    pub const D2: Type = 25;
    pub const D3: Type = 26;
    pub const D4: Type = 27;
    pub const D5: Type = 28;
    pub const D6: Type = 29;
    pub const D7: Type = 30;
    pub const D8: Type = 31;
    pub const E1: Type = 32;
    pub const E2: Type = 33;
    pub const E3: Type = 34;
    pub const E4: Type = 35;
    pub const E5: Type = 36;
    pub const E6: Type = 37;
    pub const E7: Type = 38;
    pub const E8: Type = 39;
    pub const F1: Type = 40;
    pub const F2: Type = 41;
    pub const F3: Type = 42;
    pub const F4: Type = 43;
    pub const F5: Type = 44;
    pub const F6: Type = 45;
    pub const F7: Type = 46;
    pub const F8: Type = 47;
    pub const G1: Type = 48;
    pub const G2: Type = 49;
    pub const G3: Type = 50;
    pub const G4: Type = 51;
    pub const G5: Type = 52;
    pub const G6: Type = 53;
    pub const G7: Type = 54;
    pub const G8: Type = 55;
    pub const H1: Type = 56;
    pub const H2: Type = 57;
    pub const H3: Type = 58;
    pub const H4: Type = 59;
    pub const H5: Type = 60;
    pub const H6: Type = 61;
    pub const H7: Type = 62;
    pub const H8: Type = 63;
    pub const NOOP_SQUARE: Type = 64;
}

pub type Piece = u8;

pub mod piece_color {
    pub type Type = super::Piece;

    pub const NONE: Type = 0b00000000;
    pub const WHITE: Type = 0b10000000;
    pub const BLACK: Type = 0b01000000;
    pub const PIECE_COLOR_MASK: Type = 0b11000000;
}

pub mod piece_type {
    pub type Type = super::Piece;

    pub const NONE: Type = 0b00000000;
    pub const PAWN: Type = 0b00100000;
    pub const KNIGHT: Type = 0b00010000;
    pub const BISHOP: Type = 0b00001000;
    pub const ROOK: Type = 0b00000100;
    pub const QUEEN: Type = 0b00000010;
    pub const KING: Type = 0b00000001;
    pub const PIECE_TYPE_MASK: Type = 0b00111111;
}

pub mod piece {
    use super::piece_color;
    use super::piece_type;

    pub type Type = super::Piece;

    pub const NONE: Type = piece_color::NONE | piece_type::NONE;
    pub const WHITE_PAWN: Type = piece_color::WHITE | piece_type::PAWN;
    pub const WHITE_KNIGHT: Type = piece_color::WHITE | piece_type::KNIGHT;
    pub const WHITE_BISHOP: Type = piece_color::WHITE | piece_type::BISHOP;
    pub const WHITE_ROOK: Type = piece_color::WHITE | piece_type::ROOK;
    pub const WHITE_QUEEN: Type = piece_color::WHITE | piece_type::QUEEN;
    pub const WHITE_KING: Type = piece_color::WHITE | piece_type::KING;
    pub const BLACK_PAWN: Type = piece_color::BLACK | piece_type::PAWN;
    pub const BLACK_KNIGHT: Type = piece_color::BLACK | piece_type::KNIGHT;
    pub const BLACK_BISHOP: Type = piece_color::BLACK | piece_type::BISHOP;
    pub const BLACK_ROOK: Type = piece_color::BLACK | piece_type::ROOK;
    pub const BLACK_QUEEN: Type = piece_color::BLACK | piece_type::QUEEN;
    pub const BLACK_KING: Type = piece_color::BLACK | piece_type::KING;
}

pub type CastleMoveFlags = u8;

pub mod castle_move_flags {
    pub type Type = super::CastleMoveFlags;

    pub const NO_CASTLE: Type = 0b00000000;
    pub const WHITE_LONG_CASTLE: Type = 0b10000000;
    pub const WHITE_SHORT_CASTLE: Type = 0b01000000;
    pub const BLACK_LONG_CASTLE: Type = 0b00100000;
    pub const BLACK_SHORT_CASTLE: Type = 0b00010000;

    pub const WHITE_CASTLE_MASK: Type = 0b11000000;
    pub const BLACK_CASTLE_MASK: Type = 0b00110000;
    pub const FLAGS_CASTLE_MASK: Type = 0b11110000;
    pub const MOVE_FLAG_MASK: Type = 0b00001100;
    pub const WHITE_MOVE_FLAGS_MASK: Type = 0b00001000;
    pub const BLACK_MOVE_FLAGS_MASK: Type = 0b00000100;
    pub const SWAP_MOVE_FLAGS_MASK: Type = 0b00001100;
}

pub type EnPassantFlags = u8;

pub mod en_passant_flags {
    pub type Type = super::EnPassantFlags;

    pub const NO_EN_PASSANT: Type = 0b00000000;
    pub const A: Type = 0b10000000;
    pub const B: Type = 0b01000000;
    pub const C: Type = 0b00100000;
    pub const D: Type = 0b00010000;
    pub const E: Type = 0b00001000;
    pub const F: Type = 0b00000100;
    pub const G: Type = 0b00000010;
    pub const H: Type = 0b00000001;
}

/**
 * Move encoding:
 * |0xxxxxxx|00xxxxxx|xxxxxxxx|xx0xxxx0|
 * 1: |0xxxxxxx| FROM_SQUARE. Either a number between 0 and 63 corresponding to the square from which
 *  a piece is being moved, or the number 64, meaning that this is a CASTLE move. If the move was a
 *  CASTLE_MOVE then the bits 3,4,5,6 (out of bits 1-8, or the ones immediately following the bit
 *  which would be set for the value '64') represent the type of CASTLE move in the following order:
 *  WHITE_SHORT_CASTLE, WHITE_LONG_CASTLE, BLACK_SHORT_CASTLE, BLACK_LONG_CASTLE
 * 2: |00xxxxxx| TO_SQUARE. Number between 0 and 64 corresponding to the square to which a piece is
 *  being moved.
 * 3: |xxxxxxx0| CAPTURED_PIECE. The piece that is being captured. Necessary in order to undo the
 *  move. The last bit which would normally represent the KING which can not be captured represents
 *  whether the capture was an EN_PASSANT. If the first 2 bits of this byte are 0, then the move
 *  was not a capture.
 * 4: |xx0xxxx0| PROMOTED_PIECE. The piece that a pawn is being promoted to. If the move is not a
 *  a promotion, the first 2 bits will still correspond to the color of the piece being moved, thus
 *  storing the color of the player whose move it is.
**/
pub type Move = u32;

pub mod moves {
    pub type Type = super::Move;
    pub const WHITE_SHORT_CASTLE: Type = 0b01100000_00000000_00000000_10000000;
    pub const WHITE_LONG_CASTLE: Type = 0b01010000_00000000_00000000_10000000;
    pub const BLACK_SHORT_CASTLE: Type = 0b01001000_00000000_00000000_01000000;
    pub const BLACK_LONG_CASTLE: Type = 0b01000100_00000000_00000000_01000000;

    pub const FROM_MASK: Type = 0b00111111_00000000_00000000_00000000;
    pub const FROM_MASK_SHIFT: u8 = 29;

    pub const IS_CASTLE_MASK: Type = 0b01000000_00000000_00000000_00000000;
    pub const IS_CASTLE_MASK_SHIFT: u8 = 30;

    pub const CASTLE_MASK: Type = 0b00111100_00000000_00000000_00000000;
    pub const CASTLE_MASK_SHIFT: u8 = 29;

    pub const WHITE_CASTLE_MASK: Type = 0b00110000_00000000_00000000_00000000;
    pub const BLACK_CASTLE_MASK: Type = 0b00001100_00000000_00000000_00000000;

    pub const TO_MASK: Type = 0b00000000_00111111_00000000_00000000;
    pub const TO_MASK_SHIFT: u8 = 16;

    pub const CAPTURED_PIECE_MASK: Type = 0b00000000_00000000_11111110_00000000;
    pub const CAPTURED_PIECE_MASK_SHIFT: u8 = 8;

    pub const EN_PASSANT_MASK: Type = 0b00000000_00000000_00000001_00000000;
    pub const EN_PASSANT_MASK_SHIFT: u8 = 8;

    pub const PROMOTION_PIECE_MASK: Type = 0b00000000_00000000_00000000_11111111;
    pub const PROMOTION_PIECE_MASK_SHIFT: u8 = 0;

    pub const MOVE_COLOR_MASK: Type = 0b00000000_00000000_00000000_11000000;
    pub const MOVE_COLOR_MASK_SHIFT: u8 = 0;

    pub const IS_PROMOTION_MASK: Type = 0b00000000_00000000_00000000_00111111;
    pub const IS_PROMOTION_MASK_SHIFT: u8 = 0;

    pub const WHITE_MOVE_COLOR_MASK: Type = 0b00000000_00000000_00000000_10000000;
    pub const BLACK_MOVE_COLOR_MASK: Type = 0b00000000_00000000_00000000_01000000;
}
