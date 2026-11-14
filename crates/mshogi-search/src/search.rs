//! Chess search limits, results and errors shared by the Chess engine, its
//! coordinator and the protocol and desktop consumers.

use mshogi_core::chess::{Action, ChessOutcome, ClaimRequest, PlayError};
use std::{error::Error, fmt};

/// Terminal win score, reduced by distance in plies from the search root.
/// Static evaluation scores are strictly smaller in magnitude.
pub const MATE_SCORE: i32 = 1_000_000;
/// Maximum nominal search depth in plies.
pub const MAX_DEPTH: u16 = 128;

/// Deterministic work limits. Node counts include repeated iterative-deepening
/// visits, terminal nodes and leaves. No wall time is part of this contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SearchLimits {
    /// Maximum nominal depth in plies, in 1..=MAX_DEPTH. The engine reports
    /// its actual selective depth separately.
    pub depth: u16,
    /// Stop before visiting a node beyond this total; zero visits no nodes.
    pub nodes: Option<u64>,
}

/// A complete choice; a draw claim is never disguised as a board action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SearchChoice {
    /// Legal complete board action.
    Move(Action),
    /// Valid current or intended-move claim; intended moves are not played.
    Claim(ClaimRequest),
}

/// Root decision from a fully completed iteration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SearchDecision {
    /// An ordinary move or optional/required policy claim.
    Choose(SearchChoice),
    /// The rules-owned root is already concluded.
    Finished(ChessOutcome),
}

/// Why execution ended; a limit never establishes a chess outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SearchStatus {
    /// Requested depth completed, or root was already concluded.
    Complete,
    /// Deterministic node budget was exhausted before the next iteration finished.
    NodeLimit,
    /// External stop predicate requested cancellation or a timed stop.
    Stopped,
}

/// Last fully completed iteration, plus observations of all attempted work.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchResult {
    /// None if stopped before the first full iteration. Consumers needing an
    /// immediate move must choose and label their own legal fallback.
    pub decision: Option<SearchDecision>,
    /// Root side-to-move perspective. Zero evaluates a draw/claim or equal
    /// heuristic balance; it does not itself conclude a draw. Wins score MATE_SCORE-ply,
    /// losses -MATE_SCORE+ply. None when no iteration completed.
    pub score: Option<i32>,
    /// Last fully completed depth; zero when no iteration completed.
    pub completed_depth: u16,
    /// Total node visits, including discarded incomplete iterations.
    pub nodes: u64,
    /// Chosen continuation of the completed iteration; can end with a claim.
    /// Empty for an already concluded root or before any iteration completes.
    pub principal_variation: Vec<SearchChoice>,
    /// Completion, deterministic work exhaustion, or externally requested stop.
    pub status: SearchStatus,
}

/// Invalid settings or a representational failure, never an invented result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SearchError {
    /// Depth is outside 1..=MAX_DEPTH.
    InvalidDepth(u16),
    /// Rules-generated action could not be represented/applied (e.g. counter
    /// overflow). The caller remains unchanged and no partial score is returned.
    Transition(PlayError),
}
impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "search failed: {self:?}")
    }
}
impl Error for SearchError {}
