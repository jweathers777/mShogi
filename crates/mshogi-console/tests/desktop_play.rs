//! The public desktop controller driving actual native engine subprocesses.
//! GUI/package interaction evidence is separate; these tests use no Tauri hooks.
use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy, Color};
use mshogi_desktop_session::{
    DesktopCommand, DesktopController, EnginePhase, PLAY_LIMITS, Snapshot,
};
use mshogi_engine::{ChessResult, EngineChoice, FailureKind, StopReason};
use std::{
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

fn controller(executable: &str) -> DesktopController {
    DesktopController::new(PathBuf::from(executable)).unwrap()
}
fn until(controller: &mut DesktopController, ready: impl Fn(&Snapshot) -> bool) -> Snapshot {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let snapshot = controller
            .shared_snapshot()
            .chess
            .expect("native Chess fixture");
        if ready(&snapshot) {
            return snapshot;
        }
        assert!(
            Instant::now() < deadline,
            "desktop completion deadline: phase {:?}, message {:?}",
            snapshot.engine.status,
            snapshot.engine.message
        );
        thread::sleep(Duration::from_millis(1));
    }
}
fn human_turn(controller: &mut DesktopController) -> Snapshot {
    until(controller, |snapshot| {
        assert_ne!(
            snapshot.engine.status,
            EnginePhase::Failed,
            "{:?}",
            snapshot.engine.message
        );
        snapshot.turn == snapshot.human && snapshot.engine.status == EnginePhase::Idle
    })
}
fn replay(snapshot: &Snapshot) -> AdjudicatedGame {
    let mut game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    for text in &snapshot.history {
        let action = text.parse().unwrap();
        assert!(game.position().legal_actions().contains(&action));
        game.apply(action).unwrap();
    }
    assert_eq!(game.position().to_fen(), snapshot.fen);
    assert_eq!(snapshot.rules_mode, "m2");
    assert_eq!(snapshot.limits, PLAY_LIMITS);
    assert!(snapshot.conclusion.is_none());
    game
}
fn real_observations(snapshot: &Snapshot, perspective: Color, searched_version: u64) {
    let Some(ChessResult::Move { action, telemetry }) = &snapshot.engine.last else {
        panic!(
            "expected an actual engine move, got {:?}",
            snapshot.engine.last
        )
    };
    assert_eq!(snapshot.engine.last_turn, Some(perspective));
    assert_eq!(snapshot.engine.searched_version, Some(searched_version));
    assert_eq!(snapshot.history.last().unwrap(), &action.to_string());
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
    for text in &snapshot.history[..snapshot.history.len() - 1] {
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
    let initial = controller
        .shared_snapshot()
        .chess
        .expect("native Chess fixture");
    assert_eq!(initial.human, Color::White);
    assert_eq!(initial.turn, Color::White);
    assert_eq!(initial.engine.status, EnginePhase::Idle);
    assert_eq!(initial.legal_choices.len(), 20);
    let thinking = controller
        .settled(DesktopCommand::Play {
            version: initial.version,
            action: "e2e4".parse().unwrap(),
        })
        .unwrap();
    assert_eq!(thinking.engine.status, EnginePhase::Thinking);
    assert_eq!(thinking.history, ["e2e4"]);
    assert_eq!(thinking.turn, Color::Black);
    let replied = human_turn(&mut controller);
    assert_eq!(replied.history.len(), 2);
    assert!(replied.version > thinking.version);
    real_observations(&replied, Color::Black, thinking.version);
    replay(&replied);
    let next = replied.legal_choices.first().unwrap().action;
    let thinking = controller
        .settled(DesktopCommand::Play {
            version: replied.version,
            action: next,
        })
        .unwrap();
    assert_eq!(thinking.history.len(), 3);
    assert_eq!(&thinking.history[..2], &replied.history);
    let replied = human_turn(&mut controller);
    assert_eq!(replied.history.len(), 4);
    real_observations(&replied, Color::Black, thinking.version);
    replay(&replied);
    let settled = controller
        .shared_snapshot()
        .chess
        .expect("native Chess fixture");
    assert_eq!(
        settled.history, replied.history,
        "completion is applied once"
    );
    assert_eq!(settled.version, replied.version);
    shutdown(&mut controller);
}

#[test]
fn human_black_gets_an_automatic_native_opening_and_can_answer_it() {
    let mut controller = controller(env!("CARGO_BIN_EXE_mshogi"));
    let initial = controller
        .shared_snapshot()
        .chess
        .expect("native Chess fixture");
    let thinking = controller
        .settled(DesktopCommand::NewGame {
            version: initial.version,
            document_revision: initial.document.revision,
            human: Color::Black,
        })
        .unwrap();
    assert_eq!(thinking.engine.status, EnginePhase::Thinking);
    assert_eq!(thinking.turn, Color::White);
    assert!(thinking.history.is_empty());
    let replied = human_turn(&mut controller);
    assert_eq!(replied.human, Color::Black);
    assert_eq!(replied.turn, Color::Black);
    assert_eq!(replied.history.len(), 1);
    real_observations(&replied, Color::White, thinking.version);
    replay(&replied);
    let next = replied.legal_choices.first().unwrap().action;
    let thinking = controller
        .settled(DesktopCommand::Play {
            version: replied.version,
            action: next,
        })
        .unwrap();
    assert_eq!(thinking.history.len(), 2);
    let replied = human_turn(&mut controller);
    assert_eq!(replied.history.len(), 3);
    real_observations(&replied, Color::White, thinking.version);
    replay(&replied);
    shutdown(&mut controller);
}

#[test]
fn cancel_new_game_stale_commands_and_retry_protect_the_live_game() {
    let mut controller = controller(env!("CARGO_BIN_EXE_mshogi"));
    let initial = controller
        .shared_snapshot()
        .chess
        .expect("native Chess fixture");
    let thinking = controller
        .settled(DesktopCommand::NewGame {
            version: initial.version,
            document_revision: initial.document.revision,
            human: Color::Black,
        })
        .unwrap();
    // Do not poll/accept a result before cancellation. Whether the real child
    // is queued, running, or already finished, the serialized command must win.
    thread::sleep(Duration::from_millis(20));
    let cancelled = controller
        .settled(DesktopCommand::Cancel {
            version: thinking.version,
        })
        .unwrap();
    assert_eq!(cancelled.engine.status, EnginePhase::Cancelled);
    for _ in 0..10 {
        let snapshot = controller
            .shared_snapshot()
            .chess
            .expect("native Chess fixture");
        assert_eq!(snapshot.engine.status, EnginePhase::Cancelled);
        assert!(snapshot.history.is_empty());
        assert!(snapshot.conclusion.is_none());
        thread::sleep(Duration::from_millis(2));
    }
    let retried = controller
        .settled(DesktopCommand::Retry {
            version: cancelled.version,
        })
        .unwrap();
    assert_eq!(retried.engine.status, EnginePhase::Thinking);
    let after_retry = human_turn(&mut controller);
    assert_eq!(after_retry.history.len(), 1);
    real_observations(&after_retry, Color::White, retried.version);
    replay(&after_retry);
    let superseded = controller
        .settled(DesktopCommand::NewGame {
            version: after_retry.version,
            document_revision: after_retry.document.revision,
            human: Color::Black,
        })
        .unwrap();
    assert_eq!(superseded.engine.status, EnginePhase::Thinking);
    let fresh = controller
        .settled(DesktopCommand::NewGame {
            version: superseded.version,
            document_revision: superseded.document.revision,
            human: Color::White,
        })
        .unwrap();
    assert!(
        controller
            .settled(DesktopCommand::Play {
                version: superseded.version,
                action: "e2e4".parse().unwrap(),
            })
            .is_err()
    );
    for _ in 0..10 {
        let snapshot = controller
            .shared_snapshot()
            .chess
            .expect("native Chess fixture");
        assert_eq!(snapshot.version, fresh.version);
        assert_eq!(snapshot.turn, Color::White);
        assert_eq!(snapshot.engine.status, EnginePhase::Idle);
        assert!(snapshot.history.is_empty());
        assert!(snapshot.conclusion.is_none());
        thread::sleep(Duration::from_millis(2));
    }
    let thinking = controller
        .settled(DesktopCommand::Play {
            version: fresh.version,
            action: "e2e4".parse().unwrap(),
        })
        .unwrap();
    let replied = human_turn(&mut controller);
    assert_eq!(replied.history.len(), 2);
    real_observations(&replied, Color::Black, thinking.version);
    replay(&replied);
    shutdown(&mut controller);
}

#[test]
fn actual_missing_response_and_failed_process_pause_without_a_chess_score() {
    // Separate fixture helper, excluded from packaged normal applications:
    // request1 exits successfully without output; request2 exits nonzero.
    let mut controller = controller(env!("CARGO_BIN_EXE_mshogi-internal-fixture"));
    let initial = controller
        .shared_snapshot()
        .chess
        .expect("native Chess fixture");
    controller
        .settled(DesktopCommand::NewGame {
            version: initial.version,
            document_revision: initial.document.revision,
            human: Color::Black,
        })
        .unwrap();
    let failed = until(&mut controller, |snapshot| {
        snapshot.engine.status == EnginePhase::Failed
    });
    assert!(
        matches!(&failed.engine.last, Some(ChessResult::Failure { failure }) if failure.kind == FailureKind::MissingResponse)
    );
    assert!(failed.history.is_empty());
    assert!(failed.conclusion.is_none());
    assert_eq!(failed.turn, Color::White);
    let stable = controller
        .shared_snapshot()
        .chess
        .expect("native Chess fixture");
    assert_eq!(
        stable.version, failed.version,
        "failure does not silently retry"
    );
    assert_eq!(stable.engine.status, EnginePhase::Failed);
    let retry = controller
        .settled(DesktopCommand::Retry {
            version: failed.version,
        })
        .unwrap();
    assert_eq!(retry.engine.status, EnginePhase::Thinking);
    let failed = until(&mut controller, |snapshot| {
        snapshot.engine.status == EnginePhase::Failed
    });
    assert!(
        matches!(&failed.engine.last, Some(ChessResult::Failure { failure }) if failure.kind == FailureKind::UnexpectedExit)
    );
    assert!(failed.history.is_empty());
    assert!(failed.conclusion.is_none());
    assert!(
        failed
            .engine
            .message
            .as_ref()
            .unwrap()
            .contains("Engine failure")
    );
    shutdown(&mut controller);
}

// Match the host command promise: completion means native admission settled.
trait SettledCommand {
    fn settled(&mut self, command: DesktopCommand) -> Result<Snapshot, String>;
}
impl SettledCommand for DesktopController {
    fn settled(&mut self, command: DesktopCommand) -> Result<Snapshot, String> {
        let expected = self.shared_token();
        let (initial, receipt) =
            self.shared_command_with_receipt(mshogi_desktop_session::SharedDesktopCommand {
                expected,
                game: mshogi_desktop_session::GameKind::Chess,
                operation: mshogi_desktop_session::SessionOperation::Chess { command },
            })?;
        let mut snapshot = initial.chess.expect("native Chess fixture");
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            if let Some(result) = receipt.completion() {
                result?;
                return Ok(snapshot);
            }
            assert!(
                Instant::now() < deadline,
                "native capacity admission deadline"
            );
            thread::sleep(Duration::from_millis(1));
            snapshot = self.shared_snapshot().chess.expect("native Chess fixture");
        }
    }
}
