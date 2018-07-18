use board::*;

pub trait Player {
    fn get_move(&self, c: Color, b: &Board) -> Move;
}

pub struct SeqPlayer {}

impl Player for SeqPlayer {
    fn get_move(&self, c: Color, b: &Board) -> Move {
        if c == Color::White {
            if b.any_at(2, 2) {
                Move {
                    from: (2, 2),
                    to: (1, 0),
                }
            } else {
                Move {
                    from: (1, 0),
                    to: (2, 2),
                }
            }
        } else {
            if b.any_at(1, 7) {
                Move {
                    from: (1, 7),
                    to: (2, 5),
                }
            } else {
                Move {
                    from: (2, 5),
                    to: (1, 7),
                }
            }
        }
    }
}
