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

pub mod file_rank {
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
}

pub type Square = u8;

pub mod piece_color {
    pub type Type = u8;

    pub const NONE: Type = 0b00000000;
    pub const WHITE: Type = 0b10000000;
    pub const BLACK: Type = 0b01000000;
    pub const PIECE_COLOR_MASK: Type = 0b11000000;
}

pub mod piece_type {
    pub type Type = u8;

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

    pub type Type = u8;

    pub const NONE: Type = 0b00000000;
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

pub type Flags = u16;

pub mod castle_flags {
    pub type Type = u16;

    pub const NO_CASTLE: Type = 0x0000;
    pub const WHITE_LONG_CASTLE: Type = 0x0010;
    pub const WHITE_SHORT_CASTLE: Type = 0x0020;
    pub const BLACK_LONG_CASTLE: Type = 0x0040;
    pub const BLACK_SHORT_CASTLE: Type = 0x0080;
    pub const FLAGS_CASTLE_MASK: Type = 0x00F0;
}

pub mod en_passant_flags {
    pub type Type = u16;

    pub const NO_EN_PASSANT: Type = 0x0000;
    pub const A: Type = 0x1000;
    pub const B: Type = 0x2000;
    pub const C: Type = 0x4000;
    pub const D: Type = 0x8000;
    pub const E: Type = 0x0100;
    pub const F: Type = 0x0200;
    pub const G: Type = 0x0400;
    pub const H: Type = 0x0800;
    pub const FLAGS_EN_PASSANT_MASK: Type = 0xFF00;
}

pub mod to_move_flags {
    pub type Type = u16;

    pub const WHITE: Type = 0x0000;
    pub const BLACK: Type = 0x0001;
    pub const FLAGS_TO_MOVE_MASK: Type = 0x000F;
}

pub type Move = u64;

pub mod move_specials {
    pub type Type = u64;

    pub const NONE: Type = 0x00000000;
    pub const EN_PASSANT: Type = 0x00000001;
    pub const WHITE_LONG_CASTLE: Type = 0x000010;
    pub const WHITE_SHORT_CASTLE: Type = 0x000020;
    pub const BLACK_LONG_CASTLE: Type = 0x000040;
    pub const BLACK_SHORT_CASTLE: Type = 0x000080;
}

pub mod move_masks {
    pub type Type = u64;

    pub const CAPTURED_PIECE_MASK: Type = 0xFF000000;
    pub const FROM_MASK: Type = 0x003F0000;
    pub const TO_MASK: Type = 0x00003F00;
    pub const MOVES_EN_PASSANT_MASK: Type = 0x00000001;
    pub const MOVES_CASTLE_MASK: Type = 0x000000F0;
    pub const MOVES_SPECIALS_MASK: Type = 0x000000F1;
}
