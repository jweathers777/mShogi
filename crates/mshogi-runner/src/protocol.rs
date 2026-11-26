//! Shared fresh-process runner. Native adapters own notation and rules admission;
//! this module owns request generations, bounded traffic and process retirement.
use crate::{
    process::{Engine, Output, escaped},
    *,
};
use serde::{Deserialize, Serialize};

use std::{
    sync::mpsc::RecvTimeoutError,
    time::{Duration, Instant},
};

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
    pub max_actions: u32,
    pub work: [WorkLimit; 2],
}
impl Settings<'_> {
    pub fn validate_bounds(&self, id: &str, openings: usize) -> Result<(), RunnerError> {
        if id.is_empty()
            || id.len() > 128
            || !(1..=4).contains(&openings)
            || !(1..=32).contains(&self.max_actions)
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
    type Game;
    type Side: Copy + Eq;
    type Conclusion: Copy + PartialEq;
    type Pending: PartialEq;
    fn conclusion(game: &Self::Game) -> Option<Self::Conclusion>;
    fn pending(game: &Self::Game) -> Option<Self::Pending>;
    fn score(conclusion: Self::Conclusion) -> [u8; 2];
    const HELLO: &'static str;
    const OK: &'static str;
    const NEWGAME: &'static str;
    const READY_AFTER_NEWGAME: bool;
    fn banner(_line: &str) -> bool {
        false
    }
    fn search_failure(_line: &str) -> Option<(FailureKind, &str)> {
        None
    }
    fn replay_traffic(
        _events: &[&Traffic],
        _turn: &Turn<Self::Side>,
        _config: &EngineConfig,
    ) -> Result<(), RunnerError> {
        Ok(())
    }
    fn actor(game: &Self::Game) -> Self::Side;
    fn side(side: Self::Side) -> usize;
    fn events(game: &Self::Game) -> usize;
    fn end(game: &Self::Game) -> Option<End>;
    fn canonical(game: &Self::Game) -> Result<String, RunnerError>;
    fn position(game: &Self::Game) -> String;
    fn apply_reply(game: &mut Self::Game, reply: &str) -> Result<(), RunnerError>;
    fn admit_turn(game: &Self::Game, reply: &str) -> Result<Option<String>, Rejection>;
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
) -> Result<Completed<G>, RunnerError> {
    let mut host = Controller::<G> {
        config,
        native: std::marker::PhantomData,
        start: Instant::now(),
        generation: *generation,
        version: G::events(&game),
        traffic: vec![],
        bytes: 0,
        complete: true,
        processes: vec![],
    };
    let mut turns = Vec::new();
    let mut failure = None;
    let stop = loop {
        if let Some(end) = G::end(&game) {
            break end;
        }
        if turns.len() >= config.max_actions as usize {
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
        match host.turn(&game, engine, context) {
            Ok(turn) => {
                G::apply_reply(&mut game, &turn.reply)?;
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
        traffic: host.traffic,
        traffic_complete: host.complete,
        processes: host.processes,
        elapsed_ns: crate::common::elapsed_ns(host.start),
    })
}
struct Controller<'a, G: Native> {
    config: Settings<'a>,
    native: std::marker::PhantomData<G>,
    start: Instant,
    generation: u64,
    version: usize,
    traffic: Vec<Traffic>,
    bytes: usize,
    complete: bool,
    processes: Vec<Process>,
}
impl<G: Native> Controller<'_, G> {
    fn failure(
        &self,
        engine: usize,
        kind: FailureKind,
        phase: &str,
        detail: impl Into<String>,
    ) -> Failure {
        Failure {
            kind,
            engine,
            generation: self.generation,
            phase: phase.into(),
            detail: detail.into().chars().take(1024).collect(),
        }
    }
    fn log(
        &mut self,
        engine: usize,
        direction: &str,
        phase: &str,
        text: String,
        written: Option<bool>,
    ) -> Result<(), Failure> {
        let event = Traffic {
            engine,
            generation: self.generation,
            version: self.version,
            at_ns: crate::common::elapsed_ns(self.start),
            direction: direction.into(),
            phase: phase.into(),
            text,
            written,
        };
        let size = serde_json::to_vec(&event).unwrap().len();
        if self.bytes + size > self.config.io.transcript_bytes {
            self.complete = false;
            return Err(self.failure(
                engine,
                FailureKind::ResourceLimit,
                phase,
                "protocol retention limit reached; no further actions accepted",
            ));
        }
        self.bytes += size;
        self.traffic.push(event);
        Ok(())
    }
    fn write(
        &mut self,
        engine: &mut Engine,
        phase: &str,
        text: &str,
        budget: Duration,
    ) -> Result<(), Failure> {
        if text.len() + 1 > self.config.io.command_bytes {
            return Err(self.failure(
                engine.index,
                FailureKind::ResourceLimit,
                phase,
                "command exceeds configured limit",
            ));
        }
        let result = engine.write(format!("{text}\n").into_bytes(), budget);
        self.log(
            engine.index,
            "command",
            phase,
            text.into(),
            Some(result.is_ok()),
        )?;
        result.map_err(|e| self.failure(engine.index, FailureKind::UnexpectedExit, phase, e))
    }
    fn send(&mut self, engine: &mut Engine, phase: &str, text: &str) -> Result<(), Failure> {
        let deadline = self.start + Duration::from_millis(self.config.max_game_ms);
        let left = deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Err(self.failure(
                engine.index,
                FailureKind::Watchdog,
                "experiment_limit",
                "overall experiment time limit",
            ));
        }
        let result = self.write(
            engine,
            phase,
            text,
            left.min(Duration::from_millis(self.config.watchdogs.write_ms)),
        );
        if Instant::now() >= deadline {
            return Err(self.failure(
                engine.index,
                FailureKind::Watchdog,
                "experiment_limit",
                "command after overall experiment time limit",
            ));
        }
        result
    }
    fn line(
        &mut self,
        engine: &mut Engine,
        phase: &str,
        deadline: Instant,
    ) -> Result<String, Failure> {
        let total_deadline = self.start + Duration::from_millis(self.config.max_game_ms);
        let deadline = deadline.min(total_deadline);
        loop {
            let now = Instant::now();
            if now >= total_deadline {
                return Err(self.failure(
                    engine.index,
                    FailureKind::Watchdog,
                    "experiment_limit",
                    "overall experiment time limit; request retired unaccepted",
                ));
            }
            if now >= deadline {
                return Err(self.failure(
                    engine.index,
                    FailureKind::Watchdog,
                    phase,
                    "required reply deadline exceeded",
                ));
            }
            match engine.receive((deadline - now).min(Duration::from_millis(5))) {
                Ok(Output::Line(bytes)) => {
                    self.log(engine.index, "response", phase, escaped(&bytes), None)?;
                    if Instant::now() >= total_deadline {
                        return Err(self.failure(
                            engine.index,
                            FailureKind::Watchdog,
                            "experiment_limit",
                            "reply after experiment time limit; request retired",
                        ));
                    }
                    if Instant::now() >= deadline {
                        return Err(self.failure(
                            engine.index,
                            FailureKind::LateReply,
                            phase,
                            "reply observed after retired deadline",
                        ));
                    }
                    if bytes.is_empty() {
                        continue;
                    }
                    if !bytes.is_ascii()
                        || bytes
                            .iter()
                            .any(|b| b.is_ascii_control() && !b.is_ascii_whitespace())
                    {
                        return Err(self.failure(
                            engine.index,
                            FailureKind::MalformedReply,
                            phase,
                            "non-ASCII/control output",
                        ));
                    }
                    return Ok(String::from_utf8(bytes).unwrap());
                }
                Ok(Output::Oversized) => {
                    self.complete = false;
                    return Err(self.failure(
                        engine.index,
                        FailureKind::ResourceLimit,
                        phase,
                        "oversized protocol line",
                    ));
                }
                Ok(Output::Eof) => {
                    let status = engine.settle_exit();
                    let kind = if status.is_some_and(|s| !s.success) {
                        FailureKind::UnexpectedExit
                    } else {
                        FailureKind::MissingReply
                    };
                    return Err(self.failure(
                        engine.index,
                        kind,
                        phase,
                        "EOF before required reply",
                    ));
                }
                Ok(Output::Error(e)) => {
                    return Err(self.failure(engine.index, FailureKind::UnexpectedExit, phase, e));
                }
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(self.failure(
                        engine.index,
                        FailureKind::MissingReply,
                        phase,
                        "stdout reader closed",
                    ));
                }
                Err(RecvTimeoutError::Timeout) => {}
            }
        }
    }
    fn await_token(
        &mut self,
        engine: &mut Engine,
        phase: &str,
        token: &str,
        millis: u64,
    ) -> Result<Vec<String>, Failure> {
        let deadline = Instant::now() + Duration::from_millis(millis);
        let mut lines = Vec::new();
        loop {
            let line = self.line(engine, phase, deadline)?;
            if line == token {
                return Ok(lines);
            }
            if line.starts_with("bestmove") {
                return Err(self.failure(
                    engine.index,
                    FailureKind::LateReply,
                    phase,
                    "unsolicited bestmove outside current search",
                ));
            }
            if phase == "handshake"
                && (line.starts_with("id ") || line.starts_with("option ") || G::banner(&line))
            {
                if lines.len() >= 256 {
                    return Err(self.failure(
                        engine.index,
                        FailureKind::ResourceLimit,
                        phase,
                        "too many handshake lines",
                    ));
                }
                lines.push(line);
            } else if !line.starts_with("info ") {
                return Err(self.failure(
                    engine.index,
                    FailureKind::MalformedReply,
                    phase,
                    "unexpected reply",
                ));
            }
        }
    }
    fn turn(
        &mut self,
        game: &G::Game,
        engine_index: usize,
        turn: Turn<G::Side>,
    ) -> Result<Turn<G::Side>, Failure> {
        let mut engine = Engine::spawn(
            engine_index,
            &self.config.engines[engine_index],
            self.config.io,
        )
        .map_err(|e| self.failure(engine_index, FailureKind::ProcessStart, "spawn", e))?;
        let result = (|| {
            self.send(&mut engine, "handshake", G::HELLO)?;
            let lines = self.await_token(
                &mut engine,
                "handshake",
                G::OK,
                self.config.watchdogs.handshake_ms,
            )?;
            engine.identity = lines
                .iter()
                .filter(|l| l.starts_with("id "))
                .cloned()
                .collect();
            for option in &self.config.engines[engine_index].options {
                let prefix = format!("option name {} type ", option.name);
                let advertised =
                    lines
                        .iter()
                        .find(|l| l.starts_with(&prefix))
                        .ok_or_else(|| {
                            self.failure(
                                engine_index,
                                FailureKind::Configuration,
                                "options",
                                format!("option not advertised: {}", option.name),
                            )
                        })?;
                if !option_value(advertised, &option.value) {
                    return Err(self.failure(
                        engine_index,
                        FailureKind::Configuration,
                        "options",
                        "option value not supported by advertisement",
                    ));
                }
                self.send(
                    &mut engine,
                    "options",
                    &format!("setoption name {} value {}", option.name, option.value),
                )?;
            }
            self.send(&mut engine, "readiness", "isready")?;
            self.await_token(
                &mut engine,
                "readiness",
                "readyok",
                self.config.watchdogs.ready_ms,
            )?;
            self.send(&mut engine, "newgame", G::NEWGAME)?;
            if G::READY_AFTER_NEWGAME {
                self.send(&mut engine, "newgame_ready", "isready")?;
                self.await_token(
                    &mut engine,
                    "newgame_ready",
                    "readyok",
                    self.config.watchdogs.ready_ms,
                )?;
            }
            self.send(&mut engine, "position", &turn.position)?;
            self.send(&mut engine, "search", &turn.go)?;
            let deadline =
                Instant::now() + Duration::from_millis(self.config.watchdogs.response_ms);
            let response = loop {
                let line = self.line(&mut engine, "search", deadline)?;
                if let Some((kind, detail)) = G::search_failure(&line) {
                    return Err(Failure {
                        kind,
                        engine: engine_index,
                        generation: self.generation,
                        phase: "search".into(),
                        detail: detail.chars().take(1024).collect(),
                    });
                }
                if line.starts_with("bestmove") {
                    break line;
                }
                if !line.starts_with("info ") {
                    return Err(self.failure(
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
                generation: self.generation,
                phase: e.phase.into(),
                detail: e.detail.chars().take(1024).collect(),
            })?;
            if let Some(notice) = notice {
                self.send(&mut engine, "gameover", &notice)?;
            }
            Ok(Turn {
                reply: response,
                ..turn
            })
        })();
        // Retire before cleanup. Even a well-formed late reply cannot commit.
        let cleanup_deadline =
            Instant::now() + Duration::from_millis(self.config.watchdogs.cleanup_ms);
        let mut cleanup_failure = None;
        let mut reached_eof = false;
        for command in ["stop", "quit"] {
            let left = cleanup_deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                break;
            }
            if let Err(e) = self.write(
                &mut engine,
                "cleanup",
                command,
                left.min(Duration::from_millis(self.config.watchdogs.write_ms)),
            ) {
                cleanup_failure.get_or_insert(e);
            }
        }
        let drain_deadline =
            cleanup_deadline - Duration::from_millis(self.config.watchdogs.cleanup_ms / 4);
        loop {
            if Instant::now() >= drain_deadline {
                break;
            }
            match engine.receive(
                drain_deadline
                    .saturating_duration_since(Instant::now())
                    .min(Duration::from_millis(5)),
            ) {
                Ok(Output::Line(bytes)) => {
                    if let Err(e) =
                        self.log(engine_index, "response", "retired", escaped(&bytes), None)
                    {
                        cleanup_failure.get_or_insert(e);
                    }
                    if bytes.starts_with(b"bestmove") {
                        cleanup_failure.get_or_insert_with(|| {
                            self.failure(
                                engine_index,
                                FailureKind::LateReply,
                                "retired",
                                "reply after generation retired",
                            )
                        });
                    }
                }
                Ok(Output::Oversized) => {
                    self.complete = false;
                    cleanup_failure.get_or_insert_with(|| {
                        self.failure(
                            engine_index,
                            FailureKind::ResourceLimit,
                            "retired",
                            "oversized retired output",
                        )
                    });
                }
                Ok(Output::Error(e)) => {
                    cleanup_failure.get_or_insert_with(|| {
                        self.failure(engine_index, FailureKind::UnexpectedExit, "retired", e)
                    });
                }
                Ok(Output::Eof) | Err(RecvTimeoutError::Disconnected) => {
                    reached_eof = true;
                    break;
                }
                Err(RecvTimeoutError::Timeout) => {}
            }
        }
        if !reached_eof {
            self.complete = false;
            cleanup_failure.get_or_insert_with(||self.failure(engine_index,FailureKind::ResourceLimit,"retired","retirement capture did not establish EOF within deadline; candidate not accepted"));
        }
        let left = cleanup_deadline
            .saturating_duration_since(Instant::now())
            .as_millis()
            .max(1) as u64;
        let observation = engine.cleanup(left);
        if !observation.cleanup_failures.is_empty()
            || observation.exit.as_ref().is_none_or(|e| !e.success)
        {
            cleanup_failure.get_or_insert_with(|| {
                self.failure(
                    engine_index,
                    FailureKind::UnexpectedExit,
                    "cleanup",
                    "child did not exit successfully within bounded cleanup",
                )
            });
        }
        self.processes.push(Process {
            generation: self.generation,
            observation,
        });
        match result {
            Err(e) => Err(e),
            Ok(_) if cleanup_failure.is_some() => Err(cleanup_failure.unwrap()),
            result => result,
        }
    }
}

pub(crate) struct Replay<'a, G: Native> {
    pub turns: &'a [Turn<G::Side>],
    pub traffic: &'a [Traffic],
    pub processes: &'a [Process],
    pub elapsed_ns: u64,
    pub game: &'a str,
}
pub(crate) fn replay<G: Native>(
    config: Settings<'_>,
    entry: Replay<'_, G>,
    game: &mut G::Game,
    assignment: [usize; 2],
    generation: &mut u64,
) -> Result<(), RunnerError> {
    if entry.turns.len() > config.max_actions as usize
        || entry.processes.len() > config.max_actions as usize
        || entry.game.len() > 1_048_576
    {
        return Err(RunnerError("report collection bounds exceeded".into()));
    }
    let traffic_bytes = entry
        .traffic
        .iter()
        .try_fold(0usize, |total, event| {
            serde_json::to_vec(event)
                .ok()
                .and_then(|bytes| total.checked_add(bytes.len()))
        })
        .ok_or_else(|| RunnerError("invalid traffic size".into()))?;
    if traffic_bytes > config.io.transcript_bytes {
        return Err(RunnerError("traffic exceeds configured capacity".into()));
    }
    let mut previous = 0;
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
        let command_index = |phase: &str, text: &str| {
            traffic.iter().position(|e| {
                e.direction == "command"
                    && e.phase == phase
                    && e.text == text
                    && e.written == Some(true)
            })
        };
        let position_index = command_index("position", &turn.position);
        let go_index = command_index("search", &turn.go);
        let response_index = traffic.iter().position(|e| {
            e.direction == "response"
                && e.phase == "search"
                && e.text == escaped(turn.reply.as_bytes())
                && e.written.is_none()
        });
        if traffic
            .iter()
            .any(|e| e.engine != turn.engine || e.version != turn.version)
            || !matches!((position_index,go_index,response_index),(Some(p),Some(g),Some(r)) if p<g&&g<r)
            || entry
                .processes
                .iter()
                .filter(|p| {
                    p.generation == turn.generation
                        && p.observation.engine == engine
                        && p.observation.exit.as_ref().is_some_and(|e| e.success)
                        && p.observation.cleanup_failures.is_empty()
                })
                .count()
                != 1
        {
            return Err(RunnerError(
                "accepted result lacks matching process/command/response evidence".into(),
            ));
        }
        G::replay_traffic(&traffic, turn, &config.engines[engine])?;
        *generation = turn.generation;
        G::apply_reply(game, &turn.reply)?;
    }
    if entry.processes.iter().any(|p| p.observation.engine > 1)
        || entry
            .traffic
            .iter()
            .any(|t| t.engine > 1 || t.text.len() > config.io.line_bytes * 4)
    {
        return Err(RunnerError("invalid process/traffic bounds".into()));
    }
    Ok(())
}
