//! Lossless decisions against independently specified chess facts.
use mshogi_core::chess::*;
use mshogi_engine::*;
use std::io::Cursor;
fn request(game: &AdjudicatedGame) -> EngineRequest {
    EngineRequest {
        protocol: PROTOCOL_VERSION,
        purpose: RequestPurpose::EngineTurn,
        request_id: 1,
        session_version: 7,
        process_generation: 3,
        game: game.record(),
        limits: WorkLimits {
            depth: 1,
            nodes: 10_000,
        },
    }
}
fn run(context: EngineRequest) -> EngineResponse {
    let mut bytes = serde_json::to_vec(&context).unwrap();
    bytes.push(b'\n');
    let mut output = Vec::new();
    serve_once(Cursor::new(bytes), &mut output).unwrap();
    let response: EngineResponse = serde_json::from_slice(&output).unwrap();
    assert_eq!(response.context, context);
    response
}
#[test]
fn moves_claims_conclusions_and_incomplete_work_are_distinct() {
    let mate =
        AdjudicatedGame::from_fen("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", ClaimPolicy::ExplicitOnly)
            .unwrap();
    let response = run(request(&mate));
    let EngineResult::Move { action, telemetry } = response.result else {
        panic!("{response:?}")
    };
    assert_eq!(action.to_string(), "g6g7");
    assert_eq!(telemetry.score, Some(999_999));
    assert_eq!(telemetry.completed_depth, 1);
    assert!(telemetry.nodes > 0);
    let mut checkmate = mate;
    checkmate.apply(action).unwrap();
    assert!(matches!(
        run(request(&checkmate)).result,
        EngineResult::Finished {
            outcome: ChessOutcome::Checkmate {
                winner: Color::White
            },
            ..
        }
    ));

    // Losing Black has an intended-move fifty-move claim at counter 99.
    let claim =
        AdjudicatedGame::from_fen("7k/8/8/8/8/8/8/KR6 b - - 99 1", ClaimPolicy::ExplicitOnly)
            .unwrap();
    let response = run(request(&claim));
    let EngineResult::Claim {
        claim: selected,
        telemetry,
    } = response.result
    else {
        panic!("{response:?}")
    };
    assert_eq!(selected.reason, DrawClaim::FiftyMove);
    assert!(selected.intended.is_some());
    assert_eq!(telemetry.score, Some(0));
    let mut accepted = claim.clone();
    accepted.claim(selected.into()).unwrap();
    assert_eq!(
        accepted.position(),
        claim.position(),
        "claim does not play intended move"
    );

    let mut zero = request(&AdjudicatedGame::new(ClaimPolicy::ExplicitOnly));
    zero.limits.nodes = 0;
    let response = run(zero);
    let EngineResult::Incomplete { telemetry } = response.result else {
        panic!("{response:?}")
    };
    assert_eq!(telemetry.score, None);
    assert_eq!(telemetry.completed_depth, 0);
    assert_eq!(telemetry.nodes, 0);
    assert_eq!(telemetry.status, StopReason::NodeLimit);
}
#[test]
fn same_position_different_known_histories_preserve_different_claim_results() {
    let fen = "6nk/8/8/8/8/8/8/KR4N1 b - - 0 1";
    let mut repeated = AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap();
    for action in [
        "g8f6", "g1f3", "f6g8", "f3g1", "g8f6", "g1f3", "f6g8", "f3g1",
    ] {
        repeated.apply(action.parse().unwrap()).unwrap();
    }
    let imported =
        AdjudicatedGame::from_fen(&repeated.position().to_fen(), ClaimPolicy::ExplicitOnly)
            .unwrap();
    assert_eq!(repeated.position(), imported.position());
    assert!(!repeated.available_claims().is_empty());
    assert!(imported.available_claims().is_empty());
    assert!(matches!(
        run(request(&repeated)).result,
        EngineResult::Claim { .. }
    ));
    assert!(matches!(
        run(request(&imported)).result,
        EngineResult::Move { .. }
    ));
    assert_ne!(request(&repeated).game, request(&imported).game);
}
#[test]
fn rejects_full_before_replay_without_downgrading_context() {
    let record = AdjudicatedGame::from_fen_with_mode(
        "7k/8/8/8/8/8/8/K7 w - - 0 1",
        ClaimPolicy::ExplicitOnly,
        AdjudicationMode::Full {
            limits: ReachabilityLimits {
                max_positions: Some(1),
                max_transitions: Some(1),
            },
        },
    )
    .unwrap();
    let response = run(request(&record));
    assert!(matches!(
        response.result,
        EngineResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::UnsupportedMode,
                ..
            }
        }
    ));
    assert!(matches!(
        response.context.game.adjudication_mode(),
        AdjudicationMode::Full { .. }
    ));
}
#[test]
fn malformed_oversized_unknown_and_unterminated_input_are_rejected() {
    for bytes in [
        b"{}\n".to_vec(),
        vec![b'x'; MAX_REQUEST_BYTES + 2],
        b"{}".to_vec(),
    ] {
        assert!(serve_once(Cursor::new(bytes), Vec::new()).is_err());
    }
    let mut value =
        serde_json::to_value(request(&AdjudicatedGame::new(ClaimPolicy::ExplicitOnly))).unwrap();
    value["unknown"] = true.into();
    let mut bytes = serde_json::to_vec(&value).unwrap();
    bytes.push(b'\n');
    assert!(serve_once(Cursor::new(bytes), Vec::new()).is_err());
}
#[test]
fn invalid_settings_and_search_failure_have_no_score() {
    let game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let mut invalid = request(&game);
    invalid.protocol = 0;
    assert!(matches!(
        run(invalid).result,
        EngineResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::InvalidRequest,
                ..
            }
        }
    ));
    // Any Black move requires incrementing a counter already at u32::MAX.
    let overflow = AdjudicatedGame::from_fen(
        "7k/8/8/8/8/8/8/KR6 b - - 0 4294967295",
        ClaimPolicy::ExplicitOnly,
    )
    .unwrap();
    assert!(matches!(
        run(request(&overflow)).result,
        EngineResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::Search,
                ..
            }
        }
    ));
}

#[test]
fn native_record_duplicate_fields_are_not_collapsed_before_strict_validation() {
    let context = request(&AdjudicatedGame::new(ClaimPolicy::ExplicitOnly));
    let text = serde_json::to_string(&context).unwrap();
    let text = text.replace(
        "\"claim_policy\":",
        "\"claim_policy\":\"explicit_only\",\"claim_policy\":",
    );
    assert!(serde_json::from_str::<EngineRequest>(&text).is_err());
}

fn analysis_target(game: &AdjudicatedGame) -> AnalysisTarget {
    AnalysisTarget {
        selection_id: 1,
        ply: game.history().ply_count() as u64,
        session_version: 7,
        label: "Native protocol target".into(),
        turn: game.position().side_to_move(),
    }
}
fn full_request(game: &AdjudicatedGame, positions: u64) -> EngineRequest {
    let mut context = request(game);
    context.purpose = RequestPurpose::FullReachability {
        target: analysis_target(game),
        mate: ReachabilityTarget::Either,
        limits: FullLimits {
            max_positions: positions,
            max_transitions: 1000,
        },
    };
    context.limits = FULL_SENTINEL;
    context
}
#[test]
fn full_child_operation_reports_real_possible_impossible_and_unknown_work() {
    let mated =
        AdjudicatedGame::from_fen("7k/6Q1/5K2/8/8/8/8/8 b - - 0 1", ClaimPolicy::ExplicitOnly)
            .unwrap();
    assert!(
        matches!(run(full_request(&mated, 10)).result, EngineResult::Reachability { result: MateReachability::Possible { witness, work } } if witness.is_empty() && work.positions == 1)
    );
    let dead = AdjudicatedGame::from_fen("7k/8/8/8/8/8/8/K7 w - - 0 1", ClaimPolicy::ExplicitOnly)
        .unwrap();
    assert!(
        matches!(run(full_request(&dead, 10)).result, EngineResult::Reachability { result: MateReachability::Impossible { work } } if work.static_leaves == 1)
    );
    let live = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    assert!(
        matches!(run(full_request(&live, 0)).result, EngineResult::Reachability { result: MateReachability::Unknown { reason: ReachabilityStop::PositionLimit, work } } if work.positions == 0 && work.transitions == 0)
    );
}
#[test]
fn analysis_requires_exact_provenance_supported_mode_and_explicit_limits() {
    let live = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let mut context = request(&live);
    context.purpose = RequestPurpose::MoveAnalysis {
        target: analysis_target(&live),
    };
    assert!(matches!(
        run(context.clone()).result,
        EngineResult::Move { .. }
    ));
    if let RequestPurpose::MoveAnalysis { target } = &mut context.purpose {
        target.ply = 1;
    }
    assert!(matches!(
        run(context).result,
        EngineResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::InvalidRequest,
                ..
            }
        }
    ));
    let mut invalid = full_request(&live, MAX_FULL_POSITIONS + 1);
    assert!(matches!(
        run(invalid.clone()).result,
        EngineResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::InvalidRequest,
                ..
            }
        }
    ));
    invalid.purpose = RequestPurpose::MoveAnalysis {
        target: analysis_target(&live),
    };
    invalid.limits = WorkLimits {
        depth: MAX_ANALYSIS_DEPTH + 1,
        nodes: 1,
    };
    assert!(matches!(
        run(invalid).result,
        EngineResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::InvalidRequest,
                ..
            }
        }
    ));
    let encoded = serde_json::to_string(&request(&live)).unwrap();
    assert!(
        serde_json::from_str::<EngineRequest>(
            &encoded.replace("\"purpose\":{\"kind\":\"engine_turn\"},", "")
        )
        .is_err()
    );
}
