//! Explicit game dispatch; rules and record formats remain authoritative.
//!
//! Chess history observations are data, not commands to execute. Complete chess
//! match admission still belongs to `ChessMatch`; shogi admission belongs to `Game`.

mod context;
pub(crate) mod history;
/// Reusable authoritative game retention for incremental execution sessions.
pub mod incremental;
pub use context::GameContext;

/// Refusal of a compact record or retained suffix.
pub use crate::record::compact::RecordError;
use crate::{chess, chu, shogi, tenjiku, tori, xiangqi};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::value::RawValue;
use std::{error::Error, fmt};

/// This wrapper's version, independent of record schemas and engine transport.
pub const RECORD_ENVELOPE_VERSION: u32 = 2;
/// Maximum retained shared-record diagnostic bytes.
pub const MAX_RECORD_DIAGNOSTIC_BYTES: usize = 4096;
/// The source edition accepted by the existing chess match profile.
pub const CHESS_PROFILE_EDITION: &str = "FIDE-2023-01-01+mshogi.chess.match.v1";

/// Actual game; this does not advertise execution support.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameKind {
    /// Orthodox chess under the existing profile.
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
    /// Chess match/history semantics, including the recorded claim policy.
    ChessMatchV1,
    /// Selected standard-shogi CSA software convention.
    ShogiCsa36V1,
    /// Selected bilingual WXF2018 software profile.
    XiangqiWxf2018V1,
    /// Selected modern Tori software profile.
    ToriModernV1,
    /// Selected modern JCSA Chu profile.
    ChuJcsa2019V1,
    /// Ratified Adams 1999 Modern complete Tenjiku profile.
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
    /// Exact Tenjiku choice; other games have no implicit Tenjiku default.
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
            _ => Err(serde::de::Error::custom("unsupported rules profile")),
        }
    }
}

/// Record schema meaning, rather than an unqualified number shared across games.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordSchema {
    /// Chess history schema one.
    ChessV1,
    /// Approved Shogi command history schema two.
    ShogiV2,
    /// Xiangqi game command history schema two.
    XiangqiV2,
    /// Complete modern Tori command history schema four.
    ToriV4,
    /// Complete Chu command history schema three.
    ChuV3,
    /// Tenjiku command history schema three, qualified by either whole profile.
    TenjikuV3,
}
impl RecordSchema {
    /// Wire number, scoped to the game/profile.
    pub const fn version(self) -> u32 {
        match self {
            Self::ChessV1 => chess::CHESS_RECORD_SCHEMA_VERSION,
            Self::ShogiV2 => shogi::SHOGI_RECORD_SCHEMA_VERSION,
            Self::XiangqiV2 => xiangqi::GAME_RECORD_SCHEMA_VERSION,
            Self::ToriV4 => tori::GAME_RECORD_SCHEMA_VERSION,
            Self::ChuV3 => chu::GAME_RECORD_SCHEMA_VERSION,
            Self::TenjikuV3 => tenjiku::GAME_RECORD_SCHEMA_VERSION,
        }
    }
}

/// Complete game records. Metadata is derived, so mutable outer tags cannot drift.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalRecord {
    /// Chess history/observation record; not an implicitly reconstructed match.
    Chess(Box<chess::GameRecord>),
    /// Authoritative accepted shogi game/command ledger.
    Shogi(Box<shogi::GameRecord>),
    /// Complete authoritative Xiangqi history, warnings, claims and conclusions.
    Xiangqi(Box<xiangqi::GameRecord>),
    /// Authoritative complete Tori history and derived outcomes.
    Tori(Box<tori::GameRecord>),
    /// Authoritative Chu routes, rights, obligations and outcomes.
    Chu(Box<chu::GameRecord>),
    /// Authoritative Tenjiku profile, routes, ordered effects and history.
    Tenjiku(Box<tenjiku::GameRecord>),
}
impl CanonicalRecord {
    /// Wrap the actual chess payload without changing historical observations.
    pub fn chess(record: chess::GameRecord) -> Self {
        Self::Chess(Box::new(record))
    }
    /// Wrap the actual shogi ledger without losing root/history provenance.
    pub fn shogi(record: shogi::GameRecord) -> Self {
        Self::Shogi(Box::new(record))
    }
    /// Wrap the Xiangqi ledger without changing its semantics or wire encoding.
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
    /// Wrap the actual immutable Tenjiku record without advertising execution.
    pub fn tenjiku(record: tenjiku::GameRecord) -> Self {
        Self::Tenjiku(Box::new(record))
    }
    /// Exact profile.
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
    pub fn record_schema(&self) -> RecordSchema {
        match self {
            Self::Chess(_) => RecordSchema::ChessV1,
            Self::Shogi(_) => RecordSchema::ShogiV2,
            Self::Xiangqi(_) => RecordSchema::XiangqiV2,
            Self::Tori(_) => RecordSchema::ToriV4,
            Self::Chu(_) => RecordSchema::ChuV3,
            Self::Tenjiku(_) => RecordSchema::TenjikuV3,
        }
    }
    /// Serialize the complete wrapper. Encoding is unchanged.
    pub fn to_json(&self) -> Result<String, EnvelopeError> {
        encode_envelope(self, &self.payload_json()?)
    }
    /// Strict outer parsing followed by the selected strict record parser.
    pub fn from_json(text: &str) -> Result<Self, EnvelopeError> {
        Self::from_json_with_stop(text, &mut || false)
    }
    /// Strict duplicate-preserving parsing with Tenjiku replay cancellation.
    /// Chess, Shogi and Xiangqi parsers check cancellation only before and after parsing.
    pub fn from_json_with_stop(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self, EnvelopeError> {
        Self::from_json_with_parser(text, stop, |kind, payload, stop| {
            let record = match kind {
                GameKind::Chess => Self::chess(
                    chess::GameRecord::from_json(payload).map_err(EnvelopeError::encoding)?,
                ),
                GameKind::Shogi => Self::shogi(
                    shogi::GameRecord::from_json(payload).map_err(EnvelopeError::encoding)?,
                ),
                GameKind::Xiangqi => Self::xiangqi(
                    xiangqi::GameRecord::from_json(payload).map_err(EnvelopeError::from)?,
                ),
                GameKind::Tori => Self::tori(
                    tori::GameRecord::from_json_with_stop(payload, stop)
                        .map_err(EnvelopeError::from)?,
                ),
                GameKind::Chu => Self::chu(
                    chu::GameRecord::from_json_with_stop(payload, stop)
                        .map_err(EnvelopeError::from)?,
                ),
                GameKind::Tenjiku => Self::tenjiku(
                    tenjiku::GameRecord::from_json_with_stop(payload, stop)
                        .map_err(EnvelopeError::from)?,
                ),
            };
            Ok((record, ()))
        })
        .map(|(record, ())| record)
    }
    /// Validate the shared envelope around a caller-selected strict record parser.
    /// The parser can retain its reconstructed game alongside the validated record,
    /// so consumers need not replay the same commands a second time.
    pub fn from_json_with_parser<T, F: FnMut() -> bool>(
        text: &str,
        stop: &mut F,
        parse: impl FnOnce(GameKind, &str, &mut F) -> Result<(Self, T), EnvelopeError>,
    ) -> Result<(Self, T), EnvelopeError> {
        check_stop(stop)?;
        let wire: WireRecord = serde_json::from_str(text).map_err(EnvelopeError::encoding)?;
        if wire.envelope_version != RECORD_ENVELOPE_VERSION {
            return Err(EnvelopeError::Identity);
        }
        let (kind, payload) = (wire.game_kind, wire.payload);
        check_stop(stop)?;
        let (record, parsed) = parse(kind, payload.get(), stop)?;
        check_stop(stop)?;
        if record.game_kind() != kind {
            return Err(EnvelopeError::Identity);
        }
        record.to_json()?;
        check_stop(stop)?;
        Ok((record, parsed))
    }
    fn payload_json(&self) -> Result<String, EnvelopeError> {
        let text = match self {
            Self::Chess(record) => record.to_json(),
            Self::Shogi(record) => record.to_json().map_err(EnvelopeError::encoding)?,
            Self::Xiangqi(record) => record.to_json().to_owned(),
            Self::Tori(record) => record.to_json().to_owned(),
            Self::Chu(record) => record.to_json().to_owned(),
            Self::Tenjiku(record) => record.to_json().to_owned(),
        };
        Ok(text)
    }
}
impl Serialize for CanonicalRecord {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let text = self.to_json().map_err(serde::ser::Error::custom)?;
        crate::record::serialize_json(&text, serializer)
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
        payload: &'a RawValue,
    }
    let payload = RawValue::from_string(payload.to_owned()).map_err(EnvelopeError::encoding)?;
    serde_json::to_string(&Encoded {
        envelope_version: RECORD_ENVELOPE_VERSION,
        game_kind: record.game_kind(),
        payload: &payload,
    })
    .map_err(EnvelopeError::encoding)
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireRecord {
    envelope_version: u32,
    game_kind: GameKind,
    payload: Box<RawValue>,
}
impl<'de> Deserialize<'de> for CanonicalRecord {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Capture all wrapper bytes: no intermediate map or tagged-enum buffer
        // may erase duplicate outer or nested fields.
        let raw = Box::<RawValue>::deserialize(deserializer)?;
        Self::from_json(raw.get()).map_err(serde::de::Error::custom)
    }
}
/// Explicit unsupported identity, malformed input or resource rejection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnvelopeError {
    /// A game record field or root exceeds its representation.
    Capacity,
    /// Cooperative game replay/parser stop; no partial record is published.
    Stopped,
    /// Envelope, game, profile, edition or record schema disagree.
    Identity,
    /// Strict outer/syntax rejection, with bounded diagnostics.
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
        write!(f, "game record envelope rejected: {self:?}")
    }
}
impl Error for EnvelopeError {}

/// Game-qualified sides retain interpretation; no White/Sente coercion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaggedSide {
    /// Chess White or Black.
    Chess(chess::Color),
    /// Shogi Sente or Gote.
    Shogi(shogi::Side),
    /// Xiangqi Red or Black, never Chess Black.
    Xiangqi(xiangqi::Side),
    /// Tori Sente/Gote are game-tagged sides.
    Tori(tori::Side),
    /// Chu Sente/Gote retain their own game identity.
    Chu(chu::Side),
    /// Tenjiku owner; the whole selected profile remains explicit in its record/context.
    Tenjiku(tenjiku::Side),
}
impl TaggedSide {
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
impl Serialize for TaggedSide {
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
impl<'de> Deserialize<'de> for TaggedSide {
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
pub enum GameCommandTarget<'a> {
    /// Complete chess match, including clocks, offers and version admission.
    Chess(&'a mut chess::ChessMatch),
    /// Authoritative untimed shogi game and accepted command ledger.
    Shogi(&'a mut shogi::Game),
    /// Authoritative untimed Xiangqi game; game command admission owns all rules.
    Xiangqi(&'a mut xiangqi::Game),
    /// Real Tori game, not a reconstructed consumer.
    Tori(&'a mut tori::Game),
    /// Chu owner admits commands with its own actor semantics.
    Chu(&'a mut chu::Game),
    /// Authoritative two-profile Tenjiku owner.
    Tenjiku(&'a mut tenjiku::Game),
}
/// Game commands keep game-specific actors, actions and preconditions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameCommand {
    /// Chess command, with timestamp/version and optional claim intention.
    Chess(chess::MatchRequest),
    /// Shogi action, formal declaration or resignation with its request token.
    Shogi(Box<shogi::GameRequest>),
    /// Complete Xiangqi action or terminal/claim request with its request token.
    Xiangqi(Box<xiangqi::GameRequest>),
    /// Complete context-bound Tori board/terminal/administrative command.
    Tori(Box<tori::GameRequest>),
    /// Complete Chu command with its request token.
    Chu(Box<chu::GameRequest>),
    /// Exact Tenjiku command and request token; a token of another game is refused.
    Tenjiku(Box<tenjiku::GameRequest>),
}
/// Game admission result, preserving rules-owned semantics and errors.
#[derive(Debug)]
pub enum GameReply {
    /// Complete chess acceptance/rejection, clock and conclusion observations.
    Chess(chess::MatchReply),
    /// Exact shogi reversal token or atomic rejection/internal diagnostic.
    Shogi(Result<Box<shogi::GameUndo>, shogi::GameError>),
    /// Preserve admitted/rejected-claim audit result AND exact reversal.
    Xiangqi(Result<Box<xiangqi::Submission>, xiangqi::GameError>),
    /// Exact Tori full-game reversal or atomic refusal.
    Tori(Result<Box<tori::GameUndo>, tori::GameError>),
    /// Chu exact reversal or atomic typed refusal.
    Chu(Result<Box<chu::GameUndo>, chu::GameError>),
    /// Tenjiku reversal or typed atomic refusal.
    Tenjiku(Result<Box<tenjiku::GameUndo>, tenjiku::GameError>),
}
/// A wrong-game command is rejected before either rules owner is touched.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DispatchMismatch;
/// Dispatch only by typed game tag. All actor, turn and command admission is.
pub fn dispatch_command(
    target: GameCommandTarget<'_>,
    command: GameCommand,
) -> Result<GameReply, DispatchMismatch> {
    match (target, command) {
        (GameCommandTarget::Chess(game), GameCommand::Chess(request)) => {
            Ok(GameReply::Chess(game.submit(request)))
        }
        (GameCommandTarget::Shogi(game), GameCommand::Shogi(request)) => {
            Ok(GameReply::Shogi(game.submit(*request).map(Box::new)))
        }
        (GameCommandTarget::Xiangqi(game), GameCommand::Xiangqi(request)) => {
            Ok(GameReply::Xiangqi(game.submit(*request).map(Box::new)))
        }
        (GameCommandTarget::Tori(game), GameCommand::Tori(request)) => {
            Ok(GameReply::Tori(game.submit(*request).map(Box::new)))
        }
        (GameCommandTarget::Chu(game), GameCommand::Chu(request)) => {
            Ok(GameReply::Chu(game.submit(*request).map(Box::new)))
        }
        (GameCommandTarget::Tenjiku(game), GameCommand::Tenjiku(request)) => {
            Ok(GameReply::Tenjiku(game.submit(*request).map(Box::new)))
        }
        _ => Err(DispatchMismatch),
    }
}

impl From<RecordError> for EnvelopeError {
    fn from(error: RecordError) -> Self {
        match error {
            RecordError::Stopped => Self::Stopped,
            RecordError::Capacity => Self::Capacity,
            other => Self::encoding(other),
        }
    }
}
