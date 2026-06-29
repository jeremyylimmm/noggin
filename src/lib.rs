mod attacks;
mod fen;
mod move_gen;

#[allow(unused)]
pub const STARTPOS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
#[allow(unused)]
pub const KIWIPETE_FEN: &str =
    "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

#[allow(unused)]
pub const MASK_RANK_1: u64 = 0x00000000000000ff;
#[allow(unused)]
pub const MASK_RANK_2: u64 = 0x000000000000ff00;
#[allow(unused)]
pub const MASK_RANK_3: u64 = 0x0000000000ff0000;
#[allow(unused)]
pub const MASK_RANK_4: u64 = 0x00000000ff000000;
#[allow(unused)]
pub const MASK_RANK_5: u64 = 0x000000ff00000000;
#[allow(unused)]
pub const MASK_RANK_6: u64 = 0x0000ff0000000000;
#[allow(unused)]
pub const MASK_RANK_7: u64 = 0x00ff000000000000;
#[allow(unused)]
pub const MASK_RANK_8: u64 = 0xff00000000000000;

#[allow(unused)]
pub const MASK_FILE_A: u64 = 0x0101010101010101;
#[allow(unused)]
pub const MASK_FILE_B: u64 = 0x0202020202020202;
#[allow(unused)]
pub const MASK_FILE_C: u64 = 0x0404040404040404;
#[allow(unused)]
pub const MASK_FILE_D: u64 = 0x0808080808080808;
#[allow(unused)]
pub const MASK_FILE_E: u64 = 0x1010101010101010;
#[allow(unused)]
pub const MASK_FILE_F: u64 = 0x2020202020202020;
#[allow(unused)]
pub const MASK_FILE_G: u64 = 0x4040404040404040;
#[allow(unused)]
pub const MASK_FILE_H: u64 = 0x8080808080808080;

#[allow(unused)]
const MASK_RANK: [u64; 8] = [
    MASK_RANK_1,
    MASK_RANK_2,
    MASK_RANK_3,
    MASK_RANK_4,
    MASK_RANK_5,
    MASK_RANK_6,
    MASK_RANK_7,
    MASK_RANK_8,
];

#[allow(unused)]
const MASK_FILE: [u64; 8] = [
    MASK_FILE_A,
    MASK_FILE_B,
    MASK_FILE_C,
    MASK_FILE_D,
    MASK_FILE_E,
    MASK_FILE_F,
    MASK_FILE_G,
    MASK_FILE_H,
];

#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Side {
    White,
    Black,
}

type CastleRights = u8;

const CASTLE_RIGHT_K_WHITE: CastleRights = 1 << 0;
const CASTLE_RIGHT_Q_WHITE: CastleRights = 1 << 1;
const CASTLE_RIGHT_K_BLACK: CastleRights = 1 << 2;
const CASTLE_RIGHT_Q_BLACK: CastleRights = 1 << 3;

#[derive(Copy, Clone)]
pub struct Sq(u8);

pub type Board = [Option<Piece>; 64];

#[derive(Copy, Clone)]
pub struct Move(u16);

pub struct Position {
    bbs: [u64; 12],
    board: Board,
    stm: Side,
    castle_rights: CastleRights,
    ep: Option<Sq>,
    halfmove_clock: i16,
    fullmoves: i16,
}

impl Position {
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        fen::parse(fen)
    }

    pub fn fen(&self) -> String {
        fen::to_fen(self)
    }

    pub fn occ(&self) -> u64 {
        self.bbs.iter().fold(0, |acc, x| acc | x)
    }

    pub fn side_occ(&self, side: Side) -> u64 {
        match side {
            Side::White => self.bbs[..6].iter().fold(0, |acc, x| acc | x),
            Side::Black => self.bbs[6..].iter().fold(0, |acc, x| acc | x),
        }
    }

    pub fn gen_psuedolegal_moves(&self) -> MoveList {
        move_gen::gen_psuedolegal(self)
    }
}

impl Sq {
    pub fn from_coords(rank: usize, file: usize) -> Self {
        assert!(rank < 8);
        assert!(file < 8);
        Self((rank * 8 + file) as u8)
    }

    pub fn from_san(san: &str) -> Option<Self> {
        let chars: Vec<char> = san.chars().collect();

        if chars.len() != 2 {
            return None;
        }

        let file = chars[0] as i32 - 'a' as i32;
        let rank = chars[1] as i32 - '1' as i32;

        if file < 0 || file >= 8 || rank < 0 || rank >= 8 {
            return None;
        }

        Some(Self::from_coords(rank as _, file as _))
    }

    pub fn rank(&self) -> usize {
        ((self.0 >> 3) & 7) as usize
    }

    pub fn file(&self) -> usize {
        (self.0 & 7) as usize
    }

    pub fn san(&self) -> String {
        let f = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][self.file()];
        let r = self.rank() + 1;
        format!("{}{}", f, r)
    }

    pub const fn bb(&self) -> u64 {
        1u64 << self.0
    }
}

impl std::ops::Index<Sq> for Board {
    type Output = Option<Piece>;

    fn index(&self, index: Sq) -> &Self::Output {
        &self[index.0 as usize]
    }
}

impl std::ops::IndexMut<Sq> for Board {
    fn index_mut(&mut self, index: Sq) -> &mut Self::Output {
        &mut self[index.0 as usize]
    }
}

trait Bitboards {
    fn get(&self, piece: Piece, side: Side) -> u64;
    fn get_mut(&mut self, piece: Piece, side: Side) -> &mut u64;
}

impl Bitboards for [u64; 12] {
    fn get(&self, piece: Piece, side: Side) -> u64 {
        self[bb_index(piece, side)]
    }

    fn get_mut(&mut self, piece: Piece, side: Side) -> &mut u64 {
        &mut self[bb_index(piece, side)]
    }
}

fn bb_index(piece: Piece, side: Side) -> usize {
    piece.id() + side.id() * 6
}

impl Piece {
    fn id(&self) -> usize {
        *self as usize
    }

    fn from_id(id: usize) -> Option<Self> {
        match id {
            0 => Some(Piece::Pawn),
            1 => Some(Piece::Knight),
            2 => Some(Piece::Bishop),
            3 => Some(Piece::Rook),
            4 => Some(Piece::Queen),
            5 => Some(Piece::King),
            _ => None,
        }
    }

    fn san_lowercase(&self) -> char {
        match self {
            Piece::Pawn => 'p',
            Piece::Knight => 'n',
            Piece::Bishop => 'b',
            Piece::Rook => 'r',
            Piece::Queen => 'q',
            Piece::King => 'k',
        }
    }
}

impl Side {
    fn id(&self) -> usize {
        *self as usize
    }

    fn char(&self) -> char {
        match self {
            Side::White => 'w',
            Side::Black => 'b',
        }
    }

    fn sign(&self) -> i32 {
        match self {
            Side::White => 1,
            Side::Black => -1,
        }
    }

    fn home_rank(&self) -> usize {
        match self {
            Side::White => 0,
            Side::Black => 7,
        }
    }

    fn pawn_rank(&self) -> usize {
        match self {
            Side::White => 1,
            Side::Black => 6,
        }
    }

    fn opp(&self) -> Side {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }

    fn promotion_rank(&self) -> usize {
        self.opp().home_rank()
    }
}

impl Move {
    pub fn new(from: Sq, to: Sq, promotion: Option<Piece>) -> Self {
        match promotion {
            None | Some(Piece::Knight | Piece::Bishop | Piece::Rook | Piece::Queen) => (),
            _ => {
                panic!("move promotion piece is invalid");
            }
        }

        Self(
            0 | (from.0 as u16)
                | (to.0 as u16) << 6
                | (promotion.map(|x| x.id() as u16 + 1).unwrap_or(0)) << 12,
        )
    }

    pub fn from(&self) -> Sq {
        Sq((self.0 & 63) as _)
    }

    pub fn to(&self) -> Sq {
        Sq(((self.0 >> 6) & 63) as _)
    }

    pub fn promotion(&self) -> Option<Piece> {
        let field = (self.0 >> 12) & 7;

        if field == 0 {
            None
        } else {
            Some(Piece::from_id(field as usize - 1).unwrap())
        }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.from().san(),
            self.to().san(),
            self.promotion()
                .map_or("".to_string(), |p| format!("{}", p.san_lowercase())),
        )
    }
}

#[cfg(test)]
mod move_enc_tests {
    use super::*;

    #[test]
    fn test_null_promotion_encoding() {
        assert!(Move::new(Sq(10), Sq(10), None).promotion().is_none());
    }

    #[test]
    fn test_knight_promotion_encoding() {
        assert!(matches!(
            Move::new(Sq(10), Sq(10), Some(Piece::Knight)).promotion(),
            Some(Piece::Knight)
        ));
    }

    #[test]
    fn test_bishop_promotion_encoding() {
        assert!(matches!(
            Move::new(Sq(10), Sq(10), Some(Piece::Bishop)).promotion(),
            Some(Piece::Bishop)
        ));
    }

    #[test]
    fn test_rook_promotion_encoding() {
        assert!(matches!(
            Move::new(Sq(10), Sq(10), Some(Piece::Rook)).promotion(),
            Some(Piece::Rook)
        ));
    }

    #[test]
    fn test_queen_promotion_encoding() {
        assert!(matches!(
            Move::new(Sq(10), Sq(10), Some(Piece::Queen)).promotion(),
            Some(Piece::Queen)
        ));
    }
}

pub struct MoveList {
    moves: [Move; 256],
    len: usize,
}

impl MoveList {
    fn new() -> Self {
        Self {
            moves: [Move(0); _],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn push(&mut self, mv: Move) {
        assert!(self.len() < self.moves.len());
        self.moves[self.len] = mv;
        self.len += 1;
    }
}

impl std::ops::Index<usize> for MoveList {
    type Output = Move;

    fn index(&self, index: usize) -> &Self::Output {
        &self.moves[index]
    }
}

impl std::ops::IndexMut<usize> for MoveList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.moves[index]
    }
}

impl<'a> IntoIterator for &'a MoveList {
    type Item = &'a Move;
    type IntoIter = std::slice::Iter<'a, Move>;

    fn into_iter(self) -> Self::IntoIter {
        self.moves[..self.len()].iter()
    }
}

impl IntoIterator for MoveList {
    type Item = Move;
    type IntoIter = std::iter::Take<std::array::IntoIter<Move, 256>>;

    fn into_iter(self) -> Self::IntoIter {
        self.moves.into_iter().take(self.len())
    }
}

pub fn iter_bb(bb: u64) -> BitboardIterator {
    BitboardIterator(bb)
}

#[derive(Copy, Clone)]
pub struct BitboardIterator(u64);

impl Iterator for BitboardIterator {
    type Item = Sq;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 != 0 {
            let sq = Sq(self.0.trailing_zeros() as _);
            self.0 &= self.0 - 1;
            Some(sq)
        } else {
            None
        }
    }
}
