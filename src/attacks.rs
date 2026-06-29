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
