use crate::movegen::*;
use crate::*;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

const MAX_DEPTH: usize = 128;
const MAX_PV_SIZE: usize = 32;

#[derive(Copy, Clone, PartialEq)]
enum TTKind {
    Exact,
    Upper,
    Lower,
}

#[derive(Clone)]
struct TTEntry {
    hash: u64,
    mv: Move,
    rel_score: i16,
    kind: TTKind,
    depth: i32,
}

const CONT_HISTORY_PLIES: [usize; 1] = [1];

const HASH_MOVE_SCORE: i32 = 6_000_000;
const PROMOTION_MOVE_SCORE: i32 = 5_000_000;
const GOOD_CAPTURE_MOVE_SCORE: i32 = 4_000_000;
const KILLER_MOVE_SCORE: i32 = 3_000_000;
const QUIET_MOVE_SCORE: i32 = 2_000_000;
const BAD_CAPTURE_MOVE_SCORE: i32 = 1_000_000;

const MAX_HISTORY: i16 = 30_000;

impl TTEntry {
    fn empty() -> Self {
        Self {
            hash: 0,
            mv: NULL_MOVE,
            kind: TTKind::Exact,
            rel_score: 0,
            depth: 0,
        }
    }

    fn score(&self, ply: usize) -> i32 {
        let ply = ply as i32;

        if (self.rel_score as i32).is_mate() {
            if self.rel_score > 0 {
                self.rel_score as i32 - ply
            } else {
                self.rel_score as i32 + ply
            }
        } else {
            self.rel_score as i32
        }
    }

    fn new(hash: u64, mv: Move, kind: TTKind, score: i32, ply: usize, depth: i32) -> Self {
        let ply = ply as i32;

        let rel_score = if score.is_mate() {
            if score > 0 { score + ply } else { score - ply }
        } else {
            score
        } as i16;

        Self {
            hash,
            mv,
            kind,
            rel_score,
            depth,
        }
    }

    fn cutoff(&self, ply: usize, alpha: i32, beta: i32) -> Option<(i32, Move)> {
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

struct SearchEntry {
    cont: Option<usize>,
    eval: i32,
    mv: Move,
}

struct ContinuationTable {
    data: [[[i16; 64]; 6]; 2],
}

type Line = [Move; MAX_PV_SIZE];

pub struct Searcher {
    pub stop: Arc<AtomicBool>,
    exited: bool,

    tt: Vec<TTEntry>,
    history: Box<[[[i16; 64]; 64]; 2]>,
    killers: Box<[[Move; 2]; MAX_DEPTH]>,
    ss: Vec<SearchEntry>,
    cont_hist: Box<[ContinuationTable; 64 * 6 * 2]>,
    pv_table: [Line; MAX_PV_SIZE],

    enable_uci: bool,

    time_limit_hard: f32,
    time_limit_soft: f32,

    node_limit_hard: usize,
    node_limit_soft: usize,

    nodes: usize,
    qnodes: usize,
    sel_depth: usize,
    tt_attempts: usize,
    tt_hits: usize,
    tt_collisions: usize,
    start_time: std::time::Instant,
}

struct MovePicker {
    moves: MoveList,
    scores: [i32; 256],
    next: usize,
}

impl MovePicker {
    fn new(
        searcher: &Searcher,
        pos: &Position,
        moves: MoveList,
        hash_move: Move,
        ply: usize,
    ) -> Self {
        let mut scores = [0; _];

        for i in 0..moves.len() {
            scores[i] = Self::score_move(searcher, pos, moves[i], hash_move, ply);
        }

        Self {
            moves,
            scores,
            next: 0,
        }
    }

    fn score_move(
        searcher: &Searcher,
        pos: &Position,
        mv: Move,
        hash_move: Move,
        ply: usize,
    ) -> i32 {
        let piece = pos.board[mv.from()];

        if mv == hash_move {
            HASH_MOVE_SCORE
        } else if mv.promotion() != Piece::None {
            PROMOTION_MOVE_SCORE + mv.promotion().centipawn_value()
        } else if let Some(capture_piece) = pos.is_capture(mv) {
            let base = if see_capture(pos, mv) < 0 {
                BAD_CAPTURE_MOVE_SCORE
            } else {
                GOOD_CAPTURE_MOVE_SCORE
            };
            base + capture_piece.centipawn_value() * 100 - piece.centipawn_value()
        } else if mv == searcher.killers[ply][0] || mv == searcher.killers[ply][1] {
            KILLER_MOVE_SCORE
        } else {
            let mut value =
                QUIET_MOVE_SCORE + searcher.history[mv.side().id()][mv.from()][mv.to()] as i32;

            for i in CONT_HISTORY_PLIES {
                if ply < i {
                    continue;
                }

                let cont = searcher.ss[ply - i].cont;
                if cont.is_none() {
                    continue;
                }
                let cont = cont.unwrap();

                value += searcher.cont_hist[cont].data[mv.side().id()][piece.id().unwrap()][mv.to()]
                    as i32;
            }

            value
        }
    }

    fn next(&mut self) -> Option<Move> {
        if self.next >= self.moves.len() {
            return None;
        }

        let mut best_index = self.next;
        let mut best_score = self.scores[self.next];

        for i in (self.next + 1)..self.moves.len() {
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

            tt: vec![TTEntry::empty(); 1 << 22],
            history: Box::new([[[0; 64]; 64]; 2]),
            killers: Box::new([[NULL_MOVE; 2]; MAX_DEPTH]),
            ss: vec![],
            cont_hist: Box::new(std::array::from_fn(|_| ContinuationTable::new())),
            pv_table: [[NULL_MOVE;_];_],

            enable_uci: true,

            time_limit_hard: f32::INFINITY,
            time_limit_soft: f32::INFINITY,
            node_limit_hard: 1024 * 1024 * 1024,
            node_limit_soft: 1024 * 1024 * 1024,

            nodes: 0,
            qnodes: 0,
            sel_depth: 0,
            tt_attempts: 0,
            tt_hits: 0,
            tt_collisions: 0,
            start_time: std::time::Instant::now(),
        }
    }

    pub fn tt_hit_rate(&self) -> f32 {
        self.tt_hits as f32 / self.tt_attempts as f32
    }

    pub fn tt_collision_rate(&self) -> f32 {
        self.tt_collisions as f32 / self.tt_attempts as f32
    }

    pub fn tt_fill(&self) -> f32 {
        self.tt
            .iter()
            .map(|x| if x.hash != 0 { 1 } else { 0 })
            .sum::<i32>() as f32
            / self.tt.len() as f32
    }

    pub fn disable_uci(&mut self) {
        self.enable_uci = false;
    }

    fn update_history(&mut self, mv: Move, bonus: i16) {
        let clamped = bonus.clamp(-MAX_HISTORY, MAX_HISTORY);
        let x = &mut self.history[mv.side().id()][mv.from()][mv.to()];
        *x += clamped - ((*x as i32 * clamped.abs() as i32) / MAX_HISTORY as i32) as i16;
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
        } else {
            if self.tt[index].hash != 0 && METRICS {
                self.tt_collisions += 1;
            }
            None
        }
    }

    fn add_killer(&mut self, mv: Move, ply: usize) {
        if mv != self.killers[ply][0] && mv != self.killers[ply][1] {
            self.killers[ply][1] = self.killers[ply][0];
            self.killers[ply][0] = mv;
        }
    }

    fn tt_set(&mut self, hash: u64, mv: Move, kind: TTKind, score: i32, ply: usize, depth: i32) {
        let index = (hash & TT_MASK) as usize;

        self.tt[index] = TTEntry::new(hash, mv, kind, score, ply, depth);
    }

    pub fn nodes(&self) -> usize {
        self.nodes
    }

    pub fn qnodes(&self) -> usize {
        self.qnodes
    }

    pub fn sel_depth(&self) -> usize {
        self.sel_depth
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

    fn reset(
        &mut self,
        time_limit_hard: f32,
        time_limit_soft: f32,
        node_limit_hard: usize,
        node_limit_soft: usize,
    ) {
        self.stop.store(false, Ordering::Relaxed);
        self.exited = false;

        self.time_limit_hard = time_limit_hard;
        self.time_limit_soft = time_limit_soft;
        self.node_limit_hard = node_limit_hard;
        self.node_limit_soft = node_limit_soft;

        self.pv_table = [[NULL_MOVE;_];_];

        self.nodes = 0;
        self.qnodes = 0;
        self.sel_depth = 0;
        self.tt_attempts = 0;
        self.tt_hits = 0;
        self.tt_collisions = 0;
        self.start_time = std::time::Instant::now();
    }

    pub fn qsearch(&mut self, pos: &mut Position, mut alpha: i32, beta: i32) -> i32 {
        if self.exit_on_node() {
            return 0;
        }

        let ply = self.ss.len();

        self.qnodes += 1;

        let alpha0 = alpha;

        let side = pos.to_move;
        let in_check = pos.checked(side);

        let pv_node = beta > alpha + 1;

        if pos.is_repetition(ply) {
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
        } else {
            NULL_MOVE
        };

        let mut move_picker = MovePicker::new(self, pos, moves, hash_move, ply);

        let mut move_index = 0;
        let mut best_move = NULL_MOVE;

        while let Some(mv) = move_picker.next() {
            let quiet = pos.is_capture(mv).is_none() && mv.promotion() == Piece::None;

            if !pv_node && !in_check && !quiet && see_capture(pos, mv) < 0 {
                continue;
            }

            self.push_move(pos, mv);

            if pos.checked(side) {
                self.pop_move(pos);
                continue;
            }

            let score = -self.qsearch(pos, -beta, -alpha);

            if self.exited {
                self.pop_move(pos);
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
                self.pop_move(pos);

                if quiet {
                    self.add_killer(mv, ply);
                }

                self.tt_set(pos.hash, mv, TTKind::Lower, best_score, ply, 0);

                return best_score;
            }

            self.pop_move(pos);
            move_index += 1;
        }

        if move_index == 0 && in_check {
            return -MATE_SCORE + ply as i32;
        }

        if pos.halfmove_clock >= 100 {
            return 0;
        }

        self.tt_set(
            pos.hash,
            best_move,
            if best_score > alpha0 {
                TTKind::Exact
            } else {
                TTKind::Upper
            },
            best_score,
            ply,
            0,
        );

        best_score
    }

    pub fn search(
        &mut self,
        pos: &mut Position,
        depth: i32,
        mut alpha: i32,
        beta: i32,
        exclude: Option<Move>,
    ) -> (i32, Move) {
        let pv_node = beta > alpha + 1;
        let ply = self.ss.len();

        if pv_node && ply < self.pv_table.len() {
            self.pv_table[ply][0] = NULL_MOVE;
        }

        if depth <= 0 {
            return (self.qsearch(pos, alpha, beta), NULL_MOVE);
        }

        if self.exit_on_node() {
            return (0, NULL_MOVE);
        }

        self.sel_depth = ply.max(self.sel_depth);

        let alpha0 = alpha;

        let side = pos.to_move;
        let in_check = pos.checked(side);

        if pos.is_repetition(ply) {
            return (0, NULL_MOVE);
        }

        let (hash_move, singular_beta) = if let Some(entry) = self.tt_query::<true>(pos.hash) {
            if entry.depth >= depth && !pv_node {
                if let Some((score, mv)) = entry.cutoff(ply, alpha, beta) {
                    return (score, mv);
                }
            }

            let can_se = depth > 6 &&
                               exclude.is_none() &&
                               entry.depth >= depth - 3 &&
                               entry.kind != TTKind::Upper &&
                               !entry.score(ply).is_mate();

            let singular_beta = entry.score(ply) - depth;

            (
                entry.mv,
                if can_se { Some(singular_beta) } else { None } 
            )
        } else {
            (NULL_MOVE, None)
        };

        let eval = pos.relative_eval();

        // improving heuristic

        let improving = !in_check && ply >= 2 && eval > self.ss[ply - 2].eval;

        // reverse futility pruning

        let can_rfp = !pv_node && !in_check && !beta.is_mate();

        if can_rfp {
            let rfp_margin = 150 * (depth - improving as i32);

            if eval >= beta + rfp_margin {
                return (eval, NULL_MOVE);
            }
        }

        // null move pruning

        let can_nmp = !in_check && !pv_node && !pos.only_pawns(side) && depth > 3;

        if can_nmp {
            let r = 2 + depth / 6;

            self.push_move(pos, NULL_MOVE);
            let v = -self.search(pos, depth - 1 - r, -beta, -(beta - 1), None).0;
            self.pop_move(pos);

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
            if let Some(excl) = exclude {
                if mv == excl {
                    move_index += 1;
                    continue;
                }
            }

            let quiet = pos.is_capture(mv).is_none() && mv.promotion() == Piece::None;


            // singular extensions

            let mut se_ext = 0;

            if mv == hash_move && let Some(singular_beta) = singular_beta {
                let singular_depth = (depth - 1) / 2;
                let singular_score = self.search(pos, singular_depth, singular_beta-1, singular_beta, Some(mv)).0;

                if singular_score < singular_beta {
                    se_ext = 1;
                }
            }

            self.push_move(pos, mv);

            if pos.checked(side) {
                self.pop_move(pos);
                continue;
            }

            // futility pruning

            let fp_margin = eval + 200 * depth;

            if depth < 6
                && !in_check
                && quiet
                && fp_margin < alpha
                && !alpha.is_mate()
                && !best_score.is_mated()
            {
                self.pop_move(pos);
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
                score = -self.search(pos, depth - 1 + se_ext - lmr, -(alpha + 1), -alpha, None).0;

                if lmr > 0 && score > alpha {
                    score = -self.search(pos, depth - 1 + se_ext, -(alpha + 1), -alpha, None).0;
                }
            }

            if pv_node && (move_index == 0 || score > alpha) {
                score = -self.search(pos, depth - 1 + se_ext, -beta, -alpha, None).0;
            }

            if self.exited {
                self.pop_move(pos);
                return (0, NULL_MOVE);
            }

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                alpha = score;
                best_move = mv;

                if pv_node && ply < self.pv_table.len() {
                    self.pv_table[ply][0] = mv;

                    if ply < self.pv_table.len() - 1 {
                        for i in 0..MAX_PV_SIZE-1 {
                            let pv_mv = self.pv_table[ply+1][i];

                            self.pv_table[ply][i+1] = pv_mv;

                            if pv_mv == NULL_MOVE {
                                break;
                            }
                        }
                    }
                }
            }

            if alpha >= beta {
                self.pop_move(pos);

                if exclude.is_none() {
                    self.tt_set(pos.hash, mv, TTKind::Lower, best_score, ply, depth);

                    if quiet {
                        self.add_killer(mv, ply);

                        let hist_bonus = 300 * depth - 250;
                        self.update_history(mv, hist_bonus as i16);

                        for q in quiets.iter() {
                            self.update_history(*q, -hist_bonus as i16);
                        }

                        for i in CONT_HISTORY_PLIES {
                            if ply < 1 {
                                continue;
                            }

                            let cont = self.ss[ply - i].cont;
                            if cont.is_none() {
                                continue;
                            }
                            let cont = cont.unwrap();

                            self.cont_hist[cont].update(pos, mv, hist_bonus as i16);

                            for &q in quiets.iter() {
                                self.cont_hist[cont].update(pos, q, -hist_bonus as i16);
                            }
                        }
                    }
                }

                return (best_score, best_move);
            }

            self.pop_move(pos);

            if quiet {
                quiets.push(mv);
            }

            move_index += 1;
        }

        if move_index == 0 {
            if in_check {
                return (-MATE_SCORE + ply as i32, NULL_MOVE);
            } else {
                return (0, NULL_MOVE);
            }
        }

        if pos.halfmove_clock >= 100 {
            if ply < self.pv_table.len() {
                self.pv_table[ply][0] = NULL_MOVE;
            }
            return (0, NULL_MOVE);
        }

        if exclude.is_none() {
            self.tt_set(
                pos.hash,
                best_move,
                if best_score > alpha0 {
                    TTKind::Exact
                } else {
                    TTKind::Upper
                },
                best_score,
                ply,
                depth,
            );
        }
        (best_score, best_move)
    }

    pub fn best(
        &mut self,
        pos: &mut Position,
        depth: i32,
        time_limit_hard: f32,
        time_limit_soft: f32,
        node_limit_hard: usize,
        node_limit_soft: usize,
    ) -> (Move, i32) {
        self.reset(
            time_limit_hard,
            time_limit_soft,
            node_limit_hard,
            node_limit_soft,
        );

        let mut best_move = NULL_MOVE;
        let mut best_score = 0i32;

        for d in 1..=depth {
            if self.nodes >= self.node_limit_soft || self.elapsed() >= self.time_limit_soft * 0.95 {
                break;
            }

            let mut window_lo = 25;
            let mut window_hi = 25;

            let (score, mv) = loop {
                let (alpha, beta) = if d < 4 {
                    (-INF_SCORE, INF_SCORE)
                } else {
                    (
                        (best_score - window_lo).clamp(-INF_SCORE, INF_SCORE),
                        (best_score + window_hi).clamp(-INF_SCORE, INF_SCORE),
                    )
                };

                let (score, mv) = self.search(pos, d, alpha, beta, None);

                assert!(self.ss.len() == 0);

                if (score > alpha && score < beta) || self.exited {
                    break (score, mv);
                } else if score <= alpha {
                    window_lo *= 2;
                } else {
                    window_hi *= 2;
                }
            };

            if self.exited {
                break;
            }

            best_score = score;

            let score_str = if score.is_mate() {
                let plies = MATE_SCORE - score.abs();
                format!(
                    "mate {}{}",
                    if score < 0 { "-" } else { "" },
                    (plies + 1) / 2
                )
            } else {
                format!("cp {}", score)
            };

            let nps = (self.nodes as f32 / self.elapsed()).round() as i32;
            let time = (self.elapsed() * 1000.0).round() as i32;

            best_move = mv;

            let mut pv_string = String::new();

            for (i, &pv_mv) in self.pv_table[0].iter().enumerate() {
                if pv_mv == NULL_MOVE {
                    break;
                }

                if i > 0 {
                    pv_string.push(' ');
                }

                pv_string.push_str(&pv_mv.uci_string());
            }

            if self.enable_uci {
                println!(
                    "info depth {} score {} nodes {} nps {} time {} pv {}",
                    d,
                    score_str,
                    self.nodes,
                    nps,
                    time,
                    pv_string
                );
            }
        }

        (best_move, best_score)
    }

    fn push_move(&mut self, pos: &mut Position, mv: Move) {
        let se = if mv == NULL_MOVE {
            let se = SearchEntry {
                cont: None,
                eval: pos.relative_eval(),
                mv,
            };

            pos.make_null_move();

            se
        } else {
            let piece = pos.board[mv.from()].id().unwrap();
            let to = mv.to();
            let side = mv.side().id();

            let se = SearchEntry {
                cont: Some(side * 6 * 64 + piece * 64 + to),
                eval: pos.relative_eval(),
                mv,
            };

            pos.make_move(mv);

            se
        };

        self.ss.push(se);
    }

    fn pop_move(&mut self, pos: &mut Position) {
        let se = self.ss.pop().unwrap();

        if se.mv == NULL_MOVE {
            pos.unmake_null_move();
        } else {
            pos.unmake_move();
        }
    }
}

fn see(pos: &Position, sq: usize, cur: Piece, side: Side, occ: u64) -> i32 {
    if let Some((attacker, attacker_sq)) = pos.smallest_attacker(sq, side, occ) {
        if attacker == Piece::King && pos.smallest_attacker(sq, side.opp(), occ).is_some() {
            0
        } else {
            let value = cur.centipawn_value()
                - see(pos, sq, attacker, side.opp(), occ & !(1u64 << attacker_sq));
            value.max(0)
        }
    } else {
        0
    }
}

fn see_capture(pos: &Position, mv: Move) -> i32 {
    let occ = pos.occ();

    if let Some(capture_piece) = pos.is_capture(mv) {
        let sq = mv.to();
        let value = capture_piece.centipawn_value()
            - see(
                pos,
                sq,
                pos.board[mv.from()],
                mv.side().opp(),
                occ & !(1u64 << mv.from()),
            );
        value
    } else {
        panic!("only captures can be see-ed");
    }
}

impl ContinuationTable {
    fn new() -> Self {
        Self {
            data: [[[0; 64]; 6]; 2],
        }
    }

    fn update(&mut self, pos: &Position, mv: Move, bonus: i16) {
        let clamped = bonus.clamp(-MAX_HISTORY, MAX_HISTORY);

        let piece = pos.board[mv.from()];
        let x = &mut self.data[mv.side().id()][piece.id().unwrap()][mv.to()];

        *x += clamped - ((*x as i32 * clamped.abs() as i32) / MAX_HISTORY as i32) as i16;
    }
}

#[allow(unused)]
trait TreeScore {
    fn is_mate(&self) -> bool;
    fn is_mated(&self) -> bool;
    fn is_mating(&self) -> bool;
}

impl TreeScore for i32 {
    fn is_mate(&self) -> bool {
        self.abs() > MATE_SCORE - 1000
    }

    fn is_mated(&self) -> bool {
        *self < -MATE_SCORE + 1000
    }

    fn is_mating(&self) -> bool {
        *self > MATE_SCORE - 1000
    }
}