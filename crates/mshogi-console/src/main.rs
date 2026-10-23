//! Headless console entry point; rules/diagnostic commands follow in MSH-11.

use std::{env, process::ExitCode};

use mshogi_core::CHESS_POSITION_PROFILE_ID;

const HELP: &str = "mShogi headless console scaffold

Usage: mshogi [--help | --version | profile]

  --help, -h     Show this help
  --version, -V  Show the application version
  profile        Show the target chess position profile

Game commands are not available in this console yet.";

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let first = args.next();
    if args.next().is_some() {
        return invalid_arguments();
    }

    match first.as_ref().map(|arg| arg.to_str()) {
        None | Some(Some("--help" | "-h")) => println!("{HELP}"),
        Some(Some("--version" | "-V")) => println!("mshogi {}", env!("CARGO_PKG_VERSION")),
        Some(Some("profile")) => {
            println!("Target rules profile: {CHESS_POSITION_PROFILE_ID}");
            println!(
                "Coverage: position/FEN, all chess actions with undo, attacks, legal actions, check/mate/stalemate and perft in the library; no game adjudication"
            );
        }
        _ => return invalid_arguments(),
    }
    ExitCode::SUCCESS
}

fn invalid_arguments() -> ExitCode {
    eprintln!("error: unsupported arguments; run 'mshogi --help'");
    ExitCode::from(2)
}
