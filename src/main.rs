use noggin::*;

fn main() {
    let pos =
        Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/P1P1PPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    println!("{}", pos.debug_str());

    let moves = pos.gen_psuedolegal_moves();

    for mv in &moves {
        println!("{}", mv);
    }

    println!("{} moves", moves.len());
}
