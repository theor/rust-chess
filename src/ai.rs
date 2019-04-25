use crate::board::*;
use crate::player::Player;

pub struct AiPlayer {}

impl Player for AiPlayer {
    fn get_move(&self, _c: Color, _b: &Board) -> Move {
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

    pub fn get_moves_for(&self, _c: Color, _p: Piece, _pos: Pos) {

    }
}