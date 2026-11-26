//! Native strict records. No desktop archive or execution framework is defined here.
use super::*;
use crate::record::RecordedCommand;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

/// Native schema, independent of the immutable profile and shared transport/envelope.
/// Schema 4 stores commands and reconstructs their effects.
pub const GAME_RECORD_SCHEMA_VERSION: u32 = 4;
/// Complete native record with replay-validated cached metadata.
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
    /// Strict duplicate-free typed parsing.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        strict(text)?;
        let delta: Self = serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        delta.check_capacity()?;
        Ok(delta)
    }
    /// Compact transport encoding.
    pub fn to_json(&self) -> Result<String, RecordError> {
        self.check_capacity()?;
        serde_json::to_string(self).map_err(|_| RecordError::Json)
    }
    fn check_capacity(&self) -> Result<(), RecordError> {
        if self.start_event.checked_add(self.events.len()).is_none() {
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
    /// The command, root, declared counter or event index exceeds its representation.
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
    profile: String,
    edition: String,
    schema_version: u32,
    root: WirePosition,
    events: Vec<RecordedCommand<Side, GameCommand>>,
}
impl WireRecord {
    fn from_game(game: &Game) -> Self {
        Self {
            events: game
                .events
                .iter()
                .map(|event| RecordedCommand {
                    actor: event.actor,
                    command: event.command,
                })
                .collect(),
            ..Self::without_events(game)
        }
    }
    fn without_events(game: &Game) -> Self {
        Self {
            profile: PROFILE_ID.into(),
            edition: PROFILE_EDITION.into(),
            schema_version: GAME_RECORD_SCHEMA_VERSION,
            root: WirePosition::from_state(game.root_snapshot().state()),
            events: vec![],
        }
    }
}
pub(super) fn encode(game: &Game) -> Result<String, RecordError> {
    serde_json::to_string(&WireRecord::from_game(game)).map_err(|_| RecordError::Json)
}
impl GameRecord {
    /// Encode all accepted history without truncation.
    pub fn from_game(game: &Game) -> Result<Self, RecordError> {
        let json = encode(game)?;
        Ok(Self {
            json,
            events: game.events.len(),
            board_plies: game.snapshot().board_ply,
            side: game.position().side_to_move(),
            conclusion: game.conclusion(),
            root: WirePosition::from_state(game.root_snapshot().state()),
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
        strict(text)?;
        let wire: WireRecord = serde_json::from_str(text).map_err(|_| RecordError::Json)?;
        if wire.profile != PROFILE_ID
            || wire.edition != PROFILE_EDITION
            || wire.schema_version != GAME_RECORD_SCHEMA_VERSION
        {
            return Err(RecordError::Identity);
        }

        let mut game = Game::from_root(wire.root.root()?).map_err(|_| RecordError::Root)?;
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
    /// Strict bounded command syntax. Native Game still owns context/rule admission.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        strict_command(text)?;
        serde_json::from_str(text).map_err(|_| RecordError::Json)
    }
}
// A visitor rather than an intermediate Value/map: duplicate tags/nested fields
// are rejected before serde's internally tagged enum buffering can normalize them.
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
mod compact_bounds {
    use super::*;
    use crate::record::MAX_COMPACT_ROOT_BYTES;

    #[test]
    fn maximal_wire_fields_fit_compact_bounds() {
        // Deliberately overfill material to bound every structurally valid root.
        let mut wire = WireRecord::without_events(&Game::standard());
        wire.root.root_kind = RootKind::Imported;
        wire.root.source = Some("\0".repeat(1024));
        wire.root.declared_completed_plies = u64::MAX;
        wire.root.declared_total_plies = u64::MAX;
        wire.root.hands = [[u8::MAX; 6]; 2];
        for face in Face::ALL {
            wire.root.board = Square::all()
                .map(|square| Placed {
                    square,
                    piece: Piece {
                        side: Side::Sente,
                        face,
                    },
                })
                .collect();
            assert!(serde_json::to_vec(&wire).unwrap().len() <= MAX_COMPACT_ROOT_BYTES);
        }
    }
}
