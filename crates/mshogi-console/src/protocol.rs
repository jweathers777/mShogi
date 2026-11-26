//! Shared USI and Xiangqi UCI command lifecycle.
use crate::input::{self, Ending};
use mshogi_engine::{EngineGame, EngineResult, EngineTelemetry, WorkLimits};
use mshogi_search::alphabeta::{
    MATE_BOUND, Searcher,
    timing::{RequestTime, TimeBudget},
};
use std::{
    fmt::Display,
    io::{self, Write},
    process::ExitCode,
    str::FromStr,
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, RecvTimeoutError},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub trait Dialect:
    EngineGame<Game: Send + Sync, Choice: Send, Side: Send, Pending: Send, Conclusion: Send>
{
    type Root: Eq;
    type Action: Copy + Eq + Display + FromStr<Err: Display> + Send + Sync;
    const PROTOCOL: &'static str;
    const GAME: &'static str;
    const PROFILE: &'static str;
    const DEFAULT_DEPTH: u16;
    const DEFAULT_NODES: u64;
    const DEFAULT_HASH: usize;
    const SCORE_DETAIL: &'static str;
    const USI: bool;

    fn start() -> Self::Game;
    fn root(game: &Self::Game) -> &Self::Root;
    fn parse_root<'a>(words: &'a [&str]) -> Result<(Self::Root, &'a [&'a str]), String>;
    fn from_root(root: Self::Root) -> Result<Self::Game, String>;
    fn black_clock(side: Self::Side) -> bool;
    fn play(action: Self::Action) -> Self::Command;
    fn action(choice: &Self::Choice) -> Option<Self::Action>;
    fn special_choice(game: &Self::Game, choice: &Self::Choice) -> io::Result<String>;
    fn terminal_move(game: &Self::Game, conclusion: Self::Conclusion) -> Option<&'static str>;
}

// All positions start from a native root and contain only admitted board moves.
// Each dialect has a fixed claim policy. Together, root and moves identify
// the complete native history, including automatic events.
struct Position<D: Dialect> {
    game: D::Game,
    moves: Vec<D::Action>,
}
impl<D: Dialect> Position<D> {
    fn start() -> Self {
        Self {
            game: D::start(),
            moves: Vec::new(),
        }
    }

    fn is_prefix_of(&self, root: &D::Root, moves: &[D::Action]) -> bool {
        D::root(&self.game) == root && moves.starts_with(&self.moves)
    }

    fn parse(&self, words: &[&str]) -> Result<Self, String> {
        let (root, rest) = D::parse_root(words)?;
        let moves = match rest {
            [] => &[][..],
            ["moves", moves @ ..] => moves,
            _ => return Err("expected moves suffix".into()),
        };
        let moves = moves
            .iter()
            .enumerate()
            .map(|(i, word)| {
                word.parse::<D::Action>()
                    .map_err(|e| format!("move {}: {e}", i + 1))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let (mut game, offset) = if self.is_prefix_of(&root, &moves) {
            (self.game.clone(), self.moves.len())
        } else {
            (D::from_root(root)?, 0)
        };
        for (i, &action) in moves.iter().enumerate().skip(offset) {
            let actor = D::side_to_move(&game);
            D::submit(&mut game, actor, D::play(action), &mut || false)
                .map_err(|e| format!("move {} native rejection: {e}", i + 1))?;
        }
        Ok(Self { game, moves })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Controls {
    limits: WorkLimits,
    time: Option<TimeBudget>,
    hold: bool,
}
fn default_limits<D: Dialect>() -> WorkLimits {
    WorkLimits {
        depth: D::DEFAULT_DEPTH,
        nodes: D::DEFAULT_NODES,
        time_ms: None,
    }
}
fn validate<D: Dialect>(limits: WorkLimits) -> Result<(), String> {
    if D::valid_limits(limits) {
        Ok(())
    } else {
        Err(format!(
            "limits outside depth 1..{}/nodes 0..{}",
            D::MAX_DEPTH,
            D::MAX_NODES
        ))
    }
}

fn number(text: &str) -> Result<u64, String> {
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return Err("expected unsigned ASCII decimal".into());
    }
    text.parse().map_err(|_| "number exceeds u64".into())
}

fn parse_go<D: Dialect>(
    args: &[&str],
    mut limits: WorkLimits,
    mut fixed: bool,
    side: D::Side,
) -> Result<Controls, String> {
    let mut movetime = None;
    let mut clock = [None, None];
    let mut increment = [0, 0];
    let mut byoyomi = 0;
    let mut moves = 30;
    let mut hold = false;
    let mut clocked = false;
    let mut seen = Vec::new();
    let mut words = args.iter().copied();
    while let Some(key) = words.next() {
        if seen.contains(&key) {
            return Err("duplicate go control".into());
        }
        seen.push(key);
        if key == "infinite" {
            hold = true;
            continue;
        }
        let mut value = || number(words.next().ok_or("missing go value")?);
        match key {
            "depth" => {
                limits.depth = value()?.try_into().map_err(|_| "unsupported depth")?;
                fixed = true;
            }
            "nodes" => {
                limits.nodes = value()?;
                fixed = true;
            }
            "movetime" => {
                let ms = value()?;
                if !(1..=60_000).contains(&ms) {
                    return Err("movetime outside 1..60000ms".into());
                }
                movetime = Some(ms);
            }
            "btime" | "wtime" => {
                clocked = true;
                clock[usize::from(key == "btime")] = Some(value()?);
            }
            "binc" | "winc" => {
                clocked = true;
                increment[usize::from(key == "binc")] = value()?;
            }
            "movestogo" => {
                clocked = true;
                moves = value()?;
                if moves == 0 {
                    return Err("movestogo must be positive".into());
                }
            }
            "byoyomi" if D::USI => {
                clocked = true;
                byoyomi = value()?;
            }
            _ => return Err(format!("unsupported go control {key}")),
        }
    }
    validate::<D>(limits)?;
    let own = usize::from(D::black_clock(side));
    if clocked && clock[own].is_none() {
        return Err("clock controls require the moving side's remaining time".into());
    }
    if (movetime.is_some() || clocked) && (fixed || hold) {
        return Err(
            "time controls cannot combine with fixed work, fixed options or infinite".into(),
        );
    }
    if movetime.is_some() && clocked {
        return Err("movetime cannot combine with clock controls".into());
    }
    let time = if fixed || hold {
        None
    } else {
        limits.depth = D::MAX_DEPTH;
        limits.nodes = D::MAX_NODES;
        Some(match clock[own] {
            Some(ms) => {
                TimeBudget::from_clock_ms(ms, increment[own].saturating_add(byoyomi), moves)
                    .with_response_margin()
            }
            None => TimeBudget::from_hard_ms(movetime.unwrap_or(1_000)),
        })
    };
    Ok(Controls { limits, time, hold })
}

// An Arc identifies the exact request, including its immutable game, timing,
// hash size and generation. Retiring it makes every late completion stale.
struct Request<D: Dialect> {
    position: Arc<Position<D>>,
    controls: Controls,
    generation: u64,
    hash: usize,
    stop: AtomicBool,
    started: Instant,
}
struct Completion<D: Dialect> {
    request: Arc<Request<D>>,
    result: Result<EngineResult<D>, String>,
}
struct Mailbox<D: Dialect> {
    pending: Option<Arc<Request<D>>>,
    completed: Option<Completion<D>>,
    shutdown: bool,
}
struct Worker<D: Dialect> {
    shared: Arc<(Mutex<Mailbox<D>>, Condvar)>,
    handle: JoinHandle<()>,
}
impl<D: Dialect> Worker<D> {
    fn new() -> io::Result<Self> {
        let state: Mailbox<D> = Mailbox {
            pending: None,
            completed: None,
            shutdown: false,
        };
        let shared = Arc::new((Mutex::new(state), Condvar::new()));
        let work = Arc::clone(&shared);
        let handle = thread::Builder::new()
            .name(format!("mshogi-{}-search", D::GAME))
            .spawn(move || {
                let (lock, wake) = &*work;
                let mut retained: Option<(Arc<Request<D>>, Searcher<D::Position>)> = None;
                loop {
                    let request = {
                        let mut mailbox = wake
                            .wait_while(lock.lock().unwrap(), |m| {
                                m.pending.is_none() && !m.shutdown
                            })
                            .unwrap();
                        if mailbox.shutdown {
                            return;
                        }
                        mailbox.pending.take().unwrap()
                    };
                    let timing = RequestTime::new(request.controls.time);
                    let mut fresh;
                    let engine = if request.controls.time.is_some() {
                        let (last, engine) = retained.get_or_insert_with(|| {
                            (Arc::clone(&request), Searcher::new(request.hash))
                        });
                        if last.hash != request.hash {
                            *engine = Searcher::new(request.hash);
                        } else if last.generation != request.generation
                            || !last.position.is_prefix_of(
                                D::root(&request.position.game),
                                &request.position.moves,
                            )
                        {
                            engine.clear();
                        }
                        *last = Arc::clone(&request);
                        engine
                    } else {
                        fresh = Searcher::new(request.hash);
                        &mut fresh
                    };
                    let result = D::search(
                        &request.position.game,
                        request.controls.limits,
                        timing.remaining(),
                        engine,
                        || request.stop.load(Ordering::Acquire),
                    );
                    lock.lock().unwrap().completed = Some(Completion { request, result });
                }
            })?;
        Ok(Self { shared, handle })
    }

    fn submit(&self, request: Arc<Request<D>>) {
        let mut mailbox = self.shared.0.lock().unwrap();
        mailbox.pending = Some(request);
        mailbox.completed = None;
        self.shared.1.notify_one();
    }

    fn retire(&self) {
        let mut mailbox = self.shared.0.lock().unwrap();
        mailbox.pending = None;
        mailbox.completed = None;
    }

    fn shutdown(self) {
        self.shared.0.lock().unwrap().shutdown = true;
        self.shared.1.notify_one();
        let _ = self.handle.join();
    }
}

struct Running<D: Dialect> {
    request: Arc<Request<D>>,
    hold: bool,
    result: Option<EngineResult<D>>,
}
struct Adapter<D: Dialect> {
    position: Arc<Position<D>>,
    limits: WorkLimits,
    fixed_options: bool,
    hash: usize,
    generation: u64,
    ended: bool,
    running: Option<Running<D>>,
    worker: Worker<D>,
}
impl<D: Dialect> Adapter<D> {
    fn retire(&mut self) {
        if let Some(active) = self.running.take() {
            active.request.stop.store(true, Ordering::Release);
        }
        self.worker.retire();
    }

    fn set_option(&mut self, args: &[&str]) -> Result<(), String> {
        let ["name", name, "value", value] = args else {
            return Err(
                "expected setoption name Depth|Nodes|Hash|RulesProfile value <value>".into(),
            );
        };
        let mut limits = self.limits;
        let mut hash = self.hash;
        match *name {
            "Depth" => {
                limits.depth = number(value)?
                    .try_into()
                    .map_err(|_| "depth exceeds supported bounds")?
            }
            "Nodes" => limits.nodes = number(value)?,
            "Hash" => {
                let mb = number(value)?;
                if !(1..=MAX_HASH).contains(&mb) {
                    return Err(format!("Hash outside 1..{MAX_HASH}"));
                }
                hash = mb as usize;
            }
            "RulesProfile" if *value == D::PROFILE => {}
            _ => return Err("unsupported option/value; rules profile is immutable".into()),
        }
        validate::<D>(limits)?;
        self.retire();
        self.limits = limits;
        self.hash = hash;
        self.fixed_options |= matches!(*name, "Depth" | "Nodes");
        Ok(())
    }

    fn go(&mut self, args: &[&str]) -> Result<(), String> {
        if self.ended {
            return Err("gameover retired this game; send position or usinewgame".into());
        }
        let controls = parse_go::<D>(
            args,
            self.limits,
            self.fixed_options,
            D::side_to_move(&self.position.game),
        )?;
        let request = Arc::new(Request {
            position: Arc::clone(&self.position),
            controls,
            hash: self.hash,
            generation: self.generation,
            stop: AtomicBool::new(false),
            started: Instant::now(),
        });
        self.worker.submit(Arc::clone(&request));
        self.running = Some(Running {
            request,
            hold: controls.hold,
            result: None,
        });
        Ok(())
    }

    fn command(&mut self, line: &str, out: &mut impl Write) -> io::Result<bool> {
        let words: Vec<_> = line.split_ascii_whitespace().collect();
        let Some((&cmd, args)) = words.split_first() else {
            return Ok(false);
        };
        let result = match (cmd, args) {
            (cmd, []) if cmd == D::PROTOCOL => {
                writeln!(
                    out,
                    "id name mShogi {} {}",
                    env!("CARGO_PKG_VERSION"),
                    D::GAME
                )?;
                writeln!(out, "id author John Weathers")?;
                writeln!(
                    out,
                    "option name Depth type spin default {} min 1 max {}",
                    D::DEFAULT_DEPTH,
                    D::MAX_DEPTH
                )?;
                writeln!(
                    out,
                    "option name Nodes type spin default {} min 0 max {}",
                    D::DEFAULT_NODES,
                    D::MAX_NODES
                )?;
                writeln!(
                    out,
                    "option name Hash type spin default {} min 1 max {MAX_HASH}",
                    D::DEFAULT_HASH
                )?;
                writeln!(
                    out,
                    "option name RulesProfile type combo default {0} var {0}",
                    D::PROFILE
                )?;
                writeln!(out, "{}ok", D::PROTOCOL)?;
                out.flush()?;
                Ok(())
            }
            ("isready", []) => {
                writeln!(out, "readyok")?;
                out.flush()?;
                Ok(())
            }
            ("quit", []) => return Ok(true),
            ("stop", []) => {
                if let Some(r) = &mut self.running {
                    r.hold = false;
                    r.request.stop.store(true, Ordering::Release);
                }
                Ok(())
            }
            ("usinewgame", []) if D::USI => {
                self.new_game();
                Ok(())
            }
            ("ucinewgame", []) if !D::USI => {
                self.new_game();
                Ok(())
            }
            ("position", _) => {
                self.retire();
                self.position.parse(args).map(|p| {
                    self.position = Arc::new(p);
                    self.ended = false;
                })
            }
            ("setoption", _) => self.set_option(args),
            ("gameover", ["win" | "lose" | "draw"]) if D::USI => {
                self.retire();
                self.ended = true;
                Ok(())
            }
            ("go", ["mate", ..]) if D::USI => {
                self.retire();
                writeln!(out, "checkmate notimplemented")?;
                out.flush()?;
                Ok(())
            }
            ("go", _) => {
                self.retire();
                self.go(args)
            }
            _ => Err("unsupported command/parameters".into()),
        };
        if let Err(e) = result {
            eprintln!("{}: {}", D::PROTOCOL, crate::record_io::diagnostic(&e));
        }
        Ok(false)
    }

    fn new_game(&mut self) {
        self.retire();
        self.position = Arc::new(Position::start());
        self.generation += 1;
        self.ended = false;
    }

    fn update(&mut self, out: &mut impl Write) -> io::Result<()> {
        let completed = self.worker.shared.0.lock().unwrap().completed.take();
        let Some(r) = &mut self.running else {
            return Ok(());
        };
        if let Some(done) = completed
            && Arc::ptr_eq(&done.request, &r.request)
        {
            let result = done.result.map_err(io::Error::other)?;
            let t = result.telemetry();
            report::<D>(t, r.request.started.elapsed(), out)?;
            match &result {
                EngineResult::Incomplete { .. } => {
                    return Err(io::Error::other(format!(
                        "incomplete: no completed iteration (nodes {}, status {:?})",
                        t.nodes, t.status
                    )));
                }
                EngineResult::Unresolved { pending, .. } => {
                    return Err(io::Error::other(format!(
                        "unresolved: {pending:?} (nodes {}, status {:?})",
                        t.nodes, t.status
                    )));
                }
                _ => r.result = Some(result),
            }
        }
        if r.hold {
            return Ok(());
        }
        let Some(result) = r.result.take() else {
            return Ok(());
        };
        self.running = None;
        let game = &self.position.game;
        let best = match result {
            EngineResult::Choice { choice, .. } => {
                let mut candidate = game.clone();
                let (actor, command) = D::command(&choice, D::side_to_move(game));
                D::submit(&mut candidate, actor, command, &mut || false)
                    .map_err(|e| io::Error::other(format!("searched choice rejected: {e}")))?;
                match D::action(&choice) {
                    Some(action) => action.to_string(),
                    None => D::special_choice(&candidate, &choice)?,
                }
            }
            EngineResult::Finished { conclusion, .. } => {
                let Some(token) = D::terminal_move(game, conclusion) else {
                    writeln!(out, "info string mshogi native_terminal {conclusion:?}")?;
                    return Err(io::Error::other(
                        "native_terminal: host must adjudicate before search",
                    ));
                };
                token.into()
            }
            _ => unreachable!("only complete decisions are held"),
        };
        writeln!(out, "bestmove {best}")?;
        out.flush()
    }
}
const MAX_HASH: u64 = 1024;

fn report<D: Dialect>(
    t: &EngineTelemetry<D>,
    elapsed: Duration,
    out: &mut impl Write,
) -> io::Result<()> {
    write!(
        out,
        "info depth {} nodes {} time {}",
        t.completed_depth,
        t.nodes,
        elapsed.as_millis()
    )?;
    if D::USI
        && let Some(score) = t.score
        && score.abs() < MATE_BOUND
    {
        write!(out, " score cp {score}")?;
    }
    let mut pv = t.principal_variation.iter().map_while(D::action);
    if let Some(first) = pv.next() {
        write!(out, " pv {first}")?;
        for action in pv {
            write!(out, " {action}")?;
        }
    }
    writeln!(out)?;
    writeln!(
        out,
        "info string mshogi native_score {:?} perspective {:?} status {:?} distance_units command_edges{}",
        t.score,
        t.perspective,
        t.status,
        D::SCORE_DETAIL
    )?;
    out.flush()
}

pub fn run<D: Dialect>() -> ExitCode {
    let worker = match Worker::new() {
        Ok(w) => w,
        Err(e) => {
            eprintln!("{}: worker: {e}", D::PROTOCOL);
            return ExitCode::FAILURE;
        }
    };
    let mut adapter = Adapter::<D> {
        position: Arc::new(Position::start()),
        limits: default_limits::<D>(),
        fixed_options: false,
        hash: D::DEFAULT_HASH,
        generation: 0,
        ended: false,
        running: None,
        worker,
    };
    let mut out = io::stdout().lock();
    let result = event_loop(&mut adapter, &mut out);
    adapter.retire();
    adapter.worker.shutdown();
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            let message = crate::record_io::diagnostic(&e.to_string());
            let _ = writeln!(out, "info string mshogi execution_failure {message}");
            let _ = out.flush();
            eprintln!("{}: {message}", D::PROTOCOL);
            ExitCode::FAILURE
        }
    }
}

fn event_loop<D: Dialect>(adapter: &mut Adapter<D>, out: &mut impl Write) -> io::Result<()> {
    let (send, recv) = mpsc::sync_channel(32);
    thread::Builder::new()
        .name(format!("mshogi-{}-input", D::GAME))
        .spawn(move || {
            let mut stdin = io::stdin().lock();
            loop {
                let event = input::line(&mut stdin, Ending::Protocol);
                let ended = !matches!(event, Ok(Some(_)));
                if send.send(event).is_err() || ended {
                    break;
                }
            }
        })?;
    loop {
        match recv.recv_timeout(Duration::from_millis(2)) {
            Ok(Ok(Some(bytes))) => match input::protocol_text(&bytes) {
                Some(line) => {
                    if adapter.command(line, out)? {
                        return Ok(());
                    }
                }
                None => eprintln!(
                    "{}: input must be ASCII text without control bytes",
                    D::PROTOCOL
                ),
            },
            Ok(Err(e)) => return Err(e),
            Ok(Ok(None)) | Err(RecvTimeoutError::Disconnected) => return Ok(()),
            Err(RecvTimeoutError::Timeout) => {}
        }
        adapter.update(out)?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mshogi_core::shogi::Side;
    use mshogi_engine::game::{Shogi, Xiangqi};
    use mshogi_search::shogi as search;
    fn controls(
        args: &[&str],
        fixed: bool,
        side: Side,
    ) -> Result<(WorkLimits, bool, Option<TimeBudget>), String> {
        let c = parse_go::<Shogi>(args, default_limits::<Shogi>(), fixed, side)?;
        Ok((c.limits, c.hold, c.time))
    }
    #[test]
    fn default_time_and_explicit_fixed_work_are_distinct() {
        let (limits, hold, time) = controls(&[], false, Side::Sente).unwrap();
        assert_eq!(time, Some(TimeBudget::from_hard_ms(1_000)));
        assert!(!hold);
        assert_eq!(
            limits,
            WorkLimits {
                time_ms: None,
                depth: 64,
                nodes: 5_000_000
            }
        );
        let (limits, _, time) =
            controls(&["depth", "3", "nodes", "20000"], false, Side::Sente).unwrap();
        assert_eq!(
            limits,
            WorkLimits {
                time_ms: None,
                depth: 3,
                nodes: 20_000
            }
        );
        assert_eq!(time, None);
        assert_eq!(controls(&[], true, Side::Sente).unwrap().2, None);
        assert_eq!(controls(&["infinite"], false, Side::Sente).unwrap().2, None);
        assert_eq!(
            controls(&["movetime", "125"], false, Side::Sente)
                .unwrap()
                .2,
            Some(TimeBudget::from_hard_ms(125))
        );
    }
    #[test]
    fn incompatible_or_unbounded_time_controls_are_refused() {
        for args in [
            vec!["movetime", "0"],
            vec!["movetime", "60001"],
            vec!["movetime", "100", "depth", "2"],
            vec!["movetime", "100", "infinite"],
            vec!["movetime", "100", "movetime", "200"],
        ] {
            assert!(controls(&args, false, Side::Sente).is_err());
        }
        assert!(controls(&["movetime", "100"], true, Side::Sente).is_err());
        for args in [
            vec!["btime", "1000", "depth", "2"],
            vec!["btime", "1000", "movetime", "10"],
            vec!["btime", "1000", "infinite"],
            vec!["wtime", "1000"],
            vec!["btime", "1000", "ponder"],
        ] {
            assert!(controls(&args, false, Side::Sente).is_err(), "{args:?}");
        }
    }
    #[test]
    fn fischer_clock_uses_the_moving_sides_time_and_increment() {
        let args = [
            "btime", "60000", "wtime", "30000", "binc", "1000", "winc", "500",
        ];
        let (limits, hold, sente) = controls(&args, false, Side::Sente).unwrap();
        assert!(!hold);
        assert_eq!(limits.depth, search::MAX_DEPTH);
        let expected = TimeBudget::from_clock_ms(60_000, 1_000, 30).with_response_margin();
        assert_eq!(sente, Some(expected));
        let (_, _, gote) = controls(&args, false, Side::Gote).unwrap();
        assert_eq!(
            gote,
            Some(TimeBudget::from_clock_ms(30_000, 500, 30).with_response_margin())
        );
        assert!(
            sente.unwrap().clock && sente.unwrap().hard < std::time::Duration::from_millis(60_000)
        );
    }

    #[test]
    fn moves_to_go_and_bare_go_agree_across_dialects() {
        let args = ["btime", "6000", "wtime", "9000", "movestogo", "3"];
        let shogi =
            |args: &[&str]| parse_go::<Shogi>(args, default_limits::<Shogi>(), false, Side::Sente);
        let xiangqi = |args: &[&str]| {
            parse_go::<Xiangqi>(
                args,
                default_limits::<Xiangqi>(),
                false,
                mshogi_core::xiangqi::Side::Black,
            )
        };
        assert_eq!(
            xiangqi(&[]).unwrap().time,
            Some(TimeBudget::from_hard_ms(1000))
        );
        for go in [shogi(&args), xiangqi(&args)] {
            assert_eq!(
                go.unwrap().time,
                Some(TimeBudget::from_clock_ms(6000, 0, 3).with_response_margin())
            );
        }
        assert!(xiangqi(&["btime", "6000", "movestogo", "0"]).is_err());
        assert!(xiangqi(&["movestogo", "1"]).is_err());
        assert!(xiangqi(&["btime", "6000", "byoyomi", "100"]).is_err());
        assert_eq!(
            shogi(&["btime", "6000", "byoyomi", "100"]).unwrap().time,
            Some(TimeBudget::from_clock_ms(6000, 100, 30).with_response_margin())
        );
    }
}
