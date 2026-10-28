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
            Some("uci") => println!(
                "id name mshogi-Xiangqi-UCI0-9-verification-fixture\noption name Nodes type spin default 128 min 0 max 512\noption name RulesProfile type combo default mshogi.xiangqi.wxf2018.v1 var mshogi.xiangqi.wxf2018.v1\nuciok"
            ),
            Some("isready") => println!("readyok"),
            Some("go") => match mode {
                "win" => println!("bestmove win"),
                "null" => println!("bestmove 0000"),
                "incomplete" => {
                    println!("info string mshogi execution_failure incomplete: bounded test");
                    io::stdout().flush().unwrap();
                    return;
                }
                "unresolved" => {
                    println!("info string mshogi execution_failure unresolved: bounded test");
                    io::stdout().flush().unwrap();
                    return;
                }
                "resign" => println!("bestmove resign"),
                "move" => println!("bestmove {}", args.get(2).unwrap()),
                "malformed" => println!("bestmove win garbage"),
                "illegal" => println!("bestmove a0a9"),
                "missing" => return,
                "crash" => std::process::exit(7),
                "post_crash" => {
                    println!("bestmove resign");
                    io::stdout().flush().unwrap();
                    std::process::exit(7);
                }
                "hungquit" => println!("bestmove resign"),
                "delayed_duplicate" => println!("bestmove a3a4"),
                "timeout" => {}
                "late" => {
                    thread::sleep(Duration::from_millis(80));
                    println!("bestmove a3a4");
                }
                "duplicate" => println!("bestmove a3a4\nbestmove c3c4"),
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
                println!("bestmove c3c4");
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
