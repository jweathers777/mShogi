//! Deterministic, full-width chess negamax. No pruning or transposition reuse.

use crate::chess::evaluate;
use mshogi_core::chess::{Action, AdjudicatedGame, ChessOutcome, ClaimRequest, PlayError};
use std::{error::Error, fmt};

/// Search/evaluation configuration identity for experiment records.
pub const SEARCH_ID: &str = "mshogi.chess.full-width.v1";
/// Terminal win score, reduced by distance in plies from the search root.
/// Material scores are strictly smaller in magnitude for validated positions.
pub const MATE_SCORE: i32 = 1_000_000;
/// Bounded recursion; this baseline is intended for shallow correctness work.
pub const MAX_DEPTH: u16 = 128;

/// Deterministic work limits. Node counts include repeated iterative-deepening
/// visits, terminal nodes and leaves. No wall time is part of this contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SearchLimits {
    /// Maximum full-width depth in plies, in 1..=MAX_DEPTH.
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
    /// material; it does not itself conclude a draw. Wins score MATE_SCORE-ply,
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

/// Search privately with deterministic depth/node limits and no external stop.
pub fn search(game: &AdjudicatedGame, limits: SearchLimits) -> Result<SearchResult, SearchError> {
    search_with_stop(game, limits, || false)
}

/// Search a private clone. Poll `stop` before every node visit; cancellation is
/// cooperative at node boundaries, including before the first node. The callback
/// must stay cheap and nonblocking. Timed callbacks make reruns nondeterministic;
/// record their settings, elapsed time and returned observations separately.
///
/// Optional claims compete with moves at value zero (ties prefer a claim).
/// AutoClaim forces the first rules-policy claim at every searched turn,
/// including leaves. Already concluded outcomes precede either policy.
/// Equal-scoring moves prefer lexicographically smaller UCI text. Iterative
/// deepening publishes only full iterations, never a partially explored score.
pub fn search_with_stop(
    game: &AdjudicatedGame,
    limits: SearchLimits,
    mut stop: impl FnMut() -> bool,
) -> Result<SearchResult, SearchError> {
    if !(1..=MAX_DEPTH).contains(&limits.depth) {
        return Err(SearchError::InvalidDepth(limits.depth));
    }
    let mut tree = ChessTree(game.clone());
    let mut work = Work {
        nodes: 0,
        limit: limits.nodes,
        stop: &mut stop,
    };
    let mut result = SearchResult {
        decision: None,
        score: None,
        completed_depth: 0,
        nodes: 0,
        principal_variation: Vec::new(),
        status: SearchStatus::Complete,
    };
    for depth in 1..=limits.depth {
        match negamax(&mut tree, depth, 0, &mut work) {
            Ok(value) => {
                result.decision = value
                    .pv
                    .first()
                    .copied()
                    .map(SearchDecision::Choose)
                    .or_else(|| game.outcome().map(SearchDecision::Finished));
                result.score = Some(value.score);
                result.completed_depth = depth;
                result.principal_variation = value.pv;
                if matches!(result.decision, Some(SearchDecision::Finished(_))) {
                    break;
                }
            }
            Err(Halt::Limit(status)) => {
                result.status = status;
                break;
            }
            Err(Halt::Error(error)) => return Err(error),
        }
    }
    result.nodes = work.nodes;
    Ok(result)
}

// One small private recursion seam allows independently specified finite trees
// to exercise the same backup/selection logic as chess. No public game framework.
trait Tree {
    type Choice: Copy;
    fn terminal(&self, ply: u16) -> Option<i32>;
    fn claim(&self) -> Option<(Self::Choice, bool)>;
    fn evaluate(&self) -> i32;
    fn actions(&self) -> Vec<Self::Choice>;
    fn push(&mut self, choice: Self::Choice) -> Result<(), SearchError>;
    fn pop(&mut self);
}
struct ChessTree(AdjudicatedGame);
impl Tree for ChessTree {
    type Choice = SearchChoice;
    fn terminal(&self, ply: u16) -> Option<i32> {
        self.0.outcome().map(|outcome| match outcome {
            ChessOutcome::Draw(_) => 0,
            ChessOutcome::Checkmate { winner } => {
                if winner == self.0.position().side_to_move() {
                    MATE_SCORE - i32::from(ply)
                } else {
                    -MATE_SCORE + i32::from(ply)
                }
            }
        })
    }
    fn claim(&self) -> Option<(Self::Choice, bool)> {
        use mshogi_core::chess::ClaimPolicy;
        self.0.available_claims().first().map(|&claim| {
            (
                SearchChoice::Claim(claim),
                self.0.history().claim_policy() == ClaimPolicy::AutoClaim,
            )
        })
    }
    fn evaluate(&self) -> i32 {
        evaluate(self.0.position()).score_for(self.0.position().side_to_move())
    }
    fn actions(&self) -> Vec<Self::Choice> {
        let mut actions = self.0.position().legal_actions();
        actions.sort_by_key(ToString::to_string);
        actions.into_iter().map(SearchChoice::Move).collect()
    }
    fn push(&mut self, choice: Self::Choice) -> Result<(), SearchError> {
        let SearchChoice::Move(action) = choice else {
            unreachable!("claims are terminal choices, never board transitions")
        };
        self.0.apply(action).map_err(SearchError::Transition)
    }
    fn pop(&mut self) {
        self.0.undo().expect("balanced private search transition");
    }
}
struct Value<A> {
    score: i32,
    pv: Vec<A>,
}
enum Halt {
    Limit(SearchStatus),
    Error(SearchError),
}
struct Work<'a> {
    nodes: u64,
    limit: Option<u64>,
    stop: &'a mut dyn FnMut() -> bool,
}
impl Work<'_> {
    fn visit(&mut self) -> Result<(), Halt> {
        if (self.stop)() {
            return Err(Halt::Limit(SearchStatus::Stopped));
        }
        if self.limit.is_some_and(|limit| self.nodes >= limit) {
            return Err(Halt::Limit(SearchStatus::NodeLimit));
        }
        self.nodes += 1;
        Ok(())
    }
}
fn negamax<T: Tree>(
    tree: &mut T,
    depth: u16,
    ply: u16,
    work: &mut Work<'_>,
) -> Result<Value<T::Choice>, Halt> {
    work.visit()?;
    if let Some(score) = tree.terminal(ply) {
        return Ok(Value { score, pv: vec![] });
    }
    let claim = tree.claim();
    if let Some((choice, true)) = claim {
        return Ok(Value {
            score: 0,
            pv: vec![choice],
        });
    }
    if depth == 0 {
        let score = tree.evaluate();
        return Ok(if let Some((choice, _)) = claim.filter(|_| score <= 0) {
            Value {
                score: 0,
                pv: vec![choice],
            }
        } else {
            Value { score, pv: vec![] }
        });
    }
    let mut best = claim.map(|(choice, _)| Value {
        score: 0,
        pv: vec![choice],
    });
    for choice in tree.actions() {
        tree.push(choice).map_err(Halt::Error)?;
        let child = negamax(tree, depth - 1, ply + 1, work);
        tree.pop(); // Restore even when a child stopped or failed.
        let child = child?;
        let score = -child.score;
        if best.as_ref().is_none_or(|best| score > best.score) {
            let mut pv = vec![choice];
            pv.extend(child.pv);
            best = Some(Value { score, pv });
        }
    }
    Ok(best.expect("nonterminal chess position has a legal action"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Static leaf numbers explicitly use the local side's perspective.
    // Root A: min(7, 3)=3; root B: min(5, 9)=5; therefore choose B.
    // The same graph at depth 1 deliberately picks A (local eval -8 vs -2).
    struct Synthetic {
        at: usize,
        path: Vec<usize>,
    }
    impl Tree for Synthetic {
        type Choice = usize;
        fn terminal(&self, _: u16) -> Option<i32> {
            None
        }
        fn claim(&self) -> Option<(usize, bool)> {
            None
        }
        fn evaluate(&self) -> i32 {
            [0, -8, -2, 7, 3, 5, 9][self.at]
        }
        fn actions(&self) -> Vec<usize> {
            match self.at {
                0 => vec![1, 2],
                1 => vec![3, 4],
                2 => vec![5, 6],
                _ => vec![],
            }
        }
        fn push(&mut self, choice: usize) -> Result<(), SearchError> {
            self.path.push(self.at);
            self.at = choice;
            Ok(())
        }
        fn pop(&mut self) {
            self.at = self.path.pop().unwrap();
        }
    }
    #[test]
    fn independent_minimax_tree_checks_depth_perspective_and_full_width() {
        let mut tree = Synthetic {
            at: 0,
            path: vec![],
        };
        let mut stop = || false;
        let mut work = Work {
            nodes: 0,
            limit: None,
            stop: &mut stop,
        };
        let value =
            negamax(&mut tree, 1, 0, &mut work).unwrap_or_else(|_| panic!("unexpected halt"));
        assert_eq!((value.score, value.pv), (8, vec![1]));
        assert_eq!(work.nodes, 3);
        work.nodes = 0;
        let value =
            negamax(&mut tree, 2, 0, &mut work).unwrap_or_else(|_| panic!("unexpected halt"));
        assert_eq!((value.score, value.pv), (5, vec![2, 5]));
        assert_eq!(work.nodes, 7); // No pruning, including inferior A branches.
        assert_eq!(tree.at, 0);
        assert!(tree.path.is_empty());
    }
    #[test]
    fn halted_tree_unwinds_private_transitions() {
        let mut tree = Synthetic {
            at: 0,
            path: vec![],
        };
        let mut stop = || false;
        let mut work = Work {
            nodes: 0,
            limit: Some(4),
            stop: &mut stop,
        };
        assert!(matches!(
            negamax(&mut tree, 2, 0, &mut work),
            Err(Halt::Limit(SearchStatus::NodeLimit))
        ));
        assert_eq!(tree.at, 0);
        assert!(tree.path.is_empty());
    }
}
