//! Explicit game-selected human console and isolated machine entry points.

use std::{
    env,
    io::{self, BufRead, Write},
    process::ExitCode,
};

use mshogi_core::CHESS_POSITION_PROFILE_ID;

mod chu_session;
mod diagnostics;
mod record_io;
mod session;
mod shogi_session;
mod tenjiku_session;
mod tori_session;
mod uci;
mod usi;
mod xiangqi_session;
mod xiangqi_uci;

use session::{Next, Session};

#[derive(Clone, Copy, PartialEq)]
enum Game {
    Chess,
    Shogi,
    Xiangqi,
    Tori,
    Chu,
    Tenjiku,
}
impl Game {
    fn name(self) -> &'static str {
        match self {
            Self::Chess => "chess",
            Self::Shogi => "shogi",
            Self::Xiangqi => "xiangqi",
            Self::Tori => "tori",
            Self::Chu => "chu",
            Self::Tenjiku => "tenjiku",
        }
    }
}
struct Invocation {
    game: Game,
    mode: String,
    profile: Option<mshogi_core::tenjiku::Profile>,
}
fn arguments(args: impl Iterator<Item = std::ffi::OsString>) -> Result<Invocation, String> {
    let mut args = args.into_iter();
    let mut selected = None;
    let mut profile = None;
    let mut mode = None;
    while let Some(arg) = args.next() {
        let arg = arg.to_str().ok_or("arguments must be valid Unicode")?;
        if arg == "--game" {
            if selected.is_some() {
                return Err("duplicate/conflicting --game selections".into());
            }
            let value = args
                .next()
                .ok_or("--game requires chess, shogi, xiangqi, tori, chu or tenjiku")?;
            selected = Some(match value.to_str() {
                Some("chess") => Game::Chess,
                Some("shogi") => Game::Shogi,
                Some("xiangqi") => Game::Xiangqi,
                Some("tori") => Game::Tori,
                Some("chu") => Game::Chu,
                Some("tenjiku") => Game::Tenjiku,
                _ => {
                    return Err(
                        "unknown or missing game; --game requires chess, shogi, xiangqi, tori, chu or tenjiku"
                            .into(),
                    );
                }
            });
        } else if arg == "--profile" {
            if profile.is_some() {
                return Err("duplicate/conflicting --profile selections".into());
            }
            let value = args.next().ok_or("--profile requires adams or muller")?;
            profile = Some(match value.to_str() {
                Some("adams" | "mshogi.tenjiku.adams1999-modern.v1") => {
                    mshogi_core::tenjiku::Profile::AdamsModern
                }
                Some("muller" | "mshogi.tenjiku.muller-completed.v1") => {
                    mshogi_core::tenjiku::Profile::MullerCompleted
                }
                _ => return Err("unsupported Tenjiku profile; choose adams or muller".into()),
            });
        } else if mode.replace(arg.to_owned()).is_some() {
            return Err("unsupported extra arguments".into());
        }
    }
    let mode = mode.unwrap_or("--help".into());
    if mode == "internal-search" && (selected.is_some() || profile.is_some()) {
        return Err("--game is incompatible with internal-search: its structured request supplies the game identity".into());
    }
    let game = if mode == "usi" {
        if selected.is_some_and(|g| g != Game::Shogi) {
            return Err("USI supports shogi only; explicit game selection conflicts".into());
        }
        Game::Shogi
    } else {
        selected.unwrap_or(Game::Chess)
    };
    if mode == "uci" && !matches!(game, Game::Chess | Game::Xiangqi) {
        return Err("UCI supports chess or xiangqi; select --game shogi usi for shogi; Tori/Chu/Tenjiku use shell or structured internal-search".into());
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
    if profile.is_some() && game != Game::Tenjiku {
        return Err("--profile is supported only with --game tenjiku".into());
    }
    if game == Game::Tenjiku && matches!(mode.as_str(), "profile" | "shell") && profile.is_none() {
        return Err("Tenjiku requires an explicit whole profile: --profile adams|muller".into());
    }
    Ok(Invocation {
        game,
        mode,
        profile,
    })
}
fn help(game: Game) {
    let profile_option = if game == Game::Tenjiku {
        " --profile adams|muller"
    } else {
        ""
    };
    let game = game.name();
    println!("mShogi headless console — selected game: {game}

Usage: mshogi [--game chess|shogi|xiangqi|tori|chu|tenjiku] [--profile adams|muller] [--help | --version | profile | shell | uci | usi | internal-search]

  --game <game>  Select chess, shogi, xiangqi, tori, chu or tenjiku (default chess)
  --profile     Required Tenjiku whole profile: adams or muller (no default)
  --help, -h     Show selected-game help
  --version, -V  Show the application version
  profile        Show the selected native rules profile and actual coverage
  shell          Read {game} commands from standard input, one per line
  uci            Selected Chess UCI or Xiangqi UCI0-9 adapter (rejects shogi)
  usi            Untimed bounded USI shogi adapter (bare usi selects shogi)
  internal-search  Structured native search, one request line per search until end of input; request supplies game, no --game

Use 'mshogi --game {game}{profile_option} shell' and type 'help' for native commands.
The shell is a diagnostic console, not a UCI engine. Results go to stdout;
errors to stderr as 'error: line N: ...'. A failed command changes nothing.
Exit status: 0 success, 1 any shell failure, 2 invalid program arguments.
Chess: FEN, position-only status, known move history, perft/divide depth 0..4.
Shogi: SFEN, hands, complete native event history/conclusions, advisory eligibility,
formal entering-king declaration and resignation; perft/divide depth 0..2.
Xiangqi: native intersections/FEN, Red/Black history/warnings/claims/outcomes,
canonical replay, local perft/divide 0..3 and explicit bounded cancellable no-win diagnostics.
Tori: native 7x7 board/hands, complete history/T06 third-occurrence rule, readable complete
actions, canonical standard/custom roots, bounded search and local diagnostics.
Chu: native 12x12 board without hands, King/Prince, complete compound routes,
ordered captures and returns, actual draw agreements, canonical history and bounded search.
Tenjiku: explicit whole profile, native 16x16 original/promoted identities, ordered
travel and stationary igui, full-history bounded perft/divide and search.
All: position/show/legal/move/undo/history/status; canonical envelope record
print/save/load-file/load-json. Saves require a new file.
Perft and divide accept an optional visit budget, with a default and maximum of 1,000,000.");
}
fn main() -> ExitCode {
    let Invocation {
        game,
        mode,
        profile,
    } = match arguments(env::args_os().skip(1)) {
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
            Game::Xiangqi => {
                println!("Target rules profile: {}", mshogi_core::xiangqi::PROFILE_ID);
                println!("Edition: {}", mshogi_core::xiangqi::PROFILE_EDITION);
                println!(
                    "Coverage: untimed native intersection board, complete legal actions/undo, known history/WXF warnings/claims/conclusions, strict canonical envelopes, local perft/divide 0..3, sound fast no-win assessment (Unknown is not a result). Bounded native engine-turn and read-only move-analysis execution are available through internal-search; untimed bounded Pikafish-compatible UCI0-9 is available via --game xiangqi uci."
                );
            }
            Game::Tori => {
                println!("Target rules profile: {}", mshogi_core::tori::PROFILE_ID);
                println!("Edition: {}", mshogi_core::tori::PROFILE_EDITION);
                println!(
                    "Coverage: untimed native 7x7 complete moves/drops/compulsory promotion, distinct handed quails, known history/T06 third-occurrence rule, native outcomes, resignation, exact event undo, readable action notation and strict canonical standard/custom records. Local diagnostics depth 0..3 with explicit visits; bounded native search/structured internal-search. No SFEN/USI adapter or competitive-strength claim."
                );
            }
            Game::Chu => {
                println!("Target rules profile: {}", mshogi_core::chu::PROFILE_ID);
                println!("Edition: {}", mshogi_core::chu::PROFILE_EDITION);
                println!(
                    "Coverage: untimed native 12x12 complete one/two-stage actions, ordered captures/returns, promotion, King/Prince survival, Lion protection, pre-fourth repetition admission, native conclusions and actual draw agreements; exact event undo, strict canonical standard/custom records, bounded history-aware search and local perft/divide 0..2. No hands, USI/SFEN or competitive-strength claim."
                );
            }
            Game::Tenjiku => {
                let profile = profile.expect("validated explicit Tenjiku profile");
                println!("Target rules profile: {}", profile.id());
                println!("Edition: {}", profile.edition());
                println!(
                    "Coverage: untimed native 16x16, complete ordered travel/returns/stationary igui, original identities and once-only promotion, native burn/death phases, royal/bare/repetition history and actual draw agreements; exact event undo, strict canonical standard/custom records, read-only bounded history-aware search and full-Game perft/divide 0..3. No hands, USI/SFEN or competitive-strength claim."
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
            Game::Xiangqi => {
                let mut session = match xiangqi_session::Session::new() {
                    Ok(session) => session,
                    Err(error) => {
                        eprintln!("error: {error}");
                        return ExitCode::FAILURE;
                    }
                };
                return shell(|line, out| session.execute(line, out));
            }
            Game::Tori => {
                let mut session = tori_session::Session::new();
                return shell(|line, out| session.execute(line, out));
            }
            Game::Chu => {
                let mut session = chu_session::Session::new();
                return shell(|line, out| session.execute(line, out));
            }
            Game::Tenjiku => {
                let mut session = tenjiku_session::Session::new(
                    profile.expect("validated explicit Tenjiku profile"),
                );
                return shell(|line, out| session.execute(line, out));
            }
        },
        "uci" => {
            return match game {
                Game::Chess => uci::run(),
                Game::Xiangqi => xiangqi_uci::run(),
                Game::Shogi | Game::Tori | Game::Chu | Game::Tenjiku => {
                    unreachable!("validated protocol")
                }
            };
        }
        "usi" => return usi::run(),
        "internal-search" => {
            if let Err(error) = mshogi_engine::serve(io::stdin().lock(), io::stdout().lock()) {
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
        let number = index + 1;
        let line = match read_line(&mut input) {
            Ok(Some(bytes)) => bytes,
            Ok(None) => break,
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

/// Read one line without its LF; a possible CR stays in the line.
fn read_line(input: &mut impl BufRead) -> io::Result<Option<Vec<u8>>> {
    let mut line = Vec::new();
    if input.read_until(b'\n', &mut line)? == 0 {
        return Ok(None);
    }
    if line.last() == Some(&b'\n') {
        line.pop();
    }
    Ok(Some(line))
}
