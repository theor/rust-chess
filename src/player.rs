use board::*;

pub trait Player {
    fn get_move(&self, c:Color, b:&Board) -> Move;
}

pub struct SeqPlayer {

}

impl Player for SeqPlayer {
    fn get_move(&self, c:Color, b:&Board) -> Move {
        Move{from:(0,0),to:(0,2)}
    }
}