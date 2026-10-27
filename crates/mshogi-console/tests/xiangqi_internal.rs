//! Actual internal entrypoint and separately labeled cross-game process fixtures.
//! These tests establish routing and supervision, never Xiangqi search capability.
use mshogi_core::{chess, native::*, xiangqi as xq};
use mshogi_engine::*;
use std::{
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

fn game() -> xq::Game {
    let mut game = xq::Game::from_root(
        xq::PositionRoot::from_fen("4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1").unwrap(),
        xq::ClaimPolicy::Explicit,
    )
    .unwrap();
    for _ in 0..2 {
        for text in ["e9f9", "f8e8", "f9e9", "e8f8"] {
            game.submit(xq::GameRequest {
                expected: game.context(),
                actor: game.position().side_to_move(),
                command: xq::GameCommand::Play {
                    action: text.parse().unwrap(),
                },
            })
            .unwrap();
        }
    }
    assert!(game.warnings().iter().any(|warning| warning.active));
    game
}
fn request(id: u64, game: &xq::Game) -> SharedRequest {
    SharedRequest {
        protocol: SHARED_PROTOCOL_VERSION,
        purpose: SharedPurpose::EngineTurn {},
        request_id: id,
        session_version: 19,
        process_generation: id,
        game: CanonicalRecord::xiangqi(xq::GameRecord::from_game(game).unwrap()),
        // Opaque unsupported-execution settings, not advertised Xiangqi limits.
        limits: WorkLimits {
            depth: 255,
            nodes: u64::MAX,
        },
    }
}
fn target(request: &SharedRequest, selection_id: u64) -> SharedAnalysisTarget {
    SharedAnalysisTarget {
        selection_id,
        event_index: request.game.event_count() as u64,
        board_ply: request.game.known_board_actions() as u64,
        session_version: request.session_version,
        label: "Known Xiangqi prefix with WXF warning".into(),
        turn: NativeSide::Xiangqi(xq::Side::Red),
    }
}
fn chess_request(id: u64) -> SharedRequest {
    SharedRequest {
        protocol: SHARED_PROTOCOL_VERSION,
        purpose: SharedPurpose::EngineTurn {},
        request_id: id,
        session_version: 19,
        process_generation: id,
        game: CanonicalRecord::chess(
            chess::AdjudicatedGame::new(chess::ClaimPolicy::ExplicitOnly).record(),
        ),
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
fn wait(bridge: &EngineBridge) -> SharedResponse {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if let Some(response) = bridge.poll_shared() {
            return response;
        }
        assert!(
            Instant::now() < deadline,
            "Xiangqi routing response deadline"
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
            "native supervisor shutdown deadline"
        );
        thread::sleep(Duration::from_millis(1));
    }
}
fn assert_failure(response: &SharedResponse, expected: FailureKind) {
    let SharedResult::Failure { failure } = &response.result else {
        panic!("{response:?}");
    };
    assert_eq!(failure.kind, expected, "{response:?}");
    assert!(!failure.message.is_empty());
    assert!(failure.message.len() <= MAX_STDERR_BYTES);
    let value = serde_json::to_value(&response.result).unwrap();
    assert!(value.get("score").is_none());
    assert!(value.get("action").is_none());
    assert!(value.get("conclusion").is_none());
}

#[test]
fn real_entrypoint_echoes_complete_xiangqi_context_but_never_executes_any_operation() {
    assert!(execution_capabilities(GameKind::Xiangqi).is_empty());
    let bridge = bridge(env!("CARGO_BIN_EXE_mshogi"), Duration::from_secs(15));
    let mut game = game();
    for id in 1..=4 {
        // A formal native resignation adds events while preserving board ply.
        if id == 4 {
            let ply = xq::GameRecord::from_game(&game)
                .unwrap()
                .known_board_actions();
            game.submit(xq::GameRequest {
                expected: game.context(),
                actor: xq::Side::Black,
                command: xq::GameCommand::Resign,
            })
            .unwrap();
            assert_eq!(
                xq::GameRecord::from_game(&game)
                    .unwrap()
                    .known_board_actions(),
                ply
            );
            assert!(game.conclusion().is_some());
        }
        let mut context = request(id, &game);
        if id == 2 {
            context.purpose = SharedPurpose::MoveAnalysis {
                target: target(&context, 1),
            };
        }
        if id == 3 {
            context.purpose = SharedPurpose::ChessFullReachability {
                target: target(&context, 2),
                mate: ReachabilityTarget::Either,
                limits: FullLimits {
                    max_positions: 100,
                    max_transitions: 1000,
                },
            };
            context.limits = FULL_SENTINEL;
        }
        assert!(context.valid_limits());
        let wire = serde_json::to_string(&context).unwrap();
        assert!(wire.len() < MAX_REQUEST_BYTES);
        let before = xq::GameRecord::from_game(&game).unwrap();
        bridge.submit_shared(context.clone()).unwrap();
        let response = wait(&bridge);
        assert!(response.matches(&context));
        assert_eq!(response.context, context);
        assert_failure(&response, FailureKind::UnsupportedMode);
        assert_eq!(before, xq::GameRecord::from_game(&game).unwrap());
        assert!(serde_json::to_string(&response).unwrap().len() < MAX_RESPONSE_BYTES);
    }
    stop(&bridge);
}

#[test]
fn forged_xiangqi_identity_purpose_selection_limits_and_same_board_history_are_rejected() {
    let bridge = bridge(
        env!("CARGO_BIN_EXE_mshogi-internal-fixture"),
        Duration::from_secs(15),
    );
    let game = game();
    for id in 20..=27 {
        let mut context = request(id, &game);
        // Leave headroom for the intentional limit mutation in fixture23.
        context.limits.nodes = 17;
        context.purpose = SharedPurpose::MoveAnalysis {
            target: target(&context, id),
        };
        bridge.submit_shared(context.clone()).unwrap();
        let response = wait(&bridge);
        assert_eq!(response.context, context);
        assert_failure(&response, FailureKind::ContextMismatch);
    }
    stop(&bridge);
}

#[test]
fn one_supervisor_replaces_sleeping_work_in_both_game_directions() {
    let game = game();
    for xiangqi_first in [true, false] {
        let bridge = bridge(
            env!("CARGO_BIN_EXE_mshogi-internal-fixture"),
            Duration::from_secs(15),
        );
        let old = if xiangqi_first {
            request(5, &game)
        } else {
            chess_request(5)
        };
        bridge.submit_shared(old.clone()).unwrap();
        thread::sleep(Duration::from_millis(30));
        let current = if xiangqi_first {
            chess_request(8)
        } else {
            request(8, &game)
        };
        bridge.submit_shared(current.clone()).unwrap();
        let response = wait(&bridge);
        assert_eq!(response.context, current);
        assert!(response.matches(&current));
        assert_failure(&response, FailureKind::UnsupportedMode);
        assert!(bridge.poll_shared().is_none());
        assert_eq!(bridge.submit_shared(old), Err(SubmitError::IdentityReused));
        bridge.cancel();
        assert!(bridge.poll_shared().is_none());
        stop(&bridge);
    }
}

#[test]
fn native_undo_reopen_and_reselected_identical_history_cannot_revive_retired_work() {
    let bridge = bridge(
        env!("CARGO_BIN_EXE_mshogi-internal-fixture"),
        Duration::from_secs(15),
    );
    let mut game = game();
    let root_record = xq::GameRecord::from_game(&game).unwrap();
    let mut retired = request(5, &game);
    retired.purpose = SharedPurpose::MoveAnalysis {
        target: target(&retired, 1),
    };
    bridge.submit_shared(retired.clone()).unwrap();
    thread::sleep(Duration::from_millis(30));
    bridge.cancel();
    let undo = game
        .submit(xq::GameRequest {
            expected: game.context(),
            actor: xq::Side::Black,
            command: xq::GameCommand::Resign,
        })
        .unwrap();
    game.undo(undo.undo).unwrap();
    assert_eq!(root_record, xq::GameRecord::from_game(&game).unwrap());
    let reopened = root_record.to_game().unwrap();
    assert_ne!(game.context(), reopened.context());
    retired.game = CanonicalRecord::xiangqi(xq::GameRecord::from_game(&reopened).unwrap());
    if let SharedPurpose::MoveAnalysis { target } = &mut retired.purpose {
        target.selection_id = 2;
    }
    assert_eq!(
        bridge.submit_shared(retired),
        Err(SubmitError::IdentityReused)
    );
    let mut current = request(8, &reopened);
    current.session_version += 1;
    current.purpose = SharedPurpose::MoveAnalysis {
        target: target(&current, 3),
    };
    bridge.submit_shared(current.clone()).unwrap();
    let response = wait(&bridge);
    assert_eq!(response.context, current);
    assert_failure(&response, FailureKind::UnsupportedMode);
    assert!(bridge.poll_shared().is_none());
    stop(&bridge);
}

#[test]
fn timeout_is_a_bounded_process_failure_and_does_not_conclude_the_game() {
    let bridge = bridge(
        env!("CARGO_BIN_EXE_mshogi-internal-fixture"),
        Duration::from_millis(100),
    );
    let game = game();
    let before = xq::GameRecord::from_game(&game).unwrap();
    let context = request(5, &game);
    bridge.submit_shared(context.clone()).unwrap();
    let response = wait(&bridge);
    assert_eq!(response.context, context);
    assert_failure(&response, FailureKind::Timeout);
    assert_eq!(before, xq::GameRecord::from_game(&game).unwrap());
    assert!(game.conclusion().is_none());
    stop(&bridge);
}
