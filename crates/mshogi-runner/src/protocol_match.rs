//! Shared clock ledger, native turn execution and persistent process lifecycle.
use crate::{
    clocked::{Dialect, Host, Outcome},
    process::{Engine, escaped},
    protocol::{self, End, Native},
    shogi::{Failure, FailureKind, Process, Traffic},
    *,
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
/// One accepted reply with its clock ledger.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClockedTurn<S> {
    /// Engine index.
    pub engine: usize,
    /// Native event count before the reply.
    pub version: usize,
    /// Canonical native prefix digest.
    pub context_sha256: String,
    /// Native side to move.
    pub actor: S,
    /// Position command with the complete known history.
    pub position: String,
    /// Clock command.
    pub go: String,
    /// Accepted bestmove line.
    pub reply: String,
    /// Remaining time for the two native sides before the reply.
    pub clock_before_ms: [u64; 2],
    /// Wall time from the written go to the bestmove line (rounded up).
    pub elapsed_ms: u64,
    /// Info lines received during this search (not retained).
    pub info_lines: u64,
}

/// A reply attempt that ran past the mover's remaining time plus grace.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Exhaustion<S> {
    /// Engine index.
    pub engine: usize,
    /// Native side to move.
    pub actor: S,
    /// Remaining time of the mover.
    pub remaining_ms: u64,
    /// Elapsed time when the controller stopped waiting.
    pub elapsed_ms: u64,
}

/// Complete evidence of one clocked game.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchReport<C, S, O, E> {
    /// Report version, one.
    pub schema_version: u32,
    /// Effective configuration.
    pub config: C,
    /// Executable and model identities.
    pub identities: [EngineIdentity; 2],
    /// Platform that ran the processes.
    pub platform: String,
    /// Advertised `id` lines of each engine.
    pub engine_ids: [Vec<String>; 2],
    /// Canonical native game record.
    pub game: String,
    /// Native conclusion, checked on replay.
    pub conclusion: Option<O>,
    /// Half points of engine 0 and engine 1, only for native conclusions.
    pub engine_score: Option<[u8; 2]>,
    /// Stop reason.
    pub stop: E,
    /// Failure evidence for `Failure`.
    pub failure: Option<Failure>,
    /// Clock exhaustion evidence for `ClockExhausted`.
    pub exhaustion: Option<Exhaustion<S>>,
    /// Accepted replies in order.
    pub turns: Vec<ClockedTurn<S>>,
    /// Retained protocol traffic except search `info` lines.
    pub traffic: Vec<Traffic>,
    /// False if traffic retention hit its bound.
    pub traffic_complete: bool,
    /// Final process observations of both engines.
    pub processes: Vec<Process>,
    /// Overall elapsed time.
    pub elapsed_ns: u64,
}

pub(crate) trait MatchNative: Native {
    type Wire: Copy;
    fn dialect(wire: Self::Wire) -> Dialect;
    fn position_for(wire: Self::Wire, game: &Self::Game) -> String;
    fn go(wire: Self::Wire, clock: [u64; 2], increment: [u64; 2], side: usize) -> String;
    fn admit(
        game: &mut Self::Game,
        wire: Self::Wire,
        text: &str,
    ) -> Result<(), (FailureKind, &'static str, String)>;
    fn gameover(_game: &Self::Game, _side: usize) -> Option<String> {
        None
    }
}
pub(crate) struct Settings<'a, G: MatchNative> {
    pub engines: &'a [EngineConfig; 2],
    pub watchdogs: Watchdogs,
    pub io: IoLimits,
    pub max_game_ms: u64,
    pub grace_ms: u64,
    pub first: usize,
    pub clock: [u64; 2],
    pub increment: [u64; 2],
    pub wire: [G::Wire; 2],
}
impl<G: MatchNative> Settings<'_, G> {
    fn engine_for(&self, side: G::Side) -> usize {
        if G::side(side) == 0 {
            self.first
        } else {
            1 - self.first
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Stop {
    Native(End),
    ClockExhausted,
    TimeLimit,
    Failure,
}
pub(crate) struct Completed<G: MatchNative> {
    pub game: G::Game,
    pub stop: Stop,
    pub identities: [EngineIdentity; 2],
    pub engine_ids: [Vec<String>; 2],
    pub failure: Option<Failure>,
    pub exhaustion: Option<Exhaustion<G::Side>>,
    pub turns: Vec<ClockedTurn<G::Side>>,
    pub traffic: Vec<Traffic>,
    pub traffic_complete: bool,
    pub processes: Vec<Process>,
    pub elapsed_ns: u64,
}
impl<G: MatchNative> Completed<G> {
    pub fn report<C, O, E>(
        self,
        config: C,
        conclusion: Option<O>,
        engine_score: Option<[u8; 2]>,
        stop: E,
    ) -> Result<MatchReport<C, G::Side, O, E>, RunnerError> {
        Ok(MatchReport {
            schema_version: 1,
            config,
            identities: self.identities,
            platform: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
            engine_ids: self.engine_ids,
            game: G::canonical(&self.game)?,
            conclusion,
            engine_score,
            stop,
            failure: self.failure,
            exhaustion: self.exhaustion,
            turns: self.turns,
            traffic: self.traffic,
            traffic_complete: self.traffic_complete,
            processes: self.processes,
            elapsed_ns: self.elapsed_ns,
        })
    }
}
pub(crate) fn run<G: MatchNative>(
    config: Settings<'_, G>,
    mut game: G::Game,
) -> Result<Completed<G>, RunnerError> {
    let identities = protocol::identities(config.engines);
    let mut host = Host {
        dialects: config.wire.map(G::dialect),
        engines: config.engines,
        watchdogs: config.watchdogs,
        io: config.io,
        start: Instant::now(),
        version: G::events(&game),
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
    let mut clock = config.clock;
    let mut turns = Vec::new();
    let mut exhaustion = None;
    let deadline = host.start + Duration::from_millis(config.max_game_ms);
    let stop = if failure.is_some() {
        Stop::Failure
    } else {
        loop {
            if let Some(end) = G::end(&game) {
                break Stop::Native(end);
            }
            if Instant::now() >= deadline {
                break Stop::TimeLimit;
            }
            host.version = G::events(&game);
            let actor = G::actor(&game);
            let side = G::side(actor);
            let index = config.engine_for(actor);
            let context = ClockedTurn {
                engine: index,
                version: host.version,
                context_sha256: protocol::digest::<G>(&game)?,
                actor,
                position: G::position_for(config.wire[index], &game),
                go: G::go(config.wire[index], clock, config.increment, side),
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
                    if let Err((kind, phase, detail)) =
                        G::admit(&mut game, config.wire[index], &line)
                    {
                        failure = Some(host.failure(index, kind, phase, detail));
                        break Stop::Failure;
                    }
                    clock[side] = clock[side].saturating_sub(elapsed) + config.increment[side];
                    turns.push(ClockedTurn {
                        reply: line,
                        elapsed_ms: elapsed,
                        info_lines: info,
                        ..context
                    });
                }
                Ok(Outcome::Watchdog) => break Stop::TimeLimit,
                Ok(Outcome::Exhausted(elapsed)) => {
                    exhaustion = Some(Exhaustion {
                        engine: index,
                        actor,
                        remaining_ms: clock[side],
                        elapsed_ms: elapsed,
                    });
                    break Stop::ClockExhausted;
                }
                Err(e) => {
                    failure = Some(e);
                    break Stop::Failure;
                }
            }
        }
    };
    for (index, engine) in engines.iter_mut().enumerate() {
        let side = usize::from(config.first != index);
        if let Some(notice) = G::gameover(&game, side) {
            let _ = host.send(engine, "gameover", &notice);
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
    Ok(Completed {
        game,
        stop,
        identities,
        engine_ids,
        failure,
        exhaustion,
        turns,
        traffic: host.traffic,
        traffic_complete: host.complete,
        processes,
        elapsed_ns: host.start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64,
    })
}
pub(crate) fn replay<G: MatchNative>(
    c: &Settings<'_, G>,
    game: &mut G::Game,
    turns: &[ClockedTurn<G::Side>],
    traffic: &[Traffic],
    traffic_complete: bool,
) -> Result<[u64; 2], RunnerError> {
    let mut clock = c.clock;
    for turn in turns {
        if G::end(game).is_some() {
            return Err(RunnerError(
                "turn after native conclusion or pending".into(),
            ));
        }
        let actor = G::actor(game);
        let side = G::side(actor);
        if turn.engine != c.engine_for(actor)
            || turn.actor != actor
            || turn.version != G::events(game)
            || turn.context_sha256 != protocol::digest::<G>(game)?
            || turn.position != G::position_for(c.wire[turn.engine], game)
            || turn.clock_before_ms != clock
            || turn.go != G::go(c.wire[turn.engine], clock, c.increment, side)
            || turn.elapsed_ms > clock[side].saturating_add(c.grace_ms)
        {
            return Err(RunnerError("turn context or clock ledger mismatch".into()));
        }
        let logged = |direction: &str, text: &str| {
            traffic.iter().any(|t| {
                t.engine == turn.engine
                    && t.version == turn.version
                    && t.direction == direction
                    && t.text == text
            })
        };
        if traffic_complete
            && !(logged("command", &turn.go) && logged("response", &escaped(turn.reply.as_bytes())))
        {
            return Err(RunnerError("turn lacks command/response evidence".into()));
        }
        clock[side] = clock[side].saturating_sub(turn.elapsed_ms) + c.increment[side];
        G::admit(game, c.wire[turn.engine], &turn.reply).map_err(|e| RunnerError(e.2))?;
    }
    Ok(clock)
}
