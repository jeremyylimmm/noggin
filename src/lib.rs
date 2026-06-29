mod fen;

#[allow(unused)]
pub const STARTPOS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
#[allow(unused)]
pub const KIWIPETE_FEN: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

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
struct Sq(u8);

pub type Board = [Option<Piece>; 64];

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

    pub fn side_occ(&self, side: Side) -> u64 {
        match side {
            Side::White => self.bbs[..6].iter().fold(0, |acc, x| acc | x),
            Side::Black => self.bbs[6..].iter().fold(0, |acc, x| acc | x),
        }
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

    pub fn bb(&self) -> u64 {
        1u64 << self.0
    }
}

trait BoardTrait {
    fn empty() -> Self;
}

impl BoardTrait for Board {
    fn empty() -> Self {
        [None; _]
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
}
