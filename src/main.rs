use noggin::*;

fn main() {
    let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();

    println!("{}", pos.debug_str());

    for mv_uci in [] {
        let mv = Move::from_uci(mv_uci).unwrap();
        pos = pos.make_move(mv);
        println!("{}", pos.debug_str());
    }

    let start = std::time::Instant::now();
    let nodes = pos.split_perft(6);
    let end = std::time::Instant::now();

    let elapsed = (end - start).as_secs_f32();
    let nps = nodes as f32 / elapsed;

    println!("NPS: {:.3}M", nps / 1_000_000.0);
}
