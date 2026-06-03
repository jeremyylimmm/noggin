use chengine::*;

fn main() {
    let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
    let s = search::Searcher {};

    let mv = s.best(&mut pos, 4);
    println!("{}", mv.uci_string());
}
