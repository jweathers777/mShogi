//! Bounded Pikafish-compatible UCI0-9 over authoritative native Xiangqi.
//! Ordinary search is sound_fast_only; UCI cannot request native claims.
//! Clocked requests (`wtime btime winc binc`, Red is white) keep the search
//! table while each position extends the previous one.
use mshogi_core::xiangqi::*;
use mshogi_search::alphabeta::timing::TimeBudget;
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
    time: Option<TimeBudget>,
    game_generation: u64,
    table_mb: usize,
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
                let mut session = mshogi_engine::XiangqiSearchSession::default();
                let mut generation = 0;
                let mut table_mb = search::DEFAULT_TABLE_MB;
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
                    if generation != job.context.game_generation || table_mb != job.context.table_mb
                    {
                        table_mb = job.context.table_mb;
                        session = mshogi_engine::XiangqiSearchSession::with_table(table_mb);
                        generation = job.context.game_generation;
                    }
                    let result = session
                        .search(&job.game, job.context.limits, job.context.time, || {
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
    game_generation: u64,
    table_mb: usize,
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
                    "option name Depth type spin default {} min 1 max {}",
                    search::DEFAULT_LIMITS.depth,
                    search::MAX_DEPTH
                )?;
                writeln!(
                    out,
                    "option name Nodes type spin default {} min 0 max {}",
                    search::DEFAULT_LIMITS.nodes,
                    search::MAX_NODES
                )?;
                writeln!(
                    out,
                    "option name Hash type spin default {} min 1 max {}",
                    search::DEFAULT_TABLE_MB,
                    search::MAX_TABLE_MB
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
                self.game =
                    Game::standard(ClaimPolicy::AutoValidClaims).expect("standard native root");
                self.game_generation += 1;
            }),
            "position" => self
                .retire()
                .and_then(|()| parse_position(args, &mut self.game)),
            "setoption" => self.set_option(args),
            "go" => self.retire().and_then(|()| {
                let (limits, hold, time) =
                    parse_go(args, self.limits, self.game.position().side_to_move())?;
                self.next_id = self
                    .next_id
                    .checked_add(1)
                    .ok_or("request identity exhausted")?;
                let context = Context {
                    id: self.next_id,
                    version: self.version,
                    game: self.game.context(),
                    limits,
                    time,
                    game_generation: self.game_generation,
                    table_mb: self.table_mb,
                };
                let stop = Arc::new(AtomicBool::new(false));
                self.worker.submit(Job {
                    context: context.clone(),
                    game: self.game.fork(),
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
                "unsupported command/parameters; Xiangqi UCI0-9 (no pondering or claims)".into(),
            ),
        };
        if let Err(e) = result {
            eprintln!("xiangqi-uci: {}", crate::record_io::diagnostic(&e));
        }
        Ok(false)
    }
    fn set_option(&mut self, args: &[&str]) -> Result<(), String> {
        let ["name", name, "value", value] = args else {
            return Err(
                "expected setoption name Depth|Nodes|Hash|RulesProfile value <value>".into(),
            );
        };
        let mut limits = self.limits;
        match *name {
            "Depth" => {
                limits.depth = number(value)?
                    .try_into()
                    .map_err(|_| "depth exceeds supported bounds")?
            }
            "Nodes" => limits.nodes = number(value)?,
            "Hash" => {
                let mb = number(value)?;
                if !(1..=search::MAX_TABLE_MB as u64).contains(&mb) {
                    return Err(format!("Hash outside 1..{}", search::MAX_TABLE_MB));
                }
                self.retire()?;
                self.table_mb = mb as usize;
                return Ok(());
            }
            "RulesProfile" if *value == PROFILE_ID => {}
            _ => return Err("unsupported option/value; rules profile is immutable".into()),
        }
        if !limits.valid() {
            return Err(format!(
                "limits outside depth 1..{}/nodes 0..{}",
                search::MAX_DEPTH,
                search::MAX_NODES
            ));
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
                let mut candidate = self.game.fork();
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
/// Parse `go`: fixed work (`depth`, `nodes`), `movetime`, or Fischer clocks
/// (`wtime`, `btime`, `winc`, `binc`; Red is white) allocated with the shared
/// clock policy. Time controls search to the maximum depth and node bounds.
fn parse_go(
    args: &[&str],
    mut limits: Limits,
    side: Side,
) -> Result<(Limits, bool, Option<TimeBudget>), String> {
    let mut hold = false;
    let mut movetime = None;
    let mut clock = [None, None];
    let mut increment = [0, 0];
    let mut seen = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let key = args[i];
        i += 1;
        if seen.contains(&key) {
            return Err("duplicate go control".into());
        }
        seen.push(key);
        if key == "infinite" {
            hold = true;
            continue;
        }
        if !matches!(
            key,
            "depth" | "nodes" | "movetime" | "wtime" | "btime" | "winc" | "binc"
        ) {
            return Err(format!(
                "unsupported go control {key}; no pondering or searchmoves"
            ));
        }
        let value = number(args.get(i).ok_or("missing go value")?)?;
        i += 1;
        match key {
            "depth" => limits.depth = value.try_into().map_err(|_| "unsupported depth")?,
            "nodes" => limits.nodes = value,
            "movetime" => {
                if !(1..=60_000).contains(&value) {
                    return Err("movetime outside 1..60000ms".into());
                }
                movetime = Some(value);
            }
            "wtime" => clock[0] = Some(value),
            "btime" => clock[1] = Some(value),
            "winc" => increment[0] = value,
            "binc" => increment[1] = value,
            _ => unreachable!(),
        }
    }
    if !limits.valid() {
        return Err(format!(
            "limits outside depth 1..{}/nodes 0..{}",
            search::MAX_DEPTH,
            search::MAX_NODES
        ));
    }
    let own = usize::from(side == Side::Black);
    let clocked = seen
        .iter()
        .any(|k| matches!(*k, "wtime" | "btime" | "winc" | "binc"));
    if clocked && clock[own].is_none() {
        return Err("clock controls require the moving side's remaining time".into());
    }
    let fixed = seen.iter().any(|k| matches!(*k, "depth" | "nodes"));
    if (movetime.is_some() || clocked) && (fixed || hold) {
        return Err("time controls cannot combine with fixed work or infinite".into());
    }
    if movetime.is_some() && clocked {
        return Err("movetime cannot combine with clock controls".into());
    }
    let mut time = movetime.map(TimeBudget::from_hard_ms);
    if let Some(remaining) = clock[own] {
        time = Some(
            TimeBudget::from_clock_ms(remaining, increment[own], CLOCK_MOVES)
                .with_response_margin(),
        );
    }
    if time.is_some() {
        limits = Limits {
            depth: search::MAX_DEPTH,
            nodes: search::MAX_NODES,
        };
    }
    Ok((limits, hold, time))
}
/// Expected remaining moves for clock allocation, as for Chess and Shogi.
const CLOCK_MOVES: u64 = 30;
/// Parse `position` into `current`. When the root and policy match and the
/// current board actions are a prefix of the new list, only the new suffix is
/// admitted; otherwise the game is rebuilt from the root. Native admission
/// checks every action either way. On error `current` is unchanged.
fn parse_position(args: &[&str], current: &mut Game) -> Result<(), String> {
    let (root, rest) = match args {
        ["startpos", rest @ ..] => (PositionRoot::standard(), rest),
        ["fen", a, b, c, d, e, f, rest @ ..] => (
            PositionRoot::from_fen(&[*a, *b, *c, *d, *e, *f].join(" "))
                .map_err(|e| e.to_string())?,
            rest,
        ),
        _ => return Err(
            "position requires startpos or six-field native FEN (a-i, ranks 0-9, k/a/b/n/r/c/p)"
                .into(),
        ),
    };
    let moves = match rest {
        [] => &[][..],
        ["moves", moves @ ..] => moves,
        _ => return Err("expected moves suffix".into()),
    };
    let actions = moves
        .iter()
        .enumerate()
        .map(|(index, text)| {
            text.parse::<Action>()
                .map_err(|e| format!("move {}: {e}", index + 1))
        })
        .collect::<Result<Vec<_>, _>>()?;
    // `current` was built by this function, so its commands are board actions.
    let known: Vec<Action> = current
        .events()
        .iter()
        .filter_map(|e| match e.event {
            EventKind::BoardAction { action, .. } => Some(action),
            _ => None,
        })
        .collect();
    let (mut game, start) = match known {
        known
            if current.root() == &root
                && current.claim_policy() == ClaimPolicy::AutoValidClaims
                && known.len() <= actions.len()
                && known[..] == actions[..known.len()] =>
        {
            (current.fork(), known.len())
        }
        _ => (
            Game::from_root(root, ClaimPolicy::AutoValidClaims)
                .map_err(|e| format!("native root: {e:?}"))?,
            0,
        ),
    };
    for (index, action) in actions.iter().enumerate().skip(start) {
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play { action: *action },
        })
        .map_err(|e| format!("move {} native rejection: {e:?}", index + 1))?;
    }
    *current = game;
    Ok(())
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
        game_generation: 0,
        table_mb: search::DEFAULT_TABLE_MB,
    };
    let (send, recv) = mpsc::sync_channel(32);
    if let Err(e) = thread::Builder::new()
        .name("mshogi-xiangqi-uci-input".into())
        .spawn(move || {
            let stdin = io::stdin();
            let mut input = stdin.lock();
            loop {
                let event = crate::uci::uci_line(&mut input);
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
                Ok(Ok(Some(bytes))) => match std::str::from_utf8(&bytes) {
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
