use board::*;

pub struct Validator {

}

#[derive(Debug,PartialEq)]
pub enum MoveType {
    Quiet,
    Capture,
}

impl MoveType {
    pub fn map_quiet(b:bool) -> Option<MoveType> {
        if b {
            Some(MoveType::Quiet)
        } else {
            None
        }
    }
    pub fn map_capture(b:bool) -> Option<MoveType> {
        if b {
            Some(MoveType::Capture)
        } else {
            None
        }
    }
    pub fn map(valid:bool, capture:bool) -> Option<MoveType> {
        if !valid {
            None
        } else if capture {
            Some(MoveType::Capture)
        } else {
            Some(MoveType::Quiet)
        }
    }
}

impl Validator {
    fn delta_abs(a:u8, b:u8) -> u8 {
        if a > b {
            a - b
        } else {
            b - a
        }
    }
    pub fn check_move(b:&Board, m:&Move) -> Option<MoveType> {
        use std::i16;
        let(dx,dy) = (Self::delta_abs(m.to.0,m.from.0),
                        Self::delta_abs(m.to.1,m.from.1));
        if let Some((p,c)) = b.at_pos(&m.from) {
            match p {
                Piece::Pawn => { 
                    let (fwd,from_start_row) = match c {
                        Color::White => (m.to.1 > m.from.1, m.from.1 == 1),
                        Color::Black => (m.to.1 < m.from.1, m.from.1 == 6),
                    };
                    if !fwd { None }
                    else if dx == 0 {
                        MoveType::map_quiet(
                            dy == 1 ||
                            (dy == 2 && from_start_row)
                        )
                    } else if dx == 1 { // en passant
                        MoveType::map_capture(
                            dy == 1 && b.color_at(c.rev(), &m.to)
                        )
                    } else {
                        None
                    }
                },
                Piece::Knight => {
                    let valid_dist = (dx == 2 && dy == 1) || (dx == 1 && dy == 2);
                    MoveType::map(
                        valid_dist && b.color_or_empty_at(c.rev(), &m.to),
                        !b.empty_at(&m.to)
                    )
                },
                Piece::Rook => {
                    if dx != 0 && dy != 0 {
                        None
                    } else {
                        Some(MoveType::Quiet)
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }
}