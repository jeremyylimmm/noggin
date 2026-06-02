pub mod movegen;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Piece {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Copy, Clone)]
pub enum Side {
    White,
    Black
}

#[derive(Copy, Clone)]
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
        if matches!(self, Piece::None) {
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

pub struct Position {
    pub bb: [u64; 12],
    pub board: [Piece; 64],
    pub ep_sq: Option<usize>,
    pub castling: u8,
    pub to_move: Side,
    pub halfmove_clock: usize,
    pub fullmoves: usize
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
            fullmoves
        })
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
        println!("To move: {}", if matches!(self.to_move, Side::White) {"w"} else {"b"});
        println!("En passant square: {}", if let Some(ep) = self.ep_sq {sq_to_san(ep).unwrap()} else {"-".to_string()});

        let castle_flag = |flag: u8, val| {
            if (self.castling & flag) != 0 {val} else {""}
        };

        println!("Castling: {}{}{}{}", castle_flag(WK_CASTLE, "K"), castle_flag(WQ_CASTLE, "Q"), castle_flag(BK_CASTLE, "k"), castle_flag(BQ_CASTLE, "q"));

        println!("Halfmove clock: {}", self.halfmove_clock);
        println!("Fullmoves: {}", self.fullmoves);
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