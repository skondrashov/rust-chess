pub mod constants;

use {
    crate::network::constants::*,
    constants::{pieces::*, *},
    std::{
        collections::{HashMap, HashSet},
        convert::TryInto,
        iter::FromIterator,
        vec,
    },
};

#[repr(u8)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}
use File::*;

#[inline(always)]
pub fn square(file: File, rank: u8) -> usize {
    (file as u8 + 8 * (rank - 1)).into()
}

const FILES: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

pub struct Move {
    pub from: usize,
    pub to: (usize, Square),
    pub clear_square: Option<usize>,
    pub rook_square: Option<usize>,
}

impl ToString for Move {
    fn to_string(&self) -> String {
        format!(
            "{}{}-{}{}",
            FILES[self.from % 8],
            self.from / 8 + 1,
            FILES[self.to.0 % 8],
            self.to.0 / 8 + 1
        )
    }
}

pub type Square = [bool; SQUARE_FEATURES];
pub trait SquareProperties {
    fn is_white(&self) -> bool;
    fn is_black(&self) -> bool;
    fn is_pawn(&self) -> bool;
    fn is_knight(&self) -> bool;
    fn is_bishop(&self) -> bool;
    fn is_rook(&self) -> bool;
    fn is_queen(&self) -> bool;
    fn is_king(&self) -> bool;
}
impl SquareProperties for Square {
    fn is_white(&self) -> bool {
        self[FeatureIndex::WHITE as usize]
    }
    fn is_black(&self) -> bool {
        self[FeatureIndex::BLACK as usize]
    }
    fn is_pawn(&self) -> bool {
        self[FeatureIndex::PAWN as usize]
    }
    fn is_knight(&self) -> bool {
        self[FeatureIndex::KNIGHT as usize]
    }
    fn is_bishop(&self) -> bool {
        self[FeatureIndex::BISHOP as usize]
    }
    fn is_rook(&self) -> bool {
        self[FeatureIndex::ROOK as usize]
    }
    fn is_queen(&self) -> bool {
        self[FeatureIndex::QUEEN as usize]
    }
    fn is_king(&self) -> bool {
        self[FeatureIndex::KING as usize]
    }
}

macro_rules! same_color {
    ($piece1:expr, $piece2:expr) => {
        if $piece1.is_white() {
            $piece2.is_white()
        } else {
            $piece2.is_black()
        }
    };
}

#[derive(Clone)]
pub struct Chess {
    pub to_play: Color,
    pub pieces: HashMap<usize, Piece>,
    pub check: Option<Color>,
}

#[derive(PartialEq, Eq)]
pub enum Result {
    WHITE,
    BLACK,
    DRAW,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Color {
    WHITE,
    BLACK,
}
use Color::*;

#[derive(PartialEq, Eq)]
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}
use PieceType::*;

#[derive(Clone, Default)]
pub struct Piece {
    pub features: Square,
    pub destinations: HashSet<usize>,
    pub blocking: HashSet<usize>,
    pub can_castle_long: bool,
    pub can_castle_short: bool,
    pub can_en_passant_left: bool,
    pub can_en_passant_right: bool,
}
impl SquareProperties for Piece {
    fn is_white(&self) -> bool {
        self.features.is_white()
    }
    fn is_black(&self) -> bool {
        self.features.is_black()
    }
    fn is_pawn(&self) -> bool {
        self.features.is_pawn()
    }
    fn is_knight(&self) -> bool {
        self.features.is_knight()
    }
    fn is_bishop(&self) -> bool {
        self.features.is_bishop()
    }
    fn is_rook(&self) -> bool {
        self.features.is_rook()
    }
    fn is_queen(&self) -> bool {
        self.features.is_queen()
    }
    fn is_king(&self) -> bool {
        self.features.is_king()
    }
}

#[allow(non_upper_case_globals)]
impl Piece {
    pub fn new(color: Color, type_: PieceType, destinations: Vec<usize>) -> Self {
        let mut features = [false, false, false, false, false, false, false, false];
        features[color as usize] = true;
        features[type_ as usize + 2] = true;
        Self {
            features,
            destinations: HashSet::from_iter(destinations.iter().cloned()),
            blocking: HashSet::new(),
            can_castle_long: false,
            can_castle_short: false,
            can_en_passant_left: false,
            can_en_passant_right: false,
        }
    }
}

#[rustfmt::skip]
fn default_pieces() -> HashMap<usize, Piece> {
    let mut pieces = HashMap::new();
    pieces.insert(square(A, 1), Piece::new(WHITE, ROOK, vec![]));
    pieces.insert(square(B, 1), Piece::new(WHITE, KNIGHT, vec![square(A, 3), square(C, 3)]));
    pieces.insert(square(C, 1), Piece::new(WHITE, BISHOP, vec![]));
    pieces.insert(square(D, 1), Piece::new(WHITE, QUEEN, vec![]));
    pieces.insert(square(E, 1), Piece::new(WHITE, KING, vec![]));
    pieces.insert(square(F, 1), Piece::new(WHITE, BISHOP, vec![]));
    pieces.insert(square(G, 1), Piece::new(WHITE, KNIGHT, vec![square(F, 3), square(H, 3)]));
    pieces.insert(square(H, 1), Piece::new(WHITE, ROOK, vec![]));

    pieces.insert(square(A, 2), Piece::new(WHITE, PAWN, vec![square(A, 3), square(A, 4)]));
    pieces.insert(square(B, 2), Piece::new(WHITE, PAWN, vec![square(B, 3), square(B, 4)]));
    pieces.insert(square(C, 2), Piece::new(WHITE, PAWN, vec![square(C, 3), square(C, 4)]));
    pieces.insert(square(D, 2), Piece::new(WHITE, PAWN, vec![square(D, 3), square(D, 4)]));
    pieces.insert(square(E, 2), Piece::new(WHITE, PAWN, vec![square(E, 3), square(E, 4)]));
    pieces.insert(square(F, 2), Piece::new(WHITE, PAWN, vec![square(F, 3), square(F, 4)]));
    pieces.insert(square(G, 2), Piece::new(WHITE, PAWN, vec![square(G, 3), square(G, 4)]));
    pieces.insert(square(H, 2), Piece::new(WHITE, PAWN, vec![square(H, 3), square(H, 4)]));

    pieces.insert(square(A, 7), Piece::new(BLACK, PAWN, vec![square(A, 6), square(A, 5)]));
    pieces.insert(square(B, 7), Piece::new(BLACK, PAWN, vec![square(B, 6), square(B, 5)]));
    pieces.insert(square(C, 7), Piece::new(BLACK, PAWN, vec![square(C, 6), square(C, 5)]));
    pieces.insert(square(D, 7), Piece::new(BLACK, PAWN, vec![square(D, 6), square(D, 5)]));
    pieces.insert(square(E, 7), Piece::new(BLACK, PAWN, vec![square(E, 6), square(E, 5)]));
    pieces.insert(square(F, 7), Piece::new(BLACK, PAWN, vec![square(F, 6), square(F, 5)]));
    pieces.insert(square(G, 7), Piece::new(BLACK, PAWN, vec![square(G, 6), square(G, 5)]));
    pieces.insert(square(H, 7), Piece::new(BLACK, PAWN, vec![square(H, 6), square(H, 5)]));

    pieces.insert(square(A, 8), Piece::new(BLACK, ROOK, vec![]));
    pieces.insert(square(B, 8), Piece::new(BLACK, KNIGHT, vec![square(A, 6), square(C, 6)]));
    pieces.insert(square(C, 8), Piece::new(BLACK, BISHOP, vec![]));
    pieces.insert(square(D, 8), Piece::new(BLACK, QUEEN, vec![]));
    pieces.insert(square(E, 8), Piece::new(BLACK, KING, vec![]));
    pieces.insert(square(F, 8), Piece::new(BLACK, BISHOP, vec![]));
    pieces.insert(square(G, 8), Piece::new(BLACK, KNIGHT, vec![square(F, 6), square(H, 6)]));
    pieces.insert(square(H, 8), Piece::new(BLACK, ROOK, vec![]));

    pieces
}

impl Chess {
    pub fn new(pieces: Option<HashMap<usize, Piece>>) -> Self {
        if let Some(pieces) = pieces {
            let mut board = [[false; SQUARE_FEATURES]; BOARD_SIZE];
            for (&location, piece) in &pieces {
                board[location] = piece.features;
            }
            Self {
                pieces,
                to_play: WHITE,
                check: None,
            }
        } else {
            Self {
                pieces: default_pieces(),
                to_play: WHITE,
                check: None,
            }
        }
    }

    pub fn moves(&self) -> Vec<Move> {
        self.pieces
            .iter()
            .filter(|(_, piece)| {
                if self.to_play == WHITE {
                    piece.is_white()
                } else {
                    piece.is_black()
                }
            })
            .flat_map(|(&location, piece)| {
                piece
                    .destinations
                    .iter()
                    .flat_map(move |&destination| {
                        // expand pawn moves onto first/last rank into all possible promotions
                        if piece.is_pawn() && destination > square(H, 7) {
                            vec![WHITE_QUEEN, WHITE_KNIGHT, WHITE_BISHOP, WHITE_ROOK]
                        } else if piece.is_pawn() && destination < square(A, 2) {
                            vec![BLACK_QUEEN, BLACK_KNIGHT, BLACK_BISHOP, BLACK_ROOK]
                        } else {
                            vec![piece.features]
                        }
                        .iter()
                        .map(|&destination_piece| Move {
                            from: location,
                            to: (destination, destination_piece),
                            clear_square: None,
                            rook_square: None,
                        })
                        .collect::<Vec<Move>>()
                    })
                    .chain({
                        let rank = if piece.is_white() { 1 } else { 8 };
                        if piece.can_castle_long {
                            Some(Move {
                                from: square(E, rank),
                                to: (square(C, rank), piece.features),
                                clear_square: Some(square(A, rank)),
                                rook_square: Some(square(D, rank)),
                            })
                        } else if piece.can_castle_short {
                            Some(Move {
                                from: square(E, rank),
                                to: (square(G, rank), piece.features),
                                clear_square: Some(square(H, rank)),
                                rook_square: Some(square(F, rank)),
                            })
                        } else {
                            None
                        }
                    })
                    .chain(if piece.can_en_passant_left {
                        Some(Move {
                            from: location,
                            to: (
                                if piece.is_white() {
                                    location + 7
                                } else {
                                    location - 9
                                },
                                piece.features,
                            ),
                            clear_square: Some(location - 1),
                            rook_square: None,
                        })
                    } else if piece.can_en_passant_right {
                        Some(Move {
                            from: location,
                            to: (
                                if piece.is_white() {
                                    location + 9
                                } else {
                                    location - 7
                                },
                                piece.features,
                            ),
                            clear_square: Some(location + 1),
                            rook_square: None,
                        })
                    } else {
                        None
                    })
            })
            .collect()
    }

    fn make_room_for_knights(&mut self, i: isize) {
        let (x, y) = (i % 8, i / 8);
        for (dx, dy) in KNIGHT_MOVES {
            let (blocked_x, blocked_y) = (x + dx, y + dy);
            if blocked_x >= 0 && blocked_x < 8 && blocked_y >= 0 && blocked_y < 8 {
                let blocked_i: usize = (blocked_x + blocked_y * 8).try_into().unwrap();
                if let Some(blocked_piece) = self.pieces.get_mut(&blocked_i) {
                    if blocked_piece.is_knight() {
                        blocked_piece.destinations.insert(i as usize);
                    }
                }
            }
        }
    }

    pub fn update_pieces(&mut self, move_: &Move) {
        let from_i = move_.from as isize;
        let (from_x, from_y) = (from_i % 8, from_i / 8);
        let to_i = move_.to.0 as isize;
        let (to_x, to_y) = (to_i % 8, to_i / 8);

        let mut piece = self.pieces.remove(&move_.from).unwrap();
        let piece_features = piece.features.clone();
        piece.destinations.clear();
        self.pieces.insert(move_.to.0, piece);

        // castling and en passant remove an extra piece
        if let Some(clear_square) = move_.clear_square {
            let piece = self.pieces.remove(&clear_square).unwrap();
            self.make_room_for_knights(clear_square as isize);

            // castling also places that removed piece (the rook)
            if let Some(rook_square) = move_.rook_square {
                self.pieces.insert(rook_square, piece);
            }
        }

        self.make_room_for_knights(from_i);
        for (dx, dy) in DIRECTIONS {
            let (mut blocked_x, mut blocked_y) = (from_x, from_y);
            while blocked_x >= 0 && blocked_x < 8 && blocked_y >= 0 && blocked_y < 8 {
                let blocked_i = (blocked_x + blocked_y * 8) as usize;
                if !self.pieces.contains_key(&blocked_i) {
                    blocked_x += dx;
                    blocked_y += dy;
                    continue;
                }
                let blocked_piece_features = self.pieces[&blocked_i].features.clone();

                if blocked_piece_features.is_queen()
                    || if dx * dy == 0 {
                        blocked_piece_features.is_rook()
                    } else {
                        blocked_piece_features.is_bishop()
                    }
                {
                    let mut reachable_square = true;
                    let (mut target_x, mut target_y) = (blocked_x - dx, blocked_y - dy);
                    while target_x >= 0 && target_x < 8 && target_y >= 0 && target_y < 8 {
                        let target_i = (target_x + target_y * 8) as usize;
                        if target_i == move_.to.0 {
                            if same_color!(piece_features, blocked_piece_features) {
                                self.pieces
                                    .get_mut(&blocked_i)
                                    .unwrap()
                                    .destinations
                                    .insert(target_i);
                            }
                            reachable_square = false;
                        } else if self.pieces.contains_key(&target_i) {
                            if same_color!(self.pieces[&target_i], blocked_piece_features) {
                                self.pieces
                                    .get_mut(&blocked_i)
                                    .unwrap()
                                    .destinations
                                    .insert(target_i);
                            }
                            break;
                        }
                        if reachable_square {
                            self.pieces
                                .get_mut(&blocked_i)
                                .unwrap()
                                .destinations
                                .insert(target_i);
                        } else {
                            if !self
                                .pieces
                                .get_mut(&blocked_i)
                                .unwrap()
                                .destinations
                                .remove(&target_i)
                            {
                                break;
                            }
                        }
                        target_x -= dx;
                        target_y -= dy;
                    }
                } else if blocked_piece_features.is_king()
                    && (blocked_x - from_x).abs() < 2
                    && (blocked_y - from_y).abs() < 2
                {
                    self.pieces
                        .get_mut(&blocked_i)
                        .unwrap()
                        .destinations
                        .insert(move_.from);
                } else if blocked_piece_features.is_pawn()
                    && (blocked_piece_features.is_white() && blocked_y < from_y
                        || blocked_piece_features.is_black() && blocked_y > from_y)
                {
                    if from_x == blocked_x {
                        if (from_y - blocked_y).abs() == 1
                            || (from_y - blocked_y).abs() == 2 && (blocked_y == 1 || blocked_y == 6)
                        {
                            self.pieces
                                .get_mut(&blocked_i)
                                .unwrap()
                                .destinations
                                .insert(move_.from);
                        }

                        // check for a piece behind the piece if we moved out of the way of a pawn that can move 2 squares
                        if blocked_y == 1 && from_y == 2 {
                            if !self.pieces.get(&(move_.from + 8)).is_some() {
                                self.pieces
                                    .get_mut(&blocked_i)
                                    .unwrap()
                                    .destinations
                                    .insert(move_.from + 8);
                            }
                        } else if blocked_y == 6 && from_y == 5 {
                            if !self.pieces.get(&(move_.from - 8)).is_some() {
                                self.pieces
                                    .get_mut(&blocked_i)
                                    .unwrap()
                                    .destinations
                                    .insert(move_.from - 8);
                            }
                        }
                    } else if !same_color!(blocked_piece_features, piece_features)
                        && (from_x - blocked_x).abs() == 1
                        && (from_y - blocked_y).abs() == 1
                    {
                        self.pieces
                            .get_mut(&blocked_i)
                            .unwrap()
                            .destinations
                            .remove(&move_.from);
                    }
                }
                break;
            }
        }

        // TODO: check if castling is possible

        // update pieces after move
        for (dx, dy) in KNIGHT_MOVES {
            let (blocked_x, blocked_y) = (to_x + dx, to_y + dy);
            if blocked_x >= 0 && blocked_x < 8 && blocked_y >= 0 && blocked_y < 8 {
                let blocked_i: usize = (blocked_x + blocked_y * 8).try_into().unwrap();
                if self.pieces.contains_key(&blocked_i) {
                    let blocked_piece_features = self.pieces[&blocked_i].features.clone();
                    if same_color!(blocked_piece_features, piece_features) {
                        if piece_features.is_knight() {
                            self.pieces
                                .get_mut(&move_.from)
                                .unwrap()
                                .destinations
                                .insert(blocked_i);
                        }
                        if blocked_piece_features.is_knight() {
                            self.pieces
                                .get_mut(&blocked_i)
                                .unwrap()
                                .destinations
                                .insert(move_.to.0);
                        }
                    }
                }
            }
        }
        for (dx, dy) in DIRECTIONS {
            let (mut blocked_x, mut blocked_y) = (to_x, to_y);
            while blocked_x >= 0 && blocked_x < 8 && blocked_y >= 0 && blocked_y < 8 {
                let blocked_i: usize = (blocked_x + blocked_y * 8).try_into().unwrap();
                if piece_features.is_queen()
                    || if dx * dy == 0 {
                        piece_features.is_rook()
                    } else {
                        piece_features.is_bishop()
                    }
                    || piece_features.is_king()
                        && (to_x - blocked_x).abs() < 2
                        && (to_y - blocked_y).abs() < 2
                // TODO: pawn
                {
                    if let Some(blocked_piece) = self.pieces.get(&blocked_i) {
                        if same_color!(piece_features, blocked_piece) {
                            self.pieces
                                .get_mut(&move_.to.0)
                                .unwrap()
                                .destinations
                                .insert(blocked_i);
                        }
                    } else {
                        self.pieces
                            .get_mut(&move_.to.0)
                            .unwrap()
                            .destinations
                            .insert(blocked_i);
                    }
                }

                if self.pieces.contains_key(&blocked_i) {
                    let blocked_piece_features = self.pieces[&blocked_i].features.clone();
                    if blocked_piece_features.is_queen()
                        || if dx * dy == 0 {
                            blocked_piece_features.is_rook()
                        } else {
                            blocked_piece_features.is_bishop()
                        }
                    {
                        if same_color!(blocked_piece_features, piece_features) {
                            self.pieces
                                .get_mut(&blocked_i)
                                .unwrap()
                                .destinations
                                .remove(&move_.to.0);
                        }
                        let (mut target_x, mut target_y) = (to_x - dx, to_y - dy);
                        while target_x >= 0 && target_x < 8 && target_y >= 0 && target_y < 8 {
                            let target_i: usize = (target_x + target_y * 8).try_into().unwrap();

                            self.pieces
                                .get_mut(&blocked_i)
                                .unwrap()
                                .destinations
                                .remove(&target_i);
                            target_x -= dx;
                            target_y -= dy;
                        }
                    } else if blocked_piece_features.is_king()
                        && (blocked_x - to_x).abs() < 2
                        && (blocked_y - to_y).abs() < 2
                        && same_color!(blocked_piece_features, piece_features)
                    {
                        self.pieces
                            .get_mut(&blocked_i)
                            .unwrap()
                            .destinations
                            .remove(&move_.to.0);
                    } else if blocked_piece_features.is_pawn()
                        && (blocked_piece_features.is_white() && blocked_y < from_y
                            || blocked_piece_features.is_black() && blocked_y > from_y)
                    {
                        // TODO
                    }
                    break;
                }

                blocked_x += dx;
                blocked_y += dy;
            }
        }
        // en passantable pawn
        //     if from_piece.is_pawn()
        //     && !from_piece.same_color(blocked_piece)
        //     && (from_i - blocked_i).abs() == 1
        // {
        //     blocked_piece.destinations.insert(to_i)
        // }
    }

    pub fn result(&self) -> Option<Result> {
        None
    }
}
