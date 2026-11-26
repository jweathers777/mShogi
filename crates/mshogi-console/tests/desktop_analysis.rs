//! Actual subprocess analysis through the public desktop resource owner.
mod support;
use mshogi_core::chess::Color;
use mshogi_desktop_session::{AnalysisOperation, AnalysisPhase, DesktopController, EnginePhase};
use mshogi_desktop_session::{SessionOperation, SharedOpenToken};
use mshogi_engine::{ChessResult, ExecutionPurpose};
use std::{path::PathBuf, thread, time::Duration};
use support::desktop::*;

fn controller() -> DesktopController {
    DesktopController::new(PathBuf::from(env!("CARGO_BIN_EXE_mshogi"))).unwrap()
}
fn moves(depth: u16, nodes: u64) -> AnalysisOperation {
    AnalysisOperation { depth, nodes }
}

#[test]
fn actual_analysis_suggestions_never_commit_a_turn_and_resume_is_explicit() {
    let mut controller = controller();
    let initial = controller.shared_snapshot();
    let turn = controller
        .settled(initial.token, SessionOperation::play("e2e4"))
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
    let result = until(&mut controller, 30, |s| {
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
    let played = until(&mut controller, 30, |s| {
        side_to_move(s) == Color::White && s.phase == EnginePhase::Idle
    });
    assert_eq!(history(&played).len(), 2);
    assert_eq!(history(&played)[0], "e2e4");
    assert!(chess(&played).live_conclusion.is_none());
    shutdown(&mut controller);
}

#[test]
fn actual_budget_exhaustion_and_cancelled_replacement_leave_match_unchanged() {
    let mut controller = controller();
    let initial = controller.shared_snapshot();
    let started = controller
        .settled(
            initial.token,
            SessionOperation::Analyze {
                operation: moves(1, 0),
            },
        )
        .unwrap();
    let exhausted = until(&mut controller, 30, |s| {
        chess(s).analysis.status == AnalysisPhase::Incomplete
    });
    assert_eq!(fen(&exhausted), fen(&initial));
    assert!(history(&exhausted).is_empty() && chess(&exhausted).live_conclusion.is_none());
    assert!(
        matches!(
            chess(&exhausted).analysis.result,
            Some(ChessResult::Incomplete { .. })
        ),
        "a zero node budget did not remain incomplete: {:?}",
        chess(&exhausted).analysis
    );
    let old_context = chess(&exhausted).analysis.context.as_ref().unwrap();
    let replaced = controller
        .settled(
            exhausted.token,
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
    let played = until(&mut controller, 30, |s| {
        side_to_move(s) == Color::Black && s.phase == EnginePhase::Idle
    });
    assert_eq!(history(&played).len(), 1);
    assert!(chess(&played).analysis.result.is_none());
    assert!(started.token.version < played.token.version);
    shutdown(&mut controller);
}
