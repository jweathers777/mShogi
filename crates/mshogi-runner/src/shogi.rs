//! Untimed USI experiments. A fresh child per turn gives USI's untagged replies
//! an unambiguous execution generation; retired output can never serve a later go.
use crate::{
    process::{Engine, Output, escaped},
    *,
};
use mshogi_core::{native::CanonicalRecord, shogi::*};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    sync::mpsc::RecvTimeoutError,
    time::{Duration, Instant},
};

/// One known-history root and supplied legal board prefix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Opening {
    /// Stable experiment label.
    pub id: String,
    /// None is the exact standard root; Some is an imported diagram, never prior history.
    pub sfen: Option<String>,
    /// Complete known board-action suffix.
    pub moves: Vec<String>,
}
impl Opening {
    fn game(&self) -> Result<Game, RunnerError> {
        let mut game = if let Some(sfen) = &self.sfen {
            let position = Position::from_sfen(sfen).map_err(|e| RunnerError(e.to_string()))?;
            let root = match GameRoot::import(
                position.clone(),
                RootCategory::FullMaterialDiagram,
                CounterSource::Supplied,
            ) {
                Err(RootError::CategoryMismatch) => GameRoot::import(
                    position,
                    RootCategory::ReducedMaterialDiagram,
                    CounterSource::Supplied,
                ),
                r => r,
            }
            .map_err(|e| RunnerError(format!("native root: {e:?}")))?;
            Game::from_root(root)
        } else {
            Game::new()
        };
        for text in &self.moves {
            let action: Action = text
                .parse()
                .map_err(|e| RunnerError(format!("opening action: {e}")))?;
            submit(&mut game, GameCommand::Play(action)).map_err(RunnerError)?;
        }
        Ok(game)
    }
}
/// Explicit untimed, restart-per-turn experiment input, independent of chess config.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Current report/config version, one.
    pub schema_version: u32,
    /// Exact immutable native identity.
    pub profile: String,
    /// Experiment label.
    pub id: String,
    /// Must be untimed; no clock adjudication is supplied here.
    pub time_control: String,
    /// Must be fresh_per_turn; no untagged reply crosses request generations.
    pub process_policy: String,
    /// Individual executable/option settings (existing bounded process configuration).
    pub engines: [EngineConfig; 2],
    /// Complete roots/prefixes, at most four.
    pub openings: Vec<Opening>,
    /// Reverse engine assignments for every opening.
    pub paired: bool,
    /// Per-engine finite work, no movetime.
    pub work: [WorkLimit; 2],
    /// Maximum engine commands after the opening; an experiment cutoff, not a rule.
    pub max_actions: u32,
    /// Overall per-game experiment watchdog; expiry is unfinished, not a chess clock.
    pub max_game_ms: u64,
    /// Independent process watchdogs.
    pub watchdogs: Watchdogs,
    /// Explicit stream/report bounds.
    pub io: IoLimits,
}
impl Config {
    /// Strict, bounded input parsing.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        if text.len() > 1_048_576 {
            return Err(RunnerError("shogi config exceeds1MiB".into()));
        }
        let config: Self = serde_json::from_str(text).map_err(|e| RunnerError(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }
    /// Validate typed callers before starting children.
    pub fn validate(&self) -> Result<(), RunnerError> {
        if self.schema_version != 1
            || self.profile != PROFILE_ID
            || self.time_control != "untimed"
            || self.process_policy != "fresh_per_turn"
        {
            return Err(RunnerError(
                "unsupported shogi version/profile/time_control/process_policy".into(),
            ));
        }
        if self.id.is_empty()
            || self.id.len() > 128
            || self.openings.is_empty()
            || self.openings.len() > 4
            || !(1..=32).contains(&self.max_actions)
            || !(1..=600_000).contains(&self.max_game_ms)
        {
            return Err(RunnerError("unsupported experiment bounds".into()));
        }
        for w in self.work {
            if w.movetime_ms.is_some()
                || w.depth.is_none_or(|d| !(1..=4).contains(&d))
                || w.nodes.is_none_or(|n| n > 20_000)
            {
                return Err(RunnerError(
                    "finite depth1..4/nodes0..20000 required; timed work unsupported".into(),
                ));
            }
        }
        for o in &self.openings {
            if o.id.is_empty()
                || o.id.len() > 128
                || o.moves.len() > 320
                || o.sfen.as_ref().is_some_and(|s| s.len() > 4096)
            {
                return Err(RunnerError("invalid opening bounds".into()));
            }
            o.game()?;
        }
        crate::config::validate_processes(&self.engines, self.watchdogs, self.io)?;
        if self.io.line_bytes > 4096
            || self.io.queued_lines > 32
            || self.io.transcript_bytes > 1_048_576
            || self.io.stderr_bytes > 4096
        {
            return Err(RunnerError(
                "shogi experiment I/O caps exceed supported report bounds".into(),
            ));
        }
        Ok(())
    }
}
/// Experiment failure, never a native foul or competition score.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureKind {
    /// Child launch failed.
    ProcessStart,
    /// Unexpected child exit.
    UnexpectedExit,
    /// Clean EOF without reply.
    MissingReply,
    /// Invalid protocol grammar/output.
    MalformedReply,
    /// Board action rejected by native admission.
    IllegalAction,
    /// Required reply missed process watchdog.
    Watchdog,
    /// Untagged reply arrived outside its request.
    LateReply,
    /// Stream/record resource cap reached.
    ResourceLimit,
    /// Advertised options/configuration mismatch.
    Configuration,
    /// Native invariant/admission failure unrelated to action legality.
    Native,
}
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
pub struct Turn {
    /// Engine configuration index.
    pub engine: usize,
    /// Unique request/process identity within the report.
    pub generation: u64,
    /// Native prefix including opening events, before result acceptance.
    pub version: usize,
    /// Full canonical-prefix digest, not a board-only identity.
    pub context_sha256: String,
    /// Captured native actor.
    pub actor: Side,
    /// Actual position command with complete known history.
    pub position: String,
    /// Actual finite work request.
    pub go: String,
    /// Complete accepted bestmove line, including a validated ponder suffix if supplied.
    pub reply: String,
}
/// Why this experiment stopped; distinct from the native conclusion.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stop {
    /// Native authoritative conclusion.
    NativeConclusion,
    /// Engine command experiment cap.
    ActionLimit,
    /// Overall experiment watchdog, not a clock loss.
    TimeLimit,
    /// Unscored failure.
    Failure,
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
/// One complete accepted native game plus bounded external observations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameReport {
    /// Opening index.
    pub opening: usize,
    /// Engine indices assigned to Sente/Gote.
    pub assignment: [usize; 2],
    /// Complete typed canonical envelope string preserves strict native raw parsing.
    pub game: String,
    /// Native conclusion independently checked on replay.
    pub conclusion: Option<Conclusion>,
    /// Half-point competition units only for completed successful experiments.
    pub score: Option<[u8; 2]>,
    /// Experiment stop reason.
    pub stop: Stop,
    /// Unscored execution failure if present.
    pub failure: Option<Failure>,
    /// Every accepted engine proposal with exact context provenance.
    pub turns: Vec<Turn>,
    /// Bounded complete traffic unless resource failure explicitly marks otherwise.
    pub traffic: Vec<Traffic>,
    /// Whether a hostile output exceeded retention capacity.
    pub traffic_complete: bool,
    /// Every fresh process's actual exit/diagnostic/cleanup evidence.
    pub processes: Vec<Process>,
    /// Actual overall elapsed experiment time, not a rules clock.
    pub elapsed_ns: u64,
}
/// Self-contained replay evidence. External engines are not launched on replay.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Report {
    /// Current report schema, one.
    pub schema_version: u32,
    /// Effective configuration including finite limits and all options.
    pub config: Config,
    /// Actual executable/model hashes (no PATH lookup).
    pub identities: [EngineIdentity; 2],
    /// Platform which actually ran these processes.
    pub platform: String,
    /// Opening/assignment-specific games.
    pub games: Vec<GameReport>,
}
fn canonical(game: &Game) -> Result<String, RunnerError> {
    CanonicalRecord::shogi(GameRecord::from_game(game))
        .to_json()
        .map_err(|e| RunnerError(e.to_string()))
}
fn digest(game: &Game) -> Result<String, RunnerError> {
    Ok(format!("{:x}", Sha256::digest(canonical(game)?.as_bytes())))
}
fn submit(game: &mut Game, command: GameCommand) -> Result<(), String> {
    game.submit(GameRequest {
        expected: game.context(),
        actor: game.position().side_to_move(),
        command,
    })
    .map(|_| ())
    .map_err(|e| format!("{e:?}"))
}
fn position(game: &Game) -> String {
    let mut text = if game.root().has_complete_initial_history() {
        "position startpos".into()
    } else {
        format!("position sfen {}", game.root().position().to_sfen())
    };
    if !game.events().is_empty() {
        text.push_str(" moves");
        for e in game.events() {
            if let GameCommand::Play(action) = e.command {
                text.push(' ');
                text.push_str(&action.to_string());
            }
        }
    }
    text
}
fn go(work: WorkLimit) -> String {
    format!(
        "go depth {} nodes {}",
        work.depth.unwrap(),
        work.nodes.unwrap()
    )
}
fn option_value(advertised: &str, value: &str) -> bool {
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
fn reply(text: &str) -> Result<GameCommand, String> {
    let words: Vec<_> = text.split_ascii_whitespace().collect();
    let token = match words.as_slice() {
        ["bestmove", token] => *token,
        ["bestmove", token, "ponder", ponder] => {
            ponder
                .parse::<Action>()
                .map_err(|e| format!("malformed ponder action: {e}"))?;
            if matches!(*token, "win" | "resign") {
                return Err("terminal bestmove cannot carry ponder".into());
            }
            *token
        }
        _ => return Err("expected bestmove <action|win|resign> [ponder <board-action>]".into()),
    };
    match token {
        "win" => Ok(GameCommand::DeclareEnteringKing),
        "resign" => Ok(GameCommand::Resign),
        token => token
            .parse::<Action>()
            .map(GameCommand::Play)
            .map_err(|e| e.to_string()),
    }
}
impl Report {
    /// Strict bounded report decoding followed by native replay.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        if text.len() > 67_108_864 {
            return Err(RunnerError("shogi report exceeds64MiB".into()));
        }
        let report: Self = serde_json::from_str(text).map_err(|e| RunnerError(e.to_string()))?;
        report.replay()?;
        Ok(report)
    }
    /// Canonical report JSON; all accepted history remains in the native envelope.
    pub fn to_json(&self) -> Result<String, RunnerError> {
        let text = serde_json::to_string(self).map_err(|e| RunnerError(e.to_string()))?;
        if text.len() > 67_108_864 {
            return Err(RunnerError("report exceeds supported64MiB bound".into()));
        }
        Ok(text)
    }
    /// Verify root, all accepted responses, context/limits/provenance and conclusions.
    pub fn replay(&self) -> Result<Vec<Game>, RunnerError> {
        self.config.validate()?;
        if self.schema_version != 1
            || self.games.len()
                != self.config.openings.len() * if self.config.paired { 2 } else { 1 }
        {
            return Err(RunnerError("report version/game count mismatch".into()));
        }
        let mut games = Vec::new();
        let mut generation = 0;
        for (index, entry) in self.games.iter().enumerate() {
            let opening = index / if self.config.paired { 2 } else { 1 };
            let assignment = if self.config.paired && index % 2 == 1 {
                [1, 0]
            } else {
                [0, 1]
            };
            if entry.opening != opening || entry.assignment != assignment {
                return Err(RunnerError("opening/assignment mismatch".into()));
            }
            if entry.turns.len() > self.config.max_actions as usize
                || entry.processes.len() > self.config.max_actions as usize
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
            if traffic_bytes > self.config.io.transcript_bytes {
                return Err(RunnerError("traffic exceeds configured capacity".into()));
            }
            let mut previous = 0;
            for event in &entry.traffic {
                if event.at_ns < previous
                    || event.at_ns > entry.elapsed_ns
                    || event.engine > 1
                    || event.generation == 0
                {
                    return Err(RunnerError("traffic ordering/identity invalid".into()));
                }
                previous = event.at_ns;
            }
            let mut game = self.config.openings[opening].game()?;
            for turn in &entry.turns {
                let actor = game.position().side_to_move();
                let engine = assignment[usize::from(actor == Side::Gote)];
                if turn.engine != engine
                    || turn.actor != actor
                    || turn.version != game.events().len()
                    || turn.context_sha256 != digest(&game)?
                    || turn.position != position(&game)
                    || turn.go != go(self.config.work[engine])
                    || turn.generation <= generation
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
                generation = turn.generation;
                let command = reply(&turn.reply).map_err(RunnerError)?;
                submit(&mut game, command).map_err(RunnerError)?;
            }
            let CanonicalRecord::Shogi(record) =
                CanonicalRecord::from_json(&entry.game).map_err(|e| RunnerError(e.to_string()))?
            else {
                return Err(RunnerError("report game is not shogi".into()));
            };
            if record.to_game().map_err(|e| RunnerError(e.to_string()))? != game
                || entry.conclusion != game.conclusion()
            {
                return Err(RunnerError("canonical game/conclusion mismatch".into()));
            }
            let score = if entry.stop == Stop::NativeConclusion && entry.failure.is_none() {
                game.conclusion().map(Conclusion::score_half_points)
            } else {
                None
            };
            if (!entry.traffic_complete && !matches!(entry.stop, Stop::Failure | Stop::TimeLimit))
                || entry.score != score
                || (entry.stop == Stop::NativeConclusion) != game.conclusion().is_some()
                    && entry.stop != Stop::Failure
                || (entry.stop == Stop::Failure) != entry.failure.is_some()
                || entry.turns.len() > self.config.max_actions as usize
                || (entry.stop == Stop::ActionLimit
                    && entry.turns.len() != self.config.max_actions as usize)
            {
                return Err(RunnerError("experiment stop/score mismatch".into()));
            }
            if entry.processes.iter().any(|p| p.observation.engine > 1)
                || entry
                    .traffic
                    .iter()
                    .any(|t| t.engine > 1 || t.text.len() > self.config.io.line_bytes * 4)
            {
                return Err(RunnerError("invalid process/traffic bounds".into()));
            }
            games.push(game);
        }
        Ok(games)
    }
}
struct Controller<'a> {
    config: &'a Config,
    start: Instant,
    generation: u64,
    version: usize,
    traffic: Vec<Traffic>,
    bytes: usize,
    complete: bool,
    processes: Vec<Process>,
}
impl Controller<'_> {
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
            at_ns: self.start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64,
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
                    let status = engine.exit_after_eof();
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
            if phase == "handshake" && (line.starts_with("id ") || line.starts_with("option ")) {
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
    fn turn(&mut self, game: &Game, engine_index: usize, turn: Turn) -> Result<Turn, Failure> {
        let mut engine = Engine::spawn(
            engine_index,
            &self.config.engines[engine_index],
            self.config.io,
        )
        .map_err(|e| self.failure(engine_index, FailureKind::ProcessStart, "spawn", e))?;
        let result = (|| {
            self.send(&mut engine, "handshake", "usi")?;
            let lines = self.await_token(
                &mut engine,
                "handshake",
                "usiok",
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
            self.send(&mut engine, "newgame", "usinewgame")?;
            self.send(&mut engine, "position", &turn.position)?;
            self.send(&mut engine, "search", &turn.go)?;
            let deadline =
                Instant::now() + Duration::from_millis(self.config.watchdogs.response_ms);
            let response = loop {
                let line = self.line(&mut engine, "search", deadline)?;
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
            let command = reply(&response).map_err(|e| {
                self.failure(engine_index, FailureKind::MalformedReply, "search", e)
            })?;
            let mut candidate = game.clone();
            submit(&mut candidate, command).map_err(|e| {
                self.failure(
                    engine_index,
                    if matches!(command, GameCommand::Play(_)) {
                        FailureKind::IllegalAction
                    } else {
                        FailureKind::Native
                    },
                    "admission",
                    e,
                )
            })?;
            if let Some(conclusion) = candidate.conclusion() {
                let actor = game.position().side_to_move();
                let notice = match conclusion {
                    Conclusion::Draw(_) => "draw",
                    Conclusion::Win { winner, .. } if winner == actor => "win",
                    _ => "lose",
                };
                self.send(&mut engine, "gameover", &format!("gameover {notice}"))?;
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
/// Execute actual USI children; native shogi admission alone determines outcomes.
pub fn run(config: &Config) -> Result<Report, RunnerError> {
    config.validate()?;
    let identities = config.engines.each_ref().map(|e| EngineIdentity {
        executable: crate::run::identity(&e.path),
        model: e.model_path.as_ref().map(|p| crate::run::identity(p)),
        model_identity: e.model_identity.clone(),
    });
    let mut report = Report {
        schema_version: 1,
        config: config.clone(),
        identities,
        platform: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
        games: Vec::new(),
    };
    let mut generation = 0;
    for (opening_index, opening) in config.openings.iter().enumerate() {
        for reversed in 0..if config.paired { 2 } else { 1 } {
            let assignment = if reversed == 1 { [1, 0] } else { [0, 1] };
            let mut game = opening.game()?;
            let mut host = Controller {
                config,
                start: Instant::now(),
                generation,
                version: game.events().len(),
                traffic: vec![],
                bytes: 0,
                complete: true,
                processes: vec![],
            };
            let mut turns = Vec::new();
            let mut failure = None;
            let stop = loop {
                if game.conclusion().is_some() {
                    break Stop::NativeConclusion;
                }
                if turns.len() >= config.max_actions as usize {
                    break Stop::ActionLimit;
                }
                if host.start.elapsed() >= Duration::from_millis(config.max_game_ms) {
                    break Stop::TimeLimit;
                }
                host.generation = host
                    .generation
                    .checked_add(1)
                    .ok_or_else(|| RunnerError("execution generation exhausted".into()))?;
                host.version = game.events().len();
                let actor = game.position().side_to_move();
                let engine = assignment[usize::from(actor == Side::Gote)];
                let context = Turn {
                    engine,
                    generation: host.generation,
                    version: host.version,
                    context_sha256: digest(&game)?,
                    actor,
                    position: position(&game),
                    go: go(config.work[engine]),
                    reply: String::new(),
                };
                match host.turn(&game, engine, context) {
                    Ok(turn) => {
                        submit(&mut game, reply(&turn.reply).map_err(RunnerError)?)
                            .map_err(RunnerError)?;
                        turns.push(turn);
                    }
                    Err(e) if e.phase == "experiment_limit" => break Stop::TimeLimit,
                    Err(e) => {
                        failure = Some(e);
                        break Stop::Failure;
                    }
                }
            };
            generation = host.generation;
            report.games.push(GameReport {
                opening: opening_index,
                assignment,
                game: canonical(&game)?,
                conclusion: game.conclusion(),
                score: if stop == Stop::NativeConclusion {
                    game.conclusion().map(Conclusion::score_half_points)
                } else {
                    None
                },
                stop,
                failure,
                turns,
                traffic: host.traffic,
                traffic_complete: host.complete,
                processes: host.processes,
                elapsed_ns: host.start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64,
            });
        }
    }
    report.replay()?;
    report.to_json()?;
    Ok(report)
}
