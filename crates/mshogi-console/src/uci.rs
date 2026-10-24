//! Bounded UCI adapter; protocol output belongs only to this event loop.
use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy, Color};
use mshogi_search::{
    coordination::{RequestContext, SearchCompletion, SearchCoordinator},
    search::{MATE_SCORE, MAX_DEPTH, SearchChoice, SearchDecision, SearchLimits},
};
use std::{
    io::{self, BufRead, Write},
    process::ExitCode,
    sync::mpsc::{self, Receiver, RecvTimeoutError},
    thread,
    time::{Duration, Instant},
};

const TICK: Duration = Duration::from_millis(2);
const COMMAND_CAPACITY: usize = 32;

enum Input {
    Line(Vec<u8>),
    Error(String),
    End,
}
struct Running {
    context: RequestContext,
    started: Instant,
    deadline: Option<Instant>,
    wait_for_stop: bool,
    held: Option<SearchCompletion>,
}
struct Adapter {
    game: AdjudicatedGame,
    policy: ClaimPolicy,
    version: u64,
    worker: SearchCoordinator,
    running: Option<Running>,
}

pub fn run() -> ExitCode {
    let worker = match SearchCoordinator::new() {
        Ok(worker) => worker,
        Err(error) => {
            eprintln!("uci: cannot start search worker: {error}");
            return ExitCode::FAILURE;
        }
    };
    let (send, receive) = mpsc::sync_channel(COMMAND_CAPACITY);
    if let Err(error) = thread::Builder::new()
        .name("mshogi-uci-input".into())
        .spawn(move || {
            let stdin = io::stdin();
            let mut input = stdin.lock();
            loop {
                let event = match bounded_uci_line(&mut input) {
                    Ok(Some(Ok(bytes))) => Input::Line(bytes),
                    Ok(Some(Err(()))) => {
                        Input::Error(format!("line exceeds {} bytes", crate::MAX_LINE_BYTES))
                    }
                    Ok(None) => Input::End,
                    Err(error) => {
                        let _ = send.send(Input::Error(format!("cannot read input: {error}")));
                        break;
                    }
                };
                let ended = matches!(event, Input::End);
                if send.send(event).is_err() || ended {
                    break;
                }
            }
        })
    {
        eprintln!("uci: cannot start input reader: {error}");
        return ExitCode::FAILURE;
    }
    let mut adapter = Adapter {
        game: AdjudicatedGame::new(ClaimPolicy::ExplicitOnly),
        policy: ClaimPolicy::ExplicitOnly,
        version: 0,
        worker,
        running: None,
    };
    let mut stdout = io::stdout().lock();
    match event_loop(&mut adapter, receive, &mut stdout) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("uci: adapter failed: {error}");
            ExitCode::FAILURE
        }
    }
}
// UCI permits CR and LF line endings. Treat CRLF as an empty extra line;
// empty commands are ignored. Drain oversized lines without growing allocation.
fn bounded_uci_line(input: &mut impl BufRead) -> io::Result<Option<Result<Vec<u8>, ()>>> {
    let mut line = Vec::new();
    let mut oversized = false;
    let mut seen = false;
    loop {
        let chunk = input.fill_buf()?;
        if chunk.is_empty() {
            return Ok(seen.then_some(if oversized { Err(()) } else { Ok(line) }));
        }
        seen = true;
        let end = chunk
            .iter()
            .position(|&byte| byte == b'\r' || byte == b'\n');
        let take = end.unwrap_or(chunk.len());
        if !oversized {
            if line.len() + take > crate::MAX_LINE_BYTES {
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
fn event_loop(
    adapter: &mut Adapter,
    input: Receiver<Input>,
    out: &mut impl Write,
) -> io::Result<()> {
    loop {
        match input.recv_timeout(TICK) {
            Ok(Input::Line(bytes)) => match std::str::from_utf8(&bytes) {
                Ok(line)
                    if line.is_ascii()
                        && !line
                            .bytes()
                            .any(|b| b.is_ascii_control() && !b.is_ascii_whitespace()) =>
                {
                    if adapter.command(line, out)? {
                        break;
                    }
                }
                _ => eprintln!("uci: input must be ASCII text without control bytes"),
            },
            Ok(Input::Error(message)) => eprintln!("uci: {message}"),
            Ok(Input::End) | Err(RecvTimeoutError::Disconnected) => break,
            Err(RecvTimeoutError::Timeout) => {}
        }
        adapter.update(out)?;
    }
    adapter.worker.shutdown();
    Ok(())
}
impl Adapter {
    fn invalidate(&mut self) -> Result<(), String> {
        let version = self
            .version
            .checked_add(1)
            .ok_or("game revision exhausted")?;
        self.worker.cancel();
        self.running = None;
        self.version = version;
        Ok(())
    }
    fn command(&mut self, line: &str, out: &mut impl Write) -> io::Result<bool> {
        let tokens: Vec<_> = line.split_ascii_whitespace().collect();
        let Some(index) = tokens.iter().position(|token| {
            matches!(
                *token,
                "uci"
                    | "isready"
                    | "ucinewgame"
                    | "position"
                    | "go"
                    | "stop"
                    | "quit"
                    | "setoption"
                    | "debug"
                    | "ponderhit"
                    | "register"
            )
        }) else {
            return Ok(false);
        };
        let args = &tokens[index + 1..];
        let result: Result<(), String> = match tokens[index] {
            "uci" => {
                writeln!(out, "id name mShogi {}", env!("CARGO_PKG_VERSION"))?;
                writeln!(out, "id author John Weathers")?;
                writeln!(
                    out,
                    "option name ClaimPolicy type combo default explicit var explicit var auto"
                )?;
                writeln!(out, "uciok")?;
                out.flush()?;
                Ok(())
            }
            "isready" => {
                writeln!(out, "readyok")?;
                out.flush()?;
                Ok(())
            }
            "quit" => return Ok(true),
            "stop" => {
                if let Some(running) = self.running.as_mut() {
                    running.wait_for_stop = false;
                    self.worker.cancel();
                }
                Ok(())
            }
            "ucinewgame" => self.replace_game(AdjudicatedGame::new(self.policy)),
            "position" => {
                parse_position(args, self.policy).and_then(|game| self.replace_game(game))
            }
            "setoption" => self.set_option(args),
            "go" => parse_go(args, self.game.position().side_to_move()).and_then(|go| {
                let started = Instant::now();
                let deadline = go
                    .budget
                    .map(|budget| {
                        started
                            .checked_add(budget)
                            .ok_or("time budget cannot be represented")
                    })
                    .transpose()?;
                let context = self
                    .worker
                    .submit(&self.game, self.version, go.limits)
                    .map_err(|e| e.to_string())?;
                self.running = Some(Running {
                    context,
                    started,
                    deadline,
                    wait_for_stop: go.infinite,
                    held: None,
                });
                Ok(())
            }),
            "debug" if matches!(args, ["on"] | ["off"]) => Ok(()),
            "register" => Ok(()),
            _ => Err("unsupported command or parameters".into()),
        };
        if let Err(message) = result {
            eprintln!("uci: {message}");
        }
        Ok(false)
    }
    fn replace_game(&mut self, game: AdjudicatedGame) -> Result<(), String> {
        self.invalidate()?;
        self.game = game;
        Ok(())
    }
    fn set_option(&mut self, args: &[&str]) -> Result<(), String> {
        if args.len() != 4
            || !args[0].eq_ignore_ascii_case("name")
            || !args[1].eq_ignore_ascii_case("ClaimPolicy")
            || !args[2].eq_ignore_ascii_case("value")
        {
            return Err("unsupported option; only ClaimPolicy explicit/auto is available".into());
        }
        let policy = if args[3].eq_ignore_ascii_case("explicit") {
            ClaimPolicy::ExplicitOnly
        } else if args[3].eq_ignore_ascii_case("auto") {
            ClaimPolicy::AutoClaim
        } else {
            return Err("ClaimPolicy must be explicit or auto".into());
        };
        self.replace_game(AdjudicatedGame::new(policy))?;
        self.policy = policy;
        Ok(())
    }
    fn update(&mut self, out: &mut impl Write) -> io::Result<()> {
        if let Some(completion) = self.worker.poll()
            && let Some(running) = self.running.as_mut()
            && running.context.id() == completion.context().id()
            && self
                .worker
                .accept_result(&completion, &self.game, self.version)
                .is_ok()
        {
            // Infinite holds successful search observations until stop, but a
            // failed engine must be reported immediately in every mode.
            if let Err(error) = completion.result() {
                return Err(io::Error::other(format!(
                    "search request {} failed: {error}",
                    completion.context().id().get()
                )));
            }
            running.held = Some(completion);
        }
        let Some(running) = self.running.as_mut() else {
            return Ok(());
        };
        if running
            .deadline
            .is_some_and(|deadline| Instant::now() >= deadline)
        {
            self.worker.cancel();
            running.deadline = None;
        }
        if running.wait_for_stop || running.held.is_none() {
            return Ok(());
        }
        let running = self.running.take().expect("active UCI request");
        let completion = running.held.expect("completed UCI request");
        // Recheck immediately before publication in this serialized game loop.
        if self
            .worker
            .accept_result(&completion, &self.game, self.version)
            .is_err()
        {
            return Ok(());
        }
        let elapsed = running.started.elapsed().as_millis();
        let mut best = None;
        match completion.result() {
            Ok(result) => {
                let mut info = format!(
                    "info depth {} nodes {} time {elapsed}",
                    result.completed_depth, result.nodes
                );
                if let Some(score) = result.score {
                    if score.abs() >= MATE_SCORE - i32::from(MAX_DEPTH) {
                        let plies = MATE_SCORE - score.abs();
                        let moves = (plies + 1) / 2 * score.signum();
                        info.push_str(&format!(" score mate {moves}"));
                    } else {
                        info.push_str(&format!(" score cp {score}"));
                    }
                }
                let pv: Vec<_> = result
                    .principal_variation
                    .iter()
                    .filter_map(|choice| match choice {
                        SearchChoice::Move(action) => Some(action.to_string()),
                        SearchChoice::Claim(_) => None,
                    })
                    .collect();
                if !pv.is_empty() {
                    info.push_str(" pv ");
                    info.push_str(&pv.join(" "));
                }
                writeln!(out, "{info}")?;
                match result.decision {
                    Some(SearchDecision::Choose(SearchChoice::Move(action))) => best = Some(action),
                    Some(SearchDecision::Choose(SearchChoice::Claim(_))) => eprintln!(
                        "uci: draw claim available; UCI has no claim channel; returning legal fallback for controller adjudication"
                    ),
                    Some(SearchDecision::Finished(_)) => {}
                    None => eprintln!("uci: no completed iteration; returning legal fallback"),
                }
            }
            Err(error) => {
                // A failed search is an engine failure, never an apparently
                // successful legal fallback or invented work statistics.
                return Err(io::Error::other(format!(
                    "search request {} failed: {error}",
                    running.context.id().get()
                )));
            }
        }
        if best.is_none() && self.game.outcome().is_none() {
            let mut actions = self.game.position().legal_actions();
            actions.sort_by_key(ToString::to_string);
            best = actions.first().copied();
        }
        writeln!(
            out,
            "bestmove {}",
            best.map_or_else(|| "0000".into(), |a| a.to_string())
        )?;
        out.flush()
    }
}
fn parse_position(args: &[&str], policy: ClaimPolicy) -> Result<AdjudicatedGame, String> {
    let (mut game, rest) = match args {
        ["startpos", rest @ ..] => (AdjudicatedGame::new(policy), rest),
        ["fen", a, b, c, d, e, f, rest @ ..] => (
            AdjudicatedGame::from_fen(&[*a, *b, *c, *d, *e, *f].join(" "), policy)
                .map_err(|e| e.to_string())?,
            rest,
        ),
        _ => return Err("position requires startpos or six-field fen".into()),
    };
    let moves = match rest {
        [] => &[][..],
        ["moves", moves @ ..] => moves,
        _ => return Err("expected moves after position".into()),
    };
    for (index, text) in moves.iter().enumerate() {
        let action = text
            .parse()
            .map_err(|e| format!("move {}: {e}", index + 1))?;
        game.apply(action)
            .map_err(|e| format!("move {}: {e}", index + 1))?;
    }
    Ok(game)
}
#[derive(Debug)]
struct Go {
    limits: SearchLimits,
    budget: Option<Duration>,
    infinite: bool,
}
fn number(text: &str) -> Result<u64, String> {
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return Err("limit must be an unsigned decimal integer".into());
    }
    text.parse().map_err(|_| "limit exceeds u64".into())
}
fn parse_go(args: &[&str], side: Color) -> Result<Go, String> {
    let mut depth = None;
    let mut nodes = None;
    let mut movetime = None;
    let mut time = [None, None];
    let mut increment = [0, 0];
    let mut moves_to_go = 30;
    let mut infinite = false;
    let mut index = 0;
    let mut seen = Vec::new();
    while index < args.len() {
        let token = args[index];
        index += 1;
        if matches!(token, "ponder" | "searchmoves" | "mate") {
            return Err(format!("unsupported go control: {token}"));
        }
        if token == "infinite" {
            infinite = true;
            continue;
        }
        if !matches!(
            token,
            "depth" | "nodes" | "movetime" | "wtime" | "btime" | "winc" | "binc" | "movestogo"
        ) {
            continue;
        }
        if seen.contains(&token) {
            return Err(format!("duplicate go control: {token}"));
        }
        seen.push(token);
        let value = number(
            args.get(index)
                .ok_or_else(|| format!("missing value for {token}"))?,
        )?;
        index += 1;
        match token {
            "depth" => {
                if value == 0 || value > u64::from(MAX_DEPTH) {
                    return Err(format!("depth must be in 1..={MAX_DEPTH}"));
                }
                depth = Some(value as u16);
            }
            "nodes" => nodes = Some(value),
            "movetime" => movetime = Some(value),
            "wtime" => time[0] = Some(value),
            "btime" => time[1] = Some(value),
            "winc" => increment[0] = value,
            "binc" => increment[1] = value,
            "movestogo" => {
                if value == 0 {
                    return Err("movestogo must be positive".into());
                }
                moves_to_go = value;
            }
            _ => unreachable!(),
        }
    }
    let own = usize::from(side == Color::Black);
    let millis = movetime.or_else(|| {
        time[own].map(|remaining| {
            let spendable = remaining.saturating_sub((remaining / 20).min(50));
            (remaining / moves_to_go)
                .saturating_add(increment[own])
                .min(spendable)
        })
    });
    let clock_controls = seen
        .iter()
        .any(|token| matches!(*token, "wtime" | "btime" | "winc" | "binc" | "movestogo"));
    if clock_controls && millis.is_none() && depth.is_none() && nodes.is_none() && !infinite {
        return Err(
            "clock controls require the moving side's remaining time or an explicit work limit"
                .into(),
        );
    }
    if infinite && millis.is_some() {
        return Err("infinite conflicts with a wall-clock budget".into());
    }
    let infinite = infinite || (depth.is_none() && nodes.is_none() && millis.is_none());
    Ok(Go {
        limits: SearchLimits {
            depth: depth.unwrap_or(MAX_DEPTH),
            nodes: if millis == Some(0) { Some(0) } else { nodes },
        },
        budget: millis.map(Duration::from_millis),
        infinite,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_clock_allocations_and_numeric_limits() {
        let go = parse_go(
            &[
                "wtime",
                "10000",
                "btime",
                "800",
                "winc",
                "100",
                "binc",
                "50",
                "movestogo",
                "2",
            ],
            Color::Black,
        )
        .unwrap();
        assert_eq!(go.budget, Some(Duration::from_millis(450)));
        let go = parse_go(
            &["wtime", "10000", "winc", "100", "movestogo", "10"],
            Color::White,
        )
        .unwrap();
        assert_eq!(go.budget, Some(Duration::from_millis(1100)));
        let go = parse_go(&["wtime", "0"], Color::White).unwrap();
        assert_eq!(go.budget, Some(Duration::ZERO));
        assert_eq!(go.limits.nodes, Some(0));
        assert_eq!(
            parse_go(&["wtime", "10000", "movetime", "7"], Color::White)
                .unwrap()
                .budget,
            Some(Duration::from_millis(7))
        );
        assert!(parse_go(&["wtime", "1000"], Color::Black).is_err());
        assert!(parse_go(&["winc", "100"], Color::White).is_err());
        assert!(parse_go(&["winc", "100", "depth", "1"], Color::White).is_ok());
        for args in [
            vec!["depth", "0"],
            vec!["nodes", "-1"],
            vec!["depth", "129"],
            vec!["nodes", "18446744073709551616"],
            vec!["depth", "1", "depth", "2"],
            vec!["movestogo", "0"],
            vec!["movetime"],
            vec!["infinite", "movetime", "1"],
        ] {
            assert!(parse_go(&args, Color::White).is_err(), "{args:?}");
        }
    }
    #[test]
    fn cr_lf_and_oversized_input_are_bounded_and_recover() {
        let mut input = std::io::Cursor::new(b"uci\risready\r\nquit\n");
        for expected in ["uci", "isready", "", "quit"] {
            assert_eq!(
                bounded_uci_line(&mut input).unwrap().unwrap().unwrap(),
                expected.as_bytes()
            );
        }
        assert!(bounded_uci_line(&mut input).unwrap().is_none());
        let mut input = std::io::Cursor::new(format!("{}\risready\r", "x".repeat(5000)));
        assert!(bounded_uci_line(&mut input).unwrap().unwrap().is_err());
        assert_eq!(
            bounded_uci_line(&mut input).unwrap().unwrap().unwrap(),
            b"isready"
        );
    }
}
