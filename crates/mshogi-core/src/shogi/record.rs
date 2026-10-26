//! Strict native records for the selected untimed profile.
//!
//! A record is a complete accepted ledger, not an SFEN recovery shortcut. Its
//! redundant effects are checked at every prefix by authoritative game replay.

use std::{error::Error, fmt};

use serde::{Deserialize, Deserializer, Serialize};

use super::{
    Conclusion, CounterSource, DeclarationEligibility, DrawReason, Game, GameCommand, GameError,
    GameEvent, GamePrefixError, GameRequest, GameRoot, INITIAL_SFEN, MutualCheckDiagnostic,
    PROFILE_ID, Position, RootCategory, Side, WinReason,
};

/// Bounded native input/output; independent of later console/archive transports.
pub const MAX_RECORD_BYTES: usize = 1024 * 1024;
/// Conservative ledger bound, including a possible terminal non-board command.
pub const MAX_RECORD_EVENTS: usize = 321;

/// Lossless canonical game and untimed match-command ledger.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRecord {
    wire: WireRecord,
}

/// Explicit rejection of malformed, unsupported or inconsistent native records.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordError {
    /// Input/output exceeds the native byte or event bound.
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
        /// Exact native candidate evidence.
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
    /// Export a rules-owned game, retaining imported-root provenance and every event.
    pub fn from_game(game: &Game) -> Self {
        let initial = Game::from_root(game.root().clone());
        Self {
            wire: WireRecord {
                format: "mshogi.shogi.game".into(),
                schema_version: 1,
                profile_id: PROFILE_ID.into(),
                profile_edition: "JSA-2025-10-01+CSA-2023-11-27-WCSC36-2026".into(),
                mode: "untimed".into(),
                policies: Policies::selected(),
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
                    known_history: if game.root().has_complete_initial_history() {
                        KnownHistory::Initial
                    } else {
                        KnownHistory::ImportedUnknown
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
        let json =
            serde_json::to_string(&self.wire).map_err(|e| RecordError::Json(e.to_string()))?;
        if json.len() > MAX_RECORD_BYTES || self.wire.events.len() > MAX_RECORD_EVENTS {
            return Err(RecordError::Capacity);
        }
        Ok(json)
    }

    /// Decode directly into strict typed fields, then validate the entire ledger.
    /// Parsing through a generic JSON value would erase duplicate object fields.
    pub fn from_json(json: &str) -> Result<Self, RecordError> {
        if json.len() > MAX_RECORD_BYTES {
            return Err(RecordError::Capacity);
        }
        let wire: WireRecord =
            serde_json::from_str(json).map_err(|e| RecordError::Json(e.to_string()))?;
        let record = Self { wire };
        record.to_game()?;
        Ok(record)
    }

    /// Reconstruct the root and every accepted command, checking every derived prefix.
    pub fn to_game(&self) -> Result<Game, RecordError> {
        let wire = &self.wire;
        if wire.events.len() > MAX_RECORD_EVENTS {
            return Err(RecordError::Capacity);
        }
        if wire.format != "mshogi.shogi.game"
            || wire.schema_version != 1
            || wire.profile_id != PROFILE_ID
            || wire.profile_edition != "JSA-2025-10-01+CSA-2023-11-27-WCSC36-2026"
            || wire.mode != "untimed"
            || wire.policies != Policies::selected()
        {
            return Err(RecordError::Unsupported);
        }
        let root = &wire.root;
        let position = Position::from_sfen(&root.sfen).map_err(|_| RecordError::Root)?;
        if position.to_sfen() != root.sfen {
            return Err(RecordError::Root);
        }
        let root = match (root.category, root.counter_source, root.known_history) {
            (Category::Standard, Source::InitialFixed, KnownHistory::Initial)
                if root.sfen == INITIAL_SFEN =>
            {
                GameRoot::standard()
            }
            (
                category @ (Category::FullMaterialDiagram | Category::ReducedMaterialDiagram),
                source @ (Source::Supplied | Source::Defaulted),
                KnownHistory::ImportedUnknown,
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
        // Check all command/position syntax before executing any recorded
        // command, including malformed data late in an otherwise valid ledger.
        let commands = wire
            .events
            .iter()
            .enumerate()
            .map(|(index, event)| {
                if event.index != index {
                    return Err(RecordError::Event(index));
                }
                let position = Position::from_sfen(&event.after_sfen)
                    .map_err(|_| RecordError::Event(index))?;
                if position.to_sfen() != event.after_sfen {
                    return Err(RecordError::Event(index));
                }
                event.command.to_native().ok_or(RecordError::Event(index))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut game = Game::from_root(root);
        if wire.root.conclusion != game.conclusion().map(WireConclusion::from)
            || wire.root.score_half_points != score(game.conclusion())
        {
            return Err(RecordError::Root);
        }
        for (index, (event, command)) in wire.events.iter().zip(commands).enumerate() {
            game.submit(GameRequest {
                expected: game.context(),
                actor: event.actor.native(),
                command,
            })
            .map_err(|error| replay_error(index, error))?;
            if *event != WireEvent::from(game.events().last().expect("just accepted event")) {
                return Err(RecordError::Event(index));
            }
        }
        if wire.conclusion != game.conclusion().map(WireConclusion::from)
            || wire.score_half_points != score(game.conclusion())
        {
            return Err(RecordError::Conclusion);
        }
        Ok(game)
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireRecord {
    format: String,
    schema_version: u32,
    profile_id: String,
    profile_edition: String,
    mode: String,
    policies: Policies,
    root: WireRoot,
    events: Vec<WireEvent>,
    #[serde(deserialize_with = "required_option")]
    conclusion: Option<WireConclusion>,
    #[serde(deserialize_with = "required_option")]
    score_half_points: Option<[u8; 2]>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Policies {
    no_legal_action: String,
    repetition: String,
    continuous_check: String,
    entering_king: String,
    sente_declaration_points: u16,
    gote_declaration_points: u16,
    minimum_other_camp_pieces: u8,
    length_completed_actions: u32,
    precedence: [String; 3],
    resignation: String,
    scoring: String,
    imported_history: String,
    counters: String,
    live_root_admission: String,
    timing: String,
    external_events: String,
}
impl Policies {
    fn selected() -> Self {
        Self {
            no_legal_action: "loss_with_or_without_check".into(),
            repetition: "four_known_occurrences_final_draw".into(),
            continuous_check: "all_own_actions_first_to_fourth_checker_loses".into(),
            entering_king: "formal_own_turn_camp_unchecked_false_declaration_loses".into(),
            sente_declaration_points: 28,
            gote_declaration_points: 27,
            minimum_other_camp_pieces: 10,
            length_completed_actions: 320,
            precedence: [
                "no_legal_action".into(),
                "repetition_including_continuous_check".into(),
                "length".into(),
            ],
            resignation: "opponent_wins".into(),
            scoring: "csa_sente_gote_half_point_units".into(),
            imported_history: "root_once_earlier_history_unknown".into(),
            counters: "declared_next_ply_minus_one_default_one".into(),
            live_root_admission: "nonmoving_king_unattacked_necessary_not_reachability".into(),
            timing: "unsupported_untimed_time_prerequisite_true".into(),
            external_events: "unsupported".into(),
        }
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
enum KnownHistory {
    Initial,
    ImportedUnknown,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum WireSide {
    Sente,
    Gote,
}
impl WireSide {
    fn native(self) -> Side {
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireRoot {
    category: Category,
    sfen: String,
    counter_source: Source,
    known_history: KnownHistory,
    #[serde(deserialize_with = "required_option")]
    conclusion: Option<WireConclusion>,
    #[serde(deserialize_with = "required_option")]
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
    fn to_native(&self) -> Option<GameCommand> {
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

// Capacity argument for all rules-owned ledgers: live roots below the length
// limit admit at most 320 board actions, or 319 plus a terminal command. Roots
// already at the limit admit zero events. The conservative 321-event bound is
// therefore never an extra restriction on accepted play. Canonical SFEN has at
// most 40 pieces (80 piece bytes), rank separators/empty runs, 14 bounded hand
// entries, side and u32 counter; it fits 256 bytes. A serialized event has only
// fixed strings, one <=5-byte action, that SFEN, <=321 indices, bounded point
// counts, and optional fixed-shaped declaration/conclusion: <2048 bytes. The
// fixed envelope/root is <8192 bytes. Thus 8192 + 321*2048 = 665600 < 1 MiB.
// These deliberately loose limits include all supported encoded alternatives.
