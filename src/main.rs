
use core::f32;

use noggin::*;

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
    infinite: bool
}

fn parse_ms(val: &str) -> Result<f32, String> {
    let ms = val.parse::<u32>().map_err(|_|format!("invalid time value '{}'", val))?;
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
            infinite: false
        };

        while i < args.len() {
            if args[i] == "wtime" {
                let &wtime_str = args.get(i+1).ok_or("expected wtime value")?;
                params.wtime = parse_ms(wtime_str)?;
                i += 2;
            }
            else if args[i] == "btime" {
                let &btime_str = args.get(i+1).ok_or("expected btime value")?;
                params.btime = parse_ms(btime_str)?;
                i += 2;
            }
            else if args[i] == "winc" {
                let &winc_str = args.get(i+1).ok_or("expected winc value")?;
                params.winc = parse_ms(winc_str)?;
                i += 2;
            }
            else if args[i] == "binc" {
                let &binc_str = args.get(i+1).ok_or("expected binc value")?;
                params.binc = parse_ms(binc_str)?;
                i += 2;
            }
            else if args[i] == "movetime" {
                let &movetime_str = args.get(i+1).ok_or("expected movetime value")?;
                params.movetime = Some(parse_ms(movetime_str)?);
                i += 2;
            }
            else if args[i] == "nodes" {
                let &nodes_str = args.get(i+1).ok_or("expected nodes value")?;
                params.nodes = nodes_str.parse::<usize>().map_err(|_|format!("invalid nodes value '{}'", nodes_str))?;
                i += 2;
            }
            else if args[i] == "depth" {
                let &depth_str = args.get(i+1).ok_or("expected depth value")?;
                params.depth = depth_str.parse::<usize>().map_err(|_|format!("invalid depth value '{}'", depth_str))?;
                i += 2;
            }
            else if args[i] == "infinite" {
                params.infinite = true;
                i += 1;
            }
            else {
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

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(option) = args.get(1) {
        if option == "bench" {
            bench_main();
            return;
        }
    }

    let mut pos = Position::from_fen(STARTING_FEN).unwrap();
    let mut s = search::Searcher::new();
    let mut handle: Option<std::thread::JoinHandle<()>> = None;

    macro_rules! stop {
        () => {
            s.stop();

            if let Some(h) = handle.take() {
                h.join().unwrap();
            }
        };
    }

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_string();
        let args: Vec<&str> = input.split(' ').collect();

        if args[0] == "uci" {
            stop!();
            println!("id name Noggin");
            println!("id author Noggin Authors");
            println!("option name Hash type spin default 1 min 1 max 16");
            println!("option name Threads type spin default 1 min 1 max 1");
            println!("uciok");
        }
        else if args[0] == "isready" {
            stop!();
            println!("readyok");
        }
        else if args[0] == "quit" {
            stop!();
            return;
        }
        else if args[0] == "ucinewgame" {
            stop!();
            pos = Position::from_fen(STARTING_FEN).unwrap();
            s = search::Searcher::new();
        }
        else if args[0] == "position" {
            stop!();

            if args.len() < 2 {
                println!("specify 'startpos' or 'fen'");
                continue
            }

            let mut next = if args[1] == "startpos" {
                pos = Position::from_fen(STARTING_FEN).unwrap();
                2
            }
            else if args[1] == "fen" {
                if args.len() < 8 {
                    println!("expected a FEN string");
                    continue;
                }

                let fen = args[2..8].join(" ");

                pos = if let Ok(p) = Position::from_fen(&fen) {
                    p
                }
                else {
                    println!("malformed FEN '{}'", fen);
                    continue;
                };

                8
            }
            else {
                println!("expected 'startpos' or 'fen', got '{}'", args[1]);
                continue;
            };

            if args.len() <= next {
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
                }
                else {
                    println!("malformed move '{}'", uci_mv);
                    break;
                };

                if !is_legal(&mut pos, mv) {
                    println!("illegal move '{}'", uci_mv);
                    break;
                }

                pos.make_move(mv);
            }
        }
        else if args[0] == "go" {
            stop!();

            match GoParameters::parse(&args) {
                Ok(params) => {
                    let (soft_time, hard_time) = allocate_time(&params, pos.to_move);

                    s.reset(hard_time, soft_time, params.nodes, params.nodes);

                    let mut pos_copy  = pos.clone();
                    let mut s_copy= s.clone();

                    handle = Some(std::thread::spawn(move ||{
                        let mv = s_copy.best(&mut pos_copy, params.depth as _);
                        println!("bestmove {}", mv.uci_string());
                    }));
                }

                Err(e) => {
                    println!("invalid go command: {}", e);
                }
            }
        }
        else if args[0] == "stop" {
            stop!();
        }
        else if args[0] == "setoption" {
            stop!();
        }
        else {
            stop!();
            println!("unrecognized command '{}'", input);
        }
    }
}

fn bench_main() {
    let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
    let mut s = search::Searcher::new();

    s.reset(f32::INFINITY, f32::INFINITY, 1024*1024*1024, 1024*1024*1024);
    s.best(&mut pos, 5);

    let nps = s.nodes() as f32 / s.elapsed(); 

    println!("{} nodes {} nps", s.nodes(), nps.round() as usize);
}