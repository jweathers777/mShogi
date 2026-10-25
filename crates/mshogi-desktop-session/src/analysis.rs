//! Read-only analysis acceptance and single-resource play ownership.
use super::*;
use mshogi_engine::{
    AnalysisTarget, FULL_SENTINEL, FullLimits, MAX_ANALYSIS_DEPTH, MAX_ANALYSIS_NODES,
    MAX_FULL_POSITIONS, MAX_FULL_TRANSITIONS, ReachabilityTarget, RequestPurpose,
};
/// Explicit user-selected deterministic analysis budgets.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AnalysisOperation {
    /// Ordinary minimax; evaluation cannot establish a chess result.
    Moves {
        /// Maximum plies.
        depth: u16,
        /// Maximum attempted nodes.
        nodes: u64,
    },
    /// Advanced cooperative Article-3 reachability, independent of minimax.
    FullReachability {
        /// Which side may deliver mate.
        target: ReachabilityTarget,
        /// Maximum retained positions.
        max_positions: u64,
        /// Maximum examined transitions.
        max_transitions: u64,
    },
}
impl AnalysisOperation {
    fn validate(self) -> Result<(), String> {
        let valid = match self {
            Self::Moves { depth, nodes } => {
                (1..=MAX_ANALYSIS_DEPTH).contains(&depth) && nodes <= MAX_ANALYSIS_NODES
            }
            Self::FullReachability {
                max_positions,
                max_transitions,
                ..
            } => max_positions <= MAX_FULL_POSITIONS && max_transitions <= MAX_FULL_TRANSITIONS,
        };
        if valid {
            Ok(())
        } else {
            Err("Unsupported analysis limits. Moves require depth 1–8 and at most 5,000,000 nodes; Full reachability allows at most 100,000 positions and 1,000,000 transitions.".into())
        }
    }
}
/// Analysis observations are separate from live engine-turn decisions.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisPhase {
    /// No analysis selected.
    #[default]
    Idle,
    /// Supervised analysis request outstanding.
    Thinking,
    /// Completed minimax iteration or established cooperative proof.
    Complete,
    /// Observed work limit; no missing work counts are fabricated.
    Incomplete,
    /// Cancelled; no final observations were obtained.
    Cancelled,
    /// Process/protocol/search failure, separate from proof incompleteness.
    Failed,
}
/// Exact target, configuration and final real observations for inspection.
#[derive(Clone, Debug, Default, Serialize)]
pub struct AnalysisState {
    /// Analysis lifecycle, independent of chess conclusion.
    pub status: AnalysisPhase,
    /// Native target provenance, currently live position only.
    pub target: Option<AnalysisTarget>,
    /// Explicit requested operation and budgets.
    pub operation: Option<AnalysisOperation>,
    /// Exact context retained for technical details and later historical saving.
    pub context: Option<EngineRequest>,
    /// Actual final child observations; absent on cancelled/unobserved work.
    pub result: Option<EngineResult>,
    /// Plain execution diagnostic, never a chess result.
    pub message: Option<String>,
}
impl DesktopSession {
    pub(super) fn start_analysis(&mut self, operation: AnalysisOperation) -> Result<(), String> {
        operation.validate()?;
        self.pending = None;
        self.play_paused = true;
        self.engine.status = EnginePhase::Paused;
        self.engine.message =
            Some("Play paused for analysis. Resume play explicitly to continue.".into());
        self.changed();
        self.next_request += 1;
        let target = AnalysisTarget {
            selection_id: self.next_request,
            ply: self.game.game().history().ply_count() as u64,
            session_version: self.version,
            label: format!(
                "Live position after {} plies",
                self.game.game().history().ply_count()
            ),
            turn: self.game.game().position().side_to_move(),
        };
        let (purpose, limits) = match operation {
            AnalysisOperation::Moves { depth, nodes } => (
                RequestPurpose::MoveAnalysis {
                    target: target.clone(),
                },
                WorkLimits { depth, nodes },
            ),
            AnalysisOperation::FullReachability {
                target: mate,
                max_positions,
                max_transitions,
            } => (
                RequestPurpose::FullReachability {
                    target: target.clone(),
                    mate,
                    limits: FullLimits {
                        max_positions,
                        max_transitions,
                    },
                },
                FULL_SENTINEL,
            ),
        };
        let request = EngineRequest {
            protocol: mshogi_engine::PROTOCOL_VERSION,
            purpose,
            request_id: self.next_request,
            session_version: self.version,
            process_generation: self.next_request,
            game: self.game.game().record(),
            limits,
        };
        self.analysis = AnalysisState {
            status: AnalysisPhase::Thinking,
            target: Some(target),
            operation: Some(operation),
            context: Some(request.clone()),
            result: None,
            message: None,
        };
        self.analysis_pending = Some(request);
        self.analysis_submitted = false;
        Ok(())
    }
    /// Retrieve one analysis submission; subsequent calls cannot duplicate it.
    pub fn analysis_request(&mut self) -> Option<EngineRequest> {
        if self.analysis_submitted {
            return None;
        }
        let request = self.analysis_pending.clone()?;
        self.analysis_submitted = true;
        Some(request)
    }
    /// Accept read-only observations. This never applies a board move or claim.
    pub fn accept_analysis(&mut self, response: EngineResponse) -> bool {
        if self.analysis_pending.as_ref() != Some(&response.context)
            || !self.play_paused
            || response.context.session_version != self.version
            || response.context.game != self.game.game().record()
            || matches!(response.context.purpose, RequestPurpose::EngineTurn)
        {
            return false;
        }
        if self.revision >= MAX_SAFE_ID {
            return false;
        }
        self.analysis_pending = None;
        let full = matches!(
            response.context.purpose,
            RequestPurpose::FullReachability { .. }
        );
        let (status, message) = match &response.result {
            EngineResult::Failure { failure } => (AnalysisPhase::Failed, Some(format!("Analysis failure ({:?}): {}", failure.kind, failure.message))),
            EngineResult::Reachability { result } if full => (if matches!(result, MateReachability::Unknown { .. }) { AnalysisPhase::Incomplete } else { AnalysisPhase::Complete }, None),
            EngineResult::Incomplete { .. } if !full => (AnalysisPhase::Incomplete, None),
            EngineResult::Move { telemetry, .. } | EngineResult::Claim { telemetry, .. } | EngineResult::Finished { telemetry, .. } if !full => (if telemetry.status == mshogi_engine::StopReason::Complete { AnalysisPhase::Complete } else { AnalysisPhase::Incomplete }, None),
            _ => (AnalysisPhase::Failed, Some("Analysis returned a result for the wrong operation. No game action was applied.".into())),
        };
        self.analysis.status = status;
        self.analysis.message = message;
        self.analysis.result = Some(response.result);
        self.revision += 1;
        true
    }
    pub(super) fn retire_analysis(&mut self) {
        self.analysis_pending = None;
        self.analysis_submitted = false;
        self.analysis = AnalysisState::default();
    }
    pub(super) fn cancel_analysis(&mut self) -> Result<(), String> {
        if self.analysis_pending.is_none() || self.analysis.status != AnalysisPhase::Thinking {
            return Err("There is no active analysis to cancel.".into());
        }
        self.analysis_pending = None;
        self.analysis.status = AnalysisPhase::Cancelled;
        self.analysis.result = None;
        self.analysis.message = Some(
            "Analysis cancelled before final observations were received. Play remains paused."
                .into(),
        );
        self.changed();
        Ok(())
    }
    pub(super) fn resume(&mut self) -> Result<(), String> {
        if !self.play_paused {
            return Err("There is no paused play or analysis to finish.".into());
        }
        self.retire_analysis();
        self.pending = None;
        self.play_paused = false;
        self.engine.status = EnginePhase::Idle;
        self.engine.message = None;
        self.changed();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mshogi_engine::{EngineChoice, StopReason, Telemetry};
    fn move_result(context: EngineRequest) -> EngineResponse {
        let action = "e2e4".parse().unwrap();
        EngineResponse {
            context,
            result: EngineResult::Move {
                action,
                telemetry: Telemetry {
                    score: Some(0),
                    completed_depth: 1,
                    nodes: 21,
                    principal_variation: vec![EngineChoice::Move { action }],
                    status: StopReason::Complete,
                },
            },
        }
    }
    fn analyze(session: &mut DesktopSession) -> EngineRequest {
        session
            .command(
                DesktopCommand::Analyze {
                    version: session.version,
                    operation: AnalysisOperation::Moves {
                        depth: 1,
                        nodes: 1000,
                    },
                },
                1,
            )
            .unwrap();
        session.analysis_request().unwrap()
    }
    #[test]
    fn analysis_has_a_read_only_acceptance_path_and_explicit_resume() {
        let mut session = DesktopSession::new();
        let before_match = session.game.record().to_json();
        let before_game = session.game.game().record();
        let request = analyze(&mut session);
        assert!(session.engine_request().is_none());
        assert!(!session.accept(move_result(request.clone()), 2));
        assert!(session.accept_analysis(move_result(request.clone())));
        assert!(!session.accept_analysis(move_result(request)));
        assert_eq!(session.analysis.status, AnalysisPhase::Complete);
        assert!(session.play_paused);
        assert_eq!(session.game.record().to_json(), before_match);
        assert_eq!(session.game.game().record(), before_game);
        assert!(
            session
                .command(
                    DesktopCommand::Play {
                        version: session.version,
                        action: "e2e4".parse().unwrap()
                    },
                    2
                )
                .is_err()
        );
        session
            .command(
                DesktopCommand::Resume {
                    version: session.version,
                },
                3,
            )
            .unwrap();
        assert!(!session.play_paused);
        session
            .command(
                DesktopCommand::Play {
                    version: session.version,
                    action: "e2e4".parse().unwrap(),
                },
                4,
            )
            .unwrap();
        assert!(matches!(
            session.engine_request().unwrap().purpose,
            RequestPurpose::EngineTurn
        ));
    }
    #[test]
    fn mixed_turn_analysis_replacement_and_resume_share_one_owner() {
        let mut session = DesktopSession::new();
        session
            .command(
                DesktopCommand::NewGame {
                    version: 1,
                    human: Color::Black,
                },
                0,
            )
            .unwrap();
        let turn = session.engine_request().unwrap();
        let old_analysis = analyze(&mut session);
        assert!(!session.accept(move_result(turn.clone()), 2));
        session
            .command(
                DesktopCommand::Analyze {
                    version: session.version,
                    operation: AnalysisOperation::FullReachability {
                        target: ReachabilityTarget::Either,
                        max_positions: 0,
                        max_transitions: 0,
                    },
                },
                3,
            )
            .unwrap();
        let full = session.analysis_request().unwrap();
        assert!(full.process_generation > old_analysis.process_generation);
        assert!(!session.accept_analysis(move_result(old_analysis)));
        let before = session.game.record().to_json();
        assert!(session.accept_analysis(EngineResponse {
            context: full.clone(),
            result: EngineResult::Reachability {
                result: MateReachability::Unknown {
                    reason: ReachabilityStop::PositionLimit,
                    work: ReachabilityWork::default()
                }
            }
        }));
        assert_eq!(session.analysis.status, AnalysisPhase::Incomplete);
        assert_eq!(session.game.record().to_json(), before);
        session
            .command(
                DesktopCommand::Resume {
                    version: session.version,
                },
                4,
            )
            .unwrap();
        let resumed = session.engine_request().unwrap();
        assert!(resumed.request_id > full.request_id);
        assert!(!session.accept_analysis(move_result(resumed.clone())));
        assert!(!session.accept(move_result(turn), 5));
        assert!(session.accept(move_result(resumed), 6));
        assert_eq!(session.snapshot().history, ["e2e4"]);
    }
    #[test]
    fn cancel_keeps_pause_and_has_no_fabricated_final_observations() {
        let mut session = DesktopSession::new();
        let request = analyze(&mut session);
        session
            .command(
                DesktopCommand::CancelAnalysis {
                    version: session.version,
                },
                2,
            )
            .unwrap();
        assert!(session.analysis.result.is_none());
        assert_eq!(session.analysis.status, AnalysisPhase::Cancelled);
        assert!(session.play_paused);
        assert!(session.engine_request().is_none());
        assert!(!session.accept_analysis(move_result(request)));
        let before = session.version;
        assert!(
            session
                .command(
                    DesktopCommand::Analyze {
                        version: before,
                        operation: AnalysisOperation::Moves { depth: 9, nodes: 1 }
                    },
                    3
                )
                .is_err()
        );
        assert_eq!(session.version, before);
        session
            .command(
                DesktopCommand::NewGame {
                    version: before,
                    human: Color::White,
                },
                4,
            )
            .unwrap();
        assert!(!session.play_paused);
        assert_eq!(session.analysis.status, AnalysisPhase::Idle);
    }
    #[test]
    fn same_fen_different_history_and_wrong_purpose_are_rejected() {
        let mut live = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        for action in ["g1f3", "g8f6", "f3g1", "f6g8"] {
            live.apply(action.parse().unwrap()).unwrap();
        }
        let imported =
            AdjudicatedGame::from_fen(&live.position().to_fen(), ClaimPolicy::ExplicitOnly)
                .unwrap();
        let mut session = DesktopSession::from_record(&live.record(), Color::White).unwrap();
        let request = analyze(&mut session);
        let mut wrong = request.clone();
        wrong.game = imported.record();
        assert!(!session.accept_analysis(move_result(wrong)));
        let mut wrong = request.clone();
        wrong.purpose = RequestPurpose::EngineTurn;
        assert!(!session.accept_analysis(move_result(wrong)));
        assert!(session.accept_analysis(move_result(request)));
        assert_eq!(session.snapshot().history.len(), 4);
    }
    #[test]
    fn resignation_retires_analysis_and_concluded_positions_remain_analyzable() {
        let mut session = DesktopSession::new();
        let request = analyze(&mut session);
        session
            .command(
                DesktopCommand::Resign {
                    version: session.version,
                },
                2,
            )
            .unwrap();
        let ended_record = session.game.record().to_json();
        let conclusion = session.snapshot().conclusion;
        assert!(
            !session.play_paused,
            "resignation retires the paused analysis owner"
        );
        assert_eq!(session.analysis.status, AnalysisPhase::Idle);
        assert!(!session.accept_analysis(move_result(request)));
        assert_eq!(session.game.record().to_json(), ended_record);
        assert_eq!(session.snapshot().conclusion, conclusion);
        assert!(conclusion.is_some());
        let request = analyze(&mut session);
        assert!(session.accept_analysis(move_result(request)));
        assert_eq!(session.snapshot().conclusion, conclusion);
        let record = session.game.record().to_json();
        session
            .command(
                DesktopCommand::Resume {
                    version: session.version,
                },
                3,
            )
            .unwrap();
        assert!(!session.play_paused);
        assert_eq!(session.snapshot().conclusion, conclusion);
        assert_eq!(session.game.record().to_json(), record);
        assert!(session.engine_request().is_none());
    }
    #[test]
    fn analysis_claims_and_failures_are_observations_only_with_exact_context_guards() {
        let game =
            AdjudicatedGame::from_fen("7k/8/8/8/8/8/R7/K7 w - - 99 1", ClaimPolicy::ExplicitOnly)
                .unwrap();
        let mut session = DesktopSession::from_record(&game.record(), Color::White).unwrap();
        let claim = session
            .snapshot()
            .claims
            .into_iter()
            .find(|claim| claim.intended.is_some())
            .unwrap();
        let request = analyze(&mut session);
        let before = session.game.record().to_json();
        for field in 0..4 {
            let mut wrong = request.clone();
            match field {
                0 => wrong.request_id += 1,
                1 => wrong.process_generation += 1,
                2 => wrong.limits.nodes += 1,
                _ => {
                    if let RequestPurpose::MoveAnalysis { target } = &mut wrong.purpose {
                        target.selection_id += 1;
                    }
                }
            }
            assert!(!session.accept_analysis(move_result(wrong)));
        }
        assert!(session.accept_analysis(EngineResponse {
            context: request,
            result: EngineResult::Claim {
                claim,
                telemetry: Telemetry {
                    score: Some(0),
                    completed_depth: 1,
                    nodes: 1,
                    principal_variation: vec![EngineChoice::Claim { claim }],
                    status: StopReason::Complete
                }
            }
        }));
        assert!(session.snapshot().conclusion.is_none());
        assert!(session.snapshot().history.is_empty());
        assert_eq!(session.game.record().to_json(), before);
        let request = analyze(&mut session);
        assert!(session.accept_analysis(EngineResponse {
            context: request,
            result: EngineResult::Failure {
                failure: mshogi_engine::EngineFailure {
                    kind: mshogi_engine::FailureKind::Timeout,
                    message: "test process watchdog; no final observations".into()
                }
            }
        }));
        assert_eq!(session.analysis.status, AnalysisPhase::Failed);
        assert!(session.play_paused);
        assert_eq!(session.game.record().to_json(), before);
        assert!(matches!(
            session.analysis.result,
            Some(EngineResult::Failure { .. })
        ));
    }
}
