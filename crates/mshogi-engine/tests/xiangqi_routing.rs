//! Xiangqi context transport is not evaluator/search/exact-verifier delivery.
use mshogi_core::{chess as c, native::*, xiangqi as x};
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
fn request(g: &x::Game) -> SharedRequest {
    SharedRequest {
        protocol: SHARED_PROTOCOL_VERSION,
        purpose: SharedPurpose::EngineTurn {},
        request_id: 17,
        session_version: 11,
        process_generation: 7,
        game: CanonicalRecord::xiangqi(x::GameRecord::from_game(g).unwrap()),
        // Opaque unsupported settings; these are NOT an advertised native search policy.
        limits: WorkLimits {
            depth: u16::MAX,
            nodes: u64::MAX,
        },
    }
}
fn target(context: &SharedRequest) -> SharedAnalysisTarget {
    let CanonicalRecord::Xiangqi(native) = &context.game else {
        panic!()
    };
    SharedAnalysisTarget {
        selection_id: 3,
        event_index: native.event_count() as u64,
        board_ply: native.known_board_actions() as u64,
        session_version: context.session_version,
        label: "Exact native event prefix".into(),
        turn: NativeSide::Xiangqi(native.side_to_move()),
    }
}
fn raw(text: &str) -> std::io::Result<Vec<u8>> {
    let mut output = vec![];
    serve_once(Cursor::new(format!("{text}\n")), &mut output)?;
    Ok(output)
}
fn run(context: &SharedRequest) -> SharedResponse {
    serde_json::from_slice(&raw(&serde_json::to_string(context).unwrap()).unwrap()).unwrap()
}
fn failure(response: &SharedResponse, kind: FailureKind) {
    assert!(
        matches!(&response.result, SharedResult::Failure { failure } if failure.kind == kind),
        "{response:?}"
    );
}

#[test]
fn no_xiangqi_execution_is_advertised_or_inferred_for_any_purpose_or_native_result() {
    assert!(execution_capabilities(GameKind::Xiangqi).is_empty());
    assert_eq!(execution_capabilities(GameKind::Chess).len(), 3);
    assert_eq!(execution_capabilities(GameKind::Shogi).len(), 2);
    let mut context = request(&warned());
    for limits in [
        WorkLimits { depth: 0, nodes: 0 },
        WorkLimits {
            depth: u16::MAX,
            nodes: u64::MAX,
        },
    ] {
        context.limits = limits;
        assert!(context.valid_limits());
        let response = run(&context);
        assert_eq!(response.context, context);
        assert!(response.matches(&context));
        failure(&response, FailureKind::UnsupportedMode);
        let wire = serde_json::to_value(response).unwrap();
        for absent in ["score", "action", "telemetry", "conclusion"] {
            assert!(wire["result"].get(absent).is_none());
        }
        context.purpose = SharedPurpose::MoveAnalysis {
            target: target(&context),
        };
        let response = run(&context);
        failure(&response, FailureKind::UnsupportedMode);
        assert!(response.matches(&context));
        context.purpose = SharedPurpose::EngineTurn {};
    }
    context.limits = FULL_SENTINEL;
    context.purpose = SharedPurpose::ChessFullReachability {
        target: target(&context),
        mate: ReachabilityTarget::Either,
        limits: FullLimits {
            max_positions: 1,
            max_transitions: 1,
        },
    };
    assert!(context.valid_limits());
    failure(&run(&context), FailureKind::UnsupportedMode);
    // Even a native concluded/fast-proven game is preserved as data, not executed.
    let proof = x::Game::from_root(
        x::PositionRoot::from_fen("5k3/9/9/9/9/9/9/9/9/3K5 w - - 0 1").unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap();
    assert_eq!(
        proof.no_win_assessment(),
        x::NoWinAssessment::ProvenNeitherCanWin
    );
    assert!(proof.conclusion().is_some());
    let context = request(&proof);
    let response = run(&context);
    assert_eq!(response.context, context);
    failure(&response, FailureKind::UnsupportedMode);
    let standard = x::Game::standard(x::ClaimPolicy::AutoValidClaims).unwrap();
    assert_eq!(
        standard.no_win_assessment(),
        x::NoWinAssessment::UnknownExactUnavailable
    );
    failure(&run(&request(&standard)), FailureKind::UnsupportedMode);
}

#[test]
fn response_acceptance_requires_every_identity_setting_and_full_native_history() {
    let g = warned();
    let mut context = request(&g);
    context.purpose = SharedPurpose::MoveAnalysis {
        target: target(&context),
    };
    let response = run(&context);
    assert!(response.matches(&context));
    let wire = serde_json::to_value(&context).unwrap();
    for (pointer, value) in [
        ("/protocol", json!(4)),
        ("/request_id", json!(18)),
        ("/session_version", json!(12)),
        ("/process_generation", json!(8)),
        ("/limits/depth", json!(1)),
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
        let altered: SharedRequest = serde_json::from_value(altered).unwrap();
        assert!(!response.matches(&altered), "{pointer}");
    }
    let mut altered = context.clone();
    altered.purpose = SharedPurpose::EngineTurn {};
    assert!(!response.matches(&altered));
    let fresh = x::Game::from_root(
        x::PositionRoot::from_fen(&g.position().to_fen().unwrap()).unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap();
    assert_eq!(fresh.position(), g.position());
    let mut altered = context.clone();
    altered.game = request(&fresh).game;
    altered.purpose = SharedPurpose::MoveAnalysis {
        target: target(&altered),
    };
    assert!(altered.valid_limits());
    assert!(!response.matches(&altered));
    let mut foreign = response.clone();
    foreign.result = SharedResult::Chess {
        result: EngineResult::Incomplete {
            telemetry: Telemetry {
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
    overlong.result = SharedResult::Failure {
        failure: EngineFailure {
            kind: FailureKind::UnsupportedMode,
            message: "x".repeat(MAX_STDERR_BYTES + 1),
        },
    };
    assert!(!overlong.matches(&context));
}

#[test]
fn malformed_or_wrong_target_context_is_invalid_rather_than_an_unsupported_success() {
    let mut context = request(&warned());
    context.purpose = SharedPurpose::MoveAnalysis {
        target: target(&context),
    };
    let wire = serde_json::to_value(&context).unwrap();
    for (pointer, value) in [
        ("/request_id", json!(0)),
        ("/process_generation", json!(0)),
        ("/purpose/target/selection_id", json!(0)),
        ("/purpose/target/session_version", json!(0)),
        ("/purpose/target/event_index", json!(8)),
        ("/purpose/target/board_ply", json!(9)),
        ("/purpose/target/turn/side", json!("black")),
        ("/purpose/target/label", json!("")),
    ] {
        let mut altered = wire.clone();
        *altered.pointer_mut(pointer).unwrap() = value;
        let altered: SharedRequest = serde_json::from_value(altered).unwrap();
        failure(&run(&altered), FailureKind::InvalidRequest);
    }
    let mut wrong = context.clone();
    let SharedPurpose::MoveAnalysis { target } = &mut wrong.purpose else {
        panic!()
    };
    target.turn = NativeSide::Chess(c::Color::White);
    failure(&run(&wrong), FailureKind::InvalidRequest);
}

#[test]
fn strict_raw_request_envelope_native_and_response_boundaries_preserve_duplicates() {
    let mut context = request(&warned());
    context.purpose = SharedPurpose::MoveAnalysis {
        target: target(&context),
    };
    let text = serde_json::to_string(&context).unwrap();
    for bad in [
        text.replacen("\"protocol\":3", "\"protocol\":3,\"protocol\":3", 1),
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
            "\"conclusion\":null",
            "\"conclusion\":null,\"conclusion\":null",
            1,
        ),
        text.replacen(
            "\"kind\":\"warning\"",
            "\"kind\":\"warning\",\"kind\":\"warning\"",
            1,
        ),
        text.replacen("\"protocol\":3", "\"protocol\":99", 1),
    ] {
        assert_ne!(bad, text);
        assert!(raw(&bad).is_err(), "{bad}");
    }
    let baseline: Value = serde_json::from_str(&text).unwrap();
    for key in [
        "protocol",
        "purpose",
        "request_id",
        "session_version",
        "process_generation",
        "game",
        "limits",
    ] {
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
    assert!(serde_json::from_str::<SharedResponse>(&bad).is_err());
}

#[test]
fn actual_maximum_native_audit_ledger_and_whole_encoded_transport_boundaries_fit() {
    let mut g = game();
    for _ in 0..511 {
        command(&mut g, x::Side::Black, x::GameCommand::ClaimNaturalCount);
    }
    command(&mut g, x::Side::Black, x::GameCommand::Resign);
    let native = x::GameRecord::from_game(&g).unwrap();
    assert_eq!(native.event_count(), 1024);
    assert_eq!(native.known_board_actions(), 0);
    let native_bytes = native.to_json().len();
    let context = request(&g);
    let envelope = context.game.to_json().unwrap().len();
    let frame = serde_json::to_string(&context).unwrap();
    let response = run(&context);
    failure(&response, FailureKind::UnsupportedMode);
    assert!(response.matches(&context));
    let echoed = serde_json::to_vec(&response).unwrap().len();
    assert!(native_bytes < envelope && envelope < frame.len() && frame.len() < MAX_REQUEST_BYTES);
    assert!(echoed < MAX_RESPONSE_BYTES);
    println!(
        "xiangqi512commands/1024events: native={native_bytes}, envelope={envelope}, request={}, response={echoed}",
        frame.len()
    );
    let exact = frame.replacen(
        "\"game\":",
        &format!("\"game\":{}", " ".repeat(MAX_REQUEST_BYTES - frame.len())),
        1,
    );
    assert_eq!(exact.len(), MAX_REQUEST_BYTES);
    assert!(raw(&exact).is_ok());
    assert!(raw(&format!(" {exact}")).is_err());
    // Standalone wrapper bytes have their own cap, separately from request overhead.
    let text = request(&game()).game.to_json().unwrap();
    let exact = text.replacen(
        "\"payload\":",
        &format!(
            "\"payload\":{}",
            " ".repeat(MAX_RECORD_ENVELOPE_BYTES - text.len())
        ),
        1,
    );
    assert_eq!(exact.len(), MAX_RECORD_ENVELOPE_BYTES);
    assert!(CanonicalRecord::from_json(&exact).is_ok());
    assert_eq!(
        CanonicalRecord::from_json(&format!(" {exact}")),
        Err(EnvelopeError::Capacity)
    );
}
