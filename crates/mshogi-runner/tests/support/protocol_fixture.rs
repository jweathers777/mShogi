//! Deliberately named external verification process, excluded from distributables.
use std::{
    io::{self, BufRead, Write},
    thread,
    time::Duration,
};
pub fn serve(hello: &str, banner: &str, legal: &str, duplicate: &str, illegal: &str) {
    let args: Vec<_> = std::env::args().collect();
    let mode = args.get(1).map_or("resign", String::as_str);
    let mut script_index = 0;
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let words: Vec<_> = line.split_ascii_whitespace().collect();
        match words.first().copied() {
            Some(word) if word == hello => {
                if mode == "duplicate_option" {
                    println!("option name Conflicting type spin min 1 max 1");
                    println!("option name Conflicting type spin min 2 max 2");
                }
                println!("{banner}");
            }
            Some("isready") => {
                if mode == "readiness_info" {
                    println!("info string mshogi execution_failure incomplete: stale data");
                }
                println!("readyok");
            }
            Some("position") if mode == "script" => {
                // The shared script includes the opening, so each persistent
                // child selects its reply from the complete supplied history.
                let moves = words
                    .iter()
                    .position(|w| *w == "moves")
                    .map_or(&[][..], |i| &words[i + 1..]);
                assert_eq!(moves, &args[2..2 + moves.len()]);
                script_index = moves.len();
            }
            Some("go") => match mode {
                "script" => println!(
                    "bestmove {}",
                    args.get(2 + script_index).expect("script reply")
                ),
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
                "win" => println!("bestmove win"),
                "resign" | "duplicate_option" | "readiness_info" => println!("bestmove resign"),
                "move" => println!("bestmove {}", args.get(2).unwrap()),
                "malformed" => println!("bestmove win garbage"),
                "illegal" => println!("bestmove {illegal}"),
                "missing" => return,
                "crash" => std::process::exit(7),
                "post_crash" => {
                    println!("bestmove resign");
                    io::stdout().flush().unwrap();
                    std::process::exit(7);
                }
                "hungquit" => println!("bestmove resign"),
                "delayed_duplicate" => println!("bestmove {legal}"),
                "timeout" | "late" => {}
                "duplicate" => println!("bestmove {legal}\nbestmove {duplicate}"),
                "oversize" => println!("{}", "x".repeat(5000)),
                "flood" => {
                    for _ in 0..2000 {
                        println!("info string bounded-verification-flood");
                    }
                }
                _ => panic!("unknown verification mode"),
            },
            // The retired request replies only after the controller observes
            // its watchdog and sends stop. No scheduler delay can make it timely.
            Some("stop") if mode == "late" => println!("bestmove {legal}"),
            Some("quit") if mode == "delayed_duplicate" => {
                thread::sleep(Duration::from_millis(390));
                println!("bestmove {duplicate}");
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
