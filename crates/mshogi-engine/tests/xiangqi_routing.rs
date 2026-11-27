//! Xiangqi ordinary execution, strict context and bounded transport.
mod support;
use mshogi_core::{chess as c, game::*, xiangqi as x};
use mshogi_engine::*;
use serde_json::{Value, json};
use std::io::Cursor;

fn game() -> x::Game {
    x::Game::from_root(
        x::PositionRoot::from_fen("4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1").unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap()
}
fn command(g: &mut x::Game, actor: x::Side, command: x::GameCommand) {
    g.submit(x::GameRequest {
        expected: g.context(),
        actor,
        command,
    })
    .unwrap();
}
fn warned() -> x::Game {
    let mut g = game();
    for action in ["e9f9", "f8e8", "f9e9", "e8f8"].into_iter().cycle().take(8) {
        let actor = g.position().side_to_move();
        command(
            &mut g,
            actor,
            x::GameCommand::Play {
                action: action.parse().unwrap(),
            },
        );
    }
    g
}
fn request(g: &x::Game) -> ExecutionContext {
    ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {},
        request_id: 17,
        session_version: 11,
        game: CanonicalRecord::xiangqi(x::GameRecord::from_game(g).unwrap()),
        // Measured MSH-97 ordinary limits.
        limits: WorkLimits {
            time_ms: None,
            depth: 1,
            nodes: 128,
        },
    }
}
fn target(context: &ExecutionContext) -> ExecutionTarget {
    let CanonicalRecord::Xiangqi(record) = &context.game else {
        panic!()
    };
    ExecutionTarget {
        selection_id: 3,
        event_index: record.event_count() as u64,
        board_ply: record.known_board_actions() as u64,
        session_version: context.session_version,
        label: "Exact game event prefix".into(),
        turn: TaggedSide::Xiangqi(record.side_to_move()),
    }
}
fn raw(text: &str) -> std::io::Result<Vec<u8>> {
    let mut output = vec![];
    serve_once(
        Cursor::new(format!(
            "{{\"protocol\":{PROTOCOL_VERSION},\"context\":{text}}}\n"
        )),
        &mut output,
    )?;
    Ok(output)
}
fn run(context: &ExecutionContext) -> ExecutionObservation {
    support::decode_response(
        &raw(&serde_json::to_string(context).unwrap()).unwrap(),
        &ExecutionRequest {
            protocol: PROTOCOL_VERSION,
            context: context.clone(),
        },
    )
    .unwrap()
    .observation
}
fn failure(response: &ExecutionObservation, kind: FailureKind) {
    assert!(
        matches!(&response.result, ExecutionResult::Failure { failure } if failure.kind == kind),
        "{response:?}"
    );
}

#[test]
fn actual_xiangqi_execution_returns_choices() {
    let g = warned();
    let before = x::GameRecord::from_game(&g).unwrap();
    let mut context = request(&g);
    for analysis in [false, true] {
        if analysis {
            context.purpose = ExecutionPurpose::MoveAnalysis {
                target: target(&context),
            };
        }
        let response = run(&context);
        assert_eq!(response.context, context);
        assert!(response.matches(&context));
        let ExecutionResult::Xiangqi {
            result: XiangqiResult::Choice { choice, telemetry },
        } = response.result
        else {
            panic!("{response:?}")
        };
        assert_eq!(telemetry.perspective, x::Side::Red);
        assert!(telemetry.completed_depth > 0);
        assert!(telemetry.nodes <= context.limits.nodes);
        let mut candidate = before.to_game().unwrap();
        let command = match choice {
            XiangqiChoice::Move { action } => x::GameCommand::Play { action },
            XiangqiChoice::Claim { .. } => x::GameCommand::ClaimNaturalCount,
        };
        command_result(&mut candidate, command);
        assert_eq!(before, x::GameRecord::from_game(&g).unwrap());
    }
    let proof = x::Game::from_root(
        x::PositionRoot::from_fen("5k3/9/9/9/9/9/9/9/9/3K5 w - - 0 1").unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap();
    let response = run(&request(&proof));
    assert!(
        matches!(response.result, ExecutionResult::Xiangqi { result: XiangqiResult::Finished { conclusion, ref telemetry } }
        if Some(conclusion) == proof.conclusion() && telemetry.score == Some(0))
    );
    let standard = x::Game::standard(x::ClaimPolicy::AutoValidClaims).unwrap();
    assert_eq!(standard.no_win_assessment(), x::NoWinAssessment::Unknown);
    let response = run(&request(&standard));
    assert!(matches!(
        response.result,
        ExecutionResult::Xiangqi {
            result: XiangqiResult::Choice { .. }
        }
    ));
}
fn command_result(g: &mut x::Game, record: x::GameCommand) {
    command(g, g.position().side_to_move(), record);
}

#[test]
fn ordinary_limits_and_zero_work_remain_distinct_from_conclusions() {
    let mut context = request(&warned());
    context.limits.nodes = 0;
    let response = run(&context);
    assert!(response.matches(&context));
    assert!(
        matches!(response.result, ExecutionResult::Xiangqi { result: XiangqiResult::Incomplete { ref telemetry } }
        if telemetry.nodes == 0 && telemetry.score.is_none() && telemetry.completed_depth == 0 && telemetry.principal_variation.is_empty())
    );
    for limits in [
        WorkLimits {
            time_ms: None,
            depth: 0,
            nodes: 128,
        },
        WorkLimits {
            time_ms: None,
            depth: 65,
            nodes: 128,
        },
        WorkLimits {
            time_ms: None,
            depth: 1,
            nodes: 5_000_001,
        },
    ] {
        context.limits = limits;
        assert!(!context.valid_limits());
        failure(&run(&context), FailureKind::InvalidRequest);
    }
}

#[test]
fn response_acceptance_requires_every_identity_setting_and_full_history() {
    let g = warned();
    let mut context = request(&g);
    context.purpose = ExecutionPurpose::MoveAnalysis {
        target: target(&context),
    };
    let response = run(&context);
    assert!(response.matches(&context));
    let wire = serde_json::to_value(&context).unwrap();
    for (pointer, value) in [
        ("/request_id", json!(18)),
        ("/session_version", json!(12)),
        ("/limits/depth", json!(2)),
        ("/limits/nodes", json!(0)),
        ("/purpose/target/selection_id", json!(4)),
        ("/purpose/target/session_version", json!(12)),
        ("/purpose/target/event_index", json!(8)),
        ("/purpose/target/board_ply", json!(0)),
        ("/purpose/target/label", json!("Another position")),
        ("/purpose/target/turn/side", json!("black")),
    ] {
        let mut altered = wire.clone();
        *altered.pointer_mut(pointer).unwrap() = value;
        let altered: ExecutionContext = serde_json::from_value(altered).unwrap();
        assert!(!response.matches(&altered), "{pointer}");
    }
    let mut altered = context.clone();
    altered.purpose = ExecutionPurpose::EngineTurn {};
    assert!(!response.matches(&altered));
    let fresh = x::Game::from_root(
        x::PositionRoot::from_fen(&g.position().to_fen().unwrap()).unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap();
    assert_eq!(fresh.position(), g.position());
    let mut altered = context.clone();
    altered.game = request(&fresh).game;
    altered.purpose = ExecutionPurpose::MoveAnalysis {
        target: target(&altered),
    };
    assert!(altered.valid_limits());
    assert!(!response.matches(&altered));
    let mut foreign = response.clone();
    foreign.result = ExecutionResult::Chess {
        result: ChessResult::Incomplete {
            telemetry: Telemetry {
                selective_depth: 0,
                score: None,
                completed_depth: 0,
                nodes: 0,
                principal_variation: vec![],
                status: StopReason::NodeLimit,
            },
        },
    };
    assert!(!foreign.matches(&context));
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
fn malformed_or_wrong_target_context_is_invalid_rather_than_an_unsupported_success() {
    let mut context = request(&warned());
    context.purpose = ExecutionPurpose::MoveAnalysis {
        target: target(&context),
    };
    let wire = serde_json::to_value(&context).unwrap();
    for (pointer, value) in [
        ("/request_id", json!(0)),
        ("/purpose/target/selection_id", json!(0)),
        ("/purpose/target/session_version", json!(0)),
        ("/purpose/target/event_index", json!(8)),
        ("/purpose/target/board_ply", json!(9)),
        ("/purpose/target/turn/side", json!("black")),
        ("/purpose/target/label", json!("")),
    ] {
        let mut altered = wire.clone();
        *altered.pointer_mut(pointer).unwrap() = value;
        let altered: ExecutionContext = serde_json::from_value(altered).unwrap();
        failure(&run(&altered), FailureKind::InvalidRequest);
    }
    let mut wrong = context.clone();
    let ExecutionPurpose::MoveAnalysis { target } = &mut wrong.purpose else {
        panic!()
    };
    target.turn = TaggedSide::Chess(c::Color::White);
    failure(&run(&wrong), FailureKind::InvalidRequest);
}

#[test]
fn strict_raw_request_envelope_and_response_boundaries_preserve_duplicates() {
    let mut context = request(&warned());
    context.purpose = ExecutionPurpose::MoveAnalysis {
        target: target(&context),
    };
    let text = serde_json::to_string(&context).unwrap();
    for bad in [
        text.replacen(
            "\"request_id\":17",
            "\"request_id\":17,\"request_id\":17",
            1,
        ),
        text.replacen("\"purpose\":{", "\"purpose\":{\"unknown\":1,", 1),
        text.replacen(
            "\"selection_id\":3",
            "\"selection_id\":3,\"selection_id\":3",
            1,
        ),
        text.replacen(
            "\"game_kind\":\"xiangqi\"",
            "\"game_kind\":\"xiangqi\",\"game_kind\":\"xiangqi\"",
            1,
        ),
        text.replacen("\"payload\":{", "\"payload\":{\"unknown\":1,", 1),
        text.replacen(
            "\"history\":",
            "\"history\":\"starts_at_this_position\",\"history\":",
            1,
        ),
        text.replacen(
            "\"schema_version\":2",
            "\"schema_version\":2,\"schema_version\":2",
            1,
        ),
        text.replacen(
            "\"kind\":\"play\"",
            "\"kind\":\"play\",\"kind\":\"play\"",
            1,
        ),
    ] {
        assert_ne!(bad, text);
        assert!(raw(&bad).is_err(), "{bad}");
    }
    let baseline: Value = serde_json::from_str(&text).unwrap();
    for key in ["purpose", "request_id", "session_version", "game", "limits"] {
        let mut missing = baseline.clone();
        missing.as_object_mut().unwrap().remove(key);
        assert!(raw(&missing.to_string()).is_err(), "missing {key}");
    }
    let response = serde_json::to_string(&run(&context)).unwrap();
    let bad = response.replacen(
        "\"history\":",
        "\"history\":\"starts_at_this_position\",\"history\":",
        1,
    );
    assert!(serde_json::from_str::<ExecutionObservation>(&bad).is_err());
}

#[test]
fn long_audit_ledger_request_finishes_and_echoes_its_context() {
    let mut g = game();
    for _ in 0..511 {
        command(&mut g, x::Side::Black, x::GameCommand::ClaimNaturalCount);
    }
    command(&mut g, x::Side::Black, x::GameCommand::Resign);
    let record = x::GameRecord::from_game(&g).unwrap();
    assert_eq!(record.event_count(), 1024);
    assert_eq!(record.known_board_actions(), 0);
    let context = request(&g);
    let response = run(&context);
    assert!(matches!(
        response.result,
        ExecutionResult::Xiangqi {
            result: XiangqiResult::Finished { .. }
        }
    ));
    assert!(response.matches(&context));
}

#[test]
fn telemetry_family_limits_perspective_and_required_fields_are_checked() {
    let context = request(&warned());
    let response = run(&context);
    assert!(response.matches(&context));
    let wire = serde_json::to_value(&response).unwrap();
    for (pointer, value) in [
        (
            "/result/result/telemetry/evaluator_id",
            json!("other.evaluator"),
        ),
        ("/result/result/telemetry/search_id", json!("other.search")),
        ("/result/result/telemetry/perspective", json!("black")),
        ("/result/result/telemetry/completed_depth", json!(2)),
        ("/result/result/telemetry/nodes", json!(129)),
        ("/result/result/telemetry/score", Value::Null),
        ("/result/result/telemetry/principal_variation", json!([])),
        ("/result/result/telemetry/status", json!("unresolved")),
    ] {
        let mut altered = wire.clone();
        *altered.pointer_mut(pointer).unwrap() = value;
        let altered: ExecutionObservation = serde_json::from_value(altered).unwrap();
        assert!(!altered.matches(&context), "{pointer}");
    }
    let mut missing = wire;
    missing["result"]["result"]["telemetry"]
        .as_object_mut()
        .unwrap()
        .remove("score");
    assert!(serde_json::from_value::<ExecutionObservation>(missing).is_err());
    let text = serde_json::to_string(&response).unwrap();
    let duplicate = text.replacen(
        "\"completed_depth\":1",
        "\"completed_depth\":1,\"completed_depth\":1",
        1,
    );
    assert_ne!(duplicate, text);
    assert!(serde_json::from_str::<ExecutionObservation>(&duplicate).is_err());
    let unknown = text.replacen("\"telemetry\":{", "\"telemetry\":{\"unknown\":1,", 1);
    assert!(serde_json::from_str::<ExecutionObservation>(&unknown).is_err());
}
