//! Native strict records. No desktop archive or execution framework is defined here.
use super::*;
use serde::{
    Deserialize, Serialize,
    de::{self, MapAccess, SeqAccess, Visitor},
};
use serde_json::value::RawValue;
use std::collections::BTreeSet;

/// Native schema, independent of the immutable profile and shared transport/envelope.
pub const GAME_RECORD_SCHEMA_VERSION: u32 = 1;
/// Native payload bound, leaving room inside the existing 1MiB shared envelope.
pub const MAX_GAME_RECORD_BYTES: usize = 512 * 1024;
/// Bounded complete native record with replay-validated cached metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRecord {
    json: String,
    events: usize,
    board_plies: usize,
    side: Side,
    conclusion: Option<Conclusion>,
}
/// Malformed/capacity/semantic rejection, never a game result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordError {
    /// Input or replay exceeds supported native bounds.
    Capacity,
    /// Duplicate/unknown/missing/malformed field.
    Json,
    /// Unsupported/conflicting format, game, profile, edition or schema.
    Identity,
    /// Structurally or locally inadmissible root or contradictory provenance.
    Root,
    /// Native command refusal at this zero-based event index.
    Command(usize),
    /// Requested event prefix does not exist.
    Prefix,
    /// Replay-derived facts disagree at a prefix or current state.
    Derived,
}
impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tori native record refused: {self:?}")
    }
}
impl Error for RecordError {}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Placed {
    square: Square,
    piece: Piece,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WirePosition {
    root_kind: RootKind,
    source: Option<String>,
    earlier_history_unknown: bool,
    historical_reachability_proven: bool,
    declared_completed_plies: u64,
    accepted_plies: u64,
    declared_total_plies: u64,
    side_to_move: Side,
    board: Vec<Placed>,
    hands: [[u8; 6]; 2],
}
impl WirePosition {
    fn from_state(s: &StructuralState) -> Self {
        Self {
            root_kind: s.root.kind,
            source: s.root.source.clone(),
            earlier_history_unknown: s.root.earlier_history_unknown(),
            historical_reachability_proven: s.root.historical_reachability_proven(),
            declared_completed_plies: s.root.declared_completed_plies,
            accepted_plies: s.accepted_plies,
            declared_total_plies: s.root.declared_completed_plies + s.accepted_plies,
            side_to_move: s.side,
            board: Square::all()
                .filter_map(|square| s.board[square.index()].map(|piece| Placed { square, piece }))
                .collect(),
            hands: [s.hands[0].0, s.hands[1].0],
        }
    }
    fn root(&self) -> Result<Position, RecordError> {
        if self.accepted_plies != 0 {
            return Err(RecordError::Root);
        }
        let root = match self.root_kind {
            RootKind::Standard => Position::standard(),
            kind => {
                let meta = match (kind, &self.source) {
                    (RootKind::Diagram, None) => {
                        RootMetadata::diagram(self.declared_completed_plies)
                    }
                    (RootKind::Imported, Some(source)) => {
                        RootMetadata::imported(source.clone(), self.declared_completed_plies)
                            .map_err(|_| RecordError::Root)?
                    }
                    _ => return Err(RecordError::Root),
                };
                let hands = [0, 1].map(|i| {
                    Hand::from_counts(
                        &Face::HANDS
                            .into_iter()
                            .zip(self.hands[i])
                            .collect::<Vec<_>>(),
                    )
                    .map_err(|_| RecordError::Root)
                });
                let [s, g] = hands;
                Position::from_diagram(
                    &self
                        .board
                        .iter()
                        .map(|p| (p.square, p.piece))
                        .collect::<Vec<_>>(),
                    [s?, g?],
                    self.side_to_move,
                    meta,
                )
                .map_err(|_| RecordError::Root)?
            }
        };
        if *self != Self::from_state(root.state()) {
            return Err(RecordError::Root);
        }
        Ok(root)
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireRecord {
    format: String,
    game: String,
    profile: String,
    edition: String,
    schema_version: u32,
    mode: String,
    root: WirePosition,
    root_conclusion: Option<Conclusion>,
    events: Vec<GameEvent>,
    current_position: WirePosition,
    conclusion: Option<Conclusion>,
    obligation: Option<Obligation>,
    incomplete: Option<StopReason>,
}
impl WireRecord {
    fn from_game(game: &Game) -> Self {
        Self {
            format: "mshogi.tori.game".into(),
            game: "tori".into(),
            profile: PROFILE_ID.into(),
            edition: PROFILE_EDITION.into(),
            schema_version: GAME_RECORD_SCHEMA_VERSION,
            mode: "untimed".into(),
            root: WirePosition::from_state(&game.root),
            root_conclusion: game.event_prefix(0).expect("root").conclusion,
            events: game.events.clone(),
            current_position: WirePosition::from_state(game.position().state()),
            conclusion: game.conclusion(),
            obligation: game.obligation().cloned(),
            incomplete: game.snapshot().incomplete,
        }
    }
}
pub(super) fn encode(game: &Game) -> Result<String, RecordError> {
    serde_json::to_string(&WireRecord::from_game(game)).map_err(|_| RecordError::Json)
}
impl GameRecord {
    /// Encode all accepted history without truncation. Admission guarantees this bound.
    pub fn from_game(game: &Game) -> Result<Self, RecordError> {
        let json = encode(game)?;
        if json.len() > MAX_GAME_RECORD_BYTES || game.events.len() > MAX_GAME_EVENTS {
            return Err(RecordError::Capacity);
        }
        Ok(Self {
            json,
            events: game.events.len(),
            board_plies: game.snapshot().board_ply,
            side: game.position().side_to_move(),
            conclusion: game.conclusion(),
        })
    }
    /// Actual native bytes; runtime identities are deliberately excluded.
    pub fn to_json(&self) -> &str {
        &self.json
    }
    /// Total native event/command count, not board ply.
    pub fn event_count(&self) -> usize {
        self.events
    }
    /// Actual known board actions, excluding declared counters and non-board commands.
    pub fn known_board_actions(&self) -> usize {
        self.board_plies
    }
    /// Replay-validated native turn, not an execution capability.
    pub fn side_to_move(&self) -> Side {
        self.side
    }
    /// Replay-validated native conclusion only.
    pub fn conclusion(&self) -> Option<Conclusion> {
        self.conclusion
    }
    /// Strict duplicate-preserving admission and authoritative replay of every prefix.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        Self::from_game(&Self::replay(text)?.0)
    }
    /// Reconstruct semantic history with fresh native runtime authority.
    pub fn to_game(&self) -> Result<Game, RecordError> {
        Self::replay(&self.json).map(|(game, _)| game)
    }
    /// Reconstruct the exact native command ledger and fresh LIFO undo handles.
    pub fn to_game_with_undo(&self) -> Result<(Game, Vec<GameUndo>), RecordError> {
        Self::replay(&self.json)
    }
    fn replay(text: &str) -> Result<(Game, Vec<GameUndo>), RecordError> {
        strict(text, MAX_GAME_RECORD_BYTES)?;
        let wire: WireRecord = serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        if wire.format != "mshogi.tori.game"
            || wire.game != "tori"
            || wire.profile != PROFILE_ID
            || wire.edition != PROFILE_EDITION
            || wire.schema_version != GAME_RECORD_SCHEMA_VERSION
            || wire.mode != "untimed"
        {
            return Err(RecordError::Identity);
        }
        if wire.events.len() > MAX_GAME_EVENTS || wire.events.len() > MAX_GAME_COMMANDS {
            return Err(RecordError::Capacity);
        }
        let mut game = Game::from_root(wire.root.root()?).map_err(|_| RecordError::Root)?;
        if wire.root_conclusion != game.conclusion() {
            return Err(RecordError::Derived);
        }
        let mut undo = Vec::with_capacity(wire.events.len());
        for (i, event) in wire.events.iter().enumerate() {
            undo.push(
                game.submit(GameRequest {
                    expected: game.context(),
                    actor: event.actor,
                    command: event.command,
                })
                .map_err(|e| match e {
                    GameError::Capacity(_) => RecordError::Capacity,
                    _ => RecordError::Command(i),
                })?,
            );
            if game.events.last() != Some(event) {
                return Err(RecordError::Derived);
            }
        }
        // Only AFTER duplicate-preserving validation and typed parsing. Also rejects
        // omitted nullable fields; Option's default cannot silently admit absence.
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
    /// Strict bounded command syntax. Native Game still owns context/rule admission.
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
