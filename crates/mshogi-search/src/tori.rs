//! Tori Shogi engine on the shared search core.
//!
//! The native `Game` decides everything about the real game: a concluded
//! root, the legal actions (with the departures that the repetition rule
//! forbids) and each action of the reported line. The shared core searches a
//! private [`position::ToriPosition`] and proposes a move; this adapter returns
//! it only if the native legal actions contain it.
pub mod position;

use crate::{
    alphabeta::{self, SearchPosition, Searcher, Status},
    search::SearchStatus,
};
use mshogi_core::tori::{
    Action, Conclusion, Face, Game, GameCommand, GameError, GameRecord, GameRequest, Position,
    RecordError, Side, Square,
};
use position::ToriPosition;
pub use position::piece_value;
use std::{error::Error, fmt};

/// Evaluation identity: material with Swallow advance, Phoenix cover and hand terms.
pub const EVALUATOR_ID: &str = "mshogi.tori.material-placement.v1";
/// Search identity: the shared alpha-beta core.
pub const SEARCH_ID: &str = "mshogi.tori.shared-core.v1";
/// Evaluation identity of the earlier full-width search. Saved observations
/// with this identity stay valid; see [`evaluate`].
pub const LEGACY_EVALUATOR_ID: &str = "mshogi.tori.material.v1";
/// Search identity of the earlier full-width search.
pub const LEGACY_SEARCH_ID: &str = "mshogi.tori.full-width.v1";
/// Depth bound of the earlier full-width search.
pub const LEGACY_MAX_DEPTH: u16 = 3;
/// Node bound of the earlier full-width search.
pub const LEGACY_MAX_NODES: u64 = 10_000;
/// Native terminal win minus board-action distance; not specifically a mate score.
pub const WIN_SCORE: i32 = alphabeta::MATE_SCORE;
/// Largest nominal depth of a request.
pub const MAX_DEPTH: u16 = 64;
/// Largest node limit of a request.
pub const MAX_NODES: u64 = 5_000_000;
/// Largest length of a reported line, in board actions. A saved document
/// keeps each line and replays it when it opens, so the line is kept short.
pub const MAX_LINE: usize = 8;
/// Interactive budget: the node limit ends the search.
pub const DEFAULT_LIMITS: Limits = Limits {
    depth: MAX_DEPTH,
    nodes: 300_000,
};
const TABLE_MEGABYTES: usize = 16;

/// Material of each side on the board and in hand.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Evaluation {
    /// Sente material.
    pub sente: i32,
    /// Gote material.
    pub gote: i32,
}
impl Evaluation {
    /// Positive favors this named native side.
    pub const fn score_for(self, side: Side) -> i32 {
        match side {
            Side::Sente => self.sente - self.gote,
            Side::Gote => self.gote - self.sente,
        }
    }
}
/// Material of a native position. This was the full evaluation of the earlier
/// full-width search; saved observations of that search are checked with it.
pub fn evaluate(position: &Position) -> Evaluation {
    let mut e = Evaluation { sente: 0, gote: 0 };
    for side in [Side::Sente, Side::Gote] {
        let value = Square::all()
            .filter_map(|square| position.piece(square))
            .filter(|p| p.side == side)
            .map(|p| piece_value(p.face))
            .sum::<i32>()
            + Face::HANDS
                .into_iter()
                .map(|f| i32::from(position.hand(side).count(f)) * piece_value(f))
                .sum::<i32>();
        match side {
            Side::Sente => e.sente = value,
            Side::Gote => e.gote = value,
        }
    }
    e
}
/// Mandatory finite deterministic limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    /// Maximum completed depth.
    pub depth: u16,
    /// Total admitted visits; zero publishes no choice.
    pub nodes: u64,
}
impl Limits {
    /// Native and supervised consumers share these limits.
    pub const fn valid(self) -> bool {
        self.depth >= 1 && self.depth <= MAX_DEPTH && self.nodes <= MAX_NODES
    }
}
/// A completed iteration decision, never an already applied board action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decision {
    /// Complete permitted native action.
    Choose(Action),
    /// Exact native root conclusion.
    Finished(Conclusion),
}
/// Actual work observations, always in the captured root-side perspective.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchResult {
    /// None until a complete iteration.
    pub decision: Option<Decision>,
    /// Heuristic or native win-distance score; no score without completed work.
    pub score: Option<i32>,
    /// Native analyzed side.
    pub perspective: Side,
    /// Fully completed depth.
    pub completed_depth: u16,
    /// All admitted node visits.
    pub nodes: u64,
    /// Complete native actions from completed work.
    pub principal_variation: Vec<Action>,
    /// Actual completion or interruption.
    pub status: SearchStatus,
}
/// Native admission/restoration failure is unscored, not a game conclusion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SearchError {
    /// Unsupported limits.
    InvalidLimits,
    /// Private native copy of the game failed.
    Record(RecordError),
    /// Native command refusal.
    Transition(GameError),
    /// The search and the native rules disagree on the root actions.
    Invariant,
}
impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tori search failed: {self:?}")
    }
}
impl Error for SearchError {}
/// Deterministic bounded search of the current position of a native game.
/// The game is not changed. Each call starts with an empty table, so equal
/// requests give equal results.
pub fn search(game: &Game, limits: Limits) -> Result<SearchResult, SearchError> {
    search_with_stop(game, limits, || false)
}
/// Search with a cooperative stop. The stop callback must be cheap.
pub fn search_with_stop(
    game: &Game,
    limits: Limits,
    mut stop: impl FnMut() -> bool,
) -> Result<SearchResult, SearchError> {
    if !limits.valid() {
        return Err(SearchError::InvalidLimits);
    }
    let side = game.position().side_to_move();
    let mut r = SearchResult {
        decision: None,
        score: None,
        perspective: side,
        completed_depth: 0,
        nodes: 0,
        principal_variation: vec![],
        status: SearchStatus::Complete,
    };
    if stop() {
        r.status = SearchStatus::Stopped;
        return Ok(r);
    }
    if limits.nodes == 0 {
        r.status = SearchStatus::NodeLimit;
        return Ok(r);
    }
    if let Some(conclusion) = game.conclusion() {
        r.decision = Some(Decision::Finished(conclusion));
        r.score = Some(terminal_score(conclusion, side, 0));
        r.completed_depth = 1;
        r.nodes = 1;
        return Ok(r);
    }
    let legal = game.legal_actions();
    if legal.is_empty() {
        return Err(SearchError::Invariant);
    }
    let mut position = ToriPosition::from_game(game);
    let mut searcher = Searcher::<ToriPosition>::new(TABLE_MEGABYTES);
    let outcome = searcher.search(
        &mut position,
        alphabeta::Limits {
            depth: limits.depth,
            nodes: Some(limits.nodes),
            soft: None,
            hard: None,
        },
        &mut stop,
    );
    r.nodes = outcome.nodes;
    r.completed_depth = outcome.depth;
    r.status = match outcome.status {
        Status::Complete => SearchStatus::Complete,
        Status::NodeLimit => SearchStatus::NodeLimit,
        Status::Stopped => SearchStatus::Stopped,
    };
    let Some(best) = outcome.best else {
        r.completed_depth = 0;
        return Ok(r);
    };
    // The native rules are the authority: an action that they do not list is
    // never returned.
    let action = position.action(best);
    if !legal.contains(&action) {
        return Err(SearchError::Invariant);
    }
    // Report the line as far as the native game accepts it. The search uses an
    // approximation of the repetition rule, so the native game can refuse an
    // action of the line or conclude before its end.
    let mut line = Vec::with_capacity(MAX_LINE);
    let mut made = 0;
    for &m in outcome.pv.iter().take(MAX_LINE) {
        line.push(position.action(m));
        if !position.make(m) {
            line.pop();
            break;
        }
        made += 1;
    }
    for _ in 0..made {
        position.unmake();
    }
    let mut native = GameRecord::from_game(game)
        .and_then(|record| record.to_game())
        .map_err(SearchError::Record)?;
    let mut accepted = 0;
    for &action in &line {
        if native.conclusion().is_some() {
            break;
        }
        let request = GameRequest {
            expected: native.context(),
            actor: native.position().side_to_move(),
            command: GameCommand::Play { action },
        };
        match native.submit(request) {
            Ok(_) => accepted += 1,
            Err(error) if accepted == 0 => return Err(SearchError::Transition(error)),
            Err(_) => break,
        }
    }
    line.truncate(accepted.min(MAX_LINE));
    r.decision = Some(Decision::Choose(action));
    r.score = outcome.score;
    r.principal_variation = line;
    Ok(r)
}
/// Root-side native terminal score; stalemate/repetition losses are not mate claims.
pub fn terminal_score(c: Conclusion, side: Side, distance: u16) -> i32 {
    let winner = match c {
        Conclusion::Checkmate { winner } | Conclusion::Resignation { winner } => winner,
        Conclusion::StalemateLoss { loser } | Conclusion::RepetitionForfeit { loser } => {
            loser.opponent()
        }
    };
    if winner == side {
        WIN_SCORE - i32::from(distance)
    } else {
        -WIN_SCORE + i32::from(distance)
    }
}
#[cfg(test)]
mod position_tests;
#[cfg(test)]
mod tests;
