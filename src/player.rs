use move_generator::{GenMove, Case, Flags};
use crate::board::*;

pub trait Player {
    fn get_move(&mut self, c: Color, b: &Board) -> GenMove;
}
pub struct IOPlayer {}
impl Player for IOPlayer {
    fn get_move(&mut self, _c: Color, _b: &Board) -> GenMove {
        use std::io;
        use std::io::prelude::*;

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        
        loop {
            handle.read_line(&mut buffer).unwrap();

            let p = buffer.parse::<GenMove>();
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
    fn get_move(&mut self, c: Color, b: &Board) -> GenMove {
        if c == Color::White {
            if b.any_at(2, 2) {
                GenMove::new (
                   Case::new(2, 2),
                    Case::new(1, 0),
                    Flags::NONE,
                )
            } else {
                GenMove::new (
                   Case::new(1, 0),
                    Case::new(2, 2),
                    Flags::NONE,
                )
            }
        } else {
            if b.any_at(1, 7) {
                GenMove::new (
                   Case::new(1, 7),
                    Case::new(2, 5),
                    Flags::NONE,
                )
            } else {
                GenMove::new (
                   Case::new(2, 5),
                    Case::new(1, 7),
                    Flags::NONE,
                )
            }
        }
    }
}
