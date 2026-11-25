//! Actual native Xiangqi ordinary-search results.
use crate::ExecutionContext;
use mshogi_core::{
    native::CanonicalRecord,
    xiangqi::{Action, Conclusion, NoWinAssessment, PendingAdjudication, Side},
};
use mshogi_search::xiangqi as search;
use serde::{Deserialize, Serialize};

/// Measured default for ordinary interactive consumers; derived from the native search.
pub const XIANGQI_PLAY_LIMITS: crate::WorkLimits = crate::WorkLimits {
    time_ms: None,
    depth: search::DEFAULT_LIMITS.depth,
    nodes: search::DEFAULT_LIMITS.nodes,
};

/// Largest supported ordinary depth.
pub const XIANGQI_MAX_DEPTH: u16 = search::MAX_DEPTH;
/// Largest supported ordinary node budget.
pub const XIANGQI_MAX_NODES: u64 = search::MAX_NODES;

/// Native complete board choice versus formal count claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum XiangqiChoice {
    /// Proposed complete native action.
    Move {
        /// Native origin/destination, not an external protocol token.
        action: Action,
    },
    /// Optional native count claim, not a board action or already concluded draw.
    Claim {
        /// Native claimant.
        actor: Side,
    },
}
impl From<search::Choice> for XiangqiChoice {
    fn from(value: search::Choice) -> Self {
        match value {
            search::Choice::Move(action) => Self::Move { action },
            search::Choice::Claim { actor } => Self::Claim { actor },
        }
    }
}
impl From<XiangqiChoice> for search::Choice {
    fn from(value: XiangqiChoice) -> Self {
        match value {
            XiangqiChoice::Move { action } => Self::Move(action),
            XiangqiChoice::Claim { actor } => Self::Claim { actor },
        }
    }
}
/// Ordinary search execution state, never a chess-family outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum XiangqiStop {
    /// Requested depth or native terminal root completed.
    Complete,
    /// Total visit ceiling reached.
    NodeLimit,
    /// Cooperative stop observed.
    Stopped,
    /// Native required adjudication paused search without a score.
    Unresolved,
}
/// Actual observations using native Red/Black root perspective.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct XiangqiTelemetry {
    /// Versioned classical evaluation configuration.
    pub evaluator_id: String,
    /// Versioned search algorithm.
    pub search_id: String,
    /// Native analyzed root side; never inferred from a terminal winner.
    pub perspective: Side,
    /// Hundredths of soldier heuristic or native terminal command distance.
    #[serde(deserialize_with = "required_score")]
    pub score: Option<i32>,
    /// Last complete iteration, zero before any scored completion.
    pub completed_depth: u16,
    /// All admitted visits, including partial work and iterative revisits.
    pub nodes: u64,
    /// Completed complete-action/claim continuation.
    pub principal_variation: Vec<XiangqiChoice>,
    /// Actual stop/completion state.
    pub status: XiangqiStop,
    /// Recorded sound-fast-only assessment; Unknown is not positive winnability.
    pub no_win_assessment: NoWinAssessment,
}
impl XiangqiTelemetry {
    fn is_current(&self) -> bool {
        self.evaluator_id == search::EVALUATOR_ID && self.search_id == search::SEARCH_ID
    }
    /// Native root-side score label.
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
                    format!(
                        "{} in {} native command edges",
                        if score >= 0 { "Win" } else { "Loss" },
                        i64::from(search::WIN_SCORE) - magnitude
                    )
                } else {
                    format!("{:+.2} soldiers", f64::from(score) / 100.0)
                },
            )
        })
    }
}
fn required_score<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<i32>, D::Error> {
    Option::<i32>::deserialize(d)
}
/// Choice, already concluded game, incomplete work and unresolved adjudication.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum XiangqiResult {
    /// Proposal only; native host admission owns application.
    Choice {
        /// Complete native choice.
        choice: XiangqiChoice,
        /// Actual observations.
        telemetry: XiangqiTelemetry,
    },
    /// Native root conclusion, including history penalties and non-turn commands.
    Finished {
        /// Already committed native outcome.
        conclusion: Conclusion,
        /// Actual observations.
        telemetry: XiangqiTelemetry,
    },
    /// No complete scored iteration and no fallback move.
    Incomplete {
        /// Actual attempted work and interruption.
        telemetry: XiangqiTelemetry,
    },
    /// Required native history classification remains unresolved and unscored.
    Unresolved {
        /// Native pending witness, possibly from a searched descendant.
        pending: PendingAdjudication,
        /// Actual attempted work.
        telemetry: XiangqiTelemetry,
    },
}
impl XiangqiResult {
    pub(crate) fn searched(result: search::SearchResult) -> Self {
        let telemetry = XiangqiTelemetry {
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
                search::Status::Complete => XiangqiStop::Complete,
                search::Status::NodeLimit => XiangqiStop::NodeLimit,
                search::Status::Stopped => XiangqiStop::Stopped,
                search::Status::Unresolved => XiangqiStop::Unresolved,
            },
            no_win_assessment: result.assessment,
        };
        if let Some(pending) = result.unresolved {
            return Self::Unresolved { pending, telemetry };
        }
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
        let CanonicalRecord::Xiangqi(record) = &context.game else {
            return false;
        };
        let t = match self {
            Self::Choice { telemetry, .. }
            | Self::Finished { telemetry, .. }
            | Self::Incomplete { telemetry }
            | Self::Unresolved { telemetry, .. } => telemetry,
        };
        let win_score = search::WIN_SCORE;
        if !t.is_current()
            || context.limits.depth == 0
            || context.limits.depth > search::MAX_DEPTH
            || context.limits.nodes > search::MAX_NODES
            || t.perspective != record.side_to_move()
            || t.completed_depth > context.limits.depth
            || t.nodes > context.limits.nodes
            || t.principal_variation.len() > search::MAX_LINE
            || t.score
                .is_some_and(|s| !(-win_score..=win_score).contains(&s))
            || &t.no_win_assessment != record.no_win_assessment()
        {
            return false;
        }
        match self {
            Self::Choice { choice, .. } => {
                record.conclusion().is_none()
                    && t.completed_depth > 0
                    && t.nodes > 0
                    && t.score.is_some()
                    && t.principal_variation.first() == Some(choice)
                    && t.status != XiangqiStop::Unresolved
                    && match choice {
                        XiangqiChoice::Move { .. } => true,
                        XiangqiChoice::Claim { actor } => {
                            *actor == t.perspective
                                && t.score == Some(0)
                                && t.principal_variation.len() == 1
                        }
                    }
            }
            Self::Finished { conclusion, .. } => {
                record.conclusion() == Some(*conclusion)
                    && t.completed_depth == 1
                    && t.nodes == 1
                    && t.status == XiangqiStop::Complete
                    && t.principal_variation.is_empty()
                    && t.score
                        == Some(match conclusion {
                            Conclusion::Draw { .. } => 0,
                            Conclusion::Win { winner, .. } if *winner == t.perspective => win_score,
                            _ => -win_score,
                        })
            }
            Self::Incomplete { .. } => {
                t.completed_depth == 0
                    && t.score.is_none()
                    && t.principal_variation.is_empty()
                    && matches!(t.status, XiangqiStop::NodeLimit | XiangqiStop::Stopped)
            }
            Self::Unresolved { pending, .. } => {
                record.conclusion().is_none()
                    && t.completed_depth == 0
                    && t.score.is_none()
                    && t.principal_variation.is_empty()
                    && t.status == XiangqiStop::Unresolved
                    && pending.reason.len() <= crate::MAX_STDERR_BYTES
            }
        }
    }
}

#[cfg(test)]
mod identity_tests {
    use super::*;
    use mshogi_core::xiangqi::{ClaimPolicy, Game, GameRecord};

    fn fixture() -> (ExecutionContext, XiangqiResult) {
        let game = Game::standard(ClaimPolicy::Explicit).unwrap();
        let record = GameRecord::from_game(&game).unwrap();
        let assessment = record.no_win_assessment().clone();
        let context = ExecutionContext {
            purpose: crate::ExecutionPurpose::EngineTurn {},
            request_id: 1,
            session_version: 1,
            game: CanonicalRecord::xiangqi(record),
            limits: XIANGQI_PLAY_LIMITS,
        };
        let result = XiangqiResult::Incomplete {
            telemetry: XiangqiTelemetry {
                evaluator_id: search::EVALUATOR_ID.into(),
                search_id: search::SEARCH_ID.into(),
                perspective: Side::Red,
                score: None,
                completed_depth: 0,
                nodes: 0,
                principal_variation: vec![],
                status: XiangqiStop::Stopped,
                no_win_assessment: assessment,
            },
        };
        (context, result)
    }
    #[test]
    fn unknown_identities_and_excess_request_bounds_do_not_match() {
        let (mut context, current) = fixture();
        assert!(current.matches(&context));
        for mutate in [
            |t: &mut XiangqiTelemetry| t.evaluator_id = "mshogi.xiangqi.unknown.v1".into(),
            |t: &mut XiangqiTelemetry| t.search_id = "mshogi.xiangqi.unknown.v1".into(),
        ] {
            let mut other = current.clone();
            let XiangqiResult::Incomplete { telemetry } = &mut other else {
                unreachable!()
            };
            mutate(telemetry);
            assert_eq!(telemetry.score_label(), None);
            assert!(!other.matches(&context));
        }
        context.limits.depth = search::MAX_DEPTH + 1;
        assert!(!current.matches(&context));
    }
    #[test]
    fn score_labels_distinguish_selective_distances() {
        let (_, XiangqiResult::Incomplete { mut telemetry }) = fixture() else {
            unreachable!()
        };
        telemetry.score = Some(search::WIN_SCORE - 20);
        assert_eq!(
            telemetry.score_label().as_deref(),
            Some("Win in 20 native command edges")
        );
        telemetry.score = Some(i32::MIN);
        assert_eq!(telemetry.score_label(), None);
    }
}

/// Xiangqi front-end access to the shared timed history-prefix policy.
/// Timed searches keep the table while each request extends the previous
/// native history (same root, policy and event prefix); fixed-work searches
/// use fresh state and leave the kept state alone.
#[derive(Default)]
pub struct XiangqiSearchSession {
    slot: crate::server::Slot<search::Engine>,
    /// Table size in megabytes for timed searches; 0 means the default.
    table_mb: usize,
    last: Option<(
        mshogi_core::xiangqi::PositionRoot,
        mshogi_core::xiangqi::ClaimPolicy,
        Vec<mshogi_core::xiangqi::GameEvent>,
    )>,
}
impl XiangqiSearchSession {
    /// Session whose timed searches use a table of `megabytes`.
    pub fn with_table(megabytes: usize) -> Self {
        Self {
            table_mb: megabytes,
            ..Self::default()
        }
    }
    /// Search one native history with optional elapsed-time control.
    pub fn search(
        &mut self,
        game: &mshogi_core::xiangqi::Game,
        limits: search::Limits,
        time: Option<mshogi_search::alphabeta::timing::TimeBudget>,
        stop: impl FnMut() -> bool,
    ) -> Result<search::SearchResult, search::SearchError> {
        use mshogi_search::alphabeta::timing::RequestTime;
        let timing = RequestTime::new(time);
        if time.is_none() {
            return search::search_with_state(game, limits, None, &mut search::new_engine(), stop);
        }
        let events = game.events();
        let reset = self.last.as_ref().is_none_or(|(root, policy, previous)| {
            root != game.root()
                || *policy != game.claim_policy()
                || previous.len() > events.len()
                || previous[..] != events[..previous.len()]
        });
        self.last = Some((game.root().clone(), game.claim_policy(), events.to_vec()));
        let table_mb = self.table_mb;
        let mut engine = self
            .slot
            .acquire_incremental(reset, || {
                Ok::<_, std::convert::Infallible>(if table_mb == 0 {
                    search::new_engine()
                } else {
                    search::new_engine_sized(table_mb)
                })
            })
            .expect("infallible engine allocation");
        search::search_with_state(game, limits, timing.remaining(), &mut engine, stop)
    }
}
