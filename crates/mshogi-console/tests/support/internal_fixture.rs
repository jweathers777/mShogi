//! Separate test helper: never copied into a desktop distributable.
use mshogi_engine::*;
use std::{
    io::{self, BufRead, Write},
    thread,
    time::Duration,
};
fn main() {
    let mut text = String::new();
    io::stdin().lock().read_line(&mut text).unwrap();
    let mut context: EngineRequest = serde_json::from_str(&text).unwrap();
    match context.request_id {
        1 => return,
        2 => {
            eprintln!("fixture process failure");
            std::process::exit(17);
        }
        3 => {
            println!("not json");
            return;
        }
        4 => {
            let bytes = vec![b'x'; MAX_RESPONSE_BYTES + 2];
            let _ = io::stdout().write_all(&bytes);
            let _ = io::stdout().flush();
            thread::sleep(Duration::from_secs(30));
            return;
        }
        5 => {
            thread::sleep(Duration::from_secs(30));
            return;
        }
        6 => context.process_generation += 1,
        9 => {
            let _ = io::stderr().write_all(&vec![b'e'; MAX_STDERR_BYTES * 10]);
            std::process::exit(19);
        }
        10 => {
            print!("{{}}");
            return;
        }
        11 => {
            println!("{{}}\n{{}}");
            return;
        }
        _ => {}
    }
    let result = EngineResult::Incomplete {
        telemetry: Telemetry {
            score: None,
            completed_depth: 0,
            nodes: 0,
            principal_variation: vec![],
            status: StopReason::NodeLimit,
        },
    };
    println!(
        "{}",
        serde_json::to_string(&EngineResponse { context, result }).unwrap()
    );
}
