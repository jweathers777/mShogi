//! Chess engine on the shared search core.
//!
//! The native rules decide everything about the real game: a concluded root,
//! the draw claims that are available, and which actions are legal. The shared
//! core searches a private bitboard copy of the position and proposes a move;
//! this adapter checks that move against the native legal actions before it
//! returns it.
use crate::{
    alphabeta::{self, Limits, Searcher, Status},
    chess_bitboard::ChessPosition,
    search::{
        MATE_SCORE, MAX_DEPTH, SearchChoice, SearchDecision, SearchError, SearchLimits,
        SearchResult, SearchStatus,
    },
};
use mshogi_core::chess::{AdjudicatedGame, ChessOutcome, ClaimPolicy};
use std::time::Duration;

/// Search identity for experiment records.
pub const SEARCH_ID: &str = "mshogi.chess.shared-core.v1";
/// Default transposition table size.
pub const DEFAULT_TABLE_MEGABYTES: usize = 64;
/// Largest accepted transposition table size.
pub const MAX_TABLE_MEGABYTES: usize = 4_096;

const _: () = assert!(MATE_SCORE == alphabeta::MATE_SCORE);

/// Soft iteration boundary and hard stop time for one move.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimeBudget {
    /// Do not start a new iteration after this time.
    pub soft: Duration,
    /// Stop the search at this time.
    pub hard: Duration,
}
impl TimeBudget {
    /// Fixed time for one move.
    pub fn from_hard_ms(ms: u64) -> Self {
        Self {
            soft: Duration::from_millis(ms),
            hard: Duration::from_millis(ms),
        }
    }
}

/// Settings of one search request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SearchOptions {
    /// True if the consumer can make optional draw claims for the engine.
    pub allow_draw_claims: bool,
    /// Optional time allocation.
    pub time: Option<TimeBudget>,
}
impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            allow_draw_claims: true,
            time: None,
        }
    }
}

/// Result of one engine search.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EngineResult {
    /// Decision, score, depth, nodes and principal variation.
    pub result: SearchResult,
    /// Largest ply reached, including quiescence.
    pub selective_depth: u16,
    /// Permille of sampled table entries written by this search.
    pub hashfull: u32,
}

/// Chess engine state that lasts for a full game: the transposition table and
/// the move-ordering statistics.
pub struct ChessEngine {
    searcher: Searcher<ChessPosition>,
}
impl Default for ChessEngine {
    fn default() -> Self {
        Self::new(DEFAULT_TABLE_MEGABYTES)
    }
}
impl ChessEngine {
    /// Make an engine with a table of about `megabytes`.
    pub fn new(megabytes: usize) -> Self {
        Self {
            searcher: Searcher::new(megabytes.clamp(1, MAX_TABLE_MEGABYTES)),
        }
    }
    /// Table size in megabytes.
    pub fn table_megabytes(&self) -> usize {
        self.searcher.table_megabytes()
    }
    /// Replace the table with an empty one of a new size.
    pub fn set_table_megabytes(&mut self, megabytes: usize) {
        let megabytes = megabytes.clamp(1, MAX_TABLE_MEGABYTES);
        if megabytes != self.searcher.table_megabytes() {
            self.searcher.resize_table(megabytes);
        }
    }
    /// Forget all earlier analysis: use at the start of a new game.
    pub fn new_game(&mut self) {
        self.searcher.clear();
    }

    /// Search the current position of `game`. The game is not changed.
    ///
    /// A concluded root returns its native outcome. Under `AutoClaim` an
    /// available claim is returned at once. An optional claim is preferred to a
    /// move whose score is not above zero. The stop callback must be cheap.
    pub fn search(
        &mut self,
        game: &AdjudicatedGame,
        limits: SearchLimits,
        options: SearchOptions,
        mut stop: impl FnMut() -> bool,
    ) -> Result<EngineResult, SearchError> {
        if !(1..=MAX_DEPTH).contains(&limits.depth) {
            return Err(SearchError::InvalidDepth(limits.depth));
        }
        let mut result = SearchResult {
            decision: None,
            score: None,
            completed_depth: 0,
            nodes: 0,
            principal_variation: Vec::new(),
            status: SearchStatus::Complete,
        };
        let finish = |result: SearchResult, selective_depth: u16, hashfull: u32| {
            Ok(EngineResult {
                result,
                selective_depth,
                hashfull,
            })
        };
        if limits.nodes == Some(0) {
            result.status = SearchStatus::NodeLimit;
            return finish(result, 0, 0);
        }
        if stop() {
            result.status = SearchStatus::Stopped;
            return finish(result, 0, 0);
        }
        if let Some(outcome) = game.outcome() {
            result.decision = Some(SearchDecision::Finished(outcome));
            result.score = Some(match outcome {
                ChessOutcome::Draw(_) => 0,
                // The side to move is the side that is mated.
                ChessOutcome::Checkmate { .. } => -MATE_SCORE,
            });
            result.completed_depth = 1;
            result.nodes = 1;
            return finish(result, 0, 0);
        }
        let claim = if options.allow_draw_claims {
            game.available_claims().first().copied()
        } else {
            None
        };
        if let Some(claim) = claim
            && game.history().claim_policy() == ClaimPolicy::AutoClaim
        {
            result.decision = Some(SearchDecision::Choose(SearchChoice::Claim(claim)));
            result.score = Some(0);
            result.completed_depth = 1;
            result.nodes = 1;
            result.principal_variation = vec![SearchChoice::Claim(claim)];
            return finish(result, 0, 0);
        }
        // The search board has no fullmove counter. If the native game cannot
        // represent a move from this root, report that at once and do not search.
        let legal = game.position().legal_actions();
        if let Some(&first) = legal.first() {
            let mut probe = game.clone();
            if let Err(error) = probe.apply(first) {
                return Err(SearchError::Transition(error));
            }
        }
        let mut position = ChessPosition::from_game(game);
        let outcome = self.searcher.search(
            &mut position,
            Limits {
                depth: limits.depth,
                nodes: limits.nodes,
                soft: options.time.map(|t| t.soft),
                hard: options.time.map(|t| t.hard),
            },
            &mut stop,
        );
        result.nodes = outcome.nodes;
        result.completed_depth = outcome.depth;
        result.status = match outcome.status {
            Status::Complete => SearchStatus::Complete,
            Status::NodeLimit => SearchStatus::NodeLimit,
            Status::Stopped => SearchStatus::Stopped,
        };
        if let Some(best) = outcome.best {
            let mut action = position.action(best);
            let mut score = outcome.score;
            let mut pv: Vec<_> = outcome
                .pv
                .iter()
                .map(|&m| SearchChoice::Move(position.action(m)))
                .collect();
            // The native rules are the authority: a move that they do not list
            // is never returned.
            if !legal.contains(&action) {
                debug_assert!(false, "search proposed a move that the native rules reject");
                match legal.first() {
                    Some(&first) => {
                        action = first;
                        score = None;
                        pv = vec![SearchChoice::Move(first)];
                    }
                    None => return finish(result, outcome.selective_depth, 0),
                }
            }
            // A drawn line can end with a draw claim that the native rules make
            // available there. Report that claim as the last command of the line;
            // the intended move of a claim is not played.
            if score == Some(0) {
                let mut replay = game.clone();
                let playable = pv.iter().all(|choice| match choice {
                    SearchChoice::Move(action) => replay.apply(*action).is_ok(),
                    SearchChoice::Claim(_) => false,
                });
                if playable
                    && replay.outcome().is_none()
                    && let Some(tail) = replay.available_claims().first().copied()
                {
                    pv.push(SearchChoice::Claim(tail));
                }
            }
            if let Some(claim) = claim
                && score.is_some_and(|s| s <= 0)
            {
                result.decision = Some(SearchDecision::Choose(SearchChoice::Claim(claim)));
                result.score = Some(0);
                result.principal_variation = vec![SearchChoice::Claim(claim)];
            } else {
                result.decision = Some(SearchDecision::Choose(SearchChoice::Move(action)));
                result.score = score;
                result.principal_variation = pv;
            }
        }
        finish(result, outcome.selective_depth, self.searcher.hashfull())
    }
}

#[cfg(test)]
mod tests;
