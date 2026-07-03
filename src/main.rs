use noggin::*;

fn main() {
    let mut worker = search::Worker::new();
    let mut pos = Position::from_fen(STARTPOS_FEN).unwrap();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        let args: Vec<_> = input.split_whitespace().collect();

        if args[0] == "uci" {
            println!("id name Noggin 3.0.1");
            println!("id author Noggin Authors");
            println!("option name Hash type spin default 16 min 1 max 4096");
            println!("option name Threads type spin default 1 min 1 max 1");
            println!("uciok");
        } else if args[0] == "isready" {
            println!("readyok");
        } else if args[0] == "quit" {
            return;
        } else if args[0] == "ucinewgame" {
            worker = search::Worker::new();
            pos = Position::from_fen(STARTPOS_FEN).unwrap();
        } else if args[0] == "position" {
            match parse_position_cmd(&args) {
                Ok(p) => pos = p,
                Err(msg) => println!("{}", msg),
            }
        } else if args[0] == "go" {
            worker.go(&pos);
            println!("bestmove {}", worker.pv()[0]);
        } else if args[0] == "stop" {
            // TODO: search on another thread
        } else if args[0] == "setoption" {
            // TODO: handle ts
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
