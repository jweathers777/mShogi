//! Chu adapter for the shared search core. Native Game owns actual play.
pub mod position;
#[cfg(test)]
mod position_tests;
use crate::alphabeta::{self, Searcher};
use crate::search::SearchStatus;
use mshogi_core::chu::{
    Action, Conclusion, Face, Game, GameCommand, GameError, GameRequest, Position, RecordError,
    Side,
};
use position::ChuPosition;
use std::{error::Error, fmt};
/// Classical material and placement evaluation.
pub const EVALUATOR_ID: &str = "mshogi.chu.material-placement.v1";
/// Shared alpha-beta search identity.
pub const SEARCH_ID: &str = "mshogi.chu.shared-core.v1";
/// Historical material identity for saved observations.
pub const LEGACY_EVALUATOR_ID: &str = "mshogi.chu.material.v1";
/// Historical search identity for saved observations.
pub const LEGACY_SEARCH_ID: &str = "mshogi.chu.full-width.v1";
/// Historical maximum depth.
pub const LEGACY_MAX_DEPTH: u16 = 2;
/// Historical maximum work.
pub const LEGACY_MAX_NODES: u64 = 2_000;
/// Historical terminal score.
pub const LEGACY_WIN_SCORE: i32 = 1_000_000;
/// Native terminal win minus board-action distance; not specifically mate.
pub const WIN_SCORE: i32 = alphabeta::MATE_SCORE;
/// Maximum nominal depth.
pub const MAX_DEPTH: u16 = 64;
/// Total visits, including partial iterations and quiescence.
pub const MAX_NODES: u64 = 5_000_000;
/// Maximum reported native continuation length.
pub const MAX_LINE: usize = 8;
/// Default bounded interactive work.
pub const DEFAULT_LIMITS: Limits = Limits {
    depth: MAX_DEPTH,
    nodes: 300_000,
};
/// Untrained hundredths-of-a-Pawn material values. Royals receive a finite
/// survival value; only native terminal adjudication establishes a result.
pub const fn piece_value(face: Face) -> i32 {
    match face {
        Face::Pawn => 100,
        Face::GoBetween => 150,
        Face::Lance | Face::ReverseChariot => 300,
        Face::Copper | Face::Silver | Face::Leopard | Face::Tiger => 400,
        Face::Gold | Face::Elephant => 500,
        Face::Kirin | Face::Phoenix => 600,
        Face::SideMover | Face::VerticalMover | Face::Bishop => 700,
        Face::Rook | Face::WhiteHorse | Face::Whale | Face::FlyingStag => 900,
        Face::DragonHorse | Face::DragonKing | Face::FlyingOx | Face::FreeBoar => 1100,
        Face::SoaringEagle | Face::HornedFalcon => 1300,
        Face::FreeKing => 1500,
        Face::Lion => 2000,
        Face::King | Face::Prince => 5000,
    }
}
/// Transparent board totals.
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
/// Native pieces only; material never establishes an outcome.
pub fn evaluate(position: &Position) -> Evaluation {
    let mut e = Evaluation { sente: 0, gote: 0 };
    for side in [Side::Sente, Side::Gote] {
        let value = position
            .pieces()
            .filter(|(_, p)| p.side == side)
            .map(|(_, p)| piece_value(p.face()))
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
    /// Completed nominal depth; selective lines can be longer.
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
    /// Private owner reconstruction failed.
    Record(RecordError),
    /// Native command refusal.
    Transition(GameError),
    /// Private search output or restoration invariant failed.
    Restore,
    /// No admitted board continuation, without a sourced conclusion: unscored.
    NoBoardActions,
}
impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chu search failed: {self:?}")
    }
}
impl Error for SearchError {}
/// Deterministic bounded search of a separately owned admitted native record.
pub fn search(game: &Game, limits: Limits) -> Result<SearchResult, SearchError> {
    search_with_stop(game, limits, || false)
}
/// Persistent search state for one Chu game. Native history stays outside it.
pub type Engine = Searcher<ChuPosition>;

/// Start with an empty transposition table and move statistics.
pub fn new_engine() -> Engine {
    Searcher::new(16)
}

/// Cooperative stop through the shared core. Native root queries and replay
/// are bounded native operations; hosts may terminate the child.
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
pub fn search_with_state(
    game: &Game,
    limits: Limits,
    time: Option<alphabeta::timing::TimeBudget>,
    engine: &mut Engine,
    mut stop: impl FnMut() -> bool,
) -> Result<SearchResult, SearchError> {
    let timing = alphabeta::timing::RequestTime::new(time);
    let mut stop = || stop() || timing.expired();
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
        return Err(SearchError::NoBoardActions);
    }
    let mut position = ChuPosition::from_game(game);
    position.restrict_root(&legal);
    let outcome = engine.search(
        &mut position,
        timing.limits(limits.depth, Some(limits.nodes)),
        &mut stop,
    );
    r.nodes = outcome.nodes;
    r.completed_depth = outcome.depth;
    r.status = match outcome.status {
        alphabeta::Status::Complete => SearchStatus::Complete,
        alphabeta::Status::NodeLimit => SearchStatus::NodeLimit,
        alphabeta::Status::Stopped => SearchStatus::Stopped,
    };
    let Some(best) = outcome.best else {
        r.completed_depth = 0;
        return Ok(r);
    };
    let action = position.to_native(best);
    if !legal.contains(&action) {
        return Err(SearchError::Restore);
    }
    let mut native = game.fork().map_err(SearchError::Transition)?;
    let mut first_conclusion = None;
    for &m in outcome.pv.iter().take(MAX_LINE) {
        if !r.principal_variation.is_empty() && timing.expired() {
            break;
        }
        if native.conclusion().is_some() {
            break;
        }
        let action = position.to_native(m);
        let request = GameRequest {
            expected: native.context(),
            actor: native.position().side_to_move(),
            command: GameCommand::Play {
                action: action.clone(),
            },
        };
        match native.submit(request) {
            Ok(_) => {}
            Err(error) if r.principal_variation.is_empty() => {
                return Err(SearchError::Transition(error));
            }
            Err(_) => break,
        }
        if r.principal_variation.is_empty() {
            first_conclusion = native.conclusion();
        }
        r.principal_variation.push(action);
    }
    if r.principal_variation.first() != Some(&action) {
        return Err(SearchError::Restore);
    }
    r.decision = Some(Decision::Choose(action));
    // A real one-action conclusion is stronger evidence than the private
    // approximation (in particular native immediate mate and exhaustion).
    r.score = first_conclusion
        .map(|c| terminal_score(c, side, 1))
        .or(outcome.score);
    Ok(r)
}
/// Root-side native terminal score; stalemate/repetition losses are not mate claims.
pub fn terminal_score(c: Conclusion, side: Side, distance: u16) -> i32 {
    let Some(winner) = c.winner() else {
        return 0;
    };
    if winner == side {
        WIN_SCORE - i32::from(distance)
    } else {
        -WIN_SCORE + i32::from(distance)
    }
}
#[cfg(test)]
mod tests;
