//! Explicit analysis purpose, target provenance and reproducible work budgets.
use crate::{EngineRequest, WorkLimits};
use mshogi_core::chess::{Color, MateTarget};
use serde::{Deserialize, Serialize};
/// Maximum ordinary analysis depth.
pub const MAX_ANALYSIS_DEPTH: u16 = 8;
/// Maximum ordinary analysis node budget.
pub const MAX_ANALYSIS_NODES: u64 = 5_000_000;
/// Maximum retained reachability positions.
pub const MAX_FULL_POSITIONS: u64 = 100_000;
/// Maximum generated reachability transitions.
pub const MAX_FULL_TRANSITIONS: u64 = 1_000_000;
/// Full operations perform no minimax work; their actual budgets are in purpose.
pub const FULL_SENTINEL: WorkLimits = WorkLimits { depth: 1, nodes: 0 };
/// Host-owned target provenance. The exact target history is in request.game.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalysisTarget {
    /// Monotonic selection identity; never inferred from board equality.
    pub selection_id: u64,
    /// Target played ply count.
    pub ply: u64,
    /// Session version at selection.
    pub session_version: u64,
    /// Native readable position label.
    pub label: String,
    /// Root side whose perspective describes minimax scores.
    pub turn: Color,
}
/// Cooperative target; this does not assert a forced mate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReachabilityTarget {
    /// Either side may deliver mate.
    Either,
    /// White delivers mate.
    White,
    /// Black delivers mate.
    Black,
}
impl From<ReachabilityTarget> for MateTarget {
    fn from(value: ReachabilityTarget) -> Self {
        match value {
            ReachabilityTarget::Either => Self::Either,
            ReachabilityTarget::White => Self::Side(Color::White),
            ReachabilityTarget::Black => Self::Side(Color::Black),
        }
    }
}
/// Mandatory deterministic limits for advanced cooperative reachability.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FullLimits {
    /// Maximum retained distinct positions, including root.
    pub max_positions: u64,
    /// Maximum examined legal transitions.
    pub max_transitions: u64,
}
/// Purpose is mandatory and echoed; analysis cannot become an engine turn.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RequestPurpose {
    /// A possible live engine choice, accepted only by the live turn handler.
    EngineTurn,
    /// Read-only minimax observations.
    MoveAnalysis {
        /// Exact native target provenance.
        target: AnalysisTarget,
    },
    /// Read-only Article-3 cooperative reachability, independent of minimax.
    FullReachability {
        /// Exact native target provenance.
        target: AnalysisTarget,
        /// Which side may deliver mate.
        mate: ReachabilityTarget,
        /// Actual deterministic reachability budgets.
        limits: FullLimits,
    },
}
impl EngineRequest {
    /// Check purpose-specific deterministic bounds and static target identity.
    /// This does not replay history or validate final observations.
    pub fn valid_limits(&self) -> bool {
        match &self.purpose {
            RequestPurpose::EngineTurn => {
                (1..=mshogi_search::search::MAX_DEPTH).contains(&self.limits.depth)
            }
            RequestPurpose::MoveAnalysis { target } => {
                valid_target(target, self.session_version)
                    && (1..=MAX_ANALYSIS_DEPTH).contains(&self.limits.depth)
                    && self.limits.nodes <= MAX_ANALYSIS_NODES
            }
            RequestPurpose::FullReachability { target, limits, .. } => {
                valid_target(target, self.session_version)
                    && self.limits == FULL_SENTINEL
                    && limits.max_positions <= MAX_FULL_POSITIONS
                    && limits.max_transitions <= MAX_FULL_TRANSITIONS
            }
        }
    }
}
fn valid_target(target: &AnalysisTarget, version: u64) -> bool {
    target.selection_id > 0
        && target.session_version == version
        && !target.label.is_empty()
        && target.label.len() <= 256
}
