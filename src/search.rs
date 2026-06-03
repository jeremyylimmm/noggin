use crate::*;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[derive(Clone)]
pub struct Searcher {
    stop: Arc<AtomicBool>,
    exited: bool
}


impl Searcher {
    pub fn new() -> Self {
        Self {
            stop: Arc::new(AtomicBool::new(false)),
            exited: false
        }
    }

    pub fn exit_on_node(&mut self) -> bool {
        let exit = self.stop.load(Ordering::Relaxed);

        if exit {
            self.exited = true;
        }

        exit
    }

    pub fn reset(&mut self) {
        self.stop.store(false, Ordering::Relaxed);
        self.exited = false;
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    pub fn search(&mut self, pos: &mut Position, depth: i32, ply: i32) -> (i32, Move) {
        if self.exit_on_node() {
            return (0, NULL_MOVE);
        }

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

            if self.exited {
                pos.unmake_move();
                return (0, NULL_MOVE);
            }

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

    pub fn best(&mut self, pos: &mut Position, depth: i32) -> Move {
        let mut best_move = NULL_MOVE;

        for d in 1..=depth {
            let mv = self.search(pos, depth, 0).1;

            if self.exited {
                break;
            }

            best_move = mv;
        }
        best_move
    }
}