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

fn white_pawn_captures_left(pawns: u64, mask: u64) -> u64 {
    (pawns << 7) & !(0x8080808080808080) & mask
}

fn white_pawn_captures_right(pawns: u64, mask: u64) -> u64 {
    (pawns << 9) & !(0x0101010101010101) & mask
}

fn black_pawn_captures_left(pawns: u64, mask: u64) -> u64 {
    (pawns >> 9) & !(0x8080808080808080) & mask
}

fn black_pawn_captures_right(pawns: u64, mask: u64) -> u64 {
    (pawns >> 7) & !(0x0101010101010101) & mask
}

pub fn gen_pseudolegal_moves(pos: &Position) -> MoveList {
    let mut moves = MoveList {
        data: [Move(0);_],
        count: 0
    };



    // common masks

    let occ = pos.bb.iter().fold(0, |acc, x|acc|x);
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




    moves
}