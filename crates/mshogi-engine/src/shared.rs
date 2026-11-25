//! Singular internal execution context, transport, and durable observations.
use crate::{
    ChessResult, EngineFailure, FailureKind, MAX_ANALYSIS_DEPTH, MAX_ANALYSIS_NODES,
    MAX_STDERR_BYTES, WorkLimits,
};
use mshogi_core::native::{CanonicalRecord, GameKind, NativeSide};
use serde::{Deserialize, Serialize};

/// Exact historical-selection provenance alongside the full native record.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionTarget {
    /// Fresh monotonic host selection identity, independent of semantic board equality.
    pub selection_id: u64,
    /// Complete event prefix, including same-board-ply terminal commands.
    pub event_index: u64,
    /// Known board actions, not an imported FEN/SFEN counter.
    pub board_ply: u64,
    /// Host revision at selection, never restored by native semantic undo.
    pub session_version: u64,
    /// Bounded native-readable target label.
    pub label: String,
    /// Native game-qualified perspective; White and Sente are not synonyms.
    pub turn: NativeSide,
}
/// Purpose is explicit and remains separate from any command-admission path.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ExecutionPurpose {
    /// An actual live engine-turn request. Empty struct keeps unknown fields strict.
    EngineTurn {},
    /// Read-only bounded move analysis of the exact supplied prefix.
    MoveAnalysis {
        /// Native target provenance.
        target: ExecutionTarget,
    },
}
/// Full execution context. Equality includes every submitted identity and setting.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionContext {
    /// Explicit command/observation purpose.
    pub purpose: ExecutionPurpose,
    /// Fresh monotonically increasing bridge request identity.
    pub request_id: u64,
    /// Host-owned session version, independent of board ply and semantic undo.
    pub session_version: u64,
    /// Actual canonical native history, policies and root provenance.
    pub game: CanonicalRecord,
    /// Complete deterministic ordinary-search limits.
    pub limits: WorkLimits,
}
impl ExecutionContext {
    /// Cheap bounded static admission. Native live replay stays in the supervised child.
    pub fn valid_limits(&self) -> bool {
        if self.limits.time_ms.is_some_and(|ms| ms > 60_000) {
            return false;
        }

        let target_valid = |target: &ExecutionTarget| {
            target.selection_id > 0
                && target.session_version == self.session_version
                && target.event_index == self.game.event_count() as u64
                && target.board_ply == self.game.known_board_actions() as u64
                && target.turn.game() == self.game.game_kind()
                && match &self.game {
                    CanonicalRecord::Tenjiku(record) => {
                        target.turn == NativeSide::Tenjiku(record.side_to_move())
                    }
                    CanonicalRecord::Chu(record) => {
                        target.turn == NativeSide::Chu(record.side_to_move())
                    }
                    CanonicalRecord::Tori(record) => {
                        target.turn == NativeSide::Tori(record.side_to_move())
                    }
                    CanonicalRecord::Xiangqi(record) => {
                        target.turn == NativeSide::Xiangqi(record.side_to_move())
                    }
                    _ => true,
                }
                && !target.label.is_empty()
                && target.label.len() <= 256
        };
        let ordinary_limits_valid = |analysis: bool| match self.game.game_kind() {
            GameKind::Chess if analysis => {
                (1..=MAX_ANALYSIS_DEPTH).contains(&self.limits.depth)
                    && self.limits.nodes <= MAX_ANALYSIS_NODES
            }
            GameKind::Chess => (1..=mshogi_search::search::MAX_DEPTH).contains(&self.limits.depth),
            GameKind::Tori => mshogi_search::tori::Limits {
                depth: self.limits.depth,
                nodes: self.limits.nodes,
            }
            .valid(),
            GameKind::Chu => mshogi_search::chu::Limits {
                depth: self.limits.depth,
                nodes: self.limits.nodes,
            }
            .valid(),
            GameKind::Tenjiku => mshogi_search::tenjiku::Limits {
                depth: self.limits.depth,
                nodes: self.limits.nodes,
            }
            .valid(),
            GameKind::Shogi => mshogi_search::shogi::Limits {
                depth: self.limits.depth,
                nodes: self.limits.nodes,
            }
            .valid(),
            GameKind::Xiangqi => mshogi_search::xiangqi::Limits {
                depth: self.limits.depth,
                nodes: self.limits.nodes,
            }
            .valid(),
        };
        match &self.purpose {
            ExecutionPurpose::EngineTurn {} => ordinary_limits_valid(false),
            ExecutionPurpose::MoveAnalysis { target } => {
                target_valid(target) && ordinary_limits_valid(true)
            }
        }
    }
}

/// Actual native result families; scores and conclusions retain native meaning.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ExecutionResult {
    /// Actual full-history Tenjiku search under its immutable selected profile.
    Tenjiku {
        /// Native complete-route decision, conclusion or incomplete observation.
        result: crate::TenjikuResult,
    },
    /// Actual native Chu complete-route search observations.
    Chu {
        /// Chu-owned result; no Chess mate or drop adaptation.
        result: crate::ChuResult,
    },
    /// Actual native Tori complete action, conclusion or incomplete observation.
    Tori {
        /// Tori-owned result and score interpretation.
        result: crate::ToriResult,
    },
    /// Real chess result, retaining native action/claim/outcome/incomplete distinctions.
    Chess {
        /// Existing native chess result and score interpretation.
        result: ChessResult,
    },
    /// Actual native shogi result, never a chess score.
    Shogi {
        /// Native board/declaration choice, conclusion or incomplete observation.
        result: crate::ShogiResult,
    },
    /// Actual native Xiangqi choice, conclusion, incomplete work or unresolved history.
    Xiangqi {
        /// Native Red/Black observations.
        result: crate::XiangqiResult,
    },
    /// Unscored unsupported/invalid/process failure, independent of native game outcomes.
    Failure {
        /// Bounded categorized diagnostic.
        failure: EngineFailure,
    },
}
impl ExecutionResult {
    pub(crate) fn failure(kind: FailureKind, message: impl ToString) -> Self {
        let ChessResult::Failure { failure } = ChessResult::failure(kind, message) else {
            unreachable!()
        };
        Self::Failure { failure }
    }
}
/// Durable observation tied to its complete submitted semantic context.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionObservation {
    /// Durable observation schema, independent of current transport version.
    pub schema_version: u32,
    /// Exact original native context, with no game-specific wire adaptation.
    pub context: ExecutionContext,
    /// Distinct native execution result or explicit unscored failure.
    pub result: ExecutionResult,
}
/// Current durable semantic observation schema.
pub const OBSERVATION_SCHEMA_VERSION: u32 = 1;
/// Strict observation decoding preserves native resource and cancellation failures.
#[derive(Debug)]
pub enum ObservationDecodeError {
    /// Cooperative decoding was interrupted before acceptance.
    Stopped,
    /// The embedded native record rejected admission.
    Native(mshogi_core::native::EnvelopeError),
    /// The observation wire shape or version is invalid.
    Invalid(String),
}
impl std::fmt::Display for ObservationDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stopped => f.write_str("Execution observation parsing stopped"),
            Self::Native(error) => error.fmt(f),
            Self::Invalid(error) => f.write_str(error),
        }
    }
}
impl std::error::Error for ObservationDecodeError {}
impl From<String> for ObservationDecodeError {
    fn from(error: String) -> Self {
        Self::Invalid(error)
    }
}
impl From<&str> for ObservationDecodeError {
    fn from(error: &str) -> Self {
        Self::Invalid(error.into())
    }
}
impl ExecutionObservation {
    /// Parse a durable observation without buffering its native record into a map
    /// or replaying Tenjiku twice. Native facts remain subject to `matches` and
    /// the owning document's target/result admission after this strict parse.
    pub fn from_json_with_stop(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self, String> {
        Self::from_json_with_stop_typed(text, stop).map_err(|error| error.to_string())
    }
    /// Decode without erasing the embedded native record's typed admission failure.
    pub fn from_json_with_stop_typed(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self, ObservationDecodeError> {
        Self::decode(text, None, stop)
    }
    /// Decode an observation whose context omits its native record, with the
    /// caller's already admitted record (MSH-204: experiment reports omit the
    /// record that native replay restores). An encoded record is refused.
    pub fn from_json_with_record(
        text: &str,
        game: CanonicalRecord,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self, ObservationDecodeError> {
        Self::decode(text, Some(game), stop)
    }
    fn decode(
        text: &str,
        supplied: Option<CanonicalRecord>,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self, ObservationDecodeError> {
        fn check(stop: &mut impl FnMut() -> bool) -> Result<(), ObservationDecodeError> {
            if stop() {
                Err(ObservationDecodeError::Stopped)
            } else {
                Ok(())
            }
        }
        check(stop)?;
        if text.len() > crate::MAX_RESPONSE_BYTES {
            return Err("Execution observation exceeds the encoded response limit.".into());
        }
        reject_observation_duplicates(text, stop).map_err(|error| {
            if error == "Execution observation parsing stopped" {
                ObservationDecodeError::Stopped
            } else {
                ObservationDecodeError::Invalid(error)
            }
        })?;
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            schema_version: u32,
            context: Box<serde_json::value::RawValue>,
            result: Box<serde_json::value::RawValue>,
        }
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct ContextWire {
            purpose: ExecutionPurpose,
            request_id: u64,
            session_version: u64,
            #[serde(default)]
            game: Option<Box<serde_json::value::RawValue>>,
            limits: WorkLimits,
        }
        let wire: Wire = serde_json::from_str(text).map_err(|error| error.to_string())?;
        check(stop)?;
        if wire.schema_version != OBSERVATION_SCHEMA_VERSION {
            return Err("Unsupported execution observation schema.".into());
        }
        let context: ContextWire =
            serde_json::from_str(wire.context.get()).map_err(|error| error.to_string())?;
        check(stop)?;
        // With a supplied record the field must be absent, not null.
        if supplied.is_some()
            && serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(
                wire.context.get(),
            )
            .map_err(|error| error.to_string())?
            .contains_key("game")
        {
            return Err("Execution observation context repeats the supplied record.".into());
        }
        let game = match (context.game, supplied) {
            (Some(raw), None) => CanonicalRecord::from_json_with_stop(raw.get(), stop).map_err(
                |error| match error {
                    mshogi_core::native::EnvelopeError::Stopped => ObservationDecodeError::Stopped,
                    error => ObservationDecodeError::Native(error),
                },
            )?,
            (None, Some(game)) => game,
            (None, None) => {
                return Err("Execution observation context has no native record.".into());
            }
            (Some(_), Some(_)) => {
                return Err("Execution observation context repeats the supplied record.".into());
            }
        };
        check(stop)?;
        let result = serde_json::from_str(wire.result.get()).map_err(|error| error.to_string())?;
        check(stop)?;
        Ok(Self {
            schema_version: wire.schema_version,
            context: ExecutionContext {
                purpose: context.purpose,
                request_id: context.request_id,
                session_version: context.session_version,
                game,
                limits: context.limits,
            },
            result,
        })
    }
    /// Preserve exact context and native result under the current durable schema.
    pub fn new(context: ExecutionContext, result: ExecutionResult) -> Self {
        Self {
            schema_version: OBSERVATION_SCHEMA_VERSION,
            context,
            result,
        }
    }
    /// Full context and purpose/family acceptance. The host still owns native command
    /// admission; read-only analysis is never applied as an engine-turn command here.
    pub fn matches(&self, submitted: &ExecutionContext) -> bool {
        if self.schema_version != OBSERVATION_SCHEMA_VERSION
            || submitted.request_id == 0
            || !submitted.valid_limits()
            || self.context != *submitted
        {
            return false;
        }
        match &self.result {
            ExecutionResult::Failure { failure } => failure.message.len() <= MAX_STDERR_BYTES,
            ExecutionResult::Tenjiku { result } => result.matches(submitted),
            ExecutionResult::Chu { result } => result.matches(submitted),
            ExecutionResult::Tori { result } => result.matches(submitted),
            ExecutionResult::Shogi { result } => result.matches(submitted),
            ExecutionResult::Xiangqi { result } => result.matches(submitted),
            ExecutionResult::Chess { result } => {
                if submitted.game.game_kind() != GameKind::Chess {
                    return false;
                }
                let telemetry = match result {
                    ChessResult::Failure { failure } => {
                        return failure.message.len() <= MAX_STDERR_BYTES;
                    }
                    ChessResult::Move { telemetry, .. }
                    | ChessResult::Claim { telemetry, .. }
                    | ChessResult::Finished { telemetry, .. }
                    | ChessResult::Incomplete { telemetry } => telemetry,
                };
                // Search results state an explicit bounded continuation depth and
                // must agree with the complete request work/choice shape before
                // admission.
                telemetry.valid_selective_shape(submitted.limits)
                    && match result {
                        ChessResult::Move { action, .. } => {
                            telemetry.principal_variation.first()
                                == Some(&crate::EngineChoice::Move { action: *action })
                        }
                        ChessResult::Claim { claim, .. } => {
                            telemetry.principal_variation.first()
                                == Some(&crate::EngineChoice::Claim { claim: *claim })
                        }
                        ChessResult::Finished { .. } => telemetry.principal_variation.is_empty(),
                        ChessResult::Incomplete { .. } => {
                            telemetry.completed_depth == 0
                                && telemetry.status != crate::StopReason::Complete
                        }
                        ChessResult::Failure { .. } => unreachable!(),
                    }
            }
        }
    }
}

// Preserve duplicate fields before internally tagged purpose/result parsers can
// buffer them. The seed checks cancellation at each actual JSON value boundary.
pub(crate) fn reject_observation_duplicates(
    text: &str,
    stop: &mut impl FnMut() -> bool,
) -> Result<(), String> {
    use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
    use std::{collections::BTreeSet, fmt};
    struct Unique<'a> {
        stop: &'a mut dyn FnMut() -> bool,
    }
    impl<'de> DeserializeSeed<'de> for Unique<'_> {
        type Value = ();
        fn deserialize<D: serde::Deserializer<'de>>(self, d: D) -> Result<(), D::Error> {
            if (self.stop)() {
                return Err(de::Error::custom("Execution observation parsing stopped"));
            }
            d.deserialize_any(self)
        }
    }
    impl<'de> Visitor<'de> for Unique<'_> {
        type Value = ();
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("bounded duplicate-free observation JSON")
        }
        fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<(), M::Error> {
            let mut keys = BTreeSet::new();
            while let Some(key) = map.next_key::<String>()? {
                if !keys.insert(key) {
                    return Err(de::Error::custom("duplicate observation field"));
                }
                map.next_value_seed(Unique {
                    stop: &mut *self.stop,
                })?;
            }
            Ok(())
        }
        fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<(), S::Error> {
            while seq
                .next_element_seed(Unique {
                    stop: &mut *self.stop,
                })?
                .is_some()
            {}
            Ok(())
        }
        fn visit_bool<E: de::Error>(self, _: bool) -> Result<(), E> {
            Ok(())
        }
        fn visit_i64<E: de::Error>(self, _: i64) -> Result<(), E> {
            Ok(())
        }
        fn visit_u64<E: de::Error>(self, _: u64) -> Result<(), E> {
            Ok(())
        }
        fn visit_f64<E: de::Error>(self, _: f64) -> Result<(), E> {
            Ok(())
        }
        fn visit_str<E: de::Error>(self, _: &str) -> Result<(), E> {
            Ok(())
        }
        fn visit_unit<E: de::Error>(self) -> Result<(), E> {
            Ok(())
        }
    }
    let mut d = serde_json::Deserializer::from_str(text);
    let mut stopped = false;
    let parsed = {
        let mut observed_stop = || {
            let answer = stop();
            stopped |= answer;
            answer
        };
        Unique {
            stop: &mut observed_stop,
        }
        .deserialize(&mut d)
    };
    if stopped {
        return Err("Execution observation parsing stopped".into());
    }
    parsed.map_err(|error| error.to_string())?;
    d.end().map_err(|error| error.to_string())
}

/// Current versioned one-shot transport request.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionRequest {
    /// Transport identity, distinct from native record/profile schema.
    pub protocol: u32,
    /// Complete submitted execution semantics.
    pub context: ExecutionContext,
}
impl ExecutionRequest {
    /// Static admission before launch; authoritative replay stays off path.
    pub fn valid(&self) -> bool {
        self.protocol == crate::PROTOCOL_VERSION
            && self.context.request_id > 0
            && self.context.valid_limits()
    }
    pub(crate) fn request_id(&self) -> u64 {
        self.context.request_id
    }
    pub(crate) fn failure(&self, kind: FailureKind, message: impl ToString) -> ExecutionResponse {
        ExecutionResponse {
            protocol: crate::PROTOCOL_VERSION,
            observation: ExecutionObservation::new(
                self.context.clone(),
                ExecutionResult::failure(kind, message),
            ),
        }
    }
    pub(crate) fn execute(&self, session: &mut crate::server::Session) -> ExecutionResult {
        if !self.valid() {
            return ExecutionResult::failure(
                FailureKind::InvalidRequest,
                "invalid internal transport version, identity, purpose, selection or limits",
            );
        }
        crate::server::execute(&self.context, session)
    }
}
/// Admitted host result; persistence retains only its semantic observation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExecutionResponse {
    /// Current transport identity.
    pub protocol: u32,
    /// Complete historical observation, without a transport wrapper.
    pub observation: ExecutionObservation,
}
impl ExecutionResponse {
    /// Decode a reply to a full request, retaining its exact submitted context.
    /// Retained process sessions must use `incremental::Client` instead.
    pub fn decode(bytes: &[u8], submitted: &ExecutionRequest) -> Result<Self, String> {
        let mut client = crate::incremental::Client::default();
        client
            .encode(&std::sync::Arc::new(submitted.clone()))
            .map_err(|e| e.to_string())?;
        client.decode(bytes).map_err(|e| e.1)
    }

    /// Exact transport and semantic result acceptance.
    pub fn matches(&self, submitted: &ExecutionRequest) -> bool {
        submitted.valid()
            && self.protocol == crate::PROTOCOL_VERSION
            && self.observation.matches(&submitted.context)
    }
    pub(crate) fn failure_message(&self) -> Option<&str> {
        match &self.observation.result {
            ExecutionResult::Failure { failure } => Some(&failure.message),
            _ => None,
        }
    }
}
