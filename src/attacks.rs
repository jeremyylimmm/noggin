use crate::*;

use crate::generated::sliding_attacks;

pub fn pawn_pushes(pawns: u64, occ: u64, side: Side) -> u64 {
    match side {
        Side::White => (pawns << 8) & !occ,
        Side::Black => (pawns >> 8) & !occ,
    }
}

pub fn pawn_double_pushes(pawns: u64, occ: u64, side: Side) -> u64 {
    let single = pawn_pushes(pawns & MASK_RANK[side.pawn_rank()], occ, side);
    pawn_pushes(single, occ, side)
}

pub fn pawn_attacks_left(pawns: u64, side: Side) -> (u64, i32) {
    match side {
        Side::White => ((pawns << 7) & !MASK_FILE_H, 7),
        Side::Black => ((pawns >> 9) & !MASK_FILE_H, -9),
    }
}

pub fn pawn_attacks_right(pawns: u64, side: Side) -> (u64, i32) {
    match side {
        Side::White => ((pawns << 9) & !MASK_FILE_A, 9),
        Side::Black => ((pawns >> 7) & !MASK_FILE_A, -7),
    }
}

pub fn pawn_captures_left(pawns: u64, opp_occ: u64, side: Side) -> (u64, i32) {
    let (bb, offset) = pawn_attacks_left(pawns, side);
    (bb & opp_occ, offset)
}

pub fn pawn_captures_right(pawns: u64, opp_occ: u64, side: Side) -> (u64, i32) {
    let (bb, offset) = pawn_attacks_right(pawns, side);
    (bb & opp_occ, offset)
}

pub fn pawn_attacks(pawns: u64, side: Side) -> u64 {
    pawn_attacks_left(pawns, side).0 | pawn_attacks_right(pawns, side).0
}

const KING_ATTACKS: [u64; 64] = {
    let mut table = [0u64; _];

    let mut sq = Sq(0);

    while sq.0 < 64 {
        let base = sq.bb();

        table[sq.0 as usize] = 0
            | ((base >> 1) & !MASK_FILE_H)
            | ((base << 7) & !MASK_FILE_H)
            | (base << 8)
            | ((base << 9) & !MASK_FILE_A)
            | ((base << 1) & !MASK_FILE_A)
            | ((base >> 7) & !MASK_FILE_A)
            | (base >> 8)
            | ((base >> 9) & !MASK_FILE_H);

        sq.0 += 1;
    }

    table
};

const KNIGHT_ATTACKS: [u64; 64] = {
    let mut table = [0u64; _];

    let mut sq = Sq(0);

    while sq.0 < 64 {
        let base = sq.bb();

        table[sq.0 as usize] = 0
            | ((base << 6) & !(MASK_FILE_G | MASK_FILE_H))
            | ((base << 15) & !(MASK_FILE_H))
            | ((base << 17) & !(MASK_FILE_A))
            | ((base << 10) & !(MASK_FILE_A | MASK_FILE_B))
            | ((base >> 6) & !(MASK_FILE_A | MASK_FILE_B))
            | ((base >> 15) & !(MASK_FILE_A))
            | ((base >> 17) & !(MASK_FILE_H))
            | ((base >> 10) & !(MASK_FILE_G | MASK_FILE_H));

        sq.0 += 1;
    }

    table
};

pub fn king_attacks(sq: Sq) -> u64 {
    KING_ATTACKS[sq.0 as usize]
}

pub fn king_moves(sq: Sq, allies: u64) -> u64 {
    king_attacks(sq) & !allies
}

pub fn knight_attacks(sq: Sq) -> u64 {
    KNIGHT_ATTACKS[sq.0 as usize]
}

pub fn knight_moves(sq: Sq, allies: u64) -> u64 {
    KNIGHT_ATTACKS[sq.0 as usize] & !allies
}

fn compute_magic_index(occ: u64, mask: u64, magic: u64, shift: u32) -> usize {
    ((occ & mask).overflowing_mul(magic).0 >> shift) as usize
}

pub fn rook_attacks(sq: Sq, occ: u64) -> u64 {
    let sq = sq.0 as usize;

    let magic = sliding_attacks::ROOK_MAGICS[sq];
    let mask = sliding_attacks::ROOK_MASKS[sq];
    let shift = sliding_attacks::ROOK_SHIFTS[sq];

    let index = compute_magic_index(occ, mask, magic, shift);

    sliding_attacks::ROOK_TABLES[sq][index]
}

pub fn rook_moves(sq: Sq, occ: u64, allies: u64) -> u64 {
    rook_attacks(sq, occ) & !allies
}

pub fn bishop_attacks(sq: Sq, occ: u64) -> u64 {
    let sq = sq.0 as usize;

    let magic = sliding_attacks::BISHOP_MAGICS[sq];
    let mask = sliding_attacks::BISHOP_MASKS[sq];
    let shift = sliding_attacks::BISHOP_SHIFTS[sq];

    let index = compute_magic_index(occ, mask, magic, shift);

    sliding_attacks::BISHOP_TABLES[sq][index]
}

pub fn bishop_moves(sq: Sq, occ: u64, allies: u64) -> u64 {
    bishop_attacks(sq, occ) & !allies
}

pub fn queen_attacks(sq: Sq, occ: u64) -> u64 {
    bishop_attacks(sq, occ) | rook_attacks(sq, occ)
}

pub fn queen_moves(sq: Sq, occ: u64, allies: u64) -> u64 {
    queen_attacks(sq, occ) & !allies
}
