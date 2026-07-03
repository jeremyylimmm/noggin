use crate::*;

const MAX_PLY: usize = 128;

pub struct Worker {
    pv: [[Move; MAX_PLY]; MAX_PLY],
    nodes: usize,
}

impl Worker {
    pub fn new() -> Self {
        Self {
            pv: [[Move::NULL; _]; _],
            nodes: 0,
        }
    }

    pub fn pv(&self) -> &[Move] {
        let len = self.pv[0].iter().take_while(|&x| *x != Move::NULL).count();
        &self.pv[0][..len]
    }

    pub fn search(&mut self, pos: &Position, ply: usize, depth: i32) -> Score {
        if depth <= 0 {
            return relative_eval(pos);
        }

        self.nodes += 1;

        if ply < self.pv.len() {
            self.pv[ply][0] = Move::NULL;
        }

        let moves = pos.gen_legal_moves();

        if moves.len() == 0 {
            if pos.checked().is_some() {
                return -MATE_SCORE + (ply as Score);
            } else {
                return 0;
            }
        }

        let mut best_score = -MATE_SCORE;

        for mv in moves {
            let child = pos.make_move(mv);

            let score = -self.search(&child, ply + 1, depth - 1);

            if score > best_score {
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

                best_score = score;
            }
        }

        best_score
    }

    pub fn go(&mut self, pos: &Position) -> Score {
        self.nodes = 0;

        let mut score = 0;

        let start = std::time::Instant::now();

        for d in 1..=5 {
            score = self.search(pos, 0, d);

            let elapsed = (std::time::Instant::now() - start).as_secs_f32();
            let nps = self.nodes as f32 / elapsed;

            print!(
                "info depth {} score {} nodes {} nps {} time {} pv",
                d,
                score,
                self.nodes,
                nps as i32,
                (elapsed * 1000.0) as i32
            );

            for mv in self.pv() {
                print!(" {}", mv);
            }

            println!("");
        }

        score
    }
}

fn relative_eval(pos: &Position) -> Score {
    eval::evaluate(pos) * pos.stm.sign()
}
