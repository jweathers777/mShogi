//! Deliberately named external verification process, excluded from distributables.
use std::{
    io::{self, BufRead, Write},
    thread,
    time::Duration,
};
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mode = args.get(1).map_or("resign", String::as_str);
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let words: Vec<_> = line.split_ascii_whitespace().collect();
        match words.first().copied() {
            Some("usi") => println!(
                "id name mshogi-USI-verification-fixture\noption name Nodes type spin default 5000 min 0 max 20000\noption name RulesProfile type combo default mshogi.shogi.csa36.v1 var mshogi.shogi.csa36.v1\nusiok"
            ),
            Some("isready") => println!("readyok"),
            Some("go") => match mode {
                "win" => println!("bestmove win"),
                "resign" => println!("bestmove resign"),
                "move" => println!("bestmove {}", args.get(2).unwrap()),
                "malformed" => println!("bestmove win garbage"),
                "illegal" => println!("bestmove 9a9b"),
                "missing" => return,
                "crash" => std::process::exit(7),
                "post_crash" => {
                    println!("bestmove resign");
                    io::stdout().flush().unwrap();
                    std::process::exit(7);
                }
                "hungquit" => println!("bestmove resign"),
                "delayed_duplicate" => println!("bestmove 7g7f"),
                "timeout" => {}
                "late" => {
                    thread::sleep(Duration::from_millis(80));
                    println!("bestmove 7g7f");
                }
                "duplicate" => println!("bestmove 7g7f\nbestmove 2g2f"),
                "oversize" => println!("{}", "x".repeat(5000)),
                "flood" => {
                    for _ in 0..2000 {
                        println!("info string bounded-verification-flood");
                    }
                }
                _ => panic!("unknown verification mode"),
            },
            Some("quit") if mode == "delayed_duplicate" => {
                thread::sleep(Duration::from_millis(390));
                println!("bestmove 2g2f");
                io::stdout().flush().unwrap();
                return;
            }
            Some("quit") if mode == "hungquit" => thread::sleep(Duration::from_secs(2)),
            Some("quit") => return,
            _ => {}
        }
        io::stdout().flush().unwrap();
    }
}
