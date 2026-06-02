use chengine::{KIWIPETE_FEN, Position};

fn main() {
    let pos = Position::from_fen(KIWIPETE_FEN).unwrap();
    pos.dump();

    let moves = chengine::movegen::gen_pseudolegal_moves(&pos);

    for i in 0..moves.len() {
        let mv = moves[i];
        println!("{}", mv.uci_string());
    }
}
