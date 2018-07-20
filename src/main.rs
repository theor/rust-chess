mod board;
mod player;
mod ai;
mod validator;

use board::*;

fn main() {
    use player::Player;
    let mut bo = Board::new_start();
    let w = player::IOPlayer {};
    let b = player::SeqPlayer {};

    let mut t = 0usize;

    // use std::io;
    // use std::io::prelude::*;

    // let mut buffer = String::new();
    // let stdin = io::stdin();

    println!("{}", bo);
    loop {
        let (cur, color):(&Player, Color) = if t % 2 == 0 {
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

        // if waitforinput
        // let mut handle = stdin.lock();
        // handle.read_line(&mut buffer);
    }
}

#[test]
fn has_empty() {
    assert_eq!(false, Board::has(0b0, 0, 0));
}

#[test]
fn has_0_0() {
    assert_eq!(true, Board::has(0x0000_0000_0000_0001u64, 0, 0));
}

#[test]
fn has_1_0() {
    assert_eq!(true, Board::has(0x0000_0000_0000_0002u64, 1, 0));
}

#[test]
fn has_1_1() {
    assert_eq!(true, Board::has(0x0000_0000_0000_0200u64, 1, 1));
}

#[test]
fn it_works() {
    let b = Board::new_start();
    assert_eq!(true, b.color_at(Color::White, &Pos(0,0)));
}
#[test]
fn it_works2() {
    let b = Board::new_start();
    assert_eq!(false, b.color_at(Color::Black, &Pos(0,0)));
}

#[test]
fn move_bank() {
    let b = Board::new_start();
    println!("{}", b);
}

use validator::Validator;

#[test]
fn validate_a() {
    let b = Board::new_start();
    assert_eq!(true, Validator::check_move(&b, &Move::new(0,1,0,2)));
    assert_eq!(true, Validator::check_move(&b, &Move::new(0,1,0,3)));
    assert_eq!(false, Validator::check_move(&b, &Move::new(0,1,0,4)));
    // assert_eq!(false, Validator::check_move(&b, &Move::new(0,1,1,2)));
}
#[test]
fn validate_a2() {
    let b = Board::new_start();
    assert_eq!(false, Validator::check_move(&b, &Move::new(0,1,1,2)));
}
#[test]
fn validate_knight() {
    let b = Board::new_start();
    assert_eq!(true, Validator::check_move(&b, &Move::new(1,0,0,2)));
    assert_eq!(true, Validator::check_move(&b, &Move::new(1,0,2,2)));
    assert_eq!(false, Validator::check_move(&b, &Move::new(1,0,1,2)));

    let b2 = b.apply(&Move::new(2,1,2,2)).unwrap();
    println!("{}", b2);
    assert_eq!(false, Validator::check_move(&b2, &Move::new(1,0,2,2)));
    
    let b3 = b.apply(&Move::new(2,6,2,2)).unwrap();
    println!("{}", b3);
    assert_eq!(true, Validator::check_move(&b3, &Move::new(1,0,2,2)));
}
