use crate::board::*;

pub trait Player {
    fn get_move(&self, c: Color, b: &Board) -> Move;
}
pub struct IOPlayer {}
impl Player for IOPlayer {
    fn get_move(&self, _c: Color, _b: &Board) -> Move {
        use std::io;
        use std::io::prelude::*;

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        
        loop {
            handle.read_line(&mut buffer).unwrap();

            let p = buffer.parse::<Move>();
            if let Ok(m) = p {
                println!("{:?}", m);
                return m;
            }
            else {
                println!("{:?}", p);
            }
            buffer.clear();
        }
    }
}

pub struct SeqPlayer {}

impl Player for SeqPlayer {
    fn get_move(&self, c: Color, b: &Board) -> Move {
        if c == Color::White {
            if b.any_at(2, 2) {
                Move {
                    from: Pos(2, 2),
                    to: Pos(1, 0),
                }
            } else {
                Move {
                    from: Pos(1, 0),
                    to: Pos(2, 2),
                }
            }
        } else {
            if b.any_at(1, 7) {
                Move {
                    from: Pos(1, 7),
                    to: Pos(2, 5),
                }
            } else {
                Move {
                    from: Pos(2, 5),
                    to: Pos(1, 7),
                }
            }
        }
    }
}
