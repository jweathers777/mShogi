//! Real native core consumer: no fake desktop/session or engine implementation.
use mshogi_core::{native::*, tori as t};
use serde_json::{Value, json};
fn play(g: &mut t::Game, action: t::Action) -> Box<t::GameUndo> {
    let request = t::GameRequest {
        expected: g.context(),
        actor: g.position().side_to_move(),
        command: t::GameCommand::Play { action },
    };
    let NativeReply::Tori(result) = dispatch_command(
        NativeCommandTarget::Tori(g),
        NativeCommand::Tori(Box::new(request)),
    )
    .unwrap() else {
        panic!("wrong reply")
    };
    result.unwrap()
}
fn record(g: &t::Game) -> CanonicalRecord {
    CanonicalRecord::tori(t::GameRecord::from_game(g).unwrap())
}
#[test]
fn actual_core_consumer_uses_native_choices_effects_history_inspection_context_and_records() {
    let mut g = t::Game::standard();
    let before = g.context();
    let a = g.legal_actions()[0];
    let mut u = play(&mut g, a);
    assert_eq!(g.events().len(), 1);
    assert_eq!(g.events()[0].effects.unwrap().action, a);
    assert_eq!(g.events()[0].board_ply, 1);
    let inspection = g
        .local_position()
        .inspect(t::InspectionOrigin::Board("4g".parse().unwrap()));
    assert!(!inspection.is_empty());
    assert_ne!(g.context(), before);
    let r = record(&g);
    assert_eq!(r.game_kind(), GameKind::Tori);
    assert_eq!(r.profile(), RulesProfile::ToriModernV1);
    assert_eq!(r.profile().edition(), t::PROFILE_EDITION);
    assert_eq!(r.native_schema(), NativeSchema::ToriV1);
    assert_eq!(r.known_board_actions(), 1);
    assert_eq!(r.event_count(), 1);
    let text = r.to_json().unwrap();
    assert_eq!(CanonicalRecord::from_json(&text).unwrap(), r);
    assert_eq!(serde_json::from_str::<CanonicalRecord>(&text).unwrap(), r);
    let CanonicalRecord::Tori(native) = r else {
        panic!()
    };
    let mut restored = native.to_game().unwrap();
    assert!(g.context().same_semantics(&restored.context()));
    assert_ne!(g.context(), restored.context());
    assert_eq!(
        restored
            .submit(t::GameRequest {
                expected: g.context(),
                actor: t::Side::Gote,
                command: t::GameCommand::Resign
            })
            .unwrap_err(),
        t::GameError::StaleContext
    );
    let historical = g.select_prefix(0).unwrap();
    assert_eq!(historical.board_ply, 0);
    assert_eq!(historical.occurrences, [0]);
    assert!(historical.conclusion.is_none());
    assert!(!historical.local_position().legal_actions().is_empty());
    let mut redo = g.undo(&mut u).unwrap();
    assert!(g.context().same_semantics(&before));
    assert_ne!(g.context(), before);
    g.redo(&mut redo).unwrap();
    let actor = t::Side::Sente;
    let request = t::GameRequest {
        expected: g.context(),
        actor,
        command: t::GameCommand::Resign,
    };
    let NativeReply::Tori(result) = dispatch_command(
        NativeCommandTarget::Tori(&mut g),
        NativeCommand::Tori(Box::new(request)),
    )
    .unwrap() else {
        panic!()
    };
    result.unwrap();
    assert_eq!(
        g.conclusion(),
        Some(t::Conclusion::Resignation {
            winner: t::Side::Gote
        })
    );
    assert_eq!(record(&g).event_count(), 2);
    assert_eq!(record(&g).known_board_actions(), 1);
    for side in [t::Side::Sente, t::Side::Gote] {
        let side = NativeSide::Tori(side);
        assert_eq!(
            serde_json::from_str::<NativeSide>(&serde_json::to_string(&side).unwrap()).unwrap(),
            side
        );
        assert_eq!(side.game(), GameKind::Tori);
    }
}
#[test]
fn raw_outer_and_nested_identity_duplicates_unknown_missing_conflicts_never_fall_through() {
    let mut g = t::Game::standard();
    let a = g.legal_actions()[0];
    play(&mut g, a);
    let text = record(&g).to_json().unwrap();
    let original: Value = serde_json::from_str(&text).unwrap();
    for (field, value) in [
        ("game_kind", json!("shogi")),
        ("rules_profile", json!("mshogi.shogi.csa36.v1")),
        ("profile_edition", json!("other")),
        ("native_schema_version", json!(2)),
        ("envelope_version", json!(2)),
        ("unknown", json!(1)),
    ] {
        let mut v = original.clone();
        v[field] = value;
        assert!(
            CanonicalRecord::from_json(&v.to_string()).is_err(),
            "{field}"
        );
    }
    for key in [
        "envelope_version",
        "game_kind",
        "rules_profile",
        "profile_edition",
        "native_schema_version",
        "payload",
        "profile",
        "root_kind",
        "actor",
        "kind",
        "from",
        "result_face",
    ] {
        let needle = format!("\"{key}\":");
        let bad = text.replacen(&needle, &format!("\"{key}\":null,{needle}"), 1);
        assert!(CanonicalRecord::from_json(&bad).is_err(), "duplicate {key}");
        assert!(serde_json::from_str::<CanonicalRecord>(&bad).is_err());
    }
    for key in original.as_object().unwrap().keys() {
        let mut v = original.clone();
        v.as_object_mut().unwrap().remove(key);
        assert!(CanonicalRecord::from_json(&v.to_string()).is_err());
    }
    let before = g.context();
    let mut chess = mshogi_core::chess::ChessMatch::new(
        mshogi_core::chess::AdjudicatedGame::new(mshogi_core::chess::ClaimPolicy::ExplicitOnly),
        mshogi_core::chess::TimeControl::Untimed,
    )
    .unwrap();
    let request = t::GameRequest {
        expected: g.context(),
        actor: t::Side::Sente,
        command: t::GameCommand::Resign,
    };
    assert!(
        dispatch_command(
            NativeCommandTarget::Chess(&mut chess),
            NativeCommand::Tori(Box::new(request))
        )
        .is_err()
    );
    assert_eq!(g.context(), before);
    let unknownside = json!({"game_kind":"tori","side":"red"}).to_string();
    assert!(serde_json::from_str::<NativeSide>(&unknownside).is_err());
}

#[test]
fn imported_complete_history_through_real_envelope_preserves_obligation_and_root_provenance() {
    let root = t::Position::from_diagram(
        &[
            (
                "4g".parse().unwrap(),
                t::Piece {
                    side: t::Side::Sente,
                    face: t::Face::Phoenix,
                },
            ),
            (
                "4a".parse().unwrap(),
                t::Piece {
                    side: t::Side::Gote,
                    face: t::Face::Phoenix,
                },
            ),
        ],
        [t::Hand::empty(); 2],
        t::Side::Sente,
        t::RootMetadata::imported(
            "independent imported diagram; previous events unavailable".into(),
            51,
        )
        .unwrap(),
    )
    .unwrap();
    let mut g = t::Game::from_root(root).unwrap();
    let root_record = record(&g);
    let root_key = g.repetition_key().clone();
    for (from, to) in [("4g", "5g"), ("4a", "5a"), ("5g", "4g"), ("5a", "4a")]
        .into_iter()
        .cycle()
        .take(8)
    {
        play(
            &mut g,
            t::Action::Move {
                from: from.parse().unwrap(),
                to: to.parse().unwrap(),
                result_face: t::Face::Phoenix,
            },
        );
    }
    assert_eq!(g.repetition_key(), &root_key);
    assert_eq!(g.occurrences(), [0, 4, 8]);
    assert_eq!(g.obligation().unwrap().side, t::Side::Sente);
    assert_ne!(root_record, record(&g));
    let encoded = record(&g).to_json().unwrap();
    let CanonicalRecord::Tori(native) = CanonicalRecord::from_json(&encoded).unwrap() else {
        panic!()
    };
    let replay = native.to_game().unwrap();
    assert_eq!(replay.events(), g.events());
    assert_eq!(replay.obligation(), g.obligation());
    assert_eq!(replay.position().root(), g.position().root());
    assert_eq!(replay.position().declared_total_plies(), 59);
    assert!(replay.position().root().earlier_history_unknown());
    assert!(!replay.position().root().historical_reachability_proven());
    assert_ne!(replay.context(), g.context());
    let prefix = g.record_prefix(4).unwrap();
    let prefix = prefix.to_game().unwrap();
    assert_eq!(prefix.events().len(), 4);
    assert_eq!(prefix.occurrences(), [0, 4]);
    assert!(prefix.obligation().is_none());
    let view = g.select_prefix(4).unwrap();
    assert_eq!(view.occurrences, [0, 4]);
    assert!(view.obligation.is_none());
    assert_eq!(view.local_position().position().declared_total_plies(), 55);
    assert_eq!(g.occurrences(), [0, 4, 8]);
}
