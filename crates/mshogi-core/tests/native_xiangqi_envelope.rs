//! Real MSH-94 records crossing the shared boundary; expected rules stay native.
use mshogi_core::{chess as c, native::*, shogi as s, xiangqi as x};
use serde_json::{Value, json};

const ROOT: &str = "4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1";
const C: [&str; 4] = ["e9f9", "f8e8", "f9e9", "e8f8"];
fn game() -> x::Game {
    x::Game::from_root(
        x::PositionRoot::from_fen(ROOT).unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap()
}
fn record(g: &x::Game) -> CanonicalRecord {
    CanonicalRecord::xiangqi(x::GameRecord::from_game(g).unwrap())
}
fn submit(g: &mut x::Game, actor: x::Side, command: x::GameCommand) -> x::Submission {
    let request = x::GameRequest {
        expected: g.context(),
        actor,
        command,
    };
    let NativeReply::Xiangqi(reply) = dispatch_command(
        NativeCommandTarget::Xiangqi(g),
        NativeCommand::Xiangqi(Box::new(request)),
    )
    .unwrap() else {
        panic!("wrong native reply")
    };
    *reply.unwrap()
}
fn play(g: &mut x::Game, action: &str) {
    submit(
        g,
        g.position().side_to_move(),
        x::GameCommand::Play {
            action: action.parse().unwrap(),
        },
    );
}

#[test]
fn real_records_keep_profiles_roots_counters_history_and_cached_target_metadata() {
    let mut g = game();
    for action in C.into_iter().cycle().take(8) {
        play(&mut g, action);
    }
    assert_eq!(g.warnings().len(), 1);
    let r = record(&g);
    assert_eq!(r.game_kind(), GameKind::Xiangqi);
    assert_eq!(r.profile(), RulesProfile::XiangqiWxf2018V1);
    assert_eq!(r.native_schema(), NativeSchema::XiangqiV1);
    assert_eq!(r.known_board_actions(), 8);
    assert_eq!(r.event_count(), 9); // eight actions plus third-occurrence warning
    let text = r.to_json().unwrap();
    let wire: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(wire["envelope_version"], 1);
    assert_eq!(wire["rules_profile"], x::PROFILE_ID);
    assert_eq!(wire["profile_edition"], x::PROFILE_EDITION);
    assert_eq!(wire["native_schema_version"], 1);
    assert_eq!(wire["payload"]["root"]["origin"], "imported_fen");
    assert_eq!(
        wire["payload"]["root"]["history"],
        "starts_at_this_position"
    );
    assert_eq!(wire["payload"]["root"]["no_capture_plies"], 0);
    assert_eq!(wire["payload"]["no_win_goal"], x::NATURAL_WIN_GOAL);
    assert_eq!(wire["payload"]["no_win_mode"], "sound_fast_only");
    let parsed = CanonicalRecord::from_json(&text).unwrap();
    assert_eq!(parsed, r);
    assert_eq!(serde_json::from_str::<CanonicalRecord>(&text).unwrap(), r);
    let CanonicalRecord::Xiangqi(native) = parsed else {
        panic!()
    };
    assert_eq!(native.side_to_move(), x::Side::Red);
    assert_eq!(native.event_count(), 9);
    let mut restored = native.to_game().unwrap();
    assert_eq!(restored.events(), g.events());
    assert_eq!(restored.warnings(), g.warnings());
    assert_eq!(restored.root(), g.root());
    assert_eq!(record(&restored), r);
    assert_eq!(
        restored
            .submit(x::GameRequest {
                expected: g.context(),
                actor: x::Side::Red,
                command: x::GameCommand::Resign,
            })
            .unwrap_err(),
        x::GameError::StaleContext
    );
    // Exact same current position/counters, but no inferred warning or occurrences.
    let fresh = x::Game::from_root(
        x::PositionRoot::from_fen(&g.position().to_fen().unwrap()).unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap();
    assert_eq!(fresh.position(), g.position());
    assert_ne!(record(&fresh), r);
    assert_eq!(record(&fresh).known_board_actions(), 0);
    assert_eq!(record(&fresh).event_count(), 0);
    for action in C {
        play(&mut g, action);
    }
    assert_eq!(
        g.conclusion(),
        Some(x::Conclusion::Win {
            winner: x::Side::Black,
            reason: x::WinReason::Repetition {
                fault: x::FaultKind::PerpetualCheck
            },
        })
    );
    let terminal = record(&g);
    let CanonicalRecord::Xiangqi(native) =
        CanonicalRecord::from_json(&terminal.to_json().unwrap()).unwrap()
    else {
        panic!()
    };
    assert_eq!(native.to_game().unwrap().conclusion(), g.conclusion());
    assert_eq!(native.to_game().unwrap().events(), g.events());
}

#[test]
fn shared_dispatch_preserves_audit_result_native_actor_admission_undo_and_atomic_refusal() {
    let mut g = game();
    let original = record(&g);
    let stale = g.context();
    // Black has no board turn, but may submit its own count claim. It is invalid,
    // not a board error or a win/loss, and the separate admitted audit is retained.
    let audit = submit(&mut g, x::Side::Black, x::GameCommand::ClaimNaturalCount);
    assert!(matches!(
        audit.result,
        x::CommandResult::RejectedNaturalCount
    ));
    assert_eq!(g.position(), g.root().position());
    assert_eq!(g.conclusion(), None);
    assert_eq!(record(&g).known_board_actions(), 0);
    assert_eq!(record(&g).event_count(), 2);
    let audited = record(&g);
    assert_eq!(
        CanonicalRecord::from_json(&audited.to_json().unwrap()).unwrap(),
        audited
    );
    g.undo(audit.undo).unwrap();
    assert_eq!(record(&g), original);
    assert_ne!(g.context(), stale);
    let before = g.context();
    let request = x::GameRequest {
        expected: before.clone(),
        actor: x::Side::Black,
        command: x::GameCommand::Play {
            action: "f8e8".parse().unwrap(),
        },
    };
    let NativeReply::Xiangqi(Err(error)) = dispatch_command(
        NativeCommandTarget::Xiangqi(&mut g),
        NativeCommand::Xiangqi(Box::new(request)),
    )
    .unwrap() else {
        panic!()
    };
    assert_eq!(error, x::GameError::WrongActor);
    assert_eq!(g.context(), before);
    assert_eq!(record(&g), original);
    let resign = submit(&mut g, x::Side::Black, x::GameCommand::Resign);
    assert_eq!(resign.result, x::CommandResult::Accepted);
    assert_eq!(g.position().side_to_move(), x::Side::Red);
    assert_eq!(
        g.conclusion(),
        Some(x::Conclusion::Win {
            winner: x::Side::Red,
            reason: x::WinReason::Resignation
        })
    );
    assert_eq!(record(&g).known_board_actions(), 0);
    assert_eq!(record(&g).event_count(), 2);
    g.undo(resign.undo).unwrap();
    assert_eq!(record(&g), original);

    let shogi_request = s::GameRequest {
        expected: s::Game::new().context(),
        actor: s::Side::Sente,
        command: s::GameCommand::Resign,
    };
    assert!(
        dispatch_command(
            NativeCommandTarget::Xiangqi(&mut g),
            NativeCommand::Shogi(Box::new(shogi_request))
        )
        .is_err()
    );
    assert_eq!(record(&g), original);
    let mut shogi = s::Game::new();
    let shogi_before = s::GameRecord::from_game(&shogi);
    let request = x::GameRequest {
        expected: g.context(),
        actor: x::Side::Red,
        command: x::GameCommand::Resign,
    };
    assert!(
        dispatch_command(
            NativeCommandTarget::Shogi(&mut shogi),
            NativeCommand::Xiangqi(Box::new(request))
        )
        .is_err()
    );
    assert_eq!(s::GameRecord::from_game(&shogi), shogi_before);
}

#[test]
fn claimant_specific_full_history_and_same_ply_conclusions_survive_native_handoff() {
    let fixtures: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/xiangqi/natural-count.json"
    ))
    .unwrap();
    let fixture = fixtures["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == "NATURAL-11-CHECKS")
        .unwrap();
    let mut g = x::Game::from_root(
        x::PositionRoot::from_fen(fixture["root_fen"].as_str().unwrap()).unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap();
    for action in fixture["moves"].as_array().unwrap().iter().take(100) {
        play(&mut g, action.as_str().unwrap());
    }
    assert_eq!(
        g.natural_count_eligibility(x::Side::Red).effective_plies,
        99
    );
    assert_eq!(
        g.natural_count_eligibility(x::Side::Black).effective_plies,
        100
    );
    let position = g.position().clone();
    let rejected = submit(&mut g, x::Side::Red, x::GameCommand::ClaimNaturalCount);
    assert!(matches!(
        rejected.result,
        x::CommandResult::RejectedNaturalCount
    ));
    submit(&mut g, x::Side::Black, x::GameCommand::ClaimNaturalCount);
    assert_eq!(g.position(), &position);
    assert_eq!(
        g.conclusion(),
        Some(x::Conclusion::Draw {
            reason: x::DrawReason::NaturalCount
        })
    );
    let r = record(&g);
    assert_eq!(r.known_board_actions(), 100);
    assert_eq!(r.event_count(), 105); //100 board,2 rejected audit,3 accepted claim/result
    let CanonicalRecord::Xiangqi(native) =
        CanonicalRecord::from_json(&r.to_json().unwrap()).unwrap()
    else {
        panic!()
    };
    let restored = native.to_game().unwrap();
    assert_eq!(restored.events(), g.events());
    assert_eq!(
        restored.natural_count_eligibility(x::Side::Red),
        g.natural_count_eligibility(x::Side::Red)
    );
    assert_eq!(restored.conclusion(), g.conclusion());
}

#[test]
fn raw_outer_and_nested_rejections_never_fall_through_to_another_game() {
    let text = record(&game()).to_json().unwrap();
    let baseline: Value = serde_json::from_str(&text).unwrap();
    for field in [
        "envelope_version",
        "game_kind",
        "rules_profile",
        "profile_edition",
        "native_schema_version",
        "payload",
    ] {
        let mut missing = baseline.clone();
        missing.as_object_mut().unwrap().remove(field);
        assert!(
            CanonicalRecord::from_json(&missing.to_string()).is_err(),
            "missing {field}"
        );
        let raw = text.replacen(
            &format!("\"{field}\":"),
            &format!("\"{field}\":{},\"{field}\":", baseline[field]),
            1,
        );
        assert!(
            CanonicalRecord::from_json(&raw).is_err(),
            "duplicate {field}"
        );
        assert!(serde_json::from_str::<CanonicalRecord>(&raw).is_err());
    }
    for (pointer, value) in [
        ("/envelope_version", json!(2)),
        ("/game_kind", json!("future_game")),
        ("/game_kind", json!("shogi")),
        ("/rules_profile", json!(s::PROFILE_ID)),
        ("/rules_profile", json!("mshogi.xiangqi.wxf2018.v2")),
        ("/profile_edition", json!("WXF-2024")),
        ("/native_schema_version", json!(2)),
        ("/payload/game", json!("chess")),
        ("/payload/profile", json!(s::PROFILE_ID)),
        ("/payload/edition", json!("wrong")),
        ("/payload/schema_version", json!(2)),
        ("/payload/root/profile", json!(s::PROFILE_ID)),
        ("/payload/no_win_mode", json!("exact")),
        ("/payload/fast_witness_limit", json!(2)),
    ] {
        let mut wrong = baseline.clone();
        *wrong.pointer_mut(pointer).unwrap() = value;
        assert!(
            CanonicalRecord::from_json(&wrong.to_string()).is_err(),
            "{pointer}"
        );
    }
    for raw in [
        text.replacen(
            "\"profile\":",
            &format!("\"profile\":\"{}\",\"profile\":", x::PROFILE_ID),
            1,
        ),
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
        text.replacen("\"payload\":{", "\"payload\":{\"unknown\":1,", 1),
        text.replacen("\"root\":{", "\"root\":{\"unknown\":1,", 1),
        text.replacen("\"conclusion\":null,", "", 1),
    ] {
        assert_ne!(raw, text);
        assert!(CanonicalRecord::from_json(&raw).is_err());
    }
    let mut extra = baseline.clone();
    extra["unknown"] = json!(1);
    assert!(CanonicalRecord::from_json(&extra.to_string()).is_err());
    // A genuine other native payload also cannot be relabeled as Xiangqi.
    let chess =
        CanonicalRecord::chess(c::AdjudicatedGame::new(c::ClaimPolicy::ExplicitOnly).record());
    let mut wrong_native_parser: Value = serde_json::from_str(&chess.to_json().unwrap()).unwrap();
    wrong_native_parser["payload"] = baseline["payload"].clone();
    assert!(CanonicalRecord::from_json(&wrong_native_parser.to_string()).is_err());
    let shogi = CanonicalRecord::shogi(s::GameRecord::from_game(&s::Game::new()));
    let mut wrong_native_parser: Value = serde_json::from_str(&shogi.to_json().unwrap()).unwrap();
    wrong_native_parser["payload"] = baseline["payload"].clone();
    assert!(CanonicalRecord::from_json(&wrong_native_parser.to_string()).is_err());
    let mut wrong = baseline;
    wrong["payload"] =
        serde_json::from_str::<Value>(&chess.to_json().unwrap()).unwrap()["payload"].clone();
    assert!(CanonicalRecord::from_json(&wrong.to_string()).is_err());
}

#[test]
fn side_qualification_does_not_conflate_black_or_admit_unknown_side_fields() {
    for side in [x::Side::Red, x::Side::Black] {
        let native = NativeSide::Xiangqi(side);
        let text = serde_json::to_string(&native).unwrap();
        assert_eq!(serde_json::from_str::<NativeSide>(&text).unwrap(), native);
        assert_eq!(native.game(), GameKind::Xiangqi);
    }
    assert_ne!(
        NativeSide::Xiangqi(x::Side::Black),
        NativeSide::Chess(c::Color::Black)
    );
    for text in [
        r#"{"game_kind":"xiangqi","side":"white"}"#,
        r#"{"game_kind":"chess","side":"red"}"#,
        r#"{"game_kind":"xiangqi","side":"sente"}"#,
        r#"{"game_kind":"xiangqi","side":"black","side":"black"}"#,
        r#"{"game_kind":"xiangqi","side":"black","unknown":1}"#,
        r#"{"game_kind":"xiangqi"}"#,
    ] {
        assert!(serde_json::from_str::<NativeSide>(text).is_err(), "{text}");
    }
}
