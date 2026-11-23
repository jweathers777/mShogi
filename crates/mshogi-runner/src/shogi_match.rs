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
use crate::protocol_match::MatchNative;
use crate::shogi::Adapter;
use crate::{
    clocked::Dialect,
    shogi::{FailureKind, Opening, position, reply, submit},
    *,
};
use mshogi_core::{native::CanonicalRecord, shogi::*};
use serde::{Deserialize, Serialize};

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
    fn settings(&self) -> crate::protocol_match::Settings<'_, Adapter> {
        let clock = [self.clock.base_ms; 2];
        let increment = [self.clock.increment_ms; 2];
        crate::protocol_match::Settings {
            engines: &self.engines,
            watchdogs: self.watchdogs,
            io: self.io,
            max_game_ms: self.max_game_ms,
            grace_ms: self.grace_ms,
            first: self.sente,
            clock,
            increment,
            wire: [(); 2],
        }
    }

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

/// Accepted reply and native-side clock ledger.
pub type ClockedTurn = crate::protocol_match::ClockedTurn<Side>;
/// Unscored clock exhaustion evidence.
pub type Exhaustion = crate::protocol_match::Exhaustion<Side>;

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

/// Complete native clocked game evidence, with the recorded game-specific schema.
pub type MatchReport = crate::protocol_match::MatchReport<MatchConfig, Side, Conclusion, MatchStop>;

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
        let clock = crate::protocol_match::replay::<Adapter>(
            &c.settings(),
            &mut game,
            &self.turns,
            &self.traffic,
            self.traffic_complete,
        )?;
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
    let completed =
        crate::protocol_match::run::<Adapter>(config.settings(), config.opening.game()?)?;
    let stop = match completed.stop {
        crate::protocol_match::Stop::Native(crate::protocol::End::NativeConclusion) => {
            MatchStop::NativeConclusion
        }
        crate::protocol_match::Stop::ClockExhausted => MatchStop::ClockExhausted,
        crate::protocol_match::Stop::TimeLimit => MatchStop::TimeLimit,
        crate::protocol_match::Stop::Failure => MatchStop::Failure,
        crate::protocol_match::Stop::Native(_) => unreachable!("native terminal state"),
    };
    let conclusion = completed.game.conclusion();
    let engine_score = if stop == MatchStop::NativeConclusion {
        conclusion.map(|conclusion| {
            let points = conclusion.score_half_points();
            if config.sente == 0 {
                points
            } else {
                [points[1], points[0]]
            }
        })
    } else {
        None
    };
    let report = completed.report(config.clone(), conclusion, engine_score, stop)?;
    report.replay()?;
    Ok(report)
}

impl MatchNative for Adapter {
    type Wire = ();
    fn dialect(_: ()) -> Dialect {
        Dialect {
            hello: "usi",
            ok: "usiok",
            newgame: "usinewgame",
            ucci_options: false,
        }
    }
    fn position_for(_: (), game: &Game) -> String {
        position(game)
    }
    fn go(_: (), clock: [u64; 2], increment: [u64; 2], _: usize) -> String {
        go(clock, increment[0])
    }
    fn admit(
        game: &mut Game,
        _: (),
        text: &str,
    ) -> Result<(), (FailureKind, &'static str, String)> {
        let command = reply(text).map_err(|e| (FailureKind::MalformedReply, "search", e))?;
        let kind = if matches!(command, GameCommand::Play(_)) {
            FailureKind::IllegalAction
        } else {
            FailureKind::Native
        };
        submit(game, command).map_err(|e| (kind, "admission", e))
    }
    fn gameover(game: &Game, side: usize) -> Option<String> {
        game.conclusion().map(|conclusion| {
            let notice = match conclusion {
                Conclusion::Draw(_) => "draw",
                Conclusion::Win { winner, .. } if usize::from(winner == Side::Gote) == side => {
                    "win"
                }
                _ => "lose",
            };
            format!("gameover {notice}")
        })
    }
}
