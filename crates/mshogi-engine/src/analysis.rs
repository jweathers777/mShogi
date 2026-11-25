//! Bounded work budgets for read-only move analysis.
/// Maximum ordinary analysis depth.
pub const MAX_ANALYSIS_DEPTH: u16 = 8;
/// Maximum ordinary analysis node budget.
pub const MAX_ANALYSIS_NODES: u64 = 5_000_000;
