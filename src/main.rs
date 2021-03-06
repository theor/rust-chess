mod ai;
mod board;
mod evaluate;
mod move_generator;
mod player;
mod validator;

#[macro_use]
extern crate lazy_static;

extern crate rand;

#[macro_use]
extern crate log;
extern crate simplelog;
#[macro_use]
extern crate bitflags;

extern crate clap;
use clap::{App, Arg, SubCommand};

#[cfg(test)]
#[macro_use]
extern crate galvanic_assert;

use crate::board::*;
use crate::move_generator::*;
use crate::validator::Validator;
use player::Player;

fn main() {
    use simplelog::*;
    use std::env;
    use std::fs::File;

    let args = App::new("chess")
        .version("1.0")
        .arg(
            Arg::with_name("log")
                .short("l")
                .long("log")
                .value_name("FILE")
                .help("Sets a custom log file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("interactive")
                .short("i")
                .long("interactive")
                .help("interactive mode")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("fen")
                .short("f")
                .long("fen")
                .help("fen string")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("next-move")
                .short("n")
                .help("compute next move"),
        )
        .get_matches();

    let uci = !args.is_present("interactive");
    let logfile = args.value_of("log").unwrap_or("chess");

    let path = format!("C:\\Users\\theor\\rust-chess\\{}.log", logfile);
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Debug, Config::default()).unwrap(),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create(path).unwrap(),
        ),
    ])
    .unwrap();

    info!("args {:?}", env::args());
    info!("args {:?}", args);
    info!("fen {:?}", args.value_of("fen"));

    use std::panic;

    panic::set_hook(Box::new(|p| {
        let backtrace = backtrace::Backtrace::new();

        error!("{}\r\n{:?}", p, backtrace);
    }));

    if uci {
        engine_uci(args.is_present("next-move"), args.value_of("fen"));
        return;
    }

    use crate::player::Player;
    let mut bo = Board::new_start();
    let mut w = player::IOPlayer {};
    let mut b = player::SeqPlayer {};

    let mut t = 0usize;

    // use std::io;
    // use std::io::prelude::*;

    // let mut buffer = String::new();
    // let stdin = io::stdin();

    println!("size of board: {} bytes", std::mem::size_of::<Board>());
    println!("{}", bo);
    loop {
        let (cur, color): (&mut Player, Color) = if t % 2 == 0 {
            (&mut w, Color::White)
        } else {
            (&mut b, Color::Black)
        };
        let m = cur.get_move(color, &bo);

        if let Some(newboard) = Validator::check_move(&bo, &m).and(bo.apply(&m)) {
            bo = newboard;
            t += 1;
        } else {
            println!("wrong move: {:?}", m);
        }
        println!("{:?}\n{}", color, bo);

        // if waitforinput
        // let mut handle = stdin.lock();
        // handle.read_line(&mut buffer);
    }
}

struct Engine {
    board: Board,
    ai: crate::ai::AiPlayer,
    move_count: usize,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            board: Board::empty(),
            ai: crate::ai::AiPlayer::new([42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            move_count: 0,
        }
    }

    fn output<S: std::fmt::Display + AsRef<str>>(&self, out: S) {
        info!("{}", out);
        print!("{}\n", out);
    }

    fn parse_move<I>(it: &mut std::iter::Peekable<I>) -> Option<GenMove>
    where
        I: Iterator<Item = char>,
    {
        while let Some(' ') = it.peek() {
            it.next();
        }
        if let Some(from) = crate::move_generator::Case::parse(it) {
            if let Some(to) = crate::move_generator::Case::parse(it) {
                let n = { it.peek ().cloned() };
                if let Some(x) = n {
                    let promotion = match x {
                        'k' => Some(Piece::King),
                        'q' => Some(Piece::Queen),
                        'r' => Some(Piece::Rook),
                        'b' => Some(Piece::Bishop),
                        'n' => Some(Piece::Knight),
                        'p' => Some(Piece::Pawn),
                        _ => None,
                    };
                    if promotion.is_some() {
                        it.next();
                    }
                }
                return Some(GenMove::new(from, to, Flags::NONE)); // TODO flags
            }
        }
        None
    }

    pub fn process(&mut self, cmd: &str) {
        match cmd {
            "quit" => return,
            "uci" => {
                self.output(format!("id name rustchess {}", "0.1"));
                self.output("id author theor");
                self.output("option name Clear Hash type button");
                self.output("uciok");
            }
            "isready" => self.output("readyok"),
            "ucinewgame" => {}
            "position startpos" => self.board = Board::new_start(), // reset position
            "position fen <FEN>" => unimplemented!(),               // reset position
            _ => {
                if cmd.starts_with("go") {
                    let color = if self.move_count % 2 == 0 {
                        Color::White
                    } else {
                        Color::Black
                    };
                    let mov = self.ai.get_move(color, &self.board);
                    //  if self.move_count % 2 == 0 {
                    //      if self.move_count % 4 == 0 { "b1a3" } else { "a3b1" }
                    // } else {
                    //     if self.move_count % 4 == 1 { "b8a6" } else { "a6b8" }
                    // };
                    self.output(format!("bestmove {}", mov));
                } else if cmd.starts_with("position startpos moves") {
                    // info!("parsing move list");
                    let mut it = cmd.chars().skip(23).peekable();
                    self.board = Board::new_start();
                    self.move_count = 0;
                    while let Some(mov) = Self::parse_move(&mut it) {
                        // info!("  move {}", mov);
                        self.board = self.board.apply(&mov).unwrap();
                        self.move_count += 1;
                    }
                    info!("    final board\r\n {}", self.board);
                } else {
                    error!("unknown command {}", cmd);
                }
            }
        }
    }
}

fn engine_uci(next_move: bool, fen: Option<&str>) {
    use std::io;
    use std::io::prelude::*;

    if next_move {
        let (b, c): (Board, Color) = fen
            .and_then(|s| board::parse_fen_color(&mut s.chars()))
            .unwrap_or_else(|| (Board::empty(), Color::White));
        println!("start color: {:?}\r\n{}", c, b);
        let mut ai = ai::AiPlayer::new([42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        println!("next: {}", ai.get_move(c, &b));
        return;
    }

    let mut buffer = String::new();
    let stdin = io::stdin();

    let mut engine = Engine::new();

    loop {
        let mut handle = stdin.lock();
        handle.read_line(&mut buffer).unwrap();
        {
            let cmd = buffer.trim_end();

            info!("{:?}", cmd);
            engine.process(cmd);
        }
        buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::validator::MoveType::{Capture, Quiet};

    #[test]
    fn parse() {
        //KQRBNP kqrbnp
        let b = board::parse(
            "rnbqkbnr
                          pppppppp
                          ________
                          ________
                          ________
                          ________
                          PPPPPPPP
                          RNBQKBNR",
        )
        .unwrap();
        println!("{}", b);
        assert_eq!(Board::new_start(), b);
    }

    #[test]
    fn has_empty() {
        assert_eq!(false, Board::has(0b0, 0, 0));
    }

    #[test]
    fn has_0_0() {
        assert_eq!(true, Board::has(0x0000_0000_0000_0001u64, 0, 0));
    }

    #[test]
    fn has_1_0() {
        assert_eq!(true, Board::has(0x0000_0000_0000_0002u64, 1, 0));
    }

    #[test]
    fn has_1_1() {
        assert_eq!(true, Board::has(0x0000_0000_0000_0200u64, 1, 1));
    }

    #[test]
    fn it_works() {
        let b = Board::new_start();
        assert_eq!(true, b.color_at(Color::White, &Case::new(0, 0)));
    }
    #[test]
    fn it_works2() {
        let b = Board::new_start();
        assert_eq!(false, b.color_at(Color::Black, &Case::new(0, 0)));
    }

    #[test]
    fn move_bank() {
        let b = Board::new_start();
        println!("{}", b);
    }

    #[test]
    fn validate_pawn_w_quiet_move1() {
        let b = Board::new_start();
        assert_eq!(
            Some(Quiet),
            Validator::check_move(
                &b,
                &GenMove::new(Case::new(1, 0), Case::new(2, 0), Flags::NONE)
            )
        );
    }

    //     #[test]
    //     fn validate_pawn_w_quiet_move2() {
    //         let b = Board::new_start();
    //         assert_eq!(
    //             Some(Quiet),
    //             Validator::check_move(&b, &Move::new(0, 1, 0, 3))
    //         );
    //     }

    //     #[test]
    //     fn validate_pawn_w_invalid_too_far() {
    //         let b = Board::new_start();
    //         assert_eq!(None, Validator::check_move(&b, &Move::new(0, 1, 0, 4)));
    //     }

    //     #[test]
    //     fn validate_pawn_w_capture_enpassant() {
    //         let b = Board::new_start();
    //         let b = b.apply(&Move::new(2, 6, 1, 2)).unwrap();
    //         assert_eq!(
    //             Some(Capture),
    //             Validator::check_move(&b, &Move::new(0, 1, 1, 2))
    //         );
    //     }

    //     #[test]
    //     fn validate_pawn_w_invalid_quiet_enpassant() {
    //         let b = Board::new_start();
    //         assert_eq!(None, Validator::check_move(&b, &Move::new(0, 1, 1, 2)));
    //     }

    //     // black pawn

    //     #[test]
    //     fn validate_pawn_b_quiet_move1() {
    //         let b = Board::new_start();
    //         assert_eq!(
    //             Some(Quiet),
    //             Validator::check_move(&b, &Move::new(0, 6, 0, 5))
    //         );
    //     }

    //     #[test]
    //     fn validate_pawn_b_quiet_move2() {
    //         let b = Board::new_start();
    //         assert_eq!(
    //             Some(Quiet),
    //             Validator::check_move(&b, &Move::new(0, 6, 0, 4))
    //         );
    //     }

    //     #[test]
    //     fn validate_pawn_b_invalid_too_far() {
    //         let b = Board::new_start();
    //         assert_eq!(None, Validator::check_move(&b, &Move::new(0, 6, 0, 3)));
    //     }

    //     #[test]
    //     fn validate_pawn_b_capture_enpassant() {
    //         let b = Board::new_start();
    //         let b = b.apply(&Move::new(2, 1, 1, 5)).unwrap();
    //         assert_eq!(
    //             Some(Capture),
    //             Validator::check_move(&b, &Move::new(0, 6, 1, 5))
    //         );
    //     }

    //     #[test]
    //     fn validate_pawn_b_invalid_quiet_enpassant() {
    //         let b = Board::new_start();
    //         assert_eq!(None, Validator::check_move(&b, &Move::new(0, 6, 1, 5)));
    //     }

    //     #[test]
    //     fn validate_knight() {
    //         let b = Board::new_start();
    //         assert_eq!(
    //             Some(Quiet),
    //             Validator::check_move(&b, &Move::new(1, 0, 0, 2))
    //         );
    //         assert_eq!(
    //             Some(Quiet),
    //             Validator::check_move(&b, &Move::new(1, 0, 2, 2))
    //         );
    //         assert_eq!(None, Validator::check_move(&b, &Move::new(1, 0, 1, 2)));

    //         let b2 = b.apply(&Move::new(2, 1, 2, 2)).unwrap();
    //         println!("{}", b2);
    //         assert_eq!(None, Validator::check_move(&b2, &Move::new(1, 0, 2, 2)));

    //         let b3 = b.apply(&Move::new(2, 6, 2, 2)).unwrap();
    //         println!("{}", b3);
    //         assert_eq!(
    //             Some(Capture),
    //             Validator::check_move(&b3, &Move::new(1, 0, 2, 2))
    //         );
    //     }

    //     // rook

    //     #[test]
    //     fn validate_rook_h_quiet() {
    //         let b = board::parse(
    //             "
    // ________
    // ________
    // ________
    // ________
    // ________
    // __R_____
    // ________
    // ________",
    //         )
    //         .unwrap();
    //         println!("{}", b);
    //         assert_eq!(
    //             Some(Quiet),
    //             Validator::check_move(&b, &Move::new(2, 2, 0, 2))
    //         );
    //         assert_eq!(
    //             Some(Quiet),
    //             Validator::check_move(&b, &Move::new(2, 2, 2, 0))
    //         );
    //         assert_eq!(
    //             Some(Quiet),
    //             Validator::check_move(&b, &Move::new(2, 2, 4, 2))
    //         );
    //         assert_eq!(
    //             Some(Quiet),
    //             Validator::check_move(&b, &Move::new(2, 2, 2, 4))
    //         );
    //         assert_eq!(None, Validator::check_move(&b, &Move::new(2, 2, 4, 4)));
    //     }

    //     #[test]
    //     fn validate_rook_obstacle() {
    //         //KQRBNP kqrbnp
    //         let s = "________
    //              ________
    //              ________
    //              ________
    //              ________
    //              ________
    //              ________
    //              R_n_____";
    //         let b = board::parse(s).unwrap();
    //         println!("{}", b);
    //         assert_eq!(
    //             Some(Capture),
    //             Validator::check_move(&b, &Move::new(0, 0, 2, 0))
    //         );
    //         assert_eq!(None, Validator::check_move(&b, &Move::new(0, 0, 3, 0)));
    //     }

    //     // Bishop

    //     #[test]
    //     fn validate_bishop_quiet() {
    //         //KQRBNP kqrbnp
    //         let s = "
    // ________
    // ________
    // ________
    // ________
    // ________
    // __b_____
    // ________
    // ________";
    //         let b = board::parse(s).unwrap();
    //         println!("{}", b);
    //         assert_eq!(
    //             Some(Quiet),
    //             Validator::check_move(&b, &Move::new(2, 2, 0, 0))
    //         );
    //         assert_eq!(
    //             Some(Quiet),
    //             Validator::check_move(&b, &Move::new(2, 2, 3, 3))
    //         );
    //         assert_eq!(
    //             Some(Quiet),
    //             Validator::check_move(&b, &Move::new(2, 2, 3, 1))
    //         );
    //         assert_eq!(
    //             Some(Quiet),
    //             Validator::check_move(&b, &Move::new(2, 2, 0, 4))
    //         );
    //         assert_eq!(None, Validator::check_move(&b, &Move::new(2, 2, 4, 5)));
    //     }

    //     #[test]
    //     fn validate_bishop_capture() {
    //         //KQRBNP kqrbnp
    //         let b = board::parse(
    //             "________
    //                           ________
    //                           ________
    //                           ____N___
    //                           ________
    //                           __b_____
    //                           ________
    //                           ________",
    //         )
    //         .unwrap();
    //         println!("{}", b);
    //         assert_eq!(
    //             Some(Capture),
    //             Validator::check_move(&b, &Move::new(2, 2, 4, 4))
    //         );
    //         assert_eq!(None, Validator::check_move(&b, &Move::new(2, 2, 5, 5)));
    //     }
}
