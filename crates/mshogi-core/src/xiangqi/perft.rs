//! Bounded local-board diagnostics. History/no-win outcomes never truncate a tree.
use super::{Counters, LocalPositionError, Position};
use std::{error::Error, fmt};
/// Explicit diagnostic work bound, not an engine search limit.
pub const MAX_PERFT_DEPTH: u8 = 3;
/// Diagnostic refusal, never a chess/Xiangqi outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PerftError {
    /// Root is structural-only or has an impossible turn.
    Position(LocalPositionError),
    /// Requested depth exceeds the documented diagnostic bound.
    DepthLimit,
    /// Count exceeds the output representation.
    CountOverflow,
}
impl fmt::Display for PerftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "xiangqi perft refused: {self:?}")
    }
}
impl Error for PerftError {}
impl Position {
    fn perft_position(&self, depth: u8) -> Result<Self, PerftError> {
        if depth > MAX_PERFT_DEPTH {
            return Err(PerftError::DepthLimit);
        }
        self.validate_local().map_err(PerftError::Position)?;
        let mut position = self.clone();
        // Enumeration is independent of representational counter exhaustion.
        // This local copy is never returned as an actual transitioned game.
        position.counters = Counters::new(Some(0), Some(1)).expect("normalized");
        Ok(position)
    }
    fn count_leaves(&mut self, depth: u8) -> Result<u64, PerftError> {
        if depth == 0 {
            return Ok(1);
        }
        let mut count = 0u64;
        for action in self.legal_actions().map_err(PerftError::Position)? {
            let undo = self
                .apply(action)
                .expect("generated legal action with bounded counters");
            let subtree = self.count_leaves(depth - 1);
            self.undo(undo)
                .expect("exact poststate restored recursively");
            count = count
                .checked_add(subtree?)
                .ok_or(PerftError::CountOverflow)?;
        }
        Ok(count)
    }
    /// Count exact-depth board leaves, depth0=1, positive-depth no-action=0.
    /// Does not adjudicate repetitions, movecounts or impossible-to-win material.
    /// Declared counters are ignored in a temporary copy; self remains exact.
    pub fn perft(&self, depth: u8) -> Result<u64, PerftError> {
        self.perft_position(depth)?.count_leaves(depth)
    }
    /// Root-action counts for depths1..=MAX_PERFT_DEPTH. Depth0 has no root edges.
    pub fn divide(&self, depth: u8) -> Result<Vec<(super::Action, u64)>, PerftError> {
        let mut position = self.perft_position(depth)?;
        if depth == 0 {
            return Ok(Vec::new());
        }
        let mut result = Vec::new();
        for action in position.legal_actions().map_err(PerftError::Position)? {
            let undo = position.apply(action).expect("generated bounded action");
            let count = position.count_leaves(depth - 1);
            position.undo(undo).expect("exact root poststate");
            result.push((action, count?));
        }
        result.sort_by_key(|(action, _)| action.to_string());
        Ok(result)
    }
}
