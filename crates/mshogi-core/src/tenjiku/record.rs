//! Strict profile-qualified native records; replay is authoritative, never cached JSON facts.
use super::*;
use crate::record::RecordedCommand;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::{error::Error, fmt};

/// Native schema number, independently qualified by the immutable profile/edition.
pub const GAME_RECORD_SCHEMA_VERSION: u32 = 3;

/// Exact native bytes and replay-derived metadata. No undo or runtime identity is serialized.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRecord {
    json: String,
    events: usize,
    board_plies: usize,
    side: Side,
    conclusion: Option<Conclusion>,
    profile: Profile,
}
/// Typed record refusal; neither cancellation nor capacity is a game outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordError {
    /// A command is too large, or a native runtime counter or root bound was exceeded.
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
    edition: &'static str,
    schema_version: u32,
    root: &'a StructuralState,
    #[serde(serialize_with = "serialize_events")]
    events: &'a [GameEvent],
}
fn recorded(event: &GameEvent) -> RecordedCommand<Side, &GameCommand> {
    RecordedCommand {
        actor: event.actor,
        command: &event.command,
    }
}
fn serialize_events<S: serde::Serializer>(
    events: &[GameEvent],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(events.iter().map(recorded))
}
impl<'a> WireRecord<'a> {
    fn from_game(game: &'a Game) -> Self {
        Self {
            edition: game.profile().edition(),
            schema_version: GAME_RECORD_SCHEMA_VERSION,
            root: game.native_root(),
            events: game.events(),
        }
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ParsedRecord {
    edition: String,
    schema_version: u32,
    root: StructuralState,
    events: Vec<RecordedCommand<Side, GameCommand>>,
}
/// Serialize a borrowed event ledger; no cloned history or retained undo vector.
pub(super) fn encode(game: &Game) -> Result<String, RecordError> {
    serde_json::to_string(&WireRecord::from_game(game)).map_err(|_| RecordError::Json)
}
/// Exact size for explicit diagnostics; ordinary admission uses compact record bounds.
pub(super) fn encoded_size(game: &Game) -> Result<usize, RecordError> {
    encode(game).map(|text| text.len())
}
fn check(stop: &mut impl FnMut() -> bool) -> Result<(), RecordError> {
    if stop() {
        Err(RecordError::Stopped)
    } else {
        Ok(())
    }
}
impl GameRecord {
    /// Capture all accepted semantics, with derived metadata.
    pub fn from_game(game: &Game) -> Result<Self, RecordError> {
        let json = encode(game)?;
        Ok(Self {
            json,
            events: game.events().len(),
            board_plies: game.snapshot().board_ply,
            side: game.position().side_to_move(),
            conclusion: game.snapshot().conclusion,
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
        Self::from_json_with_game(text, stop).map(|(record, _)| record)
    }
    /// Parse and validate once, retaining the reconstructed game for the caller.
    pub fn from_json_with_game(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(Self, Game), RecordError> {
        let (game, _) = Self::replay(text, false, stop)?;
        check(stop)?;
        let record = Self::from_game(&game)?;
        check(stop)?;
        Ok((record, game))
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
        strict(text)?;
        check(stop)?;
        let wire: ParsedRecord = serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        check(stop)?;
        if wire.edition != wire.root.profile().edition()
            || wire.schema_version != GAME_RECORD_SCHEMA_VERSION
        {
            return Err(RecordError::Identity);
        }

        let position = Position::install(wire.root).map_err(|_| RecordError::Root)?;
        check(stop)?;
        let mut game = Game::from_root_with_stop(position, stop).map_err(|error| match error {
            GameError::Capacity(_) => RecordError::Capacity,
            GameError::Stopped => RecordError::Stopped,
            _ => RecordError::Root,
        })?;
        check(stop)?;
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
        }
        check(stop)?;
        // Typed validation alone permits omitted Option fields; complete wire equality
        // also rejects missing nulls and every forged redundant field after raw duplicates reject.
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
    /// Parse complete bounded command syntax; native Game still owns admission.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        strict_command(text)?;
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
fn strict(text: &str) -> Result<(), RecordError> {
    crate::record::reject_duplicates(text).map_err(|_| RecordError::Json)
}
/// One command has a small fixed size, independent of game length.
fn strict_command(text: &str) -> Result<(), RecordError> {
    if text.len() > 4096 {
        return Err(RecordError::Capacity);
    }
    strict(text)
}

#[cfg(test)]
#[path = "record_tests.rs"]
mod tests;

#[cfg(test)]
mod compact_bounds {
    use super::*;
    use crate::record::MAX_COMPACT_ROOT_BYTES;
    #[test]
    fn maximal_wire_fields_fit_compact_bounds() {
        for profile in Profile::ALL {
            let game = Game::standard(profile);
            let mut root = game.native_root().clone();
            root.root = RootMetadata::imported("\0".repeat(1024), u64::MAX).unwrap();
            // Deliberately overfill material/identities to bound every valid root.
            for base in Base::ALL {
                root.board = vec![
                    Some(Piece {
                        id: u16::MAX,
                        side: Side::Sente,
                        base,
                        promoted: false,
                        right: PromotionRight::FirstDeclinedNextMove
                    });
                    256
                ];
                let wire = WireRecord {
                    edition: profile.edition(),
                    schema_version: GAME_RECORD_SCHEMA_VERSION,
                    root: &root,
                    events: &[],
                };
                assert!(serde_json::to_vec(&wire).unwrap().len() <= MAX_COMPACT_ROOT_BYTES);
            }
        }
    }
}
