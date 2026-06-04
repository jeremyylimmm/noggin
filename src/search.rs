use crate::*;
use crate::movegen::*;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[derive(Clone)]
struct TTEntry {
    hash: u64,
    mv: Move
}

const HASH_MOVE_SCORE:        i32 = 3_000_000;
const CAPTURE_MOVE_SCORE:     i32 = 2_000_000;
const NON_CAPTURE_MOVE_SCORE: i32 = 1_000_000;

impl TTEntry {
    fn empty() -> Self {
        Self {
            hash: 0,
            mv: NULL_MOVE
        }
    }

    fn new(hash: u64, mv: Move) -> Self {
        Self {
            hash,
            mv
        }
    }
}

#[derive(Clone)]
pub struct Searcher {
    pub stop: Arc<AtomicBool>,
    exited: bool,

    tt: Vec<TTEntry>,

    time_limit_hard: f32,
    time_limit_soft: f32,

    node_limit_hard: usize,
    node_limit_soft: usize,

    nodes: usize,
    start_time: std::time::Instant
}

struct MovePicker {
    moves: MoveList,
    scores: [i32; 256],
    next: usize
}

impl MovePicker {
    fn new(pos: &Position, moves: MoveList, hash_move: Move) -> Self {
        let mut scores = [0;_];

        for i in 0..moves.len() {
            scores[i] = Self::score_move(pos, moves[i], hash_move);
        }

        Self {
            moves,
            scores,
            next: 0
        }
    }

    fn score_move(pos: &Position, mv: Move, hash_move: Move) -> i32 {
        if mv == hash_move {
            HASH_MOVE_SCORE
        }
        else if let Some(capture_piece) = pos.is_capture(mv) {
            let piece = pos.board[mv.from()];
            CAPTURE_MOVE_SCORE + capture_piece.centipawn_value()*100 - piece.centipawn_value()
        }
        else {
            NON_CAPTURE_MOVE_SCORE
        }
    }

    fn next(&mut self) -> Option<Move> {
        if self.next >= self.moves.len() {
            return None;
        }

        let mut best_index = self.next;
        let mut best_score = self.scores[self.next];

        for i in (self.next+1)..self.moves.len() {
            if self.scores[i] > best_score {
                best_index = i;
                best_score = self.scores[i];
            }
        }

        self.moves.swap(self.next, best_index);
        self.scores.swap(self.next, best_index);

        let mv = self.moves[self.next];
        self.next += 1;

        Some(mv)
    }
}

const TT_SIZE: usize = 1 << 22;
const TT_MASK: u64 = (TT_SIZE - 1) as u64;

impl Searcher {
    pub fn new() -> Self {
        Self {
            stop: Arc::new(AtomicBool::new(false)),
            exited: false,

            tt: vec![TTEntry::empty(); 1<<22],

            time_limit_hard: f32::INFINITY,
            time_limit_soft: f32::INFINITY,
            node_limit_hard: 1024*1024*1024,
            node_limit_soft: 1024*1024*1024,

            nodes: 0,
            start_time: std::time::Instant::now()
        }
    }

    fn tt_query(&self, hash: u64) -> Option<TTEntry> {
        let index = (hash & TT_MASK) as usize;

        if self.tt[index].hash == hash {
            Some(self.tt[index].clone())
        }
        else {
            None
        }
    }

    fn tt_set(&mut self, hash: u64, mv: Move) {
        let index = (hash & TT_MASK) as usize;
        self.tt[index] = TTEntry::new(hash, mv);
    }

    pub fn nodes(&self) -> usize {
        self.nodes
    }

    pub fn elapsed(&self) -> f32 {
        (std::time::Instant::now() - self.start_time).as_secs_f32()
    }

    pub fn exit_on_node(&mut self) -> bool {
        if self.nodes >= self.node_limit_hard {
            self.exited = true;
        }

        if (self.nodes & 4095) == 0 {
            if self.stop.load(Ordering::Relaxed) {
                self.exited = true;
            }

            if self.elapsed() >= self.time_limit_hard * 0.95 {
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

        self.nodes = 0;
        self.start_time = std::time::Instant::now();
    }

    pub fn qsearch(&mut self, pos: &mut Position, ply: i32, mut alpha: i32, beta: i32) -> i32 {
        if self.exit_on_node() {
            return 0;
        }

        let side = pos.to_move;
        let in_check = pos.checked(side);
        

        if pos.is_threefold_repetition() {
            return 0;
        }


        let (mut best_score, moves) = if in_check {
            (-INF_SCORE, movegen::gen_pseudolegal_moves(pos))
        } else {
            let stand_pat = pos.relative_eval();

            if stand_pat > alpha {
                alpha = stand_pat;
            }

            if alpha >= beta {
                return stand_pat;
            }

            (stand_pat, movegen::gen_pseudolegal_captures(pos))
        };


        let hash_move = if let Some(entry) = self.tt_query(pos.hash) {
            entry.mv
        }
        else {
            NULL_MOVE
        };


        let mut move_picker = MovePicker::new(pos, moves, hash_move);

        let mut move_index = 0;
        let mut best_move = NULL_MOVE;

        while let Some(mv) = move_picker.next() {
            pos.make_move(mv);

            if pos.checked(side) {
                pos.unmake_move();
                continue
            }

            let score = -self.qsearch(pos, ply+1, -beta, -alpha);

            if self.exited {
                pos.unmake_move();
                return 0;
            }

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                alpha = score;
                best_move = mv;
            }

            if alpha >= beta {
                pos.unmake_move();

                self.tt_set(pos.hash, mv);

                return best_score;
            }

            pos.unmake_move();
            move_index += 1;
        }

        if move_index == 0 && in_check {
            return -MATE_SCORE + ply;
        }

        if pos.halfmove_clock == 100 {
            return 0;
        }

        self.tt_set(pos.hash, best_move);

        best_score
    }


    pub fn search(&mut self, pos: &mut Position, depth: i32, ply: i32, mut alpha: i32, beta: i32) -> (i32, Move) {
        if self.exit_on_node() {
            return (0, NULL_MOVE);
        }

        let side = pos.to_move;
        let in_check = pos.checked(side);

        if pos.is_threefold_repetition() {
            return (0, NULL_MOVE);
        }

        if depth <= 0 {
            return (self.qsearch(pos, ply, alpha, beta), NULL_MOVE);
        }

        let hash_move = if let Some(entry) = self.tt_query(pos.hash) {
            entry.mv
        }
        else {
            NULL_MOVE
        };

        let moves = movegen::gen_pseudolegal_moves(pos);
        let mut move_picker = MovePicker::new(pos, moves, hash_move);

        let mut move_index = 0;
        
        let mut best_score = std::i32::MIN;
        let mut best_move = NULL_MOVE;

        while let Some(mv) = move_picker.next() {
            pos.make_move(mv);

            if pos.checked(side) {
                pos.unmake_move();
                continue
            }

            let score = -self.search(pos, depth-1, ply+1, -beta, -alpha).0;

            if self.exited {
                pos.unmake_move();
                return (0, NULL_MOVE);
            }

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                alpha = score;
                best_move = mv;
            }

            if alpha >= beta {
                pos.unmake_move();

                self.tt_set(pos.hash, mv);

                return (best_score, best_move);
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

        if pos.halfmove_clock == 100 {
            return (0, NULL_MOVE);
        }

        self.tt_set(pos.hash, best_move);

        (best_score, best_move)
    }

    pub fn best(&mut self, pos: &mut Position, depth: i32) -> Move {
        let mut best_move = NULL_MOVE;

        for d in 1..=depth {
            if self.nodes >= self.node_limit_soft || self.elapsed() >= self.time_limit_soft * 0.95 {
                break;
            }

            let (score, mv) = self.search(pos, d, 0, -INF_SCORE, INF_SCORE);

            if self.exited {
                break;
            }

            let score_str = if score.abs() > MATE_SCORE - 1000 {
                format!("mate {}{}", if score < 0 {"-"} else {""}, MATE_SCORE - score.abs())
            }
            else {
                format!("cp {}", score)
            };

            let nps = (self.nodes as f32 / self.elapsed()).round() as i32;
            let time = (self.elapsed() * 1000.0).round() as i32;

            println!("info depth {} score {} nodes {} nps {} time {} pv {}", d, score_str, self.nodes, nps, time, best_move.uci_string());

            best_move = mv;
        }
        best_move
    }
}