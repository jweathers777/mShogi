//! Transparent Xiangqi evaluation and bounded native-history search, without a score cache.
use mshogi_core::xiangqi::{
    Action, Conclusion, ExactAnalysis, ExactFailure, ExactLimits, ExactObservation, ExactStop,
    ExactVerdict, Game, GameCommand, GameError, GameRecord, GameRequest, GameUndo, NoWinAssessment,
    PendingAdjudication, PieceKind, Position, RecordError, Side,
};
use std::{error::Error, fmt};

/// Untrained material and soldier advancement, independent of WXF adjudication.
pub const EVALUATOR_ID: &str = "mshogi.xiangqi.material-advance.v1";
/// Stable native-choice order, full-width iterative minimax, no transpositions.
pub const SEARCH_ID: &str = "mshogi.xiangqi.full-width.v1";
/// Native terminal win minus command-edge distance; not necessarily checkmate.
pub const WIN_SCORE: i32 = 1_000_000;
/// Conservative recursion and total-visit ceilings for this unoptimized native baseline.
pub const MAX_DEPTH: u16 = 3;
/// Includes iterative revisits and discarded partial work, not just scored leaves.
pub const MAX_NODES: u64 = 512;
/// Conservative initial functional-play setting; measurements are recorded in engine.md.
pub const DEFAULT_LIMITS: Limits = Limits {
    depth: 1,
    nodes: 128,
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
    /// Maximum full-width board depth,1..=MAX_DEPTH.
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
/// Recorded sound_fast_only policy. Fast Unknown is retained as uncertainty,
/// not a claim of winnability. No implicit exact graph at any node.
pub fn search_with_stop(
    game: &Game,
    limits: Limits,
    mut stop: impl FnMut() -> bool,
) -> Result<SearchResult, SearchError> {
    run(
        game,
        limits,
        AssessmentEvidence::Fast(game.no_win_assessment()),
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
        ExactVerdict::NaturalWin { .. } => run(game, limits, result.assessment, &mut stop),
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
    let record = GameRecord::from_game(game).map_err(SearchError::Record)?;
    let mut tree = record.to_game().map_err(SearchError::Record)?;
    let mut work = Work {
        nodes: 0,
        limit: limits.nodes,
        stop,
    };
    for depth in 1..=limits.depth {
        let value = minimax(&mut tree, depth, 0, result.perspective, &mut work);
        // Semantic history exactness, not retired execution-token equality.
        if GameRecord::from_game(&tree).map_err(SearchError::Record)? != record {
            return Err(SearchError::Restore);
        }
        match value {
            Ok(value) => {
                result.decision = value
                    .pv
                    .first()
                    .copied()
                    .map(Decision::Choose)
                    .or_else(|| game.conclusion().map(Decision::Finished));
                result.score = Some(value.score);
                result.completed_depth = depth;
                result.principal_variation = value.pv;
                if matches!(result.decision, Some(Decision::Finished(_))) {
                    break;
                }
            }
            Err(Halt::Limit(status)) => {
                result.status = status;
                break;
            }
            Err(Halt::Unresolved(pending)) => {
                result.decision = None;
                result.score = None;
                result.completed_depth = 0;
                result.principal_variation.clear();
                result.status = Status::Unresolved;
                result.unresolved = Some(Unresolved::Native(pending));
                break;
            }
            Err(Halt::Error(error)) => return Err(error),
        }
    }
    result.nodes = work.nodes;
    Ok(result)
}
struct Value {
    score: i32,
    pv: Vec<Choice>,
}
enum Halt {
    Limit(Status),
    Unresolved(PendingAdjudication),
    Error(SearchError),
}
struct Work<'a> {
    nodes: u64,
    limit: u64,
    stop: &'a mut dyn FnMut() -> bool,
}
impl Work<'_> {
    fn check(&mut self) -> Result<(), Halt> {
        if (self.stop)() {
            return Err(Halt::Limit(Status::Stopped));
        }
        if self.nodes >= self.limit {
            return Err(Halt::Limit(Status::NodeLimit));
        }
        Ok(())
    }
    fn visit(&mut self) -> Result<(), Halt> {
        self.check()?;
        self.nodes += 1;
        Ok(())
    }
}
fn terminal_score(conclusion: Conclusion, root: Side, distance: u16) -> i32 {
    match conclusion {
        Conclusion::Draw { .. } => 0,
        Conclusion::Win { winner, .. } if winner == root => WIN_SCORE - i32::from(distance),
        Conclusion::Win { .. } => -WIN_SCORE + i32::from(distance),
    }
}
fn descend(
    game: &mut Game,
    choice: Choice,
    depth: u16,
    distance: u16,
    root: Side,
    work: &mut Work<'_>,
) -> Result<Value, Halt> {
    work.check()?;
    let actor = match choice {
        Choice::Move(_) => game.position().side_to_move(),
        Choice::Claim { actor } => actor,
    };
    let undo: GameUndo = game
        .submit(GameRequest {
            expected: game.context(),
            actor,
            command: choice.command(),
        })
        .map_err(|e| Halt::Error(SearchError::Transition(e)))?
        .undo;
    let child = minimax(game, depth, distance + 1, root, work);
    game.undo(undo)
        .map_err(|_| Halt::Error(SearchError::Restore))?;
    let mut child = child?;
    child.pv.insert(0, choice);
    Ok(child)
}
fn minimax(
    game: &mut Game,
    depth: u16,
    distance: u16,
    root: Side,
    work: &mut Work<'_>,
) -> Result<Value, Halt> {
    work.visit()?;
    if let Some(conclusion) = game.conclusion() {
        return Ok(Value {
            score: terminal_score(conclusion, root, distance),
            pv: vec![],
        });
    }
    if let Some(pending) = game.pending_adjudication() {
        return Err(Halt::Unresolved(pending.clone()));
    }
    if matches!(game.no_win_assessment(), NoWinAssessment::Failure) {
        return Err(Halt::Error(SearchError::Invariant));
    }
    // A valid optional claim is a zero-valued alternative, not an automatic draw.
    // Non-turn claims remain native-admissible at the host boundary; ordinary
    // turn search chooses only for its mover, never decides for the opponent.
    let turn = game.position().side_to_move();
    let claim = game
        .natural_count_eligibility(turn)
        .eligible
        .then_some(Choice::Claim { actor: turn });
    let mut best = match claim {
        Some(choice) => Some(descend(game, choice, 0, distance, root, work)?),
        None => None,
    };
    if depth == 0 {
        let score = evaluate(game.position()).score_for(root);
        if best.as_ref().is_none_or(|b| {
            if turn == root {
                score > b.score
            } else {
                score < b.score
            }
        }) {
            best = Some(Value { score, pv: vec![] });
        }
        return Ok(best.expect("leaf value"));
    }
    for action in game
        .position()
        .legal_actions()
        .map_err(|_| Halt::Error(SearchError::Invariant))?
    {
        let child = descend(game, Choice::Move(action), depth - 1, distance, root, work)?;
        if best.as_ref().is_none_or(|b| {
            if turn == root {
                child.score > b.score
            } else {
                child.score < b.score
            }
        }) {
            best = Some(child);
        }
    }
    best.ok_or(Halt::Error(SearchError::Invariant))
}
