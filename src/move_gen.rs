use crate::*;

use attacks::*;

const MASK_KING_CASTLE_PATH: u64 = MASK_FILE_F | MASK_FILE_G;
const MASK_QUEEN_CASTLE_PATH: u64 = MASK_FILE_C | MASK_FILE_D;

const MASK_KING_CASTLE_EMPTY: u64 = MASK_FILE_F | MASK_FILE_G;
const MASK_QUEEN_CASTLE_EMPTY: u64 = MASK_FILE_B | MASK_FILE_C | MASK_FILE_D;

pub fn gen_psuedolegal(pos: &Position) -> MoveList {
    let mut moves = MoveList::new();

    let occ = pos.occ();
    let allies = pos.side_occ(pos.stm);
    let opp = pos.side_occ(pos.stm.opp());

    let pawns = pos.bbs.get(Piece::Pawn, pos.stm);

    for to in iter_bb(pawn_pushes(pawns, occ, pos.stm)) {
        let from = Sq((to.0 as i32 - 8 * pos.stm.sign()) as _);

        if to.rank() == pos.stm.promotion_rank() {
            moves.push(Move::new(from, to, Some(Piece::Knight)));
            moves.push(Move::new(from, to, Some(Piece::Bishop)));
            moves.push(Move::new(from, to, Some(Piece::Rook)));
            moves.push(Move::new(from, to, Some(Piece::Queen)));
        } else {
            moves.push(Move::new(from, to, None));
        }
    }

    for to in iter_bb(pawn_double_pushes(pawns, occ, pos.stm)) {
        let from = Sq(to.0 ^ 0b010000);
        moves.push(Move::new(from, to, None));
    }

    let ep_mask = pos.ep.map_or(0, |x| x.bb());

    let left_captures = attacks::pawn_captures_left(pawns, opp | ep_mask, pos.stm);
    let right_captures = attacks::pawn_captures_right(pawns, opp | ep_mask, pos.stm);

    for (bb, offset) in [left_captures, right_captures] {
        for to in iter_bb(bb) {
            let from = Sq((to.0 as i32 - offset) as _);

            if to.rank() == pos.stm.promotion_rank() {
                moves.push(Move::new(from, to, Some(Piece::Knight)));
                moves.push(Move::new(from, to, Some(Piece::Bishop)));
                moves.push(Move::new(from, to, Some(Piece::Rook)));
                moves.push(Move::new(from, to, Some(Piece::Queen)));
            } else {
                moves.push(Move::new(from, to, None));
            }
        }
    }

    let knights = pos.bbs.get(Piece::Knight, pos.stm);

    for knight in iter_bb(knights) {
        for to in iter_bb(knight_moves(knight, allies)) {
            moves.push(Move::new(knight, to, None));
        }
    }

    let bishops = pos.bbs.get(Piece::Bishop, pos.stm);

    for bishop in iter_bb(bishops) {
        for to in iter_bb(bishop_moves(bishop, occ, allies)) {
            moves.push(Move::new(bishop, to, None));
        }
    }

    let rooks = pos.bbs.get(Piece::Rook, pos.stm);

    for rook in iter_bb(rooks) {
        for to in iter_bb(rook_moves(rook, occ, allies)) {
            moves.push(Move::new(rook, to, None));
        }
    }

    let queens = pos.bbs.get(Piece::Queen, pos.stm);

    for queen in iter_bb(queens) {
        for to in iter_bb(queen_moves(queen, occ, allies)) {
            moves.push(Move::new(queen, to, None));
        }
    }

    let kings = pos.bbs.get(Piece::King, pos.stm);

    for king in iter_bb(kings) {
        for to in iter_bb(king_moves(king, allies)) {
            moves.push(Move::new(king, to, None));
        }
    }

    if !pos.checked(pos.stm) {
        let king_sq = Sq(kings.trailing_zeros() as _);

        if pos.has_king_castle_rights(pos.stm)
            && occ & (MASK_KING_CASTLE_EMPTY & MASK_RANK[pos.stm.home_rank()]) == 0
            && (MASK_KING_CASTLE_PATH & MASK_RANK[pos.stm.home_rank()]) & pos.threats == 0
        {
            let to = Sq::from_coords(king_sq.rank(), 6);
            moves.push(Move::new(king_sq, to, None));
        }

        if pos.has_queen_castle_rights(pos.stm)
            && occ & (MASK_QUEEN_CASTLE_EMPTY & MASK_RANK[pos.stm.home_rank()]) == 0
            && (MASK_QUEEN_CASTLE_PATH & MASK_RANK[pos.stm.home_rank()]) & pos.threats == 0
        {
            let to = Sq::from_coords(king_sq.rank(), 2);
            moves.push(Move::new(king_sq, to, None));
        }
    }

    moves
}
