//! Actual console entrypoint plus separately labeled supervisor instrumentation.
use mshogi_core::{chess::*, native::*, shogi};
use mshogi_engine::*;
use std::{
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};
fn context(id: u64, shogi: bool) -> SharedRequest {
    SharedRequest {
        protocol: SHARED_PROTOCOL_VERSION,
        purpose: SharedPurpose::EngineTurn {},
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
fn wait(bridge: &EngineBridge) -> SharedResponse {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if let Some(response) = bridge.poll_shared() {
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
    bridge.submit_shared(chess.clone()).unwrap();
    let response = wait(&bridge);
    assert!(response.matches(&chess));
    let SharedResult::Chess {
        result: EngineResult::Move { action, telemetry },
    } = response.result
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
    bridge.submit_shared(shogi.clone()).unwrap();
    let response = wait(&bridge);
    assert!(response.matches(&shogi));
    let SharedResult::Shogi {
        result:
            ShogiResult::Choice {
                choice: ShogiChoice::Move { action },
                telemetry,
            },
    } = response.result
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
    incomplete.limits.nodes = 0;
    bridge.submit_shared(incomplete).unwrap();
    assert!(matches!(
        wait(&bridge).result,
        SharedResult::Chess {
            result: EngineResult::Incomplete {
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
        request.purpose = SharedPurpose::MoveAnalysis {
            target: SharedAnalysisTarget {
                selection_id: 1,
                event_index: 0,
                board_ply: 0,
                session_version: 2,
                label: "Live prefix".into(),
                turn: NativeSide::Chess(Color::White),
            },
        };
        bridge.submit_shared(request.clone()).unwrap();
        let response = wait(&bridge);
        assert_eq!(response.context, request);
        assert!(
            matches!(
                response.result,
                SharedResult::Failure {
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
fn one_owner_retires_work_across_versions_games_selection_and_semantic_undo() {
    let bridge = bridge(env!("CARGO_BIN_EXE_mshogi-internal-fixture"));
    let legacy = |id| EngineRequest {
        protocol: PROTOCOL_VERSION,
        purpose: RequestPurpose::EngineTurn,
        request_id: id,
        session_version: 2,
        process_generation: id,
        game: AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record(),
        limits: WorkLimits {
            depth: 1,
            nodes: 1000,
        },
    };
    bridge.submit(legacy(5)).unwrap();
    thread::sleep(Duration::from_millis(20));
    let shogi = context(8, true);
    bridge.submit_shared(shogi.clone()).unwrap();
    // Calling the legacy poll must not consume a shared result.
    thread::sleep(Duration::from_millis(100));
    assert!(bridge.poll().is_none());
    assert_eq!(wait(&bridge).context, shogi);
    bridge.cancel();
    assert_eq!(
        bridge.submit_shared(shogi),
        Err(SubmitError::IdentityReused)
    );
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
    retired.game = CanonicalRecord::shogi(shogi::GameRecord::from_game(&game));
    assert_eq!(
        bridge.submit_shared(retired),
        Err(SubmitError::IdentityReused)
    );
    bridge.submit_shared(context(9, false)).unwrap();
    thread::sleep(Duration::from_millis(100));
    bridge.cancel();
    assert!(bridge.poll_shared().is_none());
    bridge.submit(legacy(12)).unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        assert!(bridge.poll_shared().is_none());
        if let Some(response) = bridge.poll() {
            assert_eq!(response.context.request_id, 12);
            break;
        }
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
    }
    assert!(bridge.poll_shared().is_none());
    stop(&bridge);
}
