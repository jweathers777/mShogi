//! Explicit shared transport v3. Legacy v2 values and historical observations stay unchanged.
use crate::{
    AnalysisTarget, EngineFailure, EngineRequest, EngineResult, FULL_SENTINEL, FailureKind,
    FullLimits, MAX_ANALYSIS_DEPTH, MAX_ANALYSIS_NODES, MAX_FULL_POSITIONS, MAX_FULL_TRANSITIONS,
    MAX_STDERR_BYTES, ReachabilityTarget, RequestPurpose, WorkLimits,
};
use mshogi_core::native::{CanonicalRecord, GameKind, MAX_SHARED_CHESS_EVENTS, NativeSide};
use serde::{Deserialize, Serialize};

/// Shared transport identity, independent of native record/envelope versions.
pub const SHARED_PROTOCOL_VERSION: u32 = 3;
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
pub struct SharedAnalysisTarget {
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
pub enum SharedPurpose {
    /// An actual live engine-turn request. Empty struct keeps unknown fields strict.
    EngineTurn {},
    /// Read-only bounded move analysis of the exact supplied prefix.
    MoveAnalysis {
        /// Native target provenance.
        target: SharedAnalysisTarget,
    },
    /// Existing chess Full operation; never interpreted as shogi analysis.
    ChessFullReachability {
        /// Native target provenance.
        target: SharedAnalysisTarget,
        /// Existing chess-specific cooperative target.
        mate: ReachabilityTarget,
        /// Existing bounded independently terminable work settings.
        limits: FullLimits,
    },
}
/// Full v3 execution context. Equality includes every submitted identity and setting.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedRequest {
    /// Must be SHARED_PROTOCOL_VERSION, not a native record schema.
    pub protocol: u32,
    /// Explicit command/observation purpose.
    pub purpose: SharedPurpose,
    /// Fresh monotonically increasing bridge request identity.
    pub request_id: u64,
    /// Host-owned session version, independent of board ply and semantic undo.
    pub session_version: u64,
    /// Fresh child-generation identity, shared across legacy/shared work.
    pub process_generation: u64,
    /// Actual canonical native history, policies and root provenance.
    pub game: CanonicalRecord,
    /// Complete deterministic ordinary-search limits (Full uses FULL_SENTINEL).
    pub limits: WorkLimits,
}
impl SharedRequest {
    /// Cheap bounded static admission. Native live replay stays in the supervised child.
    pub fn valid_limits(&self) -> bool {
        if self.game.game_kind() == GameKind::Chess
            && self.game.event_count() > MAX_SHARED_CHESS_EVENTS
        {
            return false;
        }
        let target_valid = |target: &SharedAnalysisTarget| {
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
            SharedPurpose::EngineTurn {} => ordinary_limits_valid(false),
            SharedPurpose::MoveAnalysis { target } => {
                target_valid(target) && ordinary_limits_valid(true)
            }
            SharedPurpose::ChessFullReachability { target, limits, .. } => {
                target_valid(target)
                    && self.limits == FULL_SENTINEL
                    && limits.max_positions <= MAX_FULL_POSITIONS
                    && limits.max_transitions <= MAX_FULL_TRANSITIONS
            }
        }
    }
    pub(crate) fn chess_adapter(&self) -> Option<EngineRequest> {
        let CanonicalRecord::Chess(game) = &self.game else {
            return None;
        };
        let target = |t: &SharedAnalysisTarget| {
            let NativeSide::Chess(turn) = t.turn else {
                return None;
            };
            Some(AnalysisTarget {
                selection_id: t.selection_id,
                ply: t.board_ply,
                session_version: t.session_version,
                label: t.label.clone(),
                turn,
            })
        };
        let purpose = match &self.purpose {
            SharedPurpose::EngineTurn {} => RequestPurpose::EngineTurn,
            SharedPurpose::MoveAnalysis { target: t } => {
                RequestPurpose::MoveAnalysis { target: target(t)? }
            }
            SharedPurpose::ChessFullReachability {
                target: t,
                mate,
                limits,
            } => RequestPurpose::FullReachability {
                target: target(t)?,
                mate: *mate,
                limits: *limits,
            },
        };
        Some(EngineRequest {
            protocol: crate::PROTOCOL_VERSION,
            purpose,
            request_id: self.request_id,
            session_version: self.session_version,
            process_generation: self.process_generation,
            game: (**game).clone(),
            limits: self.limits,
        })
    }
}

/// Actual native result families; scores and conclusions retain native meaning.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SharedResult {
    /// Real chess result, retaining native action/claim/outcome/incomplete/Full distinctions.
    Chess {
        /// Existing native chess result and score interpretation.
        result: EngineResult,
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
impl SharedResult {
    pub(crate) fn failure(kind: FailureKind, message: impl ToString) -> Self {
        let EngineResult::Failure { failure } = EngineResult::failure(kind, message) else {
            unreachable!()
        };
        Self::Failure { failure }
    }
}
/// Final shared response, permanently bound to the original v3 context.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedResponse {
    /// Exact original context; adapting chess execution never changes the echo.
    pub context: SharedRequest,
    /// Distinct native execution result or explicit unscored failure.
    pub result: SharedResult,
}
impl SharedResponse {
    /// Full context and purpose/family acceptance. The host still owns native command
    /// admission; read-only analysis is never applied as an engine-turn command here.
    pub fn matches(&self, submitted: &SharedRequest) -> bool {
        if submitted.protocol != SHARED_PROTOCOL_VERSION
            || submitted.request_id == 0
            || submitted.process_generation == 0
            || !submitted.valid_limits()
            || self.context != *submitted
        {
            return false;
        }
        match &self.result {
            SharedResult::Failure { failure } => failure.message.len() <= MAX_STDERR_BYTES,
            SharedResult::Shogi { result } => result.matches(submitted),
            SharedResult::Xiangqi { result } => result.matches(submitted),
            SharedResult::Chess { result } => {
                if submitted.game.game_kind() != GameKind::Chess {
                    return false;
                }
                if let EngineResult::Failure { failure } = result {
                    return failure.message.len() <= MAX_STDERR_BYTES;
                }
                matches!(
                    (&submitted.purpose, result),
                    (
                        SharedPurpose::ChessFullReachability { .. },
                        EngineResult::Reachability { .. }
                    )
                ) || (!matches!(
                    submitted.purpose,
                    SharedPurpose::ChessFullReachability { .. }
                ) && !matches!(result, EngineResult::Reachability { .. }))
            }
        }
    }
}
