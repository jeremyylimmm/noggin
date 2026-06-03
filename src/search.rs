use crate::*;

pub struct Searcher {
}


impl Searcher {
    pub fn search(&self, pos: &mut Position, depth: i32, ply: i32) -> (i32, Move) {
        let side = pos.to_move;
        let in_check = pos.checked(side);

        if pos.halfmove_clock == 100 {
            return (0, NULL_MOVE);
        }

        if depth <= 0 {
            return (pos.relative_eval(), NULL_MOVE);
        }

        let moves = movegen::gen_pseudolegal_moves(pos);
        let mut move_index = 0;
        
        let mut best_score = std::i32::MIN;
        let mut best_move = NULL_MOVE;

        for pmi in 0..moves.len() {
            let mv = moves[pmi];

            pos.make_move(mv);

            if pos.checked(side) {
                pos.unmake_move();
                continue
            }

            let score = -self.search(pos, depth-1, ply+1).0;

            if score > best_score {
                best_score = score;
                best_move = mv;
            }

            pos.unmake_move();
            move_index += 1;
        }

        if move_index == 0 {
            if in_check {
                return (-MATE_SCORE + ply, NULL_MOVE);
            }
            else {
                return (0, NULL_MOVE);
            }
        }

        (best_score, best_move)
    }

    pub fn best(&self, pos: &mut Position, depth: i32) -> Move {
        self.search(pos, depth, 0).1
    }
}