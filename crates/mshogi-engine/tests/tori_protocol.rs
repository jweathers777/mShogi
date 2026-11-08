//! Actual native Tori result contract through the real one-shot transport.
use mshogi_core::{native::*, tori as t};
use mshogi_engine::*;
use mshogi_search::tori as search;
use serde_json::{Value, json};
use std::io::Cursor;
fn context(game: &t::Game) -> ExecutionContext {
    ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {},
        request_id: 42,
        session_version: 9,
        process_generation: 3,
        game: CanonicalRecord::tori(t::GameRecord::from_game(game).unwrap()),
        limits: WorkLimits {
            time_ms: None,
            depth: 1,
            nodes: 1000,
        },
    }
}
fn execute(c: &ExecutionContext) -> ExecutionObservation {
    let mut out = vec![];
    serve_once(
        Cursor::new(format!(
            "{}\n",
            serde_json::to_string(&ExecutionRequest {
                protocol: PROTOCOL_VERSION,
                context: c.clone()
            })
            .unwrap()
        )),
        &mut out,
    )
    .unwrap();
    let r: ExecutionResponse = serde_json::from_slice(&out).unwrap();
    assert_eq!(&r.observation.context, c);
    r.observation
}
fn game(turn: t::Side) -> t::Game {
    t::Game::from_root(
        t::Position::from_diagram(
            &[
                (
                    "4g".parse().unwrap(),
                    t::Piece {
                        side: t::Side::Sente,
                        face: t::Face::Phoenix,
                    },
                ),
                (
                    "4a".parse().unwrap(),
                    t::Piece {
                        side: t::Side::Gote,
                        face: t::Face::Phoenix,
                    },
                ),
                (
                    "7c".parse().unwrap(),
                    t::Piece {
                        side: t::Side::Sente,
                        face: t::Face::Falcon,
                    },
                ),
                (
                    "7b".parse().unwrap(),
                    t::Piece {
                        side: t::Side::Gote,
                        face: t::Face::Swallow,
                    },
                ),
            ],
            [t::Hand::empty(); 2],
            turn,
            t::RootMetadata::diagram(71),
        )
        .unwrap(),
    )
    .unwrap()
}
#[test]
fn actual_choices_admit_and_scores_purpose_and_identity_remain_native() {
    let mut game = game(t::Side::Sente);
    for side in [t::Side::Sente, t::Side::Gote] {
        let c = context(&game);
        let observation = execute(&c);
        assert!(observation.matches(&c));
        let ExecutionResult::Tori {
            result: ToriResult::Choice { action, telemetry },
        } = &observation.result
        else {
            panic!("{observation:?}")
        };
        assert_eq!(telemetry.perspective, side);
        assert!(telemetry.score_label().is_some());
        if side == t::Side::Sente {
            assert_eq!(
                *action,
                t::Action::Move {
                    from: "7c".parse().unwrap(),
                    to: "7b".parse().unwrap(),
                    result_face: t::Face::Eagle
                }
            );
            assert_eq!(telemetry.score, Some(1100));
        }
        for n in 0..6 {
            let mut wrong = c.clone();
            match n {
                0 => wrong.request_id += 1,
                1 => wrong.session_version += 1,
                2 => wrong.process_generation += 1,
                3 => wrong.limits.nodes += 1,
                4 => wrong.limits.depth += 1,
                _ => {
                    wrong.purpose = ExecutionPurpose::MoveAnalysis {
                        target: ExecutionTarget {
                            selection_id: 1,
                            event_index: wrong.game.event_count() as u64,
                            board_ply: wrong.game.known_board_actions() as u64,
                            session_version: wrong.session_version,
                            label: "selected root".into(),
                            turn: NativeSide::Tori(side),
                        },
                    }
                }
            }
            assert!(!observation.matches(&wrong));
        }
        let before = game.snapshot().clone();
        game.submit(t::GameRequest {
            expected: game.context(),
            actor: side,
            command: t::GameCommand::Play { action: *action },
        })
        .unwrap();
        assert_ne!(before, game.snapshot().clone());
    }
    let mut c = context(&game);
    c.purpose = ExecutionPurpose::MoveAnalysis {
        target: ExecutionTarget {
            selection_id: 7,
            event_index: c.game.event_count() as u64,
            board_ply: c.game.known_board_actions() as u64,
            session_version: c.session_version,
            label: "exact Tori selected event".into(),
            turn: NativeSide::Tori(game.position().side_to_move()),
        },
    };
    let before = game.snapshot().clone();
    let obs = execute(&c);
    assert!(obs.matches(&c));
    assert_eq!(game.snapshot().clone(), before);
}
#[test]
fn native_terminal_and_no_completed_work_are_distinct() {
    let mut g = game(t::Side::Sente);
    g.submit(t::GameRequest {
        expected: g.context(),
        actor: t::Side::Gote,
        command: t::GameCommand::Resign,
    })
    .unwrap();
    let c = context(&g);
    let obs = execute(&c);
    assert!(obs.matches(&c));
    assert!(matches!(
        obs.result,
        ExecutionResult::Tori {
            result: ToriResult::Finished {
                conclusion: t::Conclusion::Resignation {
                    winner: t::Side::Sente
                },
                ..
            }
        }
    ));
    let mut c = context(&t::Game::standard());
    c.limits.nodes = 1;
    let obs = execute(&c);
    assert!(obs.matches(&c));
    let ExecutionResult::Tori {
        result: ToriResult::Incomplete { telemetry },
    } = obs.result
    else {
        panic!()
    };
    assert_eq!(telemetry.score, None);
    assert_eq!(telemetry.completed_depth, 0);
    assert_eq!(telemetry.nodes, 1);
    assert_eq!(telemetry.status, StopReason::NodeLimit);
}
#[test]
fn malformed_telemetry_complete_pv_and_terminal_mismatches_refuse() {
    let g = game(t::Side::Sente);
    let c = context(&g);
    let good = execute(&c);
    assert!(good.matches(&c));
    for n in 0..10 {
        let mut obs = good.clone();
        let ExecutionResult::Tori {
            result: ToriResult::Choice { action, telemetry },
        } = &mut obs.result
        else {
            panic!()
        };
        match n {
            0 => telemetry.perspective = t::Side::Gote,
            1 => telemetry.search_id = "wrong".into(),
            2 => telemetry.evaluator_id = "wrong".into(),
            3 => telemetry.score = Some(i32::MIN),
            4 => telemetry.completed_depth = 0,
            5 => telemetry.nodes = c.limits.nodes + 1,
            6 => telemetry.principal_variation.clear(),
            7 => {
                *action = t::Action::Move {
                    from: "1a".parse().unwrap(),
                    to: "1b".parse().unwrap(),
                    result_face: t::Face::Phoenix,
                }
            }
            8 => telemetry.score = Some(0),
            _ => telemetry.principal_variation.push(*action),
        }
        let ExecutionResult::Tori { result } = &obs.result else {
            panic!()
        };
        if n != 5 {
            assert!(result.validate(&g).is_err(), "native validation case {n}");
        } else {
            assert!(
                result.validate(&g).is_ok(),
                "submitted work limit belongs to contextual shape check"
            );
        }
        if n != 8 {
            assert!(!obs.matches(&c), "cheap shape case {n}");
        } else {
            assert!(
                obs.matches(&c),
                "leafscore validation belongs to bounded native worker"
            );
        }
    }
    let bytes = serde_json::to_string(&good).unwrap();
    for replacement in ["\"score\":0,\"score\":1100", "\"score\":null,\"unknown\":0"] {
        let changed = bytes.replace("\"score\":1100", replacement);
        assert!(serde_json::from_str::<ExecutionObservation>(&changed).is_err());
    }
    let mut v: Value = serde_json::from_str(&bytes).unwrap();
    v["result"]["result"]["telemetry"]
        .as_object_mut()
        .unwrap()
        .remove("score");
    assert!(serde_json::from_value::<ExecutionObservation>(v).is_err());
}
#[test]
fn full_remains_explicit_unscored_unsupported_and_limits_are_bounded() {
    let mut c = context(&t::Game::standard());
    c.limits = FULL_SENTINEL;
    c.purpose = ExecutionPurpose::ChessFullReachability {
        target: ExecutionTarget {
            selection_id: 1,
            event_index: 0,
            board_ply: 0,
            session_version: c.session_version,
            label: "not Tori Full".into(),
            turn: NativeSide::Tori(t::Side::Sente),
        },
        mate: ReachabilityTarget::Either,
        limits: FullLimits {
            max_positions: 1,
            max_transitions: 1,
        },
    };
    let obs = execute(&c);
    assert!(matches!(
        obs.result,
        ExecutionResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::UnsupportedMode,
                ..
            }
        }
    ));
    assert!(!serde_json::to_string(&obs).unwrap().contains("\"score\""));
    for limits in [
        WorkLimits {
            time_ms: None,
            depth: 0,
            nodes: 1,
        },
        WorkLimits {
            time_ms: None,
            depth: search::MAX_DEPTH + 1,
            nodes: 1,
        },
        WorkLimits {
            time_ms: None,
            depth: 1,
            nodes: search::MAX_NODES + 1,
        },
    ] {
        let mut c = context(&t::Game::standard());
        c.limits = limits;
        if limits.depth > 0 {
            assert!(c.valid_limits(), "legacy failure context remains parseable");
        } else {
            assert!(!c.valid_limits());
        }
        assert!(matches!(
            execute(&c).result,
            ExecutionResult::Failure {
                failure: EngineFailure {
                    kind: FailureKind::InvalidRequest,
                    ..
                }
            }
        ));
    }
    let v = json!({"kind":"tori","result":{"kind":"incomplete","telemetry":{"evaluator_id":search::EVALUATOR_ID,"search_id":search::SEARCH_ID,"perspective":"sente","score":null,"completed_depth":0,"nodes":0,"principal_variation":[],"status":"stopped"}}});
    assert!(serde_json::from_value::<ExecutionResult>(v).is_ok());
}

#[test]
fn same_board_full_history_and_forbidden_departure_require_native_worker_validation() {
    let root = t::Position::from_diagram(
        &[
            (
                "4g".parse().unwrap(),
                t::Piece {
                    side: t::Side::Sente,
                    face: t::Face::Phoenix,
                },
            ),
            (
                "4a".parse().unwrap(),
                t::Piece {
                    side: t::Side::Gote,
                    face: t::Face::Phoenix,
                },
            ),
        ],
        [t::Hand::empty(); 2],
        t::Side::Sente,
        t::RootMetadata::diagram(0),
    )
    .unwrap();
    let fresh = t::Game::from_root(root.clone()).unwrap();
    let mut repeated = t::Game::from_root(root).unwrap();
    for (from, to) in [
        ("4g", "5g"),
        ("4a", "5a"),
        ("5g", "4g"),
        ("5a", "4a"),
        ("4g", "5g"),
        ("4a", "5a"),
        ("5g", "4g"),
        ("5a", "4a"),
    ] {
        repeated
            .submit(t::GameRequest {
                expected: repeated.context(),
                actor: repeated.position().side_to_move(),
                command: t::GameCommand::Play {
                    action: t::Action::Move {
                        from: from.parse().unwrap(),
                        to: to.parse().unwrap(),
                        result_face: t::Face::Phoenix,
                    },
                },
            })
            .unwrap();
    }
    assert_eq!(fresh.repetition_key(), repeated.repetition_key());
    let c = context(&repeated);
    let good = execute(&c);
    assert!(good.matches(&c));
    let ExecutionResult::Tori { result } = &good.result else {
        panic!()
    };
    assert!(result.validate(&repeated).is_ok());
    let mut same_board = context(&fresh);
    same_board.request_id = c.request_id;
    assert!(!good.matches(&same_board));
    let forbidden = t::Action::Move {
        from: "4g".parse().unwrap(),
        to: "5g".parse().unwrap(),
        result_face: t::Face::Phoenix,
    };
    assert!(fresh.legal_actions().contains(&forbidden));
    assert!(!repeated.legal_actions().contains(&forbidden));
    let mut forged = good.clone();
    let ExecutionResult::Tori {
        result: ToriResult::Choice { action, telemetry },
    } = &mut forged.result
    else {
        panic!()
    };
    *action = forbidden;
    telemetry.principal_variation = vec![forbidden];
    // The host poller performs cheap shape/context checks; it must not replay.
    assert!(forged.matches(&c));
    let ExecutionResult::Tori { result } = &forged.result else {
        panic!()
    };
    assert!(result.validate(&repeated).is_err());
}
#[test]
fn historical_unscored_failure_contexts_survive_new_bounded_execution_limits() {
    let mut c = context(&t::Game::standard());
    c.limits = WorkLimits {
        time_ms: None,
        depth: search::MAX_DEPTH + 1,
        nodes: search::MAX_NODES + 1,
    };
    assert!(c.valid_limits());
    let failure = ExecutionObservation::new(
        c.clone(),
        ExecutionResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::UnsupportedMode,
                message: "Tori engine execution is not implemented".into(),
            },
        },
    );
    assert!(failure.matches(&c));
    let restored: ExecutionObservation =
        serde_json::from_str(&serde_json::to_string(&failure).unwrap()).unwrap();
    assert_eq!(failure, restored);
    assert!(restored.matches(&c));
    let current = execute(&c);
    assert!(current.matches(&c));
    assert!(matches!(
        current.result,
        ExecutionResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::InvalidRequest,
                ..
            }
        }
    ));
}
