use noggin::*;

fn main() {
    let pos = Position::from_fen(STARTPOS_FEN).unwrap();

    let moves = pos.gen_psuedolegal_moves();

    for mv in &moves {
        println!("{}", mv);
    }

    println!("{} moves", moves.len());
}
