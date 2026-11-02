//! Concrete history-aware Tori baseline. Native Game owns every rule and conclusion.
use crate::search::SearchStatus;
use mshogi_core::tori::{
    Action, Conclusion, Face, Game, GameCommand, GameError, GameRecord, GameRequest, Position,
    RecordError, Side, Square,
};
use std::{error::Error, fmt};
/// Transparent untrained material configuration, not playing-strength evidence.
pub const EVALUATOR_ID: &str = "mshogi.tori.material.v1";
/// Deterministic full-width search, with no board-only transposition cache.
pub const SEARCH_ID: &str = "mshogi.tori.full-width.v1";
/// Native terminal win minus board-action distance; not specifically a mate score.
pub const WIN_SCORE: i32 = 1_000_000;
/// Bounded recursive baseline.
pub const MAX_DEPTH: u16 = 3;
/// Total visits including iterative revisits and discarded work.
pub const MAX_NODES: u64 = 10_000;
/// Conservative interactive budget; only completed iterations can supply a choice.
pub const DEFAULT_LIMITS: Limits = Limits {
    depth: 2,
    nodes: 2_000,
};
/// Hundredths of a Swallow; handed quails remain distinct native identities.
pub const fn piece_value(face: Face) -> i32 {
    match face {
        Face::Phoenix => 0,
        Face::Swallow => 100,
        Face::Goose => 300,
        Face::Falcon => 700,
        Face::Eagle => 1000,
        Face::Crane => 400,
        Face::Pheasant => 300,
        Face::LeftQuail | Face::RightQuail => 500,
    }
}
/// Transparent board plus hand totals.
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
/// Native pieces/hands only; material never establishes an outcome.
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
    /// Private owner reconstruction failed.
    Record(RecordError),
    /// Native command refusal.
    Transition(GameError),
    /// Exact semantic reversal failed.
    Restore,
    /// Live native node unexpectedly had no action.
    Invariant,
}
impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tori search failed: {self:?}")
    }
}
impl Error for SearchError {}
/// Deterministic bounded search of a separately owned admitted native record.
pub fn search(game: &Game, limits: Limits) -> Result<SearchResult, SearchError> {
    search_with_stop(game, limits, || false)
}
/// Cooperative stop at node/transition boundaries. Individual native queries and
/// initial replay are bounded native operations; hosts may terminate the child.
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
    let mut tree = GameRecord::from_game(game)
        .and_then(|r| r.to_game())
        .map_err(SearchError::Record)?;
    let root = tree.context();
    let root_snapshot = tree.snapshot().clone();
    let mut work = Work {
        nodes: 0,
        limit: limits.nodes,
        stop: &mut stop,
    };
    for depth in 1..=limits.depth {
        let value = minimax(&mut tree, depth, 0, side, &mut work);
        if !tree.context().same_semantics(&root) || tree.snapshot() != &root_snapshot {
            return Err(SearchError::Restore);
        }
        match value {
            Ok(value) => {
                r.decision = value
                    .pv
                    .first()
                    .copied()
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
enum Halt {
    Limit(SearchStatus),
    Error(SearchError),
}
struct Value {
    score: i32,
    pv: Vec<Action>,
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
// Private test seam for independently solved finite minimax trees. Native Game
// remains the only product implementation; this is not a public rules framework.
trait Tree {
    type Undo;
    fn turn(&self) -> Side;
    fn conclusion(&self) -> Option<Conclusion>;
    fn evaluate(&self, root: Side) -> i32;
    fn actions(&self) -> Vec<Action>;
    fn push(&mut self, action: Action) -> Result<Self::Undo, SearchError>;
    fn pop(&mut self, undo: Self::Undo) -> Result<(), SearchError>;
}
impl Tree for Game {
    type Undo = mshogi_core::tori::GameUndo;
    fn turn(&self) -> Side {
        self.position().side_to_move()
    }
    fn conclusion(&self) -> Option<Conclusion> {
        Game::conclusion(self)
    }
    fn evaluate(&self, root: Side) -> i32 {
        evaluate(self.position()).score_for(root)
    }
    fn actions(&self) -> Vec<Action> {
        self.legal_actions()
    }
    fn push(&mut self, action: Action) -> Result<Self::Undo, SearchError> {
        self.submit(GameRequest {
            expected: self.context(),
            actor: self.turn(),
            command: GameCommand::Play { action },
        })
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
    if depth == 0 {
        return Ok(Value {
            score: game.evaluate(root),
            pv: vec![],
        });
    }
    let maximize = game.turn() == root;
    let mut best: Option<Value> = None;
    for action in game.actions() {
        work.check()?;
        let undo = game.push(action).map_err(Halt::Error)?;
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
    best.ok_or(Halt::Error(SearchError::Invariant))
}
#[cfg(test)]
mod tests;
