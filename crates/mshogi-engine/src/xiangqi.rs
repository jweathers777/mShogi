//! Actual native Xiangqi ordinary-search results, distinct from exact proof operations.
use crate::observation_identity::{Identity, Profile, SearchKind};
use crate::{ExecutionContext, ExecutionPurpose};
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
/// Engine-turn limits of saved full-width observations.
pub const XIANGQI_LEGACY_PLAY_LIMITS: crate::WorkLimits = crate::WorkLimits {
    time_ms: None,
    depth: 1,
    nodes: 128,
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
    /// Versioned classical evaluation configuration.
    pub evaluator_id: String,
    /// Versioned search algorithm, including historical observations.
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
    fn identity(&self) -> Option<Identity> {
        Identity::classify(
            &self.evaluator_id,
            &self.search_id,
            Profile {
                evaluator: search::EVALUATOR_ID,
                search: search::SEARCH_ID,
                max_depth: search::MAX_DEPTH,
                max_nodes: search::MAX_NODES,
                win_score: search::WIN_SCORE,
                line_bound: search::MAX_LINE,
            },
            Profile {
                evaluator: search::LEGACY_EVALUATOR_ID,
                search: search::LEGACY_SEARCH_ID,
                max_depth: search::LEGACY_MAX_DEPTH,
                max_nodes: search::LEGACY_MAX_NODES,
                win_score: search::LEGACY_WIN_SCORE,
                line_bound: 1,
            },
        )
    }
    /// Native root-side score label with the saved search's distance scale.
    pub fn score_label(&self) -> Option<String> {
        let identity = self.identity()?;
        let bound = match identity.kind {
            SearchKind::SharedCore => mshogi_search::alphabeta::MATE_BOUND,
            SearchKind::FullWidth => {
                identity.profile.win_score - i32::from(identity.max_depth()) - 1
            }
        };
        self.score.and_then(|score| {
            let magnitude = i64::from(score).abs();
            if magnitude > i64::from(identity.profile.win_score) {
                return None;
            }
            Some(if magnitude >= i64::from(bound) {
                format!(
                    "{} in {} native command edges",
                    if score >= 0 { "Win" } else { "Loss" },
                    i64::from(identity.profile.win_score) - magnitude
                )
            } else {
                format!("{:+.2} soldiers", f64::from(score) / 100.0)
            })
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
    pub(crate) fn matches(&self, context: &ExecutionContext) -> bool {
        let CanonicalRecord::Xiangqi(record) = &context.game else {
            return false;
        };
        if matches!(
            context.purpose,
            ExecutionPurpose::ChessFullReachability { .. }
        ) {
            return false;
        }
        let t = match self {
            Self::Choice { telemetry, .. }
            | Self::Finished { telemetry, .. }
            | Self::Incomplete { telemetry }
            | Self::Unresolved { telemetry, .. } => telemetry,
        };
        let Some(identity) = t.identity() else {
            return false;
        };
        let win_score = identity.profile.win_score;
        if context.limits.depth == 0
            || context.limits.depth > identity.max_depth()
            || context.limits.nodes > identity.max_nodes()
            || t.perspective != record.side_to_move()
            || t.completed_depth > context.limits.depth
            || t.nodes > context.limits.nodes
            || t.principal_variation.len() > identity.max_line(t.completed_depth)
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
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: 1,
            session_version: 1,
            process_generation: 1,
            game: CanonicalRecord::xiangqi(record),
            limits: XIANGQI_LEGACY_PLAY_LIMITS,
        };
        let result = XiangqiResult::Incomplete {
            telemetry: XiangqiTelemetry {
                evaluator_id: search::LEGACY_EVALUATOR_ID.into(),
                search_id: search::LEGACY_SEARCH_ID.into(),
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
    fn legacy_identities_keep_bounds_and_assessment_checks() {
        let (mut context, old) = fixture();
        assert!(old.matches(&context));
        let mut current = old.clone();
        let XiangqiResult::Incomplete { telemetry } = &mut current else {
            unreachable!()
        };
        telemetry.evaluator_id = search::EVALUATOR_ID.into();
        assert!(!current.matches(&context));
        let XiangqiResult::Incomplete { telemetry } = &mut current else {
            unreachable!()
        };
        telemetry.search_id = search::SEARCH_ID.into();
        assert!(current.matches(&context));
        context.limits = XIANGQI_PLAY_LIMITS;
        assert!(!old.matches(&context));
        assert!(current.matches(&context));
    }
    #[test]
    fn saved_score_labels_distinguish_deeper_selective_distances() {
        let (_, XiangqiResult::Incomplete { mut telemetry }) = fixture() else {
            unreachable!()
        };
        telemetry.score = Some(search::LEGACY_WIN_SCORE - 3);
        assert_eq!(
            telemetry.score_label().as_deref(),
            Some("Win in 3 native command edges")
        );
        telemetry.evaluator_id = search::EVALUATOR_ID.into();
        telemetry.search_id = search::SEARCH_ID.into();
        telemetry.score = Some(search::WIN_SCORE - 20);
        assert_eq!(
            telemetry.score_label().as_deref(),
            Some("Win in 20 native command edges")
        );
        telemetry.score = Some(i32::MIN);
        assert_eq!(telemetry.score_label(), None);
    }
}
