use noggin::*;

use std::sync::*;

enum State {
    Idle(search::Worker),
    Working(
        std::thread::JoinHandle<search::Worker>,
        Arc<atomic::AtomicBool>,
    ),
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if let Some(opt) = args.get(1) {
        if opt == "bench" {
            bench_main();
        } else {
            println!("invalid option '{}'", opt);
        }
    } else {
        uci_main();
    }
}

fn uci_main() {
    let mut state = State::Idle(search::Worker::new());
    let mut pos = Position::from_fen(STARTPOS_FEN).unwrap();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        let args: Vec<_> = input.split_whitespace().collect();

        if args[0] == "uci" {
            let worker = state.flush();

            println!("id name Noggin 3.0.1");
            println!("id author Noggin Authors");
            println!("option name Hash type spin default 16 min 1 max 4096");
            println!("option name Threads type spin default 1 min 1 max 1");
            println!("uciok");

            state = State::Idle(worker);
        } else if args[0] == "isready" {
            let worker = state.flush();
            println!("readyok");
            state = State::Idle(worker);
        } else if args[0] == "quit" {
            state.flush();
            return;
        } else if args[0] == "ucinewgame" {
            state.flush();
            let worker = search::Worker::new();
            pos = Position::from_fen(STARTPOS_FEN).unwrap();
            state = State::Idle(worker);
        } else if args[0] == "position" {
            let w = state.flush();
            match parse_position_cmd(&args) {
                Ok(p) => pos = p,
                Err(msg) => println!("{}", msg),
            }
            state = State::Idle(w);
        } else if args[0] == "go" {
            let mut worker = state.flush();

            match parse_go(&args) {
                Ok(params) => {
                    let stop = Arc::new(atomic::AtomicBool::new(false));

                    let pos = pos.clone();

                    let stop_copy = stop.clone();
                    let limits = params.limits(pos.stm());

                    let join_handle = std::thread::spawn(move || {
                        worker.go(&pos, limits, stop_copy);
                        println!("bestmove {}", worker.pv()[0]);
                        return worker;
                    });

                    state = State::Working(join_handle, stop);
                }

                Err(msg) => {
                    println!("{}", msg);
                    state = State::Idle(worker);
                }
            }
        } else if args[0] == "stop" {
            let w = state.flush();
            state = State::Idle(w);
        } else if args[0] == "setoption" {
            let w = state.flush();
            state = State::Idle(w);
        } else {
            println!("unrecognized command {}", args[0]);
        }
    }
}

fn parse_position_cmd(args: &[&str]) -> Result<Position, String> {
    assert!(args[0] == "position");

    let &base = args
        .get(1)
        .ok_or("specify 'startpos' or 'fen'".to_string())?;

    let (mut pos, next) = if base == "startpos" {
        (Position::from_fen(STARTPOS_FEN).unwrap(), 2)
    } else if base == "fen" {
        if args.len() < 8 {
            return Err("specify a fen".to_string());
        }

        let fen = args[2..8].join(" ");

        (Position::from_fen(&fen)?, 8)
    } else {
        return Err(format!("expected 'startpos' or 'fen', got '{}'", base));
    };

    if next >= args.len() {
        return Ok(pos);
    }

    if args[next] != "moves" {
        return Err(format!("expected 'moves', got {}", args[next]));
    }

    for uci_mv in &args[(next + 1)..] {
        let mv = Move::from_uci(uci_mv).ok_or(format!("malformed move {}", uci_mv))?;

        if !pos.is_legal(mv) {
            return Err(format!("move {} is illegal", uci_mv));
        }

        pos = pos.make_move(mv);
    }

    Ok(pos)
}

impl State {
    fn flush(self) -> search::Worker {
        match self {
            State::Idle(w) => w,
            State::Working(join_handle, stop) => {
                stop.store(true, atomic::Ordering::Relaxed);
                let w = join_handle.join().unwrap();
                w
            }
        }
    }
}

struct GoParams {
    wtime: Option<f32>,
    btime: Option<f32>,
    winc: Option<f32>,
    binc: Option<f32>,
    depth: Option<usize>,
    nodes: Option<usize>,
    movetime: Option<f32>,
}

fn parse_ms(x: &str) -> Option<f32> {
    let ms = x.parse::<usize>().ok()?;
    Some((ms as f64 / 1000.0) as f32)
}

fn parse_go(args: &[&str]) -> Result<GoParams, String> {
    assert!(args[0] == "go");

    let mut params = GoParams {
        wtime: None,
        btime: None,
        winc: None,
        binc: None,
        depth: None,
        nodes: None,
        movetime: None,
    };

    let mut i = 1;

    while i < args.len() {
        if args[i] == "wtime" {
            let &x = args
                .get(i + 1)
                .ok_or("unexpected end of command".to_string())?;
            params.wtime = Some(parse_ms(x).ok_or(format!("invalid value '{}'", x))?);
            i += 2;
        } else if args[i] == "btime" {
            let &x = args
                .get(i + 1)
                .ok_or("unexpected end of command".to_string())?;
            params.btime = Some(parse_ms(x).ok_or(format!("invalid value '{}'", x))?);
            i += 2;
        } else if args[i] == "winc" {
            let &x = args
                .get(i + 1)
                .ok_or("unexpected end of command".to_string())?;
            params.winc = Some(parse_ms(x).ok_or(format!("invalid value '{}'", x))?);
            i += 2;
        } else if args[i] == "binc" {
            let &x = args
                .get(i + 1)
                .ok_or("unexpected end of command".to_string())?;
            params.binc = Some(parse_ms(x).ok_or(format!("invalid value '{}'", x))?);
            i += 2;
        } else if args[i] == "depth" {
            let &x = args
                .get(i + 1)
                .ok_or("unexpected end of command".to_string())?;
            params.depth = Some(
                x.parse::<usize>()
                    .map_err(|_| format!("invalid value '{}'", x))?,
            );
            i += 2;
        } else if args[i] == "nodes" {
            let &x = args
                .get(i + 1)
                .ok_or("unexpected end of command".to_string())?;
            params.nodes = Some(
                x.parse::<usize>()
                    .map_err(|_| format!("invalid value '{}'", x))?,
            );
            i += 2;
        } else if args[i] == "movetime" {
            let &x = args
                .get(i + 1)
                .ok_or("unexpected end of command".to_string())?;
            params.movetime = Some(parse_ms(x).ok_or(format!("invalid value '{}'", x))?);
            i += 2;
        } else {
            i += 1;
        }
    }

    Ok(params)
}

impl GoParams {
    fn allocate_time(&self, stm: Side) -> (f32, f32) {
        if let Some(movetime) = self.movetime {
            return (movetime, movetime);
        }

        let (time, inc) = match stm {
            Side::White => (self.wtime, self.winc),
            Side::Black => (self.btime, self.binc),
        };

        if time.is_none() {
            return (1_000_000.0, 1_000_000.0);
        }

        let time = time.unwrap();
        let inc = inc.unwrap_or(0.0);

        let soft = (time / 40.0 + inc * 0.8).max(0.05);
        let hard = (soft * 4.0).min(time * 0.25);

        return (soft, hard);
    }

    fn limits(&self, stm: Side) -> search::Limits {
        let (soft_time, hard_time) = self.allocate_time(stm);

        let nodes = self.nodes.unwrap_or(1_000_000_000_000);

        search::Limits {
            hard_nodes: nodes,
            soft_nodes: nodes,
            hard_time: hard_time * 0.9,
            soft_time: soft_time * 0.9,
            depth: self.depth.unwrap_or(255) as i32,
        }
    }
}

fn bench_main() {
    let pos = Position::from_fen(KIWIPETE_FEN).unwrap();

    let mut worker = search::Worker::new();
    let mut limits = search::Limits::new();

    limits.depth = 6;

    worker.go(&pos, limits, Arc::new(atomic::AtomicBool::new(false)));

    let nodes = worker.nodes();
    let elapsed = worker.elapsed();

    let nps = nodes as f32 / elapsed;

    println!("{} nodes {} nps", nodes, nps as usize);
}
