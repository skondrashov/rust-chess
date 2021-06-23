use super::types;

pub const ALL_SQUARES_NUM: usize = 64;

pub const FILE_SIZE: usize = 8;
pub const RANK_SIZE: usize = 8;

pub const DIAG8_SIZE: usize = 8;
pub const DIAG7_SIZE: usize = 7;
pub const DIAG6_SIZE: usize = 6;
pub const DIAG5_SIZE: usize = 5;
pub const DIAG4_SIZE: usize = 4;
pub const DIAG3_SIZE: usize = 3;
pub const DIAG2_SIZE: usize = 2;
pub const DIAG1_SIZE: usize = 1;

pub const KNIGHT2_SIZE: usize = 2;
pub const KNIGHT3_SIZE: usize = 3;
pub const KNIGHT4_SIZE: usize = 4;
pub const KNIGHT6_SIZE: usize = 6;
pub const KNIGHT8_SIZE: usize = 8;

pub const KING3_SIZE: usize = 3;
pub const KING5_SIZE: usize = 5;
pub const KING8_SIZE: usize = 8;

pub type AllSquares = [types::Square; ALL_SQUARES_NUM];
pub type File = [types::Square; FILE_SIZE];
pub type Rank = [types::Square; RANK_SIZE];

pub type Diag8 = [types::Square; DIAG8_SIZE];
pub type Diag7 = [types::Square; DIAG7_SIZE];
pub type Diag6 = [types::Square; DIAG6_SIZE];
pub type Diag5 = [types::Square; DIAG5_SIZE];
pub type Diag4 = [types::Square; DIAG4_SIZE];
pub type Diag3 = [types::Square; DIAG3_SIZE];
pub type Diag2 = [types::Square; DIAG2_SIZE];
pub type Diag1 = [types::Square; DIAG1_SIZE];

pub type Knight2 = [types::Square; KNIGHT2_SIZE];
pub type Knight3 = [types::Square; KNIGHT3_SIZE];
pub type Knight4 = [types::Square; KNIGHT4_SIZE];
pub type Knight6 = [types::Square; KNIGHT6_SIZE];
pub type Knight8 = [types::Square; KNIGHT8_SIZE];

pub type King3 = [types::Square; KING3_SIZE];
pub type King5 = [types::Square; KING5_SIZE];
pub type King8 = [types::Square; KING8_SIZE];

struct Board {
    all_squares: AllSquares,

    file_a: File,
    file_b: File,
    file_c: File,
    file_d: File,
    file_e: File,
    file_f: File,
    file_g: File,
    file_h: File,

    rank_1: Rank,
    rank_2: Rank,
    rank_3: Rank,
    rank_4: Rank,
    rank_5: Rank,
    rank_6: Rank,
    rank_7: Rank,
    rank_8: Rank,

    diag_a8: Diag1,
    diag_a7b8: Diag2,
    diag_a6c8: Diag3,
    diag_a5d8: Diag4,
    diag_a4e8: Diag5,
    diag_a3f8: Diag6,
    diag_a2g8: Diag7,
    diag_a1h8: Diag8,
    diag_b1h7: Diag7,
    diag_c1h6: Diag6,
    diag_d1h5: Diag5,
    diag_e1h4: Diag4,
    diag_f1h3: Diag3,
    diag_g1h2: Diag2,
    diag_h1: Diag1,

    diag_a1: Diag1,
    diag_a2b1: Diag2,
    diag_a3c1: Diag3,
    diag_a4d1: Diag4,
    diag_a5e1: Diag5,
    diag_a6f1: Diag6,
    diag_a7g1: Diag7,
    diag_a8h1: Diag8,
    diag_b8h2: Diag7,
    diag_c8h3: Diag6,
    diag_d8h4: Diag5,
    diag_e8h5: Diag4,
    diag_f8h6: Diag3,
    diag_g8h7: Diag2,
    diag_h8: Diag1,

    knight_a1: Knight2,
    knight_a2: Knight3,
    knight_a3: Knight4,
    knight_a4: Knight4,
    knight_a5: Knight4,
    knight_a6: Knight4,
    knight_a7: Knight3,
    knight_a8: Knight2,
    knight_b1: Knight3,
    knight_b2: Knight4,
    knight_b3: Knight6,
    knight_b4: Knight6,
    knight_b5: Knight6,
    knight_b6: Knight6,
    knight_b7: Knight4,
    knight_b8: Knight3,
    knight_c1: Knight4,
    knight_c2: Knight6,
    knight_c3: Knight8,
    knight_c4: Knight8,
    knight_c5: Knight8,
    knight_c6: Knight8,
    knight_c7: Knight6,
    knight_c8: Knight4,
    knight_d1: Knight4,
    knight_d2: Knight6,
    knight_d3: Knight8,
    knight_d4: Knight8,
    knight_d5: Knight8,
    knight_d6: Knight8,
    knight_d7: Knight6,
    knight_d8: Knight4,
    knight_e1: Knight4,
    knight_e2: Knight6,
    knight_e3: Knight8,
    knight_e4: Knight8,
    knight_e5: Knight8,
    knight_e6: Knight8,
    knight_e7: Knight6,
    knight_e8: Knight4,
    knight_f1: Knight4,
    knight_f2: Knight6,
    knight_f3: Knight8,
    knight_f4: Knight8,
    knight_f5: Knight8,
    knight_f6: Knight8,
    knight_f7: Knight6,
    knight_f8: Knight4,
    knight_g1: Knight3,
    knight_g2: Knight4,
    knight_g3: Knight6,
    knight_g4: Knight6,
    knight_g5: Knight6,
    knight_g6: Knight6,
    knight_g7: Knight4,
    knight_g8: Knight3,
    knight_h1: Knight2,
    knight_h2: Knight3,
    knight_h3: Knight4,
    knight_h4: Knight4,
    knight_h5: Knight4,
    knight_h6: Knight4,
    knight_h7: Knight3,
    knight_h8: Knight2,

    king_a1: King3,
    king_a2: King5,
    king_a3: King5,
    king_a4: King5,
    king_a5: King5,
    king_a6: King5,
    king_a7: King5,
    king_a8: King3,
    king_b1: King5,
    king_b2: King8,
    king_b3: King8,
    king_b4: King8,
    king_b5: King8,
    king_b6: King8,
    king_b7: King8,
    king_b8: King5,
    king_c1: King5,
    king_c2: King8,
    king_c3: King8,
    king_c4: King8,
    king_c5: King8,
    king_c6: King8,
    king_c7: King8,
    king_c8: King5,
    king_d1: King5,
    king_d2: King8,
    king_d3: King8,
    king_d4: King8,
    king_d5: King8,
    king_d6: King8,
    king_d7: King8,
    king_d8: King5,
    king_e1: King5,
    king_e2: King8,
    king_e3: King8,
    king_e4: King8,
    king_e5: King8,
    king_e6: King8,
    king_e7: King8,
    king_e8: King5,
    king_f1: King5,
    king_f2: King8,
    king_f3: King8,
    king_f4: King8,
    king_f5: King8,
    king_f6: King8,
    king_f7: King8,
    king_f8: King5,
    king_g1: King5,
    king_g2: King8,
    king_g3: King8,
    king_g4: King8,
    king_g5: King8,
    king_g6: King8,
    king_g7: King8,
    king_g8: King5,
    king_h1: King3,
    king_h2: King5,
    king_h3: King5,
    king_h4: King5,
    king_h5: King5,
    king_h6: King5,
    king_h7: King5,
    king_h8: King3,

    flags: types::Flags,
}
