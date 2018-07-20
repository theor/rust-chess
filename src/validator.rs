use board::*;

pub struct Validator {

}

impl Validator {
    fn delta_abs(a:u8, b:u8) -> u8 {
        if a > b {
            a - b
        } else {
            b - a
        }
    }
    pub fn check_move(b:&Board, m:&Move) -> bool {
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
                    if !fwd { false }
                    else if dx == 0 {
                        dy == 1 ||
                        (dy == 2 && from_start_row)
                    } else if dx == 1 { // en passant
                        dy == 1 && b.color_at(c.rev(), &m.to)
                    } else {
                        false
                    }
                },
                Piece::Knight => {
                    let(dx,dy) = (Self::delta_abs(m.to.0,m.from.0),
                                  Self::delta_abs(m.to.1,m.from.1));
                    let valid_dist = (dx == 2 && dy == 1) || (dx == 1 && dy == 2);
                    valid_dist && b.color_or_empty_at(c.rev(), &m.to)
                }
                _ => false,
            }
        } else {
            false
        }
    }
}