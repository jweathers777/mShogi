//! Native strict records. No desktop archive or execution framework is defined here.
use super::*;
use serde::{
    Deserialize, Serialize,
    de::{self, MapAccess, SeqAccess, Visitor},
};
use serde_json::value::RawValue;
use std::collections::BTreeSet;

/// Native schema, independent of the immutable profile and shared transport/envelope.
/// Schema 3 removes operational stops. Schema 1 records retain the promised
/// replay path under current T06, with validated stop annotations omitted.
pub const GAME_RECORD_SCHEMA_VERSION: u32 = 3;
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
    /// Root and event ledger kept for prefix identity and suffixes, so an
    /// incremental transport neither re-parses nor replays the record.
    root: WirePosition,
    root_conclusion: Option<Conclusion>,
    ledger: Vec<GameEvent>,
}

/// New native events after an exact transport-bound retained state (MSH-204).
/// Deserialization checks syntax only; admission occurs in a native
/// incremental session. This is not a saved-game format or a root identity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameDelta {
    start_event: usize,
    events: Vec<GameEvent>,
    #[serde(deserialize_with = "required_option")]
    conclusion: Option<Conclusion>,
}
impl GameDelta {
    /// Number of accepted events at the required base. Not a history identity.
    pub fn start_event(&self) -> usize {
        self.start_event
    }
    /// Number of newly supplied events.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
    /// Strict duplicate-free typed parsing, bounded as native records.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        strict(text, MAX_GAME_RECORD_BYTES)?;
        let delta: Self = serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        delta.check_capacity()?;
        Ok(delta)
    }
    /// Compact transport encoding.
    pub fn to_json(&self) -> Result<String, RecordError> {
        self.check_capacity()?;
        let text = serde_json::to_string(self).map_err(|_| RecordError::Json)?;
        if text.len() > MAX_GAME_RECORD_BYTES {
            return Err(RecordError::Capacity);
        }
        Ok(text)
    }
    fn check_capacity(&self) -> Result<(), RecordError> {
        if self
            .start_event
            .checked_add(self.events.len())
            .is_none_or(|end| end > MAX_GAME_EVENTS)
        {
            return Err(RecordError::Capacity);
        }
        Ok(())
    }
    /// Admit every new command natively and check each derived event and the
    /// final conclusion. On any failure the undo tokens restore the same
    /// semantic game (events and state); the runtime revision still advances.
    fn apply(&self, game: &mut Game) -> Result<(), RecordError> {
        self.check_capacity()?;
        if self.start_event != game.events.len() {
            return Err(RecordError::Prefix);
        }
        let mut undo: Vec<GameUndo> = Vec::with_capacity(self.events.len());
        let mut failure = None;
        for (offset, event) in self.events.iter().enumerate() {
            let index = self.start_event + offset;
            if event.index != index {
                failure = Some(RecordError::Command(index));
                break;
            }
            match game.submit(GameRequest {
                expected: game.context(),
                actor: event.actor,
                command: event.command,
            }) {
                Ok(token) => undo.push(token),
                Err(GameError::Capacity(_)) => {
                    failure = Some(RecordError::Capacity);
                    break;
                }
                Err(_) => {
                    failure = Some(RecordError::Command(index));
                    break;
                }
            }
            if game.events.last() != Some(event) {
                failure = Some(RecordError::Derived);
                break;
            }
        }
        if failure.is_none() && self.conclusion != game.conclusion() {
            failure = Some(RecordError::Derived);
        }
        match failure {
            None => Ok(()),
            Some(error) => {
                while let Some(mut token) = undo.pop() {
                    game.undo(&mut token).expect("restore the retained game");
                }
                Err(error)
            }
        }
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
// Nullable fields are still mandatory: plain Option decoding would accept
// an omitted field as null.
fn required_option<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    Option::<T>::deserialize(d)
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
    /// The caller stopped the replay.
    Stopped,
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
}
impl WireRecord {
    fn from_game(game: &Game) -> Self {
        Self {
            events: game.events.clone(),
            ..Self::without_events(game)
        }
    }
    fn without_events(game: &Game) -> Self {
        Self {
            format: "mshogi.tori.game".into(),
            game: "tori".into(),
            profile: PROFILE_ID.into(),
            edition: PROFILE_EDITION.into(),
            schema_version: GAME_RECORD_SCHEMA_VERSION,
            mode: "untimed".into(),
            root: WirePosition::from_state(&game.root),
            root_conclusion: game.event_prefix(0).expect("root").conclusion,
            events: vec![],
            current_position: WirePosition::from_state(game.position().state()),
            conclusion: game.conclusion(),
        }
    }
}
pub(super) fn encode(game: &Game) -> Result<String, RecordError> {
    serde_json::to_string(&WireRecord::from_game(game)).map_err(|_| RecordError::Json)
}
/// Exact length of [`encode`] without encoding the history again (MSH-204).
/// Compact JSON writes the event list as `[e1,e2,...]`, so the length is the
/// record with an empty list plus each event's own encoding and the commas.
pub(super) fn encoded_len(game: &Game, event_bytes: usize) -> Result<usize, RecordError> {
    let empty = serde_json::to_vec(&WireRecord::without_events(game))
        .map_err(|_| RecordError::Json)?
        .len();
    Ok(empty + event_bytes + game.events.len().saturating_sub(1))
}
/// Compact encoded length of one event inside the record.
pub(super) fn event_len(event: &GameEvent) -> Result<usize, RecordError> {
    serde_json::to_vec(event)
        .map(|v| v.len())
        .map_err(|_| RecordError::Json)
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
            root: WirePosition::from_state(&game.root),
            root_conclusion: game.event_prefix(0).expect("root").conclusion,
            ledger: game.events.clone(),
        })
    }
    /// Whether an already validated record is an exact prefix of this record:
    /// same root and root result and the earlier events in order. Both records
    /// were replay-validated, so equal roots and events imply equal derived
    /// state. Cache identity only; nothing is admitted or reconstructed.
    pub fn has_prefix(&self, earlier: &Self) -> bool {
        let count = earlier.ledger.len();
        count <= self.ledger.len()
            && self.root == earlier.root
            && self.root_conclusion == earlier.root_conclusion
            && self.ledger[..count] == earlier.ledger[..]
    }
    /// Copy only new events for an incremental transport. The host must first
    /// establish the previous record with `has_prefix`.
    pub fn suffix(&self, start_event: usize) -> Result<GameDelta, RecordError> {
        if start_event > self.ledger.len() {
            return Err(RecordError::Prefix);
        }
        Ok(GameDelta {
            start_event,
            events: self.ledger[start_event..].to_vec(),
            conclusion: self.conclusion,
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
    /// Reconstruct semantic history with fresh native runtime authority.
    pub fn to_game(&self) -> Result<Game, RecordError> {
        self.to_game_with_stop(&mut || false)
    }
    /// Reconstruct with cooperative cancellation between replayed events.
    pub fn to_game_with_stop(&self, stop: &mut impl FnMut() -> bool) -> Result<Game, RecordError> {
        Self::replay(&self.json, stop).map(|(game, _)| game)
    }
    /// Reconstruct the exact native command ledger and fresh LIFO undo handles.
    pub fn to_game_with_undo(&self) -> Result<(Game, Vec<GameUndo>), RecordError> {
        Self::replay(&self.json, &mut || false)
    }
    fn replay(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(Game, Vec<GameUndo>), RecordError> {
        strict(text, MAX_GAME_RECORD_BYTES)?;
        #[derive(Deserialize)]
        struct Version {
            schema_version: u32,
        }
        #[derive(Deserialize)]
        struct Probe {
            #[serde(flatten)]
            version: Version,
            #[serde(flatten)]
            _rest: serde_json::Map<String, serde_json::Value>,
        }
        let probe: Probe = serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        if probe.version.schema_version == 1 {
            return Self::replay_v1(text, stop);
        }
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
            if stop() {
                return Err(RecordError::Stopped);
            }
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
        if stop() {
            return Err(RecordError::Stopped);
        }
        Ok((game, undo))
    }
}
// Operational annotations exist only in the promised schema-1 reader.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum StopReason {
    Cancelled,
    ResourceLimit,
    EngineFailure,
    WorkLimit,
}
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum CommandV1 {
    Play { action: Action },
    Resign,
    AgreeDraw,
    DeclareImpasse,
    Stop { reason: StopReason },
}
/// Schema-1 event: the earlier T06 obligation and replay-work fields are
/// rule-dependent and ignored; every other field must equal the replay.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EventV1 {
    index: usize,
    board_ply: usize,
    actor: Side,
    command: CommandV1,
    #[serde(deserialize_with = "required_option")]
    effects: Option<Effects>,
    occurrences: Vec<usize>,
    #[serde(rename = "obligation", deserialize_with = "required_option")]
    _obligation: Option<serde_json::Value>,
    #[serde(deserialize_with = "required_option")]
    conclusion: Option<Conclusion>,
    #[serde(deserialize_with = "required_option")]
    incomplete: Option<StopReason>,
    #[serde(rename = "replay_work")]
    _replay_work: u64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireRecordV1 {
    format: String,
    game: String,
    profile: String,
    edition: String,
    schema_version: u32,
    mode: String,
    root: WirePosition,
    #[serde(deserialize_with = "required_option")]
    root_conclusion: Option<Conclusion>,
    events: Vec<EventV1>,
    current_position: WirePosition,
    #[serde(deserialize_with = "required_option")]
    conclusion: Option<Conclusion>,
    #[serde(rename = "obligation", deserialize_with = "required_option")]
    _obligation: Option<serde_json::Value>,
    #[serde(deserialize_with = "required_option")]
    incomplete: Option<StopReason>,
}
impl GameRecord {
    /// Read a schema-1 record (earlier T06 rule, MSH-129) by replaying its
    /// commands under the current rule (MSH-208). A record whose history now
    /// contains a refused third occurrence, or whose derived facts differ, is
    /// refused (`Command` or `Derived`). The result encodes as the current schema.
    fn replay_v1(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(Game, Vec<GameUndo>), RecordError> {
        let wire: WireRecordV1 = serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        if wire.format != "mshogi.tori.game"
            || wire.game != "tori"
            || wire.profile != PROFILE_ID
            || wire.edition != PROFILE_EDITION_V1
            || wire.schema_version != 1
            || wire.mode != "untimed"
        {
            return Err(RecordError::Identity);
        }
        if wire.events.len() > MAX_GAME_EVENTS {
            return Err(RecordError::Capacity);
        }
        let mut game = Game::from_root(wire.root.root()?).map_err(|_| RecordError::Root)?;
        if wire.root_conclusion != game.conclusion() {
            return Err(RecordError::Derived);
        }
        let mut undo = Vec::with_capacity(wire.events.len());
        let mut incomplete = None;
        for (i, event) in wire.events.iter().enumerate() {
            if stop() {
                return Err(RecordError::Stopped);
            }
            let command = match event.command {
                CommandV1::Play { action } => Some(GameCommand::Play { action }),
                CommandV1::Resign => Some(GameCommand::Resign),
                CommandV1::AgreeDraw => Some(GameCommand::AgreeDraw),
                CommandV1::DeclareImpasse => Some(GameCommand::DeclareImpasse),
                CommandV1::Stop { reason } => {
                    if game.conclusion().is_some() || incomplete.is_some() {
                        return Err(RecordError::Command(i));
                    }
                    incomplete = Some(reason);
                    None
                }
            };
            let effects = if let Some(command) = command {
                undo.push(
                    game.submit(GameRequest {
                        expected: game.context(),
                        actor: event.actor,
                        command,
                    })
                    .map_err(|e| match e {
                        GameError::Capacity(_) => RecordError::Capacity,
                        _ => RecordError::Command(i),
                    })?,
                );
                incomplete = None;
                game.events.last().expect("accepted event").effects
            } else {
                None
            };
            if event.index != i
                || game.snapshot().board_ply != event.board_ply
                || effects != event.effects
                || game.occurrences() != event.occurrences
                || game.conclusion() != event.conclusion
                || incomplete != event.incomplete
            {
                return Err(RecordError::Derived);
            }
        }
        if WirePosition::from_state(game.position().state()) != wire.current_position
            || game.conclusion() != wire.conclusion
            || incomplete != wire.incomplete
        {
            return Err(RecordError::Derived);
        }
        if stop() {
            return Err(RecordError::Stopped);
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
