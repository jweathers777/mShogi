//! One engine adapter for the five games on the shared search core (MSH-222).
//!
//! Each game supplies its rules types and the calls that the generic code
//! makes ([`EngineGame`]); the results, their admission checks and the
//! validation of a reported line are the same for every game. Chess keeps its
//! own adapter (`ChessResult`).
use crate::{ExecutionContext, ExecutionResult, MAX_STDERR_BYTES, StopReason, WorkLimits};
use mshogi_core::native::{CanonicalRecord, GameKind, NativeSide};
use mshogi_search::alphabeta::{MATE_BOUND, SearchPosition, Searcher, timing::TimeBudget};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fmt::Debug;

/// A game on the shared search core, as the engine protocol sees it.
///
/// A constant or method here records a real difference between games.
pub trait EngineGame: Copy + Debug + Eq + 'static {
    /// Display name in diagnostics.
    const NAME: &'static str;
    /// Game kind of the requests that this game serves.
    const KIND: GameKind;
    /// Unit of a heuristic score in hundredths, plural.
    const SCORE_UNIT: &'static str;
    /// Unit of a terminal distance, singular and plural.
    const DISTANCE_UNIT: [&'static str; 2];
    /// Evaluator identity of current results.
    const EVALUATOR_ID: &'static str;
    /// Search identity of current results.
    const SEARCH_ID: &'static str;
    /// Score of a win at the analyzed position; a win in `d` scores `WIN_SCORE - d`.
    const WIN_SCORE: i32;
    /// Largest supported nominal depth.
    const MAX_DEPTH: u16;
    /// Largest supported node budget.
    const MAX_NODES: u64;
    /// Largest reported line, in choices.
    const MAX_LINE: usize;
    /// Measured default work of interactive play.
    const PLAY_LIMITS: WorkLimits;

    /// Player.
    type Side: Copy + Eq + Debug + Serialize + DeserializeOwned;
    /// Proposed choice and element of a reported line.
    type Choice: Clone + Eq + Debug + Serialize + DeserializeOwned;
    /// Game result decided by the rules.
    type Conclusion: Copy + Eq + Debug + Serialize + DeserializeOwned;
    /// Required adjudication that stops a search without a score.
    type Pending: Pending;
    /// Game with full history.
    type Game: Clone;
    /// Canonical game record.
    type Record: mshogi_core::native::incremental::IncrementalRecord<
            Game = Self::Game,
            Error: std::fmt::Display,
        >;
    /// Complete game command.
    type Command;
    /// Search position of the shared core.
    type Position: SearchPosition + 'static;

    /// This game's record, if `record` is one.
    fn record(record: &CanonicalRecord) -> Option<&Self::Record>;
    /// This game's result, if `result` is one.
    fn result(result: &ExecutionResult) -> Option<&EngineResult<Self>>;
    /// Wrap a result of this game.
    fn wrap(result: EngineResult<Self>) -> ExecutionResult;
    /// Game-qualified player.
    fn native_side(side: Self::Side) -> NativeSide;
    /// Player to move at the end of a record.
    fn record_side(record: &Self::Record) -> Self::Side;
    /// Conclusion at the end of a record.
    fn record_conclusion(record: &Self::Record) -> Option<Self::Conclusion>;
    /// Score of a conclusion at the analyzed position, for `side`.
    fn terminal_score(conclusion: Self::Conclusion, side: Self::Side) -> i32;
    /// Replay a record.
    fn to_game(record: &Self::Record) -> Result<Self::Game, String>;
    /// Does the current record extend the exact earlier root and history?
    fn extends(current: &Self::Record, earlier: &Self::Record) -> bool;
    /// New commands and effects after a transport-authenticated prefix.
    fn suffix(current: &Self::Record, start: usize) -> crate::incremental::Delta;
    /// Select this game's suffix from a transport request.
    fn delta(
        delta: &crate::incremental::Delta,
    ) -> Option<&<Self::Record as mshogi_core::native::incremental::IncrementalRecord>::Delta>;
    /// Event and board-action counts in the retained game.
    fn progress(game: &Self::Game) -> (usize, usize);
    /// Player to move.
    fn side_to_move(game: &Self::Game) -> Self::Side;
    /// Actor and command of a choice when `turn` is to move.
    fn command(choice: &Self::Choice, turn: Self::Side) -> (Self::Side, Self::Command);
    /// Submit a command in the current context of `game`.
    fn submit(
        game: &mut Self::Game,
        actor: Self::Side,
        command: Self::Command,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String>;
    /// Search state with an empty table.
    fn new_engine() -> Searcher<Self::Position>;
    /// Search `game` with caller-owned state.
    fn search(
        game: &Self::Game,
        limits: WorkLimits,
        time: Option<TimeBudget>,
        engine: &mut Searcher<Self::Position>,
    ) -> Result<EngineResult<Self>, String>;
    /// Shape check of a choice that is a formal command, not a board move
    /// (a Shogi declaration, a Xiangqi count claim); `None` for a board move.
    fn command_shape(_choice: &Self::Choice, _telemetry: &EngineTelemetry<Self>) -> Option<bool> {
        None
    }

    /// Supported depth and node budget.
    fn valid_limits(limits: WorkLimits) -> bool {
        (1..=Self::MAX_DEPTH).contains(&limits.depth) && limits.nodes <= Self::MAX_NODES
    }
}

/// A required adjudication that stops a search without a score.
pub trait Pending: Clone + Eq + Debug + Serialize + DeserializeOwned {
    /// The witness fits in a bounded diagnostic.
    fn bounded(&self) -> bool;
}
/// Pending adjudication of a game that has none.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum NoPending {}
impl Pending for NoPending {
    fn bounded(&self) -> bool {
        match *self {}
    }
}
impl Pending for mshogi_core::xiangqi::PendingAdjudication {
    fn bounded(&self) -> bool {
        self.reason.len() <= MAX_STDERR_BYTES
    }
}

/// Actual bounded search observations, from the analyzed player's side.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, bound = "")]
pub struct EngineTelemetry<G: EngineGame> {
    /// Evaluator identity.
    pub evaluator_id: String,
    /// Search identity.
    pub search_id: String,
    /// Analyzed player.
    pub perspective: G::Side,
    /// Hundredths of the score unit, or terminal distance; absent until a
    /// completed iteration.
    #[serde(deserialize_with = "required_score")]
    pub score: Option<i32>,
    /// Last completed nominal iteration.
    pub completed_depth: u16,
    /// All admitted visits, including discarded work.
    pub nodes: u64,
    /// Continuation from completed work.
    pub principal_variation: Vec<G::Choice>,
    /// Completion, resource exhaustion, stop or pending adjudication; never a
    /// game outcome.
    pub status: StopReason,
}
fn required_score<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<i32>, D::Error> {
    Option::<i32>::deserialize(d)
}
impl<G: EngineGame> EngineTelemetry<G> {
    /// Telemetry of a search by the current evaluator and search.
    pub(crate) fn current(
        perspective: G::Side,
        score: Option<i32>,
        completed_depth: u16,
        nodes: u64,
        principal_variation: Vec<G::Choice>,
        status: StopReason,
    ) -> Self {
        Self {
            evaluator_id: G::EVALUATOR_ID.into(),
            search_id: G::SEARCH_ID.into(),
            perspective,
            score,
            completed_depth,
            nodes,
            principal_variation,
            status,
        }
    }
    fn is_current(&self) -> bool {
        self.evaluator_id == G::EVALUATOR_ID && self.search_id == G::SEARCH_ID
    }
    /// Score from the analyzed side. A win or loss distance is not
    /// necessarily mate; an unknown identity has no label.
    pub fn score_label(&self) -> Option<String> {
        let score = self.score.filter(|_| self.is_current())?;
        let magnitude = i64::from(score).abs();
        if magnitude > i64::from(G::WIN_SCORE) {
            return None;
        }
        if magnitude < i64::from(MATE_BOUND) {
            return Some(format!(
                "{:+.2} {}",
                f64::from(score) / 100.0,
                G::SCORE_UNIT
            ));
        }
        let outcome = if score >= 0 { "Win" } else { "Loss" };
        Some(match i64::from(G::WIN_SCORE) - magnitude {
            0 => format!("{outcome} at the analyzed position"),
            1 => format!("{outcome} in 1 {}", G::DISTANCE_UNIT[0]),
            distance => format!("{outcome} in {distance} {}", G::DISTANCE_UNIT[1]),
        })
    }
}

/// Proposed choice, rules conclusion, incomplete work or pending adjudication.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "snake_case",
    deny_unknown_fields,
    bound = ""
)]
pub enum EngineResult<G: EngineGame> {
    /// Proposed choice; the host still admits it.
    Choice {
        /// Board move or formal command.
        choice: G::Choice,
        /// Actual observations.
        telemetry: EngineTelemetry<G>,
    },
    /// The analyzed position is already concluded.
    Finished {
        /// Conclusion owned by the rules.
        conclusion: G::Conclusion,
        /// Terminal observation.
        telemetry: EngineTelemetry<G>,
    },
    /// No completed iteration, fallback choice or score.
    Incomplete {
        /// Actual unsuccessful work.
        telemetry: EngineTelemetry<G>,
    },
    /// A required adjudication remains open; the search has no score. Only a
    /// game with pending adjudication (Xiangqi) has this result.
    Unresolved {
        /// Pending witness.
        pending: G::Pending,
        /// Actual work.
        telemetry: EngineTelemetry<G>,
    },
}
/// Root decision of a search.
pub(crate) enum Decision<G: EngineGame> {
    Choose(G::Choice),
    Finished(G::Conclusion),
    Unresolved(G::Pending),
}
impl<G: EngineGame> EngineResult<G> {
    pub(crate) fn searched(decision: Option<Decision<G>>, telemetry: EngineTelemetry<G>) -> Self {
        match decision {
            Some(Decision::Choose(choice)) => Self::Choice { choice, telemetry },
            Some(Decision::Finished(conclusion)) => Self::Finished {
                conclusion,
                telemetry,
            },
            Some(Decision::Unresolved(pending)) => Self::Unresolved { pending, telemetry },
            None => Self::Incomplete { telemetry },
        }
    }
    /// Actual telemetry of any result.
    pub fn telemetry(&self) -> &EngineTelemetry<G> {
        match self {
            Self::Choice { telemetry, .. }
            | Self::Finished { telemetry, .. }
            | Self::Incomplete { telemetry }
            | Self::Unresolved { telemetry, .. } => telemetry,
        }
    }
    /// Identity, limit and shape checks against the submitted context. This
    /// runs while the host polls the engine: it uses no replay.
    pub(crate) fn matches(&self, context: &ExecutionContext) -> bool {
        let Some(record) = G::record(&context.game) else {
            return false;
        };
        let limits = context.limits;
        let t = self.telemetry();
        if !t.is_current()
            || !G::valid_limits(limits)
            || t.perspective != G::record_side(record)
            || t.completed_depth > limits.depth
            || t.nodes > limits.nodes
            || (t.status == StopReason::NodeLimit && t.nodes != limits.nodes)
            || t.principal_variation.len() > G::MAX_LINE
            || t.score
                .is_some_and(|s| !(-G::WIN_SCORE..=G::WIN_SCORE).contains(&s))
            || (t.status == StopReason::Unresolved) != matches!(self, Self::Unresolved { .. })
        {
            return false;
        }
        let open = G::record_conclusion(record).is_none();
        let unscored = t.completed_depth == 0 && t.score.is_none();
        match self {
            Self::Choice { choice, .. } => {
                // A completed iteration reached the requested depth, or the
                // root mate search proved a win. A choice never scores a win
                // or loss at its own root.
                let completed = t.completed_depth > 0
                    && t.nodes >= u64::from(t.completed_depth)
                    && t.score
                        .is_some_and(|s| s.unsigned_abs() < G::WIN_SCORE.unsigned_abs())
                    && (t.status != StopReason::Complete
                        || t.completed_depth == limits.depth
                        || t.score.is_some_and(|s| s >= MATE_BOUND));
                // A timed search can return its first legal move unsearched.
                let fallback = limits.time_ms.is_some()
                    && unscored
                    && t.status != StopReason::Complete
                    && t.principal_variation.len() == 1;
                open && t.principal_variation.first() == Some(choice)
                    && match G::command_shape(choice, t) {
                        Some(valid) => valid && completed,
                        None => completed || fallback,
                    }
            }
            Self::Finished { conclusion, .. } => {
                // A timed request with no node budget observes the conclusion
                // without a visit.
                G::record_conclusion(record) == Some(*conclusion)
                    && t.status == StopReason::Complete
                    && t.principal_variation.is_empty()
                    && t.score == Some(G::terminal_score(*conclusion, t.perspective))
                    && ((t.completed_depth == 1 && t.nodes == 1)
                        || (limits.time_ms.is_some()
                            && limits.nodes == 0
                            && t.completed_depth == 0
                            && t.nodes == 0))
            }
            Self::Incomplete { .. } => {
                unscored && t.principal_variation.is_empty() && t.status != StopReason::Complete
            }
            Self::Unresolved { pending, .. } => {
                open && unscored && t.principal_variation.is_empty() && pending.bounded()
            }
        }
    }
    /// Replay the reported line on a copy of `game`, the game of the
    /// submitted context. Check the result with `matches` first: it owns the
    /// identity, limit and shape checks. A heuristic score is not checked.
    pub fn validate(&self, game: &G::Game) -> Result<(), String> {
        self.validate_with_stop(game, &mut || false)
    }
    /// [`Self::validate`] under the caller's stop.
    pub fn validate_with_stop(
        &self,
        game: &G::Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        let stopped = || format!("{} observation validation stopped", G::NAME);
        if stop() {
            return Err(stopped());
        }
        let line = &self.telemetry().principal_variation;
        if line.is_empty() {
            return Ok(());
        }
        let mut branch = game.clone();
        for choice in line {
            if stop() {
                return Err(stopped());
            }
            let (actor, command) = G::command(choice, G::side_to_move(&branch));
            G::submit(&mut branch, actor, command, stop)?;
        }
        Ok(())
    }
}

/// Evaluate `$body` with `$G` naming the engine game of a game kind; Chess,
/// which has its own adapter, evaluates `$chess`.
macro_rules! for_game {
    ($kind:expr, $G:ident => $body:expr, chess => $chess:expr) => {
        match $kind {
            mshogi_core::native::GameKind::Chess => $chess,
            mshogi_core::native::GameKind::Shogi => {
                type $G = $crate::game::Shogi;
                $body
            }
            mshogi_core::native::GameKind::Xiangqi => {
                type $G = $crate::game::Xiangqi;
                $body
            }
            mshogi_core::native::GameKind::Tori => {
                type $G = $crate::game::Tori;
                $body
            }
            mshogi_core::native::GameKind::Chu => {
                type $G = $crate::game::Chu;
                $body
            }
            mshogi_core::native::GameKind::Tenjiku => {
                type $G = $crate::game::Tenjiku;
                $body
            }
        }
    };
}
pub(crate) use for_game;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ExecutionObservation, ExecutionPurpose, game::*};
    use mshogi_core::{chu, shogi, tenjiku, tori, xiangqi};

    fn context(game: CanonicalRecord, limits: WorkLimits) -> ExecutionContext {
        ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: 1,
            session_version: 1,
            game,
            limits,
        }
    }
    fn telemetry<G: EngineGame>(perspective: G::Side) -> EngineTelemetry<G> {
        EngineTelemetry::current(perspective, None, 0, 0, vec![], StopReason::Stopped)
    }
    fn roots() -> Vec<CanonicalRecord> {
        let mut roots = vec![
            CanonicalRecord::shogi(shogi::GameRecord::from_game(&shogi::Game::new())),
            CanonicalRecord::xiangqi(
                xiangqi::GameRecord::from_game(
                    &xiangqi::Game::standard(xiangqi::ClaimPolicy::Explicit).unwrap(),
                )
                .unwrap(),
            ),
            CanonicalRecord::tori(tori::GameRecord::from_game(&tori::Game::standard()).unwrap()),
            CanonicalRecord::chu(chu::GameRecord::from_game(&chu::Game::standard()).unwrap()),
        ];
        for profile in tenjiku::Profile::ALL {
            let game = tenjiku::Game::standard(profile);
            roots.push(CanonicalRecord::tenjiku(
                tenjiku::GameRecord::from_game(&game).unwrap(),
            ));
        }
        roots
    }

    /// An unknown identity or an unsupported request never matches.
    fn identity_and_bounds<G: EngineGame>(root: &CanonicalRecord) {
        let side = G::record_side(G::record(root).unwrap());
        let mut c = context(root.clone(), G::PLAY_LIMITS);
        let current = EngineResult::<G>::Incomplete {
            telemetry: telemetry(side),
        };
        assert!(current.matches(&c), "{}", G::NAME);
        for field in 0..2 {
            let mut t = telemetry::<G>(side);
            if field == 0 {
                t.evaluator_id = "mshogi.unknown.v1".into();
            } else {
                t.search_id = "mshogi.unknown.v1".into();
            }
            t.score = Some(0);
            assert_eq!(t.score_label(), None);
            t.score = None;
            assert!(!EngineResult::Incomplete { telemetry: t }.matches(&c));
        }
        c.limits.depth = G::MAX_DEPTH + 1;
        assert!(!current.matches(&c), "{}", G::NAME);
        c.limits.depth = 1;
        c.limits.nodes = G::MAX_NODES + 1;
        assert!(!current.matches(&c), "{}", G::NAME);
        c.limits = G::PLAY_LIMITS;
        c.game = roots()
            .into_iter()
            .find(|other| other.game_kind() != G::KIND)
            .unwrap();
        assert!(!current.matches(&c), "{}", G::NAME);
    }
    #[test]
    fn unknown_identities_excess_bounds_and_other_games_do_not_match() {
        for root in roots() {
            for_game!(root.game_kind(), G => identity_and_bounds::<G>(&root), chess => unreachable!());
        }
    }

    #[test]
    fn score_labels_keep_each_game_unit_and_terminal_distance() {
        let mut t = telemetry::<Shogi>(shogi::Side::Gote);
        assert_eq!(t.score_label(), None);
        let win = Shogi::WIN_SCORE;
        for (score, expected) in [
            (125, "+1.25 pawns"),
            (-350, "-3.50 pawns"),
            (0, "+0.00 pawns"),
            (win, "Win at the analyzed position"),
            (win - 1, "Win in 1 command edge (including declarations)"),
            (-win + 4, "Loss in 4 command edges (including declarations)"),
        ] {
            t.score = Some(score);
            assert_eq!(t.score_label().as_deref(), Some(expected));
        }
        t.score = Some(i32::MIN);
        assert_eq!(t.score_label(), None);
        let mut t = telemetry::<Chu>(chu::Side::Sente);
        t.score = Some(Chu::WIN_SCORE - 20);
        assert_eq!(t.score_label().as_deref(), Some("Win in 20 board actions"));
        let mut t = telemetry::<Tori>(tori::Side::Sente);
        t.score = Some(-250);
        assert_eq!(t.score_label().as_deref(), Some("-2.50 Swallows"));
        let mut t = telemetry::<Xiangqi>(xiangqi::Side::Red);
        t.score = Some(Xiangqi::WIN_SCORE - 20);
        assert_eq!(
            t.score_label().as_deref(),
            Some("Win in 20 native command edges")
        );
    }

    fn shogi_context() -> ExecutionContext {
        let root = CanonicalRecord::shogi(shogi::GameRecord::from_game(&shogi::Game::new()));
        context(
            root,
            WorkLimits {
                depth: 2,
                nodes: 5000,
                time_ms: None,
            },
        )
    }
    #[test]
    fn reported_line_is_bounded() {
        let c = shogi_context();
        let action = ShogiChoice::Move {
            action: "7g7f".parse().unwrap(),
        };
        let mut t = EngineTelemetry::current(
            shogi::Side::Sente,
            Some(0),
            2,
            100,
            vec![action; Shogi::MAX_LINE],
            StopReason::Complete,
        );
        let result = |t: &ShogiTelemetry| ShogiResult::Choice {
            choice: action,
            telemetry: t.clone(),
        };
        assert!(result(&t).matches(&c));
        t.principal_variation.push(action);
        assert!(!result(&t).matches(&c));
    }

    fn timed_search(game: &shogi::Game) -> ShogiResult {
        let limits = WorkLimits {
            depth: 2,
            nodes: 0,
            time_ms: Some(1),
        };
        let time = Some(TimeBudget::from_hard_ms(0));
        Shogi::search(game, limits, time, &mut Shogi::new_engine()).unwrap()
    }
    #[test]
    fn timed_zero_depth_choice_matches_without_inventing_score_or_work() {
        let mut c = shogi_context();
        c.limits.time_ms = Some(1);
        c.limits.nodes = 0;
        let result = timed_search(&shogi::Game::new());
        assert!(result.matches(&c));
        let observation = ExecutionObservation::new(
            c.clone(),
            ExecutionResult::Shogi {
                result: result.clone(),
            },
        );
        let saved = serde_json::to_vec(&observation).unwrap();
        let restored: ExecutionObservation = serde_json::from_slice(&saved).unwrap();
        assert!(restored.matches(&c));
        c.limits.time_ms = None;
        assert!(!result.matches(&c));
        c.limits.time_ms = Some(1);
        let ShogiResult::Choice { choice, telemetry } = result else {
            panic!()
        };
        assert_eq!(telemetry.score_label(), None);
        assert_eq!(telemetry.completed_depth, 0);
        for mutation in 0..5 {
            let mut bad = telemetry.clone();
            match mutation {
                0 => bad.score = Some(0),
                1 => bad.status = StopReason::Complete,
                2 => bad.principal_variation.push(choice),
                3 => bad.nodes = 1,
                _ => bad.search_id = "mshogi.shogi.unknown.v1".into(),
            }
            let bad = ShogiResult::Choice {
                choice,
                telemetry: bad,
            };
            assert!(!bad.matches(&c));
        }
    }

    #[test]
    fn timed_zero_node_terminal_observes_native_result_without_search() {
        let mut game = shogi::Game::new();
        game.submit(shogi::GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: shogi::GameCommand::Resign,
        })
        .unwrap();
        let mut c = shogi_context();
        c.game = CanonicalRecord::shogi(shogi::GameRecord::from_game(&game));
        c.limits.nodes = 0;
        c.limits.time_ms = Some(1);
        let result = timed_search(&game);
        assert!(matches!(result, ShogiResult::Finished { .. }));
        assert!(result.matches(&c));
        c.limits.time_ms = None;
        assert!(!result.matches(&c));
    }

    fn line<G: EngineGame>(
        perspective: G::Side,
        score: i32,
        depth: u16,
        nodes: u64,
        line: Vec<G::Choice>,
    ) -> EngineTelemetry<G> {
        EngineTelemetry::current(
            perspective,
            Some(score),
            depth,
            nodes,
            line,
            StopReason::Complete,
        )
    }
    #[test]
    fn choice_work_and_root_score_rules_hold_in_every_game() {
        let c = shogi_context();
        let sente = shogi::Side::Sente;
        let action = ShogiChoice::Move {
            action: "7g7f".parse().unwrap(),
        };
        let choice = |t: EngineTelemetry<Shogi>| ShogiResult::Choice {
            choice: action,
            telemetry: t,
        };
        assert!(choice(line(sente, 0, 2, 100, vec![action])).matches(&c));
        // A node limit is the exact budget.
        let mut t = line(sente, 0, 1, 100, vec![action]);
        t.status = StopReason::NodeLimit;
        assert!(!choice(t.clone()).matches(&c));
        t.nodes = c.limits.nodes;
        assert!(choice(t).matches(&c));
        // A complete search reaches the requested depth unless it proves a win.
        assert!(!choice(line(sente, 0, 1, 100, vec![action])).matches(&c));
        assert!(choice(line(sente, MATE_BOUND, 1, 100, vec![action])).matches(&c));
        // A choice never wins or loses at its own root.
        let win = Shogi::WIN_SCORE;
        assert!(!choice(line(sente, win, 2, 100, vec![action])).matches(&c));
        assert!(!choice(line(sente, -win, 2, 100, vec![action])).matches(&c));
    }
    #[test]
    fn formal_commands_have_their_own_shape() {
        let c = shogi_context();
        let declare = |actor| ShogiChoice::Declare { actor };
        let shogi = |actor, nodes| ShogiResult::Choice {
            choice: declare(actor),
            telemetry: line(
                shogi::Side::Sente,
                Shogi::WIN_SCORE - 1,
                1,
                nodes,
                vec![declare(actor)],
            ),
        };
        assert!(shogi(shogi::Side::Sente, 2).matches(&c));
        assert!(!shogi(shogi::Side::Gote, 2).matches(&c));
        assert!(!shogi(shogi::Side::Sente, 3).matches(&c));
        let root = xiangqi::Game::standard(xiangqi::ClaimPolicy::Explicit).unwrap();
        let c = context(
            CanonicalRecord::xiangqi(xiangqi::GameRecord::from_game(&root).unwrap()),
            Xiangqi::PLAY_LIMITS,
        );
        let depth = c.limits.depth;
        let claim = |actor| XiangqiChoice::Claim { actor };
        let xiangqi = |actor, score| XiangqiResult::Choice {
            choice: claim(actor),
            telemetry: line(xiangqi::Side::Red, score, depth, 100, vec![claim(actor)]),
        };
        assert!(xiangqi(xiangqi::Side::Red, 0).matches(&c));
        assert!(!xiangqi(xiangqi::Side::Black, 0).matches(&c));
        assert!(!xiangqi(xiangqi::Side::Red, 10).matches(&c));
    }
    #[test]
    fn xiangqi_timed_fallback_and_unresolved_status_agree_with_the_result() {
        let root = xiangqi::Game::standard(xiangqi::ClaimPolicy::Explicit).unwrap();
        let mut c = context(
            CanonicalRecord::xiangqi(xiangqi::GameRecord::from_game(&root).unwrap()),
            WorkLimits {
                depth: 4,
                nodes: 1000,
                time_ms: Some(10),
            },
        );
        let action = XiangqiChoice::Move {
            action: "b0c2".parse().unwrap(),
        };
        let mut t = telemetry::<Xiangqi>(xiangqi::Side::Red);
        t.nodes = 10;
        t.principal_variation = vec![action];
        let fallback = XiangqiResult::Choice {
            choice: action,
            telemetry: t,
        };
        assert!(fallback.matches(&c));
        c.limits.time_ms = None;
        assert!(!fallback.matches(&c));
        let pending = xiangqi::PendingAdjudication {
            occurrences: vec![0, 4],
            reason: "classification required".into(),
        };
        let mut t = telemetry::<Xiangqi>(xiangqi::Side::Red);
        t.nodes = 1;
        t.status = StopReason::Unresolved;
        let unresolved = |t: &XiangqiTelemetry| XiangqiResult::Unresolved {
            pending: pending.clone(),
            telemetry: t.clone(),
        };
        assert!(unresolved(&t).matches(&c));
        let incomplete = XiangqiResult::Incomplete {
            telemetry: t.clone(),
        };
        assert!(!incomplete.matches(&c));
        t.status = StopReason::Stopped;
        assert!(!unresolved(&t).matches(&c));
    }
}
