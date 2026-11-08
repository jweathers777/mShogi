//! Native Chu completed-route and exact-context contracts via the real line server.
use mshogi_core::{chu as c, native::*};
use mshogi_engine::*;
use std::io::Cursor;
fn game() -> c::Game {
    c::Game::from_root(
        c::Position::from_diagram(
            &[
                (
                    "12l".parse().unwrap(),
                    c::Piece::new(c::Side::Sente, c::Base::King, false).unwrap(),
                ),
                (
                    "1a".parse().unwrap(),
                    c::Piece::new(c::Side::Gote, c::Base::King, false).unwrap(),
                ),
                (
                    "6f".parse().unwrap(),
                    c::Piece::new(c::Side::Sente, c::Base::Lion, false).unwrap(),
                ),
                (
                    "6e".parse().unwrap(),
                    c::Piece::new(c::Side::Gote, c::Base::Pawn, false).unwrap(),
                ),
                (
                    "5e".parse().unwrap(),
                    c::Piece::new(c::Side::Gote, c::Base::Rook, false).unwrap(),
                ),
            ],
            c::Side::Sente,
            c::RootMetadata::diagram(71),
        )
        .unwrap(),
    )
    .unwrap()
}
fn context(g: &c::Game) -> ExecutionContext {
    ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {},
        request_id: 42,
        session_version: 9,
        process_generation: 3,
        game: CanonicalRecord::chu(c::GameRecord::from_game(g).unwrap()),
        limits: CHU_PLAY_LIMITS,
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
    serde_json::from_slice::<ExecutionResponse>(&out)
        .unwrap()
        .observation
}
#[test]
fn compound_result_native_replay_work_and_atomic_validation() {
    let g = game();
    let before = c::GameRecord::from_game(&g).unwrap();
    let ctx = context(&g);
    let obs = execute(&ctx);
    assert!(obs.matches(&ctx));
    let ExecutionResult::Chu { result } = obs.result else {
        panic!()
    };
    result.validate(&g).unwrap();
    let work = result.validate_with_work(&g, &mut || false).unwrap();
    assert!(work > before.reconstruction_work());
    assert!(result.validate_with_work(&g, &mut || true).is_err());
    assert_eq!(c::GameRecord::from_game(&g).unwrap(), before);
    let ChuResult::Choice { action, telemetry } = &result else {
        panic!()
    };
    assert_eq!(action.route.len(), 2);
    assert_eq!(telemetry.perspective, c::Side::Sente);
    assert_eq!(telemetry.score, Some(mshogi_search::chu::WIN_SCORE - 1));
    assert_eq!(
        telemetry.score_label().as_deref(),
        Some("Win in 1 board action")
    );
    let mut forged = result.clone();
    let ChuResult::Choice { action, telemetry } = &mut forged else {
        panic!()
    };
    action.route.reverse();
    telemetry.principal_variation = vec![action.clone()];
    assert!(forged.validate(&g).is_ok());
    let mut forged = result.clone();
    let ChuResult::Choice { action, telemetry } = &mut forged else {
        panic!()
    };
    action.route = vec!["1a".parse().unwrap()];
    telemetry.principal_variation = vec![action.clone()];
    assert!(forged.validate(&g).is_err());
    let mut forged = result.clone();
    let ChuResult::Choice { telemetry, .. } = &mut forged else {
        panic!()
    };
    telemetry.score = Some(123);
    assert!(forged.validate(&g).is_err());
    let mut forged = result.clone();
    let ChuResult::Choice { telemetry, .. } = &mut forged else {
        panic!()
    };
    telemetry.principal_variation[0].route.pop();
    assert!(forged.validate(&g).is_err());
}
#[test]
fn full_context_purpose_history_and_limits_are_never_interchangeable() {
    let g = game();
    let c = context(&g);
    let obs = execute(&c);
    for n in 0..7 {
        let mut wrong = c.clone();
        match n {
            0 => wrong.request_id += 1,
            1 => wrong.session_version += 1,
            2 => wrong.process_generation += 1,
            3 => wrong.limits.nodes += 1,
            4 => wrong.limits.depth += 1,
            5 => {
                wrong.purpose = ExecutionPurpose::MoveAnalysis {
                    target: ExecutionTarget {
                        selection_id: 1,
                        event_index: 0,
                        board_ply: 0,
                        session_version: c.session_version,
                        label: "analyzed native root".into(),
                        turn: NativeSide::Chu(c::Side::Sente),
                    },
                }
            }
            _ => {
                let mut g = game();
                g.submit(c::GameRequest {
                    expected: g.context(),
                    actor: c::Side::Sente,
                    command: c::GameCommand::OfferDraw,
                })
                .unwrap();
                wrong.game = CanonicalRecord::chu(c::GameRecord::from_game(&g).unwrap());
            }
        }
        assert!(!obs.matches(&wrong));
    }
    let mut c = c;
    c.purpose = ExecutionPurpose::MoveAnalysis {
        target: ExecutionTarget {
            selection_id: 7,
            event_index: 0,
            board_ply: 0,
            session_version: c.session_version,
            label: "historical prefix".into(),
            turn: NativeSide::Chu(c::Side::Sente),
        },
    };
    let obs = execute(&c);
    assert!(obs.matches(&c));
    let ExecutionResult::Chu { result } = &obs.result else {
        panic!()
    };
    result.validate(&g).unwrap();
    assert_eq!(g.events().len(), 0);
}
#[test]
fn zero_budget_incomplete_and_native_agreement_are_distinct() {
    let mut g = game();
    let mut c = context(&g);
    c.limits.nodes = 0;
    let obs = execute(&c);
    assert!(obs.matches(&c));
    let ExecutionResult::Chu {
        result: ChuResult::Incomplete { telemetry },
    } = &obs.result
    else {
        panic!("{obs:?}")
    };
    assert_eq!(telemetry.score, None);
    assert_eq!(telemetry.nodes, 0);
    assert_eq!(telemetry.status, StopReason::NodeLimit);
    for (actor, command) in [
        (c::Side::Sente, c::GameCommand::OfferDraw),
        (c::Side::Gote, c::GameCommand::AcceptDraw),
    ] {
        g.submit(c::GameRequest {
            expected: g.context(),
            actor,
            command,
        })
        .unwrap();
    }
    let ctx = context(&g);
    let obs = execute(&ctx);
    assert!(obs.matches(&ctx));
    let ExecutionResult::Chu { result } = &obs.result else {
        panic!()
    };
    assert!(matches!(
        result,
        ChuResult::Finished {
            conclusion: c::Conclusion::AgreedDraw,
            ..
        }
    ));
    result.validate(&g).unwrap();
    assert_eq!(result.telemetry().score, Some(0));
}
#[test]
fn historical_unsupported_failures_survive_and_current_limits_stay_finite() {
    let mut c = context(&game());
    c.limits = WorkLimits {
        time_ms: None,
        depth: CHU_MAX_DEPTH + 1,
        nodes: CHU_MAX_NODES + 1,
    };
    assert!(c.valid_limits());
    let old = ExecutionObservation::new(
        c.clone(),
        ExecutionResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::UnsupportedMode,
                message: "Chu execution is not implemented".into(),
            },
        },
    );
    assert!(old.matches(&c));
    let round: ExecutionObservation =
        serde_json::from_str(&serde_json::to_string(&old).unwrap()).unwrap();
    assert_eq!(old, round);
    let now = execute(&c);
    assert!(now.matches(&c));
    assert!(matches!(
        now.result,
        ExecutionResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::InvalidRequest,
                ..
            }
        }
    ));
    let mut c = context(&game());
    c.purpose = ExecutionPurpose::ChessFullReachability {
        target: ExecutionTarget {
            selection_id: 1,
            event_index: 0,
            board_ply: 0,
            session_version: c.session_version,
            label: "not Chu analysis".into(),
            turn: NativeSide::Chu(c::Side::Sente),
        },
        mate: ReachabilityTarget::White,
        limits: FullLimits {
            max_positions: 1,
            max_transitions: 1,
        },
    };
    c.limits = FULL_SENTINEL;
    assert!(matches!(
        execute(&c).result,
        ExecutionResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::UnsupportedMode,
                ..
            }
        }
    ));
}
#[test]
fn bounded_standard_progression_has_both_sides_real_legal_choices_and_exact_records() {
    let mut g = c::Game::standard();
    let mut observations = vec![];
    for ply in 0..12 {
        assert_eq!(
            g.position().side_to_move(),
            if ply % 2 == 0 {
                c::Side::Sente
            } else {
                c::Side::Gote
            }
        );
        let c = context(&g);
        let obs = execute(&c);
        assert!(obs.matches(&c));
        let ExecutionResult::Chu { result } = &obs.result else {
            panic!("{obs:?}")
        };
        result.validate(&g).unwrap();
        let ChuResult::Choice { action, .. } = result else {
            panic!("{result:?}")
        };
        let action = action.clone();
        assert!(g.legal_actions().contains(&action));
        observations.push(obs);
        g.submit(c::GameRequest {
            expected: g.context(),
            actor: g.position().side_to_move(),
            command: c::GameCommand::Play { action },
        })
        .unwrap();
        let r = c::GameRecord::from_game(&g).unwrap();
        assert_eq!(c::GameRecord::from_game(&r.to_game().unwrap()).unwrap(), r);
    }
    assert_eq!(g.events().len(), 12);
    assert_eq!(observations.len(), 12);
    assert_eq!(g.conclusion(), None);
    let bytes = serde_json::to_vec(&observations).unwrap();
    let round: Vec<ExecutionObservation> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(round, observations);
    eprintln!(
        "12-ply standard Chu smoke: native_bytes={} observation_bytes={} native_work={}",
        c::GameRecord::from_game(&g).unwrap().as_json().len(),
        bytes.len(),
        g.replay_work()
    );
}
