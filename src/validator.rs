use crate::board::*;
use crate::move_generator::*;

pub struct Validator {}

#[derive(Debug, PartialEq)]
pub enum MoveType {
    Quiet,
    Capture,
}

impl MoveType {
    pub fn map_quiet(b: bool) -> Option<MoveType> {
        if b {
            Some(MoveType::Quiet)
        } else {
            None
        }
    }
    pub fn map_capture(b: bool) -> Option<MoveType> {
        if b {
            Some(MoveType::Capture)
        } else {
            None
        }
    }
    pub fn map(valid: bool, capture: bool) -> Option<MoveType> {
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
    fn delta_abs(a: u8, b: u8) -> u8 {
        if a > b {
            a - b
        } else {
            b - a
        }
    }
    fn cmp(a: u8, b: u8) -> i16 {
        if a > b {
            -1
        } else if a < b {
            1
        } else {
            0
        }
    }

    fn validate_sliding(m: &GenMove, c: Color, b: &Board, dx:u8, dy: u8) -> Option<MoveType> {
        let (sx, sy) = (Self::cmp(m.from.col(), m.to.col()), Self::cmp(m.from.row(), m.to.row()));
        let d = u8::max(dx, dy);
        for i in 1..d as i16 {
            let dpos = Case::new(
                (m.from.col() as i16 + sx * i) as u8,
                (m.from.row() as i16 + sy * i) as u8,
            );
            if !b.empty_at(&dpos) {
                return None;
            }
        }
        MoveType::map(b.color_or_empty_at(c.rev(), &m.to), !b.empty_at(&m.to))
    }

    pub fn check_move(b: &Board, m: &GenMove) -> Option<MoveType> {
        let (dx, dy) = (
            Self::delta_abs(m.to.col(), m.from.col()),
            Self::delta_abs(m.to.row(), m.from.row()),
        );
        if let Some((p, c)) = b.at_pos(&m.from) {
            match p {
                Piece::Pawn => {
                    let (fwd, from_start_row) = match c {
                        Color::White => (m.to.row() > m.from.row(), m.from.row() == 1),
                        Color::Black => (m.to.row() < m.from.row(), m.from.row() == 6),
                    };
                    if !fwd {
                        None
                    } else if dx == 0 {
                        MoveType::map_quiet(dy == 1 || (dy == 2 && from_start_row))
                    } else if dx == 1 {
                        // en passant
                        MoveType::map_capture(dy == 1 && b.color_at(c.rev(), &m.to))
                    } else {
                        None
                    }
                }
                Piece::Knight => {
                    let valid_dist = (dx == 2 && dy == 1) || (dx == 1 && dy == 2);
                    MoveType::map(
                        valid_dist && b.color_or_empty_at(c.rev(), &m.to),
                        !b.empty_at(&m.to),
                    )
                }
                Piece::Bishop => {
                    if dx != dy {
                        None
                    } else {
                        Self::validate_sliding(m, c, b, dx, dy)
                    }
                }
                Piece::Rook => {
                    if dx != 0 && dy != 0 {
                        None
                    } else {
                        Self::validate_sliding(m, c, b, dx, dy)
                    }
                }
                Piece::Queen => {
                    if dx != dy && dx != 0 && dy != 0 {
                        None
                    } else {
                        Self::validate_sliding(m, c, b, dx, dy)
                    }
                }
                Piece::King => { MoveType::map(dx + dy == 1, b.color_or_empty_at(c.rev(), &m.to)) }
            }
        } else {
            None
        }
    }
}
