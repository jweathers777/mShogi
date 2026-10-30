//! The public desktop controller driving actual native engine subprocesses.
//! GUI/package interaction evidence is separate; these tests use no Tauri hooks.
use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy, Color};
use mshogi_core::native::NativeSide;
use mshogi_desktop_session::{
    ChessPresentation, SessionOperation, SessionToken, SharedDesktopCommand, SharedOpenToken,
};
use mshogi_desktop_session::{DesktopController, EnginePhase, PLAY_LIMITS, SharedSnapshot};
use mshogi_engine::{ChessResult, EngineChoice, FailureKind, StopReason};
use std::{
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

fn controller(executable: &str) -> DesktopController {
    DesktopController::new(PathBuf::from(executable)).unwrap()
}
fn until(
    controller: &mut DesktopController,
    ready: impl Fn(&SharedSnapshot) -> bool,
) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let snapshot = controller.shared_snapshot();
        if ready(&snapshot) {
            return snapshot;
        }
        assert!(
            Instant::now() < deadline,
            "desktop completion deadline: phase {:?}, message {:?}",
            snapshot.phase,
            snapshot.diagnostic
        );
        thread::sleep(Duration::from_millis(1));
    }
}
fn human_turn(controller: &mut DesktopController) -> SharedSnapshot {
    until(controller, |snapshot| {
        assert_ne!(
            snapshot.phase,
            EnginePhase::Failed,
            "{:?}",
            snapshot.diagnostic
        );
        side_to_move(snapshot) == human(snapshot) && snapshot.phase == EnginePhase::Idle
    })
}
fn replay(snapshot: &SharedSnapshot) -> AdjudicatedGame {
    let mut game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    for text in &history(snapshot) {
        let action = text.parse().unwrap();
        assert!(game.position().legal_actions().contains(&action));
        game.apply(action).unwrap();
    }
    assert_eq!(game.position().to_fen(), fen(snapshot));
    assert_eq!(chess(snapshot).rules_mode, "m2");
    assert_eq!(snapshot.play_limits, PLAY_LIMITS);
    assert!(chess(snapshot).live_conclusion.is_none());
    game
}
fn real_observations(snapshot: &SharedSnapshot, perspective: Color, searched_version: u64) {
    let Some(ChessResult::Move { action, telemetry }) = &chess(snapshot).last_engine_result else {
        panic!(
            "expected an actual engine move, got {:?}",
            chess(snapshot).last_engine_result
        )
    };
    assert_eq!(chess(snapshot).last_engine_turn, Some(perspective));
    assert_eq!(chess(snapshot).searched_version, Some(searched_version));
    assert_eq!(history(snapshot).last().unwrap(), &action.to_string());
    assert_eq!(telemetry.completed_depth, PLAY_LIMITS.depth);
    assert!(telemetry.nodes > 0 && telemetry.nodes <= PLAY_LIMITS.nodes);
    assert!(telemetry.score.is_some());
    assert_eq!(telemetry.status, StopReason::Complete);
    assert_eq!(
        telemetry.principal_variation.len(),
        usize::from(telemetry.completed_depth)
    );
    assert_eq!(
        telemetry.principal_variation[0],
        EngineChoice::Move { action: *action }
    );
    // The continuation belongs to the position searched before the reply, not
    // to the newer displayed board. Every returned complete choice must replay.
    let mut before_reply = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    for text in &history(snapshot)[..history(snapshot).len() - 1] {
        before_reply.apply(text.parse().unwrap()).unwrap();
    }
    for choice in &telemetry.principal_variation {
        match choice {
            EngineChoice::Move { action } => before_reply.apply(*action).unwrap(),
            EngineChoice::Claim { claim } => before_reply.claim((*claim).into()).unwrap(),
        }
    }
}
fn shutdown(controller: &mut DesktopController) {
    controller.begin_shutdown();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !controller.shutdown_acknowledged() {
        assert!(
            Instant::now() < deadline,
            "desktop supervisor shutdown acknowledgement"
        );
        thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn human_white_plays_two_turns_against_actual_bounded_native_replies() {
    let mut controller = controller(env!("CARGO_BIN_EXE_mshogi"));
    let initial = controller.shared_snapshot();
    assert_eq!(human(&initial), Color::White);
    assert_eq!(side_to_move(&initial), Color::White);
    assert_eq!(initial.phase, EnginePhase::Idle);
    assert_eq!(initial.choices.len(), 20);
    let thinking = controller
        .settled(
            initial.token,
            SessionOperation::ChessPlay {
                action: "e2e4".parse().unwrap(),
            },
        )
        .unwrap();
    assert_eq!(thinking.phase, EnginePhase::Thinking);
    assert_eq!(history(&thinking), ["e2e4"]);
    assert_eq!(side_to_move(&thinking), Color::Black);
    let replied = human_turn(&mut controller);
    assert_eq!(history(&replied).len(), 2);
    assert!(replied.token.version > thinking.token.version);
    real_observations(&replied, Color::Black, thinking.token.version);
    replay(&replied);
    let next = replied.choices.first().unwrap().action.parse().unwrap();
    let thinking = controller
        .settled(replied.token, SessionOperation::ChessPlay { action: next })
        .unwrap();
    assert_eq!(history(&thinking).len(), 3);
    assert_eq!(&history(&thinking)[..2], &history(&replied));
    let replied = human_turn(&mut controller);
    assert_eq!(history(&replied).len(), 4);
    real_observations(&replied, Color::Black, thinking.token.version);
    replay(&replied);
    let settled = controller.shared_snapshot();
    assert_eq!(
        history(&settled),
        history(&replied),
        "completion is applied once"
    );
    assert_eq!(settled.token.version, replied.token.version);
    shutdown(&mut controller);
}

#[test]
fn human_black_gets_an_automatic_native_opening_and_can_answer_it() {
    let mut controller = controller(env!("CARGO_BIN_EXE_mshogi"));
    let initial = controller.shared_snapshot();
    let thinking = controller
        .fresh(
            SharedOpenToken {
                document: initial.token.document,
                version: initial.token.version,
                document_revision: initial.document.revision,
            },
            Color::Black,
        )
        .unwrap();
    assert_eq!(thinking.phase, EnginePhase::Thinking);
    assert_eq!(side_to_move(&thinking), Color::White);
    assert!(history(&thinking).is_empty());
    let replied = human_turn(&mut controller);
    assert_eq!(human(&replied), Color::Black);
    assert_eq!(side_to_move(&replied), Color::Black);
    assert_eq!(history(&replied).len(), 1);
    real_observations(&replied, Color::White, thinking.token.version);
    replay(&replied);
    let next = replied.choices.first().unwrap().action.parse().unwrap();
    let thinking = controller
        .settled(replied.token, SessionOperation::ChessPlay { action: next })
        .unwrap();
    assert_eq!(history(&thinking).len(), 2);
    let replied = human_turn(&mut controller);
    assert_eq!(history(&replied).len(), 3);
    real_observations(&replied, Color::White, thinking.token.version);
    replay(&replied);
    shutdown(&mut controller);
}

#[test]
fn cancel_new_game_stale_commands_and_retry_protect_the_live_game() {
    let mut controller = controller(env!("CARGO_BIN_EXE_mshogi"));
    let initial = controller.shared_snapshot();
    let thinking = controller
        .fresh(
            SharedOpenToken {
                document: initial.token.document,
                version: initial.token.version,
                document_revision: initial.document.revision,
            },
            Color::Black,
        )
        .unwrap();
    // Do not poll/accept a result before cancellation. Whether the real child
    // is queued, running, or already finished, the serialized command must win.
    thread::sleep(Duration::from_millis(20));
    let cancelled = controller
        .settled(thinking.token, SessionOperation::Cancel {})
        .unwrap();
    assert_eq!(cancelled.phase, EnginePhase::Cancelled);
    for _ in 0..10 {
        let snapshot = controller.shared_snapshot();
        assert_eq!(snapshot.phase, EnginePhase::Cancelled);
        assert!(history(&snapshot).is_empty());
        assert!(chess(&snapshot).live_conclusion.is_none());
        thread::sleep(Duration::from_millis(2));
    }
    let retried = controller
        .settled(cancelled.token, SessionOperation::Retry {})
        .unwrap();
    assert_eq!(retried.phase, EnginePhase::Thinking);
    let after_retry = human_turn(&mut controller);
    assert_eq!(history(&after_retry).len(), 1);
    real_observations(&after_retry, Color::White, retried.token.version);
    replay(&after_retry);
    let superseded = controller
        .fresh(
            SharedOpenToken {
                document: after_retry.token.document,
                version: after_retry.token.version,
                document_revision: after_retry.document.revision,
            },
            Color::Black,
        )
        .unwrap();
    assert_eq!(superseded.phase, EnginePhase::Thinking);
    let fresh = controller
        .fresh(
            SharedOpenToken {
                document: superseded.token.document,
                version: superseded.token.version,
                document_revision: superseded.document.revision,
            },
            Color::White,
        )
        .unwrap();
    assert!(
        controller
            .settled(
                superseded.token,
                SessionOperation::ChessPlay {
                    action: "e2e4".parse().unwrap(),
                }
            )
            .is_err()
    );
    for _ in 0..10 {
        let snapshot = controller.shared_snapshot();
        assert_eq!(snapshot.token.version, fresh.token.version);
        assert_eq!(side_to_move(&snapshot), Color::White);
        assert_eq!(snapshot.phase, EnginePhase::Idle);
        assert!(history(&snapshot).is_empty());
        assert!(chess(&snapshot).live_conclusion.is_none());
        thread::sleep(Duration::from_millis(2));
    }
    let thinking = controller
        .settled(
            fresh.token,
            SessionOperation::ChessPlay {
                action: "e2e4".parse().unwrap(),
            },
        )
        .unwrap();
    let replied = human_turn(&mut controller);
    assert_eq!(history(&replied).len(), 2);
    real_observations(&replied, Color::Black, thinking.token.version);
    replay(&replied);
    shutdown(&mut controller);
}

#[test]
fn actual_missing_response_and_failed_process_pause_without_a_chess_score() {
    // Separate fixture helper, excluded from packaged normal applications:
    // request1 exits successfully without output; request2 exits nonzero.
    let mut controller = controller(env!("CARGO_BIN_EXE_mshogi-internal-fixture"));
    let initial = controller.shared_snapshot();
    controller
        .fresh(
            SharedOpenToken {
                document: initial.token.document,
                version: initial.token.version,
                document_revision: initial.document.revision,
            },
            Color::Black,
        )
        .unwrap();
    let failed = until(&mut controller, |snapshot| {
        snapshot.phase == EnginePhase::Failed
    });
    assert!(
        matches!(&chess(&failed).last_engine_result, Some(ChessResult::Failure { failure }) if failure.kind == FailureKind::MissingResponse)
    );
    assert!(history(&failed).is_empty());
    assert!(chess(&failed).live_conclusion.is_none());
    assert_eq!(side_to_move(&failed), Color::White);
    let stable = controller.shared_snapshot();
    assert_eq!(
        stable.token.version, failed.token.version,
        "failure does not silently retry"
    );
    assert_eq!(stable.phase, EnginePhase::Failed);
    let retry = controller
        .settled(failed.token, SessionOperation::Retry {})
        .unwrap();
    assert_eq!(retry.phase, EnginePhase::Thinking);
    let failed = until(&mut controller, |snapshot| {
        snapshot.phase == EnginePhase::Failed
    });
    assert!(
        matches!(&chess(&failed).last_engine_result, Some(ChessResult::Failure { failure }) if failure.kind == FailureKind::UnexpectedExit)
    );
    assert!(history(&failed).is_empty());
    assert!(chess(&failed).live_conclusion.is_none());
    assert!(
        failed
            .diagnostic
            .as_ref()
            .unwrap()
            .contains("Engine failure")
    );
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
fn human(snapshot: &SharedSnapshot) -> Color {
    let NativeSide::Chess(side) = snapshot.human.id else {
        panic!("Chess human")
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
