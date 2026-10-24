//! Headless console entry point: program metadata and a chess command session.

use std::{
    env,
    io::{self, BufRead, Write},
    process::ExitCode,
};

use mshogi_core::CHESS_POSITION_PROFILE_ID;

mod session;
mod uci;

use session::{MAX_LINE_BYTES, Next, Session};

const HELP: &str = "mShogi headless console

Usage: mshogi [--help | --version | profile | shell | uci]

  --help, -h     Show this help
  --version, -V  Show the application version
  profile        Show the chess rules profile and its coverage
  shell          Read chess commands from standard input, one per line
  uci            Run the bounded UCI chess engine adapter

The shell starts at the standard chess position. Results go to standard
output and errors to standard error as 'error: line N: ...'; a failed
command changes nothing. Exit status: 0 if every command succeeded, 1 if
any command failed, 2 for invalid program arguments. Type 'help' in the
shell for its commands. The shell is a diagnostic console, not a UCI engine. Use 'mshogi uci'
for the engine adapter.";

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
                "Coverage: position/FEN, all chess actions with undo, attacks, legal actions, check/mate/stalemate and perft; diagnostic status is position-only"
            );
        }
        Some(Some("shell")) => return shell(),
        Some(Some("uci")) => return uci::run(),
        _ => return invalid_arguments(),
    }
    ExitCode::SUCCESS
}

fn shell() -> ExitCode {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();
    let mut session = Session::new();
    let mut failed = false;
    let mut input = stdin.lock();
    for index in 0usize.. {
        let line = bounded_line(&mut input);
        if matches!(line, Ok(None)) {
            break;
        }
        let number = index + 1;
        let line = match line {
            Ok(Some(Ok(bytes))) => bytes,
            Ok(Some(Err(()))) => {
                let _ = writeln!(
                    stderr,
                    "error: line {number}: line exceeds {MAX_LINE_BYTES} bytes"
                );
                failed = true;
                continue;
            }
            Ok(None) => unreachable!(),
            Err(error) => {
                let _ = writeln!(stderr, "error: line {number}: cannot read input: {error}");
                return ExitCode::FAILURE;
            }
        };
        let Ok(text) = String::from_utf8(line) else {
            let _ = writeln!(stderr, "error: line {number}: input is not valid UTF-8");
            failed = true;
            continue;
        };
        let mut out = String::new();
        let result = session.execute(&text, &mut out);
        // Report output before errors so the streams stay in command order.
        if stdout
            .write_all(out.as_bytes())
            .and_then(|()| stdout.flush())
            .is_err()
        {
            return ExitCode::FAILURE;
        }
        match result {
            Ok(Next::Continue) => {}
            Ok(Next::Quit) => break,
            Err(message) => {
                let _ = writeln!(stderr, "error: line {number}: {message}");
                failed = true;
            }
        }
    }
    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn invalid_arguments() -> ExitCode {
    eprintln!("error: unsupported arguments; run 'mshogi --help'");
    ExitCode::from(2)
}

/// Consume an oversized line without allocating its remainder, then recover at LF.
fn bounded_line(input: &mut impl BufRead) -> io::Result<Option<Result<Vec<u8>, ()>>> {
    let mut line = Vec::new();
    let mut oversized = false;
    let mut seen = false;
    loop {
        let chunk = input.fill_buf()?;
        if chunk.is_empty() {
            return Ok(seen.then_some(if oversized { Err(()) } else { Ok(line) }));
        }
        seen = true;
        let end = chunk.iter().position(|&b| b == b'\n');
        let take = end.unwrap_or(chunk.len());
        if !oversized {
            if line.len() + take > MAX_LINE_BYTES {
                oversized = true;
                line.clear();
            } else {
                line.extend_from_slice(&chunk[..take]);
            }
        }
        input.consume(take + usize::from(end.is_some()));
        if end.is_some() {
            return Ok(Some(if oversized { Err(()) } else { Ok(line) }));
        }
    }
}
