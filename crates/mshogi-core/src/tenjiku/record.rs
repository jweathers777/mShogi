//! Strict profile-qualified native records; replay is authoritative, never cached JSON facts.
use super::*;
use serde::{
    Deserialize, Serialize,
    de::{self, MapAccess, SeqAccess, Visitor},
};
use serde_json::value::RawValue;
use std::{collections::BTreeSet, error::Error, fmt};

/// Native schema number, independently qualified by the immutable profile/edition.
pub const GAME_RECORD_SCHEMA_VERSION: u32 = 1;
/// Initial native encoded bound; product documents must measure their own aggregate policy.
pub const MAX_GAME_RECORD_BYTES: usize = 1024 * 1024;

/// Exact native bytes and replay-derived metadata. No undo or runtime identity is serialized.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRecord {
    json: String,
    events: usize,
    board_plies: usize,
    side: Side,
    conclusion: Option<Conclusion>,
    reconstruction_work: u64,
    profile: Profile,
}
/// Typed record refusal; neither cancellation nor capacity is a game outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordError {
    /// Encoded/event/native work capacity was exceeded.
    Capacity,
    /// Malformed, duplicate, unknown, missing or trailing JSON.
    Json,
    /// Conflicting game/profile/edition/schema/mode identities.
    Identity,
    /// Invalid structural root or root admission.
    Root,
    /// Rejected command at the zero-based event index.
    Command(usize),
    /// Claimed derived facts disagree with authoritative replay.
    Derived,
    /// Unavailable event prefix.
    Prefix,
    /// Caller explicitly stopped reconstruction.
    Stopped,
}
impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tenjiku native record refused: {self:?}")
    }
}
impl Error for RecordError {}

#[derive(Serialize)]
struct WireRecord<'a> {
    format: &'static str,
    game: &'static str,
    profile: Profile,
    edition: &'static str,
    schema_version: u32,
    mode: &'static str,
    root: &'a StructuralState,
    root_conclusion: Option<Conclusion>,
    root_pending_bare: Option<Side>,
    root_draw_offer: Option<Side>,
    root_incomplete: Option<StopReason>,
    events: &'a [GameEvent],
    current_position: &'a StructuralState,
    conclusion: Option<Conclusion>,
    pending_bare: Option<Side>,
    draw_offer: Option<Side>,
    incomplete: Option<StopReason>,
}
impl<'a> WireRecord<'a> {
    fn from_game(game: &'a Game) -> Self {
        Self {
            format: "mshogi.tenjiku.game",
            game: "tenjiku",
            profile: game.profile(),
            edition: game.profile().edition(),
            schema_version: GAME_RECORD_SCHEMA_VERSION,
            mode: "untimed",
            root: game.native_root(),
            root_conclusion: game.root_snapshot().conclusion,
            root_pending_bare: game.root_snapshot().pending_bare,
            root_draw_offer: game.root_snapshot().draw_offer,
            root_incomplete: game.root_snapshot().incomplete,
            events: game.events(),
            current_position: game.position().state(),
            conclusion: game.snapshot().conclusion,
            pending_bare: game.snapshot().pending_bare,
            draw_offer: game.snapshot().draw_offer,
            incomplete: game.snapshot().incomplete,
        }
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ParsedRecord {
    format: String,
    game: String,
    profile: Profile,
    edition: String,
    schema_version: u32,
    mode: String,
    root: StructuralState,
    root_conclusion: Option<Conclusion>,
    root_pending_bare: Option<Side>,
    root_draw_offer: Option<Side>,
    root_incomplete: Option<StopReason>,
    events: Vec<GameEvent>,
    current_position: StructuralState,
    conclusion: Option<Conclusion>,
    pending_bare: Option<Side>,
    draw_offer: Option<Side>,
    incomplete: Option<StopReason>,
}
/// Serialize a borrowed event ledger; no cloned history or retained undo vector.
pub(super) fn encode(game: &Game) -> Result<String, RecordError> {
    serde_json::to_string(&WireRecord::from_game(game)).map_err(|_| RecordError::Json)
}
/// Exact native encoded size used by transactional admission, not a guessed estimate.
pub(super) fn encoded_size(game: &Game) -> Result<usize, RecordError> {
    let mut wire = WireRecord::from_game(game);
    wire.events = &[];
    let base = serde_json::to_vec(&wire)
        .map_err(|_| RecordError::Json)?
        .len();
    base.checked_add(game.events_encoded_bytes)
        .and_then(|bytes| bytes.checked_add(game.events().len().saturating_sub(1)))
        .ok_or(RecordError::Capacity)
}
fn check(stop: &mut impl FnMut() -> bool) -> Result<(), RecordError> {
    if stop() {
        Err(RecordError::Stopped)
    } else {
        Ok(())
    }
}
impl GameRecord {
    /// Capture all accepted semantics, with derived metadata and native bounds.
    pub fn from_game(game: &Game) -> Result<Self, RecordError> {
        let json = encode(game)?;
        if json.len() > MAX_GAME_RECORD_BYTES || game.events().len() > MAX_GAME_EVENTS {
            return Err(RecordError::Capacity);
        }
        Ok(Self {
            json,
            events: game.events().len(),
            board_plies: game.snapshot().board_ply,
            side: game.position().side_to_move(),
            conclusion: game.snapshot().conclusion,
            reconstruction_work: game.replay_work(),
            profile: game.profile(),
        })
    }
    /// Canonical complete native bytes.
    pub fn to_json(&self) -> &str {
        &self.json
    }
    /// Borrow canonical complete native bytes.
    pub fn as_json(&self) -> &str {
        &self.json
    }
    /// Actual accepted events, including non-board commands.
    pub fn event_count(&self) -> usize {
        self.events
    }
    /// Known complete board actions, independent of imported counters.
    pub fn known_board_actions(&self) -> usize {
        self.board_plies
    }
    /// Replay-established next actor.
    pub fn side_to_move(&self) -> Side {
        self.side
    }
    /// Replay-established native outcome only.
    pub fn conclusion(&self) -> Option<Conclusion> {
        self.conclusion
    }
    /// Deterministic native reconstruction work, never trusted from wire input.
    pub fn reconstruction_work(&self) -> u64 {
        self.reconstruction_work
    }
    /// The whole immutable profile selected before root construction.
    pub fn profile(&self) -> Profile {
        self.profile
    }
    /// Strict syntax/identity admission followed by authoritative native replay.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        Self::from_json_with_stop(text, &mut || false)
    }
    /// Strict admission with cancellation between parsing/validation and every event.
    pub fn from_json_with_stop(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self, RecordError> {
        let (game, _) = Self::replay(text, false, stop)?;
        check(stop)?;
        let record = Self::from_game(&game)?;
        check(stop)?;
        Ok(record)
    }
    /// Fresh authoritative state without retaining per-event private undo snapshots.
    pub fn to_game(&self) -> Result<Game, RecordError> {
        self.to_game_with_stop(&mut || false)
    }
    /// Fresh reconstruction with caller cancellation; caller/record remain untouched.
    pub fn to_game_with_stop(&self, stop: &mut impl FnMut() -> bool) -> Result<Game, RecordError> {
        Self::replay(&self.json, false, stop).map(|(game, _)| game)
    }
    /// Explicit consumer request for the actual complete LIFO undo sequence.
    pub fn to_game_with_undo(&self) -> Result<(Game, Vec<GameUndo>), RecordError> {
        Self::replay(&self.json, true, &mut || false)
    }
    fn replay(
        text: &str,
        retain_undo: bool,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(Game, Vec<GameUndo>), RecordError> {
        check(stop)?;
        strict(text, MAX_GAME_RECORD_BYTES)?;
        check(stop)?;
        let wire: ParsedRecord = serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        check(stop)?;
        if wire.format != "mshogi.tenjiku.game"
            || wire.game != "tenjiku"
            || wire.edition != wire.profile.edition()
            || wire.schema_version != GAME_RECORD_SCHEMA_VERSION
            || wire.mode != "untimed"
            || wire.root.profile() != wire.profile
            || wire.current_position.profile() != wire.profile
        {
            return Err(RecordError::Identity);
        }
        if wire.events.len() > MAX_GAME_EVENTS {
            return Err(RecordError::Capacity);
        }
        let position = Position::install(wire.root).map_err(|_| RecordError::Root)?;
        check(stop)?;
        let mut game = Game::from_root_with_stop(position, stop).map_err(|error| match error {
            GameError::Capacity(_) => RecordError::Capacity,
            GameError::Stopped => RecordError::Stopped,
            _ => RecordError::Root,
        })?;
        check(stop)?;
        if game.root_snapshot().conclusion != wire.root_conclusion
            || game.root_snapshot().pending_bare != wire.root_pending_bare
            || game.root_snapshot().draw_offer != wire.root_draw_offer
            || game.root_snapshot().incomplete != wire.root_incomplete
        {
            return Err(RecordError::Derived);
        }
        let mut undos = if retain_undo {
            Vec::with_capacity(wire.events.len())
        } else {
            Vec::new()
        };
        for (index, event) in wire.events.iter().enumerate() {
            check(stop)?;
            let undo = game
                .submit_with_stop(
                    GameRequest {
                        expected: game.context(),
                        actor: event.actor,
                        command: event.command.clone(),
                    },
                    stop,
                )
                .map_err(|error| match error {
                    GameError::Capacity(_) => RecordError::Capacity,
                    GameError::Stopped => RecordError::Stopped,
                    _ => RecordError::Command(index),
                })?;
            if retain_undo {
                undos.push(undo);
            } // Otherwise this iteration's token is discarded, never retained.

            check(stop)?;
            if game.events().last() != Some(event) {
                return Err(RecordError::Derived);
            }
        }
        check(stop)?;
        // Typed validation alone permits omitted Option fields; complete wire equality
        // also rejects missing nulls and every forged redundant field after raw duplicates reject.
        if game.position().state() != &wire.current_position
            || game.snapshot().conclusion != wire.conclusion
            || game.snapshot().pending_bare != wire.pending_bare
            || game.snapshot().draw_offer != wire.draw_offer
            || game.snapshot().incomplete != wire.incomplete
        {
            return Err(RecordError::Derived);
        }
        let actual: serde_json::Value =
            serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        check(stop)?;
        let expected =
            serde_json::to_value(WireRecord::from_game(&game)).map_err(|_| RecordError::Json)?;
        check(stop)?;
        if actual != expected {
            return Err(RecordError::Derived);
        }
        Ok((game, undos))
    }
}
impl Serialize for GameRecord {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        RawValue::from_string(self.json.clone())
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for GameRecord {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(d)?;
        Self::from_json(raw.get()).map_err(serde::de::Error::custom)
    }
}
impl GameCommand {
    /// Parse complete bounded command syntax; native Game still owns admission.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        strict(text, 4096)?;
        let command: Self = serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        let input: serde_json::Value = serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        let canonical = serde_json::to_value(&command).map_err(|_| RecordError::Json)?;
        // serde internally tagged unit variants can otherwise ignore unknown keys.
        if input != canonical {
            return Err(RecordError::Json);
        }
        Ok(command)
    }
}
// Recursive syntax check happens before any internally tagged enum can buffer
// and normalize a nested duplicate. Escaped-equivalent keys compare decoded.
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

#[cfg(test)]
#[path = "record_tests.rs"]
mod tests;
