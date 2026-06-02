use chengine::Position;

fn main() {
    //let pos = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let pos = Position::from_fen("8/5P2/3P4/8/2k2K2/8/8/8 w - - 0 1").unwrap();
    pos.dump();

    let moves = chengine::movegen::gen_pseudolegal_moves(&pos);

    for i in 0..moves.len() {
        let mv = moves[i];
        println!("{}", mv.uci_string());
    }
}
