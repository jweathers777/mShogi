//! Native record rejection and replay contracts. Governing rule expectations
//! come from the independently constructed MSH-67 history corpus, not this codec.

use mshogi_core::shogi::{
    Conclusion, CounterSource, Game, GameCommand, GameRecord, GameRequest, GameRoot, INITIAL_SFEN,
    MAX_RECORD_BYTES, MAX_RECORD_EVENTS, Position, RecordError, RootCategory, Side, WinReason,
};
use serde_json::{Value, json};

fn submit(game: &mut Game, actor: Side, command: GameCommand) {
    game.submit(GameRequest {
        expected: game.context(),
        actor,
        command,
    })
    .unwrap();
}
fn play(game: &mut Game, action: &str) {
    submit(
        game,
        game.position().side_to_move(),
        GameCommand::Play(action.parse().unwrap()),
    );
}
fn imported(sfen: &str, source: CounterSource) -> Game {
    Game::from_root(
        GameRoot::import(
            Position::from_sfen(sfen).unwrap(),
            RootCategory::ReducedMaterialDiagram,
            source,
        )
        .unwrap(),
    )
}
fn value(game: &Game) -> Value {
    serde_json::from_str(&GameRecord::from_game(game).to_json().unwrap()).unwrap()
}
fn rejected(value: &Value) {
    assert!(
        GameRecord::from_json(&value.to_string()).is_err(),
        "accepted {value}"
    );
}

#[test]
fn standard_round_trip_retains_every_prefix_and_same_ply_terminal_commands() {
    let mut game = Game::new();
    play(&mut game, "7g7f");
    play(&mut game, "3c3d");
    let before_resign = game.clone();
    submit(&mut game, Side::Sente, GameCommand::Resign);
    assert_eq!(game.position(), before_resign.position());
    assert_eq!(
        game.conclusion(),
        Some(Conclusion::Win {
            winner: Side::Gote,
            reason: WinReason::Resignation
        })
    );
    let record = GameRecord::from_game(&game);
    let json = record.to_json().unwrap();
    assert!(!json.ends_with('\n'));
    assert_eq!(
        GameRecord::from_json(&json).unwrap().to_json().unwrap(),
        json
    );
    assert_eq!(record.to_game().unwrap(), game);
    for n in 0..=3 {
        assert_eq!(
            record.prefix(n).unwrap().to_game().unwrap(),
            game.prefix(n).unwrap()
        );
    }
    assert_eq!(record.prefix(2).unwrap().to_game().unwrap(), before_resign);
    assert_ne!(record.prefix(2).unwrap(), record.prefix(3).unwrap());
    assert_eq!(record.prefix(4), Err(RecordError::Prefix));
}

#[test]
fn independently_established_history_and_declaration_ledgers_round_trip() {
    let corpus: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/shogi/history-outcomes.json"
    ))
    .unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        let mut game = imported(case["root_sfen"].as_str().unwrap(), CounterSource::Supplied);
        for action in case["moves"].as_array().unwrap() {
            play(&mut game, action.as_str().unwrap());
        }
        let record = GameRecord::from_game(&game);
        let json = record.to_json().unwrap();
        assert_eq!(
            GameRecord::from_json(&json).unwrap().to_game().unwrap(),
            game,
            "{}",
            case["id"]
        );
        for n in 0..=game.events().len() {
            assert_eq!(
                record.prefix(n).unwrap().to_game().unwrap(),
                game.prefix(n).unwrap(),
                "{} prefix {n}",
                case["id"]
            );
        }
    }
    for case in corpus["declaration_cases"].as_array().unwrap() {
        let mut game = imported(case["root_sfen"].as_str().unwrap(), CounterSource::Supplied);
        let actor = if case["actor"] == "sente" {
            Side::Sente
        } else {
            Side::Gote
        };
        submit(&mut game, actor, GameCommand::DeclareEnteringKing);
        let json = GameRecord::from_game(&game).to_json().unwrap();
        let record = GameRecord::from_json(&json).unwrap();
        assert_eq!(record.to_game().unwrap(), game, "{}", case["id"]);
        assert!(
            record
                .prefix(0)
                .unwrap()
                .to_game()
                .unwrap()
                .conclusion()
                .is_none()
        );
        assert_eq!(
            record.prefix(1).unwrap().to_game().unwrap().position(),
            game.root().position()
        );
    }
}

#[test]
fn root_category_origin_and_counter_provenance_are_enforced() {
    let full = Game::from_root(
        GameRoot::import(
            Position::initial(),
            RootCategory::FullMaterialDiagram,
            CounterSource::Supplied,
        )
        .unwrap(),
    );
    let full_wire = value(&full);
    assert_eq!(full_wire["root"]["known_history"], "imported_unknown");
    assert_eq!(
        GameRecord::from_json(&full_wire.to_string())
            .unwrap()
            .to_game()
            .unwrap(),
        full
    );
    let mut bad = full_wire.clone();
    bad["root"]["category"] = json!("reduced_material_diagram");
    rejected(&bad);
    bad = full_wire.clone();
    bad["root"]["known_history"] = json!("initial");
    rejected(&bad);
    bad = full_wire;
    bad["root"]["category"] = json!("standard");
    rejected(&bad);

    let supplied = imported("4k4/9/5R3/9/9/9/9/9/K8 b - 13", CounterSource::Supplied);
    let mut bad = value(&supplied);
    bad["root"]["counter_source"] = json!("defaulted");
    rejected(&bad);
    bad = value(&Game::new());
    bad["root"]["sfen"] = json!(supplied.position().to_sfen());
    rejected(&bad);
    bad = value(&Game::new());
    bad["root"]["counter_source"] = json!("supplied");
    rejected(&bad);

    let defaulted = imported("4k4/9/5R3/9/9/9/9/9/K8 b - 1", CounterSource::Defaulted);
    assert_eq!(
        GameRecord::from_json(&GameRecord::from_game(&defaulted).to_json().unwrap())
            .unwrap()
            .to_game()
            .unwrap(),
        defaulted
    );
    let mut moved_bare_position = Position::initial();
    moved_bare_position.apply("7g7f".parse().unwrap()).unwrap();
    assert_eq!(
        moved_bare_position.history_origin(),
        mshogi_core::shogi::HistoryOrigin::Initial
    );
    let imported_moved = Game::from_root(
        GameRoot::import(
            moved_bare_position,
            RootCategory::FullMaterialDiagram,
            CounterSource::Supplied,
        )
        .unwrap(),
    );
    let wire = value(&imported_moved);
    assert_eq!(wire["root"]["known_history"], "imported_unknown");
    let mut bad = wire;
    bad["root"]["category"] = json!("standard");
    bad["root"]["counter_source"] = json!("initial_fixed");
    bad["root"]["known_history"] = json!("initial");
    rejected(&bad);
}

#[test]
fn every_derived_effect_is_checked_even_if_final_state_is_unchanged() {
    let mut game = Game::new();
    play(&mut game, "7g7f");
    play(&mut game, "3c3d");
    submit(&mut game, Side::Gote, GameCommand::Resign);
    let valid = value(&game);
    for (field, replacement) in [
        ("index", json!(1)),
        ("actor", json!("gote")),
        ("gave_check", json!(true)),
        ("after_sfen", json!(INITIAL_SFEN)),
        ("conclusion", json!({"kind":"length"})),
        ("score_half_points", json!([1, 1])),
        (
            "declaration",
            json!({"own_turn":true,"king_in_camp":false,"camp_pieces":0,"points":0,"required_points":28,"not_in_check":true,"time_available":true}),
        ),
    ] {
        let mut bad = valid.clone();
        bad["events"][0][field] = replacement;
        assert_eq!(
            GameRecord::from_json(&bad.to_string()),
            Err(RecordError::Event(0)),
            "field {field}"
        );
    }
    let mut bad = valid.clone();
    bad["conclusion"]["winner"] = json!("gote");
    assert_eq!(
        GameRecord::from_json(&bad.to_string()),
        Err(RecordError::Conclusion)
    );
    bad = valid.clone();
    bad["score_half_points"] = json!([0, 2]);
    assert_eq!(
        GameRecord::from_json(&bad.to_string()),
        Err(RecordError::Conclusion)
    );
    bad = valid;
    bad["events"][2]["actor"] = json!("sente");
    assert_eq!(
        GameRecord::from_json(&bad.to_string()),
        Err(RecordError::Event(2))
    );
}

#[test]
fn declarations_retain_actor_and_evidence_and_reject_embedded_conflicting_actor() {
    let mut game = Game::new();
    submit(&mut game, Side::Gote, GameCommand::DeclareEnteringKing);
    assert_eq!(
        game.conclusion(),
        Some(Conclusion::Win {
            winner: Side::Sente,
            reason: WinReason::FalseDeclaration
        })
    );
    let valid = value(&game);
    assert_eq!(valid["events"][0]["declaration"]["own_turn"], false);
    let mut bad = valid.clone();
    bad["events"][0]["declaration"]["own_turn"] = json!(true);
    assert_eq!(
        GameRecord::from_json(&bad.to_string()),
        Err(RecordError::Event(0))
    );
    bad = valid.clone();
    bad["events"][0]["command"]["actor"] = json!("sente");
    rejected(&bad);
    bad = valid;
    bad["events"][0]["declaration"]
        .as_object_mut()
        .unwrap()
        .remove("time_available");
    rejected(&bad);
}

#[test]
fn automatic_root_conclusions_and_all_post_terminal_commands_are_validated() {
    let root_terminal = imported("4k4/9/5R3/9/9/9/9/9/K8 b - 321", CounterSource::Supplied);
    let mut bad = value(&root_terminal);
    bad["root"]["conclusion"] = Value::Null;
    assert_eq!(
        GameRecord::from_json(&bad.to_string()),
        Err(RecordError::Root)
    );
    let mut resignation = Game::new();
    submit(&mut resignation, Side::Sente, GameCommand::Resign);
    for command in [
        json!({"kind":"resign"}),
        json!({"kind":"declare_entering_king"}),
        json!({"kind":"play","action":"7g7f"}),
    ] {
        let mut bad = value(&resignation);
        let mut event = bad["events"][0].clone();
        event["index"] = json!(1);
        event["command"] = command.clone();
        bad["events"].as_array_mut().unwrap().push(event.clone());
        assert_eq!(
            GameRecord::from_json(&bad.to_string()),
            Err(RecordError::Event(1))
        );
        let mut bad = value(&root_terminal);
        event["index"] = json!(0);
        bad["events"].as_array_mut().unwrap().push(event);
        assert_eq!(
            GameRecord::from_json(&bad.to_string()),
            Err(RecordError::Event(0))
        );
    }
}

#[test]
fn strict_decode_rejects_duplicate_unknown_missing_and_unsupported_fields() {
    let mut game = Game::new();
    play(&mut game, "7g7f");
    submit(&mut game, Side::Gote, GameCommand::Resign);
    let valid = GameRecord::from_game(&game).to_json().unwrap();
    for (needle, replacement) in [
        (
            "\"schema_version\":1",
            "\"schema_version\":1,\"schema_version\":1",
        ),
        (
            "\"category\":\"standard\"",
            "\"category\":\"standard\",\"category\":\"standard\"",
        ),
        (
            "\"actor\":\"sente\"",
            "\"actor\":\"sente\",\"actor\":\"sente\"",
        ),
        ("\"kind\":\"play\"", "\"kind\":\"play\",\"kind\":\"play\""),
        (
            "\"action\":\"7g7f\"",
            "\"action\":\"7g7f\",\"action\":\"7g7f\"",
        ),
        (
            "\"kind\":\"resignation\"",
            "\"kind\":\"resignation\",\"winner\":\"sente\"",
        ),
    ] {
        let bad = valid.replacen(needle, replacement, 1);
        assert_ne!(bad, valid, "test needle {needle}");
        assert!(
            matches!(GameRecord::from_json(&bad), Err(RecordError::Json(_))),
            "accepted duplicate {needle}"
        );
    }
    let base = value(&game);
    for target in ["root", "policies", "conclusion"] {
        let mut bad = base.clone();
        bad[target]["unknown"] = json!(true);
        rejected(&bad);
    }
    for field in [
        "gave_check",
        "declaration",
        "conclusion",
        "score_half_points",
    ] {
        let mut bad = base.clone();
        bad["events"][0].as_object_mut().unwrap().remove(field);
        assert!(
            matches!(
                GameRecord::from_json(&bad.to_string()),
                Err(RecordError::Json(_))
            ),
            "missing {field}"
        );
    }
    for (field, replacement) in [
        ("mode", json!("timed")),
        ("schema_version", json!(2)),
        ("profile_id", json!("mshogi.shogi.jsa.v1")),
        ("profile_edition", json!("latest")),
    ] {
        let mut bad = base.clone();
        bad[field] = replacement;
        assert_eq!(
            GameRecord::from_json(&bad.to_string()),
            Err(RecordError::Unsupported)
        );
    }
    let mut bad = base.clone();
    bad["policies"]["length_completed_actions"] = json!(500);
    assert_eq!(
        GameRecord::from_json(&bad.to_string()),
        Err(RecordError::Unsupported)
    );
    bad = base;
    bad["policies"]["external_events"] = json!("referee_override");
    assert_eq!(
        GameRecord::from_json(&bad.to_string()),
        Err(RecordError::Unsupported)
    );
    assert!(GameRecord::from_json(&(valid.clone() + " {} ")).is_err());
    assert!(GameRecord::from_json(&valid.replace("\"index\":0", "\"index\":0.0")).is_err());
}

#[test]
fn byte_and_event_capacity_are_bounded_without_truncation() {
    assert_eq!(
        GameRecord::from_json(&" ".repeat(MAX_RECORD_BYTES + 1)),
        Err(RecordError::Capacity)
    );
    let mut game = Game::new();
    play(&mut game, "7g7f");
    let mut bad = value(&game);
    let event = bad["events"][0].clone();
    bad["events"] = Value::Array(vec![event; MAX_RECORD_EVENTS + 1]);
    assert_eq!(
        GameRecord::from_json(&bad.to_string()),
        Err(RecordError::Capacity)
    );
}

#[test]
fn maximum_retained_board_ledger_and_terminal_command_always_fit() {
    // Capacity exercise, not a playing-strength or governing-outcome oracle.
    // Pick deterministically among legal actions, avoiding early conclusions.
    let mut game = Game::new();
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
            let actor = candidate.position().side_to_move();
            submit(
                &mut candidate,
                actor,
                GameCommand::Play(choices[(start + offset) % choices.len()]),
            );
            if ply == 320 || candidate.conclusion().is_none() {
                selected = Some(candidate);
                break;
            }
        }
        game = selected.expect("fixed capacity exercise has a continuation");
    }
    assert_eq!(game.events().len(), 320);
    let encoded = GameRecord::from_game(&game).to_json().unwrap();
    assert!(encoded.len() < MAX_RECORD_BYTES);
    assert_eq!(
        GameRecord::from_json(&encoded).unwrap().to_game().unwrap(),
        game
    );
    let mut before_last = game.prefix(319).unwrap();
    submit(
        &mut before_last,
        Side::Sente,
        GameCommand::DeclareEnteringKing,
    );
    assert_eq!(before_last.events().len(), 320);
    let encoded = GameRecord::from_game(&before_last).to_json().unwrap();
    assert!(encoded.len() < MAX_RECORD_BYTES);
    assert_eq!(
        GameRecord::from_json(&encoded).unwrap().to_game().unwrap(),
        before_last
    );
}

#[test]
fn empty_command_and_conclusion_variants_reject_unknown_content() {
    let mut game = Game::new();
    submit(&mut game, Side::Sente, GameCommand::Resign);
    let mut bad = value(&game);
    bad["events"][0]["command"]["unknown"] = json!(true);
    assert!(matches!(
        GameRecord::from_json(&bad.to_string()),
        Err(RecordError::Json(_))
    ));
    let terminal = imported("4k4/9/5R3/9/9/9/9/9/K8 b - 321", CounterSource::Supplied);
    let mut bad = value(&terminal);
    bad["root"]["conclusion"]["unknown"] = json!(true);
    assert!(matches!(
        GameRecord::from_json(&bad.to_string()),
        Err(RecordError::Json(_))
    ));
}
