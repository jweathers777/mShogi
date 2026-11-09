//! Additive Chess thinking-time and selective-search execution contracts.
use mshogi_core::{
    chess::*,
    native::{CanonicalRecord, NativeSide},
};
use mshogi_engine::*;
use std::{
    io::Cursor,
    time::{Duration, Instant},
};

fn context() -> ExecutionContext {
    ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {},
        request_id: 1,
        session_version: 3,
        process_generation: 2,
        game: CanonicalRecord::chess(AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record()),
        limits: WorkLimits {
            depth: 128,
            nodes: 20_000,
            time_ms: None,
        },
    }
}
fn execute(context: ExecutionContext) -> ExecutionResponse {
    let request = ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context,
    };
    let mut input = serde_json::to_vec(&request).unwrap();
    input.push(b'\n');
    let mut output = Vec::new();
    serve_once(Cursor::new(input), &mut output).unwrap();
    let response: ExecutionResponse = serde_json::from_slice(&output).unwrap();
    assert!(response.matches(&request), "{response:?}");
    response
}
fn incomplete() -> ChessResult {
    ChessResult::Incomplete {
        telemetry: Telemetry {
            score: None,
            completed_depth: 0,
            selective_depth: Some(0),
            nodes: 0,
            principal_variation: vec![],
            status: StopReason::Stopped,
        },
    }
}
#[test]
fn omitted_extensions_preserve_historical_wire_and_observation_encoding() {
    let limits = WorkLimits {
        depth: 2,
        nodes: 20_000,
        time_ms: None,
    };
    let wire = r#"{"depth":2,"nodes":20000}"#;
    assert_eq!(serde_json::to_string(&limits).unwrap(), wire);
    assert_eq!(serde_json::from_str::<WorkLimits>(wire).unwrap(), limits);
    let old = r#"{"score":0,"completed_depth":1,"nodes":21,"principal_variation":[{"kind":"move","action":"a2a3"}],"status":"complete"}"#;
    let telemetry: Telemetry = serde_json::from_str(old).unwrap();
    assert_eq!(telemetry.selective_depth, None);
    assert_eq!(telemetry.variation_depth_bound(), Some(1));
    assert_eq!(serde_json::to_string(&telemetry).unwrap(), old);
}
#[test]
fn thinking_time_is_exact_request_identity_and_only_chess_ordinary_work_accepts_it() {
    let original = context();
    let mut timed = original.clone();
    timed.limits.time_ms = Some(1_000);
    assert!(timed.valid_limits());
    let observation = ExecutionObservation::new(
        timed.clone(),
        ExecutionResult::Chess {
            result: incomplete(),
        },
    );
    assert!(observation.matches(&timed));
    assert!(!observation.matches(&original));
    for milliseconds in [0, 1, 60_000] {
        timed.limits.time_ms = Some(milliseconds);
        assert!(timed.valid_limits());
    }
    timed.limits.time_ms = Some(60_001);
    assert!(!timed.valid_limits());
    timed = original.clone();
    timed.limits.time_ms = Some(1);
    timed.game = CanonicalRecord::shogi(mshogi_core::shogi::GameRecord::from_game(
        &mshogi_core::shogi::Game::new(),
    ));
    assert!(!timed.valid_limits());
    timed = original;
    timed.purpose = ExecutionPurpose::ChessFullReachability {
        target: ExecutionTarget {
            selection_id: 1,
            event_index: 0,
            board_ply: 0,
            session_version: timed.session_version,
            label: "Start".into(),
            turn: NativeSide::Chess(Color::White),
        },
        mate: ReachabilityTarget::Either,
        limits: FullLimits {
            max_positions: 1,
            max_transitions: 1,
        },
    };
    timed.limits = FULL_SENTINEL;
    assert!(timed.valid_limits());
    timed.limits.time_ms = Some(1);
    assert!(!timed.valid_limits());
}
#[test]
fn actual_zero_time_request_is_honest_incomplete_without_action_or_score() {
    let mut request = context();
    request.limits.time_ms = Some(0);
    let response = execute(request);
    let ExecutionResult::Chess {
        result: ChessResult::Incomplete { telemetry },
    } = response.observation.result
    else {
        panic!("Zero budget invented a decision: {response:?}");
    };
    assert_eq!(telemetry.completed_depth, 0);
    assert_eq!(telemetry.score, None);
    assert!(telemetry.principal_variation.is_empty());
    assert_eq!(telemetry.status, StopReason::Stopped);
    assert_eq!(telemetry.nodes, 0);
    assert_eq!(telemetry.selective_depth, Some(0));
}
#[test]
fn selective_continuation_bound_is_explicit_and_cannot_exceed_native_path_limit() {
    let mut request = context();
    request.limits.depth = 1;
    let actions: Vec<Action> = ["e2e4", "e7e5", "g1f3"]
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();
    let telemetry = Telemetry {
        score: Some(0),
        completed_depth: 1,
        selective_depth: Some(3),
        nodes: 50,
        principal_variation: actions
            .iter()
            .map(|&action| EngineChoice::Move { action })
            .collect(),
        status: StopReason::Complete,
    };
    let observation = ExecutionObservation::new(
        request.clone(),
        ExecutionResult::Chess {
            result: ChessResult::Move {
                action: actions[0],
                telemetry,
            },
        },
    );
    assert!(observation.matches(&request));
    for invalid in [0, 2, 129] {
        let mut changed = observation.clone();
        let ExecutionResult::Chess {
            result: ChessResult::Move { telemetry, .. },
        } = &mut changed.result
        else {
            unreachable!()
        };
        telemetry.selective_depth = Some(invalid);
        assert!(!changed.matches(&request), "selective bound {invalid}");
    }
    let mut changed = observation;
    let ExecutionResult::Chess {
        result: ChessResult::Move { telemetry, .. },
    } = &mut changed.result
    else {
        unreachable!()
    };
    telemetry.nodes = request.limits.nodes + 1;
    assert!(!changed.matches(&request));
}
#[test]
fn actual_short_timed_search_is_bounded_and_every_returned_command_is_native() {
    let mut request = context();
    request.limits.time_ms = Some(20);
    let source = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let before = source.clone();
    let start = Instant::now();
    let response = execute(request);
    assert!(
        start.elapsed() < Duration::from_secs(5),
        "short thinking request did not finish cooperatively"
    );
    let ExecutionResult::Chess { result } = response.observation.result else {
        panic!("wrong result family")
    };
    let telemetry = match result {
        ChessResult::Move { action, telemetry } => {
            assert!(source.position().legal_actions().contains(&action));
            assert_eq!(
                telemetry.principal_variation.first(),
                Some(&EngineChoice::Move { action })
            );
            telemetry
        }
        ChessResult::Incomplete { telemetry } => {
            assert!(telemetry.score.is_none() && telemetry.principal_variation.is_empty());
            telemetry
        }
        other => panic!("unexpected timed standard result: {other:?}"),
    };
    assert!(telemetry.nodes <= 20_000);
    assert!(telemetry.selective_depth.is_some());
    let mut replay = source.clone();
    for choice in telemetry.principal_variation {
        match choice {
            EngineChoice::Move { action } => replay.apply(action).unwrap(),
            EngineChoice::Claim { claim } => replay.claim(claim.into()).unwrap(),
        }
    }
    assert_eq!(source, before);
}

#[test]
fn fixed_time_play_uses_the_full_allowance_and_small_explicit_node_requests_still_stop() {
    let mut request = context();
    request.limits.nodes = 9_007_199_254_740_991;
    request.limits.time_ms = Some(50);
    let start = Instant::now();
    let response = execute(request.clone());
    assert!(start.elapsed() >= Duration::from_millis(50));
    assert!(start.elapsed() < Duration::from_secs(5));
    let ExecutionResult::Chess { result } = response.observation.result else {
        panic!("wrong game")
    };
    let telemetry = match result {
        ChessResult::Move { telemetry, .. } | ChessResult::Incomplete { telemetry } => telemetry,
        other => panic!("unexpected standard-root result {other:?}"),
    };
    assert_eq!(telemetry.status, StopReason::Stopped);
    assert!(telemetry.completed_depth < request.limits.depth);
    assert!(telemetry.nodes < request.limits.nodes);
    request.limits.nodes = 1;
    let response = execute(request);
    let ExecutionResult::Chess {
        result: ChessResult::Incomplete { telemetry },
    } = response.observation.result
    else {
        panic!("one visit invented a completed move");
    };
    assert_eq!(telemetry.status, StopReason::NodeLimit);
    assert_eq!(telemetry.nodes, 1);
    assert!(telemetry.score.is_none() && telemetry.principal_variation.is_empty());
}
