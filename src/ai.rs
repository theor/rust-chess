use board::*;
use player::Player;

pub struct AiPlayer {}

impl Player for AiPlayer {
    fn get_move(&self, c: Color, b: &Board) -> Move {
        unreachable!()
    }
}

pub struct MoveBank {
    
}

impl MoveBank {
    pub fn new() -> Self {
        MoveBank {

        }
    }

    pub fn get_moves_for(&self, c: Color, p: Piece, pos: Pos) {

    }
}