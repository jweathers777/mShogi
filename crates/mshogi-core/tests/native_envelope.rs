//! Two real native payloads, strict raw boundaries and native command authority.
use mshogi_core::{chess as c, native::*, shogi as s};
use serde_json::{Value, json};

fn shogi_game() -> s::Game {
    let root = s::GameRoot::import(
        s::Position::from_sfen("4k4/9/5R3/9/9/9/9/9/K8 b - 17").unwrap(),
        s::RootCategory::ReducedMaterialDiagram,
        s::CounterSource::Supplied,
    )
    .unwrap();
    let mut game = s::Game::from_root(root);
    for text in ["4c5c", "5a4a", "5c4c", "4a5a"] {
        game.submit(s::GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: s::GameCommand::Play(text.parse().unwrap()),
        })
        .unwrap();
    }
    game
}

#[test]
fn actual_native_payloads_round_trip_with_derived_identity_and_provenance() {
    let mut chess = c::AdjudicatedGame::new(c::ClaimPolicy::ExplicitOnly);
    chess.apply("e2e4".parse().unwrap()).unwrap();
    let mut record = chess.record();
    record.push_event(c::RecordEvent {
        ply: 1,
        kind: c::RecordEventKind::Observation(c::MatchObservation::Command {
            actor: c::Color::Black,
            command: c::RecordedCommand::Resign,
        }),
    });
    let shogi = shogi_game();
    for native in [
        CanonicalRecord::chess(record.clone()),
        CanonicalRecord::shogi(s::GameRecord::from_game(&shogi)),
    ] {
        let text = native.to_json().unwrap();
        assert_eq!(CanonicalRecord::from_json(&text).unwrap(), native);
        assert_eq!(
            serde_json::from_str::<CanonicalRecord>(&text).unwrap(),
            native
        );
        let wire: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(wire["envelope_version"], 1);
        assert_eq!(wire["native_schema_version"], 1);
        match native {
            CanonicalRecord::Chess(r) => {
                assert_eq!(wire["rules_profile"], "mshogi.chess.match.v1");
                assert_eq!(
                    wire["profile_edition"],
                    "FIDE-2023-01-01+mshogi.chess.match.v1"
                );
                assert_eq!(
                    c::GameRecord::from_json(&wire["payload"].to_string()).unwrap(),
                    record
                );
                assert_eq!(r.replay_history().unwrap().observations.len(), 1);
                // Historical resignation data is not executed as a match command.
                assert!(c::AdjudicatedGame::replay_record(&r).is_err());
            }
            CanonicalRecord::Shogi(r) => {
                assert_eq!(wire["rules_profile"], "mshogi.shogi.csa36.v1");
                assert_eq!(
                    wire["profile_edition"],
                    "JSA-2025-10-01+CSA-2023-11-27-WCSC36-2026"
                );
                assert_eq!(r.to_game().unwrap(), shogi);
                assert_eq!(
                    shogi.root().counter_source(),
                    Some(s::CounterSource::Supplied)
                );
                assert!(!shogi.root().has_complete_initial_history());
            }
        }
    }
    let full = c::AdjudicatedGame::from_fen_with_mode(
        "7k/8/8/8/8/8/8/K7 w - - 0 1",
        c::ClaimPolicy::ExplicitOnly,
        c::AdjudicationMode::Full {
            limits: c::ReachabilityLimits {
                max_positions: Some(1),
                max_transitions: Some(1),
            },
        },
    )
    .unwrap();
    let full = CanonicalRecord::chess(full.record());
    assert_eq!(full.native_schema(), NativeSchema::ChessFull);
    assert_eq!(
        CanonicalRecord::from_json(&full.to_json().unwrap()).unwrap(),
        full
    );
}

#[test]
fn outer_and_native_duplicates_unknown_missing_and_cross_identities_reject() {
    let native = CanonicalRecord::shogi(s::GameRecord::from_game(&shogi_game()));
    let text = native.to_json().unwrap();
    for key in [
        "envelope_version",
        "game_kind",
        "rules_profile",
        "profile_edition",
        "native_schema_version",
        "payload",
    ] {
        let mut value: Value = serde_json::from_str(&text).unwrap();
        value.as_object_mut().unwrap().remove(key);
        assert!(
            CanonicalRecord::from_json(&value.to_string()).is_err(),
            "missing {key}"
        );
    }
    for malformed in [
        text.replacen(
            "\"envelope_version\":1",
            "\"envelope_version\":1,\"envelope_version\":1",
            1,
        ),
        text.replacen("\"payload\":", "\"unexpected\":true,\"payload\":", 1),
        text.replacen(
            "\"schema_version\":1",
            "\"schema_version\":1,\"schema_version\":1",
            1,
        ),
        text.replacen("\"root\":", "\"extra_native\":true,\"root\":", 1),
        text.replacen(
            "\"counter_source\":",
            "\"counter_source\":\"supplied\",\"counter_source\":",
            1,
        ),
        text.replacen("\"envelope_version\":1", "\"envelope_version\":99", 1),
    ] {
        assert!(
            CanonicalRecord::from_json(&malformed).is_err(),
            "{malformed}"
        );
    }
    let mut value: Value = serde_json::from_str(&text).unwrap();
    for (key, bad) in [
        ("game_kind", json!("chess")),
        ("rules_profile", json!("mshogi.chess.match.v1")),
        ("profile_edition", json!("unreviewed")),
        ("native_schema_version", json!(2)),
    ] {
        let before = value[key].clone();
        value[key] = bad;
        assert!(
            CanonicalRecord::from_json(&value.to_string()).is_err(),
            "conflict {key}"
        );
        value[key] = before;
    }
    value["rules_profile"] = json!("mshogi.other.v1");
    assert!(CanonicalRecord::from_json(&value.to_string()).is_err());
    // All outer chess IDs can agree, but a native shogi payload still cannot dispatch as chess.
    value["rules_profile"] = json!("mshogi.chess.match.v1");
    value["game_kind"] = json!("chess");
    value["profile_edition"] = json!(CHESS_PROFILE_EDITION);
    assert!(CanonicalRecord::from_json(&value.to_string()).is_err());
    let chess =
        CanonicalRecord::chess(c::AdjudicatedGame::new(c::ClaimPolicy::ExplicitOnly).record())
            .to_json()
            .unwrap();
    assert!(
        CanonicalRecord::from_json(&chess.replacen(
            "\"claim_policy\":",
            "\"claim_policy\":\"explicit_only\",\"claim_policy\":",
            1
        ))
        .is_err()
    );
}

#[test]
fn known_history_and_same_ply_terminal_events_do_not_collapse() {
    let mut known = shogi_game();
    let fresh = s::Game::from_root(
        s::GameRoot::import(
            known.position().clone(),
            s::RootCategory::ReducedMaterialDiagram,
            s::CounterSource::Supplied,
        )
        .unwrap(),
    );
    assert_eq!(known.position().to_sfen(), fresh.position().to_sfen());
    let before = CanonicalRecord::shogi(s::GameRecord::from_game(&known));
    assert_ne!(
        before,
        CanonicalRecord::shogi(s::GameRecord::from_game(&fresh))
    );
    let board = known.position().clone();
    known
        .submit(s::GameRequest {
            expected: known.context(),
            actor: s::Side::Gote,
            command: s::GameCommand::Resign,
        })
        .unwrap();
    let after = CanonicalRecord::shogi(s::GameRecord::from_game(&known));
    assert_eq!(known.position(), &board);
    assert_eq!(before.known_board_actions(), after.known_board_actions());
    assert_eq!(after.event_count(), before.event_count() + 1);
    assert_ne!(before, after);
    assert_eq!(
        CanonicalRecord::from_json(&after.to_json().unwrap()).unwrap(),
        after
    );
}

#[test]
fn native_owners_admit_claims_declarations_and_resignation_without_shared_turn_rules() {
    let mut shogi = s::Game::new();
    let before = shogi.clone();
    assert!(!shogi.declaration_eligibility(s::Side::Gote).eligible());
    assert_eq!(shogi, before, "advisory query has no command effect");
    let request = s::GameRequest {
        expected: shogi.context(),
        actor: s::Side::Gote,
        command: s::GameCommand::DeclareEnteringKing,
    };
    let reply = dispatch_command(
        NativeCommandTarget::Shogi(&mut shogi),
        NativeCommand::Shogi(Box::new(request)),
    )
    .unwrap();
    let NativeReply::Shogi(Ok(undo)) = reply else {
        panic!("formal declaration should be admitted")
    };
    assert_eq!(shogi.position(), before.position());
    assert_eq!(
        shogi.conclusion(),
        Some(s::Conclusion::Win {
            winner: s::Side::Sente,
            reason: s::WinReason::FalseDeclaration
        })
    );
    shogi.undo(*undo).unwrap();
    let request = s::GameRequest {
        expected: shogi.context(),
        actor: s::Side::Gote,
        command: s::GameCommand::Resign,
    };
    assert!(matches!(
        dispatch_command(
            NativeCommandTarget::Shogi(&mut shogi),
            NativeCommand::Shogi(Box::new(request))
        )
        .unwrap(),
        NativeReply::Shogi(Ok(_))
    ));
    assert_eq!(
        shogi.conclusion(),
        Some(s::Conclusion::Win {
            winner: s::Side::Sente,
            reason: s::WinReason::Resignation
        })
    );
    let game = c::AdjudicatedGame::from_fen(
        "7k/8/8/8/8/8/8/KR6 b - - 99 1",
        c::ClaimPolicy::ExplicitOnly,
    )
    .unwrap();
    let board = game.position().clone();
    let mut chess = c::ChessMatch::new(game, c::TimeControl::Untimed).unwrap();
    let req = c::MatchRequest {
        at_ns: 0,
        version: 0,
        actor: c::Color::Black,
        command: c::MatchCommand::Start,
    };
    assert!(matches!(
        dispatch_command(
            NativeCommandTarget::Chess(&mut chess),
            NativeCommand::Chess(req)
        )
        .unwrap(),
        NativeReply::Chess(c::MatchReply {
            rejection: None,
            ..
        })
    ));
    let request = c::MatchRequest {
        at_ns: 0,
        version: chess.version(),
        actor: c::Color::Black,
        command: c::MatchCommand::Claim {
            reason: c::DrawClaim::FiftyMove,
            intended: Some("h8h7".parse().unwrap()),
        },
    };
    let NativeReply::Chess(reply) = dispatch_command(
        NativeCommandTarget::Chess(&mut chess),
        NativeCommand::Chess(request),
    )
    .unwrap() else {
        panic!()
    };
    assert_eq!(reply.rejection, None);
    assert_eq!(
        reply.conclusion,
        Some(c::MatchConclusion::Chess(c::ChessOutcome::Draw(
            c::DrawReason::Claimed(c::DrawClaim::FiftyMove)
        )))
    );
    assert_eq!(
        chess.game().position(),
        &board,
        "intended claim does not play its move"
    );
    let wrong = s::GameRequest {
        expected: shogi.context(),
        actor: s::Side::Sente,
        command: s::GameCommand::Resign,
    };
    let before = chess.record().to_json();
    assert!(
        dispatch_command(
            NativeCommandTarget::Chess(&mut chess),
            NativeCommand::Shogi(Box::new(wrong))
        )
        .is_err()
    );
    assert_eq!(chess.record().to_json(), before);
}

#[test]
fn record_byte_event_and_diagnostic_bounds_are_explicit() {
    let mut record = c::AdjudicatedGame::new(c::ClaimPolicy::ExplicitOnly).record();
    for _ in 0..MAX_SHARED_CHESS_EVENTS {
        record.push_event(c::RecordEvent {
            ply: 0,
            kind: c::RecordEventKind::Observation(c::MatchObservation::Clock {
                at_ns: u64::MAX,
                remaining_ns: [u64::MAX; 2],
                active: Some(c::Color::Black),
            }),
        });
    }
    let native = CanonicalRecord::chess(record.clone());
    let text = native.to_json().unwrap();
    assert!(text.len() < MAX_RECORD_ENVELOPE_BYTES);
    assert_eq!(CanonicalRecord::from_json(&text).unwrap(), native);
    record.push_event(record.events()[0].clone());
    assert_eq!(
        CanonicalRecord::chess(record).to_json(),
        Err(EnvelopeError::Capacity)
    );
    assert_eq!(
        CanonicalRecord::from_json(&" ".repeat(MAX_RECORD_ENVELOPE_BYTES + 1)),
        Err(EnvelopeError::Capacity)
    );
    let mut wire: Value = serde_json::from_str(&text).unwrap();
    wire.as_object_mut()
        .unwrap()
        .insert("x".repeat(50_000), json!(true));
    let EnvelopeError::Encoding(message) =
        CanonicalRecord::from_json(&wire.to_string()).unwrap_err()
    else {
        panic!()
    };
    assert!(message.len() <= MAX_RECORD_DIAGNOSTIC_BYTES);
}
