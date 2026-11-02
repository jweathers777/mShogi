//! Strict native Chu records; authoritative replay, not a desktop document format.
use super::game::NativeRoot;
use super::*;
use serde::{
    Deserialize, Serialize,
    de::{self, MapAccess, SeqAccess, Visitor},
};
use serde_json::value::RawValue;
use std::{collections::BTreeSet, error::Error, fmt};
/// Native record version, distinct from immutable rules edition and outer transport.
pub const GAME_RECORD_SCHEMA_VERSION: u32 = 1;
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
    format: String,
    game: String,
    profile: String,
    edition: String,
    schema_version: u32,
    mode: String,
    root: NativeRoot,
    root_conclusion: Option<Conclusion>,
    events: Vec<GameEvent>,
    current_position: StructuralState,
    current_protection: LionProtection,
    conclusion: Option<Conclusion>,
    obligations: Vec<Obligation>,
    draw_offer: Option<Side>,
    incomplete: Option<StopReason>,
}
impl WireRecord {
    fn from_game(g: &Game) -> Self {
        Self {
            format: "mshogi.chu.game".into(),
            game: "chu".into(),
            profile: PROFILE_ID.into(),
            edition: PROFILE_EDITION.into(),
            schema_version: GAME_RECORD_SCHEMA_VERSION,
            mode: "untimed".into(),
            root: g.native_root().clone(),
            root_conclusion: g.root_snapshot().conclusion,
            events: g.events().to_vec(),
            current_position: g.position().state().clone(),
            current_protection: g.local_position().protection().clone(),
            conclusion: g.conclusion(),
            obligations: g.obligations().to_vec(),
            draw_offer: g.snapshot().draw_offer,
            incomplete: g.snapshot().incomplete,
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
    /// Strict duplicate-preserving parsing followed by authoritative replay.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        Self::from_game(&Self::replay(text)?.0)
    }
    /// Reconstruct canonical semantics with fresh runtime authority.
    pub fn to_game(&self) -> Result<Game, RecordError> {
        Self::replay(&self.json).map(|v| v.0)
    }
    /// Native consumer reconstruction including actual LIFO reversal handles.
    pub fn to_game_with_undo(&self) -> Result<(Game, Vec<GameUndo>), RecordError> {
        Self::replay(&self.json)
    }
    fn replay(text: &str) -> Result<(Game, Vec<GameUndo>), RecordError> {
        strict(text, MAX_GAME_RECORD_BYTES)?;
        let w: WireRecord = serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        if w.format != "mshogi.chu.game"
            || w.game != "chu"
            || w.profile != PROFILE_ID
            || w.edition != PROFILE_EDITION
            || w.schema_version != GAME_RECORD_SCHEMA_VERSION
            || w.mode != "untimed"
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
        if game.conclusion() != w.root_conclusion {
            return Err(RecordError::Derived);
        }
        let mut undo = Vec::with_capacity(w.events.len());
        for (index, event) in w.events.iter().enumerate() {
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
            if game.events().last() != Some(event) {
                return Err(RecordError::Derived);
            }
        }
        // Maps are safe only AFTER recursive duplicate rejection and native typed replay.
        // Exact equality also rejects missing nullable fields and forged cached summaries.
        let actual: serde_json::Value =
            serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        let expected =
            serde_json::to_value(WireRecord::from_game(&game)).map_err(|_| RecordError::Json)?;
        if actual != expected {
            return Err(RecordError::Derived);
        }
        Ok((game, undo))
    }
}
impl Serialize for GameRecord {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        RawValue::from_string(self.json.clone())
            .map_err(serde::ser::Error::custom)?
            .serialize(s)
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
    struct Unique;
    impl<'de> Deserialize<'de> for Unique {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            d.deserialize_any(UniqueVisitor)
        }
    }
    struct UniqueVisitor;
    impl<'de> Visitor<'de> for UniqueVisitor {
        type Value = Unique;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("duplicate-free bounded JSON")
        }
        fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Unique, M::Error> {
            let mut keys = BTreeSet::new();
            while let Some(key) = map.next_key::<String>()? {
                if !keys.insert(key) {
                    return Err(de::Error::custom("duplicate field"));
                }
                map.next_value::<Unique>()?;
            }
            Ok(Unique)
        }
        fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Unique, S::Error> {
            while seq.next_element::<Unique>()?.is_some() {}
            Ok(Unique)
        }
        fn visit_bool<E: de::Error>(self, _: bool) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_i64<E: de::Error>(self, _: i64) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_u64<E: de::Error>(self, _: u64) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_f64<E: de::Error>(self, _: f64) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_str<E: de::Error>(self, _: &str) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_unit<E: de::Error>(self) -> Result<Unique, E> {
            Ok(Unique)
        }
    }
    let mut d = serde_json::Deserializer::from_str(text);
    Unique::deserialize(&mut d).map_err(|_| RecordError::Json)?;
    d.end().map_err(|_| RecordError::Json)
}
