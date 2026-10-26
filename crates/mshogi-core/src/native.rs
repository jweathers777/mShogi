//! Explicit two-game dispatch; native rules and record formats remain authoritative.
//!
//! Chess history observations are data, not commands to execute. Complete chess
//! match admission still belongs to `ChessMatch`; shogi admission belongs to `Game`.

use crate::{chess, shogi};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
use serde_json::value::RawValue;
use std::{error::Error, fmt};

/// This wrapper's version, independent of native schemas and engine transport.
pub const RECORD_ENVELOPE_VERSION: u32 = 1;
/// Encoded canonical wrapper input/output limit; request overhead is counted separately.
pub const MAX_RECORD_ENVELOPE_BYTES: usize = 1024 * 1024;
/// Bounded shared chess history context; native records themselves retain their own contract.
pub const MAX_SHARED_CHESS_EVENTS: usize = 4096;
/// Maximum retained shared-record diagnostic bytes.
pub const MAX_RECORD_DIAGNOSTIC_BYTES: usize = 4096;
/// The source edition accepted by the existing chess match profile.
pub const CHESS_PROFILE_EDITION: &str = "FIDE-2023-01-01+mshogi.chess.match.v1";

/// Actual native game; this does not advertise execution support.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameKind {
    /// Orthodox chess under the existing native profile.
    Chess,
    /// Standard shogi under the approved CSA profile.
    Shogi,
}
/// Immutable profile identity, never inferred from board dimensions or sides.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RulesProfile {
    /// Existing chess match/history semantics, including recorded mode and policy.
    ChessMatchV1,
    /// Selected standard-shogi CSA software convention.
    ShogiCsa36V1,
}
impl RulesProfile {
    /// Exact public profile ID.
    pub const fn id(self) -> &'static str {
        match self {
            Self::ChessMatchV1 => crate::CHESS_MATCH_PROFILE_ID,
            Self::ShogiCsa36V1 => shogi::PROFILE_ID,
        }
    }
    /// Exact accepted edition; profile revisions cannot silently advance it.
    pub const fn edition(self) -> &'static str {
        match self {
            Self::ChessMatchV1 => CHESS_PROFILE_EDITION,
            Self::ShogiCsa36V1 => shogi::PROFILE_EDITION,
        }
    }
    /// Game to which this profile belongs.
    pub const fn game(self) -> GameKind {
        match self {
            Self::ChessMatchV1 => GameKind::Chess,
            Self::ShogiCsa36V1 => GameKind::Shogi,
        }
    }
}
impl Serialize for RulesProfile {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.id())
    }
}
impl<'de> Deserialize<'de> for RulesProfile {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match String::deserialize(deserializer)?.as_str() {
            crate::CHESS_MATCH_PROFILE_ID => Ok(Self::ChessMatchV1),
            shogi::PROFILE_ID => Ok(Self::ShogiCsa36V1),
            _ => Err(serde::de::Error::custom("unsupported native rules profile")),
        }
    }
}

/// Native schema meaning, rather than an unqualified number shared across games.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeSchema {
    /// Existing chess M2 history schema one.
    ChessM2,
    /// Existing chess completeness-mode history schema two.
    ChessFull,
    /// Approved shogi game/match schema one.
    ShogiV1,
}
impl NativeSchema {
    /// Native wire number, scoped to the game/profile.
    pub const fn version(self) -> u32 {
        match self {
            Self::ChessM2 => chess::CHESS_RECORD_SCHEMA_VERSION,
            Self::ChessFull => chess::CHESS_FULL_RECORD_SCHEMA_VERSION,
            Self::ShogiV1 => shogi::SHOGI_RECORD_SCHEMA_VERSION,
        }
    }
}

/// Complete native records. Metadata is derived, so mutable outer tags cannot drift.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalRecord {
    /// Chess history/observation record; not an implicitly reconstructed match.
    Chess(Box<chess::GameRecord>),
    /// Authoritative accepted shogi game/command ledger.
    Shogi(Box<shogi::GameRecord>),
}
impl CanonicalRecord {
    /// Wrap the actual native chess payload without changing historical observations.
    pub fn chess(record: chess::GameRecord) -> Self {
        Self::Chess(Box::new(record))
    }
    /// Wrap the actual native shogi ledger without losing root/history provenance.
    pub fn shogi(record: shogi::GameRecord) -> Self {
        Self::Shogi(Box::new(record))
    }
    /// Exact native profile.
    pub const fn profile(&self) -> RulesProfile {
        match self {
            Self::Chess(_) => RulesProfile::ChessMatchV1,
            Self::Shogi(_) => RulesProfile::ShogiCsa36V1,
        }
    }
    /// Actual game, independent of execution capability.
    pub const fn game_kind(&self) -> GameKind {
        self.profile().game()
    }
    /// Complete event count, including terminal commands and historical observations.
    pub fn event_count(&self) -> usize {
        match self {
            Self::Chess(record) => record.events().len(),
            Self::Shogi(record) => record.event_count(),
        }
    }
    /// Board actions in this known segment, not imported declared counters.
    pub fn known_board_actions(&self) -> usize {
        match self {
            Self::Chess(record) => record
                .events()
                .iter()
                .filter(|e| matches!(e.kind, chess::RecordEventKind::Move(_)))
                .count(),
            Self::Shogi(record) => record.known_board_actions(),
        }
    }
    /// Schema derived from the authoritative payload.
    pub fn native_schema(&self) -> NativeSchema {
        match self {
            Self::Chess(record)
                if record.schema_version() == chess::CHESS_RECORD_SCHEMA_VERSION =>
            {
                NativeSchema::ChessM2
            }
            Self::Chess(_) => NativeSchema::ChessFull,
            Self::Shogi(_) => NativeSchema::ShogiV1,
        }
    }
    /// Serialize a complete bounded wrapper. Native encoding is unchanged.
    pub fn to_json(&self) -> Result<String, EnvelopeError> {
        self.payload_json()?;
        let text = serde_json::to_string(self).map_err(EnvelopeError::encoding)?;
        if text.len() > MAX_RECORD_ENVELOPE_BYTES {
            return Err(EnvelopeError::Capacity);
        }
        Ok(text)
    }
    /// Strict outer parsing followed by the selected strict native parser.
    pub fn from_json(text: &str) -> Result<Self, EnvelopeError> {
        if text.len() > MAX_RECORD_ENVELOPE_BYTES {
            return Err(EnvelopeError::Capacity);
        }
        let wire: WireRecord = serde_json::from_str(text).map_err(EnvelopeError::encoding)?;
        if wire.envelope_version != RECORD_ENVELOPE_VERSION
            || wire.rules_profile.game() != wire.game_kind
            || wire.profile_edition != wire.rules_profile.edition()
        {
            return Err(EnvelopeError::Identity);
        }
        let record = match wire.game_kind {
            GameKind::Chess => Self::chess(
                chess::GameRecord::from_json(wire.payload.get())
                    .map_err(EnvelopeError::encoding)?,
            ),
            GameKind::Shogi => Self::shogi(
                shogi::GameRecord::from_json(wire.payload.get())
                    .map_err(EnvelopeError::encoding)?,
            ),
        };
        if record.native_schema().version() != wire.native_schema_version {
            return Err(EnvelopeError::Identity);
        }
        record.payload_json()?;
        Ok(record)
    }
    fn payload_json(&self) -> Result<String, EnvelopeError> {
        let text = match self {
            Self::Chess(record) => {
                if record.events().len() > MAX_SHARED_CHESS_EVENTS {
                    return Err(EnvelopeError::Capacity);
                }
                record.to_json()
            }
            Self::Shogi(record) => record.to_json().map_err(EnvelopeError::encoding)?,
        };
        if text.len() > MAX_RECORD_ENVELOPE_BYTES {
            return Err(EnvelopeError::Capacity);
        }
        Ok(text)
    }
}
impl Serialize for CanonicalRecord {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let payload =
            RawValue::from_string(self.payload_json().map_err(serde::ser::Error::custom)?)
                .map_err(serde::ser::Error::custom)?;
        let mut wire = serializer.serialize_struct("CanonicalRecord", 6)?;
        wire.serialize_field("envelope_version", &RECORD_ENVELOPE_VERSION)?;
        wire.serialize_field("game_kind", &self.game_kind())?;
        wire.serialize_field("rules_profile", &self.profile())?;
        wire.serialize_field("profile_edition", self.profile().edition())?;
        wire.serialize_field("native_schema_version", &self.native_schema().version())?;
        wire.serialize_field("payload", &payload)?;
        wire.end()
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireRecord {
    envelope_version: u32,
    game_kind: GameKind,
    rules_profile: RulesProfile,
    profile_edition: String,
    native_schema_version: u32,
    payload: Box<RawValue>,
}
impl<'de> Deserialize<'de> for CanonicalRecord {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Capture all wrapper bytes: no intermediate map or tagged-enum buffer
        // may erase duplicate outer or nested native fields.
        let raw = Box::<RawValue>::deserialize(deserializer)?;
        Self::from_json(raw.get()).map_err(serde::de::Error::custom)
    }
}
/// Explicit unsupported identity, malformed input or resource rejection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnvelopeError {
    /// Encoded bytes or supported shared event count exceeded.
    Capacity,
    /// Envelope, game, profile, edition or native schema disagree.
    Identity,
    /// Strict outer/native syntax rejection, with bounded diagnostics.
    Encoding(String),
}
impl EnvelopeError {
    fn encoding(error: impl fmt::Display) -> Self {
        let mut text = error.to_string();
        if text.len() > MAX_RECORD_DIAGNOSTIC_BYTES {
            let mut end = MAX_RECORD_DIAGNOSTIC_BYTES;
            while !text.is_char_boundary(end) {
                end -= 1;
            }
            text.truncate(end);
        }
        Self::Encoding(text)
    }
}
impl fmt::Display for EnvelopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "native record envelope rejected: {self:?}")
    }
}
impl Error for EnvelopeError {}

/// Game-qualified sides retain native interpretation; no White/Sente coercion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeSide {
    /// Chess White or Black.
    Chess(chess::Color),
    /// Shogi Sente or Gote.
    Shogi(shogi::Side),
}
impl NativeSide {
    /// Side's game, for validating purpose metadata without replay.
    pub const fn game(self) -> GameKind {
        match self {
            Self::Chess(_) => GameKind::Chess,
            Self::Shogi(_) => GameKind::Shogi,
        }
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireSide {
    game_kind: GameKind,
    side: String,
}
impl Serialize for NativeSide {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        WireSide {
            game_kind: self.game(),
            side: match self {
                Self::Chess(chess::Color::White) => "white",
                Self::Chess(chess::Color::Black) => "black",
                Self::Shogi(shogi::Side::Sente) => "sente",
                Self::Shogi(shogi::Side::Gote) => "gote",
            }
            .into(),
        }
        .serialize(s)
    }
}
impl<'de> Deserialize<'de> for NativeSide {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let w = WireSide::deserialize(d)?;
        match (w.game_kind, w.side.as_str()) {
            (GameKind::Chess, "white") => Ok(Self::Chess(chess::Color::White)),
            (GameKind::Chess, "black") => Ok(Self::Chess(chess::Color::Black)),
            (GameKind::Shogi, "sente") => Ok(Self::Shogi(shogi::Side::Sente)),
            (GameKind::Shogi, "gote") => Ok(Self::Shogi(shogi::Side::Gote)),
            _ => Err(serde::de::Error::custom(
                "side does not belong to named game",
            )),
        }
    }
}

/// Borrow an existing authoritative owner; no match is fabricated from a history record.
pub enum NativeCommandTarget<'a> {
    /// Complete chess match, including clocks, offers and native version admission.
    Chess(&'a mut chess::ChessMatch),
    /// Authoritative untimed shogi game and accepted command ledger.
    Shogi(&'a mut shogi::Game),
}
/// Native commands keep game-specific actors, actions and preconditions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NativeCommand {
    /// Chess command, with native timestamp/version and optional claim intention.
    Chess(chess::MatchRequest),
    /// Shogi action, formal declaration or resignation with complete expected context.
    Shogi(Box<shogi::GameRequest>),
}
/// Native admission result, preserving rules-owned semantics and errors.
#[derive(Debug)]
pub enum NativeReply {
    /// Complete chess acceptance/rejection, clock and conclusion observations.
    Chess(chess::MatchReply),
    /// Exact shogi reversal token or atomic native rejection/internal diagnostic.
    Shogi(Result<Box<shogi::GameUndo>, shogi::GameError>),
}
/// A wrong-game command is rejected before either native owner is touched.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DispatchMismatch;
/// Dispatch only by typed game tag. All actor, turn and command admission is native.
pub fn dispatch_command(
    target: NativeCommandTarget<'_>,
    command: NativeCommand,
) -> Result<NativeReply, DispatchMismatch> {
    match (target, command) {
        (NativeCommandTarget::Chess(game), NativeCommand::Chess(request)) => {
            Ok(NativeReply::Chess(game.submit(request)))
        }
        (NativeCommandTarget::Shogi(game), NativeCommand::Shogi(request)) => {
            Ok(NativeReply::Shogi(game.submit(*request).map(Box::new)))
        }
        _ => Err(DispatchMismatch),
    }
}
