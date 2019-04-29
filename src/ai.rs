use crate::board::*;
use crate::evaluate::*;
use crate::move_generator::*;
use crate::player::Player;
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
        AiPlayer {
            rng: small_rng,
            eval: Box::new(BasicEvaluator),
        }
    }

    fn search(&self, depth: usize, c: Color, b: &Board, maximizing: bool) -> i32 {
        if depth == 0 {
            return self.eval.evaluate(b, c);
        }
        let mut best = if maximizing { -9999 } else { 9999 };
        for m in generate_moves(b, c) {
            best = if maximizing { 
                std::cmp::max(best, self.search(depth -1, !c, &b.apply(&m.clone().into()).unwrap(), !maximizing))
            } else {
                std::cmp::min(best, self.search(depth -1, !c, &b.apply(&m.clone().into()).unwrap(), !maximizing))
            };
        }
        best
    }
}

impl Player for AiPlayer {
    fn get_move(&mut self, c: Color, b: &Board) -> GenMove {
        // use rand::prelude::SliceRandom;
        let mut moves = generate_moves(b, c)
            .iter()
            .map(|m| {
                (
                    m.clone(),
                    self.search(3, !c, &b.apply(&m.clone().into()).unwrap(), false),
                )
            })
            .collect::<Vec<(GenMove, i32)>>();
        moves.sort_by_key(|x| x.1);
        info!("all {} moves\r\n{:#?}", moves.len(), moves);
        moves.iter().last().cloned().unwrap().0
        // let m = moves.choose(&mut self.rng).cloned().unwrap();
        // m
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
