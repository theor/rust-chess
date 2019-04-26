use crate::board::*;
use std::fmt;

bitflags! {
    struct Flags: u32 {
        const NONE = 0b0;
        const EN_PASSANT = 0b00000001;
        const CASTLE = 0b00000010;
        const DOUBLE_STEP = 0b00000100;
        // const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Case(u8);

impl fmt::Debug for Case {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(r: {}, c: {} / {})", self.row(), self.col(), self.0)
    }
}

impl Case {
    pub fn new(row:u8, col:u8) -> Self {
        Case(row*8u8+col)
    }

    pub fn row(&self) -> u8 { self.0 / 8 }
    pub fn col(&self) -> u8 { self.0 % 8 }
}

impl Into<Case> for u8 {
    fn into(self) -> Case { Case(self) }
}

#[derive(Debug)]
pub struct GenMove {
    from: Case,
    to: Case,
    flags: Flags,
}

impl GenMove {
    pub fn new(from: Case, to: Case) -> Self {
        GenMove {
            from,
            to,
            flags: Flags::NONE,
        }
    }
}

struct CaseIterator {
    bitboard: u64,
    last: i8,
}

impl CaseIterator {
    pub fn new(bitboard: u64) -> Self {
        Self { 
            bitboard,
            last: -1,
        }
    }
    fn next(&mut self) -> Option<Case> {
        println!("last: {}", self.last);
        if self.last >= 64 {
            println!("  end");
            None
        } else {
            let t = self.bitboard.trailing_zeros();
            println!("  trailing_zeros {}", t);
            if t == 64 {
                self.last = 64;
                None
            } else {
                self.bitboard = self.bitboard >> (t + 1);
                self.last += t as i8 + 1;
                Some(Case(self.last as u8))
            }
        }
    }
}

pub fn generate_moves(b: &Board, player: Color) -> Vec<GenMove> {
    let v = Vec::new();

    let mut pawns = CaseIterator::new(b.get_pc_board(&Piece::Pawn, &player));

    while let Some(set) = pawns.next() {
        println!("set {:?} pawn: {:?}", player, set);
        

    }

    v
}

#[test]
fn case_iterator_empty() {
    let mut c = CaseIterator::new(0);
    assert_eq!(None, c.next())
}

#[test]
fn case_iterator_first() {
    let mut c = CaseIterator::new(0b1);
    assert_eq!(Some(Case(0)), c.next());
    assert_eq!(None, c.next());
}

#[test]
fn case_iterator_one_two() {
    let mut c = CaseIterator::new(0b11);
    assert_eq!(Some(Case(0)), c.next());
    assert_eq!(Some(Case(1)), c.next());
    assert_eq!(None, c.next());
}

#[test]
fn case_iterator_1_63() {
    let mut c = CaseIterator::new(0x8000_0000_0000_0001);
    assert_eq!(Some(Case(0)), c.next());
    assert_eq!(Some(Case(63)), c.next());
    assert_eq!(None, c.next());
}

#[test]
fn case_iterator_rnd() {
    let mut c = CaseIterator::new(0x9000_0000_0000_0003);
    assert_eq!(Some(Case(0)), c.next());
    assert_eq!(Some(Case(1)), c.next());
    assert_eq!(Some(Case(60)), c.next());
    assert_eq!(Some(Case(63)), c.next());
    assert_eq!(None, c.next());
}

#[test]
fn genmoves_start() {
    let b = Board::new_start();
    println!("{:#?}", generate_moves(&b, Color::White));
}

// lazy_static! {
//     static ref WHITE_PAWNS: [u64; 64] = {
//         [0u64; 64]
//     };
// }