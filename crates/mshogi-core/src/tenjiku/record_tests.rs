use super::*;
use serde_json::{Value, json};

fn command(game: &mut Game, actor: Side, command: GameCommand) {
    game.submit(GameRequest {
        expected: game.context(),
        actor,
        command,
    })
    .unwrap();
}
fn sample(profile: Profile) -> Game {
    let mut game = Game::standard(profile);
    command(&mut game, Side::Sente, GameCommand::OfferDraw);
    command(&mut game, Side::Gote, GameCommand::DeclineDraw);
    command(&mut game, Side::Sente, GameCommand::Resign);
    game
}
#[test]
fn both_profiles_replay_actual_commands_metadata_and_fresh_authority() {
    for profile in Profile::ALL {
        let game = sample(profile);
        let record = GameRecord::from_game(&game).unwrap();
        assert_eq!(record.profile(), profile);
        assert_eq!(record.event_count(), 3);
        assert_eq!(record.known_board_actions(), 0);
        assert_eq!(record.side_to_move(), Side::Sente);
        assert_eq!(record.conclusion(), game.snapshot().conclusion);
        assert_eq!(record.reconstruction_work(), game.replay_work());
        let decoded = GameRecord::from_json(record.to_json()).unwrap();
        assert_eq!(decoded, record);
        let restored = decoded.to_game().unwrap();
        assert_eq!(restored.events(), game.events());
        assert_eq!(restored.position().state(), game.position().state());
        assert_ne!(restored.context(), game.context());
        assert_eq!(GameRecord::from_game(&restored).unwrap(), record);
        assert_eq!(encoded_size(&game).unwrap(), record.to_json().len());
    }
}
#[test]
fn ordinary_replay_retains_no_undo_but_explicit_replay_returns_real_lifo_handles() {
    for profile in Profile::ALL {
        let source = sample(profile);
        let record = GameRecord::from_game(&source).unwrap();
        let (plain, no_undos) = GameRecord::replay(record.as_json(), false, &mut || false).unwrap();
        assert!(no_undos.is_empty());
        assert_eq!(GameRecord::from_game(&plain).unwrap(), record);
        let (mut game, mut undos) = record.to_game_with_undo().unwrap();
        assert_eq!(undos.len(), 3);
        for undo in undos.iter_mut().rev() {
            game.undo(undo).unwrap();
        }
        assert!(game.events().is_empty());
        assert_eq!(game.position().state(), source.native_root());
        assert_eq!(
            game.snapshot().conclusion,
            source.root_snapshot().conclusion
        );
        assert!(game.undo(&mut undos[0]).is_err());
    }
}
#[test]
fn strict_raw_parser_rejects_duplicates_including_escaped_keys_before_tag_buffering() {
    let record = GameRecord::from_game(&sample(Profile::AdamsModern)).unwrap();
    let raw = record.to_json();
    for invalid in [
        raw.replacen("\"profile\":", "\"profile\":null,\"profile\":", 1),
        raw.replacen(
            "\"kind\":\"offer_draw\"",
            "\"kind\":\"offer_draw\",\"k\\u0069nd\":\"offer_draw\"",
            1,
        ),
        format!("{raw} {{}}"),
    ] {
        assert_ne!(invalid, raw);
        assert_eq!(GameRecord::from_json(&invalid), Err(RecordError::Json));
    }
    assert_eq!(
        GameCommand::from_json(r#"{"kind":"resign","k\u0069nd":"resign"}"#),
        Err(RecordError::Json)
    );
    assert!(GameCommand::from_json(r#"{"kind":"resign","victims":[]}"#).is_err());
}
#[test]
fn identities_and_all_redundant_fields_are_rederived_not_trusted() {
    let record = GameRecord::from_game(&sample(Profile::AdamsModern)).unwrap();
    let value: Value = serde_json::from_str(record.to_json()).unwrap();
    type Mutation = Box<dyn Fn(&mut Value)>;
    let mutations: Vec<Mutation> = vec![
        Box::new(|v| v["profile"] = json!(Profile::MullerCompleted)),
        Box::new(|v| v["edition"] = json!(Profile::MullerCompleted.edition())),
        Box::new(|v| v["schema_version"] = json!(2)),
        Box::new(|v| v["root"]["profile"] = json!(Profile::MullerCompleted)),
        Box::new(|v| v["current_position"]["profile"] = json!(Profile::MullerCompleted)),
        Box::new(|v| {
            v.as_object_mut().unwrap().remove("root_incomplete");
        }),
        Box::new(|v| v["events"][0]["actor"] = json!("gote")),
        Box::new(|v| v["events"][0]["board_ply"] = json!(1)),
        Box::new(|v| v["root_draw_offer"] = json!("sente")),
        Box::new(|v| v["draw_offer"] = json!("gote")),
        Box::new(|v| v["conclusion"] = Value::Null),
        Box::new(|v| v["unrecognized"] = json!(true)),
    ];
    for mutate in mutations {
        let mut changed = value.clone();
        mutate(&mut changed);
        assert!(
            GameRecord::from_json(&changed.to_string()).is_err(),
            "accepted forged record {changed}"
        );
    }
    assert_eq!(
        GameRecord::from_game(&record.to_game().unwrap()).unwrap(),
        record
    );
}
#[test]
fn cancellation_is_distinct_and_never_changes_record_or_source_game() {
    let game = sample(Profile::MullerCompleted);
    let record = GameRecord::from_game(&game).unwrap();
    for stop_at in [1, 4, 7, 9] {
        let mut calls = 0;
        assert_eq!(
            GameRecord::from_json_with_stop(record.to_json(), &mut || {
                calls += 1;
                calls == stop_at
            }),
            Err(RecordError::Stopped)
        );
        assert_eq!(calls, stop_at);
    }
    assert!(matches!(
        record.to_game_with_stop(&mut || true),
        Err(RecordError::Stopped)
    ));
    assert_eq!(GameRecord::from_game(&game).unwrap(), record);
}
#[test]
fn oversized_native_input_and_missing_nullable_summary_refuse() {
    assert_eq!(
        GameRecord::from_json(&" ".repeat(MAX_GAME_RECORD_BYTES + 1)),
        Err(RecordError::Capacity)
    );
    let game = Game::standard(Profile::AdamsModern);
    let record = GameRecord::from_game(&game).unwrap();
    let mut wire: Value = serde_json::from_str(record.to_json()).unwrap();
    wire.as_object_mut().unwrap().remove("pending_bare");
    assert_eq!(
        GameRecord::from_json(&wire.to_string()),
        Err(RecordError::Derived)
    );
}

#[test]
fn cached_event_size_equals_actual_encoding_for_board_controls_and_reversal() {
    for profile in Profile::ALL {
        let square = |file, rank| Square::new(file, rank).unwrap();
        let pieces = [
            (
                square(16, 16),
                Piece::new(0, Side::Sente, Base::K, false).unwrap(),
            ),
            (
                square(16, 14),
                Piece::new(1, Side::Sente, Base::P, false).unwrap(),
            ),
            (
                square(1, 1),
                Piece::new(2, Side::Gote, Base::K, false).unwrap(),
            ),
            (
                square(1, 3),
                Piece::new(3, Side::Gote, Base::P, false).unwrap(),
            ),
        ];
        let mut game = Game::from_root(
            Position::from_diagram(profile, &pieces, Side::Sente, RootMetadata::diagram(0))
                .unwrap(),
        )
        .unwrap();
        let commands = [
            (
                Side::Sente,
                GameCommand::Play {
                    action: Action::single(square(16, 14), square(16, 13)),
                },
            ),
            (
                Side::Gote,
                GameCommand::Play {
                    action: Action::single(square(1, 3), square(1, 4)),
                },
            ),
            (Side::Sente, GameCommand::OfferDraw),
            (Side::Gote, GameCommand::DeclineDraw),
            (Side::Sente, GameCommand::Resign),
        ];
        let mut undos = vec![];
        assert_eq!(encoded_size(&game).unwrap(), encode(&game).unwrap().len());
        for (actor, command) in commands {
            undos.push(
                game.submit(GameRequest {
                    expected: game.context(),
                    actor,
                    command,
                })
                .unwrap(),
            );
            assert_eq!(encoded_size(&game).unwrap(), encode(&game).unwrap().len());
        }
        for undo in undos.iter_mut().rev() {
            game.undo(undo).unwrap();
            assert_eq!(encoded_size(&game).unwrap(), encode(&game).unwrap().len());
        }
    }
}
