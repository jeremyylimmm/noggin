pub struct PackedBoard {
    occ: u64,
    pieces: [u64; 2],
    stm_ep: u8,
    halfmove_clock: u8,
    fullmoves: u16,
    score: i16,
    result: u8,
}

#[derive(Copy, Clone)]
pub struct Move(u16);

impl Move {
    fn new(from: u16, to: u16, prom: u16, ty: u16) -> Self {
        let word = (from & 63) | ((to & 63) << 6) | ((prom & 0b11) << 12) | ((ty & 0b11) << 14);
        Self(word)
    }

    pub fn from_native(pos: &noggin::Position, mv: noggin::Move) -> Self {
        let piece = pos.board[mv.from()];

        let from = mv.from() as u16;
        let mut to = mv.to() as u16;
        let prom = match mv.promotion() {
            noggin::Piece::Knight => 0,
            noggin::Piece::Bishop => 1,
            noggin::Piece::Rook => 2,
            noggin::Piece::Queen => 3,
            _ => 0,
        };

        let ty = match piece {
            noggin::Piece::Pawn => {
                if pos.is_capture(mv).is_some() && pos.board[mv.to()] == noggin::Piece::None {
                    1
                } else if mv.promotion() != noggin::Piece::None {
                    3
                } else {
                    0
                }
            }
            noggin::Piece::King => {
                if let Some((rook_from, _)) = noggin::is_castle(mv, piece) {
                    to = rook_from as u16;
                    2
                } else {
                    0
                }
            }
            _ => 0,
        };

        Self::new(from, to, prom, ty)
    }
}

pub struct Game {
    pub board: PackedBoard,
    pub seq: Vec<(Move, i16)>,
}

impl Game {
    pub fn dump(&self, file: &mut std::fs::File) {
        use std::io::Write;

        file.write_all(&self.board.occ.to_le_bytes()).unwrap();

        let pieces: Vec<u8> = self
            .board
            .pieces
            .iter()
            .map(|x| x.to_le_bytes())
            .flatten()
            .collect();

        file.write_all(&pieces).unwrap();

        file.write_all(&self.board.stm_ep.to_le_bytes()).unwrap();
        file.write_all(&self.board.halfmove_clock.to_le_bytes())
            .unwrap();
        file.write_all(&self.board.fullmoves.to_le_bytes()).unwrap();
        file.write_all(&self.board.score.to_le_bytes()).unwrap();
        file.write_all(&self.board.result.to_le_bytes()).unwrap();
        file.write_all(&[0u8]).unwrap();

        for (mv, score) in &self.seq {
            file.write_all(&mv.0.to_le_bytes()).unwrap();
            file.write_all(&score.to_le_bytes()).unwrap();
        }

        file.write_all(&0u32.to_le_bytes()).unwrap();
    }
}

impl PackedBoard {
    pub fn from_position(pos: &mut noggin::Position, result: u8) -> Self {
        let black_occ = pos.bb[6..].iter().fold(0, |acc, x| acc | x);

        let pieces: Vec<u8> = pos
            .board
            .iter()
            .enumerate()
            .map(|(sq, &p)| {
                let side = if ((1u64 << sq) & black_occ) != 0 {
                    noggin::Side::Black
                } else {
                    noggin::Side::White
                };

                let (qcastle, kcastle) = if side == noggin::Side::Black {
                    (noggin::BQ_CASTLE, noggin::BK_CASTLE)
                } else {
                    (noggin::WQ_CASTLE, noggin::WK_CASTLE)
                };

                let can_qcastle = (pos.castling & qcastle) != 0;
                let can_kcastle = (pos.castling & kcastle) != 0;

                match p {
                    noggin::Piece::None => None,
                    noggin::Piece::Pawn => Some(0),
                    noggin::Piece::Knight => Some(1),
                    noggin::Piece::Bishop => Some(2),
                    noggin::Piece::Rook => Some(match (side, sq, can_qcastle, can_kcastle) {
                        (noggin::Side::White, 0, true, _) => 6,
                        (noggin::Side::White, 7, _, true) => 6,
                        (noggin::Side::Black, 56, true, _) => 6,
                        (noggin::Side::Black, 63, _, true) => 6,
                        _ => 3,
                    }),
                    noggin::Piece::Queen => Some(4),
                    noggin::Piece::King => Some(5),
                }
                .map(|x| x | (side.id() as u8) << 3)
            })
            .flatten()
            .collect();

        let mut pieces_packed = [0u64; 2];

        for (i, &p) in pieces.iter().enumerate() {
            let bit_idx = i * 4;
            pieces_packed[bit_idx / 64] |= (p as u64) << (bit_idx % 64);
        }

        let ep = if let Some(sq) = pos.ep_sq {
            sq & 63
        } else {
            64
        } as u8;

        Self {
            occ: pos.occ(),
            pieces: pieces_packed,
            stm_ep: (ep as u8) | ((pos.to_move.id() as u8) << 7),
            halfmove_clock: pos.halfmove_clock as _,
            fullmoves: pos.fullmoves as _,
            score: pos.eval() as i16,
            result,
        }
    }
}
