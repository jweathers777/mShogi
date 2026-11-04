//! Concrete history-aware Tenjiku baseline. Native Game owns every rule and conclusion.
use crate::search::SearchStatus;
use mshogi_core::tenjiku::{
    Action, Conclusion, Face, Game, GameCommand, GameError, GameRecord, GameRequest, Position,
    RecordError, Side,
};
use std::{error::Error, fmt};
/// Transparent untrained material configuration, not playing-strength evidence.
pub const EVALUATOR_ID: &str = "mshogi.tenjiku.material.v1";
/// Deterministic full-width search, with no board-only transposition cache.
pub const SEARCH_ID: &str = "mshogi.tenjiku.full-width.v1";
/// Native terminal win minus board-action distance; not specifically a mate score.
pub const WIN_SCORE: i32 = 1_000_000;
/// Bounded recursive baseline; deeper work remains an explicit request.
pub const MAX_DEPTH: u16 = 2;
/// Total visits including iterative revisits and discarded work.
/// Release standard depth2/2000 measured about7.5s locally; no latency promise.
pub const MAX_NODES: u64 = 2_000;
/// Measured interactive budget: both standard roots,616-action Demon tactics,
///716-action area/Hawk roots and representative128-ply histories complete depth1.
/// Only completed iterations can supply a choice.
pub const DEFAULT_LIMITS: Limits = Limits {
    depth: 1,
    nodes: 1_024,
};
/// Untrained hundredths-of-a-Pawn material values. Royals receive a finite
/// survival value; only native terminal adjudication establishes a result.
pub const fn piece_value(face: Face) -> i32 {
    match face {
        Face::P => 100,
        Face::Dg | Face::N => 200,
        Face::L | Face::RC | Face::I | Face::C => 300,
        Face::S | Face::FL | Face::BT => 400,
        Face::G | Face::DE => 500,
        Face::Ky | Face::Ph => 600,
        Face::SM | Face::VM | Face::B | Face::MGn => 700,
        Face::R | Face::SSo | Face::VSo | Face::WH | Face::W | Face::FS => 900,
        Face::DK | Face::DH | Face::FO | Face::FB => 1100,
        Face::SE | Face::HF | Face::ChS | Face::HT => 1300,
        Face::FK => 1500,
        Face::FEg | Face::BGn | Face::RGn => 1800,
        Face::Ln | Face::WBf | Face::VGn => 2000,
        Face::LHk | Face::GGn => 2300,
        Face::FiD => 2500,
        Face::K | Face::CP => 5000,
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
    /// Private owner reconstruction failed.
    Record(RecordError),
    /// Native command refusal.
    Transition(GameError),
    /// Exact semantic reversal failed.
    Restore,
    /// No admitted board continuation, without a sourced conclusion: unscored.
    NoBoardActions,
}
impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tenjiku search failed: {self:?}")
    }
}
impl Error for SearchError {}
/// Deterministic bounded search of a separately owned admitted native record.
pub fn search(game: &Game, limits: Limits) -> Result<SearchResult, SearchError> {
    search_with_stop(game, limits, || false)
}
/// Cooperative stop during native replay, generation and transition admission.
/// Exact undo is finite and finishes before stop is propagated; a supervisor may
/// independently terminate the private search process.
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
    let mut tree = match GameRecord::from_game(game).and_then(|r| r.to_game_with_stop(&mut stop)) {
        Ok(tree) => tree,
        Err(RecordError::Stopped) => {
            r.status = SearchStatus::Stopped;
            return Ok(r);
        }
        Err(e) => return Err(SearchError::Record(e)),
    };
    let root = tree.search_identity();
    let root_snapshot = tree.snapshot().clone();
    let mut work = Work {
        nodes: 0,
        limit: limits.nodes,
        stop: &mut stop,
    };
    for depth in 1..=limits.depth {
        let value = minimax(&mut tree, depth, 0, side, &mut work);
        if !tree.search_identity().same_semantics(&root) || tree.snapshot() != &root_snapshot {
            return Err(SearchError::Restore);
        }
        match value {
            Ok(value) => {
                r.decision = value
                    .pv
                    .first()
                    .cloned()
                    .map(Decision::Choose)
                    .or_else(|| game.conclusion().map(Decision::Finished));
                r.score = Some(value.score);
                r.completed_depth = depth;
                r.principal_variation = value.pv;
                if game.conclusion().is_some() {
                    break;
                }
            }
            Err(Halt::Limit(status)) => {
                r.status = status;
                break;
            }
            Err(Halt::Error(e)) => return Err(e),
        }
    }
    r.nodes = work.nodes;
    Ok(r)
}
struct Work<'a> {
    nodes: u64,
    limit: u64,
    stop: &'a mut dyn FnMut() -> bool,
}
impl Work<'_> {
    fn check(&mut self) -> Result<(), Halt> {
        if (self.stop)() {
            Err(Halt::Limit(SearchStatus::Stopped))
        } else if self.nodes >= self.limit {
            Err(Halt::Limit(SearchStatus::NodeLimit))
        } else {
            Ok(())
        }
    }
    fn visit(&mut self) -> Result<(), Halt> {
        self.check()?;
        self.nodes += 1;
        Ok(())
    }
}
fn native_halt(error: SearchError) -> Halt {
    match error {
        SearchError::Transition(GameError::Stopped) | SearchError::Record(RecordError::Stopped) => {
            Halt::Limit(SearchStatus::Stopped)
        }
        error => Halt::Error(error),
    }
}
enum Halt {
    Limit(SearchStatus),
    Error(SearchError),
}
struct Value {
    score: i32,
    pv: Vec<Action>,
}
/// Root-side native terminal score; native wins are not checkmate claims.
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
// Private test seam for independently solved finite minimax trees. Native Game
// remains the only product implementation; this is not a public rules framework.
trait Tree {
    type Undo;
    fn turn(&self) -> Side;
    fn conclusion(&self) -> Option<Conclusion>;
    fn evaluate(&self, root: Side) -> i32;
    fn actions(&self, stop: &mut dyn FnMut() -> bool) -> Result<Vec<Action>, SearchError>;
    fn push(
        &mut self,
        action: Action,
        stop: &mut dyn FnMut() -> bool,
    ) -> Result<Self::Undo, SearchError>;
    fn pop(&mut self, undo: Self::Undo) -> Result<(), SearchError>;
}
impl Tree for Game {
    type Undo = mshogi_core::tenjiku::GameUndo;
    fn turn(&self) -> Side {
        self.position().side_to_move()
    }
    fn conclusion(&self) -> Option<Conclusion> {
        Game::conclusion(self)
    }
    fn evaluate(&self, root: Side) -> i32 {
        evaluate(self.position()).score_for(root)
    }
    fn actions(&self, stop: &mut dyn FnMut() -> bool) -> Result<Vec<Action>, SearchError> {
        self.legal_actions_with_stop(&mut || stop())
            .map_err(SearchError::Transition)
    }
    fn push(
        &mut self,
        action: Action,
        stop: &mut dyn FnMut() -> bool,
    ) -> Result<Self::Undo, SearchError> {
        self.submit_with_stop(
            GameRequest {
                expected: self.context(),
                actor: self.turn(),
                command: GameCommand::Play { action },
            },
            &mut || stop(),
        )
        .map_err(SearchError::Transition)
    }
    fn pop(&mut self, mut undo: Self::Undo) -> Result<(), SearchError> {
        self.undo(&mut undo)
            .map(|_| ())
            .map_err(|_| SearchError::Restore)
    }
}
fn minimax<T: Tree>(
    game: &mut T,
    depth: u16,
    distance: u16,
    root: Side,
    work: &mut Work<'_>,
) -> Result<Value, Halt> {
    work.visit()?;
    if let Some(c) = game.conclusion() {
        return Ok(Value {
            score: terminal_score(c, root, distance),
            pv: vec![],
        });
    }
    // NoBoardActions is explicitly unscored even at a nominal static cutoff.
    // Native history filtering, rather than local geometry, establishes this.
    let actions = game.actions(work.stop).map_err(native_halt)?;
    if actions.is_empty() {
        return Err(Halt::Error(SearchError::NoBoardActions));
    }
    if depth == 0 {
        return Ok(Value {
            score: game.evaluate(root),
            pv: vec![],
        });
    }
    let maximize = game.turn() == root;
    let mut best: Option<Value> = None;
    for action in actions {
        work.check()?;
        let undo = game.push(action.clone(), work.stop).map_err(native_halt)?;
        let child = minimax(game, depth - 1, distance + 1, root, work);
        game.pop(undo).map_err(Halt::Error)?;
        let mut child = child?;
        child.pv.insert(0, action);
        if best.as_ref().is_none_or(|b| {
            if maximize {
                child.score > b.score
            } else {
                child.score < b.score
            }
        }) {
            best = Some(child)
        }
    }
    best.ok_or(Halt::Error(SearchError::NoBoardActions))
}
#[cfg(test)]
mod tests;
