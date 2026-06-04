use crate::*;

use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}, time};

#[derive(Clone)]
pub struct Searcher {
    stop: Arc<AtomicBool>,
    exited: bool,

    time_limit_hard: f32,
    time_limit_soft: f32,

    node_limit_hard: usize,
    node_limit_soft: usize,

    nodes: usize,
    start_time: std::time::Instant
}


impl Searcher {
    pub fn new() -> Self {
        Self {
            stop: Arc::new(AtomicBool::new(false)),
            exited: false,
            time_limit_hard: f32::INFINITY,
            time_limit_soft: f32::INFINITY,
            node_limit_hard: 1024*1024*1024,
            node_limit_soft: 1024*1024*1024,
            nodes: 0,
            start_time: std::time::Instant::now()
        }
    }

    pub fn exit_on_node(&mut self) -> bool {
        if self.nodes >= self.node_limit_hard {
            self.exited = true;
        }

        if (self.nodes & 4095) == 0 {
            if self.stop.load(Ordering::Relaxed) {
                self.exited = true;
            }

            let elapsed = (std::time::Instant::now() - self.start_time).as_secs_f32();

            if elapsed >= self.time_limit_hard * 0.95 {
                self.exited = true;
            }
        }

        self.nodes += 1;

        self.exited
    }

    pub fn reset(&mut self, time_limit_hard: f32, time_limit_soft: f32, node_limit_hard: usize, node_limit_soft: usize) {
        self.stop.store(false, Ordering::Relaxed);
        self.exited = false;

        self.time_limit_hard = time_limit_hard;
        self.time_limit_soft = time_limit_soft;
        self.node_limit_hard = node_limit_hard;
        self.node_limit_soft = node_limit_soft;
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
        self.nodes = 0;
        self.start_time = std::time::Instant::now();

        let mut best_move = NULL_MOVE;

        for d in 1..=depth {
            let elapsed = (time::Instant::now() - self.start_time).as_secs_f32();

            if self.nodes >= self.node_limit_soft || elapsed >= self.time_limit_soft * 0.95 {
                break;
            }

            let mv = self.search(pos, d, 0).1;

            if self.exited {
                break;
            }

            best_move = mv;
        }
        best_move
    }
}