//! Portable test-only UCI child process, never an independent chess opponent.
use std::io::{self, BufRead, Write};
fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "mate".into());
    if mode == "exit" {
        std::process::exit(7)
    }
    if mode == "clean_eof" {
        return;
    }
    if mode == "partial_exit" || mode == "partial_clean_eof" {
        eprintln!("fixture diagnostic before partial stdout closes");
        print!("id name partial");
        let _ = io::stdout().flush();
        for _ in 0..32 {
            std::thread::yield_now();
        }
        std::process::exit(if mode == "partial_exit" { 7 } else { 0 });
    }
    if mode == "silent" || mode == "blocked" {
        std::thread::sleep(std::time::Duration::from_secs(60));
        return;
    }
    let mut position = String::new();
    for line in io::stdin().lock().lines() {
        let line = match line {
            Ok(s) => s,
            Err(_) => break,
        };
        if line == "uci" {
            if mode == "malformed" {
                let _ = io::stdout().write_all(b"\xff\n");
                let _ = io::stdout().flush();
            }
            if mode == "id_flood" {
                for _ in 0..1000 {
                    println!("id name repeated fixture identity");
                    println!();
                }
            }
            if mode == "stderr_flood" {
                let _ = io::stderr().write_all(&vec![b'e'; 1_048_576]);
            }
            if mode == "cycle" || mode == "quiet_threshold" {
                println!("id name mShogi scripted-policy-fixture");
            } else {
                println!("id name runner fixture");
            }
            println!("uciok");
            let _ = io::stdout().flush();
            if mode == "blocked_writer" {
                std::thread::sleep(std::time::Duration::from_secs(60));
                return;
            }
        } else if line == "isready" {
            println!("readyok");
            if mode == "idle_exit" {
                let _ = io::stdout().flush();
                return;
            }
        } else if line.starts_with("position ") {
            position = line;
        } else if line.starts_with("go") {
            if mode == "search_exit" {
                std::process::exit(11)
            }
            if mode == "search_eof" {
                return;
            }
            if mode == "search_silent" {
                std::thread::sleep(std::time::Duration::from_secs(60));
                return;
            }
            if mode == "cycle" || mode == "quiet_threshold" {
                let ply = position
                    .split(" moves ")
                    .nth(1)
                    .map_or(0, |m| m.split_whitespace().count());
                let moves = if mode == "cycle" {
                    ["g1f3", "g8f6", "f3g1", "f6g8"]
                } else {
                    ["h8g8", "b1b2", "g8h8", "b2b1"]
                };
                println!("bestmove {}", moves[ply % 4]);
            } else if mode == "oversized" {
                println!("{}", "x".repeat(70000));
            } else if mode == "flood" {
                for _ in 0..1000 {
                    println!("info string diagnostic");
                }
                println!("bestmove h8g8")
            } else if mode == "invalid" {
                println!("bestmove e2e5")
            } else if position.contains(" h8g8") {
                println!("bestmove f5c8")
            } else {
                println!("bestmove h8g8")
            }
        } else if line == "quit" {
            break;
        }
        let _ = io::stdout().flush();
    }
}
