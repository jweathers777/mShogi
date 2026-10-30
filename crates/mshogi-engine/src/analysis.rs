//! Explicit analysis purpose, target provenance and reproducible work budgets.
use crate::WorkLimits;
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
