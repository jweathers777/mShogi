//! Shared-boundary facts; chess search and shogi rules remain native.
mod support;
use mshogi_core::{chess::*, native::*, shogi};
use mshogi_engine::*;
use serde_json::{Value, json};
use std::io::Cursor;

fn request(game: &AdjudicatedGame) -> ExecutionContext {
    ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {},
        request_id: 1,
        session_version: 7,
        game: CanonicalRecord::chess(game.record()),
        limits: WorkLimits {
            time_ms: None,
            depth: 1,
            nodes: 10_000,
        },
    }
}
fn target(context: &ExecutionContext) -> ExecutionTarget {
    ExecutionTarget {
        selection_id: 1,
        event_index: context.game.event_count() as u64,
        board_ply: context.game.known_board_actions() as u64,
        session_version: context.session_version,
        label: "Native shared target".into(),
        turn: NativeSide::Chess(Color::White),
    }
}
fn raw_run(text: &str) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();
    serve_once(
        Cursor::new(format!(
            "{{\"protocol\":{PROTOCOL_VERSION},\"context\":{text}}}\n"
        )),
        &mut output,
    )?;
    Ok(output)
}
fn run(context: ExecutionContext) -> ExecutionObservation {
    let output = raw_run(&serde_json::to_string(&context).unwrap()).unwrap();
    let response = support::decode_response(
        &output,
        &ExecutionRequest {
            protocol: PROTOCOL_VERSION,
            context: context.clone(),
        },
    )
    .unwrap()
    .observation;
    assert_eq!(response.context, context);
    response
}
fn chess_result(context: ExecutionContext) -> ChessResult {
    let ExecutionResult::Chess { result } = run(context).result else {
        panic!("expected native chess result")
    };
    result
}
#[test]
fn actual_chess_play_claim_terminal_incomplete_and_analysis_are_preserved() {
    let mut mate =
        AdjudicatedGame::from_fen("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", ClaimPolicy::ExplicitOnly)
            .unwrap();
    let ChessResult::Move { action, telemetry } = chess_result(request(&mate)) else {
        panic!()
    };
    assert_eq!(action.to_string(), "g6g7");
    assert_eq!(telemetry.score, Some(999_999));
    let mut analysis = request(&mate);
    analysis.purpose = ExecutionPurpose::MoveAnalysis {
        target: target(&analysis),
    };
    assert!(matches!(
        chess_result(analysis.clone()),
        ChessResult::Move { .. }
    ));
    assert_eq!(
        mate.history().ply_count(),
        0,
        "read-only analysis is not command admission"
    );
    mate.apply(action).unwrap();
    assert!(matches!(
        chess_result(request(&mate)),
        ChessResult::Finished {
            outcome: ChessOutcome::Checkmate {
                winner: Color::White
            },
            ..
        }
    ));
    let claim =
        AdjudicatedGame::from_fen("7k/8/8/8/8/8/8/KR6 b - - 99 1", ClaimPolicy::ExplicitOnly)
            .unwrap();
    assert!(matches!(
        chess_result(request(&claim)),
        ChessResult::Claim {
            claim: EngineClaim {
                reason: DrawClaim::FiftyMove,
                intended: Some(_),
                ..
            },
            ..
        }
    ));
    let live = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let mut zero = request(&live);
    zero.limits.nodes = 0;
    assert!(matches!(
        chess_result(zero),
        ChessResult::Incomplete {
            telemetry: Telemetry {
                score: None,
                nodes: 0,
                ..
            }
        }
    ));
}
#[test]
fn actual_shogi_records_have_only_implemented_native_operations() {
    let mut game = shogi::Game::new();
    game.submit(shogi::GameRequest {
        expected: game.context(),
        actor: shogi::Side::Gote,
        command: shogi::GameCommand::Resign,
    })
    .unwrap();
    let mut context = request(&AdjudicatedGame::new(ClaimPolicy::ExplicitOnly));
    context.game = CanonicalRecord::shogi(shogi::GameRecord::from_game(&game));
    let mut t = target(&context);
    t.turn = NativeSide::Shogi(shogi::Side::Sente);
    for purpose in [
        ExecutionPurpose::EngineTurn {},
        ExecutionPurpose::MoveAnalysis { target: t },
    ] {
        context.limits = WorkLimits {
            time_ms: None,
            depth: 1,
            nodes: 10,
        };
        context.purpose = purpose;
        let response = run(context.clone());
        assert!(response.matches(&context));
        assert!(matches!(
            response.result,
            ExecutionResult::Shogi {
                result: ShogiResult::Finished {
                    conclusion: shogi::Conclusion::Win {
                        winner: shogi::Side::Sente,
                        reason: shogi::WinReason::Resignation
                    },
                    ..
                }
            }
        ));
    }
}
#[test]
fn same_position_different_history_is_not_interchangeable() {
    let mut repeated =
        AdjudicatedGame::from_fen("6nk/8/8/8/8/8/8/KR4N1 b - - 0 1", ClaimPolicy::ExplicitOnly)
            .unwrap();
    for m in [
        "g8f6", "g1f3", "f6g8", "f3g1", "g8f6", "g1f3", "f6g8", "f3g1",
    ] {
        repeated.apply(m.parse().unwrap()).unwrap();
    }
    let fresh = AdjudicatedGame::from_fen(&repeated.position().to_fen(), ClaimPolicy::ExplicitOnly)
        .unwrap();
    assert_eq!(fresh.position(), repeated.position());
    let response = run(request(&repeated));
    assert!(!response.matches(&request(&fresh)));
    assert!(matches!(
        response.result,
        ExecutionResult::Chess {
            result: ChessResult::Claim { .. }
        }
    ));
    assert!(matches!(
        chess_result(request(&fresh)),
        ChessResult::Move { .. }
    ));
}
#[test]
fn acceptance_checks_every_execution_and_selection_field() {
    let mut context = request(&AdjudicatedGame::new(ClaimPolicy::ExplicitOnly));
    context.purpose = ExecutionPurpose::MoveAnalysis {
        target: target(&context),
    };
    let response = run(context.clone());
    assert!(response.matches(&context));
    let wire = serde_json::to_value(&context).unwrap();
    for (pointer, value) in [
        ("/request_id", json!(2)),
        ("/session_version", json!(8)),
        ("/limits/nodes", json!(99)),
        ("/purpose/target/selection_id", json!(2)),
        ("/purpose/target/event_index", json!(1)),
        ("/purpose/target/board_ply", json!(1)),
        ("/purpose/target/session_version", json!(8)),
        ("/purpose/target/label", json!("Different target")),
        ("/purpose/target/turn/side", json!("black")),
    ] {
        let mut v = wire.clone();
        *v.pointer_mut(pointer).unwrap() = value;
        let changed: ExecutionContext = serde_json::from_value(v).unwrap();
        assert!(!response.matches(&changed), "{pointer}");
    }
    let mut turn = context.clone();
    turn.purpose = ExecutionPurpose::EngineTurn {};
    assert!(!response.matches(&turn));
    let mut invalid = context.clone();
    if let ExecutionPurpose::MoveAnalysis { target } = &mut invalid.purpose {
        target.turn = NativeSide::Chess(Color::Black);
    }
    assert!(matches!(
        chess_result(invalid),
        ChessResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::InvalidRequest,
                ..
            }
        }
    ));
    for change in 0..4 {
        let mut invalid = context.clone();
        match change {
            0 => invalid.request_id = 0,
            1 => invalid.limits.depth = MAX_ANALYSIS_DEPTH + 1,
            2 => invalid.limits.nodes = MAX_ANALYSIS_NODES + 1,
            _ => {
                if let ExecutionPurpose::MoveAnalysis { target } = &mut invalid.purpose {
                    target.event_index = 1
                }
            }
        };
        assert!(matches!(
            run(invalid).result,
            ExecutionResult::Failure {
                failure: EngineFailure {
                    kind: FailureKind::InvalidRequest,
                    ..
                }
            }
        ));
    }
    let mut overlong = response;
    overlong.result = ExecutionResult::Failure {
        failure: EngineFailure {
            kind: FailureKind::Search,
            message: "x".repeat(MAX_STDERR_BYTES + 1),
        },
    };
    assert!(!overlong.matches(&context));
}
#[test]
fn strict_outer_purpose_target_envelope_and_native_bytes_survive_routing() {
    let context = request(&AdjudicatedGame::new(ClaimPolicy::ExplicitOnly));
    let text = serde_json::to_string(&context).unwrap();
    for (needle, replacement) in [
        ("\"request_id\":1", "\"request_id\":1,\"request_id\":1"),
        (
            "\"kind\":\"engine_turn\"",
            "\"kind\":\"engine_turn\",\"extra\":1",
        ),
        (
            "\"game_kind\":\"chess\"",
            "\"game_kind\":\"chess\",\"game_kind\":\"chess\"",
        ),
        (
            "\"claim_policy\":\"explicit_only\"",
            "\"claim_policy\":\"explicit_only\",\"claim_policy\":\"explicit_only\"",
        ),
    ] {
        assert!(text.contains(needle));
        let malformed = text.replacen(needle, replacement, 1);
        assert!(raw_run(&malformed).is_err(), "{needle}");
    }
    let value: Value = serde_json::from_str(&text).unwrap();
    for field in value.as_object().unwrap().keys() {
        let mut v = value.clone();
        v.as_object_mut().unwrap().remove(field);
        assert!(raw_run(&v.to_string()).is_err(), "missing {field}");
    }
    for pointer in ["/game/envelope_version", "/game/payload/schema_version"] {
        let mut v = value.clone();
        *v.pointer_mut(pointer).unwrap() = json!(99);
        assert!(raw_run(&v.to_string()).is_err());
    }
    for (pointer, v) in [
        ("/game/game_kind", json!("shogi")),
        ("/game/payload/rules_profile", json!("unknown")),
        ("/purpose/kind", json!("shogi_tsume")),
    ] {
        let mut altered = value.clone();
        *altered.pointer_mut(pointer).unwrap() = v;
        assert!(raw_run(&altered.to_string()).is_err(), "{pointer}");
    }
    let mut unknown = value.clone();
    unknown["extra"] = json!(true);
    assert!(raw_run(&unknown.to_string()).is_err());
    let mut analysis = context;
    analysis.purpose = ExecutionPurpose::MoveAnalysis {
        target: target(&analysis),
    };
    let text = serde_json::to_string(&analysis).unwrap();
    assert!(
        raw_run(&text.replace(
            "\"selection_id\":1",
            "\"selection_id\":1,\"selection_id\":1"
        ))
        .is_err()
    );
    assert!(
        raw_run(&text.replace(
            "\"game_kind\":\"chess\",\"side\":\"white\"",
            "\"game_kind\":\"shogi\",\"side\":\"white\""
        ))
        .is_err()
    );
    let mut altered = serde_json::to_value(analysis).unwrap();
    altered["purpose"]["target"]["unknown"] = json!(true);
    assert!(raw_run(&altered.to_string()).is_err());
}

#[test]
fn long_shogi_history_request_echoes_its_context() {
    // Deterministic walk to the Shogi move-count rule; not a rules oracle.
    let mut game = shogi::Game::new();
    let mut seed = 0x67_c5a3_6d91_u64;
    for ply in 1..=320 {
        let choices = game.position().legal_actions();
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let start = (seed % choices.len() as u64) as usize;
        let mut selected = None;
        for offset in 0..choices.len() {
            let mut candidate = game.clone();
            candidate
                .submit(shogi::GameRequest {
                    expected: candidate.context(),
                    actor: candidate.position().side_to_move(),
                    command: shogi::GameCommand::Play(choices[(start + offset) % choices.len()]),
                })
                .unwrap();
            if ply == 320 || candidate.conclusion().is_none() {
                selected = Some(candidate);
                break;
            }
        }
        game = selected.expect("reviewed deterministic walk has a continuation");
    }
    let mut context = request(&AdjudicatedGame::new(ClaimPolicy::ExplicitOnly));
    context.game = CanonicalRecord::shogi(shogi::GameRecord::from_game(&game));
    let response = run(context.clone());
    assert!(response.matches(&context));
}
