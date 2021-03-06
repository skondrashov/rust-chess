pub const UINT32_BOARD_SIZE: u8 = 10;
pub const FLAGS_INDEX: u8 = 8;
pub const GARBAGE_INDEX: u8 = 9;
pub const SQUARE_NUM: u8 = 64;
pub const RS_PIECE_COLOR: u32 = 3;
pub const RS_SQUARE_FILE: u32 = 3;

pub const SIZE_SQUARE: u32 = 6;
pub const SIZE_FILE: u32 = 3;
pub const SIZE_RANK: u32 = 3;
pub const SIZE_PIECE_TYPE: u32 = 3;
pub const SIZE_PIECE: u32 = 4;
pub const SIZE_COLOR: u32 = 1;
pub const SIZE_CASTLE: u32 = 3;

pub const MASK_PIECE: u32 = 15;

// #[repr(u8)]
// #[inline(always)]

#[repr(u8)]
pub enum FILE {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7
}

#[repr(u8)]
enum RANK {
    R1 = 0,
    R2 = 1,
    R3 = 2,
    R4 = 3,
    R5 = 4,
    R6 = 5,
    R7 = 6,
    R8 = 7
}

#[repr(u8)]
pub enum SQUARE {
    A1 = 0, A2 = 1, A3 = 2, A4 = 3, A5 = 4, A6 = 5, A7 = 6, A8 = 7,
    B1 = 8, B2 = 9, B3 = 10, B4 = 11, B5 = 12, B6 = 13, B7 = 14, B8 = 15,
    C1 = 16, C2 = 17, C3 = 18, C4 = 19, C5 = 20, C6 = 21, C7 = 22, C8 = 23,
    D1 = 24, D2 = 25, D3 = 26, D4 = 27, D5 = 28, D6 = 29, D7 = 30, D8 = 31,
    E1 = 32, E2 = 33, E3 = 34, E4 = 35, E5 = 36, E6 = 37, E7 = 38, E8 = 39,
    F1 = 40, F2 = 41, F3 = 42, F4 = 43, F5 = 44, F6 = 45, F7 = 46, F8 = 47,
    G1 = 48, G2 = 49, G3 = 50, G4 = 51, G5 = 52, G6 = 53, G7 = 54, G8 = 55,
    H1 = 56, H2 = 57, H3 = 58, H4 = 59, H5 = 60, H6 = 61, H7 = 62, H8 = 63
}

#[repr(u8)]
pub enum COLOR {
    NONE = 0b00000000,
    WHITE = 0b10000000,
    BLACK = 0b01000000,
    COLOR_MASK = 0b11000000
}

#[repr(u8)]
pub enum PIECETYPE {
    NONE = 0b00000000,
    PAWN = 0b00100000,
    KNIGHT = 0b00010000,
    BISHOP = 0b00001000,
    ROOK = 0b00000100,
    QUEEN = 0b00000010,
    KING = 0b00000001,
    PIECETYPE_MASK = 0b00111111
}

#[repr(u8)]
pub enum PIECE {
    NONE = 0b00000000,
    WHITE_PAWN = COLOR::WHITE | PIECETYPE::PAWN,
    WHITE_KNIGHT = COLOR::WHITE | PIECETYPE::KNIGHT,
    WHITE_BISHOP = COLOR::WHITE | PIECETYPE::BISHOP,
    WHITE_ROOK = COLOR::WHITE | PIECETYPE::ROOK,
    WHITE_QUEEN = COLOR::WHITE | PIECETYPE::QUEEN,
    WHITE_KING = COLOR::WHITE | PIECETYPE::KING,
    BLACK_PAWN = COLOR::BLACK | PIECETYPE::PAWN,
    BLACK_KNIGHT = COLOR::BLACK | PIECETYPE::KNIGHT,
    BLACK_BISHOP = COLOR::BLACK | PIECETYPE::BISHOP,
    BLACK_ROOK = COLOR::BLACK | PIECETYPE::ROOK,
    BLACK_QUEEN = COLOR::BLACK | PIECETYPE::QUEEN,
    BLACK_KING = COLOR::BLACK | PIECETYPE::KING
}

#[repr(u16)]
pub enum FLAGS_CASTLE {
    NOCASTLE = 0x00,
    WHITELONGCASTLE = 0x0010,
    WHITESHORTCASTLE = 0x0020,
    BLACKLONGCASTLE = 0x0040,
    BLACKSHORTCASTLE = 0x0080,
    FLAGS_CASTLE_MASK = 0x00F0
}

#[repr(u16)]
pub enum FLAGS_EN_PASSANT {
    NO_EN_PASSANT = 0x0000,
    A = 0x1000,
    B = 0x2000,
    C = 0x4000,
    D = 0x8000,
    E = 0x0100,
    F = 0x0200,
    G = 0x0400,
    H = 0x0800,
    FLAGS_EN_PASSANT_MASK = 0xFF00
}

#[repr(u16)]
pub enum FLAGS_TO_MOVE {
    WHITE = 0x0000,
    BLACK = 0x0001,
    FLAGS_TO_MOVE_MASK = 0x000F
}

pub type move_t = u64;

#[repr(u64)]
pub enum MOVE_SPECIALS {
    NONE = 0x00000000,
    EN_PASSANT = 0x00000001,
    WHITELONGCASTLE = 0x000010,
    WHITESHORTCASTLE = 0x000020,
    BLACKLONGCASTLE = 0x000040,
    BLACKSHORTCASTLE = 0x000080,
}

#[repr(u64)]
pub enum MOVE_MASKS {
    CAPTURED_PIECE_MASK = 0xFF000000,
    FROM_MASK = 0x003F0000,
    TO_MASK = 0x00003F00,
    MOVES_EN_PASSANT_MASK = 0x00000001,
    MOVES_CASTLE_MASK = 0x000000F0,
    MOVES_SPECIALS_MASK = 0x000000F1
}
