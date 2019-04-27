use crate::board::*;
use crate::player::Player;
use crate::move_generator::*;

pub struct AiPlayer {}

impl Player for AiPlayer {
    fn get_move(&self, c: Color, b: &Board) -> Move {
        let moves = generate_moves(b, c);
        unimplemented!();
    }
}

// pub struct MoveBank {
    
// }

// impl MoveBank {
//     pub fn new() -> Self {
//         MoveBank {

//         }
//     }

//     pub fn get_moves_for(&self, _c: Color, _p: Piece, _pos: Pos) {

//     }
// }