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
    // Validated immutable prefix identity for incremental process traffic.
    root: NativeRoot,
    ledger: Vec<GameEvent>,
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
        if json.len() > MAX_GAME_RECORD_BYTES {
            return Err(RecordError::Capacity);
        }
        Ok(Self {
            json,
            events: g.events().len(),
            board_plies: g.snapshot().board_ply,
            side: g.position().side_to_move(),
            conclusion: g.conclusion(),
            root: g.native_root().clone(),
            ledger: g.events().to_vec(),
        })
    }
    /// Exact already admitted prefix identity; counts alone are not identity.
    pub fn has_prefix(&self, earlier: &Self) -> bool {
        self.root == earlier.root && self.ledger.starts_with(&earlier.ledger)
    }
    /// New events after a transport-authenticated exact base.
    pub fn suffix(&self, start_event: usize) -> Result<GameDelta, RecordError> {
        let events = self
            .ledger
            .get(start_event..)
            .ok_or(RecordError::Prefix)?
            .to_vec();
        Ok(GameDelta(DeltaWire {
            start_event,
            events,
            conclusion: self.conclusion,
        }))
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
/// Exact native suffix for a retained process session, not a saved format.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct GameDelta(DeltaWire);
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DeltaWire {
    start_event: usize,
    events: Vec<GameEvent>,
    conclusion: Option<Conclusion>,
}
impl GameDelta {
    /// Required base event count, not an identity by itself.
    pub fn start_event(&self) -> usize {
        self.0.start_event
    }
    /// Number of new events.
    pub fn event_count(&self) -> usize {
        self.0.events.len()
    }
    fn check_capacity(&self) -> Result<(), RecordError> {
        if self
            .0
            .start_event
            .checked_add(self.0.events.len())
            .is_none()
        {
            return Err(RecordError::Capacity);
        }
        Ok(())
    }
    /// Strict duplicate-free syntax; native admission happens on advance.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        strict(text, MAX_GAME_RECORD_BYTES)?;
        let wire: DeltaWire = serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        let actual: serde_json::Value =
            serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        if actual != serde_json::to_value(&wire).map_err(|_| RecordError::Json)? {
            return Err(RecordError::Derived);
        }
        let delta = Self(wire);
        delta.check_capacity()?;
        Ok(delta)
    }
    /// Bounded transport encoding.
    pub fn to_json(&self) -> Result<String, RecordError> {
        self.check_capacity()?;
        let text = serde_json::to_string(self).map_err(|_| RecordError::Json)?;
        if text.len() > MAX_GAME_RECORD_BYTES {
            return Err(RecordError::Capacity);
        }
        Ok(text)
    }
    fn apply(&self, game: &mut Game) -> Result<(), RecordError> {
        self.check_capacity()?;
        if self.0.start_event != game.events().len() {
            return Err(RecordError::Prefix);
        }
        // Fork copies the admitted state without replay. Install only after all
        // native commands, derived effects and the final result agree.
        let mut candidate = game.fork().map_err(|_| RecordError::Root)?;
        for event in &self.0.events {
            candidate
                .submit(GameRequest {
                    expected: candidate.context(),
                    actor: event.actor,
                    command: event.command.clone(),
                })
                .map_err(|e| match e {
                    GameError::Capacity(_) => RecordError::Capacity,
                    _ => RecordError::Command(event.index),
                })?;
            if candidate.events().last() != Some(event) {
                return Err(RecordError::Derived);
            }
        }
        if candidate.conclusion() != self.0.conclusion {
            return Err(RecordError::Derived);
        }
        *game = candidate;
        Ok(())
    }
}
impl<'de> Deserialize<'de> for GameDelta {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(d)?;
        Self::from_json(raw.get()).map_err(serde::de::Error::custom)
    }
}
impl crate::native::incremental::IncrementalRecord for GameRecord {
    type Game = Game;
    type Delta = GameDelta;
    type Error = RecordError;
    fn reconstruct(&self) -> Result<Game, RecordError> {
        self.to_game()
    }
    fn append(game: &mut Game, delta: &GameDelta) -> Result<(), RecordError> {
        delta.apply(game)
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

#[cfg(test)]
mod compact_bounds {
    use super::*;
    use crate::record::MAX_COMPACT_ROOT_BYTES;

    #[test]
    fn maximal_wire_fields_fit_compact_bounds() {
        // Overfilled board/protection bound every admitted native root.
        let mut wire = WireRecord::from_game(&Game::standard());
        wire.root.state.root = RootMetadata::imported("\0".repeat(1024), u64::MAX).unwrap();
        wire.root.protection.owner = Some(Side::Sente);
        wire.root.protection.squares = Square::all().collect();
        for base in Base::ALL {
            wire.root.state.board = vec![
                Some(Piece {
                    side: Side::Sente,
                    base,
                    promoted: false
                });
                144
            ];
            assert!(serde_json::to_vec(&wire).unwrap().len() <= MAX_COMPACT_ROOT_BYTES);
        }
    }
}

#[cfg(test)]
mod incremental_tests {
    use super::*;
    use crate::native::incremental::Session;

    #[test]
    fn suffix_admission_matches_replay_and_refusal_is_atomic() {
        let mut game = Game::standard();
        let mut previous = GameRecord::from_game(&game).unwrap();
        let mut session = Session::<GameRecord>::new();
        assert!(session.advance(&previous.suffix(0).unwrap()).is_err());
        session.reset(&previous).unwrap();
        let mut seed = 213u64;
        for _ in 0..20 {
            for _ in 0..2 {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                let actions = game.legal_actions();
                let action = actions[seed as usize % actions.len()].clone();
                game.submit(GameRequest {
                    expected: game.context(),
                    actor: game.position().side_to_move(),
                    command: GameCommand::Play { action },
                })
                .unwrap();
            }
            let current = GameRecord::from_game(&game).unwrap();
            assert!(current.has_prefix(&previous));
            assert!(!previous.has_prefix(&current));
            let delta = current.suffix(previous.event_count()).unwrap();
            let text = delta.to_json().unwrap();
            let delta = GameDelta::from_json(&text).unwrap();
            let context = session.game().unwrap().context();
            for field in ["board_ply", "index"] {
                let mut forged = serde_json::to_value(&delta).unwrap();
                forged["events"][1][field] = serde_json::json!(999999);
                let bad: GameDelta = serde_json::from_value(forged).unwrap();
                assert!(session.advance(&bad).is_err());
                assert_eq!(session.game().unwrap().context(), context);
                assert_eq!(
                    GameRecord::from_game(session.game().unwrap()).unwrap(),
                    previous
                );
            }
            let game = session.advance(&delta).unwrap();
            assert_eq!(GameRecord::from_game(game).unwrap(), current);
            assert_eq!(
                GameRecord::from_game(&current.to_game().unwrap()).unwrap(),
                current
            );
            previous = current;
        }
        let mut value =
            serde_json::to_value(previous.suffix(previous.event_count()).unwrap()).unwrap();
        value.as_object_mut().unwrap().remove("conclusion");
        assert!(GameDelta::from_json(&value.to_string()).is_err());
        let text = previous
            .suffix(previous.event_count())
            .unwrap()
            .to_json()
            .unwrap();
        assert!(
            GameDelta::from_json(&text.replacen(
                "\"start_event\":",
                "\"start_event\":0,\"start_event\":",
                1
            ))
            .is_err()
        );
        assert!(
            !GameRecord::from_game(&Game::standard())
                .unwrap()
                .has_prefix(&previous)
        );
        session.clear();
        assert!(
            session
                .advance(&previous.suffix(previous.event_count()).unwrap())
                .is_err()
        );
    }
}
