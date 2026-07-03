use crate::*;

const MATERIAL_VALUE: [Score; 6] = [100, 300, 350, 500, 900, 0];

pub fn evaluate(pos: &Position) -> Score {
    let mut score = 0;

    for side in [Side::White, Side::Black] {
        for piece in [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
        ] {
            let n = pos.bbs.get(piece, side).count_ones() as i32;
            let v = n * MATERIAL_VALUE[piece.id()];
            score += v * side.sign();
        }
    }

    score
}
