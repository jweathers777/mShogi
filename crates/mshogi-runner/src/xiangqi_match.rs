//! Persistent, clocked UCI Xiangqi games for strength studies.
//!
//! Each engine keeps one process for the whole game. The controller owns a
//! Fischer clock per side: it sends `go wtime btime winc binc` (Red is white),
//! measures the wall time from the written `go` to the `bestmove` line, charges
//! it to the mover and then adds the increment. Game admission with
//! `auto_valid_claims` decides every reply and every result. A pending
//! adjudication is an unscored stop, and so is clock exhaustion: the accepted
//! profile has no time-loss rule. Every report replays against the
//! rules, and its clock ledger is recomputed from the recorded elapsed times.
use crate::protocol_match::MatchInternal;
use crate::xiangqi::Adapter;
use crate::{
    FischerClock as Clock,
    transport::Dialect,
    xiangqi::{Opening, position, reply, submit},
    *,
};
use mshogi_core::xiangqi::*;
use serde::{Deserialize, Serialize};

/// Engine protocol.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    /// UCI, Pikafish dialect (files a-i, ranks 0-9).
    #[default]
    Uci,
    /// UCCI (ElephantEye): `ucci`, `setoption NAME VALUE`, `go time T increment I`.
    Ucci,
}

/// How one engine's commands and replies are written on the wire. The
/// controller translates; rules still decide every move.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Wire {
    /// Engine protocol.
    #[serde(default)]
    pub protocol: Protocol,
    /// Added to every rank number of a move on the wire (1 for
    /// Fairy-Stockfish, whose Xiangqi ranks are 1-10).
    #[serde(default)]
    pub rank_offset: u8,
    /// Fixed node count per move instead of clock commands; the clock is
    /// still charged.
    #[serde(default)]
    pub nodes: Option<u64>,
}

/// One clocked game between two UCI Xiangqi engines.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchConfig {
    /// Config version, one.
    pub schema_version: u32,
    /// Exact rules profile.
    pub profile: String,
    /// Experiment label.
    pub id: String,
    /// Two engines with their options.
    pub engines: [EngineConfig; 2],
    /// Known-history root and prefix.
    pub opening: Opening,
    /// Engine index (0 or 1) that plays Red.
    pub red: usize,
    /// Wire format of engine 0 and engine 1 (default: Pikafish UCI, clocked).
    #[serde(default)]
    pub wire: [Wire; 2],
    /// Fischer clock of both sides.
    pub clock: Clock,
    /// Per-engine clocks (time odds) for engine 0 and engine 1; replaces
    /// `clock` when present.
    #[serde(default)]
    pub odds: Option<[Clock; 2]>,
    /// Allowance beyond the remaining time for transport, in milliseconds.
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
        let (clock, increment) = self.side_clocks();
        crate::protocol_match::Settings {
            engines: &self.engines,
            watchdogs: self.watchdogs,
            io: self.io,
            max_game_ms: self.max_game_ms,
            grace_ms: self.grace_ms,
            first: self.red,
            clock,
            increment,
            wire: self.wire,
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
        if self.schema_version != 1 || self.profile != PROFILE_ID || self.red > 1 {
            return Err(RunnerError("unsupported version/profile/assignment".into()));
        }
        if self.id.is_empty()
            || self.id.len() > 128
            || [self.clock]
                .iter()
                .chain(self.odds.iter().flatten())
                .any(|c| !c.valid())
            || self.grace_ms > 5_000
            || !(1..=21_600_000).contains(&self.max_game_ms)
        {
            return Err(RunnerError("unsupported clock or experiment bounds".into()));
        }
        if self.opening.fen.as_ref().is_some_and(|s| s.len() > 4096)
            || self.opening.canonical.is_some()
        {
            return Err(RunnerError("invalid opening bounds".into()));
        }
        let game = self.opening.game()?;
        if game.conclusion().is_some() || game.pending_adjudication().is_some() {
            return Err(RunnerError(
                "opening is already concluded or pending".into(),
            ));
        }
        if self
            .wire
            .iter()
            .any(|w| w.rank_offset > 1 || w.nodes.is_some_and(|n| !(1..=100_000_000).contains(&n)))
        {
            return Err(RunnerError("unsupported wire settings".into()));
        }
        crate::config::validate_processes(&self.engines, self.watchdogs, self.io)?;
        Ok(())
    }
    /// Starting time and increment of Red and Black.
    fn side_clocks(&self) -> ([u64; 2], [u64; 2]) {
        let of = |side: Side| {
            self.odds.map_or(self.clock, |o| {
                o[crate::protocol_match::engine_for(self.red, side_index(side))]
            })
        };
        let (red, black) = (of(Side::Red), of(Side::Black));
        (
            [red.base_ms, black.base_ms],
            [red.increment_ms, black.increment_ms],
        )
    }
}

/// Accepted reply and side clock ledger.
pub type ClockedTurn = crate::protocol_match::ClockedTurn<Side>;
/// Unscored clock exhaustion evidence.
pub type Exhaustion = crate::protocol_match::Exhaustion<Side>;

pub use crate::protocol_match::MatchStop;
/// Complete clocked game evidence, with the recorded game-specific schema.
pub type MatchReport = crate::protocol_match::MatchReport<MatchConfig, Side, Conclusion>;

fn go(clock: [u64; 2], increment: [u64; 2]) -> String {
    format!(
        "go wtime {} btime {} winc {} binc {}",
        clock[0], clock[1], increment[0], increment[1]
    )
}

/// Search command of one engine on the wire.
fn go_for(wire: Wire, clock: [u64; 2], increment: [u64; 2], side: usize) -> String {
    match (wire.nodes, wire.protocol) {
        (Some(nodes), _) => format!("go nodes {nodes}"),
        (None, Protocol::Uci) => go(clock, increment),
        (None, Protocol::Ucci) => format!("go time {} increment {}", clock[side], increment[side]),
    }
}

/// Shift the rank numbers of a move token such as `h2e2` by `delta`.
fn shift_move(token: &str, delta: i32) -> Option<String> {
    let bytes = token.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    for _ in 0..2 {
        let file = *bytes.get(i)?;
        if !(b'a'..=b'i').contains(&file) {
            return None;
        }
        i += 1;
        let start = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        let rank: i32 = token.get(start..i)?.parse().ok()?;
        let rank = rank + delta;
        if !(0..=10).contains(&rank) {
            return None;
        }
        out.push(file as char);
        out.push_str(&rank.to_string());
    }
    (i == bytes.len()).then_some(out)
}

/// Position command of one engine on the wire.
fn position_for(wire: Wire, game: &Game) -> String {
    let mut text = position(game);
    if wire.protocol == Protocol::Ucci && text.starts_with("position startpos") {
        let fen = PositionRoot::standard()
            .position()
            .to_fen()
            .expect("standard root");
        text = text.replacen("position startpos", &format!("position fen {fen}"), 1);
    }
    if wire.rank_offset == 0 {
        return text;
    }
    match text.split_once(" moves ") {
        Some((head, moves)) => {
            let shifted: Vec<String> = moves
                .split(' ')
                .map(|m| shift_move(m, i32::from(wire.rank_offset)).expect("action"))
                .collect();
            format!("{head} moves {}", shifted.join(" "))
        }
        None => text,
    }
}

/// Game command of a reply line on the wire. UCCI may end a reply with
/// `resign` (a resignation; the move is not played) or `draw` (a draw
/// offer, which this board-only protocol cannot carry: ignored, the move is
/// played).
fn reply_for(wire: Wire, line: &str) -> Result<GameCommand, String> {
    let line = if wire.protocol == Protocol::Ucci {
        match line.trim_end().rsplit_once(' ') {
            Some((_, "resign")) => return Ok(GameCommand::Resign),
            Some((head, "draw")) => head.to_string(),
            _ => line.to_string(),
        }
    } else {
        line.to_string()
    };
    let line = line.as_str();
    if wire.rank_offset == 0 {
        return reply(line);
    }
    let words: Vec<String> = line
        .split_ascii_whitespace()
        .enumerate()
        .map(|(i, w)| {
            if i == 0 || w == "ponder" || w == "resign" {
                Some(w.to_string())
            } else {
                // Fail closed: an untranslatable move token is malformed.
                shift_move(w, -i32::from(wire.rank_offset))
            }
        })
        .collect::<Option<Vec<_>>>()
        .ok_or("move token outside the engine's rank range")?;
    reply(&words.join(" "))
}

fn dialect(wire: Wire) -> Dialect {
    match wire.protocol {
        Protocol::Uci => Dialect::Uci,
        Protocol::Ucci => Dialect::Ucci,
    }
}

fn side_index(side: Side) -> usize {
    usize::from(side == Side::Black)
}

impl MatchReport {
    /// Decode and verify the game history and clock ledger.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        let report: Self = common::decode(text, common::REPORT_BYTES)?;
        report.replay()?;
        Ok(report)
    }
    /// Encode bounded evidence.
    pub fn to_json(&self) -> Result<String, RunnerError> {
        common::encode_report(self)
    }
    /// Replay the game history and verify stop, score and clock receipts.
    pub fn replay(&self) -> Result<Game, RunnerError> {
        self.config.validate()?;
        crate::protocol_match::replay_report(
            self,
            self.config.settings(),
            self.config.opening.game()?,
        )
    }
}
/// Play and verify one clocked game.
pub fn run(config: &MatchConfig) -> Result<MatchReport, RunnerError> {
    config.validate()?;
    let report =
        crate::protocol_match::run_report(config, config.settings(), config.opening.game()?)?;
    report.replay()?;
    Ok(report)
}

impl MatchInternal for Adapter {
    type Wire = Wire;
    fn dialect(wire: Wire) -> Dialect {
        dialect(wire)
    }
    fn position_for(wire: Wire, game: &Game) -> String {
        position_for(wire, game)
    }
    fn go(wire: Wire, clock: [u64; 2], increment: [u64; 2], side: usize) -> String {
        go_for(wire, clock, increment, side)
    }
    fn admit(
        game: &mut Game,
        wire: Wire,
        text: &str,
    ) -> Result<(), (FailureKind, &'static str, String)> {
        let command =
            reply_for(wire, text).map_err(|e| (FailureKind::MalformedReply, "search", e))?;
        let kind = if matches!(command, GameCommand::Play { .. }) {
            FailureKind::IllegalAction
        } else {
            FailureKind::GameAdmission
        };
        submit(game, command).map_err(|e| (kind, "admission", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rank_offsets_translate_moves_both_ways() {
        assert_eq!(shift_move("h2e2", 1).unwrap(), "h3e3");
        assert_eq!(shift_move("i9h9", 1).unwrap(), "i10h10");
        assert_eq!(shift_move("i10h10", -1).unwrap(), "i9h9");
        assert!(shift_move("j1a1", 1).is_none());
        assert!(shift_move("a9a9x", 1).is_none());
        let fairy = Wire {
            rank_offset: 1,
            ..Wire::default()
        };
        assert_eq!(
            reply_for(fairy, "bestmove h1g3 ponder i10h10").unwrap(),
            reply("bestmove h0g2 ponder i9h9").unwrap()
        );
        // Rank 0 does not exist for Fairy-Stockfish: rejected, not misread.
        assert!(reply_for(fairy, "bestmove a0a1").is_err());
    }
    #[test]
    fn wire_commands_follow_each_engine_protocol() {
        let mut game = Game::standard(ClaimPolicy::AutoValidClaims).unwrap();
        submit(
            &mut game,
            GameCommand::Play {
                action: "h2e2".parse().unwrap(),
            },
        )
        .unwrap();
        let ucci = Wire {
            protocol: Protocol::Ucci,
            ..Wire::default()
        };
        assert!(position_for(ucci, &game).starts_with("position fen rnbakabnr/9/1c5c1/"));
        assert!(position_for(ucci, &game).ends_with(" moves h2e2"));
        let fairy = Wire {
            rank_offset: 1,
            ..Wire::default()
        };
        assert_eq!(position_for(fairy, &game), "position startpos moves h3e3");
        assert_eq!(
            go_for(ucci, [3000, 2000], [100, 100], 1),
            "go time 2000 increment 100"
        );
        assert_eq!(
            go_for(Wire::default(), [3000, 2000], [100, 100], 1),
            "go wtime 3000 btime 2000 winc 100 binc 100"
        );
        let nodes = Wire {
            nodes: Some(5000),
            ..Wire::default()
        };
        assert_eq!(go_for(nodes, [3000, 2000], [100, 100], 0), "go nodes 5000");
    }
    #[test]
    fn ucci_resign_and_draw_suffixes_are_explicit() {
        let ucci = Wire {
            protocol: Protocol::Ucci,
            ..Wire::default()
        };
        assert_eq!(
            reply_for(ucci, "bestmove b9a9 ponder a8a9 resign").unwrap(),
            GameCommand::Resign
        );
        assert_eq!(
            reply_for(ucci, "bestmove c9a7 ponder h3i5 draw").unwrap(),
            reply("bestmove c9a7 ponder h3i5").unwrap()
        );
        // A UCI engine has no such suffix.
        assert!(reply_for(Wire::default(), "bestmove b9a9 resign").is_err());
    }
}
