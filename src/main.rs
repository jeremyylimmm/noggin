use noggin::*;

fn main() {
    let pos = Position::from_fen(KIWIPETE_FEN).unwrap();

    let mut worker = search::Worker::new();
    worker.go(&pos);
    println!("bestmove {}", worker.pv()[0]);
}
