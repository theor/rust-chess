use crate::board::*;
use crate::move_generator::*;
use crate::player::Player;
use crate::evaluate::*;
use rand::rngs::SmallRng;

pub struct AiPlayer {
    rng: SmallRng,
    eval: Box<Evaluator>,
}

impl AiPlayer {
    pub fn new(seed: [u8; 16]) -> Self {
        use rand::SeedableRng;
        // Create small, cheap to initialize and fast RNG with a random seed.
        // The randomness is supplied by the operating system.
        let small_rng = SmallRng::from_seed(seed);
        AiPlayer { rng: small_rng, eval: Box::new(BasicEvaluator) }
    }

    // fn search(&self, depth: usize, c:Color, b: &Board){

    // }
}

impl Player for AiPlayer {
    fn get_move(&mut self, c: Color, b: &Board) -> Move {
        // use rand::prelude::SliceRandom;
        let mut moves = generate_moves(b, c).iter().map(|m| (m.clone(), self.eval.evaluate(&b.apply(&m.clone().into()).unwrap(), c))).collect::<Vec<(GenMove, i32)>>();
        moves.sort_by_key(|x| -x.1);
        info!("all {} moves\r\n{:#?}", moves.len(), moves);
        moves.iter().next().cloned().unwrap().0.into()
        // let m = moves.choose(&mut self.rng).cloned().unwrap();
        // m.into()
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
