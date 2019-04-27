use crate::board::*;
use std::fmt;

bitflags! {
    pub struct Flags: u32 {
        const NONE = 0b0;
        const EN_PASSANT = 0b00000001;
        const CASTLE = 0b00000010;
        const DOUBLE_STEP = 0b00000100;
        // const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Case(pub u8);

impl fmt::Debug for Case {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let col = "abcdefgh".chars().nth(self.col() as usize).unwrap();
        write!(f, "{}{}/{}", col, self.row() + 1, self.0)
    }
}

impl Case {
    pub fn new(row: u8, col: u8) -> Self {
        Case(row * 8u8 + col)
    }

    pub fn try_offset(&self, row: i8, col: i8) -> Option<Self> {
        let nrow = self.row() as i8 + row;
        let ncol = self.col() as i8 + col;
        if nrow >= 0 && nrow < 8 && ncol >= 0 && ncol < 8 {
            Some(Case::new(nrow as u8, ncol as u8))
        } else {
            None
        }
    }

    pub fn offset(&self, row: i8, col: i8) -> Self {
        Case::new(
            (self.row() as i8 + row) as u8,
            (self.col() as i8 + col) as u8,
        )
    }

    pub fn row(&self) -> u8 {
        self.0 / 8
    }
    pub fn col(&self) -> u8 {
        self.0 % 8
    }

    pub fn board(&self) -> u64 {
        1 << self.0
    }
}

impl Into<Case> for u8 {
    fn into(self) -> Case {
        Case(self)
    }
}

#[derive(Debug, PartialEq)]
pub struct GenMove {
    from: Case,
    to: Case,
    flags: Flags,
}

impl GenMove {
    pub fn new(from: Case, to: Case, flags: Flags) -> Self {
        GenMove { from, to, flags }
    }
}

struct CaseIterator {
    bitboard: u64,
    last: i8,
}

impl CaseIterator {
    pub fn new(bitboard: u64) -> Self {
        Self { bitboard, last: -1 }
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

pub fn generate_knight_moves(
    player: &PartialBoard,
    _other: &PartialBoard,
    moves: &mut Vec<GenMove>,
) {
    let mut pieces = CaseIterator::new(player.get_pc_board(Piece::Knight));
    while let Some(piece) = pieces.next() {
        let mut moves_it = CaseIterator::new(KNIGHT_MOVES[piece.0 as usize]);
        while let Some(dest) = moves_it.next() {
            moves.push(GenMove::new(piece, dest, Flags::NONE))
        }
    }
}
pub fn generate_pawn_moves(
    color: Color,
    player: &PartialBoard,
    _other: &PartialBoard,
    moves: &mut Vec<GenMove>,
) {
    let mut pieces = CaseIterator::new(player.get_pc_board(Piece::Pawn));
    let dir: i8 = color.map(1, -1);
    while let Some(piece) = pieces.next() {
        let dest = piece.offset(dir, 0);

        moves.push(GenMove::new(piece, dest, Flags::NONE))
    }
}
pub fn generate_moves(b: &Board, player: Color) -> Vec<GenMove> {
    let mut moves = Vec::new();
    let (this, other) = (b.get_player_board(player), b.get_player_board(!player));

    generate_pawn_moves(player, &this, &other, &mut moves);
    generate_knight_moves(&this, &other, &mut moves);

    moves
}

lazy_static! {
    static ref KNIGHT_MOVES: [u64; 64] = {
        let mut a = [0u64; 64];
        for c in 0..64 {
            let case: Case = c.into();
            let mut mov = 0u64;
            for &(delta_x, delta_y) in &[
                (1, 2),
                (-1, 2),
                (1, -2),
                (-1, -2),
                (2, 1),
                (2, -1),
                (-2, 1),
                (-2, -1),
            ] {
                if let Some(dest) = case.try_offset(delta_x, delta_y) {
                    mov |= dest.board();
                }
            }
            a[c as usize] = mov;
        }
        a
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use galvanic_assert::matchers::collection::*;
    use galvanic_assert::matchers::*;
    use std::str::Chars;

    fn case(it: &mut Chars) -> Option<Case> {
        let fcx = it.next().unwrap();
        let fx = fcx as i8 - 'a' as i8;
        let fy = it.next().unwrap().to_digit(10).unwrap();
        let from = Case::new((fy - 1) as u8, fx as u8);
        Some(from)
    }

    fn m(s: &str) -> GenMove {
        assert!(s.len() == 4);
        let mut chars = s.chars();
        let from = case(&mut chars).unwrap();
        let to = case(&mut chars).unwrap();

        GenMove::new(from, to, Flags::NONE)
    }

    fn parse_case(s: &str) -> Case {
        case(&mut s.chars()).unwrap()
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
        let moves = generate_moves(&b, Color::White);
        println!("{:#?}\n{} moves", moves, moves.len());
    }

    #[test]
    fn genmoves_knights_white() {
        let b = Board {
            white: PartialBoard {
                knights: 0x42u64,
                ..PartialBoard::empty()
            },
            black: PartialBoard::empty(),
        };
        let moves = generate_moves(&b, Color::White);
        println!("{:#?}\n{} moves", moves, moves.len());
    }

    #[test]
    fn genmoves_knight_white_center() {
        let mut b = Board::empty();
        b.white.knights = parse_case("d4").board();

        let moves = generate_moves(&b, Color::White);
        println!("{:#?}\n{} moves", moves, moves.len());
        assert_that!(
            &moves,
            contains_in_any_order(vec![
                m("d4b3"),
                m("d4b5"),
                m("d4c2"),
                m("d4c6"),
                m("d4e2"),
                m("d4e6"),
                m("d4f3"),
                m("d4f5"),
            ])
        );
    }

    fn test_moves(
        setup: Vec<(Color, Piece, &str)>,
        expected_moves: Vec<&str>,
    ){
        let mut b = Board::empty();
        for (c, p, case) in setup.iter() {
            *b.get_pc_board_mut(*p, *c) |= parse_case(case).board();
        }
        
        let moves = generate_moves(&b, Color::White);
        println!("{:#?}\n{} moves", moves, moves.len());
        assert_that!(
            &moves,
            contains_in_any_order(expected_moves.iter().map(m).collect())
        );
    }

    #[test]
    fn genmoves_knight_white_bottom_left() {
        test_moves(
            vec![(Color::White, Piece::Knight, "b2")],
            vec!["b2c4", "b2d3", "b2d1", "b2a4",]
        )
    }

    #[test]
    fn genmoves_knight_white_bottom_left2() {
        let mut b = Board::empty();
        b.white.knights = parse_case("b2").board();

        let moves = generate_moves(&b, Color::White);
        println!("{:#?}\n{} moves", moves, moves.len());
        assert_that!(
            &moves,
            contains_in_any_order(vec![m("b2c4"), m("b2d3"), m("b2d1"), m("b2a4"),])
        );
    }

    #[test]
    fn genmoves_knight_white_top_right() {
        let mut b = Board::empty();
        b.white.knights = parse_case("g7").board();

        let moves = generate_moves(&b, Color::White);
        println!("{:#?}\n{} moves", moves, moves.len());
        assert_that!(
            &moves,
            contains_in_any_order(vec![m("g7e6"), m("g7e8"), m("g7f5"), m("g7h5"),])
        );
    }
}
