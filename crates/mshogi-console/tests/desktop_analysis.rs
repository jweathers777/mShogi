//! Actual subprocess analysis through the public desktop resource owner.
use mshogi_core::chess::{Color, MateReachability};
use mshogi_core::native::NativeSide;
use mshogi_desktop_session::{
    AnalysisOperation, AnalysisPhase, DesktopController, EnginePhase, SharedSnapshot,
};
use mshogi_desktop_session::{
    ChessPresentation, SessionOperation, SessionToken, SharedDesktopCommand, SharedOpenToken,
};
use mshogi_engine::{ChessResult, ExecutionPurpose};
use std::{
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

fn controller() -> DesktopController {
    DesktopController::new(PathBuf::from(env!("CARGO_BIN_EXE_mshogi"))).unwrap()
}
fn until(
    controller: &mut DesktopController,
    ready: impl Fn(&SharedSnapshot) -> bool,
) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let snapshot = controller.shared_snapshot();
        if ready(&snapshot) {
            return snapshot;
        }
        assert!(
            Instant::now() < deadline,
            "completion deadline: {snapshot:?}"
        );
        thread::sleep(Duration::from_millis(1));
    }
}
fn shutdown(controller: &mut DesktopController) {
    controller.begin_shutdown();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !controller.shutdown_acknowledged() {
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
    }
}
fn moves(depth: u16, nodes: u64) -> AnalysisOperation {
    AnalysisOperation::Moves { depth, nodes }
}

#[test]
fn actual_analysis_suggestions_never_commit_a_turn_and_resume_is_explicit() {
    let mut controller = controller();
    let initial = controller.shared_snapshot();
    let turn = controller
        .settled(
            initial.token,
            SessionOperation::ChessPlay {
                action: "e2e4".parse().unwrap(),
            },
        )
        .unwrap();
    // Analyze is serialized before any poll can apply an engine-turn reply.
    let analyzing = controller
        .settled(
            turn.token,
            SessionOperation::Analyze {
                operation: moves(2, 20_000),
            },
        )
        .unwrap();
    assert!(analyzing.paused);
    assert_eq!(history(&analyzing), ["e2e4"]);
    let result = until(&mut controller, |s| {
        chess(s).analysis.status == AnalysisPhase::Complete
    });
    assert_eq!(history(&result), history(&analyzing));
    assert_eq!(fen(&result), fen(&analyzing));
    assert!(chess(&result).live_conclusion.is_none());
    assert_eq!(side_to_move(&result), Color::Black);
    assert!(result.paused);
    let context = chess(&result).analysis.context.as_ref().unwrap();
    let mshogi_core::native::CanonicalRecord::Chess(record) = &context.game else {
        panic!("chess analysis context");
    };
    assert!(matches!(
        context.purpose,
        ExecutionPurpose::MoveAnalysis { .. }
    ));
    assert_eq!(
        record.replay_history().unwrap().game.position().to_fen(),
        fen(&result)
    );
    let Some(ChessResult::Move { action, telemetry }) = &chess(&result).analysis.result else {
        panic!(
            "expected a real move suggestion: {:?}",
            chess(&result).analysis
        )
    };
    assert_eq!(telemetry.completed_depth, 2);
    assert!(telemetry.nodes > 0 && telemetry.nodes <= 20_000);
    assert!(
        record
            .replay_history()
            .unwrap()
            .game
            .position()
            .legal_actions()
            .contains(action)
    );
    for _ in 0..10 {
        let stable = controller.shared_snapshot();
        assert_eq!(history(&stable), history(&result));
        assert_eq!(fen(&stable), fen(&result));
        assert!(stable.paused);
    }
    let resumed = controller
        .settled(result.token, SessionOperation::Resume {})
        .unwrap();
    assert!(!resumed.paused);
    let played = until(&mut controller, |s| {
        side_to_move(s) == Color::White && s.phase == EnginePhase::Idle
    });
    assert_eq!(history(&played).len(), 2);
    assert_eq!(history(&played)[0], "e2e4");
    assert!(chess(&played).live_conclusion.is_none());
    shutdown(&mut controller);
}

#[test]
fn actual_full_budget_exhaustion_and_cancelled_replacement_leave_match_unchanged() {
    let mut controller = controller();
    let initial = controller.shared_snapshot();
    let started = controller
        .settled(
            initial.token,
            SessionOperation::Analyze {
                operation: AnalysisOperation::FullReachability {
                    target: mshogi_engine::ReachabilityTarget::Either,
                    max_positions: 1,
                    max_transitions: 1,
                },
            },
        )
        .unwrap();
    let full = until(&mut controller, |s| {
        chess(s).analysis.status == AnalysisPhase::Incomplete
    });
    assert_eq!(fen(&full), fen(&initial));
    assert!(history(&full).is_empty() && chess(&full).live_conclusion.is_none());
    let Some(ChessResult::Reachability {
        result: MateReachability::Unknown { work, .. },
    }) = &chess(&full).analysis.result
    else {
        panic!(
            "bounded Full did not remain unknown: {:?}",
            chess(&full).analysis
        )
    };
    assert!(work.positions <= 1 && work.transitions <= 1);
    assert_eq!(chess(&full).rules_mode, "m2");
    let old_context = chess(&full).analysis.context.as_ref().unwrap();
    let replaced = controller
        .settled(
            full.token,
            SessionOperation::Analyze {
                operation: moves(8, 5_000_000),
            },
        )
        .unwrap();
    assert_eq!(chess(&replaced).analysis.status, AnalysisPhase::Thinking);
    assert!(chess(&replaced).analysis.result.is_none());
    assert!(
        chess(&replaced)
            .analysis
            .context
            .as_ref()
            .unwrap()
            .request_id
            > old_context.request_id
    );
    assert!(
        chess(&replaced)
            .analysis
            .context
            .as_ref()
            .unwrap()
            .process_generation
            > old_context.process_generation
    );
    let cancelled = controller
        .settled(replaced.token, SessionOperation::Cancel {})
        .unwrap();
    assert_eq!(chess(&cancelled).analysis.status, AnalysisPhase::Cancelled);
    assert!(
        chess(&cancelled).analysis.result.is_none(),
        "cancel cannot invent final work observations"
    );
    assert!(cancelled.paused);
    for _ in 0..10 {
        let snapshot = controller.shared_snapshot();
        assert_eq!(fen(&snapshot), fen(&initial));
        assert_eq!(chess(&snapshot).analysis.status, AnalysisPhase::Cancelled);
        assert!(history(&snapshot).is_empty() && chess(&snapshot).live_conclusion.is_none());
        thread::sleep(Duration::from_millis(2));
    }
    let fresh = controller
        .fresh(
            SharedOpenToken {
                document: cancelled.token.document,
                version: cancelled.token.version,
                document_revision: cancelled.document.revision,
            },
            Color::Black,
        )
        .unwrap();
    assert!(!fresh.paused);
    assert!(chess(&fresh).analysis.result.is_none());
    let played = until(&mut controller, |s| {
        side_to_move(s) == Color::Black && s.phase == EnginePhase::Idle
    });
    assert_eq!(history(&played).len(), 1);
    assert!(chess(&played).analysis.result.is_none());
    assert!(started.token.version < played.token.version);
    shutdown(&mut controller);
}

fn chess(snapshot: &SharedSnapshot) -> &ChessPresentation {
    snapshot.chess.as_ref().expect("native Chess capabilities")
}
fn history(snapshot: &SharedSnapshot) -> Vec<String> {
    live_game(snapshot)
        .history()
        .actions()
        .map(|a| a.to_string())
        .collect()
}
fn fen(snapshot: &SharedSnapshot) -> String {
    live_game(snapshot).position().to_fen()
}
fn side_to_move(snapshot: &SharedSnapshot) -> Color {
    let NativeSide::Chess(side) = snapshot.turn.id else {
        panic!("Chess turn")
    };
    side
}
trait SettledCommand {
    fn settled(
        &mut self,
        expected: SessionToken,
        operation: SessionOperation,
    ) -> Result<SharedSnapshot, String>;
    fn fresh(&mut self, expected: SharedOpenToken, human: Color) -> Result<SharedSnapshot, String>;
}
fn complete(
    controller: &mut DesktopController,
    mut snapshot: SharedSnapshot,
    receipt: mshogi_desktop_session::AdmissionReceipt,
) -> Result<SharedSnapshot, String> {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if let Some(result) = receipt.completion() {
            result?;
            return Ok(snapshot);
        }
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
        snapshot = controller.shared_snapshot();
    }
}
impl SettledCommand for DesktopController {
    fn settled(
        &mut self,
        expected: SessionToken,
        operation: SessionOperation,
    ) -> Result<SharedSnapshot, String> {
        let (snapshot, receipt) = self.shared_command_with_receipt(SharedDesktopCommand {
            expected,
            game: mshogi_desktop_session::GameKind::Chess,
            operation,
        })?;
        complete(self, snapshot, receipt)
    }
    fn fresh(&mut self, expected: SharedOpenToken, human: Color) -> Result<SharedSnapshot, String> {
        let (snapshot, receipt) = self.shared_new_game(
            expected,
            mshogi_desktop_session::GameKind::Chess,
            NativeSide::Chess(human),
        )?;
        complete(self, snapshot, receipt)
    }
}

fn live_game(snapshot: &SharedSnapshot) -> mshogi_core::chess::AdjudicatedGame {
    snapshot
        .chess_match
        .as_ref()
        .unwrap()
        .replay()
        .unwrap()
        .game()
        .clone()
}
