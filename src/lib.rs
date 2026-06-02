pub mod movegen;

pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_B: u64 = 0x0202020202020202;
pub const FILE_C: u64 = 0x0404040404040404;
pub const FILE_D: u64 = 0x0808080808080808;
//pub const FILE_E: u64 = 0x1010101010101010;
pub const FILE_F: u64 = 0x2020202020202020;
pub const FILE_G: u64 = 0x4040404040404040;
pub const FILE_H: u64 = 0x8080808080808080;

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const KIWIPETE_FEN: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Piece {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl Side {
    fn opp(&self) -> Self {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Side {
    White,
    Black
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Move(u16);

impl Move {
    pub fn new(from: usize, to: usize, promotion: Piece) -> Self {
        Self(
            (from & 0b111111) as u16 | ((to & 0b111111) << 6) as u16 | ((promotion as u16 & 0b111) << 12)
        )
    }

    fn from(&self) -> usize {
        (self.0 & 0b111111) as usize
    }

    fn to(&self) -> usize {
        ((self.0 >> 6) & 0b111111) as usize
    }

    fn promotion(&self) -> Piece {
        let x = (self.0 >> 12) & 0b111;

        match x {
            0 => Piece::None,
            1 => Piece::Pawn,
            2 => Piece::Knight,
            3 => Piece::Bishop,
            4 => Piece::Rook,
            5 => Piece::Queen,
            6 => Piece::King,
            _ => panic!("invalid promotion piece in move encoding")
        }
    }

    pub fn uci_string(&self) -> String {
        let promotion_str = match self.promotion() {
            Piece::None => "",
            Piece::Pawn => "=P",
            Piece::Knight => "=N",
            Piece::Bishop => "=B",
            Piece::Rook => "=R",
            Piece::Queen => "=Q",
            Piece::King => "=K",
        };

        format!("{}{}{}", sq_to_san(self.from()).unwrap(), sq_to_san(self.to()).unwrap(), promotion_str)
    }
}

impl Piece {
    pub fn bb_index(&self, side: Side) -> Option<usize> {
        if *self == Piece::None {
            None
        }
        else {
            Some(match side {
                Side::White => *self as usize - 1,
                Side::Black => *self as usize - 1 + 6
            })
        }
    }
}

const WQ_CASTLE: u8 = 1 << 0;
const WK_CASTLE: u8 = 1 << 1;
const BQ_CASTLE: u8 = 1 << 2;
const BK_CASTLE: u8 = 1 << 3;

#[derive(Clone, PartialEq, Debug)]
struct Undo {
    mv: Move,
    capture_piece: Option<Piece>,
    ep_sq: Option<usize>,
    castling: u8,
    to_move: Side,
    halfmove_clock: usize,
    fullmoves: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Position {
    pub bb: [u64; 12],
    pub board: [Piece; 64],
    pub ep_sq: Option<usize>,
    pub castling: u8,
    pub to_move: Side,
    pub halfmove_clock: usize,
    pub fullmoves: usize,
    undos: Vec<Undo>
}

fn letter_to_file(l: char) -> Option<usize> {
    match l {
        'a' => Some(0),
        'b' => Some(1),
        'c' => Some(2),
        'd' => Some(3),
        'e' => Some(4),
        'f' => Some(5),
        'g' => Some(6),
        'h' => Some(7),
        _ => None
    }
}

impl Position {
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let mut cur = fen.chars().peekable();




        // parse piece placement

        let mut board_side = [Side::White; 64];
        let mut board = [Piece::None; 64];

        for rank in (0..8).rev() {
            if rank < 7 {
                if !matches!(cur.next(), Some('/')) {
                    return Err("expected a '/' between ranks".to_string());
                }
            }

            let mut file = 0usize;
            
            while file < 8 {
                let sq = rank*8+file;

                let c = cur.next().ok_or("unexpected end of FEN: rank not completed".to_string())?;

                match c {
                    skip @ '1'..='8' => {
                        file += skip.to_digit(10).unwrap() as usize;
                    }

                    x => {
                        let piece = match x {
                            'p'|'P' => Piece::Pawn,
                            'n'|'N' => Piece::Knight,
                            'b'|'B' => Piece::Bishop,
                            'r'|'R' => Piece::Rook,
                            'q'|'Q' => Piece::Queen,
                            'k'|'K' => Piece::King,
                            _ => {
                                return Err(format!("unexpected character '{}'", x));
                            }
                        };

                        board_side[sq] = if x.is_lowercase() {Side::Black} else {Side::White};
                        board[sq] = piece;

                        file += 1;
                    }
                }
            }

        }






        // fill bitboards

        let mut bb = [0;12];

        for sq in 0..64 {
            let side = board_side[sq];

            match board[sq] {
                Piece::None => {},
                p => {
                    bb[p.bb_index(side).unwrap()] |= 1u64 << sq;
                }
            }
        } 


        if !matches!(cur.next(), Some(' ')) {
            return Err("unexpected end of FEN: expected side-to-move".to_string());
        }


        // parse side to move

        let to_move_char = cur.next().ok_or("expected side to move ('w' or 'b')".to_string())?;

        let to_move = match to_move_char {
            'w' => Side::White,
            'b' => Side::Black,
            c => return Err(format!("invalid side-to-move given '{}'", c))
        };




        if !matches!(cur.next(), Some(' ')) {
            return Err("unexpected end of FEN: expected castling availability".to_string());
        }


        // parse castling flags
        
        let mut castling = 0;
        
        let mut set_castling = |flag| {
            if (castling & flag) != 0 {
                return Err("invalid castling availability flags".to_string());
            }

            castling |= flag;

            return Ok(());
        };

        match cur.peek() {
            None => {
                return Err("unexpected end of FEN: expected castling availability".to_string());
            }

            Some('-') => {cur.next().unwrap();},

            Some(_) => {
                while let Some(&c) = cur.peek() {
                    if c.is_whitespace() {
                        break;
                    }

                    match cur.next().unwrap() {
                        'q' => set_castling(BQ_CASTLE)?,
                        'Q' => set_castling(WQ_CASTLE)?,
                        'k' => set_castling(BK_CASTLE)?,
                        'K' => set_castling(WK_CASTLE)?,
                        x => return Err(format!("invalid castling flag '{}'", x))
                    }
                }
            }
        }




        if !matches!(cur.next(), Some(' ')) {
            return Err("unexpected end of FEN: expected en passant square".to_string());
        }


        // parse en passant square

        let mut ep_sq = None;
        
        match cur.peek() {
            None => return Err("unexpected end of FEN: expected en passant square".to_string()),
            Some('-') => {cur.next().unwrap();},
            Some(_) => {
                let fl = cur.next().unwrap();
                let file = letter_to_file(fl).ok_or(format!("invalid en passant square file '{}'", fl))?;

                let rc = cur.next().ok_or(format!("unexpected end of FEN: expected an en passant square rank"))?;
                let d = rc.to_digit(10).ok_or(format!("invalid en passant square rank '{}'", rc))? as usize;

                if d < 1 || d > 8 {
                    return Err(format!("en passant square rank '{}' is not valid", d));
                }

                let rank = d - 1;

                ep_sq = Some(rank*8+file);
            }
        }


        if !matches!(cur.next(), Some(' ')) {
            return Err("unexpected end of FEN: expected space before halfmove clock".to_string());
        }

        if !matches!(cur.peek(), Some('0'..='9')) {
            return Err("unexpected end of FEN: expected halfmove clock".to_string());
        }

        let mut halfmove_clock= 0;

        while let Some(c) = cur.next() {
            if c == ' ' {
                break;
            }

            let x = c.to_digit(10).ok_or(format!("unexpected halfmove clock character '{}'", c))?;

            halfmove_clock *= 10;
            halfmove_clock += x as usize;
        }



        if !matches!(cur.peek(), Some('0'..='9')) {
            return Err("unexpected end of FEN: expected fullmove number".to_string());
        }

        let mut fullmoves= 0;

        while let Some(c) = cur.next() {
            let x = c.to_digit(10).ok_or(format!("unexpected fullmove number character '{}'", c))?;

            fullmoves *= 10;
            fullmoves += x as usize;
        }


        Ok(Position{
            bb,
            board,
            ep_sq,
            castling,
            to_move,
            halfmove_clock,
            fullmoves,
            undos: vec![]
        })
    }

    pub fn checked(&self, side: Side) -> bool {
        let king_sq = self.bb[Piece::King.bb_index(side).unwrap()].trailing_zeros();
        self.sq_attacked(king_sq as _, side.opp()) 
    }

    pub fn sq_attacked(&self, sq: usize, attacker: Side) -> bool {
        let occ = self.bb.iter().fold(0, |acc, x|acc|x);

        let pawn_attacks = match attacker {
            Side::White => black_pawn_attacks(1u64 << sq),
            Side::Black => white_pawn_attacks(1u64 << sq),
        };

        let pawns = self.bb[Piece::Pawn.bb_index(attacker).unwrap()];

        if (pawn_attacks & pawns) != 0 {
            return true;
        }

        let knights = self.bb[Piece::Knight.bb_index(attacker).unwrap()];

        if (knight_attacks(sq as u32) & knights) != 0 {
            return true;
        }

        let bishops = self.bb[Piece::Bishop.bb_index(attacker).unwrap()];
        let bishop_attck = bishop_attacks(sq as u32, occ);

        if (bishop_attck & bishops) != 0 {
            return true;
        }

        let rooks = self.bb[Piece::Rook.bb_index(attacker).unwrap()];
        let rook_attck = rook_attacks(sq as u32, occ);

        if (rook_attck & rooks) != 0 {
            return true;
        }

        let queens = self.bb[Piece::Queen.bb_index(attacker).unwrap()];

        if ((bishop_attck | rook_attck) & queens) != 0 {
            return true;
        }

        let kings = self.bb[Piece::King.bb_index(attacker).unwrap()];

        if (king_attacks(sq as u32) & kings) != 0 {
            return true;
        }

        return false;
    }

    pub fn dump(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let sq = rank*8+file;
                let piece = self.board[sq];

                let mut c = match piece {
                    Piece::Pawn => 'p',
                    Piece::Knight => 'n',
                    Piece::Bishop => 'b',
                    Piece::Rook => 'r',
                    Piece::Queen => 'q',
                    Piece::King => 'k',
                    Piece::None => '.',
                };

                let is_white = if let Some(bb_idx) = piece.bb_index(Side::White) {
                    (self.bb[bb_idx] & (1u64 << sq)) != 0 
                }
                else {
                    false
                };

                if is_white {
                    c = c.to_ascii_uppercase();
                }

                print!("{} ", c);
            }

            println!("");
        }

        println!("---------------");
        println!("To move: {}", if self.to_move == Side::White {"w"} else {"b"});
        println!("En passant square: {}", if let Some(ep) = self.ep_sq {sq_to_san(ep).unwrap()} else {"-".to_string()});

        let castle_flag = |flag: u8, val| {
            if (self.castling & flag) != 0 {val} else {""}
        };

        println!("Castling: {}{}{}{}", castle_flag(WK_CASTLE, "K"), castle_flag(WQ_CASTLE, "Q"), castle_flag(BK_CASTLE, "k"), castle_flag(BQ_CASTLE, "q"));

        println!("Halfmove clock: {}", self.halfmove_clock);
        println!("Fullmoves: {}", self.fullmoves);
    }

    pub fn make_move(&mut self, mv: Move) {
        let mut undo = Undo{
            mv,
            capture_piece: None,
            ep_sq: self.ep_sq,
            castling: self.castling,
            to_move: self.to_move,
            halfmove_clock: self.halfmove_clock,
            fullmoves: self.fullmoves
        };

        let from = mv.from();
        let to = mv.to();

        let from_rank = (from >> 3) & 7;
        let from_file = from & 7;

        let to_file = to & 7;

        let to_rank = (to >> 3) & 7;

        let start = self.board[from];
        let end = match mv.promotion() {
            Piece::None => start,
            x => x
        };




        // remove moving piece

        self.board[from] = Piece::None;
        self.bb[start.bb_index(self.to_move).unwrap()] ^= 1u64 << from;





        // remove captured piece

        let capture_sq = self.capture_sq(mv, start, self.to_move);
        let capture_piece = self.board[capture_sq];

        if capture_piece != Piece::None {
            undo.capture_piece = Some(capture_piece);

            self.bb[capture_piece.bb_index(self.to_move.opp()).unwrap()] ^= 1u64 << capture_sq;
            self.board[capture_sq] = Piece::None;
        }




        // add moving piece

        debug_assert!(self.board[to] == Piece::None);
        self.board[to] = end;
        self.bb[end.bb_index(self.to_move).unwrap()] ^= 1u64 << to;



        // move rook if castling

        if let Some((rook_from, rook_to)) = is_castle(mv, start) {
            self.bb[Piece::Rook.bb_index(self.to_move).unwrap()] ^= (1u64 << rook_from) | (1u64 << rook_to);

            debug_assert!(self.board[rook_from] == Piece::Rook);

            self.board[rook_from] = Piece::None;
            self.board[rook_to] = Piece::Rook;
        }




        // set en passant sq

        let rank_diff = from_rank.abs_diff(to_rank);

        if start == Piece::Pawn && rank_diff > 1 { // double push
            self.ep_sq = Some(match self.to_move {
                Side::White => to - 8,
                Side::Black => to + 8
            });
        }
        else {
            self.ep_sq = None;
        }




        // handle castling rights

        let (kcastle_flag, qcastle_flag, opp_kcastle_flag, opp_qcastle_flag) = match self.to_move {
            Side::White => (WK_CASTLE, WQ_CASTLE, BK_CASTLE, BQ_CASTLE),
            Side::Black => (BK_CASTLE, BQ_CASTLE, WK_CASTLE, WQ_CASTLE),
        };

        let can_kcastle     = (self.castling & kcastle_flag) != 0;
        let can_qcastle     = (self.castling & qcastle_flag) != 0;
        let opp_can_kcastle = (self.castling & opp_kcastle_flag) != 0;
        let opp_can_qcastle = (self.castling & opp_qcastle_flag) != 0;

        // handle if we move a king or a rook

        match (start, from_file, can_kcastle, can_qcastle) {
            (Piece::King, _, _, _) => {
                self.castling &= !(kcastle_flag | qcastle_flag);
            }

            (Piece::Rook, 0, _, true)  => {
                self.castling &= !qcastle_flag;
            }

            (Piece::Rook, 7, true, _)  => {
                self.castling &= !kcastle_flag;
            }

            _ => {}
        }

        // handle if we capture the other player's rook

        match (capture_piece, to_file, opp_can_kcastle, opp_can_qcastle) {
            (Piece::Rook, 0, _, true) => {
                self.castling &= !opp_qcastle_flag;
            }

            (Piece::Rook, 7, true, _) => {
                self.castling &= !opp_kcastle_flag;
            }

            _ => {}
        }



        // update halfmove clock

        self.halfmove_clock += 1;

        let is_capture = capture_piece != Piece::None; 

        if start == Piece::Pawn || is_capture {
            self.halfmove_clock = 0;
        }



        // update move number

        if self.to_move == Side::Black {
            self.fullmoves += 1;
        }




        // finally, update to-move

        self.to_move = self.to_move.opp();


        // push to the undo stack

        self.undos.push(undo);
    }

    pub fn unmake_move(&mut self) {
        let undo = self.undos.pop().unwrap();

        let mv = undo.mv;

        self.ep_sq = undo.ep_sq;
        self.castling = undo.castling;
        self.to_move = undo.to_move;
        self.halfmove_clock = undo.halfmove_clock;
        self.fullmoves = undo.fullmoves;

        let end = self.board[mv.to()];

        let start = match mv.promotion() {
            Piece::None => end,
            _ => Piece::Pawn
        };

        // move rook if castle

        if let Some((rook_from, rook_to)) = is_castle(mv, start) {
            self.board[rook_to] = Piece::None;
            self.board[rook_from] = Piece::Rook;
            self.bb[Piece::Rook.bb_index(self.to_move).unwrap()] ^= (1u64 << rook_from) | (1u64 << rook_to);
        }

        // remove piece

        self.board[mv.to()] = Piece::None;
        self.bb[end.bb_index(self.to_move).unwrap()] ^= 1u64 << mv.to();

        // add back captured piece

        if let Some(capture_piece) = undo.capture_piece {
            let capture_sq = self.capture_sq(mv, start, self.to_move);
            self.board[capture_sq] = capture_piece;
            self.bb[capture_piece.bb_index(self.to_move.opp()).unwrap()] ^= 1u64 << capture_sq;
        }

        // add back moving piece

        self.board[mv.from()] = start;
        self.bb[start.bb_index(self.to_move).unwrap()] ^= 1u64 << mv.from();
    }

    fn capture_sq(&self, mv: Move, piece: Piece, side: Side) -> usize {
        let from_file = mv.from() & 7;
        let to_file = mv.to() & 7;

        let to = mv.to();

        let is_pawn = piece == Piece::Pawn;
        let different_file = from_file != to_file;
        let to_is_ep_sq = matches!(self.ep_sq, Some(x) if x == to);

        if is_pawn && different_file && to_is_ep_sq {
            match side {
                Side::White => to - 8,
                Side::Black => to + 8,
            }
        }
        else {
            to
        }
    }

    pub fn perft(&mut self, depth: isize) -> usize {
        if depth == 0 {
            return 1;
        }

        let moves = movegen::gen_pseudolegal_moves(self);

        let mut count = 0;

        let side = self.to_move;

        for i in 0..moves.len() {
            let mv = moves[i];

            self.make_move(mv);

            if !self.checked(side) {
                count += self.perft(depth-1);
            }

            self.unmake_move();
        }

        count
    }
}

fn sq_to_san(sq: usize) -> Option<String> {
    if sq >= 64 {
        None
    }
    else {
        let file = sq & 0b111;
        let rank = (sq >> 3) & 0b111;

        const FILE_LETTERS: [char;8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

        Some(format!("{}{}", FILE_LETTERS[file], rank+1))
    }
}

fn white_pawn_attacks(pawns: u64) -> u64 {
    let left = (pawns << 7) & !(FILE_H);
    let right = (pawns << 9) & !(FILE_A);

    left | right
}

fn black_pawn_attacks(pawns: u64) -> u64 {
    let left = (pawns >> 9) & !(FILE_H);
    let right = (pawns >> 7) & !(FILE_A);

    left | right
}

pub fn knight_attacks(from: u32) -> u64 {
    let knight = 1u64 << from;

    let m0 = knight <<  6 & !(FILE_G | FILE_H);
    let m1 = knight << 15 & !(FILE_H);
    let m2 = knight << 17 & !(FILE_A);
    let m3 = knight << 10 & !(FILE_A | FILE_B);
    let m4 = knight >>  6 & !(FILE_A | FILE_B);
    let m5 = knight >> 15 & !(FILE_A);
    let m6 = knight >> 17 & !(FILE_H);
    let m7 = knight >> 10 & !(FILE_G | FILE_H);

    m0|m1|m2|m3|m4|m5|m6|m7
}

pub fn king_attacks(from: u32) -> u64 {
    let king = 1u64 << from;

    let m0 = (king << 7) & !FILE_H;
    let m1 =  king << 8;
    let m2 = (king << 9) & !FILE_A;
    let m3 = (king << 1) & !FILE_A;
    let m4 = (king >> 7) & !FILE_A;
    let m5 =  king >> 8;
    let m6 = (king >> 9) & !FILE_H;
    let m7 = (king >> 1) & !FILE_H;

    m0|m1|m2|m3|m4|m5|m6|m7
}

fn slide_and_gather<F: Fn(u64)->u64>(mut cur: u64, occ: u64, slide: F) -> u64 {
    let mut result = 0;

    loop {
        cur = slide(cur);

        result |= cur;

        if cur & occ != 0 || cur == 0 {
            break;
        }
    }

    result
}

pub fn rook_attacks(from: u32, occ: u64) -> u64 {
    let rook = 1u64 << from;

    let up = slide_and_gather(rook, occ, |x|x<<8);
    let right = slide_and_gather(rook, occ, |x|x<<1 & !FILE_A);
    let down = slide_and_gather(rook, occ, |x|x>>8);
    let left = slide_and_gather(rook, occ, |x|x>>1 & !FILE_H);

    up | right | down | left
}

pub fn bishop_attacks(from: u32, occ: u64) -> u64 {
    let bishop = 1u64 << from;

    let left_up = slide_and_gather(bishop, occ, |x|x<<7 & !FILE_H);
    let right_up = slide_and_gather(bishop, occ, |x|x<<9 & !FILE_A);
    let right_down = slide_and_gather(bishop, occ, |x|x>>7 & !FILE_A);
    let left_down = slide_and_gather(bishop, occ, |x|x>>9 & !FILE_H);

    left_up | right_up | right_down | left_down
}


fn is_castle(mv: Move, piece: Piece) -> Option<(usize, usize)> {
    let from_file = mv.from() & 7;
    let to_file = mv.to() & 7;

    let castle = piece == Piece::King && from_file.abs_diff(to_file) > 1;

    if !castle {
        return None;
    }

    let rank = (mv.from() >> 3) & 7;

    Some(match mv.to() & 7 {
        2 => (rank*8+0, rank*8+3),
        6 => (rank*8+7, rank*8+5),
        _ => panic!("invalid king move")
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_and_unmake_move() {
        let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
        let baseline = pos.clone();

        let moves = movegen::gen_pseudolegal_moves(&pos);

        for i in 0..moves.len() {
            let mv = moves[i];

            pos.make_move(mv);
            pos.unmake_move();

            assert_eq!(pos, baseline);
        }
    }

    #[test]
    fn test_kiwipete_perft_depth_1() {
        let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
        assert_eq!(pos.perft(1), 48);
    }

    #[test]
    fn test_kiwipete_perft_depth_2() {
        let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
        assert_eq!(pos.perft(2), 2039);
    }

    #[test]
    fn test_kiwipete_perft_depth_3() {
        let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
        assert_eq!(pos.perft(3), 97862);
    }

    #[test]
    fn test_kiwipete_perft_depth_4() {
        let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
        assert_eq!(pos.perft(4), 4085603);
    }

    #[test]
    fn test_kiwipete_perft_depth_5() {
        let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
        assert_eq!(pos.perft(5), 193690690);
    }

    #[test]
    fn test_kiwipete_perft_depth_6() {
        let mut pos = Position::from_fen(KIWIPETE_FEN).unwrap();
        assert_eq!(pos.perft(6), 8031647685);
    }
}