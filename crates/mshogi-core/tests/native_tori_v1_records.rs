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
fn schema_one_records_without_a_third_occurrence_load_and_reencode_as_schema_two() {
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
                "incomplete",
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
            NativeSchema::ToriV2
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
