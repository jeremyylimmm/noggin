use noggin::*;

fn main() {
    let pos = Position::from_fen(STARTPOS_FEN).unwrap();

    for mv in pos.gen_psuedolegal_moves() {
        println!("{}", mv);
    }
}
