//! Concrete shogi v3 admission/result boundaries; no second rules engine.
use mshogi_core::{native::*, shogi::*};
use mshogi_engine::*;
use mshogi_search::shogi as search;
use serde_json::{Value, json};
use std::io::Cursor;
fn game(sfen: &str) -> Game {
    Game::from_root(
        GameRoot::import(
            Position::from_sfen(sfen).unwrap(),
            RootCategory::ReducedMaterialDiagram,
            CounterSource::Supplied,
        )
        .unwrap(),
    )
}
fn request(game: &Game) -> SharedRequest {
    SharedRequest {
        protocol: SHARED_PROTOCOL_VERSION,
        purpose: SharedPurpose::EngineTurn {},
        request_id: 1,
        session_version: 7,
        process_generation: 3,
        game: CanonicalRecord::shogi(GameRecord::from_game(game)),
        limits: WorkLimits {
            depth: 1,
            nodes: 5000,
        },
    }
}
fn target(c: &SharedRequest, side: Side) -> SharedAnalysisTarget {
    SharedAnalysisTarget {
        selection_id: 1,
        event_index: c.game.event_count() as u64,
        board_ply: c.game.known_board_actions() as u64,
        session_version: c.session_version,
        label: "Native shogi prefix".into(),
        turn: NativeSide::Shogi(side),
    }
}
fn raw(text: &str) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();
    serve_once(Cursor::new(format!("{text}\n")), &mut output)?;
    Ok(output)
}
fn run(c: SharedRequest) -> SharedResponse {
    let bytes = raw(&serde_json::to_string(&c).unwrap()).unwrap();
    let response: SharedResponse = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(response.context, c);
    response
}
#[test]
fn real_native_choices_conclusions_and_score_perspective_roundtrip() {
    for (sfen, expected) in [
        (
            "8k/9/9/4p4/4R4/9/9/9/K8 b - 1",
            ShogiChoice::Move {
                action: "5e5d".parse().unwrap(),
            },
        ),
        (
            "3lkl3/3p1p3/4K4/9/9/9/9/9/9 b G 1",
            ShogiChoice::Move {
                action: "G*5b".parse().unwrap(),
            },
        ),
        (
            "K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1",
            ShogiChoice::Declare { actor: Side::Sente },
        ),
    ] {
        let mut game = game(sfen);
        let c = request(&game);
        let response = run(c.clone());
        assert!(response.matches(&c));
        let SharedResult::Shogi {
            result: ShogiResult::Choice { choice, telemetry },
        } = response.result
        else {
            panic!("{response:?}")
        };
        assert_eq!(choice, expected);
        assert_eq!(telemetry.perspective, Side::Sente);
        assert_eq!(telemetry.evaluator_id, search::EVALUATOR_ID);
        assert_eq!(telemetry.search_id, search::SEARCH_ID);
        assert_eq!(telemetry.completed_depth, 1);
        let selected: search::Choice = choice.into();
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: selected.command(),
        })
        .unwrap();
        let c = request(&game);
        let response = run(c.clone());
        assert!(response.matches(&c));
        if matches!(choice, ShogiChoice::Declare { .. }) {
            assert!(matches!(
                response.result,
                SharedResult::Shogi {
                    result: ShogiResult::Finished {
                        conclusion: Conclusion::Win {
                            winner: Side::Sente,
                            reason: WinReason::EnteringKing
                        },
                        telemetry: ShogiTelemetry {
                            score: Some(search::WIN_SCORE),
                            perspective: Side::Sente,
                            ..
                        }
                    }
                }
            ));
        }
    }
}
#[test]
fn analysis_is_read_only_and_bound_to_native_side_and_same_ply_terminal_prefix() {
    let mut game = Game::new();
    game.submit(GameRequest {
        expected: game.context(),
        actor: Side::Gote,
        command: GameCommand::Resign,
    })
    .unwrap();
    let before = game.clone();
    let mut c = request(&game);
    c.purpose = SharedPurpose::MoveAnalysis {
        target: target(&c, Side::Sente),
    };
    let response = run(c.clone());
    assert!(response.matches(&c));
    assert_eq!(game, before);
    assert_eq!(c.game.event_count(), 1);
    assert_eq!(c.game.known_board_actions(), 0);
    assert!(matches!(
        response.result,
        SharedResult::Shogi {
            result: ShogiResult::Finished { .. }
        }
    ));
    let value = serde_json::to_value(&c).unwrap();
    for (path, v) in [
        ("/request_id", json!(2)),
        ("/session_version", json!(8)),
        ("/process_generation", json!(4)),
        ("/limits/nodes", json!(2)),
        ("/purpose/target/selection_id", json!(2)),
        ("/purpose/target/event_index", json!(0)),
        ("/purpose/target/board_ply", json!(1)),
        ("/purpose/target/turn/side", json!("gote")),
    ] {
        let mut changed = value.clone();
        *changed.pointer_mut(path).unwrap() = v;
        let changed: SharedRequest = serde_json::from_value(changed).unwrap();
        assert!(!response.matches(&changed), "{path}");
    }
    let mut wrong = c.clone();
    wrong.purpose = SharedPurpose::MoveAnalysis {
        target: target(&wrong, Side::Gote),
    };
    assert!(matches!(
        run(wrong).result,
        SharedResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::InvalidRequest,
                ..
            }
        }
    ));
    let mut turn = c;
    turn.purpose = SharedPurpose::EngineTurn {};
    assert!(!response.matches(&turn));
}
#[test]
fn precise_limits_and_incomplete_work_do_not_invent_scores_or_moves() {
    for (nodes, depth, completed, status) in [
        (0, 1, 0, StopReason::NodeLimit),
        (1, 1, 0, StopReason::NodeLimit),
        (31, 1, 1, StopReason::Complete),
        (32, 2, 1, StopReason::NodeLimit),
    ] {
        let mut c = request(&Game::new());
        c.limits = WorkLimits { depth, nodes };
        let response = run(c.clone());
        assert!(response.matches(&c));
        let SharedResult::Shogi { result } = response.result else {
            panic!()
        };
        let t = match result {
            ShogiResult::Incomplete { telemetry } => {
                assert_eq!(completed, 0);
                assert_eq!(telemetry.score, None);
                assert!(telemetry.principal_variation.is_empty());
                telemetry
            }
            ShogiResult::Choice { telemetry, .. } => {
                assert!(completed > 0);
                assert_eq!(telemetry.score, Some(0));
                telemetry
            }
            _ => panic!(),
        };
        assert_eq!(t.completed_depth, completed);
        assert_eq!(t.nodes, nodes);
        assert_eq!(t.status, status);
    }
    for limits in [
        WorkLimits { depth: 0, nodes: 1 },
        WorkLimits {
            depth: search::MAX_DEPTH + 1,
            nodes: 1,
        },
        WorkLimits {
            depth: 1,
            nodes: search::MAX_NODES + 1,
        },
    ] {
        let mut c = request(&Game::new());
        c.limits = limits;
        assert!(!c.valid_limits());
        assert!(matches!(
            run(c).result,
            SharedResult::Failure {
                failure: EngineFailure {
                    kind: FailureKind::InvalidRequest,
                    ..
                }
            }
        ));
    }
}
#[test]
fn capabilities_and_result_families_do_not_make_chess_full_a_shogi_operation() {
    assert_eq!(
        execution_capabilities(GameKind::Shogi),
        &[
            ExecutionOperation::EngineTurn,
            ExecutionOperation::MoveAnalysis
        ]
    );
    let mut c = request(&Game::new());
    let ordinary = run(c.clone());
    c.purpose = SharedPurpose::ChessFullReachability {
        target: target(&c, Side::Sente),
        mate: ReachabilityTarget::Either,
        limits: FullLimits {
            max_positions: 10,
            max_transitions: 10,
        },
    };
    c.limits = FULL_SENTINEL;
    let response = run(c.clone());
    assert!(response.matches(&c));
    assert!(matches!(
        response.result,
        SharedResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::UnsupportedMode,
                ..
            }
        }
    ));
    let text = serde_json::to_string(&response).unwrap();
    assert!(!text.contains("\"score\""));
    assert!(!text.contains("\"telemetry\""));
    let mut forged = ordinary;
    forged.context = c.clone();
    assert!(!forged.matches(&c));
    let chess = CanonicalRecord::chess(
        mshogi_core::chess::AdjudicatedGame::new(mshogi_core::chess::ClaimPolicy::ExplicitOnly)
            .record(),
    );
    c.purpose = SharedPurpose::EngineTurn {};
    c.limits = WorkLimits {
        depth: 1,
        nodes: 1000,
    };
    c.game = chess;
    forged.context = c.clone();
    assert!(!forged.matches(&c));
}
#[test]
fn strict_result_parsing_and_native_observation_acceptance() {
    let c = request(&Game::new());
    let response = run(c.clone());
    let text = serde_json::to_string(&response).unwrap();
    for (a, b) in [
        ("\"nodes\":31", "\"nodes\":31,\"nodes\":31"),
        (
            "\"action\":\"1g1f\"",
            "\"action\":\"1g1f\",\"action\":\"1g1f\"",
        ),
        (
            "\"perspective\":\"sente\"",
            "\"perspective\":\"sente\",\"unknown\":true",
        ),
    ] {
        assert!(text.contains(a));
        assert!(serde_json::from_str::<SharedResponse>(&text.replacen(a, b, 1)).is_err());
    }
    let value = serde_json::to_value(&response).unwrap();
    for (path, v) in [
        ("/result/result/telemetry/perspective", json!("gote")),
        ("/result/result/telemetry/evaluator_id", json!("other")),
        ("/result/result/telemetry/search_id", json!("other")),
        ("/result/result/telemetry/nodes", json!(5001)),
        ("/result/result/telemetry/completed_depth", json!(2)),
        ("/result/result/telemetry/score", Value::Null),
        ("/result/result/choice/action", json!("1g1f+")),
    ] {
        let mut bad = value.clone();
        *bad.pointer_mut(path).unwrap() = v;
        let parsed: SharedResponse = serde_json::from_value(bad).unwrap();
        assert!(!parsed.matches(&c), "{path}");
    }
    let mut missing = value;
    missing["result"]["result"]["telemetry"]
        .as_object_mut()
        .unwrap()
        .remove("score");
    assert!(serde_json::from_value::<SharedResponse>(missing).is_err());
    for text in ["\"resign\"", "\"win\"", "\"0000\""] {
        assert!(serde_json::from_str::<Action>(text).is_err());
    }
    for text in [
        "{\"kind\":\"length\",\"extra\":1}",
        "{\"kind\":\"resignation\",\"winner\":\"sente\",\"winner\":\"sente\"}",
        "{\"kind\":\"resignation\",\"winner\":\"white\"}",
    ] {
        assert!(serde_json::from_str::<Conclusion>(text).is_err());
    }
}
#[test]
fn raw_request_routing_keeps_native_duplicates_and_unknowns_visible() {
    let c = request(&Game::new());
    let text = serde_json::to_string(&c).unwrap();
    for (a, b) in [
        (
            "\"profile_id\":",
            "\"profile_id\":\"mshogi.shogi.csa36.v1\",\"profile_id\":",
        ),
        (
            "\"schema_version\":1",
            "\"schema_version\":1,\"schema_version\":1",
        ),
        (
            "\"kind\":\"engine_turn\"",
            "\"kind\":\"engine_turn\",\"unknown\":1",
        ),
    ] {
        assert!(text.contains(a));
        assert!(raw(&text.replacen(a, b, 1)).is_err());
    }
}
