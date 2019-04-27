use crate::board::*;
use std::fmt;

bitflags! {
    pub struct Flags: u32 {
        const NONE = 0b0;
        const EN_PASSANT = 0b00000001;
        const CASTLE = 0b00000010;
        const DOUBLE_STEP = 0b00000100;
        const CAPTURE = 0b1000;
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

    pub fn parse<I>(it: &mut I) -> Option<Case>
    where
        I: Iterator<Item = char>,
    {
        let fcx = it.next()?;
        let fx = fcx as i8 - 'a' as i8;
        let fy = it.next().unwrap().to_digit(10).unwrap();
        let from = Case::new((fy - 1) as u8, fx as u8);
        Some(from)
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
    color: Color,
    player: &PartialBoard,
    other: &PartialBoard,
    moves: &mut Vec<GenMove>,
) {
    let mut pieces = CaseIterator::new(player.get_pc_board(Piece::Knight));
    while let Some(piece) = pieces.next() {
        let mut moves_it = CaseIterator::new(KNIGHT_MOVES[piece.0 as usize]);
        while let Some(dest) = moves_it.next() {
            println!(
                "check\r\n\t{:#064b}\r\n\t{:#064b}\r\n\t{:#064b}",
                player.all(),
                dest.board(),
                player.all() & dest.board()
            );
            if player.all() & dest.board() == 0 {
                let flags = if other.all() & dest.board() == 0 {
                    Flags::NONE
                } else {
                    Flags::CAPTURE
                };
                moves.push(GenMove::new(piece, dest, flags));
            }
        }
    }
}

pub fn generate_pawn_moves(
    color: Color,
    player: &PartialBoard,
    other: &PartialBoard,
    moves: &mut Vec<GenMove>,
) {
    let mut pieces = CaseIterator::new(player.get_pc_board(Piece::Pawn));

    // TODO en passant
    // TODO promotion

    while let Some(piece) = pieces.next() {
        let cached_captures = color.map(PAWN_MOVES_WHITE_CAPTURES[piece.0 as usize], PAWN_MOVES_BLACK_CAPTURES[piece.0 as usize]);
        let mut cached_it = CaseIterator::new(cached_captures);
        let mut capture = false;
        println!("cached captures {:#064b}", cached_captures);
        while let Some(dest) = cached_it.next() {
            println!("check pawn {:?}\r\n\t{:#064b}\r\n\t{:#064b}\r\n\t{:#064b}", dest, dest.board(), other.all(), other.all() & dest.board());
            if other.all() & dest.board() != 0 {
                moves.push(GenMove::new(piece, dest, Flags::CAPTURE));
                capture = true;
            }
        }
        if !capture {
            let cached = color.map(PAWN_MOVES_WHITE[piece.0 as usize], PAWN_MOVES_BLACK[piece.0 as usize]);
            let mut cached_it = CaseIterator::new(cached);
            while let Some(dest) = cached_it.next() {
                if player.all() & dest.board() == 0 {
                    moves.push(GenMove::new(piece, dest, Flags::NONE))
                }
            }
        }
    }
}

pub fn generate_all_moves(
    player: Color,
    this: &PartialBoard,
    other: &PartialBoard,
    moves: &mut Vec<GenMove>,
) {
    generate_pawn_moves(player, &this, &other, moves);
    generate_knight_moves(player, &this, &other, moves);
}

pub fn generate_moves(b: &Board, player: Color) -> Vec<GenMove> {
    let mut moves = Vec::new();
    let (this, other) = (b.get_player_board(player), b.get_player_board(!player));

    generate_all_moves(player, &this, &other, &mut moves);

    moves
}

fn generate_pawn_boards(row_double:u8, factor: i8) -> [u64; 64] {
    let mut a = [0u64; 64];
    for c in 0..64 {
        let case: Case = c.into();
        match case.row() {
            0 | 7 => continue,
            x if x == row_double => a[c as usize] = case.offset(factor, 0).board() | case.offset(2 * factor, 0).board(),
            _ => a[c as usize] = case.offset(factor, 0).board(),
        }
    }
    a
}
fn generate_pawn_boards_capture(factor: i8) -> [u64; 64] {
    let mut a = [0u64; 64];
    for c in 0..64 {
        let case: Case = c.into();
        match case.row() {
            1...6 => {
                if case.col() < 7 { a[c as usize] |= case.offset(factor, 1).board(); }
                if case.col() > 0 { a[c as usize] |= case.offset(factor, -1).board(); }
            },
            _ => continue,
        }
    }
    a
}

lazy_static! {
    static ref PAWN_MOVES_WHITE: [u64; 64] = generate_pawn_boards(1, 1);
    static ref PAWN_MOVES_BLACK: [u64; 64] = generate_pawn_boards(6, -1);
    static ref PAWN_MOVES_WHITE_CAPTURES: [u64; 64] = generate_pawn_boards_capture(1);
    static ref PAWN_MOVES_BLACK_CAPTURES: [u64; 64] = generate_pawn_boards_capture(-1);
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

    fn m(s: &str) -> GenMove {
        assert!(s.len() == 4 || s.len() == 5);
        let mut chars = s.chars().peekable();
        let from = Case::parse(&mut chars).unwrap();
        let flags = match chars.peek() {
            Some('x') => {
                chars.next();
                Flags::CAPTURE
            }
            _ => Flags::NONE,
        };
        let to = Case::parse(&mut chars).unwrap();

        GenMove::new(from, to, flags)
    }

    fn parse_case(s: &str) -> Case {
        Case::parse(&mut s.chars()).unwrap()
    }

    fn test_moves_f<F>(player: Color, setup: Vec<(Color, Piece, &str)>, expected_moves: Vec<&str>, f: F)
    where
        F: Fn(Color, &PartialBoard, &PartialBoard, &mut Vec<GenMove>),
    {
        let mut b = Board::empty();
        for (c, p, case) in setup.iter() {
            *b.get_pc_board_mut(*p, *c) |= parse_case(case).board();
        }

        let expected_moves = expected_moves.iter().map(|x| m(x));
        let mut moves = Vec::new();
        let (this, other) = (b.get_player_board(player), b.get_player_board(!player));
        f(player, &this, &other, &mut moves);
        println!("{:#?}\n{} moves", moves, moves.len());
        assert_that!(&moves.len(), eq(expected_moves.len()));
        assert_that!(&moves, contains_in_any_order(expected_moves));
    }

    fn test_moves(player: Color, setup: Vec<(Color, Piece, &str)>, expected_moves: Vec<&str>) {
        test_moves_f(player, setup, expected_moves, generate_all_moves)
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
        test_moves(
            Color::White,
            vec![(Color::White, Piece::Knight, "d4")],
            vec![
                "d4b3", "d4b5", "d4c2", "d4c6", "d4e2", "d4e6", "d4f3", "d4f5",
            ],
        )
    }

    #[test]
    fn genmoves_knight_white_bottom_left() {
        test_moves(
            Color::White,
            vec![(Color::White, Piece::Knight, "b2")],
            vec!["b2c4", "b2d3", "b2d1", "b2a4"],
        )
    }

    #[test]
    fn genmoves_knight_white_top_right() {
        test_moves(
            Color::White,
            vec![(Color::White, Piece::Knight, "g7")],
            vec!["g7e6", "g7e8", "g7f5", "g7h5"],
        )
    }

    #[test]
    fn genmoves_knight_white_top_right_one_occupied_case() {
        test_moves_f(
            Color::White,
            vec![
                (Color::White, Piece::Knight, "g7"),
                (Color::White, Piece::Pawn, "e6"),
            ],
            vec![/*"g7e6",*/ "g7e8", "g7f5", "g7h5"],
            generate_knight_moves,
        )
    }

    #[test]
    fn genmoves_knight_white_top_right_capture() {
        test_moves_f(
            Color::White,
            vec![
                (Color::White, Piece::Knight, "g7"),
                (Color::Black, Piece::Pawn, "e6"),
            ],
            vec!["g7xe6", "g7e8", "g7f5", "g7h5"],
            generate_knight_moves,
        )
    }

    #[test]
    fn genmoves_pawn_white_capture() {
        test_moves_f(
            Color::White,
            vec![
                (Color::White, Piece::Pawn, "e2"),
                (Color::Black, Piece::Knight, "f3"),
            ],
            vec!["e2xf3"],
            generate_pawn_moves,
        )
    }

    #[test]
    fn genmoves_pawn_black_capture() {
        test_moves_f(
            Color::Black,
            vec![
                (Color::Black, Piece::Pawn, "f3"),
                (Color::White, Piece::Knight, "e2"),
            ],
            vec!["f3xe2"],
            generate_pawn_moves,
        )
    }
}
