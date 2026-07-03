use noggin::*;

use std::sync::*;

enum State {
    Idle(search::Worker),
    Working(std::thread::JoinHandle<search::Worker>, Arc<atomic::AtomicBool>)
}

fn main() {
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

            let stop = Arc::new(atomic::AtomicBool::new(false));

            let pos = pos.clone();
            let stop_copy = stop.clone();

            let join_handle = std::thread::spawn(move ||{
                worker.go(&pos, stop_copy);
                println!("bestmove {}", worker.pv()[0]);
                return worker;
            });

            state = State::Working(join_handle, stop);
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