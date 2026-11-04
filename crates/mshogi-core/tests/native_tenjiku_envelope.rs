//! Public shared routing retains each native Tenjiku profile and its derived facts.
use mshogi_core::{native::*, tenjiku as t};
use serde_json::{Value, json};

fn square(file: u8, rank: u8) -> t::Square {
    t::Square::new(file, rank).unwrap()
}

fn fixture_game(profile: t::Profile) -> t::Game {
    let entries = [
        (16, 16, t::Side::Sente, t::Base::K),
        (15, 14, t::Side::Sente, t::Base::P),
        (8, 10, t::Side::Sente, t::Base::FiD),
        (1, 1, t::Side::Gote, t::Base::K),
        (2, 3, t::Side::Gote, t::Base::P),
        (9, 8, t::Side::Gote, t::Base::FiD),
        (8, 7, t::Side::Gote, t::Base::P),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (file, rank, side, base))| {
        (
            square(file, rank),
            t::Piece::new(id as u16, side, base, false).unwrap(),
        )
    })
    .collect::<Vec<_>>();
    t::Game::from_root(
        t::Position::from_diagram(
            profile,
            &entries,
            t::Side::Sente,
            t::RootMetadata::imported("shared-tenjiku-fixture".into(), 120).unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}

fn wrapped(game: &t::Game) -> CanonicalRecord {
    CanonicalRecord::tenjiku(t::GameRecord::from_game(game).unwrap())
}

fn other(profile: t::Profile) -> t::Profile {
    match profile {
        t::Profile::AdamsModern => t::Profile::MullerCompleted,
        t::Profile::MullerCompleted => t::Profile::AdamsModern,
    }
}

fn play(game: &mut t::Game) {
    let request = t::GameRequest {
        expected: game.context(),
        actor: t::Side::Sente,
        command: t::GameCommand::Play {
            action: t::Action::single(square(8, 10), square(8, 8)),
        },
    };
    assert!(matches!(
        dispatch_command(
            NativeCommandTarget::Tenjiku(game),
            NativeCommand::Tenjiku(Box::new(request)),
        )
        .unwrap(),
        NativeReply::Tenjiku(Ok(_))
    ));
}

#[test]
fn both_profiles_keep_native_effects_root_history_and_fresh_authority() {
    for profile in t::Profile::ALL {
        let mut game = fixture_game(profile);
        let root = wrapped(&game);
        let root_context = game.context();
        let action = t::Action::single(square(8, 10), square(8, 8));
        assert!(
            game.local_position()
                .inspection(square(8, 10))
                .iter()
                .any(|c| c.action == action)
        );
        assert_eq!(wrapped(&game), root);
        play(&mut game);
        let effects = game.events()[0].effects.as_ref().unwrap();
        assert!(effects.mover_after.is_none());
        if profile == t::Profile::AdamsModern {
            assert!(
                matches!(&effects.phases[0], t::EffectPhase::Burn { victims }
                if victims.len() == 1 && victims[0].square == square(8, 7))
            );
            assert!(matches!(
                effects.phases[1],
                t::EffectPhase::MoverDies { .. }
            ));
        } else {
            assert_eq!(effects.phases.len(), 1);
            assert!(matches!(
                effects.phases[0],
                t::EffectPhase::MoverDies { .. }
            ));
        }
        let expected = game.context();
        assert!(matches!(
            dispatch_command(
                NativeCommandTarget::Tenjiku(&mut game),
                NativeCommand::Tenjiku(Box::new(t::GameRequest {
                    expected,
                    actor: t::Side::Gote,
                    command: t::GameCommand::OfferDraw,
                })),
            )
            .unwrap(),
            NativeReply::Tenjiku(Ok(_))
        ));
        let record = wrapped(&game);
        assert_eq!(record.game_kind(), GameKind::Tenjiku);
        assert_eq!(record.profile(), RulesProfile::from(profile));
        assert_eq!(record.profile().tenjiku(), Some(profile));
        assert_eq!(record.profile().id(), profile.id());
        assert_eq!(record.profile().edition(), profile.edition());
        assert_eq!(record.native_schema(), NativeSchema::TenjikuV1);
        assert_eq!(record.event_count(), 2);
        assert_eq!(record.known_board_actions(), 1);
        let raw = record.to_json().unwrap();
        assert_eq!(serde_json::to_string(&record).unwrap(), raw);
        assert_eq!(CanonicalRecord::from_json(&raw).unwrap(), record);
        assert_eq!(
            serde_json::from_str::<CanonicalRecord>(&raw).unwrap(),
            record
        );
        let CanonicalRecord::Tenjiku(restored) = CanonicalRecord::from_json(&raw).unwrap() else {
            panic!("lost native Tenjiku record")
        };
        let restored = restored.to_game().unwrap();
        assert!(game.context().same_semantics(&restored.context()));
        assert_ne!(game.context(), restored.context());
        assert_eq!(restored.profile(), profile);
        assert_eq!(
            restored.position().root().source(),
            Some("shared-tenjiku-fixture")
        );
        assert_eq!(restored.position().root().declared_completed_plies(), 120);
        assert_eq!(restored.snapshot().draw_offer, Some(t::Side::Gote));
        assert_eq!(
            CanonicalRecord::tenjiku(restored.record_prefix(0).unwrap()),
            root
        );
        let accepted = game.context();
        assert!(matches!(
            dispatch_command(
                NativeCommandTarget::Tenjiku(&mut game),
                NativeCommand::Tenjiku(Box::new(t::GameRequest {
                    expected: root_context,
                    actor: t::Side::Sente,
                    command: t::GameCommand::Resign,
                })),
            )
            .unwrap(),
            NativeReply::Tenjiku(Err(t::GameError::StaleContext))
        ));
        assert_eq!(game.context(), accepted);
        assert_eq!(wrapped(&game), record);
    }
    assert_eq!(RulesProfile::ChessMatchV1.tenjiku(), None);
    assert_ne!(
        RulesProfile::from(t::Profile::AdamsModern),
        RulesProfile::from(t::Profile::MullerCompleted)
    );
}

#[test]
fn exact_profile_edition_schema_and_duplicate_preserving_native_payload() {
    for profile in t::Profile::ALL {
        let mut game = fixture_game(profile);
        play(&mut game);
        let record = wrapped(&game);
        let raw = record.to_json().unwrap();
        let mut value: Value = serde_json::from_str(&raw).unwrap();
        value["rules_profile"] = json!(other(profile).id());
        value["profile_edition"] = json!(other(profile).edition());
        assert_eq!(
            CanonicalRecord::from_json(&value.to_string()).unwrap_err(),
            EnvelopeError::Identity
        );
        assert!(serde_json::from_value::<CanonicalRecord>(value).is_err());
        for (field, value) in [
            ("profile_edition", json!(other(profile).edition())),
            ("native_schema_version", json!(2)),
            ("envelope_version", json!(2)),
            ("game_kind", json!("chu")),
        ] {
            let mut changed: Value = serde_json::from_str(&raw).unwrap();
            changed[field] = value;
            assert!(
                CanonicalRecord::from_json(&changed.to_string()).is_err(),
                "{field}"
            );
        }
        let duplicate = raw.replacen(
            "\"promote\":false",
            "\"promote\":false,\"promote\":false",
            1,
        );
        assert_ne!(duplicate, raw);
        assert!(CanonicalRecord::from_json(&duplicate).is_err());
        assert!(serde_json::from_str::<CanonicalRecord>(&duplicate).is_err());
        let profile_key = format!("\"rules_profile\":\"{}\"", profile.id());
        let duplicate = raw.replacen(&profile_key, &format!("{profile_key},{profile_key}"), 1);
        assert_ne!(duplicate, raw);
        assert!(CanonicalRecord::from_json(&duplicate).is_err());
        let mut forged: Value = serde_json::from_str(&raw).unwrap();
        if profile == t::Profile::AdamsModern {
            forged["payload"]["events"][0]["effects"]["phases"][0]["victims"][0]["square"] =
                json!([8, 6]);
        } else {
            forged["payload"]["events"][0]["effects"]["phases"][0]["square"] = json!([8, 9]);
        }
        assert!(CanonicalRecord::from_json(&forged.to_string()).is_err());
        assert_eq!(wrapped(&game), record);
    }
    for side in [t::Side::Sente, t::Side::Gote] {
        let qualified = NativeSide::Tenjiku(side);
        assert_eq!(qualified.game(), GameKind::Tenjiku);
        assert_eq!(
            serde_json::from_str::<NativeSide>(&serde_json::to_string(&qualified).unwrap())
                .unwrap(),
            qualified
        );
    }
    for raw in [
        r#"{"game_kind":"tenjiku","side":"white"}"#,
        r#"{"game_kind":"tenjiku","side":"red"}"#,
        r#"{"game_kind":"tenjiku","side":"sente","side":"gote"}"#,
    ] {
        assert!(serde_json::from_str::<NativeSide>(raw).is_err());
    }
}

#[test]
fn cross_profile_and_wrong_game_commands_are_atomic_refusals() {
    for profile in t::Profile::ALL {
        let mut game = fixture_game(profile);
        let other = fixture_game(other(profile));
        let before = wrapped(&game);
        let authority = game.context();
        assert!(matches!(
            dispatch_command(
                NativeCommandTarget::Tenjiku(&mut game),
                NativeCommand::Tenjiku(Box::new(t::GameRequest {
                    expected: other.context(),
                    actor: t::Side::Sente,
                    command: t::GameCommand::Resign,
                })),
            )
            .unwrap(),
            NativeReply::Tenjiku(Err(t::GameError::StaleContext))
        ));
        let tori = mshogi_core::tori::Game::standard();
        assert!(
            dispatch_command(
                NativeCommandTarget::Tenjiku(&mut game),
                NativeCommand::Tori(Box::new(mshogi_core::tori::GameRequest {
                    expected: tori.context(),
                    actor: mshogi_core::tori::Side::Sente,
                    command: mshogi_core::tori::GameCommand::Resign,
                })),
            )
            .is_err()
        );
        assert_eq!(wrapped(&game), before);
        assert_eq!(game.context(), authority);
    }
}

#[test]
fn complete_wrapper_input_bound_and_cooperative_native_replay_stop() {
    for profile in t::Profile::ALL {
        let mut game = fixture_game(profile);
        play(&mut game);
        let record = wrapped(&game);
        let raw = record.to_json().unwrap();
        let exact = format!("{raw}{}", " ".repeat(MAX_RECORD_ENVELOPE_BYTES - raw.len()));
        assert_eq!(exact.len(), MAX_RECORD_ENVELOPE_BYTES);
        assert_eq!(CanonicalRecord::from_json(&exact).unwrap(), record);
        assert_eq!(
            CanonicalRecord::from_json(&(exact + " ")).unwrap_err(),
            EnvelopeError::Capacity
        );
        assert_eq!(
            CanonicalRecord::from_json_with_stop(&raw, &mut || true).unwrap_err(),
            EnvelopeError::Stopped
        );
        let mut checks = 0;
        let error = CanonicalRecord::from_json_with_stop(&raw, &mut || {
            checks += 1;
            checks >= 20
        })
        .unwrap_err();
        assert_eq!(error, EnvelopeError::Stopped);
        assert_eq!(checks, 20);
        assert_eq!(wrapped(&game), record);
    }
}

#[test]
fn native_event_budget_refusal_remains_typed_below_the_wrapper_byte_cap() {
    for profile in t::Profile::ALL {
        let mut game = fixture_game(profile);
        game.submit(t::GameRequest {
            expected: game.context(),
            actor: t::Side::Sente,
            command: t::GameCommand::OfferDraw,
        })
        .unwrap();
        let record = wrapped(&game);
        let authority = game.context();
        let mut wire: Value = serde_json::from_str(&record.to_json().unwrap()).unwrap();
        let event = wire["payload"]["events"][0].clone();
        // Syntactically complete native events exceed the event budget before
        // replay. This exercises native capacity, independently of byte limits.
        wire["payload"]["events"] = json!(vec![event; t::MAX_GAME_EVENTS + 1]);
        let payload = wire["payload"].to_string();
        let raw = wire.to_string();
        assert!(payload.len() < t::MAX_GAME_RECORD_BYTES);
        assert!(raw.len() < MAX_RECORD_ENVELOPE_BYTES);
        assert_eq!(
            t::GameRecord::from_json(&payload).unwrap_err(),
            t::RecordError::Capacity
        );
        assert_eq!(
            CanonicalRecord::from_json(&raw).unwrap_err(),
            EnvelopeError::Capacity
        );
        assert_eq!(
            CanonicalRecord::from_json_with_stop(&raw, &mut || false).unwrap_err(),
            EnvelopeError::Capacity
        );
        assert_eq!(
            CanonicalRecord::from_json_with_stop(&raw, &mut || true).unwrap_err(),
            EnvelopeError::Stopped
        );
        assert!(serde_json::from_str::<CanonicalRecord>(&raw).is_err());
        assert_eq!(wrapped(&game), record);
        assert_eq!(game.context(), authority);
    }
}
