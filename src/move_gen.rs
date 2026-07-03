use crate::*;

use attacks::*;

const MASK_KING_CASTLE_PATH: u64 = MASK_FILE_F | MASK_FILE_G;
const MASK_QUEEN_CASTLE_PATH: u64 = MASK_FILE_C | MASK_FILE_D;

const MASK_KING_CASTLE_EMPTY: u64 = MASK_FILE_F | MASK_FILE_G;
const MASK_QUEEN_CASTLE_EMPTY: u64 = MASK_FILE_B | MASK_FILE_C | MASK_FILE_D;

fn gen_evasions(pos: &Position) -> MoveList {
    let mut moves = MoveList::new();

    let allies = pos.side_occ(pos.stm);

    let kings = pos.bbs.get(Piece::King, pos.stm);

    for king in iter_bb(kings) {
        for to in iter_bb(king_moves(king, allies) & !pos.threats) {
            moves.push(Move::new(king, to, None));
        }
    }

    moves
}

fn gen_standard(pos: &Position, checker: Option<Sq>) -> MoveList {
    let mut moves = MoveList::new();

    let king_sq = pos.king_sq(pos.stm);

    let occ = pos.occ();
    let allies = pos.side_occ(pos.stm);
    let opp = pos.side_occ(pos.stm.opp());

    let pawns = pos.bbs.get(Piece::Pawn, pos.stm);

    let legality_mask = if let Some(sq) = checker {
        sq.bb() | line_between_diagonal(king_sq, sq).0 | line_between_straight(king_sq, sq).0
    } else {
        u64::MAX
    };

    for to in iter_bb(pawn_pushes(pawns, occ, pos.stm)) {
        let from = Sq((to.0 as i32 - 8 * pos.stm.sign()) as _);

        if (pos.pin_ray(from) & legality_mask & to.bb()) == 0 {
            continue;
        }

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
        if (pos.pin_ray(from) & legality_mask & to.bb()) != 0 {
            moves.push(Move::new(from, to, None));
        }
    }

    if let Some(ep_sq) = pos.ep {
        let ep_mask = ep_sq.bb();

        let left_ep = attacks::pawn_captures_left(pawns, ep_mask, pos.stm);
        let right_ep = attacks::pawn_captures_right(pawns, ep_mask, pos.stm);

        for (bb, offset) in [left_ep, right_ep] {
            for to in iter_bb(bb) {
                let cap_sq = Sq((to.0 ^ 0b001000) as _);
                let from = Sq((to.0 as i32 - offset) as _);

                if pos.ep_legal(cap_sq, from, to) {
                    moves.push(Move::new(from, to, None));
                }
            }
        }
    }

    let left_captures = attacks::pawn_captures_left(pawns, opp, pos.stm);
    let right_captures = attacks::pawn_captures_right(pawns, opp, pos.stm);

    for (bb, offset) in [left_captures, right_captures] {
        for to in iter_bb(bb) {
            let from = Sq((to.0 as i32 - offset) as _);

            if (pos.pin_ray(from) & legality_mask & to.bb()) == 0 {
                continue;
            }

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
        let pin_ray = pos.pin_ray(knight);
        for to in iter_bb(knight_moves(knight, allies)) {
            if (pin_ray & legality_mask & to.bb()) != 0 {
                moves.push(Move::new(knight, to, None));
            }
        }
    }

    let bishops = pos.bbs.get(Piece::Bishop, pos.stm);

    for bishop in iter_bb(bishops) {
        let mask = pos.pin_ray(bishop) & legality_mask;
        for to in iter_bb(bishop_moves(bishop, occ, allies) & mask) {
            moves.push(Move::new(bishop, to, None));
        }
    }

    let rooks = pos.bbs.get(Piece::Rook, pos.stm);

    for rook in iter_bb(rooks) {
        let mask = pos.pin_ray(rook) & legality_mask;
        for to in iter_bb(rook_moves(rook, occ, allies) & mask) {
            moves.push(Move::new(rook, to, None));
        }
    }

    let queens = pos.bbs.get(Piece::Queen, pos.stm);

    for queen in iter_bb(queens) {
        let mask = pos.pin_ray(queen) & legality_mask;
        for to in iter_bb(queen_moves(queen, occ, allies) & mask) {
            moves.push(Move::new(queen, to, None));
        }
    }

    let kings = pos.bbs.get(Piece::King, pos.stm);

    for king in iter_bb(kings) {
        for to in iter_bb(king_moves(king, allies) & !pos.threats) {
            moves.push(Move::new(king, to, None));
        }
    }

    if checker.is_none() {
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

pub fn gen_legal(pos: &Position) -> MoveList {
    match pos.checked() {
        Check::None => gen_standard(pos, None),
        Check::Single(sq) => gen_standard(pos, Some(sq)),
        Check::Double => gen_evasions(pos),
    }
}
