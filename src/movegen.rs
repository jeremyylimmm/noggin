use crate::*;

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

    pub fn swap(&mut self, a: usize, b: usize) {
        self.data.swap(a, b)
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
    (white_pawn_pushes(pawns & (0xff << 8), occ) << 8) & !occ
}

fn black_pawn_double_pushes(pawns: u64, occ: u64) -> u64 {
    (black_pawn_pushes(pawns & (0xff << 48), occ) >> 8) & !occ
}

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
    knight_attacks(from) & !allies
}

fn king_moves(from: u32, allies: u64) -> u64 {
    king_attacks(from) & !allies
}

fn rook_moves(from: u32, occ: u64, allies: u64) -> u64 {
    rook_attacks(from, occ) & !allies
}

fn bishop_moves(from: u32, occ: u64, allies: u64) -> u64 {
    bishop_attacks(from, occ) & !allies
}

fn queen_moves(from: u32, occ: u64, allies: u64) -> u64 {
    rook_moves(from, occ, allies) | bishop_moves(from, occ, allies)
}

fn gen_non_batched_moves<F: Fn(u32)->u64>(mut pieces: u64, f: F, moves: &mut MoveList) {
    while pieces != 0 {
        let from = pieces.trailing_zeros();
        let mut to_bb = f(from);

        while to_bb != 0 {
            let to = to_bb.trailing_zeros();

            moves.push(Move::new(from as usize, to as usize, Piece::None));

            to_bb &= to_bb - 1;
        }

        pieces &= pieces - 1;
    }
}

fn add_pawn_move(from: i32, to: i32, promotion_rank: i32, moves: &mut MoveList) {
    let to_rank = (to >> 3) & 7;

    if to_rank == promotion_rank {
        moves.push(Move::new(from as usize, to as usize, Piece::Knight));
        moves.push(Move::new(from as usize, to as usize, Piece::Bishop));
        moves.push(Move::new(from as usize, to as usize, Piece::Rook));
        moves.push(Move::new(from as usize, to as usize, Piece::Queen));
    }
    else {
        moves.push(Move::new(from as usize, to as usize, Piece::None));
    }
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

    let (home_rank, promotion_rank) = match pos.to_move {
        Side::White => (0, 7),
        Side::Black => (7, 0)
    };


    // pawn pushes

    let pawns = pos.bb[Piece::Pawn.bb_index(pos.to_move).unwrap()];

    let (mut pawn_pushes, mut double_pawn_pushes, pawn_push_offset) = match pos.to_move {
        Side::White => (white_pawn_pushes(pawns, occ), white_pawn_double_pushes(pawns, occ), 1i32),
        Side::Black => (black_pawn_pushes(pawns, occ), black_pawn_double_pushes(pawns, occ), -1i32),
    };

    while pawn_pushes != 0 {
        let to = pawn_pushes.trailing_zeros() as i32;
        let from = to - pawn_push_offset * 8 as i32; 
        add_pawn_move(from, to, promotion_rank, &mut moves);
        pawn_pushes &= pawn_pushes-1;
    }

    while double_pawn_pushes != 0 {
        let to = double_pawn_pushes.trailing_zeros() as i32;
        let from = to - pawn_push_offset * 16 as i32;
        add_pawn_move(from, to, promotion_rank, &mut moves);
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
        add_pawn_move(from, to, promotion_rank, &mut moves);
        pawn_captures_left &= pawn_captures_left - 1;
    }

    while pawn_captures_right != 0 {
        let to = pawn_captures_right.trailing_zeros() as i32;
        let from = to - pawn_capture_offset_right;
        add_pawn_move(from, to, promotion_rank, &mut moves);
        pawn_captures_right &= pawn_captures_right - 1;
    }

    // knight moves

    let knights = pos.bb[Piece::Knight.bb_index(pos.to_move).unwrap()];
    gen_non_batched_moves(knights, |from|knight_moves(from, allies), &mut moves);

    // king moves

    let kings = pos.bb[Piece::King.bb_index(pos.to_move).unwrap()];
    gen_non_batched_moves(kings, |from|king_moves(from, allies), &mut moves);

    // rook moves

    let rooks = pos.bb[Piece::Rook.bb_index(pos.to_move).unwrap()];
    gen_non_batched_moves(rooks, |from|rook_moves(from, occ, allies), &mut moves);

    // bishop moves

    let bishops = pos.bb[Piece::Bishop.bb_index(pos.to_move).unwrap()];
    gen_non_batched_moves(bishops, |from|bishop_moves(from, occ, allies), &mut moves);

    // queen moves
    
    let queens = pos.bb[Piece::Queen.bb_index(pos.to_move).unwrap()];
    gen_non_batched_moves(queens, |from|queen_moves(from, occ, allies), &mut moves);





    // castling

    let (kcastle_flag, qcastle_flag) = match pos.to_move {
        Side::White => (WK_CASTLE, WQ_CASTLE),
        Side::Black => (BK_CASTLE, BQ_CASTLE),
    };

    let king_sq = pos.bb[Piece::King.bb_index(pos.to_move).unwrap()].trailing_zeros();

    let king_rank = (king_sq >> 3) & 7;
    let king_file = king_sq & 7;

    let home_rank_mask = 0xff << (home_rank*8);
    let kcastle_path = (FILE_F | FILE_G) & home_rank_mask;
    let qcastle_path = (FILE_C | FILE_D) & home_rank_mask;

    let bb_attacked = |mut bb: u64| {
        while bb != 0 {
            let sq = bb.trailing_zeros();
            if pos.sq_attacked(sq as usize, pos.to_move.opp()) {
                return true;
            }
            bb &= bb-1;
        }
        return false;
    };

    if (pos.castling & kcastle_flag) != 0 {
        assert!(king_rank == home_rank);
        assert!(king_file == 4);

        if (occ & kcastle_path) == 0 && !bb_attacked(kcastle_path) && !pos.checked(pos.to_move) {
            let to = home_rank*8+6;
            moves.push(Move::new(king_sq as usize, to as usize, Piece::None));
        }
    }

    if (pos.castling & qcastle_flag) != 0 {
        assert!(king_rank == home_rank);
        assert!(king_file == 4);

        if (occ & (qcastle_path | (FILE_B & home_rank_mask))) == 0 && !bb_attacked(qcastle_path) && !pos.checked(pos.to_move) {
            let to = home_rank*8+2;
            moves.push(Move::new(king_sq as usize, to as usize, Piece::None));
        }
    }


    moves
}