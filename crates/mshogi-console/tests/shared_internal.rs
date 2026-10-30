//! Actual console entrypoint plus separately labeled supervisor instrumentation.
use mshogi_core::{chess::*, native::*, shogi};
use mshogi_engine::*;
use std::{
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};
fn context(id: u64, shogi: bool) -> ExecutionRequest {
    ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: id,
            session_version: 2,
            process_generation: id,
            game: if shogi {
                CanonicalRecord::shogi(shogi::GameRecord::from_game(&shogi::Game::new()))
            } else {
                CanonicalRecord::chess(AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record())
            },
            limits: WorkLimits {
                depth: 1,
                nodes: 1000,
            },
        },
    }
}
fn bridge(path: &str) -> EngineBridge {
    EngineBridge::with_options(
        PathBuf::from(path),
        BridgeOptions {
            watchdog: Duration::from_secs(15),
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
        assert!(Instant::now() < deadline, "shared completion deadline");
        thread::sleep(Duration::from_millis(1));
    }
}
fn stop(bridge: &EngineBridge) {
    bridge.shutdown();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !bridge.shutdown_acknowledged() {
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
    }
}
#[test]
fn real_entrypoint_executes_both_native_games() {
    let bridge = bridge(env!("CARGO_BIN_EXE_mshogi"));
    let chess = context(1, false);
    bridge.submit(chess.clone()).unwrap();
    let response = wait(&bridge);
    assert!(response.matches(&chess));
    let ExecutionResult::Chess {
        result: ChessResult::Move { action, telemetry },
    } = response.observation.result
    else {
        panic!("{response:?}")
    };
    assert!(
        AdjudicatedGame::new(ClaimPolicy::ExplicitOnly)
            .position()
            .legal_actions()
            .contains(&action)
    );
    assert_eq!(telemetry.nodes, 21);
    assert_eq!(telemetry.score, Some(0));
    let shogi = context(2, true);
    bridge.submit(shogi.clone()).unwrap();
    let response = wait(&bridge);
    assert!(response.matches(&shogi));
    let ExecutionResult::Shogi {
        result:
            ShogiResult::Choice {
                choice: ShogiChoice::Move { action },
                telemetry,
            },
    } = response.observation.result
    else {
        panic!("{response:?}")
    };
    assert!(
        shogi::Game::new()
            .position()
            .legal_actions()
            .contains(&action)
    );
    assert_eq!(telemetry.nodes, 31);
    assert_eq!(telemetry.score, Some(0));
    let mut incomplete = context(3, false);
    incomplete.context.limits.nodes = 0;
    bridge.submit(incomplete).unwrap();
    assert!(matches!(
        wait(&bridge).observation.result,
        ExecutionResult::Chess {
            result: ChessResult::Incomplete {
                telemetry: Telemetry {
                    score: None,
                    nodes: 0,
                    ..
                }
            }
        }
    ));
    stop(&bridge);
}
#[test]
fn forged_purpose_history_limits_selection_and_identities_never_publish_as_valid_work() {
    let bridge = bridge(env!("CARGO_BIN_EXE_mshogi-internal-fixture"));
    for id in 20..=26 {
        let mut request = context(id, false);
        request.context.purpose = ExecutionPurpose::MoveAnalysis {
            target: ExecutionTarget {
                selection_id: 1,
                event_index: 0,
                board_ply: 0,
                session_version: 2,
                label: "Live prefix".into(),
                turn: NativeSide::Chess(Color::White),
            },
        };
        bridge.submit(request.clone()).unwrap();
        let response = wait(&bridge);
        assert_eq!(response.observation.context, request.context);
        assert!(
            matches!(
                response.observation.result,
                ExecutionResult::Failure {
                    failure: EngineFailure {
                        kind: FailureKind::ContextMismatch,
                        ..
                    }
                }
            ),
            "{id}: {response:?}"
        );
    }
    stop(&bridge);
}
#[test]
fn one_owner_retires_work_across_games_selection_and_semantic_undo() {
    let bridge = bridge(env!("CARGO_BIN_EXE_mshogi-internal-fixture"));
    bridge.submit(context(5, false)).unwrap();
    thread::sleep(Duration::from_millis(20));
    let shogi = context(8, true);
    bridge.submit(shogi.clone()).unwrap();
    assert_eq!(wait(&bridge).observation.context, shogi.context);
    bridge.cancel();
    assert_eq!(bridge.submit(shogi), Err(SubmitError::IdentityReused));
    let mut game = shogi::Game::new();
    let root = game.clone();
    let undo = game
        .submit(shogi::GameRequest {
            expected: game.context(),
            actor: shogi::Side::Gote,
            command: shogi::GameCommand::Resign,
        })
        .unwrap();
    game.undo(undo).unwrap();
    assert_eq!(game, root);
    // Native semantic equality cannot reset the host request/generation ledger.
    let mut retired = context(8, true);
    retired.context.game = CanonicalRecord::shogi(shogi::GameRecord::from_game(&game));
    assert_eq!(bridge.submit(retired), Err(SubmitError::IdentityReused));
    bridge.submit(context(9, false)).unwrap();
    thread::sleep(Duration::from_millis(100));
    bridge.cancel();
    assert!(bridge.poll().is_none());
    bridge.submit(context(12, false)).unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if let Some(response) = bridge.poll() {
            assert_eq!(response.observation.context.request_id, 12);
            break;
        }
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
    }
    assert!(bridge.poll().is_none());
    stop(&bridge);
}
