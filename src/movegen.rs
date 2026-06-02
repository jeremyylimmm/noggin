use crate::{Position, Move, Piece, Side};

pub struct MoveList {
    data: [Move;256],
    count: usize
}

impl MoveList {
    fn push(&mut self, mv: Move) {
        self.data[self.count] = mv;
        self.count += 1;
    }

    pub fn len(&self) -> usize {
        self.count
    }
}

impl std::ops::Index<usize> for MoveList {
    type Output = Move;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::IndexMut<usize> for MoveList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

fn white_pawn_pushes(pawns: u64, occ: u64) -> u64 {
    (pawns << 8) & !occ
}

fn black_pawn_pushes(pawns: u64, occ: u64) -> u64 {
    (pawns >> 8) & !occ
}

fn white_pawn_double_pushes(pawns: u64, occ: u64) -> u64 {
    ((pawns & (0xff << 8)) << 16) & !occ
}

fn black_pawn_double_pushes(pawns: u64, occ: u64) -> u64 {
    ((pawns & (0xff << 48)) >> 16) & !occ
}

const FILE_A: u64 = 0x0101010101010101;
const FILE_B: u64 = 0x0202020202020202;
const FILE_G: u64 = 0x4040404040404040;
const FILE_H: u64 = 0x8080808080808080;

fn white_pawn_captures_left(pawns: u64, mask: u64) -> u64 {
    (pawns << 7) & !(FILE_H) & mask
}

fn white_pawn_captures_right(pawns: u64, mask: u64) -> u64 {
    (pawns << 9) & !(FILE_A) & mask
}

fn black_pawn_captures_left(pawns: u64, mask: u64) -> u64 {
    (pawns >> 9) & !(FILE_H) & mask
}

fn black_pawn_captures_right(pawns: u64, mask: u64) -> u64 {
    (pawns >> 7) & !(FILE_A) & mask
}

fn knight_moves(from: u32, allies: u64) -> u64 {
    let knight = 1u64 << from;

    let m0 = knight <<  6 & !(FILE_G | FILE_H);
    let m1 = knight << 15 & !(FILE_H);
    let m2 = knight << 17 & !(FILE_A);
    let m3 = knight << 10 & !(FILE_A | FILE_B);
    let m4 = knight >>  6 & !(FILE_A | FILE_B);
    let m5 = knight >> 15 & !(FILE_A);
    let m6 = knight >> 17 & !(FILE_H);
    let m7 = knight >> 10 & !(FILE_G | FILE_H);

    (m0|m1|m2|m3|m4|m5|m6|m7) & !allies
}

fn king_moves(from: u32, allies: u64) -> u64 {
    let king = 1u64 << from;

    let m0 = (king << 7) & !FILE_H;
    let m1 =  king << 8;
    let m2 = (king << 9) & !FILE_A;
    let m3 = (king << 1) & !FILE_A;
    let m4 = (king >> 7) & !FILE_A;
    let m5 =  king >> 8;
    let m6 = (king >> 9) & !FILE_H;
    let m7 = (king >> 1) & !FILE_H;

    (m0|m1|m2|m3|m4|m5|m6|m7) & !allies
}

pub fn gen_pseudolegal_moves(pos: &Position) -> MoveList {
    let mut moves = MoveList {
        data: [Move(0);_],
        count: 0
    };



    // common masks

    let occ = pos.bb.iter().fold(0, |acc, x|acc|x);
    let allies = pos.bb[if matches!(pos.to_move, Side::White) {0..6} else {6..12}].iter().fold(0, |acc, x|acc|x);
    let enemies = pos.bb[if matches!(pos.to_move, Side::Black) {0..6} else {6..12}].iter().fold(0, |acc, x|acc|x);
    let ep_mask = if let Some(ep) = pos.ep_sq {1u64 << ep} else {0};





    // pawn pushes

    let pawns = pos.bb[Piece::Pawn.bb_index(pos.to_move).unwrap()];

    let (mut pawn_pushes, mut double_pawn_pushes, pawn_push_offset) = match pos.to_move {
        Side::White => (white_pawn_pushes(pawns, occ), white_pawn_double_pushes(pawns, occ), 1i32),
        Side::Black => (black_pawn_pushes(pawns, occ), black_pawn_double_pushes(pawns, occ), -1i32),
    };

    while pawn_pushes != 0 {
        let to = pawn_pushes.trailing_zeros() as i32;
        let from = to - pawn_push_offset * 8 as i32; 

        moves.push(Move::new(from as usize, to as usize));        

        pawn_pushes &= pawn_pushes-1;
    }

    while double_pawn_pushes != 0 {
        let to = double_pawn_pushes.trailing_zeros() as i32;
        let from = to - pawn_push_offset * 16 as i32;

        moves.push(Move::new(from as usize, to as usize));

        double_pawn_pushes &= double_pawn_pushes - 1;
    }


    // pawn captures

    let pawn_capture_mask = enemies | ep_mask;

    let (mut pawn_captures_left, mut pawn_captures_right, pawn_capture_offset_left, pawn_capture_offset_right) = match pos.to_move {
        Side::White => (white_pawn_captures_left(pawns, pawn_capture_mask), white_pawn_captures_right(pawns, pawn_capture_mask), 7, 9), 
        Side::Black => (black_pawn_captures_left(pawns, pawn_capture_mask), black_pawn_captures_right(pawns, pawn_capture_mask), -9, -7), 
    };

    while pawn_captures_left != 0 {
        let to = pawn_captures_left.trailing_zeros() as i32;
        let from = to - pawn_capture_offset_left;

        moves.push(Move::new(from as usize, to as usize));

        pawn_captures_left &= pawn_captures_left - 1;
    }

    while pawn_captures_right != 0 {
        let to = pawn_captures_right.trailing_zeros() as i32;
        let from = to - pawn_capture_offset_right;

        moves.push(Move::new(from as usize, to as usize));

        pawn_captures_right &= pawn_captures_right - 1;
    }


    // knight moves

    let mut knights = pos.bb[Piece::Knight.bb_index(pos.to_move).unwrap()];

    while knights != 0 {
        let from = knights.trailing_zeros();
        let mut to_bb = knight_moves(from, allies);

        while to_bb != 0 {
            let to = to_bb.trailing_zeros();
            moves.push(Move::new(from as usize, to as usize));
            to_bb &= to_bb - 1;
        }

        knights &= knights - 1;
    }



    // king moves

    let mut kings = pos.bb[Piece::King.bb_index(pos.to_move).unwrap()];

    while kings != 0 {
        let from = kings.trailing_zeros();
        let mut to_bb = king_moves(from, allies);

        while to_bb != 0 {
            let to = to_bb.trailing_zeros();
            moves.push(Move::new(from as usize, to as usize));
            to_bb &= to_bb - 1;
        }

        kings &= kings - 1;
    }


    moves
}