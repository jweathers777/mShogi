//! Local legal-sequence diagnostics; no history-dependent stopping or search.
use super::{Action, ActionError, Position, UndoError};
use std::{error::Error, fmt};

/// One complete root choice and its sequence count.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DivideEntry {
    /// Complete root action, including promotion/drop identity.
    pub action: Action,
    /// Sequences below this root.
    pub nodes: u64,
}
/// A diagnostic failed explicitly; no branch is silently dropped.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PerftError {
    /// Divide has no root actions at depth zero.
    ZeroDivideDepth,
    /// Count cannot fit in u64.
    CountOverflow,
    /// Unexpected failure applying a listed legal action.
    Transition(ActionError),
    /// Unexpected failure reversing a traversed action.
    Restore(UndoError),
}
impl fmt::Display for PerftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "shogi perft failed: {self:?}")
    }
}
impl Error for PerftError {}
impl Position {
    /// Count complete legal sequences of exactly `depth` plies.
    ///
    /// Depth zero is one, including positions with no legal action; a no-action
    /// node contributes zero at positive depth. This ignores match history,
    /// clocks and length conclusions. Counters are normalized on a private copy
    /// so representation limits do not prune branches. Cost is exponential:
    /// diagnostic callers must choose a bounded depth, not use this as search.
    pub fn perft(&self, depth: u32) -> Result<u64, PerftError> {
        self.normalized_for_board_queries().count_sequences(depth)
    }
    /// Root divide in deterministic USI lexical order. Depth must be positive.
    /// Counts sum to `perft(depth)`; the caller's complete state is untouched.
    pub fn divide(&self, depth: u32) -> Result<Vec<DivideEntry>, PerftError> {
        if depth == 0 {
            return Err(PerftError::ZeroDivideDepth);
        }
        let mut working = self.normalized_for_board_queries();
        let mut entries = Vec::new();
        for action in working.legal_actions() {
            let undo = working.apply(action).map_err(PerftError::Transition)?;
            let nodes = working.count_sequences(depth - 1);
            working.undo(undo).map_err(PerftError::Restore)?;
            entries.push(DivideEntry {
                action,
                nodes: nodes?,
            });
        }
        Ok(entries)
    }
    fn count_sequences(&mut self, depth: u32) -> Result<u64, PerftError> {
        if depth == 0 {
            return Ok(1);
        }
        let actions = self.legal_actions();
        if depth == 1 {
            return Ok(actions.len() as u64);
        }
        let mut total = 0_u64;
        for action in actions {
            let undo = self.apply(action).map_err(PerftError::Transition)?;
            let nodes = self.count_sequences(depth - 1);
            self.undo(undo).map_err(PerftError::Restore)?;
            total = total.checked_add(nodes?).ok_or(PerftError::CountOverflow)?;
        }
        Ok(total)
    }
}
