//! Strict native Chu records; authoritative replay, not a desktop document format.
use super::game::NativeRoot;
use super::*;
use crate::record::RecordedCommand;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::{error::Error, fmt};
/// Native record version, distinct from immutable rules edition and outer transport.
pub const GAME_RECORD_SCHEMA_VERSION: u32 = 3;
/// Native encoded payload bound, leaving space in the existing shared envelope.
pub const MAX_GAME_RECORD_BYTES: usize = 512 * 1024;
/// Exact validated canonical record, excluding runtime/undo handles.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRecord {
    json: String,
    events: usize,
    board_plies: usize,
    side: Side,
    conclusion: Option<Conclusion>,
    // Derived only from authoritative native admission/replay; never serialized.
    reconstruction_work: u64,
}
/// Bounded syntax/identity/replay refusal, never a game conclusion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordError {
    /// Input/event/replay resource limit.
    Capacity,
    /// Duplicate/unknown/missing/malformed input.
    Json,
    /// Conflicting or unsupported game/profile/edition/schema/mode.
    Identity,
    /// Structurally invalid root or invented provenance.
    Root,
    /// Native command refused at zero-based event index.
    Command(usize),
    /// Replay-derived facts differ.
    Derived,
    /// Missing historical prefix.
    Prefix,
    /// Caller explicitly stopped reconstruction.
    Stopped,
}
impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chu native record refused: {self:?}")
    }
}
impl Error for RecordError {}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireRecord {
    profile: String,
    edition: String,
    schema_version: u32,
    root: NativeRoot,
    events: Vec<RecordedCommand<Side, GameCommand>>,
}
impl WireRecord {
    fn from_game(g: &Game) -> Self {
        Self {
            profile: PROFILE_ID.into(),
            edition: PROFILE_EDITION.into(),
            schema_version: GAME_RECORD_SCHEMA_VERSION,
            root: g.native_root().clone(),
            events: g
                .events()
                .iter()
                .map(|event| RecordedCommand {
                    actor: event.actor,
                    command: event.command.clone(),
                })
                .collect(),
        }
    }
}
pub(super) fn encode(g: &Game) -> Result<String, RecordError> {
    serde_json::to_string(&WireRecord::from_game(g)).map_err(|_| RecordError::Json)
}
impl GameRecord {
    /// Encode all accepted native semantics without truncation.
    pub fn from_game(g: &Game) -> Result<Self, RecordError> {
        let json = encode(g)?;
        if json.len() > MAX_GAME_RECORD_BYTES || g.events().len() > MAX_GAME_EVENTS {
            return Err(RecordError::Capacity);
        }
        Ok(Self {
            json,
            events: g.events().len(),
            board_plies: g.snapshot().board_ply,
            side: g.position().side_to_move(),
            conclusion: g.conclusion(),
            reconstruction_work: g.replay_work(),
        })
    }
    /// Exact native bytes. No pending runtime operation is serialized.
    pub fn to_json(&self) -> &str {
        &self.json
    }
    /// Alias for borrowing canonical native bytes.
    pub fn as_json(&self) -> &str {
        &self.json
    }
    /// Accepted native events, not board moves.
    pub fn event_count(&self) -> usize {
        self.events
    }
    /// Complete accepted board actions, excluding declared imported counter.
    pub fn known_board_actions(&self) -> usize {
        self.board_plies
    }
    /// Replay-validated native board turn.
    pub fn side_to_move(&self) -> Side {
        self.side
    }
    /// Actual replay-validated result only.
    pub fn conclusion(&self) -> Option<Conclusion> {
        self.conclusion
    }
    /// Authoritative deterministic work of reconstructing this validated record.
    /// This cache is native-derived and is absent from serialized input.
    pub fn reconstruction_work(&self) -> u64 {
        self.reconstruction_work
    }
    /// Strict duplicate-preserving parsing followed by authoritative replay.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        Self::from_json_with_stop(text, &mut || false)
    }
    /// Admission with cooperative cancellation between replayed events.
    pub fn from_json_with_stop(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self, RecordError> {
        Self::from_json_with_game(text, stop).map(|(record, _)| record)
    }
    /// Parse and validate once, retaining the reconstructed game for the caller.
    pub fn from_json_with_game(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(Self, Game), RecordError> {
        let game = Self::replay(text, stop)?.0;
        Ok((Self::from_game(&game)?, game))
    }
    /// Reconstruct canonical semantics with fresh runtime authority.
    pub fn to_game(&self) -> Result<Game, RecordError> {
        self.to_game_with_stop(&mut || false)
    }
    /// Reconstruct with cooperative cancellation between replayed events.
    pub fn to_game_with_stop(&self, stop: &mut impl FnMut() -> bool) -> Result<Game, RecordError> {
        Self::replay(&self.json, stop).map(|v| v.0)
    }
    /// Native consumer reconstruction including actual LIFO reversal handles.
    pub fn to_game_with_undo(&self) -> Result<(Game, Vec<GameUndo>), RecordError> {
        Self::replay(&self.json, &mut || false)
    }
    fn replay(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(Game, Vec<GameUndo>), RecordError> {
        strict(text, MAX_GAME_RECORD_BYTES)?;
        let w: WireRecord = serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        if w.profile != PROFILE_ID
            || w.edition != PROFILE_EDITION
            || w.schema_version != GAME_RECORD_SCHEMA_VERSION
        {
            return Err(RecordError::Identity);
        }
        if w.events.len() > MAX_GAME_EVENTS {
            return Err(RecordError::Capacity);
        }
        let mut game = Game::from_native_root(w.root).map_err(|e| {
            if matches!(e, GameError::Capacity(_)) {
                RecordError::Capacity
            } else {
                RecordError::Root
            }
        })?;
        let mut undo = Vec::with_capacity(w.events.len());
        for (index, event) in w.events.iter().enumerate() {
            if stop() {
                return Err(RecordError::Stopped);
            }
            undo.push(
                game.submit(GameRequest {
                    expected: game.context(),
                    actor: event.actor,
                    command: event.command.clone(),
                })
                .map_err(|e| {
                    if matches!(e, GameError::Capacity(_)) {
                        RecordError::Capacity
                    } else {
                        RecordError::Command(index)
                    }
                })?,
            );
        }
        // Maps are safe only AFTER recursive duplicate rejection and native typed replay.
        // Exact equality also rejects missing nullable root and command fields.
        let actual: serde_json::Value =
            serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        let expected =
            serde_json::to_value(WireRecord::from_game(&game)).map_err(|_| RecordError::Json)?;
        if actual != expected {
            return Err(RecordError::Derived);
        }
        if stop() {
            return Err(RecordError::Stopped);
        }
        Ok((game, undo))
    }
}
impl Serialize for GameRecord {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        crate::record::serialize_json(&self.json, serializer)
    }
}
impl<'de> Deserialize<'de> for GameRecord {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(d)?;
        Self::from_json(raw.get()).map_err(serde::de::Error::custom)
    }
}
impl GameCommand {
    /// Strict bounded syntax; Game still owns actor/context/rules admission.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        strict(text, 4096)?;
        serde_json::from_str(text).map_err(|_| RecordError::Json)
    }
}
impl Action {
    /// Strict complete native action syntax; local/Game admission still required.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        strict(text, 4096)?;
        serde_json::from_str(text).map_err(|_| RecordError::Json)
    }
}

// A visitor rather than an intermediate Value/map: duplicate tags/nested fields
// are rejected before serde's internally tagged enum buffering can normalize them.
fn strict(text: &str, limit: usize) -> Result<(), RecordError> {
    if text.len() > limit {
        return Err(RecordError::Capacity);
    }
    crate::record::reject_duplicates(text).map_err(|_| RecordError::Json)
}
