//! Wire strictness, canonical action identity and ordered observation replay.
use mshogi_core::chess::*;
use serde_json::{Value, json};
fn a(s: &str) -> Action {
    s.parse().unwrap()
}
fn event(ply: u64, kind: RecordEventKind) -> RecordEvent {
    RecordEvent { ply, kind }
}

#[test]
fn canonical_record_has_an_independent_expected_encoding() {
    let mut game = Game::new(ClaimPolicy::ExplicitOnly);
    game.apply(a("e2e4")).unwrap();
    let expected = r#"{"schema_version":1,"rules_profile":"mshogi.chess.match.v1","initial_fen":"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1","history_origin":"standard_start","claim_policy":"explicit_only","events":[{"ply":0,"event":{"kind":"move","action":"e2e4"}}]}"#;
    assert_eq!(game.record().to_json(), expected);
    let decoded = GameRecord::from_json(expected).unwrap();
    assert_eq!(decoded.replay_history().unwrap().game, game);
    assert_eq!(
        GameRecord::from_json(
            &serde_json::to_string_pretty(&serde_json::from_str::<Value>(expected).unwrap())
                .unwrap()
        )
        .unwrap()
        .to_json(),
        expected
    );
}

#[test]
fn ordered_same_ply_observations_and_intended_claim_are_preserved_without_execution() {
    let mut record = Game::from_fen("7k/8/8/8/8/8/8/KR6 w - - 99 999", ClaimPolicy::AutoClaim)
        .unwrap()
        .record();
    let observations = vec![
        MatchObservation::Clock {
            at_ns: 100,
            remaining_ns: [90, 100],
            active: Some(Color::White),
        },
        MatchObservation::Command {
            actor: Color::White,
            command: RecordedCommand::ClaimDraw {
                reason: DrawClaim::FiftyMove,
                intended: Some(a("b1b2")),
            },
        },
        MatchObservation::Clock {
            at_ns: 101,
            remaining_ns: [89, 100],
            active: None,
        },
    ];
    for o in &observations {
        record.push_event(event(0, RecordEventKind::Observation(o.clone())));
    }
    let text = record.to_json();
    let parsed = GameRecord::from_json(&text).unwrap();
    assert_eq!(parsed, record);
    assert_eq!(parsed.to_json(), text);
    let replay = parsed.replay_history().unwrap();
    assert_eq!(replay.game.ply_count(), 0);
    assert_eq!(replay.game.current_occurrences(), 1);
    assert_eq!(replay.game.claim_policy(), ClaimPolicy::AutoClaim);
    assert_eq!(replay.game.history_origin(), HistoryOrigin::ImportedFen);
    assert_eq!(replay.game.position().halfmove_clock(), 99);
    assert_eq!(replay.game.position().fullmove_number(), 999);
    for (index, o) in replay.observations.iter().enumerate() {
        assert_eq!(o.sequence, index + 1);
        assert_eq!(o.ply, 0);
        assert_eq!(o.position, replay.game.position().clone());
        assert_eq!(o.observation, observations[index]);
    }
    // These are observations, not accepted claims: appending a played action is
    // transportable here. MSH-17/18 will decide whether such a live match is valid.
    record.push_event(event(0, RecordEventKind::Move(a("b1b2"))));
    record.push_event(event(
        1,
        RecordEventKind::Observation(MatchObservation::Command {
            actor: Color::Black,
            command: RecordedCommand::Resign,
        }),
    ));
    let replay = GameRecord::from_json(&record.to_json())
        .unwrap()
        .replay_history()
        .unwrap();
    assert_eq!(replay.game.ply_count(), 1);
    assert_eq!(replay.observations[3].sequence, 5);
    assert_eq!(replay.observations[3].ply, 1);
    assert_eq!(replay.observations[3].position.halfmove_clock(), 100);
}

#[test]
fn every_observation_variant_round_trips_and_preserves_context() {
    let commands = [
        RecordedCommand::Resign,
        RecordedCommand::OfferDraw,
        RecordedCommand::AcceptDraw,
        RecordedCommand::DeclineDraw,
        RecordedCommand::ClaimDraw {
            reason: DrawClaim::Threefold,
            intended: None,
        },
        RecordedCommand::ClaimDraw {
            reason: DrawClaim::Threefold,
            intended: Some(a("e7e8n")),
        },
    ];
    let mut record = Game::new(ClaimPolicy::ExplicitOnly).record();
    record.push_event(event(0, RecordEventKind::Move(a("e2e4"))));
    for actor in [Color::White, Color::Black] {
        for c in &commands {
            record.push_event(event(
                1,
                RecordEventKind::Observation(MatchObservation::Command {
                    actor,
                    command: c.clone(),
                }),
            ));
        }
    }
    for active in [None, Some(Color::White), Some(Color::Black)] {
        record.push_event(event(
            1,
            RecordEventKind::Observation(MatchObservation::Clock {
                at_ns: u64::MAX,
                remaining_ns: [0, u64::MAX],
                active,
            }),
        ));
    }
    let parsed = GameRecord::from_json(&record.to_json()).unwrap();
    assert_eq!(parsed, record);
    let replay = parsed.replay_history().unwrap();
    assert_eq!(replay.observations.len(), 15);
    assert_eq!(replay.game.actions().collect::<Vec<_>>(), vec![a("e2e4")]);
    for (i, o) in replay.observations.iter().enumerate() {
        assert_eq!(o.sequence, i + 2);
        assert_eq!(o.ply, 1);
        assert_eq!(o.position, replay.game.position().clone());
    }
}

#[test]
fn invalid_versions_metadata_effects_contexts_and_actions_are_rejected() {
    let base = Game::new(ClaimPolicy::ExplicitOnly).record().to_json();
    let value: Value = serde_json::from_str(&base).unwrap();
    for (field, replacement) in [
        ("schema_version", json!(2)),
        ("rules_profile", json!("unknown")),
        ("history_origin", json!("inferred")),
        ("claim_policy", json!("invented")),
        ("initial_fen", json!("invalid")),
        ("private_undo", json!([])),
    ] {
        let mut v = value.clone();
        v[field] = replacement;
        assert!(GameRecord::from_json(&v.to_string()).is_err(), "{field}");
    }
    assert!(
        GameRecord::from_json(&base.replace(
            "\"schema_version\":1",
            "\"schema_version\":1,\"schema_version\":1"
        ))
        .is_err()
    );
    let mut fake = value.clone();
    fake["initial_fen"] = json!("7k/8/8/8/8/8/8/K7 w - - 0 1");
    assert!(matches!(
        GameRecord::from_json(&fake.to_string()),
        Err(RecordError::Origin)
    ));
    for payload in [
        json!({"kind":"move","action":"e2e4","victims":[]}),
        json!({"kind":"move","action":"e7e8x"}),
        json!({"kind":"clock","at_ns":-1,"remaining_ns":[0,0],"active":null}),
        json!({"kind":"clock","at_ns":0,"remaining_ns":[0,0],"active":"red"}),
        json!({"kind":"command","actor":"white","command":{"kind":"resign","effects":[]}}),
    ] {
        let mut v = value.clone();
        v["events"] = json!([{"ply":0,"event":payload}]);
        assert!(GameRecord::from_json(&v.to_string()).is_err(), "{payload}");
    }
    let mut record = GameRecord::from_json(&base).unwrap();
    record.push_event(event(1, RecordEventKind::Move(a("e2e4"))));
    assert!(matches!(
        record.replay_history(),
        Err(RecordError::Context(1))
    ));
    let mut record = GameRecord::from_json(&base).unwrap();
    record.push_event(event(0, RecordEventKind::Move(a("e2e4"))));
    record.push_event(event(1, RecordEventKind::Move(a("e7e4"))));
    assert!(matches!(
        record.replay_history(),
        Err(RecordError::Action(2, _))
    ));
}

#[test]
fn strict_coordinate_identity_is_shared_without_legalizing_actions() {
    for text in ["e2e4", "e1g1", "e5d6", "e7e8q", "e7e8r", "e7e8b", "e7e8n"] {
        assert_eq!(a(text).to_string(), text);
    }
    for text in [
        "", "E2e4", "e2e9", "e2e4q!", "e2e4 ", " e2e4", "e7e8k", "é2e4", "e2e4\n",
    ] {
        assert!(text.parse::<Action>().is_err(), "{text}");
    }
    // Syntax carries identity only; null moves are refused by checked execution.
    let mut game = Game::new(ClaimPolicy::ExplicitOnly);
    assert_eq!(game.apply(a("e2e2")), Err(ActionError::SameSquare));
}
