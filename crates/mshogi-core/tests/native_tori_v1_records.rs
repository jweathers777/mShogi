//! MSH-208 (Tori Repetition Rule): schema-1 Tori records written by the
//! earlier rule (MSH-129) load by replay under the current rule, unless their
//! history contains a move that now creates a third occurrence.
//! The data are the native records of the desktop QA documents written by the
//! earlier code (desktop/scripts/fixtures/tori-*.mshogi, before MSH-208).
use mshogi_core::native::{CanonicalRecord, NativeSchema};
use mshogi_core::tori as t;
use serde_json::{Value, json};

fn v1(name: &str) -> String {
    std::fs::read_to_string(format!(
        "{}/tests/data/tori-v1/{name}.json",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
}

#[test]
fn schema_one_records_without_a_third_occurrence_load_and_reencode_as_current_schema() {
    for name in [
        "tori-contract",
        "tori-forced-prefix7",
        "tori-mate-in-one",
        "tori-quiet-loss",
    ] {
        let text = v1(name);
        let old: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(old["schema_version"], 1);
        let record = t::GameRecord::from_json(&text).unwrap_or_else(|e| panic!("{name}: {e}"));
        let new: Value = serde_json::from_str(record.to_json()).unwrap();
        assert_eq!(new["schema_version"], t::GAME_RECORD_SCHEMA_VERSION);
        assert_eq!(new["edition"], t::PROFILE_EDITION);
        assert_eq!(
            new["events"].as_array().unwrap().len(),
            old["events"].as_array().unwrap().len()
        );
        assert_eq!(new["current_position"], old["current_position"]);
        assert_eq!(new["conclusion"], old["conclusion"]);
        for (a, b) in new["events"]
            .as_array()
            .unwrap()
            .iter()
            .zip(old["events"].as_array().unwrap())
        {
            for field in [
                "index",
                "board_ply",
                "actor",
                "command",
                "effects",
                "occurrences",
                "conclusion",
            ] {
                assert_eq!(a[field], b[field], "{name} {field}");
            }
            assert!(a.get("obligation").is_none() && a.get("replay_work").is_none());
        }
        // The re-encoded record is an ordinary current record.
        assert_eq!(t::GameRecord::from_json(record.to_json()).unwrap(), record);
        // The same record inside a schema-1 shared envelope.
        let envelope = json!({
            "envelope_version": 1, "game_kind": "tori", "rules_profile": "mshogi.tori.modern.v1",
            "profile_edition": t::PROFILE_EDITION_V1, "native_schema_version": 1,
            "payload": old,
        });
        let CanonicalRecord::Tori(native) =
            CanonicalRecord::from_json(&envelope.to_string()).unwrap()
        else {
            panic!("{name}: not Tori")
        };
        assert_eq!(native.to_json(), record.to_json());
        assert_eq!(
            CanonicalRecord::tori(*native).native_schema(),
            NativeSchema::ToriV3
        );
    }
    // In the forced prefix the Gote Eagle cannot return to 4d: that would make
    // the root key's third occurrence. Gote has other moves; no conclusion.
    let g = t::GameRecord::from_json(&v1("tori-forced-prefix7"))
        .unwrap()
        .to_game()
        .unwrap();
    let r = g.repetition().unwrap();
    assert_eq!(r.side, t::Side::Gote);
    assert_eq!(
        r.forbidden,
        [t::Action::Move {
            from: "5d".parse().unwrap(),
            to: "4d".parse().unwrap(),
            result_face: t::Face::Eagle
        }]
    );
    assert!(g.conclusion().is_none());
    assert!(!g.legal_actions().is_empty());
}

#[test]
fn schema_one_record_with_a_third_occurrence_is_refused_at_that_command() {
    let text = v1("tori-obligation8");
    // Event 7 (the eighth move) returns to the root key a third time.
    assert_eq!(
        t::GameRecord::from_json(&text).unwrap_err(),
        t::RecordError::Command(7)
    );
    let envelope = json!({
        "envelope_version": 1, "game_kind": "tori", "rules_profile": "mshogi.tori.modern.v1",
        "profile_edition": t::PROFILE_EDITION_V1, "native_schema_version": 1,
        "payload": serde_json::from_str::<Value>(&text).unwrap(),
    });
    assert!(CanonicalRecord::from_json(&envelope.to_string()).is_err());
    // Envelope labels must match the payload's own schema and edition, in
    // both directions (review of MSH-208).
    let current = t::GameRecord::from_json(&v1("tori-contract")).unwrap();
    let mixed = [
        (
            serde_json::from_str::<Value>(current.to_json()).unwrap(),
            json!(1),
            json!(t::PROFILE_EDITION_V1),
        ),
        (
            serde_json::from_str::<Value>(&v1("tori-contract")).unwrap(),
            json!(t::GAME_RECORD_SCHEMA_VERSION),
            json!(t::PROFILE_EDITION),
        ),
    ];
    for (payload, schema, edition) in mixed {
        let envelope = json!({
            "envelope_version": 1, "game_kind": "tori", "rules_profile": "mshogi.tori.modern.v1",
            "profile_edition": edition, "native_schema_version": schema, "payload": payload,
        });
        assert!(matches!(
            CanonicalRecord::from_json(&envelope.to_string()),
            Err(mshogi_core::native::EnvelopeError::Identity)
        ));
    }
    // A schema-1 body under the current edition or schema number is refused.
    let mut wrong: Value = serde_json::from_str(&v1("tori-contract")).unwrap();
    wrong["edition"] = json!(t::PROFILE_EDITION);
    assert_eq!(
        t::GameRecord::from_json(&wrong.to_string()).unwrap_err(),
        t::RecordError::Identity
    );
}

#[test]
fn schema_one_replay_stops_per_event() {
    let text = v1("tori-forced-prefix7");
    let events = serde_json::from_str::<Value>(&text).unwrap()["events"]
        .as_array()
        .unwrap()
        .len();
    assert!(events > 0);
    assert!(t::GameRecord::from_json_with_stop(&text, &mut || false).is_ok());
    // Stopped before each event and after the last one.
    for limit in 0..=events {
        let mut calls = 0;
        let mut stop = || {
            calls += 1;
            calls > limit
        };
        assert_eq!(
            t::GameRecord::from_json_with_stop(&text, &mut stop).unwrap_err(),
            t::RecordError::Stopped
        );
    }
}

#[test]
fn legacy_stop_is_validated_and_removed_without_losing_player_history() {
    let mut old: Value = serde_json::from_str(&v1("tori-forced-prefix7")).unwrap();
    let original = t::GameRecord::from_json(&old.to_string()).unwrap();
    let mut interruption = old["events"][0].clone();
    interruption["index"] = json!(1);
    interruption["command"] = json!({"kind":"stop", "reason":"engine_failure"});
    interruption["effects"] = Value::Null;
    interruption["incomplete"] = json!("engine_failure");
    old["events"]
        .as_array_mut()
        .unwrap()
        .insert(1, interruption);
    for (index, event) in old["events"].as_array_mut().unwrap().iter_mut().enumerate() {
        event["index"] = json!(index);
    }
    // The next actual move clears the old annotation, without changing repetition.
    let migrated = t::GameRecord::from_json(&old.to_string()).unwrap();
    assert_eq!(migrated, original);
    let (_, undos) = migrated.to_game_with_undo().unwrap();
    assert_eq!(undos.len(), original.event_count());
    for (field, value) in [
        ("index", json!(2)),
        ("board_ply", json!(0)),
        ("occurrences", json!([0])),
        ("effects", old["events"][0]["effects"].clone()),
        ("incomplete", json!("cancelled")),
    ] {
        let mut forged = old.clone();
        forged["events"][1][field] = value;
        assert!(
            t::GameRecord::from_json(&forged.to_string()).is_err(),
            "{field}"
        );
    }
    let mut last = old["events"].as_array().unwrap().last().unwrap().clone();
    last["index"] = json!(old["events"].as_array().unwrap().len());
    last["command"] = json!({"kind":"stop", "reason":"cancelled"});
    last["effects"] = Value::Null;
    last["incomplete"] = json!("cancelled");
    old["events"].as_array_mut().unwrap().push(last);
    old["incomplete"] = json!("cancelled");
    assert_eq!(
        t::GameRecord::from_json(&old.to_string()).unwrap(),
        original
    );
    old["incomplete"] = Value::Null;
    assert!(t::GameRecord::from_json(&old.to_string()).is_err());
}

#[test]
fn operational_stop_is_not_a_current_native_command() {
    let command = r#"{"kind":"stop","reason":"cancelled"}"#;
    assert!(t::GameCommand::from_json(command).is_err());
    assert!(mshogi_core::chu::GameCommand::from_json(command).is_err());
    assert!(mshogi_core::tenjiku::GameCommand::from_json(command).is_err());
}
