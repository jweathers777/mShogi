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
    shogi::{FailureKind, Opening, position, reply, submit},
    transport::Dialect,
    *,
};
use mshogi_core::shogi::*;
use serde::{Deserialize, Serialize};

pub use crate::FischerClock as Clock;
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
        let config: Self = crate::common::decode_config(text)?;
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
            || !self.clock.valid()
            || self.grace_ms > 5_000
            || !(1..=21_600_000).contains(&self.max_game_ms)
        {
            return Err(RunnerError("unsupported clock or experiment bounds".into()));
        }
        if self.opening.sfen.as_ref().is_some_and(|s| s.len() > 4096) {
            return Err(RunnerError("invalid opening bounds".into()));
        }
        let game = self.opening.game()?;
        if game.conclusion().is_some() {
            return Err(RunnerError("opening is already concluded".into()));
        }
        crate::config::validate_processes(&self.engines, self.watchdogs, self.io)?;
        Ok(())
    }
}

/// Accepted reply and native-side clock ledger.
pub type ClockedTurn = crate::protocol_match::ClockedTurn<Side>;
/// Unscored clock exhaustion evidence.
pub type Exhaustion = crate::protocol_match::Exhaustion<Side>;

pub use crate::protocol_match::MatchStop;
/// Complete native clocked game evidence, with the recorded game-specific schema.
pub type MatchReport = crate::protocol_match::MatchReport<MatchConfig, Side, Conclusion>;

fn go(clock: [u64; 2], increment: u64) -> String {
    format!(
        "go btime {} wtime {} binc {increment} winc {increment}",
        clock[0], clock[1]
    )
}

impl MatchReport {
    /// Decode and verify the native history and clock ledger.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        let report: Self = common::decode(text, common::REPORT_BYTES)?;
        report.replay()?;
        Ok(report)
    }
    /// Encode bounded evidence.
    pub fn to_json(&self) -> Result<String, RunnerError> {
        common::encode_report(self)
    }
    /// Replay the native history and verify stop, score and clock receipts.
    pub fn replay(&self) -> Result<Game, RunnerError> {
        self.config.validate()?;
        crate::protocol_match::replay_report(
            self,
            self.config.settings(),
            self.config.opening.game()?,
        )
    }
}
/// Play and verify one clocked native game.
pub fn run(config: &MatchConfig) -> Result<MatchReport, RunnerError> {
    config.validate()?;
    let report =
        crate::protocol_match::run_report(config, config.settings(), config.opening.game()?)?;
    report.replay()?;
    Ok(report)
}

impl MatchNative for Adapter {
    type Wire = ();
    fn dialect(_: ()) -> Dialect {
        Dialect::Usi
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
