use noggin::*;

fn main() {
    let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();

    println!("{}", pos.debug_str());

    for mv_uci in [] {
        let mv = Move::from_uci(mv_uci).unwrap();
        pos = pos.make_move(mv);
        println!("{}", pos.debug_str());
    }

    pos.split_perft(6);
}
