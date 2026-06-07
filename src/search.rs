use crate::*;
use crate::movegen::*;

use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

const MAX_DEPTH: usize = 128;

#[derive(Copy, Clone, PartialEq)]
enum TTKind {
    Exact,
    Upper,
    Lower
}

#[derive(Clone)]
struct TTEntry {
    hash: u64,
    mv: Move,
    rel_score: i16,
    kind: TTKind,
    depth: i32,
}

const HASH_MOVE_SCORE:        i32 = 5_000_000;
const PROMOTION_MOVE_SCORE:   i32 = 4_000_000;
const CAPTURE_MOVE_SCORE:     i32 = 3_000_000;
const KILLER_MOVE_SCORE:      i32 = 2_000_000;
const QUIET_MOVE_SCORE:       i32 = 1_000_000;

const MAX_HISTORY: i16 = 30_000;

impl TTEntry {
    fn empty() -> Self {
        Self {
            hash: 0,
            mv: NULL_MOVE,
            kind: TTKind::Exact,
            rel_score: 0,
            depth: 0
        }
    }

    fn score(&self, ply: i32) -> i32 {
        if self.rel_score.abs() as i32 > MATE_SCORE - 1000 {
            if self.rel_score > 0 {
                self.rel_score as i32 - ply
            }
            else {
                self.rel_score as i32 + ply
            }
        }
        else {
            self.rel_score as i32
        }
    }

    fn new(hash: u64, mv: Move, kind: TTKind, score: i32, ply: i32, depth: i32) -> Self {
        let rel_score = if score.abs() > MATE_SCORE - 1000 {
            if score > 0 {
                score + ply
            }
            else {
                score - ply
            }
        }
        else {
            score
        } as i16;


        Self {
            hash,
            mv,
            kind,
            rel_score,
            depth
        }
    }

    fn cutoff(&self, ply: i32, alpha: i32, beta: i32) -> Option<(i32, Move)> {
        let score = self.score(ply);

        match self.kind {
            TTKind::Exact => {
                if score > alpha && score < beta {
                    return Some((score, self.mv));
                }
            }

            TTKind::Lower => {
                if score >= beta {
                    return Some((score, NULL_MOVE));
                }
            }

            TTKind::Upper => {
                if score <= alpha {
                    return Some((score, NULL_MOVE));
                }
            }
        }

        return None;
    }
}

#[derive(Clone)]
pub struct Searcher {
    pub stop: Arc<AtomicBool>,
    exited: bool,

    tt: Vec<TTEntry>,
    history: Box<[[[i16; 64]; 64];2]>,
    killers: Box<[[Move; 2]; MAX_DEPTH]>,

    enable_uci: bool,

    time_limit_hard: f32,
    time_limit_soft: f32,

    node_limit_hard: usize,
    node_limit_soft: usize,

    nodes: usize,
    qnodes: usize,
    tt_attempts: usize,
    tt_hits: usize,
    tt_collisions: usize,
    start_time: std::time::Instant
}

struct MovePicker {
    moves: MoveList,
    scores: [i32; 256],
    next: usize
}

impl MovePicker {
    fn new(searcher: &Searcher, pos: &Position, moves: MoveList, hash_move: Move, ply: i32) -> Self {
        let mut scores = [0;_];

        for i in 0..moves.len() {
            scores[i] = Self::score_move(searcher, pos, moves[i], hash_move, ply);
        }

        Self {
            moves,
            scores,
            next: 0
        }
    }

    fn score_move(searcher: &Searcher, pos: &Position, mv: Move, hash_move: Move, ply: i32) -> i32 {
        let ply = ply as usize;

        if mv == hash_move {
            HASH_MOVE_SCORE
        }
        else if mv.promotion() != Piece::None {
            PROMOTION_MOVE_SCORE + mv.promotion().centipawn_value()
        }
        else if let Some(capture_piece) = pos.is_capture(mv) {
            let piece = pos.board[mv.from()];
            CAPTURE_MOVE_SCORE + capture_piece.centipawn_value()*100 - piece.centipawn_value()
        }
        else if mv == searcher.killers[ply][0] || mv == searcher.killers[ply][1] {
            KILLER_MOVE_SCORE
        }
        else {
            QUIET_MOVE_SCORE + searcher.history[mv.side().id()][mv.from()][mv.to()] as i32
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
            history: Box::new([[[0; 64]; 64]; 2]),
            killers: Box::new([[NULL_MOVE; 2]; MAX_DEPTH]),

            enable_uci: true,

            time_limit_hard: f32::INFINITY,
            time_limit_soft: f32::INFINITY,
            node_limit_hard: 1024*1024*1024,
            node_limit_soft: 1024*1024*1024,

            nodes: 0,
            qnodes: 0,
            tt_attempts: 0,
            tt_hits: 0,
            tt_collisions: 0,
            start_time: std::time::Instant::now()
        }
    }

    pub fn tt_hit_rate(&self) -> f32 {
        self.tt_hits as f32 / self.tt_attempts as f32
    }

    pub fn tt_collision_rate(&self) -> f32 {
        self.tt_collisions as f32 / self.tt_attempts as f32
    }

    pub fn tt_fill(&self) -> f32 {
        self.tt.iter().map(|x|if x.hash != 0 {1} else {0}).sum::<i32>() as f32 / self.tt.len() as f32
    }

    pub fn disable_uci(&mut self) {
        self.enable_uci = false;
    }

    fn update_history(&mut self, mv: Move, bonus: i16) {
        let clamped = bonus.clamp(-MAX_HISTORY, MAX_HISTORY);
        let x = &mut self.history[mv.side().id()][mv.from()][mv.to()];
        *x += clamped - *x * clamped.abs() / MAX_HISTORY;
    }

    fn tt_query<const METRICS: bool>(&mut self, hash: u64) -> Option<TTEntry> {
        let index = (hash & TT_MASK) as usize;

        if METRICS {
            self.tt_attempts += 1;
        }

        if self.tt[index].hash == hash {
            if METRICS {
                self.tt_hits += 1;
            }
            Some(self.tt[index].clone())
        }
        else {
            if self.tt[index].hash != 0 && METRICS {
                self.tt_collisions += 1;
            }
            None
        }
    }

    fn add_killer(&mut self, mv: Move, ply: i32) {
        let ply = ply as usize;

        if mv != self.killers[ply][0] && mv != self.killers[ply][1] {
            self.killers[ply][1] = self.killers[ply][0];
            self.killers[ply][0] = mv;
        }
    }

    fn tt_set(&mut self, hash: u64, mv: Move, kind: TTKind, score: i32, ply: i32, depth: i32) {
        let index = (hash & TT_MASK) as usize;

        self.tt[index] = TTEntry::new(hash, mv, kind, score, ply, depth);
    }

    pub fn nodes(&self) -> usize {
        self.nodes
    }

    pub fn qnodes(&self) -> usize {
        self.qnodes
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
        self.qnodes = 0;
        self.tt_attempts = 0;
        self.tt_hits = 0;
        self.tt_collisions = 0;
        self.start_time = std::time::Instant::now();
    }

    pub fn qsearch(&mut self, pos: &mut Position, ply: i32, mut alpha: i32, beta: i32) -> i32 {
        if self.exit_on_node() {
            return 0;
        }

        self.qnodes += 1;

        let alpha0 = alpha;

        let side = pos.to_move;
        let in_check = pos.checked(side);
        
        let pv_node = beta > alpha + 1;

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


        let hash_move = if let Some(entry) = self.tt_query::<false>(pos.hash) {
            if !pv_node {
                if let Some((score, _)) = entry.cutoff(ply, alpha, beta) {
                    return score;
                }
            }

            entry.mv
        }
        else {
            NULL_MOVE
        };


        let mut move_picker = MovePicker::new(self, pos, moves, hash_move, ply);

        let mut move_index = 0;
        let mut best_move = NULL_MOVE;

        while let Some(mv) = move_picker.next() {
            let quiet = pos.is_capture(mv).is_none() && mv.promotion() == Piece::None;

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

                if quiet {
                    self.add_killer(mv, ply);
                }

                self.tt_set(pos.hash, mv, TTKind::Lower, best_score, ply, 0);

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

        self.tt_set(pos.hash, best_move, if best_score > alpha0 {TTKind::Exact} else {TTKind::Upper}, best_score, ply, 0);

        best_score
    }


    pub fn search(&mut self, pos: &mut Position, depth: i32, ply: i32, mut alpha: i32, beta: i32) -> (i32, Move) {
        if self.exit_on_node() {
            return (0, NULL_MOVE);
        }

        let alpha0 = alpha;

        let side = pos.to_move;
        let in_check = pos.checked(side);

        let pv_node = beta > alpha + 1;

        if pos.is_threefold_repetition() {
            return (0, NULL_MOVE);
        }

        if depth <= 0 {
            return (self.qsearch(pos, ply, alpha, beta), NULL_MOVE);
        }

        let hash_move = if let Some(entry) = self.tt_query::<true>(pos.hash) {
            if entry.depth >= depth && !pv_node {
                if let Some((score, mv)) = entry.cutoff(ply, alpha, beta) {
                    return (score, mv);
                }
            }

            entry.mv
        }
        else {
            NULL_MOVE
        };

        let eval = pos.relative_eval();

        // reverse futility pruning

        let can_rfp = !pv_node && !in_check && beta.abs() < MATE_SCORE - 1000;

        if can_rfp {
            let rfp_margin = 150 * depth;

            if eval >= beta + rfp_margin {
                return (eval, NULL_MOVE);
            }
        }
        


        // null move pruning

        let can_nmp = !in_check && !pv_node && !pos.only_pawns(side) && depth > 3;
        
        if can_nmp {
            let r = 2 + depth / 6;

            pos.make_null_move();
            let v = -self.search(pos, depth-1-r, ply+1, -beta, -(beta-1)).0;
            pos.unmake_null_move();

            if v >= beta {
                return (v, NULL_MOVE);
            }
        }


        let moves = movegen::gen_pseudolegal_moves(pos);
        let mut move_picker = MovePicker::new(self, pos, moves, hash_move, ply);

        let mut move_index = 0;
        
        let mut best_score = std::i32::MIN;
        let mut best_move = NULL_MOVE;

        let mut quiets = MoveList::new();

        while let Some(mv) = move_picker.next() {
            let quiet = pos.is_capture(mv).is_none() && mv.promotion() == Piece::None;

            pos.make_move(mv);

            if pos.checked(side) {
                pos.unmake_move();
                continue
            }

            // futility pruning

            let fp_margin = eval + 200 * depth;

            if depth < 6 && !in_check && quiet && fp_margin < alpha && alpha.abs() < MATE_SCORE - 1000 && best_score > -MATE_SCORE + 1000 {
                pos.unmake_move();
                continue;
            }

            // late move reduction

            let mut lmr = 0;
            let can_lmr = move_index > 2 && depth > 2;

            if can_lmr {
                let frac = 0.2 + (depth as f32).ln() * (move_index as f32).ln() / 3.35;
                lmr = (frac.round() as i32).max(0);
            }

            // principal variation search

            let mut score = -INF_SCORE;

            if !pv_node || (move_index > 0) {
                score = -self.search(pos, depth-1-lmr, ply+1, -(alpha+1), -alpha).0;

                if lmr > 0 && score > alpha {
                    score = -self.search(pos, depth-1, ply+1, -(alpha+1), -alpha).0;
                }
            }

            if pv_node && (move_index == 0 || score > alpha) {
                score = -self.search(pos, depth-1, ply+1, -beta, -alpha).0;
            }






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

                self.tt_set(pos.hash, mv, TTKind::Lower, best_score, ply, depth);

                if quiet {
                    self.add_killer(mv, ply);

                    let hist_bonus = 300 * depth - 250;
                    self.update_history(mv, hist_bonus as i16);

                    for q in quiets.iter() {
                        self.update_history(*q, -hist_bonus as i16); 
                    }
                }

                return (best_score, best_move);
            }

            pos.unmake_move();

            if quiet {
                quiets.push(mv);
            }

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

        self.tt_set(pos.hash, best_move, if best_score > alpha0 {TTKind::Exact} else {TTKind::Upper}, best_score, ply, depth);

        (best_score, best_move)
    }

    pub fn best(&mut self, pos: &mut Position, depth: i32) -> Move {
        let mut best_move = NULL_MOVE;

        let mut window_centre = 0i32;

        for d in 1..=depth {
            if self.nodes >= self.node_limit_soft || self.elapsed() >= self.time_limit_soft * 0.95 {
                break;
            }

            let mut window_lo = 25;
            let mut window_hi = 25;

            let (score, mv) = loop {
                let (alpha, beta) = if d < 4 {
                    (-INF_SCORE, INF_SCORE)
                }
                else {
                    (
                        (window_centre - window_lo).clamp(-INF_SCORE, INF_SCORE),
                        (window_centre + window_hi).clamp(-INF_SCORE, INF_SCORE)
                    )
                };

                let (score, mv) = self.search(pos, d, 0, alpha, beta);

                if (score > alpha && score < beta) || self.exited {
                    break (score, mv);
                }
                else if score <= alpha {
                    window_lo *= 2;
                }
                else {
                    window_hi *= 2;
                }
            };

            if self.exited {
                break;
            }

            window_centre = score;

            let score_str = if score.abs() > MATE_SCORE - 1000 {
                format!("mate {}{}", if score < 0 {"-"} else {""}, MATE_SCORE - score.abs())
            }
            else {
                format!("cp {}", score)
            };

            let nps = (self.nodes as f32 / self.elapsed()).round() as i32;
            let time = (self.elapsed() * 1000.0).round() as i32;

            best_move = mv;

            if self.enable_uci {
                println!("info depth {} score {} nodes {} nps {} time {} pv {}", d, score_str, self.nodes, nps, time, best_move.uci_string());
            }
        }
        best_move
    }
}