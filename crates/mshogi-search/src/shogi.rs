//! Shogi adapter for the game-independent alpha-beta core.
pub mod eval;
pub mod position;
#[cfg(test)]
mod position_tests;
use crate::alphabeta::{self, SearchPosition, Searcher, Status};
use crate::search::SearchStatus;
use mshogi_core::shogi::{Action, Conclusion, Game, GameCommand, GameError, GameRequest, Side};
use position::ShogiPosition;
use std::{error::Error, fmt};

/// Classical material and placement evaluation identity.
pub const EVALUATOR_ID: &str = "mshogi.shogi.material-placement.v1";
/// Shared alpha-beta search identity.
pub const SEARCH_ID: &str = "mshogi.shogi.shared-core.v1";
/// Maximum reported native continuation length.
pub const MAX_LINE: usize = 8;
/// Native terminal win, minus command-edge distance (not necessarily checkmate).
pub const WIN_SCORE: i32 = alphabeta::MATE_SCORE;
/// Maximum nominal depth.
pub const MAX_DEPTH: u16 = 64;
/// Maximum total admitted visits, including iterative revisits and discarded work.
pub const MAX_NODES: u64 = 5_000_000;
/// Interactive node budget; the node limit normally ends a request.
pub const DEFAULT_LIMITS: Limits = Limits {
    depth: MAX_DEPTH,
    nodes: 300_000,
};

/// Mandatory deterministic limits. Zero nodes visits nothing and publishes no choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    /// Maximum nominal depth, 1..=MAX_DEPTH.
    pub depth: u16,
    /// Total node budget, 0..=MAX_NODES; no unbounded native shogi entry point.
    pub nodes: u64,
}
impl Limits {
    /// The same supported bounds apply in the native API and supervised transport.
    pub const fn valid(self) -> bool {
        self.depth >= 1 && self.depth <= MAX_DEPTH && self.nodes <= MAX_NODES
    }
}
/// Native complete action versus explicit terminal declaration choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Choice {
    /// A complete legal board move/drop/promotion.
    Move(Action),
    /// Formal entering-king declaration for the native actor; not a board action.
    Declare {
        /// Native declarer; submitting a declaration does not change board turn.
        actor: Side,
    },
}
impl Choice {
    /// Retain native command semantics; admission still belongs to Game::submit.
    pub const fn command(self) -> GameCommand {
        match self {
            Self::Move(action) => GameCommand::Play(action),
            Self::Declare { .. } => GameCommand::DeclareEnteringKing,
        }
    }
}
/// Completed decision or an explicitly unscored timed legal fallback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decision {
    /// A proposed native choice, not an already applied command.
    Choose(Choice),
    /// The exact rules-owned root conclusion.
    Finished(Conclusion),
}
/// Actual observations. All scores use the captured native root side's perspective.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchResult {
    /// A timed live request can retain a legal root choice at depth zero.
    pub decision: Option<Decision>,
    /// Root-side score; terminal distance counts board/declaration command edges.
    pub score: Option<i32>,
    /// Native root side used for each score.
    pub perspective: Side,
    /// Completed nominal depth; declarations are checked at the root.
    pub completed_depth: u16,
    /// Admitted visits, including quiescence and partial work.
    pub nodes: u64,
    /// Completed continuation, or the one unsearched timed fallback move.
    pub principal_variation: Vec<Choice>,
    /// Actual completion, finite budget exhaustion or cooperative stop.
    pub status: SearchStatus,
}
/// Invalid settings or native transition/invariant/restoration failure; no score.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SearchError {
    /// Outside the documented bounded native settings.
    InvalidLimits,
    /// Native diagnostic retained, including mutual-check invariant evidence.
    Transition(GameError),
    /// A balanced private reversal failed or final root was not exactly restored.
    Restore,
    /// A native live node unexpectedly had no choice (never invent its outcome).
    Invariant,
}
impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "shogi search failed: {self:?}")
    }
}
impl Error for SearchError {}
/// Run deterministically with mandatory finite work settings.
pub fn search(game: &Game, limits: Limits) -> Result<SearchResult, SearchError> {
    search_with_stop(game, limits, || false)
}
/// Persistent search state for one Shogi game. Native history stays outside it.
pub type Engine = Searcher<ShogiPosition>;

/// Default size of the search table in megabytes.
pub const DEFAULT_TABLE_MB: usize = 16;

/// Start with an empty transposition table and move statistics.
pub fn new_engine() -> Engine {
    Searcher::new(DEFAULT_TABLE_MB)
}

/// Search a private position with cooperative stop polling. A partial
/// iteration contributes work only, preserving the last complete iteration.
/// Each call starts with empty state.
pub fn search_with_stop(
    game: &Game,
    limits: Limits,
    stop: impl FnMut() -> bool,
) -> Result<SearchResult, SearchError> {
    search_with_state(game, limits, None, &mut new_engine(), stop)
}

/// Search with caller-owned state and an optional total adapter time budget.
/// Use a fresh engine for reproducible fixed-limit requests.
/// A timed live request retains a native legal root-order move if no iteration
/// completes. That choice has no score or completed depth. External cancellation
/// still returns no fallback; native terminal and declaration decisions stay
/// authoritative. The mandatory root checks can outlast a minimal time budget.
pub fn search_with_state(
    game: &Game,
    limits: Limits,
    time: Option<alphabeta::timing::TimeBudget>,
    engine: &mut Engine,
    mut stop: impl FnMut() -> bool,
) -> Result<SearchResult, SearchError> {
    let timing = alphabeta::timing::RequestTime::new(time);
    if !limits.valid() {
        return Err(SearchError::InvalidLimits);
    }
    let root_side = game.position().side_to_move();
    let mut result = SearchResult {
        decision: None,
        score: None,
        perspective: root_side,
        completed_depth: 0,
        nodes: 0,
        principal_variation: vec![],
        status: SearchStatus::Complete,
    };
    if stop() {
        result.status = SearchStatus::Stopped;
        return Ok(result);
    }
    if limits.nodes == 0 && time.is_none() {
        result.status = SearchStatus::NodeLimit;
        return Ok(result);
    }
    if let Some(conclusion) = game.conclusion() {
        result.decision = Some(Decision::Finished(conclusion));
        result.score = Some(terminal_score(conclusion, root_side, 0));
        // With no search work admitted, observe the existing native result
        // without claiming an iteration or a node visit.
        result.completed_depth = u16::from(limits.nodes > 0);
        result.nodes = u64::from(limits.nodes > 0);
        return Ok(result);
    }
    if game.declaration_eligibility(root_side).eligible() && (time.is_none() || limits.nodes >= 2) {
        result.nodes = 1;
        if stop() {
            result.status = SearchStatus::Stopped;
            return Ok(result);
        }
        if limits.nodes < 2 {
            result.status = SearchStatus::NodeLimit;
            return Ok(result);
        }
        let choice = Choice::Declare { actor: root_side };
        let mut native = game.clone();
        native
            .submit(GameRequest {
                expected: native.context(),
                actor: root_side,
                command: choice.command(),
            })
            .map_err(SearchError::Transition)?;
        let conclusion = native.conclusion().ok_or(SearchError::Invariant)?;
        result.decision = Some(Decision::Choose(choice));
        result.score = Some(terminal_score(conclusion, root_side, 1));
        result.completed_depth = 1;
        result.nodes = 2;
        result.principal_variation.push(choice);
        return Ok(result);
    }
    let legal = game.position().legal_actions();
    if legal.is_empty() {
        return Err(SearchError::Invariant);
    }
    let mut position = ShogiPosition::from_game(game);
    let outcome = if time.is_some() {
        let fallback = position.find(legal[0]).ok_or(SearchError::Invariant)?;
        engine.search_with_timed_fallback(
            &mut position,
            timing.limits(limits.depth, Some(limits.nodes)),
            &mut stop,
            fallback,
        )
    } else {
        engine.search(
            &mut position,
            timing.limits(limits.depth, Some(limits.nodes)),
            &mut stop,
        )
    };
    result.nodes = outcome.nodes;
    result.completed_depth = outcome.depth;
    result.status = match outcome.status {
        Status::Complete => SearchStatus::Complete,
        Status::NodeLimit => SearchStatus::NodeLimit,
        Status::Stopped => SearchStatus::Stopped,
    };
    let Some(best) = outcome.best else {
        result.completed_depth = 0;
        return Ok(result);
    };
    let action = position.action(best);
    if !legal.contains(&action) {
        return Err(SearchError::Invariant);
    }
    let mut native = game.clone();
    for &m in outcome.pv.iter().take(MAX_LINE) {
        if !result.principal_variation.is_empty() && timing.expired() {
            break;
        }
        if native.conclusion().is_some() {
            break;
        }
        let action = position.action(m);
        let request = GameRequest {
            expected: native.context(),
            actor: native.position().side_to_move(),
            command: GameCommand::Play(action),
        };
        match native.submit(request) {
            Ok(_) => {}
            Err(error) if result.principal_variation.is_empty() => {
                return Err(SearchError::Transition(error));
            }
            Err(_) => break,
        }
        if !position.make(m) {
            return Err(SearchError::Invariant);
        }
        result.principal_variation.push(Choice::Move(action));
    }
    if result.principal_variation.first() != Some(&Choice::Move(action)) {
        return Err(SearchError::Invariant);
    }
    result.decision = Some(Decision::Choose(Choice::Move(action)));
    result.score = outcome.score;
    Ok(result)
}
/// Native terminal score from the named root side.
pub fn terminal_score(conclusion: Conclusion, side: Side, distance: u16) -> i32 {
    match conclusion {
        Conclusion::Draw(_) => 0,
        Conclusion::Win { winner, .. } => {
            if winner == side {
                WIN_SCORE - i32::from(distance)
            } else {
                -WIN_SCORE + i32::from(distance)
            }
        }
    }
}
