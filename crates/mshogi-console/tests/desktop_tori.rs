//! Actual controller and supervised child; no Tori search or gameplay GUI claim.
use mshogi_core::{native::*, tori};
use mshogi_desktop_session::*;
use mshogi_engine::*;
use std::{
    thread,
    time::{Duration, Instant},
};
fn command(c: &mut DesktopController, operation: SessionOperation) -> SharedSnapshot {
    let s = c.shared_snapshot();
    let (_, r) = c
        .shared_command_with_receipt(SharedDesktopCommand {
            expected: s.token,
            game: s.game,
            operation,
        })
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let s = c.shared_snapshot();
        if let Some(r) = r.completion() {
            r.unwrap();
            return s;
        }
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
    }
}
fn controller(path: &str) -> DesktopController {
    let mut c = DesktopController::new(path.into()).unwrap();
    let mut game = tori::Game::standard();
    let action = game.legal_actions()[0];
    game.submit(tori::GameRequest {
        expected: game.context(),
        actor: tori::Side::Sente,
        command: tori::GameCommand::Play { action },
    })
    .unwrap();
    command(
        &mut c,
        SessionOperation::Replace {
            record: CanonicalRecord::tori(tori::GameRecord::from_game(&game).unwrap()),
            human: NativeSide::Tori(tori::Side::Sente),
        },
    );
    c
}
fn observation(c: &mut DesktopController) -> ExecutionObservation {
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        let s = c.shared_snapshot();
        if let Some(o) = s.observations.last() {
            return o.observation.clone();
        }
        assert!(Instant::now() < deadline, "{s:?}");
        thread::sleep(Duration::from_millis(1));
    }
}
#[test]
fn real_internal_tori_unsupported_controller_observation_save_reopen_stays_stopped() {
    let mut c = controller(env!("CARGO_BIN_EXE_mshogi"));
    let before = c.shared_snapshot().context;
    let context = c
        .verify_tori_execution(
            WorkLimits {
                depth: 1,
                nodes: 100,
            },
            true,
        )
        .unwrap();
    let observed = observation(&mut c);
    assert_eq!(observed.context, context);
    assert!(matches!(
        &observed.result,
        ExecutionResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::UnsupportedMode,
                ..
            }
        }
    ));
    assert_eq!(c.shared_snapshot().context, before);
    let path =
        std::env::temp_dir().join(format!("msh131-real-child-{}.mshogi", std::process::id()));
    let id = c.queue_shared_save(path.clone()).unwrap();
    wait_file(&mut c, id);
    let old = c.shared_snapshot().token;
    let token = c.shared_open_token();
    let id = c.queue_shared_open(path.clone(), token).unwrap();
    wait_file(&mut c, id);
    let opened = c.shared_snapshot();
    assert!(opened.paused);
    assert!(opened.token.version > old.version);
    assert!(opened.token.document > old.document);
    assert!(opened.observations[0].historical);
    assert_eq!(opened.context, before);
    assert!(!c.accept_shared(ExecutionResponse {
        protocol: PROTOCOL_VERSION,
        observation: observed
    }));
    for operation in [
        SessionOperation::Resume {},
        SessionOperation::Retry {},
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves {
                depth: 1,
                nodes: 100,
            },
        },
    ] {
        let s = c.shared_snapshot();
        let (_, r) = c
            .shared_command_with_receipt(SharedDesktopCommand {
                expected: s.token,
                game: s.game,
                operation,
            })
            .unwrap();
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            c.shared_snapshot();
            if let Some(result) = r.completion() {
                assert!(result.is_err());
                break;
            }
            assert!(Instant::now() < deadline);
            thread::sleep(Duration::from_millis(1));
        }
    }
    std::fs::remove_file(path).unwrap();
    c.begin_shutdown();
}
fn wait_file(c: &mut DesktopController, id: u64) {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let s = c.shared_snapshot();
        if s.io.id == id && matches!(s.io.phase, IoPhase::Complete | IoPhase::Failed) {
            assert!(matches!(s.io.phase, IoPhase::Complete), "{s:?}");
            return;
        }
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
    }
}
fn started(context: &ExecutionContext) -> std::path::PathBuf {
    let marker = std::env::temp_dir().join(format!(
        "msh131-started-{}-{}.txt",
        std::process::id(),
        context.request_id
    ));
    let deadline = Instant::now() + Duration::from_secs(5);
    while !marker.is_file() {
        assert!(Instant::now() < deadline, "actual child did not start");
        thread::sleep(Duration::from_millis(1));
    }
    let child: u32 = std::fs::read_to_string(&marker).unwrap().parse().unwrap();
    assert_ne!(child, std::process::id());
    marker
}
#[test]
fn real_supervised_tori_failure_cancel_replace_cleanup_and_cross_game_retirement() {
    let mut c = controller(env!("CARGO_BIN_EXE_mshogi-internal-fixture"));
    let failed = c
        .verify_tori_execution(WorkLimits { depth: 1, nodes: 1 }, false)
        .unwrap();
    let observed = observation(&mut c);
    assert_eq!(observed.context, failed);
    assert!(matches!(
        observed.result,
        ExecutionResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::UnexpectedExit,
                ..
            }
        }
    ));
    let slow = c
        .verify_tori_execution(WorkLimits { depth: 1, nodes: 7 }, true)
        .unwrap();
    let marker = started(&slow);
    let started_at = Instant::now();
    command(&mut c, SessionOperation::Cancel {});
    assert!(started_at.elapsed() < Duration::from_secs(5));
    std::fs::remove_file(marker).unwrap();
    assert!(!c.accept_shared(ExecutionResponse {
        protocol: PROTOCOL_VERSION,
        observation: ExecutionObservation::new(
            slow.clone(),
            ExecutionResult::Failure {
                failure: EngineFailure {
                    kind: FailureKind::UnsupportedMode,
                    message: "delayed retired response".into()
                }
            }
        )
    }));
    let replacement = c
        .verify_tori_execution(WorkLimits { depth: 1, nodes: 7 }, true)
        .unwrap();
    assert!(replacement.request_id > slow.request_id);
    assert!(replacement.process_generation > slow.process_generation);
    let marker = started(&replacement);
    command(
        &mut c,
        SessionOperation::Replace {
            record: CanonicalRecord::shogi(mshogi_core::shogi::GameRecord::from_game(
                &mshogi_core::shogi::Game::new(),
            )),
            human: NativeSide::Shogi(mshogi_core::shogi::Side::Sente),
        },
    );
    assert!(!c.accept_shared(ExecutionResponse {
        protocol: PROTOCOL_VERSION,
        observation: ExecutionObservation::new(
            replacement,
            ExecutionResult::Failure {
                failure: EngineFailure {
                    kind: FailureKind::UnsupportedMode,
                    message: "old Tori result for Shogi".into()
                }
            }
        )
    }));
    std::fs::remove_file(marker).unwrap();
    c.begin_shutdown();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !c.shutdown_acknowledged() {
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
    }
}
