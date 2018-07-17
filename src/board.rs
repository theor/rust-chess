use std::fmt;

pub struct Move {
    pub from: (u8,u8),
    pub to: (u8,u8),
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone)]
pub struct Board {
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_king: u64,

    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_king: u64,
}

impl Board {
    pub fn new_start() -> Board {
        Board {
            white_pawns: 0xFF00u64,
            white_knights: 0x42u64,
            white_bishops: 0x24u64,
            white_rooks: 0x81u64,
            white_queens: 0x10u64,
            white_king: 0x08u64,

            black_pawns: 0x00FF_0000_0000_0000u64,
            black_knights: 0x4200_0000_0000_0000u64,
            black_bishops: 0x2400_0000_0000_0000u64,
            black_rooks: 0x8100_0000_0000_0000u64,
            black_queens: 0x1000_0000_0000_0000u64,
            black_king: 0x0800_0000_0000_0000u64,
        }
    }

    pub fn apply(&self, m:&Move) -> Board {
        self.clone()
    }

    pub fn has(u: u64, x: u8, y: u8) -> bool {
        u & (1u64 << (63 - (y * 8 + x))) != 0u64
    }
    fn get_pc_board(&self, p: &Piece, c: &Color) -> u64 {
        use Piece::*;
        use Color::*;
        match (p, c) {
            (&Pawn, &White) => self.white_pawns,
            (&Knight, &White) => self.white_knights,
            (&Bishop, &White) => self.white_bishops,
            (&Rook, &White) => self.white_rooks,
            (&Queen, &White) => self.white_queens,
            (&King, &White) => self.white_king,

            (&Pawn, &Black) => self.black_pawns,
            (&Knight, &Black) => self.black_knights,
            (&Bishop, &Black) => self.black_bishops,
            (&Rook, &Black) => self.black_rooks,
            (&Queen, &Black) => self.black_queens,
            (&King, &Black) => self.black_king,
        }
    }
    pub fn at(&self, x: u8, y: u8) -> Option<(Piece, Color)> {
        use Piece::*;
        use Color::*;
        for c in &[White, Black] {
            for p in &[Pawn, Knight, Bishop, Rook, Queen, King] {
                let u = self.get_pc_board(p, c);
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
    fn fmt_f(&self, f: &mut fmt::Formatter, ffn: &Fn(Option<(Piece, Color)>, &mut fmt::Formatter) -> ()) -> fmt::Result {
        let b = self.hydrate();
        for y in 0u8..8u8 {
            write!(f, "|");
            for x in 0u8..8u8 {
                let i: usize = (y * 8 + x) as usize;
                ffn(b[i], f);
            }
            write!(f, "|\n");
        }
        Ok(())
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Piece::*;
        use Color::*;
        self.fmt_f(f, &|a,f| match a {
            None => write!(f, "{}", " ").unwrap(),

            Some((Pawn, White)) => write!(f, "{}", "\u{2659}").unwrap(),
            Some((Knight, White)) => write!(f, "{}", "\u{2658}").unwrap(),
            Some((Bishop, White)) => write!(f, "{}", "\u{2657}").unwrap(),
            Some((Rook, White)) => write!(f, "{}", "\u{2656}").unwrap(),
            Some((Queen, White)) => write!(f, "{}", "\u{2655}").unwrap(),
            Some((King, White)) => write!(f, "{}", "\u{2654}").unwrap(),

            Some((Pawn, Black)) => write!(f, "{}", "\u{265F}").unwrap(),
            Some((Knight, Black)) => write!(f, "{}", "\u{265E}").unwrap(),
            Some((Bishop, Black)) => write!(f, "{}", "\u{265D}").unwrap(),
            Some((Rook, Black)) => write!(f, "{}", "\u{265C}").unwrap(),
            Some((Queen, Black)) => write!(f, "{}", "\u{265B}").unwrap(),
            Some((King, Black)) => write!(f, "{}", "\u{265A}").unwrap(),
        })
    }
}