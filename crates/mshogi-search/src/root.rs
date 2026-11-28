//! Root search for games with rules-owned histories and a private alpha-beta position.
use crate::alphabeta::{
    self, SearchPosition, Searcher,
    timing::{RequestTime, TimeBudget},
};
use std::{error::Error, fmt};

/// Maximum nominal depth for these adapters.
pub const MAX_DEPTH: u16 = 64;
/// Maximum total visits, including partial iterations.
pub const MAX_NODES: u64 = 5_000_000;
/// Maximum number of game choices in a reported continuation.
pub const MAX_LINE: usize = 8;
/// Root terminal win, less distance in actions or commands.
pub const WIN_SCORE: i32 = alphabeta::MATE_SCORE;

/// Mandatory deterministic work limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    /// Largest nominal iteration.
    pub depth: u16,
    /// Total admitted visits.
    pub nodes: u64,
}
impl Limits {
    /// Check the supported range before starting a search.
    pub const fn valid(self) -> bool {
        self.depth >= 1 && self.depth <= MAX_DEPTH && self.nodes <= MAX_NODES
    }
}

/// Game choice or an existing rules-owned conclusion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Decision<C, F> {
    /// A move or formal command.
    Choose(C),
    /// The root was already concluded.
    Finished(F),
}

/// Reason the root search ended.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    /// The requested depth or an authoritative root result was reached.
    Complete,
    /// The node budget was exhausted.
    NodeLimit,
    /// The request stopped before the next iteration completed.
    Stopped,
    /// The rules require adjudication before play can continue.
    Unresolved,
}

impl From<alphabeta::Status> for Status {
    fn from(status: alphabeta::Status) -> Self {
        match status {
            alphabeta::Status::Complete => Self::Complete,
            alphabeta::Status::NodeLimit => Self::NodeLimit,
            alphabeta::Status::Stopped => Self::Stopped,
        }
    }
}

/// Observations from the captured root side, including incomplete requests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchResult<C, F, S, P, A> {
    /// Last completed decision, or a legal choice from an interrupted first iteration.
    pub decision: Option<Decision<C, F>>,
    /// Heuristic or terminal distance.
    pub score: Option<i32>,
    /// Game side to move at the root.
    pub perspective: S,
    /// Fully completed depth.
    pub completed_depth: u16,
    /// Visits including partial iterations.
    pub nodes: u64,
    /// Continuation from the selected decision.
    pub principal_variation: Vec<C>,
    /// Completion or interruption reason.
    pub status: Status,
    /// Required adjudication, if any.
    pub unresolved: Option<P>,
    /// Game root assessment for games that report one.
    pub assessment: A,
}

/// Errors are unscored and retain transition diagnostics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SearchError<E> {
    /// Unsupported finite work limits.
    InvalidLimits,
    /// The rules rejected a command.
    Transition(E),
    /// The live root has no admitted board continuation.
    NoBoardActions,
    /// The private search disagreed with the game.
    Invariant,
}
impl<E: fmt::Debug> fmt::Display for SearchError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "root search failed: {self:?}")
    }
}
impl<E: fmt::Debug> Error for SearchError<E> {}

type Move<G> = <<G as RootGame>::Position as SearchPosition>::Move;
/// Engine observations for a specific game.
pub type ResultFor<G> = SearchResult<
    <G as RootGame>::Choice,
    <G as RootGame>::Conclusion,
    <G as RootGame>::Side,
    <G as RootGame>::Pending,
    <G as RootGame>::Assessment,
>;
type Failure<G> = SearchError<<G as RootGame>::GameError>;

/// Game operations and policy used by the shared root search.
pub trait RootGame: Sized {
    /// Private search position.
    type Position: SearchPosition;
    /// Game board action.
    type Action: PartialEq;
    /// Board action or formal command.
    type Choice: Clone + PartialEq + From<Self::Action>;
    /// Game side to move.
    type Side: Copy + Eq;
    /// Game conclusion.
    type Conclusion: Copy;
    /// Request error.
    type GameError: fmt::Debug;
    /// Required adjudication (uninhabited in games without it).
    type Pending: Clone;
    /// No-win assessment, or unit.
    type Assessment;
    /// Megabytes in a fresh search table.
    const TABLE_MB: usize = 16;
    /// A deadline can return a game root-order move before an iteration completes.
    const TIMED_FALLBACK: bool = false;
    /// A time or node limit can return a legal move from an incomplete first iteration.
    const LIMIT_FALLBACK: bool = false;
    /// Observe a terminal result or timed fallback even with no visits available.
    const TIMED_ZERO_NODES: bool = false;
    /// A first-action game conclusion overrides the private score.
    const SCORE_FIRST_CONCLUSION: bool = false;

    /// Game side to move.
    fn side(&self) -> Self::Side;
    /// Committed conclusion, if play has ended.
    fn finished(&self) -> Option<Self::Conclusion>;
    /// Winner of the game conclusion, or none for a draw.
    fn winner(conclusion: Self::Conclusion) -> Option<Self::Side>;
    /// Isolated game for replay with game admission.
    fn copy_for_replay(&self) -> Self;
    /// Root assessment, including on interrupted requests.
    fn assessment(&self) -> Self::Assessment;
    /// A witness that blocks play at this position.
    fn pending(&self) -> Option<&Self::Pending> {
        None
    }
    /// Handle game-specific adjudication or declaration before generating moves.
    fn root_decision(
        &self,
        _result: &mut ResultFor<Self>,
        _limits: Limits,
        _timed: bool,
        _stop: &mut dyn FnMut() -> bool,
    ) -> Result<bool, Failure<Self>> {
        Ok(false)
    }
    /// Legal board actions in order. None indicates cooperative cancellation.
    fn legal(
        &self,
        stop: &mut dyn FnMut() -> bool,
    ) -> Result<Option<Vec<Self::Action>>, Failure<Self>>;
    /// Build the private position with any game root restrictions.
    fn position_for_search(&self, legal: &[Self::Action]) -> Self::Position;
    /// Convert a private move at the current replay position.
    fn action(position: &Self::Position, m: Move<Self>) -> Self::Action;
    /// Convert the first legal action for games that publish a fallback.
    fn fallback(
        _position: &Self::Position,
        _action: &Self::Action,
    ) -> Result<Option<Move<Self>>, Failure<Self>> {
        Ok(None)
    }
    /// Submit a choice in this copy's current context.
    fn play(
        &mut self,
        choice: &Self::Choice,
        stop: &mut dyn FnMut() -> bool,
    ) -> Result<(), Self::GameError>;
    /// A cancellation reported by game admission.
    fn stopped(_error: &Self::GameError) -> bool {
        false
    }
    /// Check cancellation before starting costly game replay.
    fn stop_before_replay(_stop: &mut dyn FnMut() -> bool) -> bool {
        false
    }
    /// Formal draw claim available at this root. Check history before allocating search time.
    fn draw_claim(&self) -> Option<Self::Choice> {
        None
    }
    /// Decode the next move, optionally ending the private line.
    fn replay_action(position: &mut Self::Position, m: Move<Self>) -> Option<Self::Action> {
        Some(Self::action(position, m))
    }
    /// Advance a state-dependent decoder after game admission.
    fn advance(_position: &mut Self::Position, _m: Move<Self>) -> Result<(), Failure<Self>> {
        Ok(())
    }
}

/// Score a rules-owned conclusion from the root side.
pub fn terminal_score<G: RootGame>(conclusion: G::Conclusion, side: G::Side, distance: u16) -> i32 {
    let win = WIN_SCORE - i32::from(distance);
    match G::winner(conclusion) {
        None => 0,
        Some(winner) if winner == side => win,
        Some(_) => -win,
    }
}

/// Search with fresh private state.
pub fn search<G: RootGame>(game: &G, limits: Limits) -> Result<ResultFor<G>, Failure<G>> {
    search_with_stop(game, limits, || false)
}

/// Search with fresh state and cooperative cancellation.
pub fn search_with_stop<G: RootGame>(
    game: &G,
    limits: Limits,
    stop: impl FnMut() -> bool,
) -> Result<ResultFor<G>, Failure<G>> {
    search_with_state(game, limits, None, &mut Searcher::new(G::TABLE_MB), stop)
}

/// Search with caller-owned state. The game remains the authority on
/// root decisions and on each action of the reported line.
pub fn search_with_state<G: RootGame>(
    game: &G,
    limits: Limits,
    time: Option<TimeBudget>,
    engine: &mut Searcher<G::Position>,
    mut stop: impl FnMut() -> bool,
) -> Result<ResultFor<G>, Failure<G>> {
    let timing = RequestTime::new(time);
    let mut stop = || stop() || (!(G::TIMED_FALLBACK || G::LIMIT_FALLBACK) && timing.expired());
    if !limits.valid() {
        return Err(SearchError::InvalidLimits);
    }
    let side = game.side();
    let mut result = SearchResult {
        decision: None,
        score: None,
        perspective: side,
        completed_depth: 0,
        nodes: 0,
        principal_variation: vec![],
        status: Status::Complete,
        unresolved: None,
        assessment: game.assessment(),
    };
    if stop() {
        result.status = Status::Stopped;
        return Ok(result);
    }
    if limits.nodes == 0 && !(G::LIMIT_FALLBACK || G::TIMED_ZERO_NODES && time.is_some()) {
        result.status = Status::NodeLimit;
        return Ok(result);
    }
    if let Some(conclusion) = game.finished() {
        result.decision = Some(Decision::Finished(conclusion));
        result.score = Some(terminal_score::<G>(conclusion, side, 0));
        result.completed_depth = u16::from(limits.nodes > 0);
        result.nodes = u64::from(limits.nodes > 0);
        return Ok(result);
    }
    if let Some(pending) = game.pending() {
        result.unresolved = Some(pending.clone());
        result.status = Status::Unresolved;
        result.nodes = 1;
        return Ok(result);
    }
    if game.root_decision(&mut result, limits, time.is_some(), &mut stop)? {
        return Ok(result);
    }
    let Some(legal) = game.legal(&mut stop)? else {
        result.status = Status::Stopped;
        return Ok(result);
    };
    let first = legal.first().ok_or(SearchError::NoBoardActions)?;
    let draw_claim = game.draw_claim();
    let mut position = game.position_for_search(&legal);
    let fallback = if G::LIMIT_FALLBACK || time.is_some() {
        G::fallback(&position, first)?
    } else {
        None
    };
    let budget = timing.limits(limits.depth, Some(limits.nodes));
    let outcome = match fallback {
        Some(m) => {
            engine.search_with_fallback(&mut position, budget, &mut stop, m, G::LIMIT_FALLBACK)
        }
        None => engine.search(&mut position, budget, &mut stop),
    };
    result.nodes = outcome.nodes;
    result.status = outcome.status.into();
    let Some(best) = outcome.best else {
        return Ok(result);
    };
    result.completed_depth = outcome.depth;
    if let Some(choice) = draw_claim.filter(|_| outcome.score.is_some_and(|score| score <= 0)) {
        let mut rules_position = game.copy_for_replay();
        rules_position
            .play(&choice, &mut stop)
            .map_err(SearchError::Transition)?;
        let conclusion = rules_position.finished().ok_or(SearchError::Invariant)?;
        if G::winner(conclusion).is_some() {
            return Err(SearchError::Invariant);
        }
        result.decision = Some(Decision::Choose(choice.clone()));
        result.principal_variation.push(choice);
        result.score = Some(0);
        return Ok(result);
    }
    let action = G::action(&position, best);
    if !legal.contains(&action) {
        return Err(SearchError::Invariant);
    }
    if G::stop_before_replay(&mut stop) {
        result.status = Status::Stopped;
        result.completed_depth = 0;
        return Ok(result);
    }
    let mut rules_position = game.copy_for_replay();
    let mut first_score = None;
    for &m in outcome.pv.iter().take(MAX_LINE) {
        let first = result.principal_variation.is_empty();
        if (!first && timing.expired())
            || rules_position.finished().is_some()
            || rules_position.pending().is_some()
        {
            break;
        }
        let Some(action) = G::replay_action(&mut position, m) else {
            break;
        };
        let choice = G::Choice::from(action);
        match rules_position.play(&choice, &mut stop) {
            Ok(()) => {}
            Err(error) if G::stopped(&error) => {
                result.status = Status::Stopped;
                if first {
                    result.completed_depth = 0;
                    return Ok(result);
                }
                break;
            }
            Err(error) if first => return Err(SearchError::Transition(error)),
            Err(_) => break,
        }
        G::advance(&mut position, m)?;
        if first && outcome.depth > 0 && G::SCORE_FIRST_CONCLUSION {
            first_score = rules_position
                .finished()
                .map(|c| terminal_score::<G>(c, side, 1));
        }
        result.principal_variation.push(choice);
    }
    let choice = G::Choice::from(action);
    if result.principal_variation.first() != Some(&choice) {
        return Err(SearchError::Invariant);
    }
    result.decision = Some(Decision::Choose(choice));
    result.score = first_score.or(outcome.score);
    Ok(result)
}

// Per-game names used by the engine and existing search callers.
macro_rules! root_api {
    ($nodes:literal) => {
        pub use crate::root::{Limits, MAX_DEPTH, MAX_LINE, MAX_NODES, WIN_SCORE};
        /// Finite interactive work limits for this game.
        pub const DEFAULT_LIMITS: Limits = Limits {
            depth: MAX_DEPTH,
            nodes: $nodes,
        };
        /// Default transposition table size in megabytes.
        pub const DEFAULT_TABLE_MB: usize = <Game as RootGame>::TABLE_MB;
        /// Persistent private search state.
        pub type Engine = crate::alphabeta::Searcher<<Game as RootGame>::Position>;
        /// Game choice or conclusion for this game.
        pub type Decision = crate::root::Decision<<Game as RootGame>::Choice, Conclusion>;
        /// Engine observations from one request.
        pub type SearchResult = crate::root::ResultFor<Game>;
        /// Search or game admission failure.
        pub type SearchError = crate::root::SearchError<GameError>;
        /// Start with an empty table and move statistics.
        pub fn new_engine() -> Engine {
            Engine::new(DEFAULT_TABLE_MB)
        }
        /// Deterministic search with fresh state.
        pub fn search(game: &Game, limits: Limits) -> Result<SearchResult, SearchError> {
            crate::root::search(game, limits)
        }
        /// Search with fresh state and cooperative cancellation.
        pub fn search_with_stop(
            game: &Game,
            limits: Limits,
            stop: impl FnMut() -> bool,
        ) -> Result<SearchResult, SearchError> {
            crate::root::search_with_stop(game, limits, stop)
        }
        /// Search with caller-owned state and an optional total time budget.
        pub fn search_with_state(
            game: &Game,
            limits: Limits,
            time: Option<crate::alphabeta::timing::TimeBudget>,
            engine: &mut Engine,
            stop: impl FnMut() -> bool,
        ) -> Result<SearchResult, SearchError> {
            crate::root::search_with_state(game, limits, time, engine, stop)
        }
        /// Terminal score in game command distance.
        pub fn terminal_score(c: Conclusion, side: Side, distance: u16) -> i32 {
            crate::root::terminal_score::<Game>(c, side, distance)
        }
    };
}
pub(crate) use root_api;

// The five rule modules expose these operations with the same shape.
macro_rules! types {
    ($pos:ty, $choice:ty, $copy:ident, $action:ident) => {
        type Position = $pos;
        type Action = Action;
        type Side = Side;
        type Conclusion = Conclusion;
        type Choice = $choice;
        type GameError = GameError;
        fn side(&self) -> Side {
            self.position().side_to_move()
        }
        fn finished(&self) -> Option<Conclusion> {
            self.conclusion()
        }
        fn copy_for_replay(&self) -> Self {
            self.$copy()
        }
        fn action(position: &Self::Position, m: position::Move) -> Action {
            position.$action(m)
        }
    };
}
pub(crate) use types;

macro_rules! no_adjudication {
    () => {
        type Pending = std::convert::Infallible;
        type Assessment = ();
        fn assessment(&self) {}
    };
}
pub(crate) use no_adjudication;

macro_rules! restricted_board {
    ($pos:ty) => {
        const SCORE_FIRST_CONCLUSION: bool = true;
        const LIMIT_FALLBACK: bool = true;
        fn winner(c: Conclusion) -> Option<Side> {
            c.winner()
        }
        fn position_for_search(&self, legal: &[Action]) -> Self::Position {
            let mut pos = <$pos>::from_game(self);
            pos.restrict_root(legal);
            pos
        }
        fn fallback(
            pos: &Self::Position,
            _action: &Action,
        ) -> Result<Option<position::Move>, SearchError> {
            Ok(pos.first_root_move())
        }
        fn stop_before_replay(stop: &mut dyn FnMut() -> bool) -> bool {
            stop()
        }
    };
}
pub(crate) use restricted_board;

// Both timed protocols need the first legal move, even at depth zero.
// Their compact moves also need the current private position to decode the PV.
macro_rules! timed_moves {
    ($pos:ty) => {
        const TIMED_FALLBACK: bool = true;
        fn position_for_search(&self, _legal: &[Action]) -> Self::Position {
            <$pos>::from_game(self)
        }
        fn fallback(
            pos: &Self::Position,
            action: &Action,
        ) -> Result<Option<position::Move>, SearchError> {
            pos.find(*action).map(Some).ok_or(SearchError::Invariant)
        }
        fn advance(pos: &mut Self::Position, m: position::Move) -> Result<(), SearchError> {
            if pos.make(m) {
                Ok(())
            } else {
                Err(SearchError::Invariant)
            }
        }
        fn play(
            &mut self,
            choice: &Choice,
            _stop: &mut dyn FnMut() -> bool,
        ) -> Result<(), GameError> {
            self.submit(GameRequest {
                expected: self.context(),
                actor: self.side(),
                command: choice.command(),
            })
            .map(drop)
        }
    };
}
pub(crate) use timed_moves;
