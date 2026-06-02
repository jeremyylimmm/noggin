use chengine::{KIWIPETE_FEN, Position};

fn main() {
    let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
    pos.splitperft(2);
}
