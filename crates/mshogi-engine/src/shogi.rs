//! Actual shogi results, distinct from chess scores and cooperative Full proofs.
use crate::{ExecutionContext, ExecutionPurpose, StopReason};
use mshogi_core::{
    native::CanonicalRecord,
    shogi::{Action, Conclusion, Side},
};
use mshogi_search::shogi as search;
use serde::{Deserialize, Serialize};

/// Time-primary Shogi play; depth and nodes remain finite safety ceilings.
pub const SHOGI_PLAY_LIMITS: crate::WorkLimits = crate::WorkLimits {
    time_ms: Some(1_000),
    depth: search::MAX_DEPTH,
    nodes: search::MAX_NODES,
};

/// Native shogi complete continuation choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ShogiChoice {
    /// Complete move/drop/promotion USI primitive.
    Move {
        /// Authoritative native board action; never a null/resign/declaration token.
        action: Action,
    },
    /// Formal entering-king declaration, not a board action or concluded response.
    Declare {
        /// Native named actor; turn is unchanged by submission.
        actor: Side,
    },
}
impl From<search::Choice> for ShogiChoice {
    fn from(choice: search::Choice) -> Self {
        match choice {
            search::Choice::Move(action) => Self::Move { action },
            search::Choice::Declare { actor } => Self::Declare { actor },
        }
    }
}
impl From<ShogiChoice> for search::Choice {
    fn from(choice: ShogiChoice) -> Self {
        match choice {
            ShogiChoice::Move { action } => Self::Move(action),
            ShogiChoice::Declare { actor } => Self::Declare { actor },
        }
    }
}
/// Honest completed-iteration observations with all attempted admitted visits.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShogiTelemetry {
    /// Fixed native evaluator configuration identity.
    pub evaluator_id: String,
    /// Search configuration identity.
    pub search_id: String,
    /// Native root side, independent of final command actor or terminal winner.
    pub perspective: Side,
    /// Hundredths-of-pawn heuristic, or native terminal win distance; null before
    /// any completed work. Terminal distance includes declaration command edges.
    #[serde(deserialize_with = "required_score")]
    pub score: Option<i32>,
    /// Last complete nominal depth; declarations remain explicit choices.
    pub completed_depth: u16,
    /// All admitted visits, including discarded partial iterations.
    pub nodes: u64,
    /// Native continuation from completed work, possibly ending in declaration.
    pub principal_variation: Vec<ShogiChoice>,
    /// Actual completion, work exhaustion or external stop, never a game outcome.
    pub status: StopReason,
}
impl ShogiTelemetry {
    fn is_current(&self) -> bool {
        self.evaluator_id == search::EVALUATOR_ID && self.search_id == search::SEARCH_ID
    }

    /// Player-facing value in the native root-side interpretation. Terminal
    /// distance counts complete command edges, including formal declarations;
    /// it is deliberately never described as a chess mate distance.
    pub fn score_label(&self) -> Option<String> {
        if !self.is_current() {
            return None;
        }
        self.score.and_then(|score| {
            let magnitude = i64::from(score).abs();
            if magnitude > i64::from(search::WIN_SCORE) {
                return None;
            }
            Some(
                if magnitude >= i64::from(mshogi_search::alphabeta::MATE_BOUND) {
                    let distance = i64::from(search::WIN_SCORE) - magnitude;
                    let result = if score >= 0 { "Win" } else { "Loss" };
                    if distance == 0 {
                        format!("{result} at the analyzed position")
                    } else {
                        format!(
                            "{result} in {distance} command {} (including declarations)",
                            if distance == 1 { "edge" } else { "edges" }
                        )
                    }
                } else {
                    format!("{:+.2} pawns", f64::from(score) / 100.0)
                },
            )
        })
    }
}
fn required_score<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<i32>, D::Error> {
    Option::<i32>::deserialize(d)
}
/// Native choice, actual concluded game, or explicit incomplete first iteration.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ShogiResult {
    /// Proposed complete choice; only native host admission can commit it.
    Choice {
        /// Board action or formal declaration remains explicit.
        choice: ShogiChoice,
        /// Real observations, not declaration eligibility points.
        telemetry: ShogiTelemetry,
    },
    /// Root was already concluded by native rules/history.
    Finished {
        /// Exact native conclusion, including repetition/check intervals.
        conclusion: Conclusion,
        /// Actual terminal-root observation.
        telemetry: ShogiTelemetry,
    },
    /// No completed iteration, score or fallback choice.
    Incomplete {
        /// Attempted work and real interruption reason.
        telemetry: ShogiTelemetry,
    },
}
impl ShogiResult {
    pub(crate) fn searched(result: search::SearchResult) -> Self {
        let telemetry = ShogiTelemetry {
            evaluator_id: search::EVALUATOR_ID.into(),
            search_id: search::SEARCH_ID.into(),
            perspective: result.perspective,
            score: result.score,
            completed_depth: result.completed_depth,
            nodes: result.nodes,
            principal_variation: result
                .principal_variation
                .into_iter()
                .map(Into::into)
                .collect(),
            status: match result.status {
                mshogi_search::search::SearchStatus::Complete => StopReason::Complete,
                mshogi_search::search::SearchStatus::NodeLimit => StopReason::NodeLimit,
                mshogi_search::search::SearchStatus::Stopped => StopReason::Stopped,
            },
        };
        match result.decision {
            Some(search::Decision::Choose(choice)) => Self::Choice {
                choice: choice.into(),
                telemetry,
            },
            Some(search::Decision::Finished(conclusion)) => Self::Finished {
                conclusion,
                telemetry,
            },
            None => Self::Incomplete { telemetry },
        }
    }
    pub(crate) fn matches(&self, context: &ExecutionContext) -> bool {
        let CanonicalRecord::Shogi(record) = &context.game else {
            return false;
        };
        if matches!(
            context.purpose,
            ExecutionPurpose::ChessFullReachability { .. }
        ) {
            return false;
        }
        let telemetry = match self {
            Self::Choice { telemetry, .. }
            | Self::Finished { telemetry, .. }
            | Self::Incomplete { telemetry } => telemetry,
        };
        let win_score = search::WIN_SCORE;
        if !telemetry.is_current()
            || context.limits.depth == 0
            || context.limits.depth > search::MAX_DEPTH
            || context.limits.nodes > search::MAX_NODES
            || telemetry.perspective != record.side_to_move()
            || telemetry.completed_depth > context.limits.depth
            || telemetry.nodes > context.limits.nodes
            || telemetry.principal_variation.len() > search::MAX_LINE
            || telemetry
                .score
                .is_some_and(|s| !(-win_score..=win_score).contains(&s))
        {
            return false;
        }
        match self {
            Self::Choice { choice, .. } => {
                let timed_fallback = context.limits.time_ms.is_some()
                    && telemetry.completed_depth == 0
                    && telemetry.score.is_none()
                    && telemetry.status != StopReason::Complete
                    && telemetry.principal_variation.len() == 1
                    && matches!(choice, ShogiChoice::Move { .. });
                record.conclusion().is_none()
                    && (timed_fallback
                        || (telemetry.completed_depth > 0
                            && telemetry.nodes > 0
                            && telemetry.score.is_some()))
                    && telemetry.principal_variation.first() == Some(choice)
                    && match choice {
                        ShogiChoice::Declare { actor } => {
                            *actor == telemetry.perspective
                                && telemetry.score == Some(win_score - 1)
                                && telemetry.principal_variation.len() == 1
                                && telemetry.completed_depth == 1
                                && telemetry.nodes == 2
                                && telemetry.status == StopReason::Complete
                        }
                        ShogiChoice::Move { .. } => true,
                    }
            }
            Self::Finished { conclusion, .. } => {
                record.conclusion() == Some(*conclusion)
                    && ((telemetry.completed_depth == 1 && telemetry.nodes == 1)
                        || (context.limits.time_ms.is_some()
                            && context.limits.nodes == 0
                            && telemetry.completed_depth == 0
                            && telemetry.nodes == 0))
                    && telemetry.status == StopReason::Complete
                    && telemetry.score
                        == Some(match conclusion {
                            Conclusion::Draw(_) => 0,
                            Conclusion::Win { winner, .. } => {
                                if *winner == telemetry.perspective {
                                    win_score
                                } else {
                                    -win_score
                                }
                            }
                        })
                    && telemetry.principal_variation.is_empty()
            }
            Self::Incomplete { .. } => {
                telemetry.completed_depth == 0
                    && telemetry.score.is_none()
                    && telemetry.principal_variation.is_empty()
                    && telemetry.status != StopReason::Complete
            }
        }
    }
}

#[cfg(test)]
mod presentation_tests {
    use super::*;
    #[test]
    fn score_labels_retain_native_root_units_and_terminal_command_edges() {
        let mut telemetry = ShogiTelemetry {
            evaluator_id: search::EVALUATOR_ID.into(),
            search_id: search::SEARCH_ID.into(),
            perspective: Side::Gote,
            score: None,
            completed_depth: 0,
            nodes: 0,
            principal_variation: vec![],
            status: StopReason::Complete,
        };
        assert_eq!(telemetry.score_label(), None);
        for (score, expected) in [
            (125, "+1.25 pawns"),
            (-350, "-3.50 pawns"),
            (0, "+0.00 pawns"),
            (search::WIN_SCORE, "Win at the analyzed position"),
            (
                search::WIN_SCORE - 1,
                "Win in 1 command edge (including declarations)",
            ),
            (
                -search::WIN_SCORE + 4,
                "Loss in 4 command edges (including declarations)",
            ),
        ] {
            telemetry.score = Some(score);
            assert_eq!(telemetry.score_label().as_deref(), Some(expected));
        }
        telemetry.score = Some(i32::MIN);
        assert_eq!(telemetry.score_label(), None);
        telemetry.score = Some(0);
        telemetry.evaluator_id = "mshogi.shogi.unknown.v1".into();
        assert_eq!(telemetry.score_label(), None);
    }
}

#[cfg(test)]
mod identity_tests {
    use super::*;
    use mshogi_core::shogi::{Game, GameRecord};

    fn context() -> ExecutionContext {
        ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: 1,
            session_version: 1,
            game: CanonicalRecord::shogi(GameRecord::from_game(&Game::new())),
            limits: crate::WorkLimits {
                depth: 2,
                nodes: 5000,
                time_ms: None,
            },
        }
    }
    fn incomplete(evaluator: &str, search: &str) -> ShogiResult {
        ShogiResult::Incomplete {
            telemetry: ShogiTelemetry {
                evaluator_id: evaluator.into(),
                search_id: search.into(),
                perspective: Side::Sente,
                score: None,
                completed_depth: 0,
                nodes: 0,
                principal_variation: vec![],
                status: StopReason::Stopped,
            },
        }
    }
    #[test]
    fn unknown_identities_and_excess_request_bounds_do_not_match() {
        let mut c = context();
        assert!(incomplete(search::EVALUATOR_ID, search::SEARCH_ID).matches(&c));
        assert!(!incomplete(search::EVALUATOR_ID, "mshogi.shogi.unknown.v1").matches(&c));
        assert!(!incomplete("mshogi.shogi.unknown.v1", search::SEARCH_ID).matches(&c));
        c.limits.depth = search::MAX_DEPTH + 1;
        assert!(!incomplete(search::EVALUATOR_ID, search::SEARCH_ID).matches(&c));
        c.limits.depth = 1;
        c.limits.nodes = search::MAX_NODES + 1;
        assert!(!incomplete(search::EVALUATOR_ID, search::SEARCH_ID).matches(&c));
    }
    #[test]
    fn reported_line_is_bounded() {
        let c = context();
        let action = ShogiChoice::Move {
            action: "7g7f".parse().unwrap(),
        };
        let ShogiResult::Incomplete { mut telemetry } =
            incomplete(search::EVALUATOR_ID, search::SEARCH_ID)
        else {
            unreachable!()
        };
        telemetry.completed_depth = 1;
        telemetry.nodes = 100;
        telemetry.score = Some(0);
        telemetry.principal_variation = vec![action; search::MAX_LINE];
        let result = |telemetry: &ShogiTelemetry| ShogiResult::Choice {
            choice: action,
            telemetry: telemetry.clone(),
        };
        assert!(result(&telemetry).matches(&c));
        telemetry.principal_variation = vec![action; search::MAX_LINE + 1];
        assert!(!result(&telemetry).matches(&c));
    }

    #[test]
    fn timed_zero_depth_choice_matches_without_inventing_score_or_work() {
        let mut c = context();
        c.limits.time_ms = Some(1);
        c.limits.nodes = 0;
        let result = ShogiResult::searched(
            search::search_with_state(
                &Game::new(),
                search::Limits { depth: 2, nodes: 0 },
                Some(mshogi_search::alphabeta::timing::TimeBudget::from_hard_ms(
                    0,
                )),
                &mut search::new_engine(),
                || false,
            )
            .unwrap(),
        );
        assert!(result.matches(&c));
        let observation = crate::ExecutionObservation::new(
            c.clone(),
            crate::ExecutionResult::Shogi {
                result: result.clone(),
            },
        );
        let saved = serde_json::to_vec(&observation).unwrap();
        let restored: crate::ExecutionObservation = serde_json::from_slice(&saved).unwrap();
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
            assert!(
                !ShogiResult::Choice {
                    choice,
                    telemetry: bad
                }
                .matches(&c)
            );
        }
    }

    #[test]
    fn timed_zero_node_terminal_observes_native_result_without_search() {
        let mut game = Game::new();
        game.submit(mshogi_core::shogi::GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: mshogi_core::shogi::GameCommand::Resign,
        })
        .unwrap();
        let mut c = context();
        c.game = CanonicalRecord::shogi(GameRecord::from_game(&game));
        c.limits.nodes = 0;
        c.limits.time_ms = Some(1);
        let result = ShogiResult::searched(
            search::search_with_state(
                &game,
                search::Limits { depth: 2, nodes: 0 },
                Some(mshogi_search::alphabeta::timing::TimeBudget::from_hard_ms(
                    0,
                )),
                &mut search::new_engine(),
                || false,
            )
            .unwrap(),
        );
        assert!(result.matches(&c));
        c.limits.time_ms = None;
        assert!(!result.matches(&c));
    }
}

/// Maximum supported Shogi search depth for native consumers.
pub const SHOGI_MAX_DEPTH: u16 = search::MAX_DEPTH;
/// Maximum supported Shogi node budget for native consumers.
pub const SHOGI_MAX_NODES: u64 = search::MAX_NODES;

/// Shogi front-end access to the same timed history-prefix policy as the internal protocol.
/// Fixed-work searches use fresh state and leave the retained timed state alone.
#[derive(Default)]
pub struct ShogiSearchSession {
    slot: crate::server::Slot<search::Engine>,
}
impl ShogiSearchSession {
    /// Search one native history with optional elapsed-time control.
    pub fn search(
        &mut self,
        game: &mshogi_core::shogi::Game,
        limits: search::Limits,
        time: Option<mshogi_search::alphabeta::timing::TimeBudget>,
        stop: impl FnMut() -> bool,
    ) -> Result<search::SearchResult, search::SearchError> {
        use mshogi_search::alphabeta::timing::RequestTime;
        let timing = RequestTime::new(time);
        let record = mshogi_core::shogi::GameRecord::from_game(game);
        let mut engine = self
            .slot
            .acquire_relation(
                &CanonicalRecord::shogi(record.clone()),
                time.is_some(),
                |last| matches!(last, CanonicalRecord::Shogi(previous) if record.has_prefix(previous)),
                || Ok::<_, std::convert::Infallible>(search::new_engine()),
            )
            .expect("infallible engine allocation");
        search::search_with_state(game, limits, timing.remaining(), &mut engine, stop)
    }
}
