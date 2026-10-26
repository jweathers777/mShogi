//! Real supervised native child play/analysis; separate tests, never packaged.
use mshogi_core::{native::*, shogi::*};
use mshogi_engine::*;
use mshogi_search::shogi as search;
use serde_json::json;
use std::{
    thread,
    time::{Duration, Instant},
};
fn bridge() -> EngineBridge {
    EngineBridge::with_options(
        env!("CARGO_BIN_EXE_mshogi").into(),
        BridgeOptions {
            watchdog: Duration::from_secs(30),
            cleanup: Duration::from_secs(1),
        },
    )
    .unwrap()
}
fn request(game: &Game, id: u64) -> SharedRequest {
    SharedRequest {
        protocol: SHARED_PROTOCOL_VERSION,
        purpose: SharedPurpose::EngineTurn {},
        request_id: id,
        session_version: id,
        process_generation: id,
        game: CanonicalRecord::shogi(GameRecord::from_game(game)),
        limits: WorkLimits {
            depth: search::DEFAULT_LIMITS.depth,
            nodes: search::DEFAULT_LIMITS.nodes,
        },
    }
}
fn wait(bridge: &EngineBridge) -> SharedResponse {
    let deadline = Instant::now() + Duration::from_secs(35);
    loop {
        if let Some(response) = bridge.poll_shared() {
            return response;
        }
        assert!(Instant::now() < deadline, "actual shogi child deadline");
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
fn actual_child_plays_both_assignments_with_native_admission_and_recorded_observations() {
    let mut evidence = vec![];
    for human in [Side::Sente, Side::Gote] {
        let bridge = bridge();
        let mut game = Game::new();
        let engine = human.opponent();
        let mut observations = vec![];
        for _ in 0..4 {
            if game.position().side_to_move() == human {
                let action = game.position().legal_actions()[0];
                game.submit(GameRequest {
                    expected: game.context(),
                    actor: human,
                    command: GameCommand::Play(action),
                })
                .unwrap();
            } else {
                let c = request(&game, observations.len() as u64 + 1);
                bridge.submit_shared(c.clone()).unwrap();
                let response = wait(&bridge);
                assert!(response.matches(&c));
                let SharedResult::Shogi {
                    result: ShogiResult::Choice { choice, telemetry },
                } = &response.result
                else {
                    panic!("{response:?}")
                };
                assert_eq!(telemetry.perspective, engine);
                assert!(telemetry.completed_depth > 0);
                assert!(telemetry.nodes <= c.limits.nodes);
                let selected: search::Choice = (*choice).into();
                let actor = match selected {
                    search::Choice::Move(_) => engine,
                    search::Choice::Declare { actor } => actor,
                };
                game.submit(GameRequest {
                    expected: game.context(),
                    actor,
                    command: selected.command(),
                })
                .unwrap();
                observations.push(response);
            }
        }
        assert_eq!(observations.len(), 2);
        assert_eq!(game.events().len(), 4);
        evidence.push(json!({"human":human,"engine":engine,"observations":observations,"final_game":CanonicalRecord::shogi(GameRecord::from_game(&game))}));
        stop(&bridge);
    }
    println!(
        "SHOGI_PROCESS_EVIDENCE {}",
        json!({"format":"mshogi.shogi.engine-smoke.v1","method":"actual internal-search child, EngineBridge, native Game::submit","cases":evidence})
    );
}
#[test]
fn actual_analysis_declaration_incomplete_and_history_contexts_remain_distinct() {
    let bridge = bridge();
    let root = GameRoot::import(
        Position::from_sfen("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1").unwrap(),
        RootCategory::ReducedMaterialDiagram,
        CounterSource::Supplied,
    )
    .unwrap();
    let mut game = Game::from_root(root);
    let before = game.clone();
    let mut c = request(&game, 1);
    c.purpose = SharedPurpose::MoveAnalysis {
        target: SharedAnalysisTarget {
            selection_id: 1,
            event_index: 0,
            board_ply: 0,
            session_version: 1,
            label: "Declaration target".into(),
            turn: NativeSide::Shogi(Side::Sente),
        },
    };
    bridge.submit_shared(c.clone()).unwrap();
    let analysis = wait(&bridge);
    assert!(analysis.matches(&c));
    assert_eq!(game, before, "analysis proposal is not applied");
    assert!(matches!(
        analysis.result,
        SharedResult::Shogi {
            result: ShogiResult::Choice {
                choice: ShogiChoice::Declare { actor: Side::Sente },
                ..
            }
        }
    ));
    let mut play = request(&game, 2);
    play.limits.depth = 1;
    assert!(!analysis.matches(&play));
    bridge.submit_shared(play.clone()).unwrap();
    let response = wait(&bridge);
    assert!(response.matches(&play));
    let SharedResult::Shogi {
        result: ShogiResult::Choice { choice, .. },
    } = response.result
    else {
        panic!()
    };
    let choice: search::Choice = choice.into();
    game.submit(GameRequest {
        expected: game.context(),
        actor: Side::Sente,
        command: choice.command(),
    })
    .unwrap();
    assert_eq!(game.position(), before.position());
    assert_eq!(game.events().len(), 1);
    let mut zero = request(&game, 3);
    zero.limits.nodes = 0;
    bridge.submit_shared(zero.clone()).unwrap();
    let response = wait(&bridge);
    assert!(response.matches(&zero));
    assert!(matches!(
        response.result,
        SharedResult::Shogi {
            result: ShogiResult::Incomplete {
                telemetry: ShogiTelemetry {
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
fn actual_cross_game_cancel_replacement_and_historical_selection_retire_old_work() {
    let bridge = bridge();
    let root = GameRoot::import(
        Position::from_sfen("8k/9/9/9/9/9/9/9/K8 b RBGSNLPrbgsnlp 1").unwrap(),
        RootCategory::ReducedMaterialDiagram,
        CounterSource::Supplied,
    )
    .unwrap();
    let game = Game::from_root(root);
    let mut old = request(&game, 1);
    old.limits = WorkLimits {
        depth: search::MAX_DEPTH,
        nodes: search::MAX_NODES,
    };
    bridge.submit_shared(old.clone()).unwrap();
    thread::sleep(Duration::from_millis(20));
    bridge.cancel();
    assert!(bridge.poll_shared().is_none());
    assert_eq!(bridge.submit_shared(old), Err(SubmitError::IdentityReused));
    let live = Game::new();
    let mut analysis = request(&live, 2);
    analysis.purpose = SharedPurpose::MoveAnalysis {
        target: SharedAnalysisTarget {
            selection_id: 2,
            event_index: 0,
            board_ply: 0,
            session_version: 2,
            label: "Replacement selection".into(),
            turn: NativeSide::Shogi(Side::Sente),
        },
    };
    analysis.limits = WorkLimits {
        depth: 1,
        nodes: 1000,
    };
    bridge.submit_shared(analysis.clone()).unwrap();
    let response = wait(&bridge);
    assert!(response.matches(&analysis));
    let mut changed = analysis.clone();
    if let SharedPurpose::MoveAnalysis { target } = &mut changed.purpose {
        target.selection_id = 3;
    }
    assert!(!response.matches(&changed));
    bridge.cancel();
    assert!(bridge.poll_shared().is_none());
    let chess = EngineRequest {
        protocol: PROTOCOL_VERSION,
        purpose: RequestPurpose::EngineTurn,
        request_id: 3,
        session_version: 3,
        process_generation: 3,
        game: mshogi_core::chess::AdjudicatedGame::new(
            mshogi_core::chess::ClaimPolicy::ExplicitOnly,
        )
        .record(),
        limits: WorkLimits {
            depth: 1,
            nodes: 1000,
        },
    };
    bridge.submit(chess.clone()).unwrap();
    let deadline = Instant::now() + Duration::from_secs(35);
    loop {
        assert!(bridge.poll_shared().is_none());
        if let Some(response) = bridge.poll() {
            assert_eq!(response.context, chess);
            assert!(matches!(response.result, EngineResult::Move { .. }));
            break;
        }
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
    }
    stop(&bridge);
}
