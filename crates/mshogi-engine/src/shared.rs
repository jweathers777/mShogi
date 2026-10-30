//! Singular internal execution context, transport, and durable observations.
use crate::{
    ChessResult, EngineFailure, FULL_SENTINEL, FailureKind, FullLimits, MAX_ANALYSIS_DEPTH,
    MAX_ANALYSIS_NODES, MAX_FULL_POSITIONS, MAX_FULL_TRANSITIONS, MAX_STDERR_BYTES,
    ReachabilityTarget, WorkLimits,
};
use mshogi_core::native::{CanonicalRecord, GameKind, MAX_SHARED_CHESS_EVENTS, NativeSide};
use serde::{Deserialize, Serialize};

/// Actual implemented operations; a tagged context alone adds no execution capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionOperation {
    /// Complete live-turn choice.
    EngineTurn,
    /// Read-only bounded ordinary search observations.
    MoveAnalysis,
    /// Chess cooperative mating reachability, not shogi tsume or ordinary search.
    ChessFullReachability,
}
/// Advertise only real native execution consumers. Shogi has no chess Full operation.
pub fn execution_capabilities(game: GameKind) -> &'static [ExecutionOperation] {
    match game {
        GameKind::Chess => &[
            ExecutionOperation::EngineTurn,
            ExecutionOperation::MoveAnalysis,
            ExecutionOperation::ChessFullReachability,
        ],
        GameKind::Xiangqi => &[
            ExecutionOperation::EngineTurn,
            ExecutionOperation::MoveAnalysis,
        ],
        GameKind::Shogi => &[
            ExecutionOperation::EngineTurn,
            ExecutionOperation::MoveAnalysis,
        ],
    }
}
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
    /// Existing chess Full operation; never interpreted as shogi analysis.
    ChessFullReachability {
        /// Native target provenance.
        target: ExecutionTarget,
        /// Existing chess-specific cooperative target.
        mate: ReachabilityTarget,
        /// Existing bounded independently terminable work settings.
        limits: FullLimits,
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
    /// Fresh child-generation identity, shared across all native game work.
    pub process_generation: u64,
    /// Actual canonical native history, policies and root provenance.
    pub game: CanonicalRecord,
    /// Complete deterministic ordinary-search limits (Full uses FULL_SENTINEL).
    pub limits: WorkLimits,
}
impl ExecutionContext {
    /// Cheap bounded static admission. Native live replay stays in the supervised child.
    pub fn valid_limits(&self) -> bool {
        if self.game.game_kind() == GameKind::Chess
            && self.game.event_count() > MAX_SHARED_CHESS_EVENTS
        {
            return false;
        }
        let target_valid = |target: &ExecutionTarget| {
            target.selection_id > 0
                && target.session_version == self.session_version
                && target.event_index == self.game.event_count() as u64
                && target.board_ply == self.game.known_board_actions() as u64
                && target.turn.game() == self.game.game_kind()
                && match &self.game {
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
            ExecutionPurpose::ChessFullReachability { target, limits, .. } => {
                target_valid(target)
                    && self.limits == FULL_SENTINEL
                    && limits.max_positions <= MAX_FULL_POSITIONS
                    && limits.max_transitions <= MAX_FULL_TRANSITIONS
            }
        }
    }
}

/// Actual native result families; scores and conclusions retain native meaning.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ExecutionResult {
    /// Real chess result, retaining native action/claim/outcome/incomplete/Full distinctions.
    Chess {
        /// Existing native chess result and score interpretation.
        result: ChessResult,
    },
    /// Actual native shogi result, never a chess or cooperative Full score.
    Shogi {
        /// Native board/declaration choice, conclusion or incomplete observation.
        result: crate::ShogiResult,
    },
    /// Actual native Xiangqi choice, conclusion, incomplete work or unresolved history.
    Xiangqi {
        /// Native Red/Black observations, not Chess Full reachability.
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
impl ExecutionObservation {
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
            || submitted.process_generation == 0
            || !submitted.valid_limits()
            || self.context != *submitted
        {
            return false;
        }
        match &self.result {
            ExecutionResult::Failure { failure } => failure.message.len() <= MAX_STDERR_BYTES,
            ExecutionResult::Shogi { result } => result.matches(submitted),
            ExecutionResult::Xiangqi { result } => result.matches(submitted),
            ExecutionResult::Chess { result } => {
                if submitted.game.game_kind() != GameKind::Chess {
                    return false;
                }
                if let ChessResult::Failure { failure } = result {
                    return failure.message.len() <= MAX_STDERR_BYTES;
                }
                matches!(
                    (&submitted.purpose, result),
                    (
                        ExecutionPurpose::ChessFullReachability { .. },
                        ChessResult::Reachability { .. }
                    )
                ) || (!matches!(
                    submitted.purpose,
                    ExecutionPurpose::ChessFullReachability { .. }
                ) && !matches!(result, ChessResult::Reachability { .. }))
            }
        }
    }
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
            && self.context.process_generation > 0
            && self.context.valid_limits()
    }
    pub(crate) fn request_id(&self) -> u64 {
        self.context.request_id
    }
    pub(crate) fn generation(&self) -> u64 {
        self.context.process_generation
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
    pub(crate) fn execute(&self) -> ExecutionResponse {
        if !self.valid() {
            return self.failure(
                FailureKind::InvalidRequest,
                "invalid internal transport version, identity, purpose, selection or limits",
            );
        }
        ExecutionResponse {
            protocol: crate::PROTOCOL_VERSION,
            observation: ExecutionObservation::new(
                self.context.clone(),
                crate::server::execute(&self.context),
            ),
        }
    }
}
/// Versioned wire result; persistence retains only its semantic observation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionResponse {
    /// Current transport identity.
    pub protocol: u32,
    /// Complete historical observation, without a transport wrapper.
    pub observation: ExecutionObservation,
}
impl ExecutionResponse {
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
