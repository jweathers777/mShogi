//! Actual native executable and portable supervised pipe/process fixtures.
use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy};
use mshogi_engine::*;
use std::{
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};
fn request(id: u64) -> EngineRequest {
    EngineRequest {
        protocol: PROTOCOL_VERSION,
        request_id: id,
        session_version: 2,
        process_generation: id,
        game: AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record(),
        limits: WorkLimits {
            depth: 1,
            nodes: 1000,
        },
    }
}
fn bridge(path: &str, watchdog: Duration) -> EngineBridge {
    EngineBridge::with_options(
        PathBuf::from(path),
        BridgeOptions {
            watchdog,
            cleanup: Duration::from_secs(1),
        },
    )
    .unwrap()
}
fn wait(bridge: &EngineBridge) -> EngineResponse {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if let Some(response) = bridge.poll() {
            return response;
        }
        assert!(
            Instant::now() < deadline,
            "engine bridge completion deadline"
        );
        thread::sleep(Duration::from_millis(1));
    }
}
fn stop(bridge: &EngineBridge) {
    bridge.shutdown();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !bridge.shutdown_acknowledged() {
        assert!(
            Instant::now() < deadline,
            "engine cleanup shutdown acknowledgement"
        );
        thread::sleep(Duration::from_millis(1));
    }
}
#[test]
fn real_bundled_entrypoint_echoes_context_and_plays_an_actual_legal_move() {
    let bridge = bridge(env!("CARGO_BIN_EXE_mshogi"), Duration::from_secs(15));
    let context = request(1);
    bridge.submit(context.clone()).unwrap();
    let response = wait(&bridge);
    assert_eq!(response.context, context);
    let EngineResult::Move { action, telemetry } = response.result else {
        panic!("{response:?}")
    };
    let game = AdjudicatedGame::replay_record(&context.game).unwrap();
    assert!(game.position().legal_actions().contains(&action));
    assert_eq!(telemetry.completed_depth, 1);
    assert_eq!(telemetry.nodes, 21);
    assert_eq!(telemetry.score, Some(0));
    assert_eq!(
        telemetry.principal_variation,
        vec![EngineChoice::Move { action }]
    );
    stop(&bridge);
}
#[test]
fn real_entrypoint_preserves_incomplete_search_instead_of_fallback_move() {
    let bridge = bridge(env!("CARGO_BIN_EXE_mshogi"), Duration::from_secs(15));
    let mut context = request(1);
    context.limits.nodes = 0;
    bridge.submit(context).unwrap();
    let EngineResult::Incomplete { telemetry } = wait(&bridge).result else {
        panic!("no incomplete result")
    };
    assert_eq!(telemetry.score, None);
    assert_eq!(telemetry.nodes, 0);
    stop(&bridge);
}
#[test]
fn actual_process_failures_remain_explicit_unscored_and_bounded() {
    for (id, kind) in [
        (1, FailureKind::MissingResponse),
        (2, FailureKind::UnexpectedExit),
        (3, FailureKind::MalformedResponse),
        (4, FailureKind::OversizedResponse),
        (5, FailureKind::Timeout),
        (6, FailureKind::ContextMismatch),
        (9, FailureKind::UnexpectedExit),
        (10, FailureKind::MalformedResponse),
        (11, FailureKind::MalformedResponse),
    ] {
        // Startup gets a generous deadline except the fixture intentionally waiting.
        let bridge = bridge(
            env!("CARGO_BIN_EXE_mshogi-internal-fixture"),
            if id == 5 {
                Duration::from_millis(100)
            } else {
                Duration::from_secs(15)
            },
        );
        bridge.submit(request(id)).unwrap();
        let response = wait(&bridge);
        let EngineResult::Failure { failure } = response.result else {
            panic!("fixture {id}: {response:?}")
        };
        assert_eq!(failure.kind, kind, "fixture {id}: {}", failure.message);
        assert!(failure.message.len() <= MAX_STDERR_BYTES);
        stop(&bridge);
    }
    let bridge = bridge("/definitely/missing/mshogi-engine", Duration::from_secs(15));
    bridge.submit(request(1)).unwrap();
    assert!(matches!(
        wait(&bridge).result,
        EngineResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::Start,
                ..
            }
        }
    ));
    stop(&bridge);
}
#[test]
fn cancellation_replacement_and_reused_identity_cannot_publish_old_work() {
    let bridge = bridge(
        env!("CARGO_BIN_EXE_mshogi-internal-fixture"),
        Duration::from_secs(15),
    );
    bridge.submit(request(5)).unwrap();
    thread::sleep(Duration::from_millis(20));
    bridge.cancel();
    assert_eq!(bridge.submit(request(5)), Err(SubmitError::IdentityReused));
    bridge.submit(request(8)).unwrap();
    let response = wait(&bridge);
    assert_eq!(response.context.request_id, 8);
    assert!(matches!(response.result, EngineResult::Incomplete { .. }));
    assert!(bridge.poll().is_none());
    bridge.cancel();
    stop(&bridge);
    assert_eq!(bridge.submit(request(12)), Err(SubmitError::ShuttingDown));
}
#[test]
fn shutdown_terminates_an_active_child_without_waiting_on_the_caller() {
    let bridge = bridge(
        env!("CARGO_BIN_EXE_mshogi-internal-fixture"),
        Duration::from_secs(15),
    );
    bridge.submit(request(5)).unwrap();
    thread::sleep(Duration::from_millis(20));
    stop(&bridge);
    assert!(bridge.poll().is_none());
}
