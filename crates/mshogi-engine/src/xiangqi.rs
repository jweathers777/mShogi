//! Actual native Xiangqi ordinary-search results, distinct from exact proof operations.
use crate::{SharedPurpose, SharedRequest};
use mshogi_core::{
    native::CanonicalRecord,
    xiangqi::{Action, Conclusion, NoWinAssessment, PendingAdjudication, Side},
};
use mshogi_search::xiangqi as search;
use serde::{Deserialize, Serialize};

/// Measured default for ordinary interactive consumers; derived from the native search.
pub const XIANGQI_PLAY_LIMITS: crate::WorkLimits = crate::WorkLimits {
    depth: search::DEFAULT_LIMITS.depth,
    nodes: search::DEFAULT_LIMITS.nodes,
};

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
    /// Versioned material/advancement configuration.
    pub evaluator_id: String,
    /// Versioned deterministic full-width algorithm.
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
    pub(crate) fn searched(result: search::SearchResult) -> Result<Self, &'static str> {
        let search::AssessmentEvidence::Fast(assessment) = result.assessment else {
            return Err("ordinary child does not accept a different exact execution purpose");
        };
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
            no_win_assessment: assessment,
        };
        if let Some(search::Unresolved::Native(pending)) = result.unresolved {
            return Ok(Self::Unresolved { pending, telemetry });
        }
        match result.decision {
            Some(search::Decision::Choose(choice)) => Ok(Self::Choice {
                choice: choice.into(),
                telemetry,
            }),
            Some(search::Decision::Finished(conclusion)) => Ok(Self::Finished {
                conclusion,
                telemetry,
            }),
            None => Ok(Self::Incomplete { telemetry }),
            _ => Err("unexpected exact proof in ordinary search"),
        }
    }
    pub(crate) fn matches(&self, context: &SharedRequest) -> bool {
        let CanonicalRecord::Xiangqi(record) = &context.game else {
            return false;
        };
        if matches!(context.purpose, SharedPurpose::ChessFullReachability { .. }) {
            return false;
        }
        let t = match self {
            Self::Choice { telemetry, .. }
            | Self::Finished { telemetry, .. }
            | Self::Incomplete { telemetry }
            | Self::Unresolved { telemetry, .. } => telemetry,
        };
        if t.evaluator_id != search::EVALUATOR_ID
            || t.search_id != search::SEARCH_ID
            || t.perspective != record.side_to_move()
            || t.completed_depth > context.limits.depth
            || t.nodes > context.limits.nodes
            || t.principal_variation.len() > usize::from(t.completed_depth) + 1
            || t.score
                .is_some_and(|s| !(-search::WIN_SCORE..=search::WIN_SCORE).contains(&s))
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
                            Conclusion::Win { winner, .. } if *winner == t.perspective => {
                                search::WIN_SCORE
                            }
                            _ => -search::WIN_SCORE,
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
