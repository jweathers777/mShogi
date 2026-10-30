//! Actual internal entrypoint and separately labeled cross-game process fixtures.
//! Actual child establishes ordinary native execution; separate fixtures establish supervision.
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
fn request(id: u64, game: &xq::Game) -> ExecutionRequest {
    ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: id,
            session_version: 19,
            process_generation: id,
            game: CanonicalRecord::xiangqi(xq::GameRecord::from_game(game).unwrap()),
            // Measured native Xiangqi ordinary-search limits.
            limits: WorkLimits {
                depth: 1,
                nodes: 128,
            },
        },
    }
}
fn target(request: &ExecutionRequest, selection_id: u64) -> ExecutionTarget {
    ExecutionTarget {
        selection_id,
        event_index: request.context.game.event_count() as u64,
        board_ply: request.context.game.known_board_actions() as u64,
        session_version: request.context.session_version,
        label: "Known Xiangqi prefix with WXF warning".into(),
        turn: NativeSide::Xiangqi(xq::Side::Red),
    }
}
fn chess_request(id: u64) -> ExecutionRequest {
    ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
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
fn assert_failure(response: &ExecutionResponse, expected: FailureKind) {
    let ExecutionResult::Failure { failure } = &response.observation.result else {
        panic!("{response:?}");
    };
    assert_eq!(failure.kind, expected, "{response:?}");
    assert!(!failure.message.is_empty());
    assert!(failure.message.len() <= MAX_STDERR_BYTES);
    let value = serde_json::to_value(&response.observation.result).unwrap();
    assert!(value.get("score").is_none());
    assert!(value.get("action").is_none());
    assert!(value.get("conclusion").is_none());
}

#[test]
fn real_entrypoint_executes_native_play_and_readonly_analysis_with_full_context() {
    assert_eq!(execution_capabilities(GameKind::Xiangqi).len(), 2);
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
            context.context.purpose = ExecutionPurpose::MoveAnalysis {
                target: target(&context, 1),
            };
        }
        if id == 3 {
            context.context.purpose = ExecutionPurpose::ChessFullReachability {
                target: target(&context, 2),
                mate: ReachabilityTarget::Either,
                limits: FullLimits {
                    max_positions: 100,
                    max_transitions: 1000,
                },
            };
            context.context.limits = FULL_SENTINEL;
        }
        assert!(context.context.valid_limits());
        let wire = serde_json::to_string(&context).unwrap();
        assert!(wire.len() < MAX_REQUEST_BYTES);
        let before = xq::GameRecord::from_game(&game).unwrap();
        bridge.submit(context.clone()).unwrap();
        let response = wait(&bridge);
        assert!(response.matches(&context));
        assert_eq!(response.observation.context, context.context);
        match id {
            1 | 2 => {
                let ExecutionResult::Xiangqi {
                    result:
                        XiangqiResult::Choice {
                            choice,
                            ref telemetry,
                        },
                } = response.observation.result
                else {
                    panic!("{response:?}")
                };
                assert_eq!(telemetry.perspective, xq::Side::Red);
                assert!(telemetry.nodes > 0 && telemetry.nodes <= context.context.limits.nodes);
                let mut candidate = before.to_game().unwrap();
                let command = match choice {
                    XiangqiChoice::Move { action } => xq::GameCommand::Play { action },
                    XiangqiChoice::Claim { .. } => xq::GameCommand::ClaimNaturalCount,
                };
                candidate
                    .submit(xq::GameRequest {
                        expected: candidate.context(),
                        actor: candidate.position().side_to_move(),
                        command,
                    })
                    .unwrap();
            }
            3 => assert_failure(&response, FailureKind::UnsupportedMode),
            4 => assert!(
                matches!(response.observation.result, ExecutionResult::Xiangqi { result: XiangqiResult::Finished { conclusion, ref telemetry } }
                if Some(conclusion) == game.conclusion() && telemetry.perspective == xq::Side::Red && telemetry.score.unwrap() > 0)
            ),
            _ => unreachable!(),
        }
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
        context.context.limits.nodes = 17;
        context.context.purpose = ExecutionPurpose::MoveAnalysis {
            target: target(&context, id),
        };
        bridge.submit(context.clone()).unwrap();
        let response = wait(&bridge);
        assert_eq!(response.observation.context, context.context);
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
        bridge.submit(old.clone()).unwrap();
        thread::sleep(Duration::from_millis(30));
        let current = if xiangqi_first {
            chess_request(8)
        } else {
            request(8, &game)
        };
        bridge.submit(current.clone()).unwrap();
        let response = wait(&bridge);
        assert_eq!(response.observation.context, current.context);
        assert!(response.matches(&current));
        if xiangqi_first {
            assert!(matches!(
                response.observation.result,
                ExecutionResult::Chess {
                    result: ChessResult::Incomplete { .. }
                }
            ));
        } else {
            assert_failure(&response, FailureKind::UnsupportedMode);
        }
        assert!(bridge.poll().is_none());
        assert_eq!(bridge.submit(old), Err(SubmitError::IdentityReused));
        bridge.cancel();
        assert!(bridge.poll().is_none());
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
    retired.context.purpose = ExecutionPurpose::MoveAnalysis {
        target: target(&retired, 1),
    };
    bridge.submit(retired.clone()).unwrap();
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
    retired.context.game = CanonicalRecord::xiangqi(xq::GameRecord::from_game(&reopened).unwrap());
    if let ExecutionPurpose::MoveAnalysis { target } = &mut retired.context.purpose {
        target.selection_id = 2;
    }
    assert_eq!(bridge.submit(retired), Err(SubmitError::IdentityReused));
    let mut current = request(8, &reopened);
    current.context.session_version += 1;
    current.context.purpose = ExecutionPurpose::MoveAnalysis {
        target: target(&current, 3),
    };
    bridge.submit(current.clone()).unwrap();
    let response = wait(&bridge);
    assert_eq!(response.observation.context, current.context);
    assert_failure(&response, FailureKind::UnsupportedMode);
    assert!(bridge.poll().is_none());
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
    bridge.submit(context.clone()).unwrap();
    let response = wait(&bridge);
    assert_eq!(response.observation.context, context.context);
    assert_failure(&response, FailureKind::Timeout);
    assert_eq!(before, xq::GameRecord::from_game(&game).unwrap());
    assert!(game.conclusion().is_none());
    stop(&bridge);
}

#[test]
fn actual_child_zero_work_and_same_board_different_history_retain_native_meaning() {
    let bridge = bridge(env!("CARGO_BIN_EXE_mshogi"), Duration::from_secs(15));
    let mut historical = game();
    for text in ["e9f9", "f8e8", "f9e9", "e8f8"] {
        historical
            .submit(xq::GameRequest {
                expected: historical.context(),
                actor: historical.position().side_to_move(),
                command: xq::GameCommand::Play {
                    action: text.parse().unwrap(),
                },
            })
            .unwrap();
    }
    let expected = xq::Conclusion::Win {
        winner: xq::Side::Black,
        reason: xq::WinReason::Repetition {
            fault: xq::FaultKind::PerpetualCheck,
        },
    };
    assert_eq!(historical.conclusion(), Some(expected));
    let fresh = xq::Game::from_root(
        xq::PositionRoot::from_fen(&historical.position().to_fen().unwrap()).unwrap(),
        xq::ClaimPolicy::Explicit,
    )
    .unwrap();
    assert_eq!(fresh.position(), historical.position());
    assert_eq!(fresh.conclusion(), None);
    let historical_record = xq::GameRecord::from_game(&historical).unwrap();
    let fresh_record = xq::GameRecord::from_game(&fresh).unwrap();
    for (id, source) in [(40, &historical), (41, &fresh), (42, &fresh)] {
        let mut context = request(id, source);
        context.context.purpose = ExecutionPurpose::MoveAnalysis {
            target: target(&context, id),
        };
        if id == 42 {
            context.context.limits.nodes = 0;
        }
        bridge.submit(context.clone()).unwrap();
        let response = wait(&bridge);
        assert!(response.matches(&context));
        match id {
            40 => assert!(
                matches!(response.observation.result, ExecutionResult::Xiangqi { result: XiangqiResult::Finished { conclusion, ref telemetry } }
                if conclusion == expected && telemetry.score.unwrap() < 0 && telemetry.perspective == xq::Side::Red)
            ),
            41 => assert!(matches!(
                response.observation.result,
                ExecutionResult::Xiangqi {
                    result: XiangqiResult::Choice { .. }
                }
            )),
            42 => assert!(
                matches!(response.observation.result, ExecutionResult::Xiangqi { result: XiangqiResult::Incomplete { ref telemetry } }
                if telemetry.score.is_none() && telemetry.nodes == 0 && telemetry.completed_depth == 0 && telemetry.principal_variation.is_empty())
            ),
            _ => unreachable!(),
        }
    }
    assert_eq!(
        historical_record,
        xq::GameRecord::from_game(&historical).unwrap()
    );
    assert_eq!(fresh_record, xq::GameRecord::from_game(&fresh).unwrap());
    stop(&bridge);
}

#[test]
fn real_child_cancellation_and_cross_game_replacement_retire_search_work() {
    let bridge = bridge(env!("CARGO_BIN_EXE_mshogi"), Duration::from_secs(15));
    let game = xq::Game::standard(xq::ClaimPolicy::Explicit).unwrap();
    let before = xq::GameRecord::from_game(&game).unwrap();
    // First complete an actual child handshake, then cancel maximal native work.
    let warm = request(40, &game);
    bridge.submit(warm.clone()).unwrap();
    assert!(wait(&bridge).matches(&warm));
    let mut old = request(41, &game);
    old.context.limits = WorkLimits {
        depth: mshogi_search::xiangqi::MAX_DEPTH,
        nodes: mshogi_search::xiangqi::MAX_NODES,
    };
    bridge.submit(old.clone()).unwrap();
    thread::sleep(Duration::from_millis(20));
    bridge.cancel();
    assert!(bridge.poll().is_none());
    assert_eq!(bridge.submit(old), Err(SubmitError::IdentityReused));
    let current = chess_request(42);
    bridge.submit(current.clone()).unwrap();
    let response = wait(&bridge);
    assert!(response.matches(&current));
    assert!(matches!(
        response.observation.result,
        ExecutionResult::Chess { .. }
    ));
    // Replace actual Chess work with native Xiangqi analysis; target remains exact.
    let mut retired = chess_request(43);
    retired.context.limits = WorkLimits {
        depth: 6,
        nodes: u64::MAX,
    };
    bridge.submit(retired.clone()).unwrap();
    let mut analysis = request(44, &game);
    analysis.context.purpose = ExecutionPurpose::MoveAnalysis {
        target: target(&analysis, 40),
    };
    bridge.submit(analysis.clone()).unwrap();
    let response = wait(&bridge);
    assert!(response.matches(&analysis));
    assert!(!response.matches(&retired));
    assert!(matches!(
        response.observation.result,
        ExecutionResult::Xiangqi { .. }
    ));
    assert_eq!(before, xq::GameRecord::from_game(&game).unwrap());
    assert!(bridge.poll().is_none());
    stop(&bridge);
}
