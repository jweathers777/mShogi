//! Explicit game-selected human console and isolated machine entry points.

use std::{
    env,
    io::{self, BufRead, Write},
    process::ExitCode,
};

use mshogi_core::CHESS_POSITION_PROFILE_ID;

mod record_io;
mod session;
mod shogi_session;
mod uci;
mod usi;

use session::{MAX_LINE_BYTES, Next, Session};

#[derive(Clone, Copy, PartialEq)]
enum Game {
    Chess,
    Shogi,
}
impl Game {
    fn name(self) -> &'static str {
        match self {
            Self::Chess => "chess",
            Self::Shogi => "shogi",
        }
    }
}
struct Invocation {
    game: Game,
    mode: String,
}
fn arguments(args: impl Iterator<Item = std::ffi::OsString>) -> Result<Invocation, String> {
    let mut args = args.into_iter();
    let mut selected = None;
    let mut mode = None;
    while let Some(arg) = args.next() {
        let arg = arg.to_str().ok_or("arguments must be valid Unicode")?;
        if arg == "--game" {
            if selected.is_some() {
                return Err("duplicate/conflicting --game selections".into());
            }
            let value = args.next().ok_or("--game requires chess or shogi")?;
            selected = Some(match value.to_str() {
                Some("chess") => Game::Chess,
                Some("shogi") => Game::Shogi,
                _ => return Err("unknown or missing game; --game requires chess or shogi".into()),
            });
        } else if mode.replace(arg.to_owned()).is_some() {
            return Err("unsupported extra arguments".into());
        }
    }
    let mut mode = mode.unwrap_or("--help".into());
    if mode == "internal-search" && selected.is_some() {
        return Err("--game is incompatible with internal-search: its structured request supplies the game identity".into());
    }
    let game = if mode == "usi" {
        if selected == Some(Game::Chess) {
            return Err("USI supports shogi only; explicit chess selection conflicts".into());
        }
        Game::Shogi
    } else if mode == "shogi" {
        if selected == Some(Game::Chess) {
            return Err("shogi alias conflicts with --game chess".into());
        }
        mode = "shell".into();
        Game::Shogi
    } else {
        selected.unwrap_or(Game::Chess)
    };
    if mode == "uci" && game != Game::Chess {
        return Err("UCI supports chess only; select --game shogi usi for shogi".into());
    }
    if !matches!(
        mode.as_str(),
        "--help"
            | "-h"
            | "--version"
            | "-V"
            | "profile"
            | "shell"
            | "uci"
            | "usi"
            | "internal-search"
    ) {
        return Err("unsupported arguments".into());
    }
    Ok(Invocation { game, mode })
}
fn help(game: Game) {
    let game = game.name();
    println!("mShogi headless console — selected game: {game}

Usage: mshogi [--game chess|shogi] [--help | --version | profile | shell | shogi | uci | usi | internal-search]

  --game <game>  Select chess or shogi (default chess)
  --help, -h     Show selected-game help
  --version, -V  Show the application version
  profile        Show the selected native rules profile and actual coverage
  shell          Read {game} commands from standard input, one per line
  shogi          Compatibility alias for --game shogi shell
  uci            Chess UCI adapter (rejects --game shogi)
  usi            Untimed bounded USI shogi adapter (bare usi selects shogi)
  internal-search  Structured native search; request supplies game, no --game

Use 'mshogi --game {game} shell' and type 'help' for native commands.
The shell is a diagnostic console, not a UCI engine. Results go to stdout;
errors to stderr as 'error: line N: ...'. A failed command changes nothing.
Exit status: 0 success, 1 any shell failure, 2 invalid program arguments.
Chess: FEN, position-only status, known move history, perft/divide depth 0..4.
Shogi: SFEN, hands, complete native event history/conclusions, advisory eligibility,
formal entering-king declaration and resignation; perft/divide depth 0..2.
Both: position/show/legal/move/undo/history/status; canonical envelope record
print/save/load-file/load-json. Saves require a new file. No shared board engine.");
}
fn main() -> ExitCode {
    let Invocation { game, mode } = match arguments(env::args_os().skip(1)) {
        Ok(invocation) => invocation,
        Err(error) => return invalid_arguments(&error),
    };
    match mode.as_str() {
        "--help" | "-h" => help(game),
        "--version" | "-V" => println!("mshogi {}", env!("CARGO_PKG_VERSION")),
        "profile" => match game {
            Game::Chess => {
                println!("Target rules profile: {CHESS_POSITION_PROFILE_ID}");
                println!(
                    "Coverage: position/FEN, all chess actions with undo, attacks, legal actions, check/mate/stalemate and perft; diagnostic status is position-only"
                );
            }
            Game::Shogi => {
                println!("Target rules profile: {}", mshogi_core::shogi::PROFILE_ID);
                println!("Edition: {}", mshogi_core::shogi::PROFILE_EDITION);
                println!(
                    "Coverage: untimed native SFEN/game/events, complete legal moves/drops/promotions, hands, advisory eligibility, formal declarations/resignation, conclusions, event undo and canonical envelope records; local perft/divide depth 0..2. Earlier imported history is unknown. USI is available in separate usi mode."
                );
            }
        },
        "shell" => match game {
            Game::Chess => {
                let mut session = Session::new();
                return shell(|line, out| session.execute(line, out));
            }
            Game::Shogi => {
                let mut session = shogi_session::Session::new();
                return shell(|line, out| session.execute(line, out));
            }
        },
        "uci" => return uci::run(),
        "usi" => return usi::run(),
        "internal-search" => {
            if let Err(error) = mshogi_engine::serve_once(io::stdin().lock(), io::stdout().lock()) {
                eprintln!("internal search failed: {error}");
                return ExitCode::FAILURE;
            }
        }
        _ => unreachable!("validated mode"),
    }
    ExitCode::SUCCESS
}

fn shell(mut execute: impl FnMut(&str, &mut String) -> Result<Next, String>) -> ExitCode {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();
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
        let result = execute(&text, &mut out);
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

fn invalid_arguments(message: &str) -> ExitCode {
    eprintln!("error: {message}; run 'mshogi --help'");
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
