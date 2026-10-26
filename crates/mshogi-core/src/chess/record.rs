//! Versioned chess history records. Observation replay does not adjudicate matches.

use super::{
    Action, ActionError, ActionParseError, AdjudicationMode, ClaimPolicy, Color, FenError, Game,
    HistoryOrigin, Position, STANDARD_START_FEN,
};
use crate::CHESS_MATCH_PROFILE_ID;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

/// Version of the chess history record encoding, distinct from rules identity.
pub const CHESS_RECORD_SCHEMA_VERSION: u32 = 1;
/// Opt-in completeness configuration encoding; schema 1 remains unchanged.
pub const CHESS_FULL_RECORD_SCHEMA_VERSION: u32 = 2;

/// Draw claim named in a recorded request, not proof that it was valid.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrawClaim {
    /// Threefold repetition, current or intended action.
    Threefold,
    /// Fifty-move rule, current or intended action.
    FiftyMove,
}

/// Command data retained for future semantic replay by MSH-17/18.
/// A request here is not an accepted command or a concluded game outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordedCommand {
    /// Resignation request.
    Resign,
    /// Draw offer.
    OfferDraw,
    /// Acceptance of a draw offer.
    AcceptDraw,
    /// Rejection of a draw offer.
    DeclineDraw,
    /// Optional claim, with an intended action kept out of played history.
    ClaimDraw {
        /// Which claim the actor requests.
        reason: DrawClaim,
        /// Proposed action, if this is an intended-move claim.
        intended: Option<Action>,
    },
}

/// Match data observed at a particular board-history context, not adjudication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatchObservation {
    /// A command request; validity and effects are checked in MSH-17/18.
    Command {
        /// Actor identified independently of the side to move.
        actor: Color,
        /// Complete requested command.
        command: RecordedCommand,
    },
    /// Clock values in integer nanoseconds. This layer preserves observations;
    /// it does not validate elapsed-time accounting, ordering or flag decisions.
    Clock {
        /// Monotonic source timestamp in the match's recorded time origin.
        at_ns: u64,
        /// Remaining time, White then Black (zero may represent expiry).
        remaining_ns: [u64; 2],
        /// Running side, or None if stopped/untimed.
        active: Option<Color>,
    },
}

/// Payload of one totally ordered record event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordEventKind {
    /// A complete played action; effects are always derived by rules replay.
    Move(Action),
    /// Command/clock observation, preserved without executing its semantics.
    Observation(MatchObservation),
}

/// Event's declared action context. Array order supplies the independent event
/// sequence; several observations may share the same action ply.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordEvent {
    /// Number of played actions before this event, checked during history replay.
    pub ply: u64,
    /// Action or observation.
    pub kind: RecordEventKind,
}

/// Chess-only public record with no private undo, repetition table or effects.
/// Parsed/built events are untrusted until `replay_history` checks their actions
/// and contexts. Observation semantics are explicitly a later layer's contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRecord {
    initial: Position,
    origin: HistoryOrigin,
    policy: ClaimPolicy,
    events: Vec<RecordEvent>,
    adjudication_mode: AdjudicationMode,
}

/// An observation with verified action context and its one-based event sequence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextualObservation {
    /// Position in the complete record's event array, starting at one.
    pub sequence: usize,
    /// Played action count at observation time.
    pub ply: u64,
    /// Validated position at observation time, including counters/raw EP.
    pub position: Position,
    /// Preserved data, not a declaration that a command/clock was valid.
    pub observation: MatchObservation,
}

/// Reconstructed action history plus observations. No match outcome is implied.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoryReplay {
    /// Rules-owned game rebuilt by applying complete actions.
    pub game: Game,
    /// Observations in total event order, including multiple at the same ply.
    pub observations: Vec<ContextualObservation>,
}

/// Invalid record syntax, metadata, action or action context. No partial game
/// is returned on failure. Event numbers in errors are one-based.
#[derive(Debug)]
pub enum RecordError {
    /// Invalid JSON, unknown/duplicate fields or an unsupported enum value.
    Json(serde_json::Error),
    /// Unsupported wire schema.
    Schema(u32),
    /// Unsupported rule profile.
    Profile(String),
    /// Initial FEN violates the position contract.
    Initial(FenError),
    /// A known standard origin was asserted for a different initial state.
    Origin,
    /// Color token is neither white nor black.
    Color(usize),
    /// An action token is malformed.
    ActionSyntax(usize, ActionParseError),
    /// An event's supplied action ply differs from reconstructed history.
    Context(usize),
    /// A played action cannot be applied at its history context.
    Action(usize, ActionError),
}
impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(e) => write!(f, "record JSON: {e}"),
            Self::Schema(v) => write!(f, "unsupported chess record schema {v}"),
            Self::Profile(p) => write!(f, "unsupported chess record profile {p:?}"),
            Self::Initial(e) => write!(f, "record initial position: {e}"),
            Self::Origin => {
                f.write_str("standard-start origin requires the exact standard initial state")
            }
            Self::Color(n) => write!(f, "record event {n}: expected white or black"),
            Self::ActionSyntax(n, e) => write!(f, "record event {n}: {e}"),
            Self::Context(n) => write!(f, "record event {n}: action ply does not match history"),
            Self::Action(n, e) => write!(f, "record event {n}: {e}"),
        }
    }
}
impl Error for RecordError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Json(e) => Some(e),
            Self::Initial(e) => Some(e),
            Self::ActionSyntax(_, e) => Some(e),
            Self::Action(_, e) => Some(e),
            _ => None,
        }
    }
}

impl Game {
    /// Export the current analysis branch as complete actions, never undo state.
    pub fn record(&self) -> GameRecord {
        GameRecord {
            initial: self.initial_position().clone(),
            origin: self.history_origin(),
            policy: self.claim_policy(),
            adjudication_mode: AdjudicationMode::M2,
            events: self
                .actions()
                .enumerate()
                .map(|(ply, a)| RecordEvent {
                    ply: ply as u64,
                    kind: RecordEventKind::Move(a),
                })
                .collect(),
        }
    }
}

impl GameRecord {
    /// Native encoding identity, independent of transport/envelope versions.
    pub const fn schema_version(&self) -> u32 {
        match self.adjudication_mode {
            AdjudicationMode::M2 => CHESS_RECORD_SCHEMA_VERSION,
            AdjudicationMode::Full { .. } => CHESS_FULL_RECORD_SCHEMA_VERSION,
        }
    }
    /// Exact event prefix, preserving initial state, policy and mode.
    pub fn prefix(&self, count: usize) -> Option<Self> {
        if count > self.events.len() {
            return None;
        }
        let mut prefix = self.clone();
        prefix.events.truncate(count);
        Some(prefix)
    }
    /// Live adjudication mode; diagnostic history replay does not execute it.
    pub const fn adjudication_mode(&self) -> AdjudicationMode {
        self.adjudication_mode
    }
    pub(super) fn set_adjudication_mode(&mut self, mode: AdjudicationMode) {
        self.adjudication_mode = mode;
    }
    /// Initial position with all six saved fields.
    pub const fn initial_position(&self) -> &Position {
        &self.initial
    }
    /// History provenance, never inferred from counters or diagram.
    pub const fn history_origin(&self) -> HistoryOrigin {
        self.origin
    }
    /// Selected policy, retained without applying claims during reconstruction.
    pub const fn claim_policy(&self) -> ClaimPolicy {
        self.policy
    }
    /// Ordered actions and observations, with explicit action contexts.
    pub fn events(&self) -> &[RecordEvent] {
        &self.events
    }
    /// Append untrusted data. History replay validates context and played actions.
    pub fn push_event(&mut self, event: RecordEvent) {
        self.events.push(event);
    }

    /// Canonical compact JSON: fixed field/array order, canonical FEN/coordinates,
    /// no newline. JSON whitespace on input does not affect canonical output.
    pub fn to_json(&self) -> String {
        let events = self
            .events
            .iter()
            .map(|e| WireEntry {
                ply: e.ply,
                event: match &e.kind {
                    RecordEventKind::Move(a) => WireEvent::Move {
                        action: a.to_string(),
                    },
                    RecordEventKind::Observation(MatchObservation::Command { actor, command }) => {
                        WireEvent::Command {
                            actor: color(*actor).into(),
                            command: match command {
                                RecordedCommand::Resign => WireCommand::Resign {},
                                RecordedCommand::OfferDraw => WireCommand::OfferDraw {},
                                RecordedCommand::AcceptDraw => WireCommand::AcceptDraw {},
                                RecordedCommand::DeclineDraw => WireCommand::DeclineDraw {},
                                RecordedCommand::ClaimDraw { reason, intended } => {
                                    WireCommand::ClaimDraw {
                                        reason: *reason,
                                        intended: intended.map(|a| a.to_string()),
                                    }
                                }
                            },
                        }
                    }
                    RecordEventKind::Observation(MatchObservation::Clock {
                        at_ns,
                        remaining_ns,
                        active,
                    }) => WireEvent::Clock {
                        at_ns: *at_ns,
                        remaining_ns: *remaining_ns,
                        active: active.map(|c| color(c).into()),
                    },
                },
            })
            .collect();
        serde_json::to_string(&WireRecord {
            schema_version: if self.adjudication_mode == AdjudicationMode::M2 {
                CHESS_RECORD_SCHEMA_VERSION
            } else {
                CHESS_FULL_RECORD_SCHEMA_VERSION
            },
            adjudication_mode: if self.adjudication_mode == AdjudicationMode::M2 {
                None
            } else {
                Some(self.adjudication_mode)
            },
            rules_profile: CHESS_MATCH_PROFILE_ID.into(),
            initial_fen: self.initial.to_fen(),
            history_origin: self.origin,
            claim_policy: self.policy,
            events,
        })
        .expect("record contains only serializable finite primitives")
    }

    /// Decode strict versioned data. Does not silently accept unknown fields or
    /// treat syntactically valid played actions as legal: call `replay_history`.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        let wire: WireRecord = serde_json::from_str(text).map_err(RecordError::Json)?;
        if !matches!(
            (wire.schema_version, wire.adjudication_mode),
            (CHESS_RECORD_SCHEMA_VERSION, None)
                | (
                    CHESS_FULL_RECORD_SCHEMA_VERSION,
                    Some(AdjudicationMode::Full { .. })
                )
        ) {
            return Err(RecordError::Schema(wire.schema_version));
        }
        if wire.rules_profile != CHESS_MATCH_PROFILE_ID {
            return Err(RecordError::Profile(wire.rules_profile));
        }
        let initial = Position::from_fen(&wire.initial_fen).map_err(RecordError::Initial)?;
        if wire.history_origin == HistoryOrigin::StandardStart
            && initial.to_fen() != STANDARD_START_FEN
        {
            return Err(RecordError::Origin);
        }
        let mut events = Vec::with_capacity(wire.events.len());
        for (index, e) in wire.events.into_iter().enumerate() {
            let n = index + 1;
            let parse = |s: &str| {
                s.parse::<Action>()
                    .map_err(|err| RecordError::ActionSyntax(n, err))
            };
            let parse_color = |s: &str| match s {
                "white" => Ok(Color::White),
                "black" => Ok(Color::Black),
                _ => Err(RecordError::Color(n)),
            };
            let kind = match e.event {
                WireEvent::Move { action } => RecordEventKind::Move(parse(&action)?),
                WireEvent::Command { actor, command } => {
                    RecordEventKind::Observation(MatchObservation::Command {
                        actor: parse_color(&actor)?,
                        command: match command {
                            WireCommand::Resign {} => RecordedCommand::Resign,
                            WireCommand::OfferDraw {} => RecordedCommand::OfferDraw,
                            WireCommand::AcceptDraw {} => RecordedCommand::AcceptDraw,
                            WireCommand::DeclineDraw {} => RecordedCommand::DeclineDraw,
                            WireCommand::ClaimDraw { reason, intended } => {
                                RecordedCommand::ClaimDraw {
                                    reason,
                                    intended: intended.map(|s| parse(&s)).transpose()?,
                                }
                            }
                        },
                    })
                }
                WireEvent::Clock {
                    at_ns,
                    remaining_ns,
                    active,
                } => RecordEventKind::Observation(MatchObservation::Clock {
                    at_ns,
                    remaining_ns,
                    active: active.map(|s| parse_color(&s)).transpose()?,
                }),
            };
            events.push(RecordEvent { ply: e.ply, kind });
        }
        Ok(Self {
            initial,
            origin: wire.history_origin,
            policy: wire.claim_policy,
            adjudication_mode: wire.adjudication_mode.unwrap_or_default(),
            events,
        })
    }

    /// Reconstruct all played actions and observation contexts deterministically.
    /// Does not exercise stored policy or validate/execute observed claims,
    /// resignations, offers, clocks or outcomes. MSH-17/18 own those semantics.
    /// This method returns no partial replay on a played-action/context failure.
    pub fn replay_history(&self) -> Result<HistoryReplay, RecordError> {
        let mut game = Game::from_initial(self.initial.clone(), self.origin, self.policy);
        let mut observations = Vec::new();
        for (index, event) in self.events.iter().enumerate() {
            let sequence = index + 1;
            if event.ply != game.ply_count() as u64 {
                return Err(RecordError::Context(sequence));
            }
            match &event.kind {
                RecordEventKind::Move(action) => game
                    .apply(*action)
                    .map_err(|e| RecordError::Action(sequence, e))?,
                RecordEventKind::Observation(observation) => {
                    observations.push(ContextualObservation {
                        sequence,
                        ply: event.ply,
                        position: game.position().clone(),
                        observation: observation.clone(),
                    })
                }
            }
        }
        Ok(HistoryReplay { game, observations })
    }
}

fn color(c: Color) -> &'static str {
    match c {
        Color::White => "white",
        Color::Black => "black",
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireRecord {
    schema_version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    adjudication_mode: Option<AdjudicationMode>,
    rules_profile: String,
    initial_fen: String,
    history_origin: HistoryOrigin,
    claim_policy: ClaimPolicy,
    events: Vec<WireEntry>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireEntry {
    ply: u64,
    event: WireEvent,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum WireEvent {
    Move {
        action: String,
    },
    Command {
        actor: String,
        command: WireCommand,
    },
    Clock {
        at_ns: u64,
        remaining_ns: [u64; 2],
        active: Option<String>,
    },
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum WireCommand {
    Resign {},
    OfferDraw {},
    AcceptDraw {},
    DeclineDraw {},
    ClaimDraw {
        reason: DrawClaim,
        intended: Option<String>,
    },
}
