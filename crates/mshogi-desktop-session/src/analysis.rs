//! Read-only analysis acceptance and single-resource play ownership.
use super::*;
use mshogi_engine::{
    ExecutionTarget, MAX_ANALYSIS_DEPTH, MAX_ANALYSIS_NODES, MAX_FULL_POSITIONS,
    MAX_FULL_TRANSITIONS, ReachabilityTarget,
};
/// Explicit user-selected deterministic analysis budgets.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AnalysisOperation {
    /// Ordinary minimax; evaluation cannot establish a chess result.
    Moves {
        /// Maximum plies.
        depth: u16,
        /// Maximum attempted nodes.
        nodes: u64,
    },
    /// Advanced cooperative Article-3 reachability, independent of minimax.
    FullReachability {
        /// Which side may deliver mate.
        target: ReachabilityTarget,
        /// Maximum retained positions.
        max_positions: u64,
        /// Maximum examined transitions.
        max_transitions: u64,
    },
}
impl AnalysisOperation {
    pub(super) fn validate(self) -> Result<(), String> {
        let valid = match self {
            Self::Moves { depth, nodes } => {
                (1..=MAX_ANALYSIS_DEPTH).contains(&depth) && nodes <= MAX_ANALYSIS_NODES
            }
            Self::FullReachability {
                max_positions,
                max_transitions,
                ..
            } => max_positions <= MAX_FULL_POSITIONS && max_transitions <= MAX_FULL_TRANSITIONS,
        };
        if valid {
            Ok(())
        } else {
            Err("Unsupported analysis limits. Moves require depth 1–8 and at most 5,000,000 nodes; Full reachability allows at most 100,000 positions and 1,000,000 transitions.".into())
        }
    }
}
/// Analysis observations are separate from live engine-turn decisions.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisPhase {
    /// No analysis selected.
    #[default]
    Idle,
    /// Supervised analysis request outstanding.
    Thinking,
    /// Completed minimax iteration or established cooperative proof.
    Complete,
    /// Observed work limit; no missing work counts are fabricated.
    Incomplete,
    /// Cancelled; no final observations were obtained.
    Cancelled,
    /// Process/protocol/search failure, separate from proof incompleteness.
    Failed,
}
/// Exact target, configuration and final real observations for inspection.
#[derive(Clone, Debug, Default, Serialize)]
pub struct AnalysisState {
    /// Analysis lifecycle, independent of chess conclusion.
    pub status: AnalysisPhase,
    /// Native target provenance, currently live position only.
    pub target: Option<ExecutionTarget>,
    /// Explicit requested operation and budgets.
    pub operation: Option<AnalysisOperation>,
    /// Exact context retained for technical details and later historical saving.
    pub context: Option<ExecutionContext>,
    /// Actual final child observations; absent on cancelled/unobserved work.
    pub result: Option<ChessResult>,
    /// Plain execution diagnostic, never a chess result.
    pub message: Option<String>,
}
