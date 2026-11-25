//! Read-only analysis acceptance and single-resource play ownership.
use super::*;
use mshogi_engine::{ExecutionTarget, MAX_ANALYSIS_DEPTH, MAX_ANALYSIS_NODES};
/// Explicit user-selected deterministic move-analysis budgets.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisOperation {
    /// Maximum plies.
    pub depth: u16,
    /// Maximum attempted nodes.
    pub nodes: u64,
}
impl AnalysisOperation {
    pub(super) fn validate(self) -> Result<(), String> {
        if (1..=MAX_ANALYSIS_DEPTH).contains(&self.depth) && self.nodes <= MAX_ANALYSIS_NODES {
            Ok(())
        } else {
            Err("Unsupported analysis limits. Move analysis requires depth 1–8 and at most 5,000,000 nodes.".into())
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
    /// Completed minimax iteration.
    Complete,
    /// Observed work limit; no missing work counts are fabricated.
    Incomplete,
    /// Cancelled; no final observations were obtained.
    Cancelled,
    /// Process/protocol/search failure, separate from search incompleteness.
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
