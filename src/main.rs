use noggin::search::*;
use noggin::*;

use noggin::pcg::PCG32;

mod viri;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

fn is_legal(pos: &mut Position, mv: Move) -> bool {
    let moves = movegen::gen_pseudolegal_moves(pos);

    let mut legal = std::collections::HashSet::<Move>::new();

    let side = pos.to_move;

    for i in 0..moves.len() {
        let mv = moves[i];
        pos.make_move(mv);

        if !pos.checked(side) {
            legal.insert(mv);
        }

        pos.unmake_move();
    }

    legal.contains(&mv)
}

#[derive(Debug)]
struct GoParameters {
    wtime: f32,
    btime: f32,
    winc: f32,
    binc: f32,
    movetime: Option<f32>,
    nodes: usize,
    depth: usize,
    infinite: bool,
}

fn parse_ms(val: &str) -> Result<f32, String> {
    let ms = val
        .parse::<u32>()
        .map_err(|_| format!("invalid time value '{}'", val))?;
    Ok(ms as f32 / 1000.0)
}

impl GoParameters {
    fn parse(args: &[&str]) -> Result<Self, String> {
        if args[0] != "go" {
            return Err("expected 'go'".to_string());
        }

        let mut i = 1;

        let mut params = GoParameters {
            wtime: f32::INFINITY,
            btime: f32::INFINITY,
            winc: 0.0,
            binc: 0.0,
            movetime: None,
            nodes: 1024 * 1024 * 1024,
            depth: 50,
            infinite: false,
        };

        while i < args.len() {
            if args[i] == "wtime" {
                let &wtime_str = args.get(i + 1).ok_or("expected wtime value")?;
                params.wtime = parse_ms(wtime_str)?;
                i += 2;
            } else if args[i] == "btime" {
                let &btime_str = args.get(i + 1).ok_or("expected btime value")?;
                params.btime = parse_ms(btime_str)?;
                i += 2;
            } else if args[i] == "winc" {
                let &winc_str = args.get(i + 1).ok_or("expected winc value")?;
                params.winc = parse_ms(winc_str)?;
                i += 2;
            } else if args[i] == "binc" {
                let &binc_str = args.get(i + 1).ok_or("expected binc value")?;
                params.binc = parse_ms(binc_str)?;
                i += 2;
            } else if args[i] == "movetime" {
                let &movetime_str = args.get(i + 1).ok_or("expected movetime value")?;
                params.movetime = Some(parse_ms(movetime_str)?);
                i += 2;
            } else if args[i] == "nodes" {
                let &nodes_str = args.get(i + 1).ok_or("expected nodes value")?;
                params.nodes = nodes_str
                    .parse::<usize>()
                    .map_err(|_| format!("invalid nodes value '{}'", nodes_str))?;
                i += 2;
            } else if args[i] == "depth" {
                let &depth_str = args.get(i + 1).ok_or("expected depth value")?;
                params.depth = depth_str
                    .parse::<usize>()
                    .map_err(|_| format!("invalid depth value '{}'", depth_str))?;
                i += 2;
            } else if args[i] == "infinite" {
                params.infinite = true;
                i += 1;
            } else {
                i += 1;
            }
        }

        Ok(params)
    }
}

fn allocate_time(params: &GoParameters, to_move: Side) -> (f32, f32) {
    if let Some(movetime) = params.movetime {
        return (movetime, movetime);
    }

    let (time, inc) = match to_move {
        Side::White => (params.wtime, params.winc),
        Side::Black => (params.btime, params.binc),
    };

    let soft = (time / 40.0 + inc * 0.8).max(0.05);
    let hard = (soft * 4.0).min(time * 0.25);

    return (soft, hard);
}

enum Context {
    Idle(Position, Searcher),
    Searching(
        std::thread::JoinHandle<(Position, Searcher)>,
        Arc<AtomicBool>,
    ),
}

impl Context {
    fn get(self) -> (Position, Searcher) {
        match self {
            Self::Idle(p, s) => (p, s),
            Self::Searching(handle, stop) => {
                stop.store(true, Ordering::Relaxed);
                handle.join().unwrap()
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(option) = args.get(1) {
        if option == "bench" {
            bench_main();
            return;
        } else if option == "metrics" {
            metrics_main();
            return;
        } else if option == "datagen" {
            datagen_main();
            return;
        }
    }

    let mut context = Context::Idle(Position::from_fen(STARTING_FEN).unwrap(), Searcher::new());

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_string();
        let args: Vec<&str> = input.split(' ').collect();

        if args[0] == "uci" {
            let (pos, searcher) = context.get();

            println!("id name Noggin");
            println!("id author Noggin Authors");
            println!("option name Hash type spin default 1 min 1 max 16");
            println!("option name Threads type spin default 1 min 1 max 1");
            println!("uciok");

            context = Context::Idle(pos, searcher);
        } else if args[0] == "isready" {
            let (pos, searcher) = context.get();

            println!("readyok");

            context = Context::Idle(pos, searcher);
        } else if args[0] == "quit" {
            context.get();
            return;
        } else if args[0] == "ucinewgame" {
            context.get();
            let pos = Position::from_fen(STARTING_FEN).unwrap();
            let searcher = search::Searcher::new();
            context = Context::Idle(pos, searcher);
        } else if args[0] == "position" {
            let (prev, searcher) = context.get();

            if args.len() < 2 {
                println!("specify 'startpos' or 'fen'");
                context = Context::Idle(prev, searcher);
                continue;
            }

            let (mut pos, mut next) = if args[1] == "startpos" {
                (Position::from_fen(STARTING_FEN).unwrap(), 2)
            } else if args[1] == "fen" {
                if args.len() < 8 {
                    println!("expected a FEN string");
                    context = Context::Idle(prev, searcher);
                    continue;
                }

                let fen = args[2..8].join(" ");

                let pos = if let Ok(p) = Position::from_fen(&fen) {
                    p
                } else {
                    println!("malformed FEN '{}'", fen);
                    context = Context::Idle(prev, searcher);
                    continue;
                };

                (pos, 8)
            } else {
                println!("expected 'startpos' or 'fen', got '{}'", args[1]);
                context = Context::Idle(prev, searcher);
                continue;
            };

            if args.len() <= next {
                context = Context::Idle(pos, searcher);
                continue;
            }

            if args[next] != "moves" {
                println!("unexpected token '{}'", args[next]);
            }

            loop {
                next += 1;

                if next >= args.len() {
                    break;
                }

                let uci_mv = args[next];

                let mv = if let Some(mv) = parse_uci_move(uci_mv, pos.to_move) {
                    mv
                } else {
                    println!("malformed move '{}'", uci_mv);
                    break;
                };

                if !is_legal(&mut pos, mv) {
                    println!("illegal move '{}'", uci_mv);
                    break;
                }

                pos.make_move(mv);
            }

            context = Context::Idle(pos, searcher);
        } else if args[0] == "go" {
            let (mut pos, mut searcher) = context.get();

            match GoParameters::parse(&args) {
                Ok(params) => {
                    let (soft_time, hard_time) = allocate_time(&params, pos.to_move);

                    let stop = searcher.stop.clone();

                    let handle = std::thread::spawn(move || {
                        let mv = searcher
                            .best(
                                &mut pos,
                                params.depth as _,
                                hard_time,
                                soft_time,
                                params.nodes,
                                params.nodes,
                            )
                            .0;
                        println!("bestmove {}", mv.uci_string());
                        (pos, searcher)
                    });

                    context = Context::Searching(handle, stop);
                }

                Err(e) => {
                    println!("invalid go command: {}", e);
                    context = Context::Idle(pos, searcher);
                }
            }
        } else if args[0] == "stop" {
            let (pos, searcher) = context.get();
            context = Context::Idle(pos, searcher);
        } else if args[0] == "setoption" {
            let (pos, searcher) = context.get();
            context = Context::Idle(pos, searcher);
        } else {
            let (pos, searcher) = context.get();
            println!("unrecognized command '{}'", input);
            context = Context::Idle(pos, searcher);
        }
    }
}

fn bench_main() {
    let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
    let mut s = search::Searcher::new();

    s.best(
        &mut pos,
        12,
        f32::INFINITY,
        f32::INFINITY,
        1024 * 1024 * 1024,
        1024 * 1024 * 1024,
    );

    let nps = s.nodes() as f32 / s.elapsed();

    println!("{} nodes {} nps", s.nodes(), nps.round() as usize);
}

fn metrics_main() {
    for d in 1..=12 {
        let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
        let mut s = search::Searcher::new();

        s.disable_uci();

        s.best(
            &mut pos,
            d,
            f32::INFINITY,
            f32::INFINITY,
            1024 * 1024 * 1024,
            1024 * 1024 * 1024,
        );

        println!("metrics depth {}", d);
        println!("=================");
        println!("time: {:.2}s", s.elapsed());
        println!("nodes: {}", s.nodes());
        println!(
            "qnodes: {} ({:.2}%)",
            s.qnodes(),
            s.qnodes() as f32 / s.nodes() as f32 * 100.0
        );
        println!("nps: {:.2}M", s.nodes() as f32 / s.elapsed() / 1_000_000.0);
        println!("tt-hit: {:.2}%", s.tt_hit_rate() * 100.0);
        println!("tt-fill: {:.2}%", s.tt_fill() * 100.0);
        println!("tt-collision: {:.2}%", s.tt_collision_rate() * 100.0);
        println!("");
    }
}

fn get_timestamp() -> std::time::Duration {
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now();
    let timestamp = now.duration_since(UNIX_EPOCH).unwrap();

    timestamp
}

fn attempt_datagen_match(match_index: usize) -> Option<viri::Game> {
    let mut pos = Position::from_fen(STARTING_FEN).unwrap();

    let seed_mix = match_index as u64;
    let mut rng = PCG32::new(
        get_timestamp().as_nanos() as u64 ^ seed_mix.wrapping_mul(6364136223846793005),
        seed_mix * 2 + 3,
    );

    let mut searcher = Searcher::new();
    searcher.disable_uci();

    let mut seq = vec![];
    let mut starting_pos = None;

    const RANDOM_PLIES: usize = 10;

    for ply in 0..10000 {
        let mut legal_moves = movegen::gen_pseudolegal_moves(&pos);
        pos.filter_legal(&mut legal_moves);

        if let Some(result) = pos.game_over(&legal_moves) {
            let start = starting_pos?;

            let result_str = match result {
                GameResult::Checkmate(Side::White) => "White mates",
                GameResult::Checkmate(Side::Black) => "Black mates",
                GameResult::Stalemate => "Draw by stalemate",
                GameResult::TheefoldRepetition => "Draw by threefold-repetition",
                GameResult::FiftyMove => "Draw by 50-move rule",
            };

            println!(
                "Game {} ended in {} moves - {}",
                match_index,
                seq.len(),
                result_str
            );

            let wdl = match result {
                GameResult::Checkmate(Side::Black) => 0,
                GameResult::Checkmate(Side::White) => 2,
                _ => 1,
            };

            return Some(viri::Game {
                board: viri::PackedBoard::from_position(&start, wdl),
                seq,
            });
        }

        if ply < RANDOM_PLIES {
            let i = rng.range(0, legal_moves.len());
            let mv = legal_moves[i];
            pos.make_move(mv);
        } else {
            if ply == RANDOM_PLIES {
                starting_pos = Some(pos.clone());
            }

            let (mv, score) = searcher.best(&mut pos, 50, 1e6, 1e6, 1_000_000_000, 5_000);

            let vmv = viri::Move::from_native(&pos, mv);
            let score = (pos.to_move.sign() * score).try_into().unwrap();

            seq.push((vmv, score));

            pos.make_move(mv);
        }
    }

    None
}

fn run_datagen_match(index: usize, file: &std::sync::Mutex<std::fs::File>) {
    let m = loop {
        if let Some(m) = attempt_datagen_match(index) {
            break m;
        }
    };

    let mut f = file.lock().unwrap();

    m.dump(&mut f);
}

fn datagen_main() {
    use rayon::prelude::*;

    loop {
        let file =
            std::fs::File::create(format!("data/data_{}.bin", get_timestamp().as_secs())).unwrap();
        let file = std::sync::Mutex::new(file);

        const MATCHES_PER_SHARD: usize = 10000;

        (0..MATCHES_PER_SHARD)
            .into_par_iter()
            .for_each(|i| run_datagen_match(i, &file));
    }
}
