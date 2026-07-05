use std::sync::*;

use crate::*;

const MAX_PLY: usize = 128;

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

pub struct Worker {
    pv: [[Move; MAX_PLY]; MAX_PLY],

    nodes: usize,
    stopped: bool,
    start_time: std::time::Instant,

    stop: Arc<atomic::AtomicBool>,

    limits: Limits,
}

impl Worker {
    pub fn new() -> Self {
        Self {
            pv: [[Move::NULL; _]; _],
            nodes: 0,
            stopped: false,
            start_time: std::time::Instant::now(),
            stop: Arc::new(atomic::AtomicBool::new(false)),
            limits: Limits::new(),
        }
    }

    pub fn pv(&self) -> &[Move] {
        let len = self.pv[0].iter().take_while(|&x| *x != Move::NULL).count();
        &self.pv[0][..len]
    }

    pub fn search(&mut self, pos: &Position, mut alpha: Score, beta: Score, ply: usize, depth: i32) -> Score {
        if ply < self.pv.len() {
            self.pv[ply][0] = Move::NULL;
        }

        if depth <= 0 {
            return relative_eval(pos);
        }

        self.nodes += 1;

        if self.check_stop() {
            return 0;
        }

        let moves = pos.gen_legal_moves();

        if moves.len() == 0 {
            if pos.checked().is_some() {
                return -MATE_SCORE + (ply as Score);
            } else {
                return 0;
            }
        }

        let mut best_score = -INF_SCORE;

        for mv in moves {
            let child = pos.make_move(mv);

            let score = -self.search(&child, -beta, -alpha, ply + 1, depth - 1);

            if self.stopped {
                return 0;
            }

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                if ply < self.pv.len() {
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

                alpha = score;
            }

            if alpha >= beta {
                return best_score;
            }
        }

        best_score
    }

    pub fn elapsed(&self) -> f32 {
        let now = std::time::Instant::now();
        (now - self.start_time).as_secs_f32()
    }

    pub fn go(&mut self, pos: &Position, limits: Limits, stop: Arc<atomic::AtomicBool>) -> Score {
        self.nodes = 0;
        self.stopped = false;
        self.stop = stop;
        self.limits = limits;

        self.start_time = std::time::Instant::now();

        let mut root_score = 0;
        let mut root_pv = [Move::NULL; MAX_PLY];

        for d in 1..=self.limits.depth {
            let score = self.search(pos, -INF_SCORE, INF_SCORE, 0, d);

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
                let moves = (plies+1)/2;
                format!("mate {}", sign * moves)
            }
            else {
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