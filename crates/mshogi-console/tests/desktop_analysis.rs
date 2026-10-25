//! Actual subprocess analysis through the public desktop resource owner.
use mshogi_core::chess::{Color, MateReachability};
use mshogi_desktop_session::{
    AnalysisOperation, AnalysisPhase, DesktopCommand, DesktopController, EnginePhase, Snapshot,
};
use mshogi_engine::{EngineResult, RequestPurpose};
use std::{
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

fn controller() -> DesktopController {
    DesktopController::new(PathBuf::from(env!("CARGO_BIN_EXE_mshogi"))).unwrap()
}
fn until(controller: &mut DesktopController, ready: impl Fn(&Snapshot) -> bool) -> Snapshot {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let snapshot = controller.snapshot();
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
    let initial = controller.snapshot();
    let turn = controller
        .command(DesktopCommand::Play {
            version: initial.version,
            action: "e2e4".parse().unwrap(),
        })
        .unwrap();
    // Analyze is serialized before any poll can apply an engine-turn reply.
    let analyzing = controller
        .command(DesktopCommand::Analyze {
            version: turn.version,
            operation: moves(2, 20_000),
        })
        .unwrap();
    assert!(analyzing.play_paused);
    assert_eq!(analyzing.history, ["e2e4"]);
    let result = until(&mut controller, |s| {
        s.analysis.status == AnalysisPhase::Complete
    });
    assert_eq!(result.history, analyzing.history);
    assert_eq!(result.fen, analyzing.fen);
    assert!(result.conclusion.is_none());
    assert_eq!(result.turn, Color::Black);
    assert!(result.play_paused);
    let context = result.analysis.context.as_ref().unwrap();
    assert!(matches!(
        context.purpose,
        RequestPurpose::MoveAnalysis { .. }
    ));
    assert_eq!(
        context
            .game
            .replay_history()
            .unwrap()
            .game
            .position()
            .to_fen(),
        result.fen
    );
    let Some(EngineResult::Move { action, telemetry }) = &result.analysis.result else {
        panic!("expected a real move suggestion: {:?}", result.analysis)
    };
    assert_eq!(telemetry.completed_depth, 2);
    assert!(telemetry.nodes > 0 && telemetry.nodes <= 20_000);
    assert!(
        context
            .game
            .replay_history()
            .unwrap()
            .game
            .position()
            .legal_actions()
            .contains(action)
    );
    for _ in 0..10 {
        let stable = controller.snapshot();
        assert_eq!(stable.history, result.history);
        assert_eq!(stable.fen, result.fen);
        assert!(stable.play_paused);
    }
    let resumed = controller
        .command(DesktopCommand::Resume {
            version: result.version,
        })
        .unwrap();
    assert!(!resumed.play_paused);
    let played = until(&mut controller, |s| {
        s.turn == Color::White && s.engine.status == EnginePhase::Idle
    });
    assert_eq!(played.history.len(), 2);
    assert_eq!(played.history[0], "e2e4");
    assert!(played.conclusion.is_none());
    shutdown(&mut controller);
}

#[test]
fn actual_full_budget_exhaustion_and_cancelled_replacement_leave_match_unchanged() {
    let mut controller = controller();
    let initial = controller.snapshot();
    let started = controller
        .command(DesktopCommand::Analyze {
            version: initial.version,
            operation: AnalysisOperation::FullReachability {
                target: mshogi_engine::ReachabilityTarget::Either,
                max_positions: 1,
                max_transitions: 1,
            },
        })
        .unwrap();
    let full = until(&mut controller, |s| {
        s.analysis.status == AnalysisPhase::Incomplete
    });
    assert_eq!(full.fen, initial.fen);
    assert!(full.history.is_empty() && full.conclusion.is_none());
    let Some(EngineResult::Reachability {
        result: MateReachability::Unknown { work, .. },
    }) = &full.analysis.result
    else {
        panic!("bounded Full did not remain unknown: {:?}", full.analysis)
    };
    assert!(work.positions <= 1 && work.transitions <= 1);
    assert_eq!(full.rules_mode, "m2");
    let old_context = full.analysis.context.as_ref().unwrap();
    let replaced = controller
        .command(DesktopCommand::Analyze {
            version: full.version,
            operation: moves(8, 5_000_000),
        })
        .unwrap();
    assert_eq!(replaced.analysis.status, AnalysisPhase::Thinking);
    assert!(replaced.analysis.result.is_none());
    assert!(replaced.analysis.context.as_ref().unwrap().request_id > old_context.request_id);
    assert!(
        replaced
            .analysis
            .context
            .as_ref()
            .unwrap()
            .process_generation
            > old_context.process_generation
    );
    let cancelled = controller
        .command(DesktopCommand::CancelAnalysis {
            version: replaced.version,
        })
        .unwrap();
    assert_eq!(cancelled.analysis.status, AnalysisPhase::Cancelled);
    assert!(
        cancelled.analysis.result.is_none(),
        "cancel cannot invent final work observations"
    );
    assert!(cancelled.play_paused);
    for _ in 0..10 {
        let snapshot = controller.snapshot();
        assert_eq!(snapshot.fen, initial.fen);
        assert_eq!(snapshot.analysis.status, AnalysisPhase::Cancelled);
        assert!(snapshot.history.is_empty() && snapshot.conclusion.is_none());
        thread::sleep(Duration::from_millis(2));
    }
    let fresh = controller
        .command(DesktopCommand::NewGame {
            version: cancelled.version,
            human: Color::Black,
        })
        .unwrap();
    assert!(!fresh.play_paused);
    assert!(fresh.analysis.result.is_none());
    let played = until(&mut controller, |s| {
        s.turn == Color::Black && s.engine.status == EnginePhase::Idle
    });
    assert_eq!(played.history.len(), 1);
    assert!(played.analysis.result.is_none());
    assert!(started.version < played.version);
    shutdown(&mut controller);
}
