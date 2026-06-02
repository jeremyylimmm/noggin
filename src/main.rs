use chengine::*;

fn main() {
    //let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();

    let mut pos = Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/PPN2Q2/2PBBP1P/R3K2r w Qkq - 0 1").unwrap();
    let mv = parse_uci_move("f3h1").unwrap();
    pos.make_move(mv);

    pos.dump();

    //let mut pos = Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/PPN5/2PBBP1P/R3K2Q b Qkq - 0 1").unwrap();
    pos.splitperft(1);
}
