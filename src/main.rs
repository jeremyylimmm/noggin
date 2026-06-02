use chengine::Position;

fn main() {
    //let pos = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let pos = Position::from_fen("rnbqkbnr/ppp1ppp1/7p/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3").unwrap();
    pos.dump();

    let moves = chengine::movegen::gen_pseudolegal_moves(&pos);

    for i in 0..moves.len() {
        let mv = moves[i];
        println!("{}", mv.uci_name());
    }
}
