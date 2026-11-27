//! Strict game records for the selected untimed profile.
//!
//! A record is a complete accepted ledger, not an SFEN recovery shortcut. Its
//! commands restore effects and conclusions through authoritative game replay.

use std::{error::Error, fmt};

use crate::record::RecordedCommand;
use serde::{Deserialize, Deserializer, Serialize};

use super::{
    Conclusion, CounterSource, DeclarationEligibility, DrawReason, Game, GameCommand, GameError,
    GameEvent, GamePrefixError, GameRequest, GameRoot, INITIAL_SFEN, MutualCheckDiagnostic,
    PROFILE_ID, Position, RootCategory, Side, WinReason,
};
use crate::game::history::AppendGame;

/// Game/match record schema, distinct from profile and transport identities.
pub const SHOGI_RECORD_SCHEMA_VERSION: u32 = 2;

/// Lossless canonical game and untimed match-command ledger.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRecord {
    wire: CachedRecord,
}

/// New game events after an exact transport-bound retained state.
/// Deserialization checks syntax, not legality: admission occurs in a
/// incremental session. This is not a saved-game format or a root identity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameDelta {
    start_event: usize,
    events: Vec<WireEvent>,
    #[serde(deserialize_with = "required_option")]
    conclusion: Option<WireConclusion>,
    #[serde(deserialize_with = "required_option")]
    score_half_points: Option<[u8; 2]>,
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
    /// Expected complete event count after successful admission.
    pub fn end_event(&self) -> Option<usize> {
        self.start_event.checked_add(self.events.len())
    }
    /// Strict duplicate-preserving typed parsing.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        let delta: Self =
            serde_json::from_str(text).map_err(|e| RecordError::Json(e.to_string()))?;
        delta.check_capacity()?;
        Ok(delta)
    }
    /// Compact transport encoding; unchanged saved-game records use GameRecord.
    pub fn to_json(&self) -> Result<String, RecordError> {
        self.check_capacity()?;
        serde_json::to_string(self).map_err(|e| RecordError::Json(e.to_string()))
    }
    fn check_capacity(&self) -> Result<(), RecordError> {
        if self.end_event().is_none() {
            return Err(RecordError::Capacity);
        }
        Ok(())
    }
    fn apply(&self, game: &mut Game) -> Result<(), RecordError> {
        self.check_capacity()?;
        if self.start_event != game.events().len() {
            return Err(RecordError::Prefix);
        }
        // Parse every command before mutation. Derived diagrams are compared
        // byte-for-byte to canonical output; they are never imported.
        let commands = self
            .events
            .iter()
            .enumerate()
            .map(|(offset, event)| {
                let index = self.start_event + offset;
                if event.index != index {
                    return Err(RecordError::Event(index));
                }
                event.command.to_command().ok_or(RecordError::Event(index))
            })
            .collect::<Result<Vec<_>, _>>()?;
        game.append_transaction(|game| {
            for (event, command) in self.events.iter().zip(commands) {
                game.apply_command(event.actor.into_side(), command)
                    .map_err(|e| replay_error(event.index, e))?;
                if *event != WireEvent::from(game.events().last().expect("just appended event")) {
                    return Err(RecordError::Event(event.index));
                }
            }
            if self.conclusion != game.conclusion().map(WireConclusion::from)
                || self.score_half_points != score(game.conclusion())
            {
                return Err(RecordError::Conclusion);
            }
            Ok(())
        })
        .map(drop)
    }
}

impl crate::game::incremental::IncrementalRecord for GameRecord {
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

/// Explicit rejection of malformed, unsupported or inconsistent game records.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordError {
    /// The event index overflows its integer type.
    Capacity,
    /// Strict typed JSON decoding failed, including duplicate and unknown fields.
    Json(String),
    /// Schema, profile edition, mode or selected policies are unsupported.
    Unsupported,
    /// Root category, provenance, canonical state or root conclusion is inconsistent.
    Root,
    /// Authoritative replay rejected a command, or its derived effects disagree.
    Event(usize),
    /// Internal invariant failure during authoritative replay, not malformed-input adjudication.
    Invariant {
        /// Event that encountered the internal failure.
        event_index: usize,
        /// Exact candidate evidence.
        diagnostic: Box<MutualCheckDiagnostic>,
    },
    /// Final conclusion or half-point score disagrees with replay.
    Conclusion,
    /// Requested prefix is beyond the retained ledger.
    Prefix,
}
impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid shogi game record: {self:?}")
    }
}
impl Error for RecordError {}

impl GameRecord {
    /// Validated record schema identity.
    pub fn schema_version(&self) -> u32 {
        SHOGI_RECORD_SCHEMA_VERSION
    }
    /// Accepted command-event count, independent of the declared SFEN counter.
    pub fn event_count(&self) -> usize {
        self.wire.events.len()
    }
    /// Board actions in known history; terminal commands add no board ply.
    pub fn known_board_actions(&self) -> usize {
        self.wire
            .events
            .iter()
            .filter(|event| matches!(event.command, WireCommand::Play { .. }))
            .count()
    }
    /// Current side from the already validated canonical post-diagram, without
    /// replaying history on a host acceptance path. This is not an SFEN restore.
    pub fn side_to_move(&self) -> Side {
        let sfen = self
            .wire
            .events
            .last()
            .map_or(self.wire.root.sfen.as_str(), |event| {
                event.after_sfen.as_str()
            });
        Position::from_sfen(sfen)
            .expect("game record diagrams are validated before construction")
            .side_to_move()
    }
    /// Final conclusion retained in the validated record, without replay.
    pub fn conclusion(&self) -> Option<Conclusion> {
        self.wire
            .conclusion
            .clone()
            .map(WireConclusion::into_conclusion)
    }
    /// Export a rules-owned game, retaining imported-root provenance and every event.
    pub fn from_game(game: &Game) -> Self {
        let initial = Game::from_root(game.root().clone());
        Self {
            wire: CachedRecord {
                root: WireRoot {
                    category: match game.root().category() {
                        RootCategory::Standard => Category::Standard,
                        RootCategory::FullMaterialDiagram => Category::FullMaterialDiagram,
                        RootCategory::ReducedMaterialDiagram => Category::ReducedMaterialDiagram,
                    },
                    sfen: game.root().position().to_sfen(),
                    counter_source: match game.root().counter_source() {
                        None => Source::InitialFixed,
                        Some(CounterSource::Supplied) => Source::Supplied,
                        Some(CounterSource::Defaulted) => Source::Defaulted,
                    },
                    conclusion: initial.conclusion().map(WireConclusion::from),
                    score_half_points: score(initial.conclusion()),
                },
                events: game.events().iter().map(WireEvent::from).collect(),
                conclusion: game.conclusion().map(WireConclusion::from),
                score_half_points: score(game.conclusion()),
            },
        }
    }

    /// Compact UTF-8 JSON in stable field order, without a trailing newline.
    pub fn to_json(&self) -> Result<String, RecordError> {
        serde_json::to_string(&SavedRecord::from_record(self))
            .map_err(|e| RecordError::Json(e.to_string()))
    }

    /// Decode directly into strict typed fields, then validate the entire ledger.
    /// Parsing through a generic JSON value would erase duplicate object fields.
    pub fn from_json(json: &str) -> Result<Self, RecordError> {
        Self::from_json_with_game(json).map(|(record, _)| record)
    }

    /// Parse and validate once, retaining the reconstructed game for the caller.
    pub fn from_json_with_game(json: &str) -> Result<(Self, Game), RecordError> {
        crate::record::reject_duplicates(json).map_err(|e| RecordError::Json(e.to_string()))?;
        let wire: SavedRecord =
            serde_json::from_str(json).map_err(|e| RecordError::Json(e.to_string()))?;
        let game = wire.to_game()?;
        Ok((Self::from_game(&game), game))
    }

    /// Reconstruct the complete root and every recorded command through rules.
    pub fn to_game(&self) -> Result<Game, RecordError> {
        SavedRecord::from_record(self).to_game()
    }

    /// Whether an already validated record is an exact prefix of this record.
    /// This compares cache identity only; it does not admit input or reconstruct
    /// a game. Root provenance, policies, every event and the prefix's
    /// terminal result must agree. Use `prefix` when reconstruction is needed.
    pub fn has_prefix(&self, earlier: &Self) -> bool {
        let current = &self.wire;
        let previous = &earlier.wire;
        let count = previous.events.len();
        if count > current.events.len()
            || current.root != previous.root
            || current.events[..count] != previous.events
        {
            return false;
        }
        let (conclusion, score) = if count == 0 {
            (&current.root.conclusion, current.root.score_half_points)
        } else {
            let event = &current.events[count - 1];
            (&event.conclusion, event.score_half_points)
        };
        conclusion == &previous.conclusion && score == previous.score_half_points
    }

    /// Copy only new events for an incremental transport. The host must first
    /// establish the exact previous record with `has_prefix`; a count alone
    /// cannot establish that relationship. No replay or full ledger copy occurs.
    pub fn suffix(&self, start_event: usize) -> Result<GameDelta, RecordError> {
        if start_event > self.wire.events.len() {
            return Err(RecordError::Prefix);
        }
        Ok(GameDelta {
            start_event,
            events: self.wire.events[start_event..].to_vec(),
            conclusion: self.wire.conclusion.clone(),
            score_half_points: self.wire.score_half_points,
        })
    }

    /// Rebuild an event prefix; a same-board terminal command remains a distinct prefix.
    pub fn prefix(&self, event_count: usize) -> Result<Self, RecordError> {
        let game = self.to_game()?;
        let prefix = game.prefix(event_count).map_err(|error| match error {
            GamePrefixError::OutOfRange => RecordError::Prefix,
            GamePrefixError::Replay { event_index, error } => replay_error(event_index, error),
        })?;
        Ok(Self::from_game(&prefix))
    }
}

fn replay_error(event_index: usize, error: GameError) -> RecordError {
    match error {
        GameError::MutualCheckInvariant(diagnostic) => RecordError::Invariant {
            event_index,
            diagnostic,
        },
        _ => RecordError::Event(event_index),
    }
}

fn score(conclusion: Option<Conclusion>) -> Option<[u8; 2]> {
    conclusion.map(Conclusion::score_half_points)
}

// Nullable fields are still mandatory. Serde's ordinary Option decoding treats
// omission as null, which would silently accept a lossy record.
fn required_option<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    Option::<T>::deserialize(d)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CachedRecord {
    root: WireRoot,
    events: Vec<WireEvent>,
    conclusion: Option<WireConclusion>,
    score_half_points: Option<[u8; 2]>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedRecord {
    schema_version: u32,
    profile_id: String,
    profile_edition: String,
    root: SavedRoot,
    events: Vec<RecordedCommand<WireSide, WireCommand>>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedRoot {
    category: Category,
    sfen: String,
    counter_source: Source,
}
impl SavedRecord {
    fn from_record(record: &GameRecord) -> Self {
        let wire = &record.wire;
        Self {
            schema_version: SHOGI_RECORD_SCHEMA_VERSION,
            profile_id: PROFILE_ID.into(),
            profile_edition: super::PROFILE_EDITION.into(),
            root: SavedRoot {
                category: wire.root.category,
                sfen: wire.root.sfen.clone(),
                counter_source: wire.root.counter_source,
            },
            events: wire
                .events
                .iter()
                .map(|event| RecordedCommand {
                    actor: event.actor,
                    command: event.command.clone(),
                })
                .collect(),
        }
    }
    fn to_game(&self) -> Result<Game, RecordError> {
        if self.schema_version != SHOGI_RECORD_SCHEMA_VERSION
            || self.profile_id != PROFILE_ID
            || self.profile_edition != super::PROFILE_EDITION
        {
            return Err(RecordError::Unsupported);
        }
        let root = &self.root;
        let position = Position::from_sfen(&root.sfen).map_err(|_| RecordError::Root)?;
        if position.to_sfen() != root.sfen {
            return Err(RecordError::Root);
        }
        let root = match (root.category, root.counter_source) {
            (Category::Standard, Source::InitialFixed) if root.sfen == INITIAL_SFEN => {
                GameRoot::standard()
            }
            (
                category @ (Category::FullMaterialDiagram | Category::ReducedMaterialDiagram),
                source @ (Source::Supplied | Source::Defaulted),
            ) => GameRoot::import(
                position,
                if category == Category::FullMaterialDiagram {
                    RootCategory::FullMaterialDiagram
                } else {
                    RootCategory::ReducedMaterialDiagram
                },
                if source == Source::Supplied {
                    CounterSource::Supplied
                } else {
                    CounterSource::Defaulted
                },
            )
            .map_err(|_| RecordError::Root)?,
            _ => return Err(RecordError::Root),
        };
        // Parse the entire command list before executing any command.
        let commands = self
            .events
            .iter()
            .enumerate()
            .map(|(index, event)| event.command.to_command().ok_or(RecordError::Event(index)))
            .collect::<Result<Vec<_>, _>>()?;
        let mut game = Game::from_root(root);
        for (index, (event, command)) in self.events.iter().zip(commands).enumerate() {
            game.submit(GameRequest {
                expected: game.context(),
                actor: event.actor.into_side(),
                command,
            })
            .map_err(|error| replay_error(index, error))?;
        }
        Ok(game)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Category {
    Standard,
    FullMaterialDiagram,
    ReducedMaterialDiagram,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Source {
    InitialFixed,
    Supplied,
    Defaulted,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum WireSide {
    Sente,
    Gote,
}
impl WireSide {
    fn into_side(self) -> Side {
        match self {
            Self::Sente => Side::Sente,
            Self::Gote => Side::Gote,
        }
    }
}
impl From<Side> for WireSide {
    fn from(side: Side) -> Self {
        match side {
            Side::Sente => Self::Sente,
            Side::Gote => Self::Gote,
        }
    }
}
impl Serialize for Side {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        WireSide::from(*self).serialize(s)
    }
}
impl<'de> Deserialize<'de> for Side {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        WireSide::deserialize(d).map(WireSide::into_side)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WireRoot {
    category: Category,
    sfen: String,
    counter_source: Source,
    conclusion: Option<WireConclusion>,
    score_half_points: Option<[u8; 2]>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum WireCommand {
    Play { action: String },
    Resign {},
    DeclareEnteringKing {},
}
impl WireCommand {
    fn to_command(&self) -> Option<GameCommand> {
        Some(match self {
            Self::Play { action } => {
                let parsed: super::Action = action.parse().ok()?;
                if parsed.to_string() != *action {
                    return None;
                }
                GameCommand::Play(parsed)
            }
            Self::Resign {} => GameCommand::Resign,
            Self::DeclareEnteringKing {} => GameCommand::DeclareEnteringKing,
        })
    }
}
impl From<GameCommand> for WireCommand {
    fn from(command: GameCommand) -> Self {
        match command {
            GameCommand::Play(action) => Self::Play {
                action: action.to_string(),
            },
            GameCommand::Resign => Self::Resign {},
            GameCommand::DeclareEnteringKing => Self::DeclareEnteringKing {},
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireDeclaration {
    own_turn: bool,
    king_in_camp: bool,
    camp_pieces: u8,
    points: u16,
    required_points: u16,
    not_in_check: bool,
    time_available: bool,
}
impl From<DeclarationEligibility> for WireDeclaration {
    fn from(d: DeclarationEligibility) -> Self {
        Self {
            own_turn: d.own_turn,
            king_in_camp: d.king_in_camp,
            camp_pieces: d.camp_pieces,
            points: d.points,
            required_points: d.required_points,
            not_in_check: d.not_in_check,
            time_available: d.time_available,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum WireConclusion {
    NoLegalAction {
        winner: WireSide,
        in_check: bool,
    },
    ContinuousCheck {
        winner: WireSide,
        first: usize,
        fourth: usize,
    },
    Resignation {
        winner: WireSide,
    },
    EnteringKing {
        winner: WireSide,
    },
    FalseDeclaration {
        winner: WireSide,
    },
    Fourfold {
        first: usize,
        fourth: usize,
    },
    Length {},
}
impl From<Conclusion> for WireConclusion {
    fn from(conclusion: Conclusion) -> Self {
        match conclusion {
            Conclusion::Win { winner, reason } => {
                let winner = winner.into();
                match reason {
                    WinReason::NoLegalAction { in_check } => {
                        Self::NoLegalAction { winner, in_check }
                    }
                    WinReason::ContinuousCheck { first, fourth } => Self::ContinuousCheck {
                        winner,
                        first,
                        fourth,
                    },
                    WinReason::Resignation => Self::Resignation { winner },
                    WinReason::EnteringKing => Self::EnteringKing { winner },
                    WinReason::FalseDeclaration => Self::FalseDeclaration { winner },
                }
            }
            Conclusion::Draw(DrawReason::Fourfold { first, fourth }) => {
                Self::Fourfold { first, fourth }
            }
            Conclusion::Draw(DrawReason::Length) => Self::Length {},
        }
    }
}
impl WireConclusion {
    fn into_conclusion(self) -> Conclusion {
        match self {
            Self::NoLegalAction { winner, in_check } => Conclusion::Win {
                winner: winner.into_side(),
                reason: WinReason::NoLegalAction { in_check },
            },
            Self::ContinuousCheck {
                winner,
                first,
                fourth,
            } => Conclusion::Win {
                winner: winner.into_side(),
                reason: WinReason::ContinuousCheck { first, fourth },
            },
            Self::Resignation { winner } => Conclusion::Win {
                winner: winner.into_side(),
                reason: WinReason::Resignation,
            },
            Self::EnteringKing { winner } => Conclusion::Win {
                winner: winner.into_side(),
                reason: WinReason::EnteringKing,
            },
            Self::FalseDeclaration { winner } => Conclusion::Win {
                winner: winner.into_side(),
                reason: WinReason::FalseDeclaration,
            },
            Self::Fourfold { first, fourth } => {
                Conclusion::Draw(DrawReason::Fourfold { first, fourth })
            }
            Self::Length {} => Conclusion::Draw(DrawReason::Length),
        }
    }
}
impl Serialize for Conclusion {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        WireConclusion::from(*self).serialize(s)
    }
}
impl<'de> Deserialize<'de> for Conclusion {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        WireConclusion::deserialize(d).map(WireConclusion::into_conclusion)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireEvent {
    index: usize,
    actor: WireSide,
    command: WireCommand,
    #[serde(deserialize_with = "required_option")]
    gave_check: Option<bool>,
    #[serde(deserialize_with = "required_option")]
    declaration: Option<WireDeclaration>,
    after_sfen: String,
    #[serde(deserialize_with = "required_option")]
    conclusion: Option<WireConclusion>,
    #[serde(deserialize_with = "required_option")]
    score_half_points: Option<[u8; 2]>,
}
impl From<&GameEvent> for WireEvent {
    fn from(event: &GameEvent) -> Self {
        Self {
            index: event.index,
            actor: event.actor.into(),
            command: event.command.into(),
            gave_check: event.gave_check,
            declaration: event.declaration.map(WireDeclaration::from),
            after_sfen: event.after_sfen.clone(),
            conclusion: event.conclusion.map(WireConclusion::from),
            score_half_points: score(event.conclusion),
        }
    }
}

impl Serialize for GameRecord {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let json = self.to_json().map_err(serde::ser::Error::custom)?;
        crate::record::serialize_json(&json, serializer)
    }
}

#[cfg(test)]
mod prefix_identity_tests {
    use super::*;
    fn play(game: &mut Game, action: &str) {
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play(action.parse().unwrap()),
        })
        .unwrap();
    }
    #[test]
    fn exact_relation_agrees_with_reconstructed_prefixes() {
        let mut game = Game::new();
        let mut state = 173_u64;
        for _ in 0..24 {
            if game.conclusion().is_some() {
                break;
            }
            let actions = game.position().legal_actions();
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let action = actions[state as usize % actions.len()];
            game.submit(GameRequest {
                expected: game.context(),
                actor: game.position().side_to_move(),
                command: GameCommand::Play(action),
            })
            .unwrap();
        }
        let records: Vec<_> = (0..=game.events().len())
            .map(|n| GameRecord::from_game(&game.prefix(n).unwrap()))
            .collect();
        for (i, current) in records.iter().enumerate() {
            for (j, earlier) in records.iter().enumerate() {
                assert_eq!(current.has_prefix(earlier), j <= i);
                assert_eq!(
                    current.has_prefix(earlier),
                    current.prefix(j).is_ok_and(|r| &r == earlier)
                );
            }
        }
    }
    #[test]
    fn identity_rejects_same_board_different_histories_and_root_provenance() {
        let mut a = Game::new();
        let mut b = Game::new();
        for m in ["7g7f", "3c3d", "2g2f", "8c8d"] {
            play(&mut a, m);
        }
        for m in ["2g2f", "8c8d", "7g7f", "3c3d"] {
            play(&mut b, m);
        }
        assert_eq!(a.position(), b.position());
        let a = GameRecord::from_game(&a);
        let b = GameRecord::from_game(&b);
        assert!(!a.has_prefix(&b));
        assert!(!b.has_prefix(&a));
        let imported = Game::from_root(
            GameRoot::import(
                Position::initial(),
                RootCategory::FullMaterialDiagram,
                CounterSource::Supplied,
            )
            .unwrap(),
        );
        let imported = GameRecord::from_game(&imported);
        let standard = GameRecord::from_game(&Game::new());
        assert!(!standard.has_prefix(&imported));
        assert!(!imported.has_prefix(&standard));
    }
    #[test]
    fn terminal_command_and_empty_root_remain_distinct() {
        let mut game = Game::new();
        let empty = GameRecord::from_game(&game);
        assert!(empty.has_prefix(&empty));
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Resign,
        })
        .unwrap();
        let terminal = GameRecord::from_game(&game);
        assert_eq!(game.position(), Game::new().position());
        assert!(terminal.has_prefix(&empty));
        assert!(terminal.has_prefix(&terminal));
        assert!(!empty.has_prefix(&terminal));
        let mut other = Game::new();
        other
            .submit(GameRequest {
                expected: other.context(),
                actor: super::super::Side::Gote,
                command: GameCommand::Resign,
            })
            .unwrap();
        assert!(!terminal.has_prefix(&GameRecord::from_game(&other)));
        let root = Game::from_root(
            GameRoot::import(
                Position::from_sfen("8k/9/9/9/9/9/9/9/K8 b - 321").unwrap(),
                RootCategory::ReducedMaterialDiagram,
                CounterSource::Supplied,
            )
            .unwrap(),
        );
        let root = GameRecord::from_game(&root);
        assert!(root.has_prefix(&root));
        assert!(!root.has_prefix(&empty));
    }
}
