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

pub const EN_PASSANT_FLAGS_SIZE: usize = 1;
pub const CASTLE_MOVE_FLAGS_SIZE: usize = 1;

pub const ALL_SQUARES_START: usize = 0;

pub const FILE_A_START: usize = ALL_SQUARES_START + ALL_SQUARES_NUM;
pub const FILE_B_START: usize = FILE_A_START + FILE_SIZE;
pub const FILE_C_START: usize = FILE_B_START + FILE_SIZE;
pub const FILE_D_START: usize = FILE_C_START + FILE_SIZE;
pub const FILE_E_START: usize = FILE_D_START + FILE_SIZE;
pub const FILE_F_START: usize = FILE_E_START + FILE_SIZE;
pub const FILE_G_START: usize = FILE_F_START + FILE_SIZE;
pub const FILE_H_START: usize = FILE_G_START + FILE_SIZE;

pub const RANK_1_START: usize = FILE_H_START + FILE_SIZE;
pub const RANK_2_START: usize = RANK_1_START + RANK_SIZE;
pub const RANK_3_START: usize = RANK_2_START + RANK_SIZE;
pub const RANK_4_START: usize = RANK_3_START + RANK_SIZE;
pub const RANK_5_START: usize = RANK_4_START + RANK_SIZE;
pub const RANK_6_START: usize = RANK_5_START + RANK_SIZE;
pub const RANK_7_START: usize = RANK_6_START + RANK_SIZE;
pub const RANK_8_START: usize = RANK_7_START + RANK_SIZE;

pub const DIAG_A8_START: usize = RANK_8_START + RANK_SIZE;
pub const DIAG_A7B8_START: usize = DIAG_A8_START + DIAG1_SIZE;
pub const DIAG_A6C8_START: usize = DIAG_A7B8_START + DIAG2_SIZE;
pub const DIAG_A5D8_START: usize = DIAG_A6C8_START + DIAG3_SIZE;
pub const DIAG_A4E8_START: usize = DIAG_A5D8_START + DIAG4_SIZE;
pub const DIAG_A3F8_START: usize = DIAG_A4E8_START + DIAG5_SIZE;
pub const DIAG_A2G8_START: usize = DIAG_A3F8_START + DIAG6_SIZE;
pub const DIAG_A1H8_START: usize = DIAG_A2G8_START + DIAG7_SIZE;
pub const DIAG_B1H7_START: usize = DIAG_A1H8_START + DIAG8_SIZE;
pub const DIAG_C1H6_START: usize = DIAG_B1H7_START + DIAG7_SIZE;
pub const DIAG_D1H5_START: usize = DIAG_C1H6_START + DIAG6_SIZE;
pub const DIAG_E1H4_START: usize = DIAG_D1H5_START + DIAG5_SIZE;
pub const DIAG_F1H3_START: usize = DIAG_E1H4_START + DIAG4_SIZE;
pub const DIAG_G1H2_START: usize = DIAG_F1H3_START + DIAG3_SIZE;
pub const DIAG_H1_START: usize = DIAG_G1H2_START + DIAG2_SIZE;

pub const DIAG_A1_START: usize = DIAG_H1_START + DIAG1_SIZE;
pub const DIAG_A2B1_START: usize = DIAG_A1_START + DIAG1_SIZE;
pub const DIAG_A3C1_START: usize = DIAG_A2B1_START + DIAG2_SIZE;
pub const DIAG_A4D1_START: usize = DIAG_A3C1_START + DIAG3_SIZE;
pub const DIAG_A5E1_START: usize = DIAG_A4D1_START + DIAG4_SIZE;
pub const DIAG_A6F1_START: usize = DIAG_A5E1_START + DIAG5_SIZE;
pub const DIAG_A7G1_START: usize = DIAG_A6F1_START + DIAG6_SIZE;
pub const DIAG_A8H1_START: usize = DIAG_A7G1_START + DIAG7_SIZE;
pub const DIAG_B8H2_START: usize = DIAG_A8H1_START + DIAG8_SIZE;
pub const DIAG_C8H3_START: usize = DIAG_B8H2_START + DIAG7_SIZE;
pub const DIAG_D8H4_START: usize = DIAG_C8H3_START + DIAG6_SIZE;
pub const DIAG_E8H5_START: usize = DIAG_D8H4_START + DIAG5_SIZE;
pub const DIAG_F8H6_START: usize = DIAG_E8H5_START + DIAG4_SIZE;
pub const DIAG_G8H7_START: usize = DIAG_F8H6_START + DIAG3_SIZE;
pub const DIAG_H8_START: usize = DIAG_G8H7_START + DIAG2_SIZE;

pub const KNIGHT_A1_START: usize = DIAG_H8_START + DIAG1_SIZE;
pub const KNIGHT_A2_START: usize = KNIGHT_A1_START + KNIGHT2_SIZE;
pub const KNIGHT_A3_START: usize = KNIGHT_A2_START + KNIGHT3_SIZE;
pub const KNIGHT_A4_START: usize = KNIGHT_A3_START + KNIGHT4_SIZE;
pub const KNIGHT_A5_START: usize = KNIGHT_A4_START + KNIGHT4_SIZE;
pub const KNIGHT_A6_START: usize = KNIGHT_A5_START + KNIGHT4_SIZE;
pub const KNIGHT_A7_START: usize = KNIGHT_A6_START + KNIGHT4_SIZE;
pub const KNIGHT_A8_START: usize = KNIGHT_A7_START + KNIGHT3_SIZE;
pub const KNIGHT_B1_START: usize = KNIGHT_A8_START + KNIGHT2_SIZE;
pub const KNIGHT_B2_START: usize = KNIGHT_B1_START + KNIGHT3_SIZE;
pub const KNIGHT_B3_START: usize = KNIGHT_B2_START + KNIGHT4_SIZE;
pub const KNIGHT_B4_START: usize = KNIGHT_B3_START + KNIGHT6_SIZE;
pub const KNIGHT_B5_START: usize = KNIGHT_B4_START + KNIGHT6_SIZE;
pub const KNIGHT_B6_START: usize = KNIGHT_B5_START + KNIGHT6_SIZE;
pub const KNIGHT_B7_START: usize = KNIGHT_B6_START + KNIGHT6_SIZE;
pub const KNIGHT_B8_START: usize = KNIGHT_B7_START + KNIGHT4_SIZE;
pub const KNIGHT_C1_START: usize = KNIGHT_B8_START + KNIGHT3_SIZE;
pub const KNIGHT_C2_START: usize = KNIGHT_C1_START + KNIGHT4_SIZE;
pub const KNIGHT_C3_START: usize = KNIGHT_C2_START + KNIGHT6_SIZE;
pub const KNIGHT_C4_START: usize = KNIGHT_C3_START + KNIGHT8_SIZE;
pub const KNIGHT_C5_START: usize = KNIGHT_C4_START + KNIGHT8_SIZE;
pub const KNIGHT_C6_START: usize = KNIGHT_C5_START + KNIGHT8_SIZE;
pub const KNIGHT_C7_START: usize = KNIGHT_C6_START + KNIGHT8_SIZE;
pub const KNIGHT_C8_START: usize = KNIGHT_C7_START + KNIGHT6_SIZE;
pub const KNIGHT_D1_START: usize = KNIGHT_C8_START + KNIGHT4_SIZE;
pub const KNIGHT_D2_START: usize = KNIGHT_D1_START + KNIGHT4_SIZE;
pub const KNIGHT_D3_START: usize = KNIGHT_D2_START + KNIGHT6_SIZE;
pub const KNIGHT_D4_START: usize = KNIGHT_D3_START + KNIGHT8_SIZE;
pub const KNIGHT_D5_START: usize = KNIGHT_D4_START + KNIGHT8_SIZE;
pub const KNIGHT_D6_START: usize = KNIGHT_D5_START + KNIGHT8_SIZE;
pub const KNIGHT_D7_START: usize = KNIGHT_D6_START + KNIGHT8_SIZE;
pub const KNIGHT_D8_START: usize = KNIGHT_D7_START + KNIGHT6_SIZE;
pub const KNIGHT_E1_START: usize = KNIGHT_D8_START + KNIGHT4_SIZE;
pub const KNIGHT_E2_START: usize = KNIGHT_E1_START + KNIGHT4_SIZE;
pub const KNIGHT_E3_START: usize = KNIGHT_E2_START + KNIGHT6_SIZE;
pub const KNIGHT_E4_START: usize = KNIGHT_E3_START + KNIGHT8_SIZE;
pub const KNIGHT_E5_START: usize = KNIGHT_E4_START + KNIGHT8_SIZE;
pub const KNIGHT_E6_START: usize = KNIGHT_E5_START + KNIGHT8_SIZE;
pub const KNIGHT_E7_START: usize = KNIGHT_E6_START + KNIGHT8_SIZE;
pub const KNIGHT_E8_START: usize = KNIGHT_E7_START + KNIGHT6_SIZE;
pub const KNIGHT_F1_START: usize = KNIGHT_E8_START + KNIGHT4_SIZE;
pub const KNIGHT_F2_START: usize = KNIGHT_F1_START + KNIGHT4_SIZE;
pub const KNIGHT_F3_START: usize = KNIGHT_F2_START + KNIGHT6_SIZE;
pub const KNIGHT_F4_START: usize = KNIGHT_F3_START + KNIGHT8_SIZE;
pub const KNIGHT_F5_START: usize = KNIGHT_F4_START + KNIGHT8_SIZE;
pub const KNIGHT_F6_START: usize = KNIGHT_F5_START + KNIGHT8_SIZE;
pub const KNIGHT_F7_START: usize = KNIGHT_F6_START + KNIGHT8_SIZE;
pub const KNIGHT_F8_START: usize = KNIGHT_F7_START + KNIGHT6_SIZE;
pub const KNIGHT_G1_START: usize = KNIGHT_F8_START + KNIGHT4_SIZE;
pub const KNIGHT_G2_START: usize = KNIGHT_G1_START + KNIGHT3_SIZE;
pub const KNIGHT_G3_START: usize = KNIGHT_G2_START + KNIGHT4_SIZE;
pub const KNIGHT_G4_START: usize = KNIGHT_G3_START + KNIGHT6_SIZE;
pub const KNIGHT_G5_START: usize = KNIGHT_G4_START + KNIGHT6_SIZE;
pub const KNIGHT_G6_START: usize = KNIGHT_G5_START + KNIGHT6_SIZE;
pub const KNIGHT_G7_START: usize = KNIGHT_G6_START + KNIGHT6_SIZE;
pub const KNIGHT_G8_START: usize = KNIGHT_G7_START + KNIGHT4_SIZE;
pub const KNIGHT_H1_START: usize = KNIGHT_G8_START + KNIGHT3_SIZE;
pub const KNIGHT_H2_START: usize = KNIGHT_H1_START + KNIGHT2_SIZE;
pub const KNIGHT_H3_START: usize = KNIGHT_H2_START + KNIGHT3_SIZE;
pub const KNIGHT_H4_START: usize = KNIGHT_H3_START + KNIGHT4_SIZE;
pub const KNIGHT_H5_START: usize = KNIGHT_H4_START + KNIGHT4_SIZE;
pub const KNIGHT_H6_START: usize = KNIGHT_H5_START + KNIGHT4_SIZE;
pub const KNIGHT_H7_START: usize = KNIGHT_H6_START + KNIGHT4_SIZE;
pub const KNIGHT_H8_START: usize = KNIGHT_H7_START + KNIGHT3_SIZE;

pub const KING_A1_START: usize = KNIGHT_H8_START + KNIGHT2_SIZE;
pub const KING_A2_START: usize = KING_A1_START + KING3_SIZE;
pub const KING_A3_START: usize = KING_A2_START + KING5_SIZE;
pub const KING_A4_START: usize = KING_A3_START + KING5_SIZE;
pub const KING_A5_START: usize = KING_A4_START + KING5_SIZE;
pub const KING_A6_START: usize = KING_A5_START + KING5_SIZE;
pub const KING_A7_START: usize = KING_A6_START + KING5_SIZE;
pub const KING_A8_START: usize = KING_A7_START + KING5_SIZE;
pub const KING_B1_START: usize = KING_A8_START + KING3_SIZE;
pub const KING_B2_START: usize = KING_B1_START + KING5_SIZE;
pub const KING_B3_START: usize = KING_B2_START + KING8_SIZE;
pub const KING_B4_START: usize = KING_B3_START + KING8_SIZE;
pub const KING_B5_START: usize = KING_B4_START + KING8_SIZE;
pub const KING_B6_START: usize = KING_B5_START + KING8_SIZE;
pub const KING_B7_START: usize = KING_B6_START + KING8_SIZE;
pub const KING_B8_START: usize = KING_B7_START + KING8_SIZE;
pub const KING_C1_START: usize = KING_B8_START + KING5_SIZE;
pub const KING_C2_START: usize = KING_C1_START + KING5_SIZE;
pub const KING_C3_START: usize = KING_C2_START + KING8_SIZE;
pub const KING_C4_START: usize = KING_C3_START + KING8_SIZE;
pub const KING_C5_START: usize = KING_C4_START + KING8_SIZE;
pub const KING_C6_START: usize = KING_C5_START + KING8_SIZE;
pub const KING_C7_START: usize = KING_C6_START + KING8_SIZE;
pub const KING_C8_START: usize = KING_C7_START + KING8_SIZE;
pub const KING_D1_START: usize = KING_C8_START + KING5_SIZE;
pub const KING_D2_START: usize = KING_D1_START + KING5_SIZE;
pub const KING_D3_START: usize = KING_D2_START + KING8_SIZE;
pub const KING_D4_START: usize = KING_D3_START + KING8_SIZE;
pub const KING_D5_START: usize = KING_D4_START + KING8_SIZE;
pub const KING_D6_START: usize = KING_D5_START + KING8_SIZE;
pub const KING_D7_START: usize = KING_D6_START + KING8_SIZE;
pub const KING_D8_START: usize = KING_D7_START + KING8_SIZE;
pub const KING_E1_START: usize = KING_D8_START + KING5_SIZE;
pub const KING_E2_START: usize = KING_E1_START + KING5_SIZE;
pub const KING_E3_START: usize = KING_E2_START + KING8_SIZE;
pub const KING_E4_START: usize = KING_E3_START + KING8_SIZE;
pub const KING_E5_START: usize = KING_E4_START + KING8_SIZE;
pub const KING_E6_START: usize = KING_E5_START + KING8_SIZE;
pub const KING_E7_START: usize = KING_E6_START + KING8_SIZE;
pub const KING_E8_START: usize = KING_E7_START + KING8_SIZE;
pub const KING_F1_START: usize = KING_E8_START + KING5_SIZE;
pub const KING_F2_START: usize = KING_F1_START + KING5_SIZE;
pub const KING_F3_START: usize = KING_F2_START + KING8_SIZE;
pub const KING_F4_START: usize = KING_F3_START + KING8_SIZE;
pub const KING_F5_START: usize = KING_F4_START + KING8_SIZE;
pub const KING_F6_START: usize = KING_F5_START + KING8_SIZE;
pub const KING_F7_START: usize = KING_F6_START + KING8_SIZE;
pub const KING_F8_START: usize = KING_F7_START + KING8_SIZE;
pub const KING_G1_START: usize = KING_F8_START + KING5_SIZE;
pub const KING_G2_START: usize = KING_G1_START + KING5_SIZE;
pub const KING_G3_START: usize = KING_G2_START + KING8_SIZE;
pub const KING_G4_START: usize = KING_G3_START + KING8_SIZE;
pub const KING_G5_START: usize = KING_G4_START + KING8_SIZE;
pub const KING_G6_START: usize = KING_G5_START + KING8_SIZE;
pub const KING_G7_START: usize = KING_G6_START + KING8_SIZE;
pub const KING_G8_START: usize = KING_G7_START + KING8_SIZE;
pub const KING_H1_START: usize = KING_G8_START + KING5_SIZE;
pub const KING_H2_START: usize = KING_H1_START + KING3_SIZE;
pub const KING_H3_START: usize = KING_H2_START + KING5_SIZE;
pub const KING_H4_START: usize = KING_H3_START + KING5_SIZE;
pub const KING_H5_START: usize = KING_H4_START + KING5_SIZE;
pub const KING_H6_START: usize = KING_H5_START + KING5_SIZE;
pub const KING_H7_START: usize = KING_H6_START + KING5_SIZE;
pub const KING_H8_START: usize = KING_H7_START + KING5_SIZE;

pub const EN_PASSANT_FLAGS_AT: usize = KING_H8_START + KING3_SIZE;
pub const CASTLE_MOVE_FLAGS_AT: usize = EN_PASSANT_FLAGS_AT + EN_PASSANT_FLAGS_SIZE;

pub const TOTAL_BOARD_SIZE: usize = CASTLE_MOVE_FLAGS_AT + CASTLE_MOVE_FLAGS_SIZE;

pub type BoardArray = [types::Square; TOTAL_BOARD_SIZE];
//
// pub type AllSquares = [types::Square; ALL_SQUARES_NUM];
// pub type File = [types::Square; FILE_SIZE];
// pub type Rank = [types::Square; RANK_SIZE];
//
// pub type Diag8 = [types::Square; DIAG8_SIZE];
// pub type Diag7 = [types::Square; DIAG7_SIZE];
// pub type Diag6 = [types::Square; DIAG6_SIZE];
// pub type Diag5 = [types::Square; DIAG5_SIZE];
// pub type Diag4 = [types::Square; DIAG4_SIZE];
// pub type Diag3 = [types::Square; DIAG3_SIZE];
// pub type Diag2 = [types::Square; DIAG2_SIZE];
// pub type Diag1 = [types::Square; DIAG1_SIZE];
//
// pub type Knight2 = [types::Square; KNIGHT2_SIZE];
// pub type Knight3 = [types::Square; KNIGHT3_SIZE];
// pub type Knight4 = [types::Square; KNIGHT4_SIZE];
// pub type Knight6 = [types::Square; KNIGHT6_SIZE];
// pub type Knight8 = [types::Square; KNIGHT8_SIZE];
//
// pub type King3 = [types::Square; KING3_SIZE];
// pub type King5 = [types::Square; KING5_SIZE];
// pub type King8 = [types::Square; KING8_SIZE];
//
// #[repr(packed(1))]
// struct Board {
//     all_squares: AllSquares,
//
//     file_a: File,
//     file_b: File,
//     file_c: File,
//     file_d: File,
//     file_e: File,
//     file_f: File,
//     file_g: File,
//     file_h: File,
//
//     rank_1: Rank,
//     rank_2: Rank,
//     rank_3: Rank,
//     rank_4: Rank,
//     rank_5: Rank,
//     rank_6: Rank,
//     rank_7: Rank,
//     rank_8: Rank,
//
//     diag_a8: Diag1,
//     diag_a7b8: Diag2,
//     diag_a6c8: Diag3,
//     diag_a5d8: Diag4,
//     diag_a4e8: Diag5,
//     diag_a3f8: Diag6,
//     diag_a2g8: Diag7,
//     diag_a1h8: Diag8,
//     diag_b1h7: Diag7,
//     diag_c1h6: Diag6,
//     diag_d1h5: Diag5,
//     diag_e1h4: Diag4,
//     diag_f1h3: Diag3,
//     diag_g1h2: Diag2,
//     diag_h1: Diag1,
//
//     diag_a1: Diag1,
//     diag_a2b1: Diag2,
//     diag_a3c1: Diag3,
//     diag_a4d1: Diag4,
//     diag_a5e1: Diag5,
//     diag_a6f1: Diag6,
//     diag_a7g1: Diag7,
//     diag_a8h1: Diag8,
//     diag_b8h2: Diag7,
//     diag_c8h3: Diag6,
//     diag_d8h4: Diag5,
//     diag_e8h5: Diag4,
//     diag_f8h6: Diag3,
//     diag_g8h7: Diag2,
//     diag_h8: Diag1,
//
//     knight_a1: Knight2,
//     knight_a2: Knight3,
//     knight_a3: Knight4,
//     knight_a4: Knight4,
//     knight_a5: Knight4,
//     knight_a6: Knight4,
//     knight_a7: Knight3,
//     knight_a8: Knight2,
//     knight_b1: Knight3,
//     knight_b2: Knight4,
//     knight_b3: Knight6,
//     knight_b4: Knight6,
//     knight_b5: Knight6,
//     knight_b6: Knight6,
//     knight_b7: Knight4,
//     knight_b8: Knight3,
//     knight_c1: Knight4,
//     knight_c2: Knight6,
//     knight_c3: Knight8,
//     knight_c4: Knight8,
//     knight_c5: Knight8,
//     knight_c6: Knight8,
//     knight_c7: Knight6,
//     knight_c8: Knight4,
//     knight_d1: Knight4,
//     knight_d2: Knight6,
//     knight_d3: Knight8,
//     knight_d4: Knight8,
//     knight_d5: Knight8,
//     knight_d6: Knight8,
//     knight_d7: Knight6,
//     knight_d8: Knight4,
//     knight_e1: Knight4,
//     knight_e2: Knight6,
//     knight_e3: Knight8,
//     knight_e4: Knight8,
//     knight_e5: Knight8,
//     knight_e6: Knight8,
//     knight_e7: Knight6,
//     knight_e8: Knight4,
//     knight_f1: Knight4,
//     knight_f2: Knight6,
//     knight_f3: Knight8,
//     knight_f4: Knight8,
//     knight_f5: Knight8,
//     knight_f6: Knight8,
//     knight_f7: Knight6,
//     knight_f8: Knight4,
//     knight_g1: Knight3,
//     knight_g2: Knight4,
//     knight_g3: Knight6,
//     knight_g4: Knight6,
//     knight_g5: Knight6,
//     knight_g6: Knight6,
//     knight_g7: Knight4,
//     knight_g8: Knight3,
//     knight_h1: Knight2,
//     knight_h2: Knight3,
//     knight_h3: Knight4,
//     knight_h4: Knight4,
//     knight_h5: Knight4,
//     knight_h6: Knight4,
//     knight_h7: Knight3,
//     knight_h8: Knight2,
//
//     king_a1: King3,
//     king_a2: King5,
//     king_a3: King5,
//     king_a4: King5,
//     king_a5: King5,
//     king_a6: King5,
//     king_a7: King5,
//     king_a8: King3,
//     king_b1: King5,
//     king_b2: King8,
//     king_b3: King8,
//     king_b4: King8,
//     king_b5: King8,
//     king_b6: King8,
//     king_b7: King8,
//     king_b8: King5,
//     king_c1: King5,
//     king_c2: King8,
//     king_c3: King8,
//     king_c4: King8,
//     king_c5: King8,
//     king_c6: King8,
//     king_c7: King8,
//     king_c8: King5,
//     king_d1: King5,
//     king_d2: King8,
//     king_d3: King8,
//     king_d4: King8,
//     king_d5: King8,
//     king_d6: King8,
//     king_d7: King8,
//     king_d8: King5,
//     king_e1: King5,
//     king_e2: King8,
//     king_e3: King8,
//     king_e4: King8,
//     king_e5: King8,
//     king_e6: King8,
//     king_e7: King8,
//     king_e8: King5,
//     king_f1: King5,
//     king_f2: King8,
//     king_f3: King8,
//     king_f4: King8,
//     king_f5: King8,
//     king_f6: King8,
//     king_f7: King8,
//     king_f8: King5,
//     king_g1: King5,
//     king_g2: King8,
//     king_g3: King8,
//     king_g4: King8,
//     king_g5: King8,
//     king_g6: King8,
//     king_g7: King8,
//     king_g8: King5,
//     king_h1: King3,
//     king_h2: King5,
//     king_h3: King5,
//     king_h4: King5,
//     king_h5: King5,
//     king_h6: King5,
//     king_h7: King5,
//     king_h8: King3,
//
//     flags: types::Flags,
// }
