//! Explicit native-game dispatch; native rules and record formats remain authoritative.
//!
//! Chess history observations are data, not commands to execute. Complete chess
//! match admission still belongs to `ChessMatch`; shogi admission belongs to `Game`.

use crate::{chess, chu, shogi, tenjiku, tori, xiangqi};
use serde::{Deserialize, Serialize, Serializer};
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
    /// Xiangqi under the approved WXF profile.
    Xiangqi,
    /// Tori Shogi under the approved modern profile.
    Tori,
    /// Chu Shogi under the approved JCSA profile.
    Chu,
    /// Tenjiku Shogi, with its immutable whole profile selected separately.
    Tenjiku,
}
/// Immutable profile identity, never inferred from board dimensions or sides.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RulesProfile {
    /// Existing chess match/history semantics, including recorded mode and policy.
    ChessMatchV1,
    /// Selected standard-shogi CSA software convention.
    ShogiCsa36V1,
    /// Selected bilingual WXF2018 software profile.
    XiangqiWxf2018V1,
    /// Selected modern Tori software profile.
    ToriModernV1,
    /// Selected modern JCSA Chu profile.
    ChuJcsa2019V1,
    /// Ratified Adams1999 Modern complete Tenjiku profile.
    TenjikuAdamsModernV1,
    /// Ratified Muller Tenjiku description with named completion clauses.
    TenjikuMullerCompletedV1,
}
impl RulesProfile {
    /// Exact public profile ID.
    pub const fn id(self) -> &'static str {
        match self {
            Self::ChessMatchV1 => crate::CHESS_MATCH_PROFILE_ID,
            Self::ShogiCsa36V1 => shogi::PROFILE_ID,
            Self::XiangqiWxf2018V1 => xiangqi::PROFILE_ID,
            Self::ToriModernV1 => tori::PROFILE_ID,
            Self::ChuJcsa2019V1 => chu::PROFILE_ID,
            Self::TenjikuAdamsModernV1 => tenjiku::Profile::AdamsModern.id(),
            Self::TenjikuMullerCompletedV1 => tenjiku::Profile::MullerCompleted.id(),
        }
    }
    /// Exact accepted edition; profile revisions cannot silently advance it.
    pub const fn edition(self) -> &'static str {
        match self {
            Self::ChessMatchV1 => CHESS_PROFILE_EDITION,
            Self::ShogiCsa36V1 => shogi::PROFILE_EDITION,
            Self::XiangqiWxf2018V1 => xiangqi::PROFILE_EDITION,
            Self::ToriModernV1 => tori::PROFILE_EDITION,
            Self::ChuJcsa2019V1 => chu::PROFILE_EDITION,
            Self::TenjikuAdamsModernV1 => tenjiku::Profile::AdamsModern.edition(),
            Self::TenjikuMullerCompletedV1 => tenjiku::Profile::MullerCompleted.edition(),
        }
    }
    /// Game to which this profile belongs.
    pub const fn game(self) -> GameKind {
        match self {
            Self::ChessMatchV1 => GameKind::Chess,
            Self::ShogiCsa36V1 => GameKind::Shogi,
            Self::XiangqiWxf2018V1 => GameKind::Xiangqi,
            Self::ToriModernV1 => GameKind::Tori,
            Self::ChuJcsa2019V1 => GameKind::Chu,
            Self::TenjikuAdamsModernV1 | Self::TenjikuMullerCompletedV1 => GameKind::Tenjiku,
        }
    }
    /// Exact native Tenjiku choice; other games have no implicit Tenjiku default.
    pub const fn tenjiku(self) -> Option<tenjiku::Profile> {
        match self {
            Self::TenjikuAdamsModernV1 => Some(tenjiku::Profile::AdamsModern),
            Self::TenjikuMullerCompletedV1 => Some(tenjiku::Profile::MullerCompleted),
            _ => None,
        }
    }
}
impl From<tenjiku::Profile> for RulesProfile {
    fn from(profile: tenjiku::Profile) -> Self {
        match profile {
            tenjiku::Profile::AdamsModern => Self::TenjikuAdamsModernV1,
            tenjiku::Profile::MullerCompleted => Self::TenjikuMullerCompletedV1,
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
            xiangqi::PROFILE_ID => Ok(Self::XiangqiWxf2018V1),
            tori::PROFILE_ID => Ok(Self::ToriModernV1),
            chu::PROFILE_ID => Ok(Self::ChuJcsa2019V1),
            "mshogi.tenjiku.adams1999-modern.v1" => Ok(Self::TenjikuAdamsModernV1),
            "mshogi.tenjiku.muller-completed.v1" => Ok(Self::TenjikuMullerCompletedV1),
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
    /// Xiangqi native history/command/event schema one.
    XiangqiV1,
    /// Complete modern Tori native history schema one.
    ToriV1,
    /// Complete native Chu history schema one.
    ChuV1,
    /// Tenjiku native schema one, qualified independently by either whole profile.
    TenjikuV1,
}
impl NativeSchema {
    /// Native wire number, scoped to the game/profile.
    pub const fn version(self) -> u32 {
        match self {
            Self::ChessM2 => chess::CHESS_RECORD_SCHEMA_VERSION,
            Self::ChessFull => chess::CHESS_FULL_RECORD_SCHEMA_VERSION,
            Self::ShogiV1 => shogi::SHOGI_RECORD_SCHEMA_VERSION,
            Self::XiangqiV1 => xiangqi::GAME_RECORD_SCHEMA_VERSION,
            Self::ToriV1 => tori::GAME_RECORD_SCHEMA_VERSION,
            Self::ChuV1 => chu::GAME_RECORD_SCHEMA_VERSION,
            Self::TenjikuV1 => tenjiku::GAME_RECORD_SCHEMA_VERSION,
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
    /// Complete authoritative Xiangqi history, warnings, claims and conclusions.
    Xiangqi(Box<xiangqi::GameRecord>),
    /// Authoritative complete native Tori history and derived outcomes.
    Tori(Box<tori::GameRecord>),
    /// Authoritative native Chu routes, rights, obligations and outcomes.
    Chu(Box<chu::GameRecord>),
    /// Authoritative native Tenjiku profile, routes, ordered effects and history.
    Tenjiku(Box<tenjiku::GameRecord>),
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
    /// Wrap the native Xiangqi ledger without changing its semantics or wire encoding.
    pub fn xiangqi(record: xiangqi::GameRecord) -> Self {
        Self::Xiangqi(Box::new(record))
    }
    /// Wrap a real Tori record without fabricating a session or execution capability.
    pub fn tori(record: tori::GameRecord) -> Self {
        Self::Tori(Box::new(record))
    }
    /// Wrap an actual Chu record without advertising engine execution.
    pub fn chu(record: chu::GameRecord) -> Self {
        Self::Chu(Box::new(record))
    }
    /// Wrap the actual immutable native Tenjiku record without advertising execution.
    pub fn tenjiku(record: tenjiku::GameRecord) -> Self {
        Self::Tenjiku(Box::new(record))
    }
    /// Exact native profile.
    pub fn profile(&self) -> RulesProfile {
        match self {
            Self::Chess(_) => RulesProfile::ChessMatchV1,
            Self::Shogi(_) => RulesProfile::ShogiCsa36V1,
            Self::Xiangqi(_) => RulesProfile::XiangqiWxf2018V1,
            Self::Tori(_) => RulesProfile::ToriModernV1,
            Self::Chu(_) => RulesProfile::ChuJcsa2019V1,
            Self::Tenjiku(record) => record.profile().into(),
        }
    }
    /// Actual game, independent of execution capability.
    pub fn game_kind(&self) -> GameKind {
        self.profile().game()
    }
    /// Complete event count, including terminal commands and historical observations.
    pub fn event_count(&self) -> usize {
        match self {
            Self::Chess(record) => record.events().len(),
            Self::Shogi(record) => record.event_count(),
            Self::Xiangqi(record) => record.event_count(),
            Self::Tori(record) => record.event_count(),
            Self::Chu(record) => record.event_count(),
            Self::Tenjiku(record) => record.event_count(),
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
            Self::Xiangqi(record) => record.known_board_actions(),
            Self::Tori(record) => record.known_board_actions(),
            Self::Chu(record) => record.known_board_actions(),
            Self::Tenjiku(record) => record.known_board_actions(),
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
            Self::Xiangqi(_) => NativeSchema::XiangqiV1,
            Self::Tori(_) => NativeSchema::ToriV1,
            Self::Chu(_) => NativeSchema::ChuV1,
            Self::Tenjiku(_) => NativeSchema::TenjikuV1,
        }
    }
    /// Serialize a complete bounded wrapper. Native encoding is unchanged.
    pub fn to_json(&self) -> Result<String, EnvelopeError> {
        encode_envelope(self, &self.payload_json()?)
    }
    /// Strict outer parsing followed by the selected strict native parser.
    pub fn from_json(text: &str) -> Result<Self, EnvelopeError> {
        Self::from_json_with_stop(text, &mut || false)
    }
    /// Strict duplicate-preserving parsing with native Tenjiku replay cancellation.
    /// Older native parsers remain bounded calls with cancellation checked around them.
    pub fn from_json_with_stop(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self, EnvelopeError> {
        check_stop(stop)?;
        if text.len() > MAX_RECORD_ENVELOPE_BYTES {
            return Err(EnvelopeError::Capacity);
        }
        let wire: WireRecord = serde_json::from_str(text).map_err(EnvelopeError::encoding)?;
        check_stop(stop)?;
        if wire.envelope_version != RECORD_ENVELOPE_VERSION
            || wire.rules_profile.game() != wire.game_kind
            || wire.profile_edition != wire.rules_profile.edition()
            || wire.game_kind == GameKind::Tenjiku
                && wire.native_schema_version != tenjiku::GAME_RECORD_SCHEMA_VERSION
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
            GameKind::Xiangqi => Self::xiangqi(
                xiangqi::GameRecord::from_json(wire.payload.get())
                    .map_err(EnvelopeError::encoding)?,
            ),
            GameKind::Tori => Self::tori(
                tori::GameRecord::from_json(wire.payload.get()).map_err(EnvelopeError::encoding)?,
            ),
            GameKind::Chu => Self::chu(
                chu::GameRecord::from_json(wire.payload.get()).map_err(EnvelopeError::encoding)?,
            ),
            GameKind::Tenjiku => Self::tenjiku(
                tenjiku::GameRecord::from_json_with_stop(wire.payload.get(), stop).map_err(
                    |error| match error {
                        tenjiku::RecordError::Stopped => EnvelopeError::Stopped,
                        tenjiku::RecordError::Capacity => EnvelopeError::Capacity,
                        error => EnvelopeError::encoding(error),
                    },
                )?,
            ),
        };
        check_stop(stop)?;
        if record.native_schema().version() != wire.native_schema_version
            || record.profile() != wire.rules_profile
        {
            return Err(EnvelopeError::Identity);
        }
        record.to_json()?;
        check_stop(stop)?;
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
            Self::Xiangqi(record) => record.to_json().to_owned(),
            Self::Tori(record) => record.to_json().to_owned(),
            Self::Chu(record) => record.to_json().to_owned(),
            Self::Tenjiku(record) => record.to_json().to_owned(),
        };
        if text.len() > MAX_RECORD_ENVELOPE_BYTES {
            return Err(EnvelopeError::Capacity);
        }
        Ok(text)
    }
}
impl Serialize for CanonicalRecord {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        // Compute native encoding once and enforce the complete canonical wrapper
        // bound for every serializer, including the existing field-wise format.
        let payload = self.payload_json().map_err(serde::ser::Error::custom)?;
        let text = encode_envelope(self, &payload).map_err(serde::ser::Error::custom)?;
        if matches!(self, Self::Tenjiku(_)) {
            // Preserve the already measured Tenjiku document/fixture encoding.
            return RawValue::from_string(text)
                .map_err(serde::ser::Error::custom)?
                .serialize(serializer);
        }
        // Existing games retain their original pretty field indentation while
        // each authoritative native payload remains raw and byte-preserved.
        let payload = RawValue::from_string(payload).map_err(serde::ser::Error::custom)?;
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
fn check_stop(stop: &mut impl FnMut() -> bool) -> Result<(), EnvelopeError> {
    if stop() {
        Err(EnvelopeError::Stopped)
    } else {
        Ok(())
    }
}
fn encode_envelope(record: &CanonicalRecord, payload: &str) -> Result<String, EnvelopeError> {
    #[derive(Serialize)]
    struct Encoded<'a> {
        envelope_version: u32,
        game_kind: GameKind,
        rules_profile: RulesProfile,
        profile_edition: &'static str,
        native_schema_version: u32,
        payload: &'a RawValue,
    }
    let payload = RawValue::from_string(payload.to_owned()).map_err(EnvelopeError::encoding)?;
    let text = serde_json::to_string(&Encoded {
        envelope_version: RECORD_ENVELOPE_VERSION,
        game_kind: record.game_kind(),
        rules_profile: record.profile(),
        profile_edition: record.profile().edition(),
        native_schema_version: record.native_schema().version(),
        payload: &payload,
    })
    .map_err(EnvelopeError::encoding)?;
    if text.len() > MAX_RECORD_ENVELOPE_BYTES {
        Err(EnvelopeError::Capacity)
    } else {
        Ok(text)
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
    /// Cooperative native replay/parser stop; no partial record is published.
    Stopped,
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
    /// Xiangqi Red or Black, never Chess Black.
    Xiangqi(xiangqi::Side),
    /// Tori Sente/Gote are game-qualified native owners.
    Tori(tori::Side),
    /// Chu Sente/Gote retain their own native identity.
    Chu(chu::Side),
    /// Tenjiku owner; the whole selected profile remains explicit in its record/context.
    Tenjiku(tenjiku::Side),
}
impl NativeSide {
    /// Side's game, for validating purpose metadata without replay.
    pub const fn game(self) -> GameKind {
        match self {
            Self::Chess(_) => GameKind::Chess,
            Self::Shogi(_) => GameKind::Shogi,
            Self::Xiangqi(_) => GameKind::Xiangqi,
            Self::Tori(_) => GameKind::Tori,
            Self::Chu(_) => GameKind::Chu,
            Self::Tenjiku(_) => GameKind::Tenjiku,
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
                Self::Xiangqi(xiangqi::Side::Red) => "red",
                Self::Xiangqi(xiangqi::Side::Black) => "black",
                Self::Tori(tori::Side::Sente) => "sente",
                Self::Tori(tori::Side::Gote) => "gote",
                Self::Chu(chu::Side::Sente) => "sente",
                Self::Chu(chu::Side::Gote) => "gote",
                Self::Tenjiku(tenjiku::Side::Sente) => "sente",
                Self::Tenjiku(tenjiku::Side::Gote) => "gote",
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
            (GameKind::Xiangqi, "red") => Ok(Self::Xiangqi(xiangqi::Side::Red)),
            (GameKind::Xiangqi, "black") => Ok(Self::Xiangqi(xiangqi::Side::Black)),
            (GameKind::Tori, "sente") => Ok(Self::Tori(tori::Side::Sente)),
            (GameKind::Tori, "gote") => Ok(Self::Tori(tori::Side::Gote)),
            (GameKind::Chu, "sente") => Ok(Self::Chu(chu::Side::Sente)),
            (GameKind::Chu, "gote") => Ok(Self::Chu(chu::Side::Gote)),
            (GameKind::Tenjiku, "sente") => Ok(Self::Tenjiku(tenjiku::Side::Sente)),
            (GameKind::Tenjiku, "gote") => Ok(Self::Tenjiku(tenjiku::Side::Gote)),
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
    /// Authoritative untimed Xiangqi game; native command admission owns all rules.
    Xiangqi(&'a mut xiangqi::Game),
    /// Real Tori native game, not a reconstructed consumer.
    Tori(&'a mut tori::Game),
    /// Native Chu owner admits commands with its own actor semantics.
    Chu(&'a mut chu::Game),
    /// Authoritative two-profile Tenjiku owner.
    Tenjiku(&'a mut tenjiku::Game),
}
/// Native commands keep game-specific actors, actions and preconditions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NativeCommand {
    /// Chess command, with native timestamp/version and optional claim intention.
    Chess(chess::MatchRequest),
    /// Shogi action, formal declaration or resignation with complete expected context.
    Shogi(Box<shogi::GameRequest>),
    /// Complete Xiangqi action or terminal/claim request with exact native context.
    Xiangqi(Box<xiangqi::GameRequest>),
    /// Complete context-bound Tori board/terminal/administrative command.
    Tori(Box<tori::GameRequest>),
    /// Complete native Chu command and expected history.
    Chu(Box<chu::GameRequest>),
    /// Exact native Tenjiku command/context; profile mismatch remains native refusal.
    Tenjiku(Box<tenjiku::GameRequest>),
}
/// Native admission result, preserving rules-owned semantics and errors.
#[derive(Debug)]
pub enum NativeReply {
    /// Complete chess acceptance/rejection, clock and conclusion observations.
    Chess(chess::MatchReply),
    /// Exact shogi reversal token or atomic native rejection/internal diagnostic.
    Shogi(Result<Box<shogi::GameUndo>, shogi::GameError>),
    /// Preserve admitted/rejected-claim audit result AND exact native reversal.
    Xiangqi(Result<Box<xiangqi::Submission>, xiangqi::GameError>),
    /// Exact Tori full-game reversal or atomic native refusal.
    Tori(Result<Box<tori::GameUndo>, tori::GameError>),
    /// Native Chu exact reversal or atomic typed refusal.
    Chu(Result<Box<chu::GameUndo>, chu::GameError>),
    /// Native Tenjiku reversal or typed atomic native refusal.
    Tenjiku(Result<Box<tenjiku::GameUndo>, tenjiku::GameError>),
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
        (NativeCommandTarget::Xiangqi(game), NativeCommand::Xiangqi(request)) => {
            Ok(NativeReply::Xiangqi(game.submit(*request).map(Box::new)))
        }
        (NativeCommandTarget::Tori(game), NativeCommand::Tori(request)) => {
            Ok(NativeReply::Tori(game.submit(*request).map(Box::new)))
        }
        (NativeCommandTarget::Chu(game), NativeCommand::Chu(request)) => {
            Ok(NativeReply::Chu(game.submit(*request).map(Box::new)))
        }
        (NativeCommandTarget::Tenjiku(game), NativeCommand::Tenjiku(request)) => {
            Ok(NativeReply::Tenjiku(game.submit(*request).map(Box::new)))
        }
        _ => Err(DispatchMismatch),
    }
}

#[cfg(test)]
mod envelope_size_tests {
    use super::*;

    #[test]
    fn complete_encoding_counts_profile_specific_wrapper_headroom_exactly() {
        for profile in tenjiku::Profile::ALL {
            let record = CanonicalRecord::tenjiku(
                tenjiku::GameRecord::from_game(&tenjiku::Game::standard(profile)).unwrap(),
            );
            // Exercise only the envelope accounting helper with a JSON string;
            // this synthetic payload is deliberately not a native game record.
            let empty = encode_envelope(&record, "\"\"").unwrap();
            let padding = MAX_RECORD_ENVELOPE_BYTES - empty.len();
            let payload = format!("\"{}\"", "x".repeat(padding));
            assert!(payload.len() < MAX_RECORD_ENVELOPE_BYTES);
            assert_eq!(
                encode_envelope(&record, &payload).unwrap().len(),
                MAX_RECORD_ENVELOPE_BYTES
            );
            let overflow = format!("\"{}x\"", "x".repeat(padding));
            assert!(overflow.len() < MAX_RECORD_ENVELOPE_BYTES);
            assert_eq!(
                encode_envelope(&record, &overflow).unwrap_err(),
                EnvelopeError::Capacity
            );
        }
    }
}
