//! Xiangqi adapter for the shared alpha-beta core; native rules own actual play.
/// Tunable classical evaluation weights and the Texel model.
pub mod eval;
pub mod position;
#[cfg(test)]
mod position_tests;
use crate::alphabeta::{self, SearchPosition, Searcher};
use mshogi_core::xiangqi::{
    Action, Conclusion, ExactAnalysis, ExactFailure, ExactLimits, ExactObservation, ExactStop,
    ExactVerdict, Game, GameCommand, GameError, GameRequest, NoWinAssessment, PendingAdjudication,
    PieceKind, Position, RecordError, Side,
};
use position::XiangqiPosition;
use std::{error::Error, fmt};

/// Untrained material and soldier advancement, independent of WXF adjudication.
pub const EVALUATOR_ID: &str = "mshogi.xiangqi.material-placement.v1";
/// Historical evaluation identity.
pub const LEGACY_EVALUATOR_ID: &str = "mshogi.xiangqi.material-advance.v1";
/// Shared alpha-beta search identity.
pub const SEARCH_ID: &str = "mshogi.xiangqi.shared-core.v1";
/// Historical full-width identity.
pub const LEGACY_SEARCH_ID: &str = "mshogi.xiangqi.full-width.v1";
/// Historical maximum depth.
pub const LEGACY_MAX_DEPTH: u16 = 3;
/// Historical maximum visits.
pub const LEGACY_MAX_NODES: u64 = 512;
/// Historical terminal score.
pub const LEGACY_WIN_SCORE: i32 = 1_000_000;
/// Maximum reported native continuation length.
pub const MAX_LINE: usize = 8;
/// Native terminal win minus command-edge distance; not necessarily checkmate.
pub const WIN_SCORE: i32 = 1_000_000;
/// Largest nominal search depth.
pub const MAX_DEPTH: u16 = 64;
/// Includes iterative revisits and discarded partial work, not just scored leaves.
pub const MAX_NODES: u64 = 5_000_000;
/// Default bounded interactive work.
pub const DEFAULT_LIMITS: Limits = Limits {
    depth: MAX_DEPTH,
    nodes: 300_000,
};
/// Hundredths of an unadvanced soldier. A General is valued only by native outcomes.
pub const fn piece_value(kind: PieceKind) -> i32 {
    match kind {
        PieceKind::General => 0,
        PieceKind::Advisor | PieceKind::Elephant => 200,
        PieceKind::Horse => 400,
        PieceKind::Rook => 900,
        PieceKind::Cannon => 450,
        PieceKind::Soldier => 100,
    }
}
/// Explicit native ownership; neither a large score nor zero establishes an outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Evaluation {
    /// Red material plus soldier advancement.
    pub red: i32,
    /// Black material plus soldier advancement.
    pub black: i32,
}
impl Evaluation {
    /// Positive favors this named side, independent of turn.
    pub const fn score_for(self, side: Side) -> i32 {
        match side {
            Side::Red => self.red - self.black,
            Side::Black => self.black - self.red,
        }
    }
}
/// Only positional term: +20 per forward rank beyond the soldier's home rank3.
/// Black's forward rank is9-rank; no mobility/king-safety or dead-position heuristic.
pub fn evaluate(position: &Position) -> Evaluation {
    let mut value = Evaluation { red: 0, black: 0 };
    for (at, piece) in position.pieces() {
        let forward_rank = match piece.side {
            Side::Red => at.rank(),
            Side::Black => 9 - at.rank(),
        };
        let advance = if piece.kind == PieceKind::Soldier {
            20 * i32::from(forward_rank.saturating_sub(3))
        } else {
            0
        };
        let total = match piece.side {
            Side::Red => &mut value.red,
            Side::Black => &mut value.black,
        };
        *total += piece_value(piece.kind) + advance;
    }
    value
}
/// Mandatory finite deterministic work. Zero visits is explicitly supported.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    /// Maximum nominal depth,1..=MAX_DEPTH.
    pub depth: u16,
    /// Total admitted visits including iterative/partial work.
    pub nodes: u64,
}
impl Limits {
    /// Same admitted bounds in native search and shared child execution.
    pub const fn valid(self) -> bool {
        self.depth >= 1 && self.depth <= MAX_DEPTH && self.nodes <= MAX_NODES
    }
}
/// Complete board action versus optional formal count claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Choice {
    /// Native complete action, including all history-layer consequences.
    Move(Action),
    /// Only an actually eligible claim is searched; admission remains native.
    Claim {
        /// Native claimant; ordinary turn search chooses for its mover.
        actor: Side,
    },
}
impl Choice {
    /// No outcome/score is applied by this conversion.
    pub const fn command(self) -> GameCommand {
        match self {
            Self::Move(action) => GameCommand::Play { action },
            Self::Claim { .. } => GameCommand::ClaimNaturalCount,
        }
    }
}
/// Actual completed decision, never a fabricated first legal fallback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decision {
    /// Proposed complete native choice.
    Choose(Choice),
    /// Already committed native root conclusion.
    Finished(Conclusion),
    /// Read-only root exact proof. Does not install a canonical native conclusion.
    ExactNoWin,
}
/// Real execution state, distinct from a game outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    /// Requested iteration work or explicit proof completed.
    Complete,
    /// Deterministic total visit bound reached.
    NodeLimit,
    /// Cooperative external stop observed.
    Stopped,
    /// Required native adjudication has no score.
    Unresolved,
}
/// Required adjudication could not establish a score.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Unresolved {
    /// Native history/conduct classification blocks all commands.
    Native(PendingAdjudication),
    /// Explicit required root proof did not complete; never interpreted as winnable.
    Exact(ExactStop),
}
/// Retain the origin and exact original carrier, including source record and runtime target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AssessmentEvidence {
    /// Recorded sound-fast-only root evidence, including a fast miss.
    Fast(NoWinAssessment),
    /// Accepted independently bounded root proof and complete source provenance.
    Exact(Box<ExactObservation>),
}
/// Native observations; the immutable caller game is the analyzed source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchResult {
    /// Last complete decision; absent before completion or when unresolved.
    pub decision: Option<Decision>,
    /// Native root-side heuristic or actual terminal distance. None when unscored.
    pub score: Option<i32>,
    /// Native root side for every observation.
    pub perspective: Side,
    /// Last fully completed board depth; zero when no scored iteration completed.
    pub completed_depth: u16,
    /// Total admitted visits including iterative/partial work.
    pub nodes: u64,
    /// Continuation from completed work, possibly ending in a count claim.
    pub principal_variation: Vec<Choice>,
    /// Actual execution completion/stop reason.
    pub status: Status,
    /// Native required uncertainty, never an invented score.
    pub unresolved: Option<Unresolved>,
    /// Original fast/exact root evidence; not a minimax evaluation.
    pub assessment: AssessmentEvidence,
}
/// Native failures remain unscored, retaining concrete diagnostics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SearchError {
    /// Outside documented finite settings.
    InvalidLimits,
    /// Authoritative private reconstruction failed.
    Record(RecordError),
    /// Native command admission/invariant failed.
    Transition(GameError),
    /// Balanced native reversal or canonical root comparison failed.
    Restore,
    /// Native live node lacked choices or a sound assessment.
    Invariant,
    /// Explicit exact verifier failure, distinct from incomplete proof.
    Exact(ExactFailure),
}
impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "xiangqi search failed: {self:?}")
    }
}
impl Error for SearchError {}

/// Deterministic recorded-policy search with mandatory finite limits.
pub fn search(game: &Game, limits: Limits) -> Result<SearchResult, SearchError> {
    search_with_stop(game, limits, || false)
}
/// Persistent search state for one Xiangqi game.
pub type Engine = Searcher<XiangqiPosition>;
/// Default search table size in megabytes.
pub const DEFAULT_TABLE_MB: usize = 64;
/// Largest supported search table in megabytes.
pub const MAX_TABLE_MB: usize = 1024;
/// Start with an empty table and move statistics.
pub fn new_engine() -> Engine {
    Searcher::new(DEFAULT_TABLE_MB)
}
/// Start with an empty table of `megabytes` (clamped to 1..=MAX_TABLE_MB).
pub fn new_engine_sized(megabytes: usize) -> Engine {
    Searcher::new(megabytes.clamp(1, MAX_TABLE_MB))
}
/// Recorded sound_fast_only policy. Fast Unknown is retained as uncertainty,
/// not a claim of winnability. No implicit exact graph at any node.
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
    // The core enforces the hard deadline through its limits, so a timed
    // request that expires is not a cancellation and may use its fallback.
    run(
        game,
        limits,
        AssessmentEvidence::Fast(game.no_win_assessment()),
        engine,
        &timing,
        &mut stop,
    )
}
/// Explicit advanced root requirement, independently bounded and cancellable.
/// Positive existential evidence is not a forced win; descendants still obey the
/// recorded fast policy. Exact negative/Unknown is read-only and unscored here.
/// Native accept checks the complete root context, purpose/target/identity/limits.
pub fn search_requiring_root_exact(
    game: &Game,
    limits: Limits,
    exact: ExactLimits,
    mut stop: impl FnMut() -> bool,
) -> Result<SearchResult, SearchError> {
    if !limits.valid() {
        return Err(SearchError::InvalidLimits);
    }
    // Already-concluded actual games take precedence over a hypothetical proof.
    if game.conclusion().is_some() || game.pending_adjudication().is_some() {
        return search_with_stop(game, limits, stop);
    }
    let mut analysis = ExactAnalysis::new().map_err(SearchError::Transition)?;
    let request = analysis
        .begin(game, exact)
        .map_err(SearchError::Transition)?;
    let observation = request.run(&mut stop);
    let report = analysis
        .accept(game, &observation)
        .map_err(SearchError::Transition)?;
    let mut result = empty(game, AssessmentEvidence::Exact(Box::new(observation)));
    match report.verdict {
        ExactVerdict::Failure { reason } => Err(SearchError::Exact(reason)),
        ExactVerdict::Unknown { reason } => {
            result.status = if reason == ExactStop::Cancelled {
                Status::Stopped
            } else {
                Status::Unresolved
            };
            result.unresolved = Some(Unresolved::Exact(reason));
            Ok(result)
        }
        ExactVerdict::NeitherCanWin => {
            result.decision = Some(Decision::ExactNoWin);
            Ok(result)
        }
        ExactVerdict::NaturalWin { .. } => run(
            game,
            limits,
            result.assessment,
            &mut new_engine(),
            &alphabeta::timing::RequestTime::new(None),
            &mut stop,
        ),
    }
}
fn empty(game: &Game, assessment: AssessmentEvidence) -> SearchResult {
    SearchResult {
        decision: None,
        score: None,
        perspective: game.position().side_to_move(),
        completed_depth: 0,
        nodes: 0,
        principal_variation: vec![],
        status: Status::Complete,
        unresolved: None,
        assessment,
    }
}
fn run(
    game: &Game,
    limits: Limits,
    evidence: AssessmentEvidence,
    engine: &mut Engine,
    timing: &alphabeta::timing::RequestTime,
    stop: &mut dyn FnMut() -> bool,
) -> Result<SearchResult, SearchError> {
    if !limits.valid() {
        return Err(SearchError::InvalidLimits);
    }
    let mut result = empty(game, evidence);
    // Cooperative stop before bounded native reconstruction; process owner also
    // retains the independent watchdog/kill path for parsing/replay/queries.
    if stop() {
        result.status = Status::Stopped;
        return Ok(result);
    }
    if limits.nodes == 0 {
        result.status = Status::NodeLimit;
        return Ok(result);
    }
    if let Some(conclusion) = game.conclusion() {
        result.decision = Some(Decision::Finished(conclusion));
        result.score = Some(terminal_score(conclusion, result.perspective, 0));
        result.completed_depth = 1;
        result.nodes = 1;
        return Ok(result);
    }
    if let Some(pending) = game.pending_adjudication() {
        result.status = Status::Unresolved;
        result.unresolved = Some(Unresolved::Native(pending.clone()));
        result.nodes = 1;
        return Ok(result);
    }
    if matches!(game.no_win_assessment(), NoWinAssessment::Failure) {
        return Err(SearchError::Invariant);
    }
    let legal = game
        .position()
        .legal_actions()
        .map_err(|_| SearchError::Invariant)?;
    if legal.is_empty() {
        return Err(SearchError::Invariant);
    }
    let root = result.perspective;
    let claim = game.natural_count_eligibility(root).eligible;
    let mut position = XiangqiPosition::from_game(game);
    let outcome = if timing.remaining().is_some() {
        // A timed request always returns a legal move (native root order).
        let fallback = position.find(legal[0]).ok_or(SearchError::Invariant)?;
        engine.search_with_timed_fallback(
            &mut position,
            timing.limits(limits.depth, Some(limits.nodes)),
            stop,
            fallback,
        )
    } else {
        engine.search(
            &mut position,
            timing.limits(limits.depth, Some(limits.nodes)),
            stop,
        )
    };
    result.nodes = outcome.nodes;
    result.completed_depth = outcome.depth;
    result.status = match outcome.status {
        alphabeta::Status::Complete => Status::Complete,
        alphabeta::Status::NodeLimit => Status::NodeLimit,
        alphabeta::Status::Stopped => Status::Stopped,
    };
    let Some(best) = outcome.best else {
        result.completed_depth = 0;
        return Ok(result);
    };
    let mut native = game.fork().map_err(SearchError::Transition)?;
    // A valid root claim is an option, not an automatic draw. A completed
    // search must prefer it when the move score is no better than zero.
    if claim && outcome.score.is_some_and(|score| score <= 0) {
        let choice = Choice::Claim { actor: root };
        native
            .submit(GameRequest {
                expected: native.context(),
                actor: root,
                command: choice.command(),
            })
            .map_err(SearchError::Transition)?;
        if !matches!(native.conclusion(), Some(Conclusion::Draw { .. })) {
            return Err(SearchError::Invariant);
        }
        result.decision = Some(Decision::Choose(choice));
        result.principal_variation.push(choice);
        result.score = Some(0);
        return Ok(result);
    }
    let action = position.action(best);
    if !legal.contains(&action) {
        return Err(SearchError::Invariant);
    }
    for &m in outcome.pv.iter().take(MAX_LINE) {
        if !result.principal_variation.is_empty() && timing.expired() {
            break;
        }
        if native.conclusion().is_some() || native.pending_adjudication().is_some() {
            break;
        }
        let action = position.action(m);
        let request = GameRequest {
            expected: native.context(),
            actor: native.position().side_to_move(),
            command: GameCommand::Play { action },
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
/// Native terminal score from the captured root side.
pub fn terminal_score(conclusion: Conclusion, root: Side, distance: u16) -> i32 {
    match conclusion {
        Conclusion::Draw { .. } => 0,
        Conclusion::Win { winner, .. } if winner == root => WIN_SCORE - i32::from(distance),
        Conclusion::Win { .. } => -WIN_SCORE + i32::from(distance),
    }
}
