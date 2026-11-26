//! Exact-depth diagnostics over native legal actions.

use std::{error::Error, fmt};

/// Standard visit allowance for interactive diagnostics and detailed traversals.
pub const STANDARD_VISIT_LIMIT: u64 = 1_000_000;

/// Native operations required by the shared perft and divide traversal.
pub trait PerftPosition: Sized {
    /// A complete native action, including promotion, drop or route choices.
    type Action: Clone + fmt::Debug;
    /// The information needed to restore one private transition.
    type Undo;

    /// Bound recursion depth, independent of the caller's visit allowance.
    const MAX_DEPTH: u32 = 64;
    /// Maximum visit allowance, when the native diagnostic has a tighter bound.
    const MAX_VISITS: u64 = u64::MAX;
    /// A generated legal action can be counted at the horizon without applying it.
    const COUNT_LEGAL_LEAVES: bool = false;

    /// Make the private traversal copy and validate the root where required.
    fn perft_copy(&self) -> Result<Self, TraversalError>;
    /// Generate complete legal actions in native order.
    fn perft_actions(
        &self,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Vec<Self::Action>, TraversalError>;
    /// Apply a generated action to the working copy.
    fn perft_apply(
        &mut self,
        action: &Self::Action,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self::Undo, TraversalError>;
    /// Restore the working copy after one action, including on cancellation.
    fn perft_undo(&mut self, undo: Self::Undo) -> Result<(), String>;
    /// Order the displayed root entries when native generation order differs.
    fn sort_divide_actions(_actions: &mut [Self::Action]) {}
}

/// A failure returned by a native traversal operation.
#[derive(Debug)]
pub enum TraversalError {
    /// The native operation detected cancellation.
    Stopped,
    /// The native diagnostic message.
    Native(String),
}

impl TraversalError {
    pub(crate) fn native(error: impl fmt::Display) -> Self {
        Self::Native(error.to_string())
    }
}

/// Complete count of legal sequences at the requested depth.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PerftReport {
    /// Requested number of complete actions.
    pub depth: u32,
    /// Sequences of exactly the requested depth.
    pub leaves: u64,
    /// Visited nodes at all depths, including the root.
    pub visited: u64,
}

/// A complete root action and its exact subtree count.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DivideEntry<A> {
    /// Complete legal root action.
    pub action: A,
    /// Sequences starting with that action.
    pub leaves: u64,
}

/// Exact divide counts in the game's diagnostic order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DivideReport<A> {
    /// Total count and visits across all root actions.
    pub total: PerftReport,
    /// Exact count for each action in divide order.
    pub entries: Vec<DivideEntry<A>>,
}

/// A refusal or incomplete traversal. No partial counts are returned.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PerftError<A> {
    /// Divide requires at least one ply.
    ZeroDivideDepth,
    /// Requested depth exceeds the native diagnostic limit.
    DepthLimit {
        /// Highest supported depth.
        max: u32,
    },
    /// The requested visit allowance is zero or exceeds the native limit.
    VisitLimit {
        /// Highest supported visit allowance.
        max: u64,
    },
    /// The caller stopped the traversal.
    Stopped {
        /// Native action prefix at cancellation.
        path: Vec<A>,
    },
    /// The traversal exhausted its visit allowance.
    WorkLimit {
        /// Native action prefix to the next unvisited node.
        path: Vec<A>,
        /// Requested visit allowance.
        limit: u64,
    },
    /// A native operation could not continue.
    Traversal {
        /// Native action prefix at failure.
        path: Vec<A>,
        /// Native error message.
        reason: String,
    },
    /// An applied private action could not be restored.
    Restore {
        /// Native action prefix at failure.
        path: Vec<A>,
        /// Native error message.
        reason: String,
    },
    /// The exact count cannot fit in u64.
    CountOverflow {
        /// Native action prefix where the count overflowed.
        path: Vec<A>,
    },
}

impl<A: fmt::Debug> fmt::Display for PerftError<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroDivideDepth => f.write_str("perft: divide requires depth of at least 1"),
            Self::DepthLimit { max } => write!(f, "perft: depth must be in 0..={max}"),
            Self::VisitLimit { max } => write!(f, "perft: visits must be in 1..={max}"),
            Self::Stopped { path } => write!(f, "perft: stopped at {path:?}"),
            Self::WorkLimit { path, limit } => {
                write!(
                    f,
                    "perft: WorkLimit at {path:?} (visit limit {limit} reached)"
                )
            }
            Self::Traversal { path, reason } => write!(f, "perft: traversal at {path:?}: {reason}"),
            Self::Restore { path, reason } => write!(f, "perft: restore at {path:?}: {reason}"),
            Self::CountOverflow { path } => write!(f, "perft: count exceeds u64 at {path:?}"),
        }
    }
}
impl<A: fmt::Debug> Error for PerftError<A> {}

/// Count legal sequences of exactly the requested depth on a private copy.
/// Depth zero counts the root. A position without legal actions contributes
/// zero at positive depth. The budget includes every node, also at depth zero.
pub fn perft<P: PerftPosition>(
    root: &P,
    depth: u32,
    visit_limit: u64,
    stop: impl FnMut() -> bool,
) -> Result<PerftReport, PerftError<P::Action>> {
    run(root, depth, visit_limit, stop, None)
}

/// Count each complete root action's subtree, using one budget across all rows.
pub fn divide<P: PerftPosition>(
    root: &P,
    depth: u32,
    visit_limit: u64,
    stop: impl FnMut() -> bool,
) -> Result<DivideReport<P::Action>, PerftError<P::Action>> {
    if depth == 0 {
        return Err(PerftError::ZeroDivideDepth);
    }
    let mut entries = Vec::new();
    let total = run(root, depth, visit_limit, stop, Some(&mut entries))?;
    Ok(DivideReport { total, entries })
}

fn run<P: PerftPosition>(
    root: &P,
    depth: u32,
    visit_limit: u64,
    stop: impl FnMut() -> bool,
    entries: Option<&mut Vec<DivideEntry<P::Action>>>,
) -> Result<PerftReport, PerftError<P::Action>> {
    if depth > P::MAX_DEPTH {
        return Err(PerftError::DepthLimit { max: P::MAX_DEPTH });
    }
    if !(1..=P::MAX_VISITS).contains(&visit_limit) {
        return Err(PerftError::VisitLimit { max: P::MAX_VISITS });
    }
    let mut walker = Walker {
        visit_limit,
        visited: 0,
        stop,
        path: Vec::new(),
    };
    walker.visit()?;
    let mut working = root.perft_copy().map_err(|error| walker.native(error))?;
    let leaves = walker.walk(&mut working, depth, entries)?;
    Ok(PerftReport {
        depth,
        leaves,
        visited: walker.visited,
    })
}

struct Walker<A, F> {
    visit_limit: u64,
    visited: u64,
    stop: F,
    path: Vec<A>,
}

impl<A: Clone, F: FnMut() -> bool> Walker<A, F> {
    fn native(&self, error: TraversalError) -> PerftError<A> {
        match error {
            TraversalError::Stopped => PerftError::Stopped {
                path: self.path.clone(),
            },
            TraversalError::Native(reason) => PerftError::Traversal {
                path: self.path.clone(),
                reason,
            },
        }
    }

    fn check_stop(&mut self) -> Result<(), PerftError<A>> {
        if (self.stop)() {
            return Err(self.native(TraversalError::Stopped));
        }
        Ok(())
    }

    fn visit(&mut self) -> Result<(), PerftError<A>> {
        self.check_stop()?;
        if self.visited == self.visit_limit {
            return Err(PerftError::WorkLimit {
                path: self.path.clone(),
                limit: self.visit_limit,
            });
        }
        self.visited += 1;
        Ok(())
    }

    fn walk<P: PerftPosition<Action = A>>(
        &mut self,
        position: &mut P,
        depth: u32,
        mut entries: Option<&mut Vec<DivideEntry<A>>>,
    ) -> Result<u64, PerftError<A>> {
        self.check_stop()?;
        if depth == 0 {
            return Ok(1);
        }

        let mut actions = position
            .perft_actions(&mut self.stop)
            .map_err(|error| self.native(error))?;
        self.check_stop()?;
        if entries.is_some() {
            P::sort_divide_actions(&mut actions);
        }

        let mut total = 0u64;
        for action in actions {
            let leaves = self.descend(position, &action, depth - 1)?;
            total = total
                .checked_add(leaves)
                .ok_or_else(|| PerftError::CountOverflow {
                    path: self.path.clone(),
                })?;
            if let Some(rows) = &mut entries {
                rows.push(DivideEntry { action, leaves });
            }
        }
        Ok(total)
    }

    fn descend<P: PerftPosition<Action = A>>(
        &mut self,
        position: &mut P,
        action: &A,
        depth: u32,
    ) -> Result<u64, PerftError<A>> {
        self.path.push(action.clone());
        let result = (|| {
            self.visit()?;
            if depth == 0 && P::COUNT_LEGAL_LEAVES {
                return Ok(1);
            }
            let undo = position
                .perft_apply(action, &mut self.stop)
                .map_err(|error| self.native(error))?;
            let count = self.walk(position, depth, None);
            position
                .perft_undo(undo)
                .map_err(|reason| PerftError::Restore {
                    path: self.path.clone(),
                    reason,
                })?;
            count
        })();
        self.path.pop();
        result
    }
}
