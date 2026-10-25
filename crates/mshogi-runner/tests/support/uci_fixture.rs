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
            if mode == "id_flood" {
                for _ in 0..1000 {
                    println!("id name repeated fixture identity");
                    println!();
                }
            }
            if mode == "stderr_flood" {
                let _ = io::stderr().write_all(&vec![b'e'; 1_048_576]);
            }
            println!("id name runner fixture");
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
            if mode == "oversized" {
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
