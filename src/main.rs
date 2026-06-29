use noggin::*;

fn main() {
    let pos = Position::from_fen(KIWIPETE_FEN).unwrap();
    println!("{}", pos.fen());
}
