//! Shared fresh-process runner. Native adapters own notation and rules admission;
//! this module owns request generations, bounded traffic and process retirement.
use crate::{process::Engine, *};
use serde::{Deserialize, Serialize};

use std::time::{Duration, Instant};

/// Bounded explicit failure evidence.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Failure {
    /// Stable unscored category.
    pub kind: FailureKind,
    /// Configured engine index.
    pub engine: usize,
    /// Request generation, zero before a child starts.
    pub generation: u64,
    /// Handshake/readiness/search/cleanup stage.
    pub phase: String,
    /// Exact bounded explanation.
    pub detail: String,
}
/// Serialized actual protocol observation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Traffic {
    /// Engine configuration index.
    pub engine: usize,
    /// Fresh process/request identity.
    pub generation: u64,
    /// Native event-prefix version at dispatch.
    pub version: usize,
    /// Controller-relative monotonic observation time.
    pub at_ns: u64,
    /// command or response.
    pub direction: String,
    /// Phase owning this observation.
    pub phase: String,
    /// Exact ASCII protocol text (hostile bytes escaped).
    pub text: String,
    /// Command write/flush completed; absent on response.
    pub written: Option<bool>,
}
/// One accepted result bound to a retained native history prefix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Turn<S> {
    /// Engine configuration index.
    pub engine: usize,
    /// Unique request/process identity within the report.
    pub generation: u64,
    /// Native prefix including opening events, before result acceptance.
    pub version: usize,
    /// Full canonical-prefix digest, not a board-only identity.
    pub context_sha256: String,
    /// Captured native actor.
    pub actor: S,
    /// Actual position command with complete known history.
    pub position: String,
    /// Actual finite work request.
    pub go: String,
    /// Complete accepted bestmove line, including a validated ponder suffix if supplied.
    pub reply: String,
}
/// One observed child, bound to its unique request generation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Process {
    /// Unique request/process generation within this report.
    pub generation: u64,
    /// Actual bounded native child observation.
    pub observation: EngineObservation,
}
pub(crate) fn option_value(advertised: &str, value: &str) -> bool {
    let Some((_, spec)) = advertised.split_once(" type ") else {
        return false;
    };
    let words: Vec<_> = spec.split_ascii_whitespace().collect();
    match words.first().copied() {
        Some("check") => matches!(value, "true" | "false"),
        Some("spin") => {
            let bound = |key: &str| {
                words
                    .windows(2)
                    .find(|w| w[0] == key)
                    .and_then(|w| w[1].parse::<i64>().ok())
            };
            match (value.parse::<i64>(), bound("min"), bound("max")) {
                (Ok(v), Some(min), Some(max)) => v >= min && v <= max,
                _ => false,
            }
        }
        Some("combo") => spec.split(" var ").skip(1).any(|v| v == value),
        Some("string") | Some("filename") => true,
        _ => false,
    }
}
pub(crate) fn go(work: WorkLimit) -> String {
    format!(
        "go depth {} nodes {}",
        work.depth.unwrap(),
        work.nodes.unwrap()
    )
}

#[derive(Clone, Copy)]
pub(crate) struct Settings<'a> {
    pub engines: &'a [EngineConfig; 2],
    pub watchdogs: Watchdogs,
    pub io: IoLimits,
    pub max_game_ms: u64,
    pub max_actions: usize,
    pub work: [WorkLimit; 2],
}
impl Settings<'_> {
    pub fn validate_bounds(&self, id: &str, openings: usize) -> Result<(), RunnerError> {
        if id.is_empty()
            || id.len() > 128
            || !(1..=4).contains(&openings)
            || self.max_actions == 0
            || !(1..=600_000).contains(&self.max_game_ms)
        {
            return Err(RunnerError("unsupported experiment bounds".into()));
        }
        Ok(())
    }
    pub fn validate_work(&self, name: &str, depth: u16, nodes: u64) -> Result<(), RunnerError> {
        for w in self.work {
            if w.movetime_ms.is_some()
                || w.depth.is_none_or(|d| !(1..=depth).contains(&d))
                || w.nodes.is_none_or(|n| n > nodes)
            {
                return Err(RunnerError(format!(
                    "finite supported {name} depth/nodes required; timed work unsupported"
                )));
            }
        }
        Ok(())
    }
    pub fn validate_processes(&self, name: &str) -> Result<(), RunnerError> {
        crate::config::validate_processes(self.engines, self.watchdogs, self.io)?;
        if self.io.line_bytes > 4096
            || self.io.queued_lines > 32
            || self.io.transcript_bytes > 1_048_576
            || self.io.stderr_bytes > 4096
        {
            return Err(RunnerError(format!(
                "{name} experiment I/O caps exceed supported report bounds"
            )));
        }
        Ok(())
    }
}
/// Why this experiment stopped; distinct from the native conclusion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum End {
    /// Native authoritative conclusion.
    NativeConclusion,
    /// Native adjudication is pending.
    Unresolved,
    /// Accepted-action cap.
    ActionLimit,
    /// Overall game watchdog.
    TimeLimit,
    /// Unscored process, protocol or admission failure.
    Failure,
}
pub(crate) struct Rejection {
    pub kind: FailureKind,
    pub phase: &'static str,
    pub detail: String,
}
pub(crate) trait Native {
    type Game: Clone;
    type Side: Copy + Eq + Serialize;
    type Conclusion: Copy + PartialEq + Serialize;
    type Pending: PartialEq + Serialize;
    fn conclusion(game: &Self::Game) -> Option<Self::Conclusion>;
    fn pending(game: &Self::Game) -> Option<Self::Pending>;
    fn score(conclusion: Self::Conclusion) -> [u8; 2];
    const DIALECT: crate::transport::Dialect;
    fn search_failure(_line: &str) -> Option<(FailureKind, &str)> {
        None
    }
    fn actor(game: &Self::Game) -> Self::Side;
    fn side(side: Self::Side) -> usize;
    fn events(game: &Self::Game) -> usize;
    fn end(game: &Self::Game) -> Option<End>;
    fn canonical(game: &Self::Game) -> Result<String, RunnerError>;
    fn position(game: &Self::Game) -> String;
    fn admit_turn(game: &mut Self::Game, reply: &str) -> Result<Option<String>, Rejection>;
}
pub(crate) fn digest<G: Native>(game: &G::Game) -> Result<String, RunnerError> {
    Ok(crate::common::sha256(G::canonical(game)?))
}
pub(crate) struct Completed<G: Native> {
    pub game: G::Game,
    pub stop: End,
    pub failure: Option<Failure>,
    pub turns: Vec<Turn<G::Side>>,
    pub traffic: Vec<Traffic>,
    pub traffic_complete: bool,
    pub processes: Vec<Process>,
    pub elapsed_ns: u64,
}
pub(crate) fn run_game<G: Native>(
    config: Settings<'_>,
    mut game: G::Game,
    assignment: [usize; 2],
    generation: &mut u64,
    budget: crate::common::ReportBudget,
) -> Result<Completed<G>, RunnerError> {
    let mut host = crate::transport::Transport::new(
        config.watchdogs,
        config.io,
        Some(config.max_game_ms),
        Vec::new(),
    );
    host.report_budget = budget;
    host.generation = *generation;
    let mut processes = Vec::new();
    let mut turns = Vec::new();
    let mut failure = None;
    let stop = loop {
        if let Some(end) = G::end(&game) {
            break end;
        }
        if turns.len() >= config.max_actions {
            break End::ActionLimit;
        }
        if host.start.elapsed() >= Duration::from_millis(config.max_game_ms) {
            break End::TimeLimit;
        }
        host.generation = host
            .generation
            .checked_add(1)
            .ok_or_else(|| RunnerError("execution generation exhausted".into()))?;
        host.version = G::events(&game);
        let actor = G::actor(&game);
        let engine = assignment[G::side(actor)];
        let context = Turn {
            engine,
            generation: host.generation,
            version: host.version,
            context_sha256: digest::<G>(&game)?,
            actor,
            position: G::position(&game),
            go: go(config.work[engine]),
            reply: String::new(),
        };
        let mut candidate = game.clone();
        match turn::<G>(
            &mut host,
            &config,
            &mut processes,
            &mut candidate,
            engine,
            context,
        ) {
            Ok(turn) => {
                host.report_budget.include(&turn)?;
                host.report_budget
                    .include(processes.last().expect("completed turn has a process"))?;
                game = candidate;
                turns.push(turn);
            }
            Err(e) if e.phase == "experiment_limit" => break End::TimeLimit,
            Err(e) => {
                failure = Some(e);
                break End::Failure;
            }
        }
    };
    *generation = host.generation;
    Ok(Completed {
        game,
        stop,
        failure,
        turns,
        traffic: host.journal,
        traffic_complete: host.complete,
        processes,
        elapsed_ns: crate::common::elapsed_ns(host.start),
    })
}
fn turn<G: Native>(
    host: &mut crate::transport::Transport<Vec<Traffic>>,
    config: &Settings<'_>,
    processes: &mut Vec<Process>,
    game: &mut G::Game,
    engine_index: usize,
    turn: Turn<G::Side>,
) -> Result<Turn<G::Side>, Failure> {
    let mut engine = Engine::spawn(engine_index, &config.engines[engine_index], config.io)
        .map_err(|e| host.failure(engine_index, FailureKind::ProcessStart, "spawn", e))?;
    let mut result = (|| {
        host.prepare(
            &mut engine,
            G::DIALECT,
            &config.engines[engine_index].options,
            None,
            None,
        )?;
        host.send(&mut engine, "position", &turn.position)?;
        host.send(&mut engine, "search", &turn.go)?;
        let deadline = Instant::now() + Duration::from_millis(config.watchdogs.response_ms);
        let response = loop {
            let line = host
                .line(&mut engine, "search", deadline, None)?
                .ok_or_else(|| {
                    host.failure(
                        engine_index,
                        FailureKind::Watchdog,
                        "search",
                        "required reply deadline exceeded",
                    )
                })?;
            if let Some((kind, detail)) = G::search_failure(line.trim()) {
                return Err(Failure {
                    kind,
                    engine: engine_index,
                    generation: host.generation,
                    phase: "search".into(),
                    detail: detail.chars().take(1024).collect(),
                });
            }
            if line.trim_start().starts_with("bestmove") {
                break line;
            }
            if !line.trim_start().starts_with("info ") {
                return Err(host.failure(
                    engine_index,
                    FailureKind::MalformedReply,
                    "search",
                    "unexpected search response",
                ));
            }
        };
        let notice = G::admit_turn(game, &response).map_err(|e| Failure {
            kind: e.kind,
            engine: engine_index,
            generation: host.generation,
            phase: e.phase.into(),
            detail: e.detail.chars().take(1024).collect(),
        })?;
        if let Some(notice) = notice {
            host.send(&mut engine, "gameover", &notice)?;
        }
        Ok(Turn {
            reply: response,
            ..turn
        })
    })();
    // Retire before cleanup. Even a well-formed late reply cannot commit.
    let (observation, cleanup_failure) = host.retire(&mut engine);
    if let Err(f) = &mut result {
        engine.refine_pipe_failure(&mut f.kind, &mut f.detail);
    }
    processes.push(Process {
        generation: host.generation,
        observation,
    });
    match result {
        Err(e) => Err(e),
        Ok(_) if cleanup_failure.is_some() => Err(cleanup_failure.unwrap()),
        result => result,
    }
}

pub(crate) struct Replay<'a, G: Native> {
    pub turns: &'a [Turn<G::Side>],
    pub traffic: &'a [Traffic],
    pub processes: &'a [Process],
    pub elapsed_ns: u64,
}
pub(crate) fn replay<G: Native>(
    config: Settings<'_>,
    entry: Replay<'_, G>,
    game: &mut G::Game,
    assignment: [usize; 2],
    generation: &mut u64,
) -> Result<(), RunnerError> {
    if entry.turns.len() > config.max_actions || entry.processes.len() > config.max_actions {
        return Err(RunnerError("report collection bounds exceeded".into()));
    }
    let mut previous = 0;
    validate_traffic(config.io, entry.traffic)?;
    for event in entry.traffic {
        if event.at_ns < previous
            || event.at_ns > entry.elapsed_ns
            || event.engine > 1
            || event.generation == 0
        {
            return Err(RunnerError("traffic ordering/identity invalid".into()));
        }
        previous = event.at_ns;
    }
    for turn in entry.turns {
        let actor = G::actor(game);
        let engine = assignment[G::side(actor)];
        if turn.engine != engine
            || turn.actor != actor
            || turn.version != G::events(game)
            || turn.context_sha256 != digest::<G>(game)?
            || turn.position != G::position(game)
            || turn.go != go(config.work[engine])
            || turn.generation <= *generation
        {
            return Err(RunnerError(
                "accepted turn context/limit/generation mismatch".into(),
            ));
        }
        let traffic: Vec<_> = entry
            .traffic
            .iter()
            .filter(|e| e.generation == turn.generation)
            .collect();
        if traffic
            .iter()
            .any(|e| e.engine != turn.engine || e.version != turn.version)
            || entry
                .processes
                .iter()
                .filter(|p| {
                    p.generation == turn.generation
                        && p.observation.engine == engine
                        && p.observation.exit.as_ref().is_some_and(|e| e.success)
                        && p.observation.cleanup_failures.is_empty()
                        && p.observation.retirement_failure.is_none()
                })
                .count()
                != 1
        {
            return Err(RunnerError(
                "accepted result lacks matching process/command/response evidence".into(),
            ));
        }
        let notice = G::admit_turn(game, &turn.reply).map_err(|e| RunnerError(e.detail))?;
        replay_traffic::<G>(&traffic, turn, &config.engines[engine], notice)?;
        *generation = turn.generation;
    }
    if entry.processes.iter().any(|p| p.observation.engine > 1) {
        return Err(RunnerError("invalid process/traffic bounds".into()));
    }
    Ok(())
}
pub(crate) fn validate_traffic(io: IoLimits, traffic: &[Traffic]) -> Result<(), RunnerError> {
    let mut remaining = io.transcript_bytes;
    for event in traffic {
        let command = event.written.is_some();
        if (command && !io.admits_command(&event.phase, &event.text))
            || (!command && event.text.len() > io.line_bytes * 4)
            || !io.charge_entry(
                &mut remaining,
                command,
                &event.phase,
                crate::common::encoded_len(event)? + 1,
            )
        {
            return Err(RunnerError(
                "traffic exceeds configured exchange bounds".into(),
            ));
        }
    }
    Ok(())
}

/// Validate the actual accepted-generation protocol, not merely the existence
/// of a matching bestmove somewhere in a contradictory transcript.
fn replay_traffic<G: Native>(
    events: &[&Traffic],
    turn: &Turn<G::Side>,
    config: &EngineConfig,
    notice: Option<String>,
) -> Result<(), RunnerError> {
    let invalid = || RunnerError("accepted generation protocol phase/order/reply mismatch".into());
    let (hello, ok, newgame) = G::DIALECT.tokens();
    let ready_again = G::DIALECT != crate::transport::Dialect::Usi;
    let mut expected = vec![("handshake", hello.to_owned())];
    expected.extend(config.options.iter().map(|o| {
        (
            "options",
            format!("setoption name {} value {}", o.name, o.value),
        )
    }));
    expected.extend([
        ("readiness", "isready".to_owned()),
        ("newgame", newgame.to_owned()),
    ]);
    if ready_again {
        expected.push(("newgame_ready", "isready".into()));
    }
    expected.extend([
        ("position", turn.position.clone()),
        ("search", turn.go.clone()),
    ]);
    if let Some(notice) = notice {
        expected.push(("gameover", notice));
    }
    expected.extend([
        ("cleanup", "stop".to_owned()),
        ("cleanup", "quit".to_owned()),
    ]);
    let mut written = 0;
    let mut awaiting = "";
    let mut handshake = false;
    let mut ready = false;
    let mut new_ready = !ready_again;
    let mut replied = false;
    let mut advertisements = Vec::new();
    let mut count = 0usize;
    for event in events {
        match event.direction.as_str() {
            "command" => {
                let Some((phase, text)) = expected.get(written) else {
                    return Err(invalid());
                };
                if event.phase != *phase || event.text != *text || event.written != Some(true) {
                    return Err(invalid());
                }
                let admitted = match *phase {
                    "handshake" => awaiting.is_empty() && !handshake,
                    "options" | "readiness" => handshake && awaiting.is_empty(),
                    "newgame" => ready && awaiting.is_empty(),
                    "newgame_ready" => ready && awaiting.is_empty(),
                    "position" => new_ready && awaiting.is_empty(),
                    "search" => new_ready && awaiting.is_empty() && !replied,
                    "gameover" => replied && awaiting.is_empty(),
                    "cleanup" => replied && (awaiting.is_empty() || awaiting == "retired"),
                    _ => false,
                };
                if !admitted {
                    return Err(invalid());
                }
                awaiting = match *phase {
                    "handshake" | "readiness" | "newgame_ready" | "search" => phase,
                    "cleanup" => "retired",
                    _ => "",
                };
                written += 1;
                count = 0;
            }
            "response" => {
                if event.written.is_some() || event.phase != awaiting || awaiting.is_empty() {
                    return Err(invalid());
                }
                let raw = crate::process::protocol_text(&event.text).ok_or_else(invalid)?;
                let text = raw.trim();
                if !text.is_empty()
                    && matches!(awaiting, "handshake" | "readiness" | "newgame_ready")
                    && text != ok
                    && text != "readyok"
                {
                    count += 1;
                    if count > crate::transport::HANDSHAKE_LINES {
                        return Err(invalid());
                    }
                }
                match awaiting {
                    "handshake" if text == ok => {
                        for option in &config.options {
                            if !crate::transport::supports_option(
                                advertisements.iter().map(String::as_str),
                                G::DIALECT,
                                option,
                            ) {
                                return Err(invalid());
                            }
                        }
                        handshake = true;
                        awaiting = "";
                    }
                    "readiness" if text == "readyok" => {
                        ready = true;
                        awaiting = "";
                    }
                    "newgame_ready" if text == "readyok" => {
                        new_ready = true;
                        awaiting = "";
                    }
                    "search" if raw == turn.reply && text.starts_with("bestmove") => {
                        replied = true;
                        awaiting = "";
                    }
                    "handshake" if crate::transport::handshake_header(text) => {
                        if text.starts_with("option ") {
                            advertisements.push(text.to_owned());
                        }
                    }
                    "handshake" if text.is_empty() || text.starts_with("info ") => {}
                    "readiness" | "newgame_ready"
                        if text.is_empty() || text.starts_with("info ") => {}
                    "search"
                        if text.is_empty()
                            || (text.starts_with("info ") && G::search_failure(text).is_none()) => {
                    }
                    "retired" if !text.starts_with("bestmove") => {}
                    _ => return Err(invalid()),
                }
            }
            _ => return Err(invalid()),
        }
    }
    if written != expected.len() || !handshake || !ready || !new_ready || !replied {
        return Err(invalid());
    }
    Ok(())
}
