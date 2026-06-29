use crate::*;

fn parse_piece_placement(piece_placement: &str) -> Result<([u64; 12], Board), String> {
    let mut bbs = [0u64; 12];
    let mut board = [None; 64];

    let mut c = piece_placement.chars();

    for r in (0..8).rev() {
        if r < 7 {
            if !matches!(c.next(), Some('/')) {
                return Err(format!("expected a '/'"));
            }
        }

        let mut f = 0;

        while f < 8 {
            let sq = Sq::from_coords(r, f);

            match c.next() {
                Some(c @ '1'..='8') => {
                    let x = c as usize - '0' as usize;
                    f += x;

                    if f > 8 {
                        return Err(format!("rank {} has too many pieces", r + 1));
                    }
                }

                Some(c @ ('p' | 'P')) => {
                    board[sq] = Some(Piece::Pawn);
                    *bbs.get_mut(Piece::Pawn, side_from_case(c)) |= sq.bb();
                    f += 1;
                }

                Some(c @ ('n' | 'N')) => {
                    board[sq] = Some(Piece::Knight);
                    *bbs.get_mut(Piece::Knight, side_from_case(c)) |= sq.bb();
                    f += 1;
                }

                Some(c @ ('b' | 'B')) => {
                    board[sq] = Some(Piece::Bishop);
                    *bbs.get_mut(Piece::Bishop, side_from_case(c)) |= sq.bb();
                    f += 1;
                }

                Some(c @ ('r' | 'R')) => {
                    board[sq] = Some(Piece::Rook);
                    *bbs.get_mut(Piece::Rook, side_from_case(c)) |= sq.bb();
                    f += 1;
                }

                Some(c @ ('q' | 'Q')) => {
                    board[sq] = Some(Piece::Queen);
                    *bbs.get_mut(Piece::Queen, side_from_case(c)) |= sq.bb();
                    f += 1;
                }

                Some(c @ ('k' | 'K')) => {
                    board[sq] = Some(Piece::King);
                    *bbs.get_mut(Piece::King, side_from_case(c)) |= sq.bb();
                    f += 1;
                }

                Some(c) => {
                    return Err(format!("unexpected character '{}'", c));
                }

                _ => {
                    return Err("unexpected end of piece placement".to_string());
                }
            }
        }
    }

    Ok((bbs, board))
}

fn side_from_case(c: char) -> Side {
    if c.is_uppercase() {
        Side::White
    } else {
        Side::Black
    }
}

pub fn parse(fen: &str) -> Result<Position, String> {
    let args: Vec<&str> = fen.split_whitespace().collect();

    let &piece_placement = args.get(0).ok_or("no piece placement field".to_string())?;
    let (bbs, board) = fen::parse_piece_placement(piece_placement)?;

    let &stm_str = args.get(1).ok_or("no side-to-move field")?;

    let stm = if stm_str == "w" {
        Side::White
    } else if stm_str == "b" {
        Side::Black
    } else {
        return Err(format!("side-to-move field {} is invalid", stm_str));
    };

    let &castle_rights_str = args.get(2).ok_or("no castle rights field".to_string())?;

    let castle_rights = if castle_rights_str == "-" {
        0
    } else {
        let mut castle_rights: CastleRights = 0;

        for c in castle_rights_str.chars() {
            let flag = match c {
                'K' => CASTLE_RIGHT_K_WHITE,
                'Q' => CASTLE_RIGHT_Q_WHITE,
                'k' => CASTLE_RIGHT_K_BLACK,
                'q' => CASTLE_RIGHT_Q_BLACK,
                _ => return Err(format!("invalid castle right '{}'", c)),
            };

            castle_rights |= flag;
        }

        castle_rights
    };

    let &ep_str = args.get(3).ok_or("no en passant square field")?;

    let ep = if ep_str == "-" {
        None
    } else {
        Some(Sq::from_san(ep_str).ok_or(format!("square '{}' is invalid", ep_str))?)
    };

    let &halfmove_clock_str = args.get(4).ok_or("no halfmove clock field")?;
    let halfmove_clock = halfmove_clock_str
        .parse::<usize>()
        .map_err(|_| format!("halfmove clock field '{}' is invalid", halfmove_clock_str))?;

    let &fullmoves_str = args.get(5).ok_or("no fullmoves field")?;
    let fullmoves = fullmoves_str
        .parse::<usize>()
        .map_err(|_| format!("fullmoves field '{}' is invalid", fullmoves_str))?;

    Ok(Position {
        bbs,
        board,
        stm,
        castle_rights,
        ep,
        halfmove_clock: halfmove_clock as _,
        fullmoves: fullmoves as _,
    })
}

pub fn to_fen(pos: &Position) -> String {
    use std::fmt::Write;

    let mut result = String::new();

    let black = pos.side_occ(Side::Black);

    for r in (0..8).rev() {
        if r < 7 {
            write!(result, "/").unwrap();
        }

        let mut f = 0;

        while f < 8 {
            let sq = Sq::from_coords(r, f);
            f += 1;

            match pos.board[sq] {
                Some(p) => {
                    let is_black = black & sq.bb() != 0;

                    let x = if is_black {
                        p.san_lowercase()
                    } else {
                        p.san_lowercase().to_ascii_uppercase()
                    };

                    write!(result, "{}", x).unwrap();
                }

                None => {
                    let mut count = 1;

                    while f < 8 && pos.board[Sq::from_coords(r, f)].is_none() {
                        count += 1;
                        f += 1;
                    }

                    write!(result, "{}", count).unwrap();
                }
            }
        }
    }

    write!(result, " {} ", pos.stm.char()).unwrap();

    if pos.castle_rights == 0 {
        write!(result, "- ").unwrap();
    } else {
        for (c, flag) in [
            ('K', CASTLE_RIGHT_K_WHITE),
            ('Q', CASTLE_RIGHT_Q_WHITE),
            ('k', CASTLE_RIGHT_K_BLACK),
            ('q', CASTLE_RIGHT_Q_BLACK),
        ] {
            if pos.castle_rights & flag != 0 {
                write!(result, "{}", c).unwrap();
            }
        }
    }

    if let Some(sq) = pos.ep {
        write!(result, " {} ", sq.san()).unwrap();
    }
    else {
        write!(result, " - ").unwrap();
    }

    write!(result, "{} {}", pos.halfmove_clock, pos.fullmoves).unwrap();

    result
}

#[cfg(test)]
mod tests {
    use crate::{KIWIPETE_FEN, Position, STARTPOS_FEN};

    #[test]
    fn test_startpos_read_write() {
        assert_eq!(Position::from_fen(STARTPOS_FEN).unwrap().fen(), STARTPOS_FEN);
    }

    #[test]
    fn test_kiwipete_read_write() {
        assert_eq!(Position::from_fen(KIWIPETE_FEN).unwrap().fen(), KIWIPETE_FEN);
    }
}