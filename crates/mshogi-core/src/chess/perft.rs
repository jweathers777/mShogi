//! Perft and divide move-tree diagnostics for `mshogi.chess.position.v1`.
//!
//! These count legal action sequences; they are correctness diagnostics, not a
//! search, an evaluation or a strength measurement.

use std::{error::Error, fmt};

use super::{Action, ActionError, Position, Promotion, UndoError};

/// One legal root action and the number of legal sequences below it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DivideEntry {
    /// The complete root action.
    pub action: Action,
    /// `perft(depth - 1)` of the position after `action`.
    pub nodes: u64,
}

/// A perft or divide request that could not be answered.
///
/// No variant is a silent skip: traversal either counts every legal branch or
/// stops with one of these errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PerftError {
    /// Divide needs at least one ply: depth zero has no root actions.
    ZeroDivideDepth,
    /// The node count does not fit in `u64`.
    CountOverflow,
    /// A listed legal action could not be applied during traversal. Counter
    /// normalization makes this unreachable; it is reported, never skipped.
    Traversal(ActionError),
    /// Restoring a traversed action failed; reported, never ignored.
    Restore(UndoError),
}

impl fmt::Display for PerftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroDivideDepth => f.write_str("perft: divide requires depth of at least 1"),
            Self::CountOverflow => f.write_str("perft: node count exceeds u64"),
            Self::Traversal(error) => write!(f, "perft: traversal stopped: {error}"),
            Self::Restore(error) => write!(f, "perft: traversal stopped: {error}"),
        }
    }
}

impl Error for PerftError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Traversal(error) => Some(error),
            Self::Restore(error) => Some(error),
            _ => None,
        }
    }
}

impl Position {
    /// Count the legal action sequences of exactly `depth` plies.
    ///
    /// Depth zero is 1, even at checkmate or stalemate. A position with no legal
    /// action contributes 0 at any positive depth. Nothing stops early at a
    /// draw counter, a repetition or an insufficient-material shortcut.
    ///
    /// Counters never limit traversal: it runs on a private copy whose counters
    /// are normalized at the root, so a starting position at the u32 limit
    /// yields the same count as one with ordinary counters. From that copy no
    /// increment can overflow within any `u32` depth, so no legal branch is
    /// dropped; an unexpected failure is reported as an error, never skipped or
    /// counted as a leaf. The caller's position is not modified.
    ///
    /// Cost grows exponentially with depth; callers choose bounded depths.
    ///
    /// ```
    /// use mshogi_core::chess::Position;
    /// let position = Position::from_fen(
    ///     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    /// )?;
    /// assert_eq!(position.perft(0)?, 1);
    /// assert_eq!(position.perft(2)?, 400);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn perft(&self, depth: u32) -> Result<u64, PerftError> {
        self.normalized().count(depth)
    }

    /// Every legal root action with `perft(depth - 1)` of its result.
    ///
    /// The counts sum to `perft(depth)`. Entries are complete actions, each
    /// promotion choice separately, ordered by source square (a1 to h8, file
    /// fastest), then destination square, then promotion (none, queen, rook,
    /// bishop, knight). Comparing two divides in this order and descending
    /// into the first differing root action localizes a disagreement.
    /// Depth zero is rejected with [`PerftError::ZeroDivideDepth`].
    pub fn divide(&self, depth: u32) -> Result<Vec<DivideEntry>, PerftError> {
        if depth == 0 {
            return Err(PerftError::ZeroDivideDepth);
        }
        let mut working = self.normalized();
        let mut actions = working.legal_actions();
        actions.sort_by_key(|action| order_key(*action));
        let mut entries = Vec::with_capacity(actions.len());
        for action in actions {
            let record = working.apply(action).map_err(PerftError::Traversal)?;
            let nodes = working.count(depth - 1);
            working.undo(&record).map_err(PerftError::Restore)?;
            entries.push(DivideEntry {
                action,
                nodes: nodes?,
            });
        }
        Ok(entries)
    }

    // Counters never affect move generation; see `legal_actions`.
    fn normalized(&self) -> Position {
        let mut copy = self.clone();
        copy.halfmove_clock = 0;
        copy.fullmove_number = 1;
        copy
    }

    fn count(&mut self, depth: u32) -> Result<u64, PerftError> {
        if depth == 0 {
            return Ok(1);
        }
        let actions = self.legal_actions();
        if depth == 1 {
            return Ok(actions.len() as u64);
        }
        let mut total: u64 = 0;
        for action in actions {
            let record = self.apply(action).map_err(PerftError::Traversal)?;
            let nodes = self.count(depth - 1);
            self.undo(&record).map_err(PerftError::Restore)?;
            total = total.checked_add(nodes?).ok_or(PerftError::CountOverflow)?;
        }
        Ok(total)
    }
}

fn order_key(action: Action) -> (u8, u8, u8) {
    let promotion = match action.promotion {
        None => 0,
        Some(Promotion::Queen) => 1,
        Some(Promotion::Rook) => 2,
        Some(Promotion::Bishop) => 3,
        Some(Promotion::Knight) => 4,
    };
    (
        action.from.rank() * 8 + action.from.file(),
        action.to.rank() * 8 + action.to.file(),
        promotion,
    )
}
