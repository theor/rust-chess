use crate::board::*;
use crate::move_generator::*;
use crate::player::Player;
use rand::rngs::SmallRng;

pub struct AiPlayer {
    rng: SmallRng,
}

impl AiPlayer {
    pub fn new(seed: [u8; 16]) -> Self {
        use rand::SeedableRng;
        // Create small, cheap to initialize and fast RNG with a random seed.
        // The randomness is supplied by the operating system.
        let small_rng = SmallRng::from_seed(seed);
        AiPlayer { rng: small_rng, }
    }
}

impl Player for AiPlayer {
    fn get_move(&mut self, c: Color, b: &Board) -> Move {
        use rand::prelude::SliceRandom;
        let moves = generate_moves(b, c);
        info!("all {} moves {:?}", moves.len(), moves);
        let m = moves.choose(&mut self.rng).cloned().unwrap();
        m.into()
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
