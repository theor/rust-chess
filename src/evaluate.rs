use board::*;

pub trait Evaluator {
    fn evaluate(&self, b: &Board, player: Color) -> i32;
}

pub struct BasicEvaluator;

fn eval(b: &PartialBoard) -> u32 {
    b.pawns.count_ones() * 10 +
    b.knights.count_ones() * 30 +
    b.bishops.count_ones() * 30 +
    b.rooks.count_ones() * 50 +
    b.queens.count_ones() * 90 +
    b.king.count_ones() * 900
}

impl Evaluator for BasicEvaluator {
    fn evaluate(&self, b: &Board, player: Color) -> i32 {
        let this = b.color(player);
        let other = b.color(!player);
        return eval(this) as i32 - eval(other) as i32;
    }
}