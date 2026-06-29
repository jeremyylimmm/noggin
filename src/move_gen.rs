use crate::*;

use attacks::*;

pub fn gen_psuedolegal(pos: &Position) -> MoveList {
    let mut moves = MoveList::new();

    let occ = pos.occ();
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

    let (left_captures, right_captures) = match pos.stm {
        Side::White => (
            pawn_captures_left_white(pawns, opp | ep_mask),
            pawn_captures_right_white(pawns, opp | ep_mask),
        ),

        Side::Black => (
            pawn_captures_left_black(pawns, opp | ep_mask),
            pawn_captures_right_black(pawns, opp | ep_mask),
        ),
    };

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

    moves
}
