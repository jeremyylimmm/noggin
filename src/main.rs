use chengine::Position;

fn main() {
    //let pos = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let pos = Position::from_fen("8/8/3k4/8/8/K7/8/8 w - - 0 1").unwrap();
    pos.dump();

    let moves = chengine::movegen::gen_pseudolegal_moves(&pos);

    for i in 0..moves.len() {
        let mv = moves[i];
        println!("{}", mv.uci_name());
    }
}
