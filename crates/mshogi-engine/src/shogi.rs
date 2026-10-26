//! Actual shogi results, distinct from chess scores and cooperative Full proofs.
use crate::{SharedPurpose, SharedRequest, StopReason};
use mshogi_core::{
    native::CanonicalRecord,
    shogi::{Action, Conclusion, Side},
};
use mshogi_search::shogi as search;
use serde::{Deserialize, Serialize};

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
    /// Fixed native full-width search configuration identity.
    pub search_id: String,
    /// Native root side, independent of final command actor or terminal winner.
    pub perspective: Side,
    /// Hundredths-of-pawn heuristic, or native terminal win distance; null before
    /// any completed work. Terminal distance includes declaration command edges.
    #[serde(deserialize_with = "required_score")]
    pub score: Option<i32>,
    /// Last complete board-depth iteration; declarations are examined at leaves.
    pub completed_depth: u16,
    /// All admitted visits, including discarded partial iterations.
    pub nodes: u64,
    /// Native continuation from completed work, possibly ending in declaration.
    pub principal_variation: Vec<ShogiChoice>,
    /// Actual completion, work exhaustion or external stop, never a game outcome.
    pub status: StopReason,
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
    pub(crate) fn matches(&self, context: &SharedRequest) -> bool {
        let CanonicalRecord::Shogi(record) = &context.game else {
            return false;
        };
        if matches!(context.purpose, SharedPurpose::ChessFullReachability { .. }) {
            return false;
        }
        let telemetry = match self {
            Self::Choice { telemetry, .. }
            | Self::Finished { telemetry, .. }
            | Self::Incomplete { telemetry } => telemetry,
        };
        if telemetry.evaluator_id != search::EVALUATOR_ID
            || telemetry.search_id != search::SEARCH_ID
            || telemetry.perspective != record.side_to_move()
            || telemetry.completed_depth > context.limits.depth
            || telemetry.nodes > context.limits.nodes
            || telemetry.principal_variation.len() > usize::from(telemetry.completed_depth) + 1
            || telemetry
                .score
                .is_some_and(|s| !(-search::WIN_SCORE..=search::WIN_SCORE).contains(&s))
        {
            return false;
        }
        match self {
            Self::Choice { choice, .. } => {
                record.conclusion().is_none()
                    && telemetry.completed_depth > 0
                    && telemetry.nodes > 0
                    && telemetry.score.is_some()
                    && telemetry.principal_variation.first() == Some(choice)
                    && match choice {
                        ShogiChoice::Declare { actor } => {
                            *actor == telemetry.perspective
                                && telemetry.score == Some(search::WIN_SCORE - 1)
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
                    && telemetry.completed_depth == 1
                    && telemetry.nodes == 1
                    && telemetry.status == StopReason::Complete
                    && telemetry.score
                        == Some(match conclusion {
                            Conclusion::Draw(_) => 0,
                            Conclusion::Win { winner, .. } => {
                                if *winner == telemetry.perspective {
                                    search::WIN_SCORE
                                } else {
                                    -search::WIN_SCORE
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
