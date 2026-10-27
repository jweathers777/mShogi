//! Bounded native canonical records. Redundant events are checked by native replay.
use super::game::CommandEntry;
use super::{
    ClaimPolicy, Conclusion, Game, GameError, GameEvent, GameRequest, MAX_GAME_COMMANDS,
    MAX_GAME_EVENTS, NoWinAssessment, PROFILE_EDITION, PROFILE_ID, PendingAdjudication,
    PositionRoot,
};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::{error::Error, fmt};
/// Actual encoded native payload bound, independent of later shared/desktop envelopes.
pub const MAX_GAME_RECORD_BYTES: usize = 1024 * 1024;
/// Native record schema; profile, governing edition and transport remain distinct.
pub const GAME_RECORD_SCHEMA_VERSION: u32 = 1;
/// Native game record validated against every authoritative command/event prefix.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRecord {
    json: String,
}
/// Malformed, unsupported, resource-limited or forged data is never a game outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordError {
    /// Encoded bytes, commands or events exceed the native bound.
    Capacity,
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
    format: String,
    schema_version: u32,
    game: String,
    profile: String,
    edition: String,
    mode: String,
    no_win_goal: String,
    no_win_mode: String,
    fast_witness_limit: u8,
    claim_policy: ClaimPolicy,
    root: Box<RawValue>,
    commands: Vec<CommandEntry>,
    events: Vec<GameEvent>,
    current_position: Box<RawValue>,
    conclusion: Option<Conclusion>,
    pending: Option<PendingAdjudication>,
    no_win_assessment: NoWinAssessment,
}
impl WireRecord {
    fn from_game(game: &Game) -> Self {
        Self {
            format: "mshogi.xiangqi.game".into(),
            schema_version: GAME_RECORD_SCHEMA_VERSION,
            game: "xiangqi".into(),
            profile: PROFILE_ID.into(),
            edition: PROFILE_EDITION.into(),
            mode: "untimed".into(),
            no_win_goal: super::NATURAL_WIN_GOAL.into(),
            no_win_mode: "sound_fast_only".into(),
            fast_witness_limit: 1,
            claim_policy: game.policy,
            root: RawValue::from_string(game.root.to_json()).expect("native root JSON"),
            commands: game.commands.clone(),
            events: game.events.clone(),
            current_position: RawValue::from_string(
                PositionRoot::analysis(game.position.clone()).to_json(),
            )
            .expect("native position JSON"),
            conclusion: game.conclusion,
            pending: game.pending_adjudication().cloned(),
            no_win_assessment: game.no_win_assessment(),
        }
    }
}
impl GameRecord {
    pub(super) fn encoded_candidate(game: &Game) -> Result<String, RecordError> {
        if game.commands.len() > MAX_GAME_COMMANDS || game.events.len() > MAX_GAME_EVENTS {
            return Err(RecordError::Capacity);
        }
        let json =
            serde_json::to_string(&WireRecord::from_game(game)).map_err(|_| RecordError::Json)?;
        if json.len() > MAX_GAME_RECORD_BYTES {
            return Err(RecordError::Capacity);
        }
        Ok(json)
    }
    /// Export the complete currently accepted native commands/events, including audits.
    pub fn from_game(game: &Game) -> Result<Self, RecordError> {
        Ok(Self {
            json: Self::encoded_candidate(game)?,
        })
    }
    /// Stable compact JSON, independent of runtime request identities.
    pub fn to_json(&self) -> &str {
        &self.json
    }
    /// Strict direct decoding precedes every normalization. Raw native roots retain
    /// duplicate fields for their own parser; no map is used ahead of that boundary.
    pub fn from_json(json: &str) -> Result<Self, RecordError> {
        let game = Self::replay(json)?;
        Self::from_game(&game)
    }
    fn replay(json: &str) -> Result<Game, RecordError> {
        if json.len() > MAX_GAME_RECORD_BYTES {
            return Err(RecordError::Capacity);
        }
        let wire: WireRecord = serde_json::from_str(json).map_err(|_| RecordError::Json)?;
        if wire.format != "mshogi.xiangqi.game"
            || wire.schema_version != GAME_RECORD_SCHEMA_VERSION
            || wire.game != "xiangqi"
            || wire.profile != PROFILE_ID
            || wire.edition != PROFILE_EDITION
            || wire.mode != "untimed"
            || wire.no_win_goal != super::NATURAL_WIN_GOAL
            || wire.no_win_mode != "sound_fast_only"
            || wire.fast_witness_limit != 1
        {
            return Err(RecordError::Unsupported);
        }
        if wire.commands.len() > MAX_GAME_COMMANDS || wire.events.len() > MAX_GAME_EVENTS {
            return Err(RecordError::Capacity);
        }
        let root = PositionRoot::from_json(wire.root.get()).map_err(|_| RecordError::Root)?;
        // Also parse the redundant native payload before comparing: duplicates
        // cannot disappear when JSON object ordering/whitespace is normalized.
        PositionRoot::from_json(wire.current_position.get()).map_err(|_| RecordError::Derived)?;
        let mut game = Game::from_root(root, wire.claim_policy).map_err(|_| RecordError::Root)?;
        for (i, entry) in wire.commands.iter().enumerate() {
            let submission = game
                .submit(GameRequest {
                    expected: game.context(),
                    actor: entry.actor,
                    command: entry.command,
                })
                .map_err(|error| match error {
                    GameError::Invariant => RecordError::Invariant(i),
                    GameError::Capacity => RecordError::Capacity,
                    _ => RecordError::Command(i),
                })?;
            if submission.result != entry.result || game.events.len() != entry.event_end {
                return Err(RecordError::Command(i));
            }
            if game.events != wire.events[..game.events.len().min(wire.events.len())] {
                return Err(RecordError::Derived);
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
        Ok(game)
    }
    /// Reconstruct with fresh runtime identity; observations are never pending work.
    pub fn to_game(&self) -> Result<Game, RecordError> {
        Self::replay(&self.json)
    }
}
