use std::sync::*;

use crate::*;

const MAX_PLY: usize = 128;

const MOVE_SCORE_HASH_MOVE: i32 = 30_000_000;
const MOVE_SCORE_CAPTURE_BASE: i32 = 20_000_000;
const MOVE_SCORE_NON_CAPTURE_BASE: i32 = 10_000_000;

#[derive(Copy, Clone)]
pub struct Limits {
    pub hard_nodes: usize,
    pub soft_nodes: usize,

    pub hard_time: f32,
    pub soft_time: f32,

    pub depth: i32,
}

impl Limits {
    pub fn new() -> Self {
        Self {
            hard_nodes: 1_000_000_000_000,
            soft_nodes: 1_000_000_000_000,

            hard_time: 1_000_000.0,
            soft_time: 1_000_000.0,

            depth: 255,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum TTKind {
    Exact,
    Lower,
    Upper,
}

#[derive(Copy, Clone)]
struct TTEntry {
    kind: TTKind,
    depth: u8,
    hash_lo: u16,
    mv: Move,
    rel_score: i16,
}

impl TTEntry {
    const NULL: Self = TTEntry {
        kind: TTKind::Exact,
        depth: 0,
        hash_lo: 0,
        mv: Move::NULL,
        rel_score: 0,
    };

    fn cutoff(&self, ply: i32, alpha: Score, beta: Score) -> Option<Score> {
        let score = self.score(ply);
        match self.kind {
            TTKind::Exact => {
                if score > alpha && score < beta {
                    Some(score)
                } else {
                    None
                }
            }
            TTKind::Lower => {
                if score > beta {
                    Some(score)
                } else {
                    None
                }
            }
            TTKind::Upper => {
                if score < alpha {
                    Some(score)
                } else {
                    None
                }
            }
        }
    }

    fn score(&self, ply: i32) -> Score {
        let rel = self.rel_score as Score;

        if rel.is_mate() {
            if rel < 0 {
                rel + ply as Score
            } else {
                rel - ply as Score
            }
        } else {
            rel
        }
    }
}

pub struct Worker {
    pv: Box<[[Move; MAX_PLY]; MAX_PLY]>,

    tt: Vec<TTEntry>,

    nodes: usize,
    stopped: bool,
    start_time: std::time::Instant,

    stop: Arc<atomic::AtomicBool>,

    limits: Limits,

    pos_stack: Vec<Position>,
}

impl Worker {
    pub fn new(tt_size_mb: usize) -> Self {
        Self {
            pv: Box::new([[Move::NULL; _]; _]),
            tt: vec![TTEntry::NULL; tt_len(tt_size_mb)],
            nodes: 0,
            stopped: false,
            start_time: std::time::Instant::now(),
            stop: Arc::new(atomic::AtomicBool::new(false)),
            limits: Limits::new(),
            pos_stack: vec![],
        }
    }

    pub fn pv(&self) -> &[Move] {
        let len = self.pv[0].iter().take_while(|&x| *x != Move::NULL).count();
        &self.pv[0][..len]
    }

    fn qsearch(&mut self, mut alpha: Score, beta: Score, ply: usize) -> Score {
        self.nodes += 1;

        if self.check_stop() {
            return 0;
        }

        let alpha0 = alpha;

        let is_pv = beta > alpha + 1;

        if self.is_repetition(ply as _) {
            return 0;
        }

        let pos = self.pos_stack.last().unwrap().clone();

        let moves = move_gen::gen_legal_qsearch(&pos);

        if moves.len() == 0 {
            if pos.checked().is_some() {
                return -MATE_SCORE + ply as Score;
            }
        }

        if pos.halfmove_clock >= 100 {
            return 0;
        }

        let hash_mv = if let Some(entry) = self.tt_query(&pos) {
            if !is_pv && let Some(cutoff_score) = entry.cutoff(ply as _, alpha, beta) {
                return cutoff_score;
            }

            entry.mv
        } else {
            Move::NULL
        };

        let mut picker = MovePicker::new(&pos, moves, hash_mv);

        let mut best_score = if pos.checked().is_some() {
            -INF_SCORE
        } else {
            let stand_pat = relative_eval(&pos);

            if stand_pat >= beta {
                return stand_pat;
            }

            stand_pat
        };

        let mut best_mv = Move::NULL;

        while let Some((_, mv)) = picker.next() {
            let child = pos.make_move(mv);

            self.pos_stack.push(child);
            let score = -self.qsearch(-beta, -alpha, ply + 1);
            self.pos_stack.pop();

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                best_mv = mv;
                alpha = score;
            }

            if alpha >= beta {
                self.tt_write(&pos, 0, mv, ply as _, TTKind::Lower, best_score);
                return best_score;
            }
        }

        self.tt_write(
            &pos,
            0,
            best_mv,
            ply as _,
            if best_score > alpha0 {
                TTKind::Exact
            } else {
                TTKind::Upper
            },
            best_score,
        );

        best_score
    }

    fn search(&mut self, mut alpha: Score, beta: Score, ply: usize, depth: i32) -> Score {
        if ply < self.pv.len() {
            self.pv[ply][0] = Move::NULL;
        }

        if depth <= 0 {
            return self.qsearch(alpha, beta, ply);
        }

        self.nodes += 1;

        if self.check_stop() {
            return 0;
        }

        let alpha0 = alpha;

        let is_pv = beta > alpha + 1;

        if self.is_repetition(ply as _) {
            return 0;
        }

        let pos = self.pos_stack.last().unwrap().clone();

        let moves = pos.gen_legal_moves();

        if moves.len() == 0 {
            if pos.checked().is_some() {
                return -MATE_SCORE + (ply as Score);
            } else {
                return 0;
            }
        }

        if pos.halfmove_clock >= 100 {
            return 0;
        }

        let hash_mv = if let Some(entry) = self.tt_query(&pos) {
            if !is_pv
                && entry.depth as i32 >= depth
                && let Some(cut_score) = entry.cutoff(ply as _, alpha, beta)
            {
                return cut_score;
            }

            entry.mv
        } else {
            Move::NULL
        };

        let mut picker = MovePicker::new(&pos, moves, hash_mv);

        let mut best_score = -INF_SCORE;
        let mut best_mv = Move::NULL;

        while let Some((mv_index, mv)) = picker.next() {
            let child = pos.make_move(mv);

            self.pos_stack.push(child);

            let mut score = 0;

            if !is_pv || mv_index > 0 {
                score = -self.search(-(alpha + 1), -alpha, ply + 1, depth - 1);
            }

            if is_pv && (mv_index == 0 || score > alpha) {
                score = -self.search(-beta, -alpha, ply + 1, depth - 1);
            }

            self.pos_stack.pop();

            if self.stopped {
                return 0;
            }

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                if is_pv && ply < self.pv.len() {
                    self.pv[ply][0] = mv;

                    if (ply + 1) < self.pv.len() {
                        for i in 0..(self.pv[ply + 1].len() - 1) {
                            let x = self.pv[ply + 1][i];
                            self.pv[ply][i + 1] = x;

                            if x == Move::NULL {
                                break;
                            }
                        }
                    } else {
                        self.pv[ply][1] = Move::NULL;
                    }
                }

                best_mv = mv;
                alpha = score;
            }

            if alpha >= beta {
                self.tt_write(&pos, depth, mv, ply as _, TTKind::Lower, best_score);
                return best_score;
            }
        }

        self.tt_write(
            &pos,
            depth,
            best_mv,
            ply as _,
            if best_score > alpha0 {
                TTKind::Exact
            } else {
                TTKind::Upper
            },
            best_score,
        );

        best_score
    }

    pub fn resize_tt(&mut self, size_mb: usize) {
        let new_len = tt_len(size_mb);
        self.tt = vec![TTEntry::NULL; new_len];
    }

    pub fn reset(&mut self) {
        self.tt.fill(TTEntry::NULL);
    }

    pub fn elapsed(&self) -> f32 {
        let now = std::time::Instant::now();
        (now - self.start_time).as_secs_f32()
    }

    pub fn go(
        &mut self,
        pos_stack: Vec<Position>,
        limits: Limits,
        stop: Arc<atomic::AtomicBool>,
    ) -> Score {
        self.nodes = 0;
        self.stopped = false;
        self.stop = stop;
        self.limits = limits;

        self.pos_stack = pos_stack;

        self.start_time = std::time::Instant::now();

        let mut root_score = 0;
        let mut root_pv = [Move::NULL; MAX_PLY];

        for d in 1..=self.limits.depth {
            let score = self.search(-INF_SCORE, INF_SCORE, 0, d);

            if self.stopped {
                break;
            }

            root_score = score;

            let pv = self.pv();
            root_pv[..pv.len()].copy_from_slice(pv);

            let elapsed = self.elapsed();
            let nps = self.nodes as f32 / elapsed;

            let score_str = if score.is_mate() {
                let sign = score.signum();
                let plies = MATE_SCORE - score.abs();
                let moves = (plies + 1) / 2;
                format!("mate {}", sign * moves)
            } else {
                format!("cp {}", score)
            };

            print!(
                "info depth {} score {} nodes {} nps {} time {} pv",
                d,
                score_str,
                self.nodes,
                nps as i32,
                (elapsed * 1000.0) as i32
            );

            for mv in self.pv() {
                print!(" {}", mv);
            }

            println!("");

            if self.nodes >= self.limits.soft_nodes || elapsed >= self.limits.soft_time {
                break;
            }
        }

        // so we always have a valid pv
        self.pv[0] = root_pv;

        root_score
    }

    fn is_repetition(&self, ply: i32) -> bool {
        let pos = self.pos_stack.last().unwrap();

        let hash = pos.hash;
        let hm = pos.halfmove_clock as usize;

        let mut count = 1;
        let mut offset = 2;

        let max_offset = hm.min(self.pos_stack.len() - 1);

        while offset <= max_offset {
            let index = self.pos_stack.len() - 1 - offset;

            if self.pos_stack[index].hash == hash {
                count += 1;

                let threshold = if (ply - offset as i32) <= 0 { 3 } else { 2 };

                if count >= threshold {
                    return true;
                }
            }

            offset += 2;
        }

        false
    }

    fn check_stop(&mut self) -> bool {
        if self.nodes & 4095 == 0 {
            if self.stop.load(atomic::Ordering::Relaxed)
                || self.elapsed() >= self.limits.hard_time
                || self.nodes >= self.limits.hard_nodes
            {
                self.stopped = true;
            }
        }

        return self.stopped;
    }

    pub fn nodes(&self) -> usize {
        self.nodes
    }

    fn tt_index(&self, pos: &Position) -> usize {
        pos.hash.carrying_mul(self.tt.len() as _, 0).1 as usize
    }

    fn tt_query(&self, pos: &Position) -> Option<TTEntry> {
        let entry = self.tt[self.tt_index(pos)];

        if entry.hash_lo == pos.hash as u16 {
            Some(entry)
        } else {
            None
        }
    }

    fn tt_write(
        &mut self,
        pos: &Position,
        depth: i32,
        mv: Move,
        ply: i32,
        kind: TTKind,
        score: Score,
    ) {
        let idx = self.tt_index(pos);
        let entry = &mut self.tt[idx];

        let rel_score = if score.is_mate() {
            if score < 0 {
                (score - ply) as i16
            } else {
                (score + ply) as i16
            }
        } else {
            score as i16
        };

        *entry = TTEntry {
            kind,
            depth: depth as _,
            hash_lo: pos.hash as u16,
            mv,
            rel_score,
        };
    }
}

fn relative_eval(pos: &Position) -> Score {
    eval::evaluate(pos) * pos.stm.sign()
}

trait ScoreType {
    fn is_mate(&self) -> bool;
}

impl ScoreType for Score {
    fn is_mate(&self) -> bool {
        self.abs() > (MATE_SCORE - 1000)
    }
}

struct MovePicker {
    moves: MoveList,
    scores: [i32; 256],
    index: usize,
}

impl MovePicker {
    fn new(pos: &Position, moves: MoveList, hash_mv: Move) -> Self {
        let mut scores = [0; _];

        for i in 0..moves.len() {
            scores[i] = Self::score_move(pos, moves[i], hash_mv);
        }

        Self {
            moves,
            scores,
            index: 0,
        }
    }

    fn score_move(pos: &Position, mv: Move, hash_mv: Move) -> i32 {
        if mv == hash_mv {
            MOVE_SCORE_HASH_MOVE
        } else if let Some((_, p)) = pos.capture(mv) {
            MOVE_SCORE_CAPTURE_BASE + p.material_value() - p.id() as i32
        } else {
            MOVE_SCORE_NON_CAPTURE_BASE
        }
    }

    fn next(&mut self) -> Option<(usize, Move)> {
        if self.index >= self.moves.len() {
            None
        } else {
            let mut best_idx = 0xffffffff;
            let mut best_score = i32::MIN;

            for i in self.index..self.moves.len() {
                let s = self.scores[i];

                if s > best_score {
                    best_idx = i;
                    best_score = s;
                }
            }

            self.moves.swap(self.index, best_idx);
            self.scores.swap(self.index, best_idx);

            let idx = self.index;
            self.index += 1;

            let mv = self.moves[idx];

            Some((idx, mv))
        }
    }
}

fn tt_len(size_mb: usize) -> usize {
    let bytes = size_mb * 1024 * 1024;
    let n = bytes / std::mem::size_of::<TTEntry>();
    n.max(1024)
}
