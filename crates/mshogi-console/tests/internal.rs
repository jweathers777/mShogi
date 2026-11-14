//! Actual native executable and portable supervised pipe/process fixtures.
use mshogi_core::{
    chess::{AdjudicatedGame, ClaimPolicy},
    native::CanonicalRecord,
};
use mshogi_engine::*;
use std::{
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};
fn request(id: u64) -> ExecutionRequest {
    ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: id,
            session_version: 2,
            process_generation: id,
            game: CanonicalRecord::chess(AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record()),
            limits: WorkLimits {
                time_ms: None,
                depth: 1,
                nodes: 1000,
            },
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
fn wait(bridge: &EngineBridge) -> ExecutionResponse {
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
    assert_eq!(response.observation.context, context.context);
    let ExecutionResult::Chess {
        result: ChessResult::Move { action, telemetry },
    } = response.observation.result
    else {
        panic!("{response:?}")
    };
    let CanonicalRecord::Chess(record) = &context.context.game else {
        panic!()
    };
    let game = AdjudicatedGame::replay_record(record).unwrap();
    assert!(game.position().legal_actions().contains(&action));
    // Independently derived Nb1-c3 preference:44cp piece placement,
    // 15cp safe knight mobility and2cp rook mobility, for61cp total.
    // Lexical ties choose b1c3.
    // This also proves the actual supervised child uses the active candidate,
    // rather than accidentally retaining the material/full-width coordinator.
    assert_eq!(action.to_string(), "b1c3");
    assert_eq!(telemetry.completed_depth, 1);
    // Root plus20 legal leaves are required; PVS may re-search improved moves.
    assert!(telemetry.nodes >= 21 && telemetry.nodes <= context.context.limits.nodes);
    assert_eq!(telemetry.score, Some(61));
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
    context.context.limits.nodes = 0;
    bridge.submit(context).unwrap();
    let ExecutionResult::Chess {
        result: ChessResult::Incomplete { telemetry },
    } = wait(&bridge).observation.result
    else {
        panic!("no incomplete result")
    };
    assert_eq!(telemetry.score, None);
    assert_eq!(telemetry.nodes, 0);
    stop(&bridge);
}

#[test]
fn real_entrypoint_echoes_zero_thinking_budget_without_inventing_a_move() {
    let bridge = bridge(env!("CARGO_BIN_EXE_mshogi"), Duration::from_secs(15));
    let mut context = request(12);
    context.context.limits = WorkLimits {
        depth: 128,
        nodes: 20_000,
        time_ms: Some(0),
    };
    bridge.submit(context.clone()).unwrap();
    let response = wait(&bridge);
    assert!(response.matches(&context));
    let mut wrong_time = context;
    wrong_time.context.limits.time_ms = Some(1);
    assert!(!response.matches(&wrong_time));
    let ExecutionResult::Chess {
        result: ChessResult::Incomplete { telemetry },
    } = response.observation.result
    else {
        panic!("zero thinking time fabricated work: {response:?}")
    };
    assert_eq!(telemetry.completed_depth, 0);
    assert_eq!(telemetry.nodes, 0);
    assert_eq!(telemetry.selective_depth, Some(0));
    assert_eq!(telemetry.score, None);
    assert!(telemetry.principal_variation.is_empty());
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
        let ExecutionResult::Failure { failure } = response.observation.result else {
            panic!("fixture {id}: {response:?}")
        };
        assert_eq!(failure.kind, kind, "fixture {id}: {}", failure.message);
        assert!(failure.message.len() <= MAX_STDERR_BYTES);
        stop(&bridge);
    }
    let bridge = bridge("/definitely/missing/mshogi-engine", Duration::from_secs(15));
    bridge.submit(request(1)).unwrap();
    assert!(matches!(
        wait(&bridge).observation.result,
        ExecutionResult::Failure {
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
    assert_eq!(response.observation.context.request_id, 8);
    assert!(matches!(
        response.observation.result,
        ExecutionResult::Chess {
            result: ChessResult::Incomplete { .. }
        }
    ));
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

/// Nodes of a timed, depth-limited search of the start position.
fn timed_nodes(bridge: &EngineBridge, id: u64) -> u64 {
    let mut context = request(id);
    context.context.limits = WorkLimits {
        depth: 8,
        nodes: 50_000_000,
        time_ms: Some(60_000),
    };
    bridge.submit(context).unwrap();
    let ExecutionResult::Chess {
        result: ChessResult::Move { telemetry, .. },
    } = wait(bridge).observation.result
    else {
        panic!("no move")
    };
    assert_eq!(telemetry.completed_depth, 8);
    telemetry.nodes
}
#[test]
fn one_engine_process_keeps_its_state_between_searches_until_cancellation() {
    let bridge = bridge(env!("CARGO_BIN_EXE_mshogi"), Duration::from_secs(60));
    let cold = timed_nodes(&bridge, 1);
    // The same process answers again: its table has the earlier search.
    let warm = timed_nodes(&bridge, 2);
    assert!(warm * 2 < cold, "cold {cold}, warm {warm}");
    // A cancelled search stops the process. The next search starts a new one.
    let mut long = request(3);
    long.context.limits = WorkLimits {
        depth: 128,
        nodes: u64::MAX,
        time_ms: Some(60_000),
    };
    bridge.submit(long).unwrap();
    thread::sleep(Duration::from_millis(200));
    bridge.cancel();
    assert_eq!(timed_nodes(&bridge, 4), cold);
    // A new request replaces a running search: the process is stopped, the
    // replaced search publishes nothing, and the new search starts empty.
    let mut long = request(5);
    long.context.limits = WorkLimits {
        depth: 128,
        nodes: u64::MAX,
        time_ms: Some(60_000),
    };
    bridge.submit(long).unwrap();
    thread::sleep(Duration::from_millis(200));
    assert_eq!(timed_nodes(&bridge, 6), cold);
    assert!(bridge.poll().is_none());
    stop(&bridge);
}
#[test]
fn a_kept_engine_process_that_ended_is_replaced_without_a_failure() {
    // The fixture ends after each response, as an engine that stopped while idle.
    let bridge = bridge(
        env!("CARGO_BIN_EXE_mshogi-internal-fixture"),
        Duration::from_secs(15),
    );
    for id in [7, 8, 12] {
        bridge.submit(request(id)).unwrap();
        let response = wait(&bridge);
        assert_eq!(response.observation.context.request_id, id);
        assert!(matches!(
            response.observation.result,
            ExecutionResult::Chess {
                result: ChessResult::Incomplete { .. }
            }
        ));
        // Give the fixture time to end in two of the three cases.
        thread::sleep(Duration::from_millis(if id == 8 { 0 } else { 100 }));
    }
    stop(&bridge);
}
#[test]
fn a_kept_engine_process_that_fails_during_a_request_is_a_typed_failure() {
    let bridge = bridge(
        env!("CARGO_BIN_EXE_mshogi-internal-fixture"),
        Duration::from_secs(15),
    );
    // The fixture answers request 30, stays, reads request 31 and then fails.
    bridge.submit(request(30)).unwrap();
    assert_eq!(wait(&bridge).observation.context.request_id, 30);
    bridge.submit(request(31)).unwrap();
    let ExecutionResult::Failure { failure } = wait(&bridge).observation.result else {
        panic!("a failed engine process was replaced without a report")
    };
    assert_eq!(failure.kind, FailureKind::UnexpectedExit);
    assert!(
        failure
            .message
            .contains("fixture failure during a later request")
    );
    // The next request starts a new process.
    bridge.submit(request(32)).unwrap();
    assert_eq!(wait(&bridge).observation.context.request_id, 32);
    stop(&bridge);
}
