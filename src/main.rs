mod board;
mod player;

use board::*;

fn main() {
    use player::Player;
    let mut bo = Board::new_start();
    let w = player::IOPlayer {};
    let b = player::SeqPlayer {};

    let mut t = 0usize;

    use std::io;
    use std::io::prelude::*;

    let mut buffer = String::new();
    let stdin = io::stdin();

    println!("{}", bo);
    loop {
        let (cur:&Player, color) = if t % 2 == 0 {
            (&w, Color::White)
        } else {
            (&b, Color::Black)
        };
        let m = cur.get_move(color, &bo);

        if let Some(newboard) = bo.apply(&m) {
            bo = newboard;
            t += 1;
        } else {
            println!("wrong move: {:?}", m);
        }
        println!("{:?}\n{}", color, bo);

        let mut handle = stdin.lock();
        handle.read_line(&mut buffer);
    }
}

#[test]
fn has_empty() {
    assert_eq!(false, Board::has(0b0, 0, 0));
}

#[test]
fn has_0_0() {
    assert_eq!(true, Board::has(0x8000_0000_0000_0000u64, 0, 0));
}

#[test]
fn has_1_0() {
    assert_eq!(true, Board::has(0x4000_0000_0000_0000u64, 1, 0));
}

#[test]
fn has_1_1() {
    assert_eq!(true, Board::has(0x0040_0000_0000_0000u64, 1, 1));
}

#[test]
fn it_works() {
    let b = Board::new_start();
    println!("{}", b);
}
