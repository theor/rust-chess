use move_generator::Case;
use move_generator::GenMove;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl std::ops::Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        if self == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }
}

impl Color {
    pub fn rev(&self) -> Color {
        match self {
            &Color::White => Color::Black,
            &Color::Black => Color::White,
        }
    }

    pub fn map<T>(&self, white: T, black: T) -> T {
        match self {
            &Color::White => white,
            &Color::Black => black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartialBoard {
    pub pawns: u64,
    pub knights: u64,
    pub bishops: u64,
    pub rooks: u64,
    pub queens: u64,
    pub king: u64,
}

impl PartialBoard {
    pub fn empty() -> PartialBoard {
        PartialBoard {
            pawns: 0u64,
            knights: 0u64,
            bishops: 0u64,
            rooks: 0u64,
            queens: 0u64,
            king: 0u64,
        }
    }

    pub fn all(&self) -> u64 {
        self.pawns | self.knights | self.bishops | self.rooks | self.queens | self.king
    }

    pub fn get_pc_board(&self, p: Piece) -> u64 {
        use crate::Piece::*;
        match p {
            Pawn => self.pawns,
            Knight => self.knights,
            Bishop => self.bishops,
            Rook => self.rooks,
            Queen => self.queens,
            King => self.king,
        }
    }

    pub fn get_pc_board_mut(&mut self, p: Piece) -> &mut u64 {
        use crate::Piece::*;
        match p {
            Pawn => &mut self.pawns,
            Knight => &mut self.knights,
            Bishop => &mut self.bishops,
            Rook => &mut self.rooks,
            Queen => &mut self.queens,
            King => &mut self.king,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub white: PartialBoard,
    pub black: PartialBoard,
}

impl Board {
    pub fn empty() -> Board {
        Board {
            white: PartialBoard::empty(),
            black: PartialBoard::empty(),
        }
    }
    pub fn new_start() -> Board {
        Board {
            white: PartialBoard {
                pawns: 0xFF00u64,
                knights: 0x42u64,
                bishops: 0x24u64,
                rooks: 0x81u64,
                queens: 0x08u64,
                king: 0x10u64,
            },
            black: PartialBoard {
                pawns: 0x00FF_0000_0000_0000u64,
                knights: 0x4200_0000_0000_0000u64,
                bishops: 0x2400_0000_0000_0000u64,
                rooks: 0x8100_0000_0000_0000u64,
                queens: 0x0800_0000_0000_0000u64,
                king: 0x1000_0000_0000_0000u64,
            },
        }
    }

    pub fn color(&self, color: Color) -> &PartialBoard {
        match color {
            Color::White => &self.white,
            Color::Black => &self.black,
        }
    }

    pub fn all(&self) -> u64 {
        self.white.all() | self.black.all()
    }

    pub fn apply(&self, m: &GenMove) -> Option<Board> {
        let (x, y) = m.from.pos();
        let (tx, ty) = m.to.pos();
        let (p, c) = self.at(x, y)?;

        let mut new = self.clone();
        {
            let from = new.get_pc_board_mut(p, c);
            Board::unset(from, x, y);
        }
        {
            // capture
            if let Some((cp, cc)) = new.at(tx, ty) {
                Board::unset(new.get_pc_board_mut(cp, cc), tx, ty);
            }

            // promotion
            let target_piece_board = m.promotion.unwrap_or(p);
            let from = new.get_pc_board_mut(target_piece_board, c);
            Board::set(from, tx, ty);
        }
        Some(new)
    }

    pub fn set(u: &mut u64, x: u8, y: u8) {
        *u = *u | (1u64 << (y * 8 + x))
    }
    pub fn unset(u: &mut u64, x: u8, y: u8) {
        *u = *u ^ (1u64 << (y * 8 + x))
    }
    pub fn has(u: u64, x: u8, y: u8) -> bool {
        u & (1u64 << (y * 8 + x)) != 0u64
    }

    pub fn get_player_board(&self, c: Color) -> &PartialBoard {
        match c {
            Color::White => &self.white,
            Color::Black => &self.black,
        }
    }

    pub fn get_player_board_mut(&mut self, c: Color) -> &mut PartialBoard {
        match c {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        }
    }

    pub fn get_pc_board_mut(&mut self, p: Piece, c: Color) -> &mut u64 {
        self.get_player_board_mut(c).get_pc_board_mut(p)
    }

    pub fn get_pc_board(&self, p: Piece, c: Color) -> u64 {
        self.get_player_board(c).get_pc_board(p)
    }

    pub fn any_at(&self, x: u8, y: u8) -> bool {
        Board::has(self.all(), x, y)
    }

    pub fn empty_at(&self, p: &Case) -> bool {
        let (x, y) = p.pos();
        !Board::has(self.all(), x, y)
    }

    pub fn color_or_empty_at(&self, c: Color, p: &Case) -> bool {
        let (x, y) = p.pos();
        match c {
            Color::White => !Board::has(self.black.all(), x, y),
            Color::Black => !Board::has(self.white.all(), x, y),
        }
    }

    pub fn color_at(&self, c: Color, p: &Case) -> bool {
        let (x, y) = p.pos();
        match c {
            Color::White => Board::has(self.white.all(), x, y),
            Color::Black => Board::has(self.black.all(), x, y),
        }
    }

    pub fn at_pos(&self, m: &Case) -> Option<(Piece, Color)> {
        let (x, y) = m.pos();
        self.at(x, y)
    }

    pub fn at(&self, x: u8, y: u8) -> Option<(Piece, Color)> {
        use crate::Color::*;
        use crate::Piece::*;
        for c in &[White, Black] {
            for p in &[Pawn, Knight, Bishop, Rook, Queen, King] {
                let u = self.get_pc_board(*p, *c);
                if Board::has(u, x, y) {
                    return Some((*p, *c));
                }
            }
        }
        None
    }
    pub fn hydrate(&self) -> [Option<(Piece, Color)>; 64] {
        let mut res = [None; 64];
        for x in 0u8..8u8 {
            for y in 0u8..8u8 {
                let i: usize = (y * 8 + x) as usize;
                match self.at(x, y) {
                    None => {}
                    Some((p, c)) => res[i] = Some((p, c)),
                }
            }
        }
        res
    }
    fn fmt_f(
        &self,
        f: &mut fmt::Formatter,
        ffn: &Fn(Option<(Piece, Color)>, &mut fmt::Formatter) -> (),
    ) -> fmt::Result {
        let b = self.hydrate();
        write!(f, "  ")?;
        for x in 0u8..8u8 {
            use std::char;
            write!(f, "{}", char::from_u32('a' as u32 + x as u32).unwrap())?;
        }
        write!(f, "\n")?;
        for yy in 0u8..8u8 {
            let y = 7 - yy;
            write!(f, "{}|", y)?;
            for x in 0u8..8u8 {
                let i: usize = (y * 8 + x) as usize;
                ffn(b[i], f);
            }
            write!(f, "|\n")?;
        }
        Ok(())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::Color::*;
        use crate::Piece::*;
        self.fmt_f(f, &|a, f| match a {
            None => write!(f, "{}", " ").unwrap(),

            Some((Pawn, Black)) => write!(f, "{}", "\u{2659}").unwrap(),
            Some((Knight, Black)) => write!(f, "{}", "\u{2658}").unwrap(),
            Some((Bishop, Black)) => write!(f, "{}", "\u{2657}").unwrap(),
            Some((Rook, Black)) => write!(f, "{}", "\u{2656}").unwrap(),
            Some((Queen, Black)) => write!(f, "{}", "\u{2655}").unwrap(),
            Some((King, Black)) => write!(f, "{}", "\u{2654}").unwrap(),

            Some((Pawn, White)) => write!(f, "{}", "\u{265F}").unwrap(),
            Some((Knight, White)) => write!(f, "{}", "\u{265E}").unwrap(),
            Some((Bishop, White)) => write!(f, "{}", "\u{265D}").unwrap(),
            Some((Rook, White)) => write!(f, "{}", "\u{265C}").unwrap(),
            Some((Queen, White)) => write!(f, "{}", "\u{265B}").unwrap(),
            Some((King, White)) => write!(f, "{}", "\u{265A}").unwrap(),
        })
    }
}

pub fn parse_fen_color<I>(it: &mut I) -> Option<(Board, Color)>
where
    I: Iterator<Item = char>,
{
    let (mut r, mut c) = (7, 0);

    let mut b = Board::empty();

    use Color::*;
    use Piece::*;

    fn set(b: &mut Board, p: Piece, c: Color, col: &mut u8, row: u8) {
        // println!("  set {:?} {:?} at {} {}", c, p, col, row);
        Board::set(b.get_pc_board_mut(p, c), *col, row);
        *col += 1;
    }

    while let Some(ch) = it.next() {
        // println!("    char {}", ch);
        match ch {
            '/' => {
                r -= 1;
                c = 0;
                // println!("LINE {}", r);
            }
            ' ' => break,
            'k' => set(&mut b, King, Black, &mut c, r),
            'q' => set(&mut b, Queen, Black, &mut c, r),
            'r' => set(&mut b, Rook, Black, &mut c, r),
            'b' => set(&mut b, Bishop, Black, &mut c, r),
            'n' => set(&mut b, Knight, Black, &mut c, r),
            'p' => set(&mut b, Pawn, Black, &mut c, r),
            'K' => set(&mut b, King, White, &mut c, r),
            'Q' => set(&mut b, Queen, White, &mut c, r),
            'R' => set(&mut b, Rook, White, &mut c, r),
            'B' => set(&mut b, Bishop, White, &mut c, r),
            'N' => set(&mut b, Knight, White, &mut c, r),
            'P' => set(&mut b, Pawn, White, &mut c, r),
            d if d.is_digit(10) => {
                let skip = d.to_digit(10).unwrap() as u8;
                // println!("  skip {}", skip);
                c += skip;
            }
            _ => panic!("wtf"),
        }
    }

    let c = match it.next() {
        Some('b') => Color::Black,
        _ => Color::White,
    };

    Some((b, c))
}
pub fn parse_fen(s: &str) -> Option<Board> {
    let mut it = s.chars().peekable();
    parse_fen_color(&mut it).map(|x| x.0)
}

#[test]
fn test_fen() {
    let fen = "r1bqkbnr/pp6/2n3p1/3ppp1p/2Pp1P1P/1P4P1/P1N1P3/R1BQKBNR b KQkq - 1 12";
    let b = parse_fen(fen).unwrap();
    println!("{}", b);
}

pub fn parse(s: &str) -> Option<Board> {
    //KQRBNP
    use crate::Color::*;
    use crate::Piece::*;
    let mut b = Board::empty();
    let mut y = 7;
    for l in s.lines() {
        // skip empty lines
        if l.len() == 0 {
            continue;
        }
        let mut x = 0;
        for cc in l.chars() {
            if let Some((p, c)) = match cc {
                'k' => Some((King, Black)),
                'q' => Some((Queen, Black)),
                'r' => Some((Rook, Black)),
                'b' => Some((Bishop, Black)),
                'n' => Some((Knight, Black)),
                'p' => Some((Pawn, Black)),
                'K' => Some((King, White)),
                'Q' => Some((Queen, White)),
                'R' => Some((Rook, White)),
                'B' => Some((Bishop, White)),
                'N' => Some((Knight, White)),
                'P' => Some((Pawn, White)),
                '_' => None,
                _ => continue, // ignore everything else
            } {
                let bb = b.get_pc_board_mut(p, c);
                Board::set(bb, x, y);
            }
            x += 1;
        }
        if y == 0 {
            break;
        }
        y -= 1;
    }
    Some(b)
}
