//! Transparent shogi material and bounded full-width search; no board-only cache.
use crate::search::SearchStatus;
use mshogi_core::shogi::{
    Action, Conclusion, Game, GameCommand, GameError, GameRequest, GameUndo, PieceKind, Position,
    Side,
};
use std::{error::Error, fmt};

/// Untrained material identity, distinct from declaration qualification points.
pub const EVALUATOR_ID: &str = "mshogi.shogi.material.v1";
/// Deterministic full-width, native-history minimax identity.
pub const SEARCH_ID: &str = "mshogi.shogi.full-width.v1";
/// Native terminal win, minus command-edge distance (not necessarily checkmate).
pub const WIN_SCORE: i32 = 1_000_000;
/// Conservative recursive depth for this unoptimized bounded baseline.
pub const MAX_DEPTH: u16 = 4;
/// Maximum total admitted visits, including iterative revisits and discarded work.
pub const MAX_NODES: u64 = 20_000;
/// Measured interactive baseline; drop-heavy requests may finish only depth one.
pub const DEFAULT_LIMITS: Limits = Limits {
    depth: 2,
    nodes: 5_000,
};

/// Hundredths of an unpromoted pawn; kings are scored by rules-owned outcomes.
pub const fn piece_value(kind: PieceKind, promoted: bool) -> i32 {
    match kind {
        PieceKind::King => 0,
        PieceKind::Gold => 500,
        PieceKind::Pawn | PieceKind::Lance | PieceKind::Knight | PieceKind::Silver if promoted => {
            500
        }
        PieceKind::Pawn => 100,
        PieceKind::Lance | PieceKind::Knight => 300,
        PieceKind::Silver => 400,
        PieceKind::Bishop => {
            if promoted {
                900
            } else {
                700
            }
        }
        PieceKind::Rook => {
            if promoted {
                1000
            } else {
                800
            }
        }
    }
}
/// Material only; neither a zero score nor a large score establishes an outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Evaluation {
    /// Board plus unpromoted hand resources owned by Sente.
    pub sente: i32,
    /// Board plus unpromoted hand resources owned by Gote.
    pub gote: i32,
}
impl Evaluation {
    /// Positive favors the named native side, independent of current turn.
    pub const fn score_for(self, side: Side) -> i32 {
        match side {
            Side::Sente => self.sente - self.gote,
            Side::Gote => self.gote - self.sente,
        }
    }
}
/// Inspect native pieces and both hands; no legality/history/outcome approximation.
pub fn evaluate(position: &Position) -> Evaluation {
    let mut result = Evaluation { sente: 0, gote: 0 };
    for piece in position.board().iter().flatten() {
        let total = match piece.side {
            Side::Sente => &mut result.sente,
            Side::Gote => &mut result.gote,
        };
        *total += piece_value(piece.kind, piece.promoted);
    }
    for side in [Side::Sente, Side::Gote] {
        let hand = position.hand(side);
        let total = match side {
            Side::Sente => &mut result.sente,
            Side::Gote => &mut result.gote,
        };
        for kind in PieceKind::HAND_KINDS {
            *total += i32::from(hand.count(kind)) * piece_value(kind, false);
        }
    }
    result
}
/// Mandatory deterministic limits. Zero nodes visits nothing and publishes no choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    /// Maximum completed full-width board depth, 1..=MAX_DEPTH.
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
/// Last completed decision; an incomplete first iteration has none.
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
    /// None before a complete iteration, never a fabricated fallback.
    pub decision: Option<Decision>,
    /// Root-side score; terminal distance counts board/declaration command edges.
    pub score: Option<i32>,
    /// Native side used for every minimax/static/terminal observation.
    pub perspective: Side,
    /// Completed board-depth iteration (declarations are examined even at leaves).
    pub completed_depth: u16,
    /// Admitted node visits, including terminal/declaration nodes and partial work.
    pub nodes: u64,
    /// Completed continuation, possibly ending in a declaration.
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
/// Use private native history. Stop is polled at node boundaries and before
/// transitions; each individual native rules query remains bounded. A partial
/// iteration contributes work only, preserving the last complete iteration.
pub fn search_with_stop(
    game: &Game,
    limits: Limits,
    mut stop: impl FnMut() -> bool,
) -> Result<SearchResult, SearchError> {
    if !limits.valid() {
        return Err(SearchError::InvalidLimits);
    }
    let root_side = game.position().side_to_move();
    let mut tree = NativeTree(game.clone());
    let mut work = Work {
        nodes: 0,
        limit: limits.nodes,
        stop: &mut stop,
    };
    let mut result = SearchResult {
        decision: None,
        score: None,
        perspective: root_side,
        completed_depth: 0,
        nodes: 0,
        principal_variation: vec![],
        status: SearchStatus::Complete,
    };
    for depth in 1..=limits.depth {
        let value = minimax(&mut tree, depth, 0, root_side, &mut work);
        if tree.0 != *game {
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
                if matches!(
                    result.decision,
                    Some(Decision::Finished(_) | Decision::Choose(Choice::Declare { .. }))
                ) {
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
// Small private seam for independently specified trees, same-turn terminal
// choices and injected descendant errors; not a public universal rules API.
trait Tree {
    type Undo;
    fn turn(&self) -> Side;
    fn conclusion(&self) -> Option<Conclusion>;
    fn declaration(&self) -> Option<Choice>;
    fn evaluate(&self, side: Side) -> i32;
    fn actions(&self) -> Vec<Choice>;
    fn push(&mut self, choice: Choice) -> Result<Self::Undo, SearchError>;
    fn pop(&mut self, undo: Self::Undo) -> Result<(), SearchError>;
}
struct NativeTree(Game);
impl Tree for NativeTree {
    type Undo = GameUndo;
    fn turn(&self) -> Side {
        self.0.position().side_to_move()
    }
    fn conclusion(&self) -> Option<Conclusion> {
        self.0.conclusion()
    }
    fn declaration(&self) -> Option<Choice> {
        self.0
            .declaration_eligibility(self.turn())
            .eligible()
            .then_some(Choice::Declare { actor: self.turn() })
    }
    fn evaluate(&self, side: Side) -> i32 {
        evaluate(self.0.position()).score_for(side)
    }
    fn actions(&self) -> Vec<Choice> {
        self.0
            .position()
            .legal_actions()
            .into_iter()
            .map(Choice::Move)
            .collect()
    }
    fn push(&mut self, choice: Choice) -> Result<Self::Undo, SearchError> {
        let actor = match choice {
            Choice::Move(_) => self.turn(),
            Choice::Declare { actor } => actor,
        };
        self.0
            .submit(GameRequest {
                expected: self.0.context(),
                actor,
                command: choice.command(),
            })
            .map_err(SearchError::Transition)
    }
    fn pop(&mut self, undo: Self::Undo) -> Result<(), SearchError> {
        self.0.undo(undo).map_err(|_| SearchError::Restore)
    }
}
struct Value {
    score: i32,
    pv: Vec<Choice>,
}
enum Halt {
    Limit(SearchStatus),
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
            return Err(Halt::Limit(SearchStatus::Stopped));
        }
        if self.nodes >= self.limit {
            return Err(Halt::Limit(SearchStatus::NodeLimit));
        }
        Ok(())
    }
    fn visit(&mut self) -> Result<(), Halt> {
        self.check()?;
        self.nodes += 1;
        Ok(())
    }
}
fn terminal_score(conclusion: Conclusion, side: Side, distance: u16) -> i32 {
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
fn descend<T: Tree>(
    tree: &mut T,
    choice: Choice,
    depth: u16,
    distance: u16,
    root: Side,
    work: &mut Work<'_>,
) -> Result<Value, Halt> {
    work.check()?;
    let undo = tree.push(choice).map_err(Halt::Error)?;
    let child = minimax(tree, depth, distance + 1, root, work);
    tree.pop(undo).map_err(Halt::Error)?;
    let mut child = child?;
    child.pv.insert(0, choice);
    Ok(child)
}
fn minimax<T: Tree>(
    tree: &mut T,
    depth: u16,
    distance: u16,
    root: Side,
    work: &mut Work<'_>,
) -> Result<Value, Halt> {
    work.visit()?;
    if let Some(conclusion) = tree.conclusion() {
        return Ok(Value {
            score: terminal_score(conclusion, root, distance),
            pv: vec![],
        });
    }
    // Native submission establishes the terminal result; advisory eligibility
    // alone is never emitted as an already concluded game. Same-turn declaration
    // backup uses stable root perspective, with no alternating-turn negation.
    if let Some(declaration) = tree.declaration() {
        return descend(tree, declaration, 0, distance, root, work);
    }
    if depth == 0 {
        return Ok(Value {
            score: tree.evaluate(root),
            pv: vec![],
        });
    }
    let maximize = tree.turn() == root;
    let mut best: Option<Value> = None;
    for choice in tree.actions() {
        let child = descend(tree, choice, depth - 1, distance, root, work)?;
        if best.as_ref().is_none_or(|best| {
            if maximize {
                child.score > best.score
            } else {
                child.score < best.score
            }
        }) {
            best = Some(child);
        }
    }
    best.ok_or(Halt::Error(SearchError::Invariant))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mshogi_core::shogi::{MutualCheckDiagnostic, Square};
    // Explicit finite graph, not a synthetic game profile. Root A has min(7,3),
    // B has min(5,9); choose B/5 at depth2. At depth1 choose A/8.
    struct Small {
        at: usize,
        fail: bool,
    }
    fn action(n: usize) -> Choice {
        Choice::Move(Action::Move {
            from: Square::new(n as u8, 1).unwrap(),
            to: Square::new(n as u8, 2).unwrap(),
            promote: false,
        })
    }
    impl Tree for Small {
        type Undo = usize;
        fn turn(&self) -> Side {
            if self.at == 1 || self.at == 2 {
                Side::Gote
            } else {
                Side::Sente
            }
        }
        fn conclusion(&self) -> Option<Conclusion> {
            None
        }
        fn declaration(&self) -> Option<Choice> {
            None
        }
        fn evaluate(&self, side: Side) -> i32 {
            let value = [0, 8, 2, 7, 3, 5, 9][self.at];
            if side == Side::Sente { value } else { -value }
        }
        fn actions(&self) -> Vec<Choice> {
            match self.at {
                0 => vec![action(1), action(2)],
                1 => vec![action(3), action(4)],
                2 => vec![action(5), action(6)],
                _ => vec![],
            }
        }
        fn push(&mut self, choice: Choice) -> Result<usize, SearchError> {
            let Choice::Move(Action::Move { from, .. }) = choice else {
                unreachable!()
            };
            let next = usize::from(from.file());
            if self.fail && next == 5 {
                // Deliberately injected diagnostic, NOT a legal mutual-check
                // position. Native tests separately exercise private corruption.
                let game = Game::new();
                return Err(SearchError::Transition(GameError::MutualCheckInvariant(
                    Box::new(MutualCheckDiagnostic {
                        candidate_context: game.context(),
                        candidate_sfen: game.position().to_sfen(),
                        occurrences: vec![0, 4, 8, 12],
                        interval_checks: vec![(Side::Sente, true), (Side::Gote, true)],
                    }),
                )));
            }
            let before = self.at;
            self.at = next;
            Ok(before)
        }
        fn pop(&mut self, undo: usize) -> Result<(), SearchError> {
            self.at = undo;
            Ok(())
        }
    }
    #[test]
    fn independently_solvable_tree_checks_stable_root_minimax_and_full_width() {
        let mut tree = Small { at: 0, fail: false };
        let mut stop = || false;
        let mut work = Work {
            nodes: 0,
            limit: MAX_NODES,
            stop: &mut stop,
        };
        let one = minimax(&mut tree, 1, 0, Side::Sente, &mut work).unwrap_or_else(|_| panic!());
        assert_eq!((one.score, one.pv), (8, vec![action(1)]));
        assert_eq!(work.nodes, 3);
        assert_eq!(tree.at, 0);
        work.nodes = 0;
        let two = minimax(&mut tree, 2, 0, Side::Sente, &mut work).unwrap_or_else(|_| panic!());
        assert_eq!((two.score, two.pv), (5, vec![action(2), action(5)]));
        assert_eq!(work.nodes, 7);
        assert_eq!(tree.at, 0);
    }
    #[test]
    fn limited_and_failed_descendants_unwind_and_retain_native_diagnostic() {
        let mut tree = Small { at: 0, fail: false };
        let mut stop = || false;
        let mut work = Work {
            nodes: 0,
            limit: 4,
            stop: &mut stop,
        };
        assert!(matches!(
            minimax(&mut tree, 2, 0, Side::Sente, &mut work),
            Err(Halt::Limit(SearchStatus::NodeLimit))
        ));
        assert_eq!(tree.at, 0);
        tree.fail = true;
        work.nodes = 0;
        work.limit = MAX_NODES;
        let Err(Halt::Error(SearchError::Transition(GameError::MutualCheckInvariant(diagnostic)))) =
            minimax(&mut tree, 2, 0, Side::Sente, &mut work)
        else {
            panic!("expected unscored injected diagnostic")
        };
        assert_eq!(tree.at, 0);
        assert_eq!(diagnostic.occurrences, vec![0, 4, 8, 12]);
        assert_eq!(diagnostic.candidate_context, Game::new().context());
    }
}
