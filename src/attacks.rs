use crate::*;

pub fn pawn_pushes(pawns: u64, occ: u64, side: Side) -> u64 {
    (pawns << (8 * side.sign())) & !occ
}

pub fn pawn_double_pushes(pawns: u64, occ: u64, side: Side) -> u64 {
    let single = pawn_pushes(pawns & MASK_RANK[side.pawn_rank()], occ, side);
    pawn_pushes(single, occ, side)
}

pub fn pawn_captures_left_white(pawns: u64, opp_occ: u64) -> (u64, i32) {
    ((pawns << 7) & opp_occ & !MASK_FILE_H, 7)
}

pub fn pawn_captures_right_white(pawns: u64, opp_occ: u64) -> (u64, i32) {
    ((pawns << 9) & opp_occ & !MASK_FILE_A, 9)
}

pub fn pawn_captures_left_black(pawns: u64, opp_occ: u64) -> (u64, i32) {
    ((pawns >> 9) & opp_occ & !MASK_FILE_H, -9)
}

pub fn pawn_captures_right_black(pawns: u64, opp_occ: u64) -> (u64, i32) {
    ((pawns >> 7) & opp_occ & !MASK_FILE_A, -7)
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

pub fn king_moves(sq: Sq, allies: u64) -> u64 {
    KING_ATTACKS[sq.0 as usize] & !allies
}

pub fn knight_moves(sq: Sq, allies: u64) -> u64 {
    KNIGHT_ATTACKS[sq.0 as usize] & !allies
}
