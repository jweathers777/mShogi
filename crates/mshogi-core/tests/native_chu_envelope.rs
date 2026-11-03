//! Actual Chu payload routing; rule expectations remain in the independent native corpus.
use mshogi_core::{chu as c, native::*};
use serde_json::{Value, json};
fn game() -> c::Game {
    let entries = [
        ("1l", "K", c::Side::Sente),
        ("12a", "K", c::Side::Gote),
        ("6f", "Ln", c::Side::Sente),
        ("10g", "Ln", c::Side::Gote),
    ]
    .map(|(at, code, side)| {
        (
            at.parse().unwrap(),
            c::Piece::from_code(side, code).unwrap(),
        )
    });
    c::Game::from_root(
        c::Position::from_diagram(
            &entries,
            c::Side::Sente,
            c::RootMetadata::imported("shared-routing-fixture".into(), 23).unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}
#[test]
fn real_chu_identity_history_commands_and_duplicate_preserving_routing() {
    let mut g = game();
    for (actor, command) in [
        (
            c::Side::Sente,
            c::GameCommand::Play {
                action: c::Action {
                    from: "6f".parse().unwrap(),
                    route: vec!["6e".parse().unwrap(), "6f".parse().unwrap()],
                    promote: false,
                },
            },
        ),
        (
            c::Side::Gote,
            c::GameCommand::Play {
                action: c::Action {
                    from: "10g".parse().unwrap(),
                    route: vec!["10h".parse().unwrap(), "10g".parse().unwrap()],
                    promote: false,
                },
            },
        ),
        (c::Side::Gote, c::GameCommand::OfferDraw),
    ] {
        let expected = g.context();
        assert!(matches!(
            dispatch_command(
                NativeCommandTarget::Chu(&mut g),
                NativeCommand::Chu(Box::new(c::GameRequest {
                    expected,
                    actor,
                    command
                }))
            )
            .unwrap(),
            NativeReply::Chu(Ok(_))
        ));
    }
    let r = CanonicalRecord::chu(c::GameRecord::from_game(&g).unwrap());
    assert_eq!(r.game_kind(), GameKind::Chu);
    assert_eq!(r.profile(), RulesProfile::ChuJcsa2019V1);
    assert_eq!(r.native_schema(), NativeSchema::ChuV1);
    assert_eq!(r.event_count(), 3);
    assert_eq!(r.known_board_actions(), 2);
    let raw = r.to_json().unwrap();
    assert_eq!(CanonicalRecord::from_json(&raw).unwrap(), r);
    assert_eq!(serde_json::from_str::<CanonicalRecord>(&raw).unwrap(), r);
    let mut v: Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v["profile_edition"], c::PROFILE_EDITION);
    let CanonicalRecord::Chu(restored) = CanonicalRecord::from_json(&raw).unwrap() else {
        panic!()
    };
    let restored = restored.to_game().unwrap();
    assert!(g.context().same_semantics(&restored.context()));
    assert_ne!(g.context(), restored.context());
    assert_eq!(
        restored.position().root().source(),
        Some("shared-routing-fixture")
    );
    assert_eq!(restored.snapshot().draw_offer, Some(c::Side::Gote));
    for field in [
        "game_kind",
        "rules_profile",
        "profile_edition",
        "native_schema_version",
        "envelope_version",
    ] {
        let old = v[field].clone();
        v[field] = json!("unsupported");
        assert!(CanonicalRecord::from_json(&v.to_string()).is_err());
        v[field] = old;
    }
    let duplicate = raw.replacen(
        "\"promote\":false",
        "\"promote\":false,\"promote\":false",
        1,
    );
    assert_ne!(duplicate, raw);
    assert!(CanonicalRecord::from_json(&duplicate).is_err());
    assert!(serde_json::from_str::<NativeSide>(r#"{"game_kind":"chu","side":"white"}"#).is_err());
    let before = c::GameRecord::from_game(&g).unwrap();
    let mut other = mshogi_core::tori::Game::standard();
    let wrong = NativeCommand::Tori(Box::new(mshogi_core::tori::GameRequest {
        expected: other.context(),
        actor: mshogi_core::tori::Side::Sente,
        command: mshogi_core::tori::GameCommand::Resign,
    }));
    assert!(dispatch_command(NativeCommandTarget::Chu(&mut g), wrong).is_err());
    assert_eq!(c::GameRecord::from_game(&g).unwrap(), before);
    other.retire_context().unwrap();
}
