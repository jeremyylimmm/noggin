
use chengine::*;

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

fn main() {
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
            println!("id name Chengine");
            println!("id author Chengine Authors");
            println!("option name Hash type spin default 1 min 1 max 1");
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

                let mv = if let Some(mv) = parse_uci_move(uci_mv) {
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

            s.reset();

            let mut pos_copy  = pos.clone();
            let mut s_copy= s.clone();

            handle = Some(std::thread::spawn(move ||{
                let mv = s_copy.best(&mut pos_copy, 5);
                println!("bestmove {}", mv.uci_string());
            }));
        }
        else if args[0] == "stop" {
            stop!();
        }
        else {
            stop!();
            println!("unrecognized command '{}'", input);
        }
    }
}
