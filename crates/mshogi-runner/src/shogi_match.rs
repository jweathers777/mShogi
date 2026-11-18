//! Persistent, clocked USI Shogi games for external strength studies.
//!
//! Each engine keeps one process for the whole game. The controller owns a
//! Fischer clock per side: it sends `go btime wtime binc winc`, measures the
//! wall time from the written `go` to the `bestmove` line, charges it to the
//! mover and then adds the increment. Native `mshogi.shogi.csa36.v1` admission
//! decides every reply and every result (including entering-king declarations
//! sent as `bestmove win`, resignation and the 320-move draw). Clock
//! exhaustion is an unscored experiment stop: the accepted profile has no
//! time-loss rule. Every report replays against the native rules, and its clock
//! ledger is recomputed from the recorded elapsed times.
use crate::{
    clocked::{Dialect, Host, Outcome},
    process::{Engine, escaped},
    shogi::{
        Failure, FailureKind, Opening, Process, Traffic, canonical, digest, position, reply, submit,
    },
    *,
};
use mshogi_core::{native::CanonicalRecord, shogi::*};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Equal Fischer clock for both sides, in milliseconds.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Clock {
    /// Initial time per side.
    pub base_ms: u64,
    /// Increment added after each accepted move.
    pub increment_ms: u64,
}

/// One clocked game between two USI engines.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchConfig {
    /// Config version, one.
    pub schema_version: u32,
    /// Exact native rules profile.
    pub profile: String,
    /// Experiment label.
    pub id: String,
    /// Two engines with their options.
    pub engines: [EngineConfig; 2],
    /// Known-history root and prefix.
    pub opening: Opening,
    /// Engine index (0 or 1) that plays Sente.
    pub sente: usize,
    /// Fischer clock of both sides.
    pub clock: Clock,
    /// Allowance beyond the remaining time for transport, in milliseconds.
    /// A reply later than remaining plus grace exhausts the clock.
    pub grace_ms: u64,
    /// Overall experiment watchdog; expiry is unscored, not a rules result.
    pub max_game_ms: u64,
    /// Process watchdogs; `response_ms` is unused (the clock bounds replies).
    pub watchdogs: Watchdogs,
    /// Stream and report bounds.
    pub io: IoLimits,
}

impl MatchConfig {
    /// Strict bounded parsing.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        if text.len() > 1_048_576 {
            return Err(RunnerError("match config exceeds 1 MiB".into()));
        }
        let config: Self = serde_json::from_str(text).map_err(|e| RunnerError(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }
    /// Validate before starting children.
    pub fn validate(&self) -> Result<(), RunnerError> {
        if self.schema_version != 1 || self.profile != PROFILE_ID || self.sente > 1 {
            return Err(RunnerError("unsupported version/profile/assignment".into()));
        }
        if self.id.is_empty()
            || self.id.len() > 128
            || !(1_000..=3_600_000).contains(&self.clock.base_ms)
            || self.clock.increment_ms > 60_000
            || self.grace_ms > 5_000
            || !(1..=21_600_000).contains(&self.max_game_ms)
        {
            return Err(RunnerError("unsupported clock or experiment bounds".into()));
        }
        if self.opening.moves.len() > 320
            || self.opening.sfen.as_ref().is_some_and(|s| s.len() > 4096)
        {
            return Err(RunnerError("invalid opening bounds".into()));
        }
        let game = self.opening.game()?;
        if game.conclusion().is_some() {
            return Err(RunnerError("opening is already concluded".into()));
        }
        crate::config::validate_processes(&self.engines, self.watchdogs, self.io)?;
        Ok(())
    }
    fn engine_for(&self, side: Side) -> usize {
        if side == Side::Sente {
            self.sente
        } else {
            1 - self.sente
        }
    }
}

/// One accepted reply with its clock ledger.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClockedTurn {
    /// Engine index.
    pub engine: usize,
    /// Native event count before the reply.
    pub version: usize,
    /// Canonical native prefix digest.
    pub context_sha256: String,
    /// Native side to move.
    pub actor: Side,
    /// Position command with the complete known history.
    pub position: String,
    /// Clock command.
    pub go: String,
    /// Accepted bestmove line.
    pub reply: String,
    /// Remaining Sente and Gote time before the reply.
    pub clock_before_ms: [u64; 2],
    /// Wall time from the written go to the bestmove line (rounded up).
    pub elapsed_ms: u64,
    /// Info lines received during this search (not retained).
    pub info_lines: u64,
}

/// A reply attempt that ran past the mover's remaining time plus grace.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Exhaustion {
    /// Engine index.
    pub engine: usize,
    /// Native side to move.
    pub actor: Side,
    /// Remaining time of the mover.
    pub remaining_ms: u64,
    /// Elapsed time when the controller stopped waiting.
    pub elapsed_ms: u64,
}

/// Why the experiment stopped.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchStop {
    /// Native conclusion; the game is scored.
    NativeConclusion,
    /// A side's clock ran out; unscored.
    ClockExhausted,
    /// Overall experiment watchdog; unscored.
    TimeLimit,
    /// Protocol, process or admission failure; unscored.
    Failure,
}

/// Complete evidence of one clocked game.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchReport {
    /// Report version, one.
    pub schema_version: u32,
    /// Effective configuration.
    pub config: MatchConfig,
    /// Executable and model identities.
    pub identities: [EngineIdentity; 2],
    /// Platform that ran the processes.
    pub platform: String,
    /// Advertised `id` lines of each engine.
    pub engine_ids: [Vec<String>; 2],
    /// Canonical native game record.
    pub game: String,
    /// Native conclusion, checked on replay.
    pub conclusion: Option<Conclusion>,
    /// Half points of engine 0 and engine 1, only for native conclusions.
    pub engine_score: Option<[u8; 2]>,
    /// Stop reason.
    pub stop: MatchStop,
    /// Failure evidence for `Failure`.
    pub failure: Option<Failure>,
    /// Clock exhaustion evidence for `ClockExhausted`.
    pub exhaustion: Option<Exhaustion>,
    /// Accepted replies in order.
    pub turns: Vec<ClockedTurn>,
    /// Retained protocol traffic except search `info` lines.
    pub traffic: Vec<Traffic>,
    /// False if traffic retention hit its bound.
    pub traffic_complete: bool,
    /// Final process observations of both engines.
    pub processes: Vec<Process>,
    /// Overall elapsed time.
    pub elapsed_ns: u64,
}

fn go(clock: [u64; 2], increment: u64) -> String {
    format!(
        "go btime {} wtime {} binc {increment} winc {increment}",
        clock[0], clock[1]
    )
}

impl MatchReport {
    /// Strict decoding followed by native replay.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        if text.len() > 67_108_864 {
            return Err(RunnerError("match report exceeds 64 MiB".into()));
        }
        let report: Self = serde_json::from_str(text).map_err(|e| RunnerError(e.to_string()))?;
        report.replay()?;
        Ok(report)
    }
    /// Canonical JSON.
    pub fn to_json(&self) -> Result<String, RunnerError> {
        serde_json::to_string(self).map_err(|e| RunnerError(e.to_string()))
    }
    /// Replay every accepted reply through the native rules and recompute
    /// the clock ledger, stop and score.
    pub fn replay(&self) -> Result<Game, RunnerError> {
        let c = &self.config;
        c.validate()?;
        if self.schema_version != 1 {
            return Err(RunnerError("unsupported match report version".into()));
        }
        let mut game = c.opening.game()?;
        let mut clock = [c.clock.base_ms; 2];
        for turn in &self.turns {
            if game.conclusion().is_some() {
                return Err(RunnerError("turn after native conclusion".into()));
            }
            let actor = game.position().side_to_move();
            let side = usize::from(actor == Side::Gote);
            if turn.engine != c.engine_for(actor)
                || turn.actor != actor
                || turn.version != game.events().len()
                || turn.context_sha256 != digest(&game)?
                || turn.position != position(&game)
                || turn.clock_before_ms != clock
                || turn.go != go(clock, c.clock.increment_ms)
                || turn.elapsed_ms > clock[side].saturating_add(c.grace_ms)
            {
                return Err(RunnerError("turn context or clock ledger mismatch".into()));
            }
            let logged = |direction: &str, text: &str| {
                self.traffic.iter().any(|t| {
                    t.engine == turn.engine
                        && t.version == turn.version
                        && t.direction == direction
                        && t.text == text
                })
            };
            if self.traffic_complete
                && !(logged("command", &turn.go)
                    && logged("response", &escaped(turn.reply.as_bytes())))
            {
                return Err(RunnerError("turn lacks command/response evidence".into()));
            }
            clock[side] = clock[side].saturating_sub(turn.elapsed_ms) + c.clock.increment_ms;
            submit(&mut game, reply(&turn.reply).map_err(RunnerError)?).map_err(RunnerError)?;
        }
        let CanonicalRecord::Shogi(record) =
            CanonicalRecord::from_json(&self.game).map_err(|e| RunnerError(e.to_string()))?
        else {
            return Err(RunnerError("report game is not Shogi".into()));
        };
        if record.to_game().map_err(|e| RunnerError(e.to_string()))? != game
            || self.conclusion != game.conclusion()
        {
            return Err(RunnerError("canonical game or conclusion mismatch".into()));
        }
        let expected_score = match (&self.stop, game.conclusion()) {
            (MatchStop::NativeConclusion, Some(conclusion)) => {
                let [sente, gote] = conclusion.score_half_points();
                Some(if c.sente == 0 {
                    [sente, gote]
                } else {
                    [gote, sente]
                })
            }
            _ => None,
        };
        let consistent = match self.stop {
            MatchStop::NativeConclusion => game.conclusion().is_some() && self.failure.is_none(),
            MatchStop::ClockExhausted => {
                game.conclusion().is_none()
                    && self.exhaustion.as_ref().is_some_and(|e| {
                        let actor = game.position().side_to_move();
                        let side = usize::from(actor == Side::Gote);
                        e.actor == actor
                            && e.engine == c.engine_for(actor)
                            && e.remaining_ms == clock[side]
                            && e.elapsed_ms > clock[side].saturating_add(c.grace_ms)
                    })
            }
            MatchStop::TimeLimit => game.conclusion().is_none(),
            // Failures stop an open game; a concluded game cannot also fail.
            MatchStop::Failure => game.conclusion().is_none() && self.failure.is_some(),
        };
        if !consistent
            || self.engine_score != expected_score
            || (self.stop != MatchStop::ClockExhausted && self.exhaustion.is_some())
            || (self.stop != MatchStop::Failure && self.failure.is_some())
        {
            return Err(RunnerError("stop, score or exhaustion mismatch".into()));
        }
        Ok(game)
    }
}

/// Play one clocked game. Native admission alone decides results.
pub fn run(config: &MatchConfig) -> Result<MatchReport, RunnerError> {
    config.validate()?;
    let identities = config.engines.each_ref().map(|e| EngineIdentity {
        executable: crate::run::identity(&e.path),
        model: e.model_path.as_ref().map(|p| crate::run::identity(p)),
        model_identity: e.model_identity.clone(),
    });
    let mut game = config.opening.game()?;
    let mut host = Host {
        dialect: Dialect {
            hello: "usi",
            ok: "usiok",
            newgame: "usinewgame",
        },
        engines: &config.engines,
        watchdogs: config.watchdogs,
        io: config.io,
        start: Instant::now(),
        version: game.events().len(),
        traffic: vec![],
        bytes: 0,
        complete: true,
    };
    let mut engines: Vec<Engine> = Vec::new();
    let mut failure = None;
    let mut engine_ids: [Vec<String>; 2] = [vec![], vec![]];
    for index in 0..2 {
        match Engine::spawn(index, &config.engines[index], config.io) {
            Ok(engine) => engines.push(engine),
            Err(e) => {
                failure = Some(host.failure(index, FailureKind::ProcessStart, "spawn", e));
                break;
            }
        }
    }
    if failure.is_none() {
        for index in 0..2 {
            match host.prepare(&mut engines[index]) {
                Ok(ids) => engine_ids[index] = ids,
                Err(e) => {
                    failure = Some(e);
                    break;
                }
            }
        }
    }
    let mut clock = [config.clock.base_ms; 2];
    let mut turns = Vec::new();
    let mut exhaustion = None;
    let deadline = host.start + Duration::from_millis(config.max_game_ms);
    let stop = if failure.is_some() {
        MatchStop::Failure
    } else {
        loop {
            if game.conclusion().is_some() {
                break MatchStop::NativeConclusion;
            }
            if Instant::now() >= deadline {
                break MatchStop::TimeLimit;
            }
            host.version = game.events().len();
            let actor = game.position().side_to_move();
            let side = usize::from(actor == Side::Gote);
            let index = config.engine_for(actor);
            let context = ClockedTurn {
                engine: index,
                version: host.version,
                context_sha256: digest(&game)?,
                actor,
                position: position(&game),
                go: go(clock, config.clock.increment_ms),
                reply: String::new(),
                clock_before_ms: clock,
                elapsed_ms: 0,
                info_lines: 0,
            };
            let engine = &mut engines[index];
            let outcome = host.search(
                engine,
                &context.position,
                &context.go,
                clock[side],
                config.grace_ms,
                deadline,
            );
            match outcome {
                Ok(Outcome::Accepted(line, elapsed, info)) => {
                    let command = match reply(&line) {
                        Ok(command) => command,
                        Err(e) => {
                            failure =
                                Some(host.failure(index, FailureKind::MalformedReply, "search", e));
                            break MatchStop::Failure;
                        }
                    };
                    if let Err(e) = submit(&mut game, command) {
                        failure = Some(host.failure(
                            index,
                            if matches!(command, GameCommand::Play(_)) {
                                FailureKind::IllegalAction
                            } else {
                                FailureKind::Native
                            },
                            "admission",
                            e,
                        ));
                        break MatchStop::Failure;
                    }
                    clock[side] = clock[side].saturating_sub(elapsed) + config.clock.increment_ms;
                    turns.push(ClockedTurn {
                        reply: line,
                        elapsed_ms: elapsed,
                        info_lines: info,
                        ..context
                    });
                }
                Ok(Outcome::Watchdog) => break MatchStop::TimeLimit,
                Ok(Outcome::Exhausted(elapsed)) => {
                    exhaustion = Some(Exhaustion {
                        engine: index,
                        actor,
                        remaining_ms: clock[side],
                        elapsed_ms: elapsed,
                    });
                    break MatchStop::ClockExhausted;
                }
                Err(e) => {
                    failure = Some(e);
                    break MatchStop::Failure;
                }
            }
        }
    };
    // Tell both engines the native result, then retire them.
    if let Some(conclusion) = game.conclusion() {
        for (index, engine) in engines.iter_mut().enumerate() {
            let side = if config.sente == index {
                Side::Sente
            } else {
                Side::Gote
            };
            let notice = match conclusion {
                Conclusion::Draw(_) => "draw",
                Conclusion::Win { winner, .. } if winner == side => "win",
                _ => "lose",
            };
            let _ = host.send(engine, "gameover", &format!("gameover {notice}"));
        }
    }
    let mut processes = Vec::new();
    for engine in engines.iter_mut() {
        let _ = host.send(engine, "cleanup", "quit");
        processes.push(Process {
            generation: 1,
            observation: engine.cleanup(config.watchdogs.cleanup_ms),
        });
    }
    let engine_score = match (&stop, game.conclusion()) {
        (MatchStop::NativeConclusion, Some(conclusion)) => {
            let [sente, gote] = conclusion.score_half_points();
            Some(if config.sente == 0 {
                [sente, gote]
            } else {
                [gote, sente]
            })
        }
        _ => None,
    };
    let report = MatchReport {
        schema_version: 1,
        config: config.clone(),
        identities,
        platform: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
        engine_ids,
        game: canonical(&game)?,
        conclusion: game.conclusion(),
        engine_score,
        stop,
        failure,
        exhaustion,
        turns,
        traffic: host.traffic,
        traffic_complete: host.complete,
        processes,
        elapsed_ns: host.start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64,
    };
    report.replay()?;
    Ok(report)
}
