//! Bounded Pikafish-compatible UCI0-9 over authoritative native Xiangqi.
//! Ordinary search is sound_fast_only; UCI cannot request exact proof or native claims.
use mshogi_core::xiangqi::*;
use mshogi_search::xiangqi::{self as search, Choice, Decision, Limits, SearchResult};
use std::{
    io::{self, Write},
    process::ExitCode,
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, RecvTimeoutError},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
const TICK: Duration = Duration::from_millis(2);
#[derive(Clone)]
struct Context {
    id: u64,
    version: u64,
    game: GameContext,
    limits: Limits,
}
struct Job {
    context: Context,
    game: Game,
    stop: Arc<AtomicBool>,
}
struct Completion {
    context: Context,
    result: Result<SearchResult, String>,
}
#[derive(Default)]
struct Mailbox {
    pending: Option<Job>,
    completed: Option<Completion>,
    shutdown: bool,
}
struct Worker {
    shared: Arc<(Mutex<Mailbox>, Condvar)>,
    handle: Option<JoinHandle<()>>,
}
impl Worker {
    fn new() -> io::Result<Self> {
        let shared = Arc::new((Mutex::new(Mailbox::default()), Condvar::new()));
        let work = shared.clone();
        let handle = thread::Builder::new()
            .name("mshogi-xiangqi-uci-search".into())
            .spawn(move || {
                loop {
                    let job = {
                        let (lock, wake) = &*work;
                        let mut box_ = lock.lock().unwrap();
                        while box_.pending.is_none() && !box_.shutdown {
                            box_ = wake.wait(box_).unwrap();
                        }
                        if box_.shutdown {
                            break;
                        }
                        box_.pending.take().unwrap()
                    };
                    let result = search::search_with_stop(&job.game, job.context.limits, || {
                        job.stop.load(Ordering::Acquire)
                    })
                    .map_err(|e| e.to_string());
                    let mut box_ = work.0.lock().unwrap();
                    if box_.shutdown {
                        break;
                    }
                    box_.completed = Some(Completion {
                        context: job.context,
                        result,
                    });
                }
            })?;
        Ok(Self {
            shared,
            handle: Some(handle),
        })
    }
    fn submit(&self, job: Job) {
        let mut box_ = self.shared.0.lock().unwrap();
        box_.pending = Some(job);
        box_.completed = None;
        self.shared.1.notify_one();
    }
    fn retire(&self) {
        let mut b = self.shared.0.lock().unwrap();
        b.pending = None;
        b.completed = None;
    }
    fn poll(&self) -> Option<Completion> {
        self.shared.0.lock().unwrap().completed.take()
    }
    fn shutdown(&mut self) {
        let mut b = self.shared.0.lock().unwrap();
        b.shutdown = true;
        b.pending = None;
        self.shared.1.notify_one();
        drop(b);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
struct Running {
    context: Context,
    stop: Arc<AtomicBool>,
    started: Instant,
    hold: bool,
    result: Option<SearchResult>,
}
struct Adapter {
    game: Game,
    version: u64,
    next_id: u64,
    limits: Limits,
    running: Option<Running>,
    worker: Worker,
    ended: bool,
}
impl Adapter {
    fn retire(&mut self) -> Result<(), String> {
        if let Some(active) = self.running.take() {
            active.stop.store(true, Ordering::Release);
        }
        self.worker.retire();
        self.version = self
            .version
            .checked_add(1)
            .ok_or("session version exhausted")?;
        Ok(())
    }
    fn command(&mut self, line: &str, out: &mut impl Write) -> io::Result<bool> {
        let words: Vec<_> = line.split_ascii_whitespace().collect();
        let Some((&command, args)) = words.split_first() else {
            return Ok(false);
        };
        let result: Result<(), String> = match command {
            "uci" if args.is_empty() => {
                writeln!(out, "id name mShogi {} xiangqi", env!("CARGO_PKG_VERSION"))?;
                writeln!(out, "id author John Weathers")?;
                writeln!(
                    out,
                    "option name Depth type spin default 1 min 1 max {}",
                    search::MAX_DEPTH
                )?;
                writeln!(
                    out,
                    "option name Nodes type spin default 128 min 0 max {}",
                    search::MAX_NODES
                )?;
                writeln!(
                    out,
                    "option name RulesProfile type combo default {PROFILE_ID} var {PROFILE_ID}"
                )?;
                writeln!(out, "uciok")?;
                out.flush()?;
                Ok(())
            }
            "isready" if args.is_empty() => {
                writeln!(out, "readyok")?;
                out.flush()?;
                Ok(())
            }
            "quit" if args.is_empty() => return Ok(true),
            "stop" if args.is_empty() => {
                if let Some(r) = self.running.as_mut() {
                    r.hold = false;
                    r.stop.store(true, Ordering::Release);
                }
                Ok(())
            }
            "ucinewgame" if args.is_empty() => self.retire().map(|()| {
                self.game = Game::standard(ClaimPolicy::AutoValidClaims).expect("standard native root");
                self.ended = false;
            }),
            "position" => self
                .retire()
                .and_then(|()| parse_position(args))
                .map(|game| {
                    self.game = game;
                    self.ended = false;
                }),
            "setoption" => self.set_option(args),
            "go" => self.retire().and_then(|()| {
                if self.ended {
                    return Err("gameover retired this game; send position or ucinewgame".into());
                }
                let (limits, hold) = parse_go(args, self.limits)?;
                self.next_id = self
                    .next_id
                    .checked_add(1)
                    .ok_or("request identity exhausted")?;
                let context = Context {
                    id: self.next_id,
                    version: self.version,
                    game: self.game.context(),
                    limits,
                };
                let stop = Arc::new(AtomicBool::new(false));
                self.worker.submit(Job {
                    context: context.clone(),
                    game: GameRecord::from_game(&self.game).and_then(|record| record.to_game()).map_err(|e| e.to_string())?,
                    stop: stop.clone(),
                });
                self.running = Some(Running {
                    context,
                    stop,
                    started: Instant::now(),
                    hold,
                    result: None,
                });
                Ok(())
            }),
            _ => Err(
                "unsupported command/parameters; untimed Xiangqi UCI0-9 only (no clocks, pondering, claims or exact proof)"
                    .into(),
            ),
        };
        if let Err(e) = result {
            eprintln!("xiangqi-uci: {}", crate::record_io::diagnostic(&e));
        }
        Ok(false)
    }
    fn set_option(&mut self, args: &[&str]) -> Result<(), String> {
        let ["name", name, "value", value] = args else {
            return Err("expected setoption name Depth|Nodes|RulesProfile value <value>".into());
        };
        let mut limits = self.limits;
        match *name {
            "Depth" => {
                limits.depth = number(value)?
                    .try_into()
                    .map_err(|_| "depth exceeds supported bounds")?
            }
            "Nodes" => limits.nodes = number(value)?,
            "RulesProfile" if *value == PROFILE_ID => {}
            _ => return Err("unsupported option/value; rules profile is immutable".into()),
        }
        if !limits.valid() {
            return Err("limits outside depth1..3/nodes0..512".into());
        }
        self.retire()?;
        self.limits = limits;
        Ok(())
    }
    fn update(&mut self, out: &mut impl Write) -> io::Result<()> {
        if let Some(completion) = self.worker.poll()
            && let Some(r) = self.running.as_mut()
            && completion.context.id == r.context.id
            && completion.context.version == self.version
            && completion.context.game == self.game.context()
            && completion.context.limits == r.context.limits
        {
            let result = completion.result.map_err(io::Error::other)?;
            report(&result, r.started.elapsed(), out)?;
            if result.decision.is_none() {
                return Err(io::Error::other(format!(
                    "{}: no completed decision (nodes {}, status {:?}, unresolved {:?})",
                    if result.status == search::Status::Unresolved {
                        "unresolved"
                    } else {
                        "incomplete"
                    },
                    result.nodes,
                    result.status,
                    result.unresolved
                )));
            }
            r.result = Some(result);
        }
        let Some(r) = self.running.as_ref() else {
            return Ok(());
        };
        if r.hold || r.result.is_none() {
            return Ok(());
        }
        let r = self.running.take().unwrap();
        if r.context.game != self.game.context() || r.context.version != self.version {
            return Ok(());
        }
        let result = r.result.unwrap();
        let text = match result.decision.unwrap() {
            Decision::Choose(choice) => {
                let mut candidate = GameRecord::from_game(&self.game)
                    .and_then(|record| record.to_game())
                    .map_err(io::Error::other)?;
                candidate
                    .submit(GameRequest {
                        expected: candidate.context(),
                        actor: candidate.position().side_to_move(),
                        command: choice.command(),
                    })
                    .map_err(|e| io::Error::other(format!("searched choice rejected: {e:?}")))?;
                match choice {
                    Choice::Move(action) => action.to_string(),
                    Choice::Claim { .. } => {
                        return Err(io::Error::other(
                            "unsupported_adjudication: UCI0-9 has no native claim channel; host must use auto_valid_claims",
                        ));
                    }
                }
            }
            Decision::Finished(conclusion) => {
                writeln!(out, "info string mshogi native_terminal {conclusion:?}")?;
                return Err(io::Error::other(
                    "native_terminal: UCI0-9 has no exact native conclusion representation; host adjudicates before go",
                ));
            }
            Decision::ExactNoWin => {
                return Err(io::Error::other(
                    "unsupported exact result in ordinary UCI search",
                ));
            }
        };
        writeln!(out, "bestmove {text}")?;
        out.flush()
    }
}
fn number(text: &str) -> Result<u64, String> {
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return Err("expected unsigned ASCII decimal".into());
    }
    text.parse().map_err(|_| "number exceeds u64".into())
}
fn parse_go(args: &[&str], mut limits: Limits) -> Result<(Limits, bool), String> {
    let mut hold = false;
    let mut seen = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let key = args[i];
        i += 1;
        if seen.contains(&key) {
            return Err("duplicate go control".into());
        }
        seen.push(key);
        match key {
            "infinite" => hold = true,
            "depth" | "nodes" => {
                let value = number(args.get(i).ok_or("missing go value")?)?;
                i += 1;
                if key == "depth" {
                    limits.depth = value.try_into().map_err(|_| "unsupported depth")?;
                } else {
                    limits.nodes = value;
                }
            }
            _ => {
                return Err(format!(
                    "unsupported go control {key}; no clocks, pondering, exact proof or searchmoves"
                ));
            }
        }
    }
    if !limits.valid() {
        return Err("limits outside depth1..3/nodes0..512".into());
    }
    Ok((limits, hold))
}
fn parse_position(args: &[&str]) -> Result<Game, String> {
    let (root, rest) =
        match args {
            ["startpos", rest @ ..] => (PositionRoot::standard(), rest),
            ["fen", a, b, c, d, e, f, rest @ ..] => (
                PositionRoot::from_fen(&[*a, *b, *c, *d, *e, *f].join(" "))
                    .map_err(|e| e.to_string())?,
                rest,
            ),
            _ => return Err(
                "position requires startpos or six-field native FEN (a-i, ranks0-9, k/a/b/n/r/c/p)"
                    .into(),
            ),
        };
    let mut game = Game::from_root(root, ClaimPolicy::AutoValidClaims)
        .map_err(|e| format!("native root: {e:?}"))?;
    let moves = match rest {
        [] => &[][..],
        ["moves", moves @ ..] => moves,
        _ => return Err("expected moves suffix".into()),
    };
    for (index, text) in moves.iter().enumerate() {
        let action: Action = text
            .parse()
            .map_err(|e| format!("move {}: {e}", index + 1))?;
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play { action },
        })
        .map_err(|e| format!("move {} native rejection: {e:?}", index + 1))?;
    }
    Ok(game)
}
fn report(result: &SearchResult, elapsed: Duration, out: &mut impl Write) -> io::Result<()> {
    let mut line = format!(
        "info depth {} nodes {} time {}",
        result.completed_depth,
        result.nodes,
        elapsed.as_millis()
    );
    let pv: Vec<_> = result
        .principal_variation
        .iter()
        .take_while(|c| matches!(c, Choice::Move(_)))
        .map(|c| match c {
            Choice::Move(a) => a.to_string(),
            _ => unreachable!(),
        })
        .collect();
    if !pv.is_empty() {
        line.push_str(" pv ");
        line.push_str(&pv.join(" "));
    }
    writeln!(out, "{line}")?;
    writeln!(
        out,
        "info string mshogi native_score {:?} perspective {:?} status {:?} distance_units command_edges units soldier_hundredths mode sound_fast_only",
        result.score, result.perspective, result.status
    )?;
    out.flush()
}
pub fn run() -> ExitCode {
    let worker = match Worker::new() {
        Ok(w) => w,
        Err(e) => {
            eprintln!("xiangqi-uci: worker: {e}");
            return ExitCode::FAILURE;
        }
    };
    let mut adapter = Adapter {
        game: Game::standard(ClaimPolicy::AutoValidClaims).expect("standard native root"),
        version: 0,
        next_id: 0,
        limits: search::DEFAULT_LIMITS,
        running: None,
        worker,
        ended: false,
    };
    let (send, recv) = mpsc::sync_channel(32);
    if let Err(e) = thread::Builder::new()
        .name("mshogi-xiangqi-uci-input".into())
        .spawn(move || {
            let stdin = io::stdin();
            let mut input = stdin.lock();
            loop {
                let event = crate::uci::bounded_uci_line(&mut input);
                let ended = matches!(event, Ok(None) | Err(_));
                if send.send(event).is_err() || ended {
                    break;
                }
            }
        })
    {
        eprintln!("xiangqi-uci: input reader: {e}");
        adapter.worker.shutdown();
        return ExitCode::FAILURE;
    }
    let mut out = io::stdout().lock();
    let result = (|| -> io::Result<()> {
        loop {
            match recv.recv_timeout(TICK) {
                Ok(Ok(Some(Ok(bytes)))) => match std::str::from_utf8(&bytes) {
                    Ok(line)
                        if line.is_ascii()
                            && !line
                                .bytes()
                                .any(|b| b.is_ascii_control() && !b.is_ascii_whitespace()) =>
                    {
                        if adapter.command(line, &mut out)? {
                            break;
                        }
                    }
                    _ => eprintln!("xiangqi-uci: input must be ASCII text without control bytes"),
                },
                Ok(Ok(Some(Err(())))) => {
                    eprintln!("xiangqi-uci: line exceeds {} bytes", crate::MAX_LINE_BYTES)
                }
                Ok(Err(e)) => return Err(e),
                Ok(Ok(None)) | Err(RecvTimeoutError::Disconnected) => break,
                Err(RecvTimeoutError::Timeout) => {}
            }
            adapter.update(&mut out)?;
        }
        Ok(())
    })();
    if let Some(r) = adapter.running.take() {
        r.stop.store(true, Ordering::Release);
    }
    adapter.worker.shutdown();
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            let message = crate::record_io::diagnostic(&e.to_string());
            let _ = writeln!(out, "info string mshogi execution_failure {message}");
            let _ = out.flush();
            eprintln!("xiangqi-uci: {message}");
            ExitCode::FAILURE
        }
    }
}
