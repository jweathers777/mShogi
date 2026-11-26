//! Native canonical records. Native replay derives all event results.
use super::{
    ClaimPolicy, Conclusion, Game, GameError, GameRequest, NoWinAssessment, PositionRoot, Side,
};
use crate::record::RecordedCommand;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::{error::Error, fmt};
/// Native record schema; profile, governing edition and transport remain distinct.
pub const GAME_RECORD_SCHEMA_VERSION: u32 = 2;
/// Native game record validated against every authoritative command/event prefix.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRecord {
    json: String,
    // Derived once by the authoritative constructor/replay, never trusted wire data.
    event_count: usize,
    known_board_actions: usize,
    side_to_move: Side,
    conclusion: Option<Conclusion>,
    assessment: NoWinAssessment,
}
/// Malformed, unsupported, resource-limited or forged data is never a game outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordError {
    /// Duplicate/unknown/missing/malformed field or unsupported JSON value.
    Json,
    /// Wrong game/profile/edition/schema/mode or verification goal.
    Unsupported,
    /// Native root syntax, provenance or static domain is invalid.
    Root,
    /// A command failed authoritative admission or disagreed with its result.
    Command(usize),
    /// Native replay encountered an internal failure, not a rule outcome.
    Invariant(usize),
    /// Derived events/current state/metadata do not match native replay.
    Derived,
}
impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid xiangqi native record: {self:?}")
    }
}
impl Error for RecordError {}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireRecord {
    schema_version: u32,
    claim_policy: ClaimPolicy,
    root: Box<RawValue>,
    commands: Vec<RecordedCommand<Side, super::GameCommand>>,
}
impl WireRecord {
    fn from_game(game: &Game) -> Self {
        Self {
            schema_version: GAME_RECORD_SCHEMA_VERSION,
            claim_policy: game.policy,
            root: RawValue::from_string(game.root.to_json()).expect("native root JSON"),
            commands: game
                .commands
                .iter()
                .map(|entry| RecordedCommand {
                    actor: entry.actor,
                    command: entry.command,
                })
                .collect(),
        }
    }
}
impl GameRecord {
    /// Export the complete currently accepted native commands/events, including audits.
    pub fn from_game(game: &Game) -> Result<Self, RecordError> {
        Ok(Self {
            json: serde_json::to_string(&WireRecord::from_game(game))
                .map_err(|_| RecordError::Json)?,
            event_count: game.events().len(),
            known_board_actions: game
                .events()
                .iter()
                .filter(|event| matches!(event.event, super::EventKind::BoardAction { .. }))
                .count(),
            side_to_move: game.position().side_to_move(),
            conclusion: game.conclusion(),
            assessment: game.no_win_assessment(),
        })
    }
    /// Exact complete event count, including same-ply warnings and claim audits.
    pub const fn event_count(&self) -> usize {
        self.event_count
    }
    /// Known board actions; an imported declared counter never adds missing history.
    pub const fn known_board_actions(&self) -> usize {
        self.known_board_actions
    }
    /// Native current turn, not a winner or an execution capability.
    pub const fn side_to_move(&self) -> Side {
        self.side_to_move
    }
    /// Already replay-validated native conclusion; no replay here.
    pub const fn conclusion(&self) -> Option<Conclusion> {
        self.conclusion
    }
    /// Already replay-validated sound fast no-win assessment.
    pub fn no_win_assessment(&self) -> &NoWinAssessment {
        &self.assessment
    }
    /// Stable compact JSON, independent of runtime request identities.
    pub fn to_json(&self) -> &str {
        &self.json
    }
    /// Strict direct decoding precedes every normalization. Raw native roots retain
    /// duplicate fields for their own parser; no map is used ahead of that boundary.
    pub fn from_json(json: &str) -> Result<Self, RecordError> {
        Self::from_json_with_game(json).map(|(record, _)| record)
    }
    /// Parse and validate once, retaining the reconstructed game for the caller.
    pub fn from_json_with_game(json: &str) -> Result<(Self, Game), RecordError> {
        let game = Self::replay(json)?;
        Ok((Self::from_game(&game)?, game))
    }
    fn replay(json: &str) -> Result<Game, RecordError> {
        Self::replay_with_undo(json, false).map(|(game, _)| game)
    }
    fn replay_with_undo(
        json: &str,
        retain_undo: bool,
    ) -> Result<(Game, Vec<super::GameUndo>), RecordError> {
        crate::record::reject_duplicates(json).map_err(|_| RecordError::Json)?;
        let wire: WireRecord = serde_json::from_str(json).map_err(|_| RecordError::Json)?;
        if wire.schema_version != GAME_RECORD_SCHEMA_VERSION {
            return Err(RecordError::Unsupported);
        }

        let root = PositionRoot::from_json(wire.root.get()).map_err(|_| RecordError::Root)?;
        let mut game = Game::from_root(root, wire.claim_policy).map_err(|_| RecordError::Root)?;
        let mut undo = Vec::with_capacity(if retain_undo { wire.commands.len() } else { 0 });
        for (i, entry) in wire.commands.iter().enumerate() {
            let submission = game
                .submit(GameRequest {
                    expected: game.context(),
                    actor: entry.actor,
                    command: entry.command,
                })
                .map_err(|error| match error {
                    GameError::Invariant => RecordError::Invariant(i),
                    _ => RecordError::Command(i),
                })?;
            if retain_undo {
                undo.push(submission.undo);
            }
        }
        // Strict typed and native parsers above have already rejected duplicates.
        // This final presence/equality check additionally rejects *missing* nullable
        // fields instead of letting Serde's Option default erase their absence.
        // It allows harmless JSON whitespace and object-key ordering differences.
        let actual: serde_json::Value =
            serde_json::from_str(json).map_err(|_| RecordError::Json)?;
        let expected =
            serde_json::to_value(WireRecord::from_game(&game)).map_err(|_| RecordError::Json)?;
        if actual != expected {
            return Err(RecordError::Derived);
        }
        Ok((game, undo))
    }
    /// Authoritative bounded replay with fresh runtime identity and LIFO command
    /// undo handles. Consumers must not reconstruct commands from derived events.
    /// Root automatic conclusions have no command handle.
    pub fn to_game_with_undo(&self) -> Result<(Game, Vec<super::GameUndo>), RecordError> {
        Self::replay_with_undo(&self.json, true)
    }
    /// Reconstruct with fresh runtime identity; observations are never pending work.
    pub fn to_game(&self) -> Result<Game, RecordError> {
        Self::replay(&self.json)
    }
}

impl Serialize for GameRecord {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        crate::record::serialize_json(&self.json, serializer)
    }
}
