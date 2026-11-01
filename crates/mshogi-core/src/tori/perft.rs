//! Explicitly bounded diagnostics over native complete local choices, not search.
use super::{Action, ActionError, LocalPosition, ReversalError};
use std::{error::Error, fmt};
/// Maximum explicit diagnostic depth; ordinary queries perform no tree traversal.
pub const MAX_PERFT_DEPTH: u8 = 8;
/// Absolute visit ceiling; caller also supplies its own smaller bound.
pub const MAX_PERFT_VISITS: u64 = 1_000_000;
/// Complete diagnostic result; incomplete traversals return an error, never a count.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PerftReport {
    /// Requested board-action depth.
    pub depth: u8,
    /// Leaves at exactly that depth (depth0 includes the root).
    pub leaves: u64,
    /// All visited nodes including root; also the metered work unit.
    pub visited: u64,
}
/// One root action's exact subtree count.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DivideEntry {
    /// Native complete root action.
    pub action: Action,
    /// Leaves below this action at the requested remaining depth.
    pub leaves: u64,
}
/// Complete deterministic divide; all rows share one visit budget.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DivideReport {
    /// Total exact traversal result.
    pub total: PerftReport,
    /// Same canonical order as legal_actions.
    pub entries: Vec<DivideEntry>,
}
/// Explicit incomplete diagnostic, with the exact native path where applicable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PerftError {
    /// Unsupported depth or visit bound; no traversal occurred.
    Limits,
    /// Caller visit budget exhausted; no partial count is returned as exact.
    WorkLimit {
        /// Action prefix reaching the next unvisited node.
        path: Vec<Action>,
    },
    /// A supposedly enumerated action could not be committed on the private copy.
    Action {
        /// Exact native prefix.
        path: Vec<Action>,
        /// Actual native error, never relabelled as a game result.
        error: ActionError,
    },
    /// Native reversal failed on the private diagnostic copy.
    Reversal {
        /// Exact native prefix.
        path: Vec<Action>,
        /// Actual restoration failure.
        error: ReversalError,
    },
    /// A diagnostic counter/transition could not be represented.
    Capacity {
        /// Exact native action prefix.
        path: Vec<Action>,
    },
}
impl fmt::Display for PerftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tori local perft incomplete: {self:?}")
    }
}
impl Error for PerftError {}
struct Budget {
    used: u64,
    limit: u64,
}
impl Budget {
    fn visit(&mut self, path: &[Action]) -> Result<(), PerftError> {
        if self.used == self.limit {
            return Err(PerftError::WorkLimit {
                path: path.to_vec(),
            });
        }
        self.used += 1;
        Ok(())
    }
}
fn restore(
    p: &mut LocalPosition,
    undo: &mut super::LocalUndo,
    path: &[Action],
) -> Result<(), PerftError> {
    p.undo(undo).map_err(|error| PerftError::Reversal {
        path: path.to_vec(),
        error,
    })?;
    Ok(())
}
fn walk(
    p: &mut LocalPosition,
    depth: u8,
    budget: &mut Budget,
    path: &mut Vec<Action>,
) -> Result<u64, PerftError> {
    budget.visit(path)?;
    if depth == 0 {
        return Ok(1);
    }
    let mut leaves = 0u64;
    for action in p.legal_actions() {
        path.push(action);
        let (_, mut undo) = p.apply(action).map_err(|error| PerftError::Action {
            path: path.clone(),
            error,
        })?;
        let count = walk(p, depth - 1, budget, path)?;
        restore(p, &mut undo, path)?;
        path.pop();
        leaves = leaves
            .checked_add(count)
            .ok_or_else(|| PerftError::Capacity { path: path.clone() })?;
    }
    Ok(leaves)
}
fn setup(p: &LocalPosition, depth: u8, visits: u64) -> Result<(LocalPosition, Budget), PerftError> {
    if depth > MAX_PERFT_DEPTH || visits == 0 || visits > MAX_PERFT_VISITS {
        return Err(PerftError::Limits);
    }
    // Board diagnostics ignore declared counters, just like legal_actions. The
    // private copy is never installed and cannot change the original provenance.
    let copy = p.board_probe();
    Ok((
        copy,
        Budget {
            used: 0,
            limit: visits,
        },
    ))
}
impl LocalPosition {
    /// Exact leaf count if the explicit depth/work budget completes. No history,
    /// engine work or analysis is started; caller's position/tokens are untouched.
    pub fn perft(&self, depth: u8, max_visits: u64) -> Result<PerftReport, PerftError> {
        let (mut p, mut budget) = setup(self, depth, max_visits)?;
        let leaves = walk(&mut p, depth, &mut budget, &mut Vec::new())?;
        Ok(PerftReport {
            depth,
            leaves,
            visited: budget.used,
        })
    }
    /// Deterministic root-action diagnostics. Depth must be1..=MAX_PERFT_DEPTH;
    /// work is metered across the entire divide, not separately for each row.
    pub fn divide(&self, depth: u8, max_visits: u64) -> Result<DivideReport, PerftError> {
        if depth == 0 {
            return Err(PerftError::Limits);
        }
        let (mut p, mut budget) = setup(self, depth, max_visits)?;
        budget.visit(&[])?;
        let mut entries = Vec::new();
        let mut leaves = 0u64;
        for action in p.legal_actions() {
            let mut path = vec![action];
            let (_, mut undo) = p.apply(action).map_err(|error| PerftError::Action {
                path: path.clone(),
                error,
            })?;
            let n = walk(&mut p, depth - 1, &mut budget, &mut path)?;
            restore(&mut p, &mut undo, &path)?;
            leaves = leaves
                .checked_add(n)
                .ok_or_else(|| PerftError::Capacity { path: path.clone() })?;
            entries.push(DivideEntry { action, leaves: n });
        }
        Ok(DivideReport {
            total: PerftReport {
                depth,
                leaves,
                visited: budget.used,
            },
            entries,
        })
    }
}
