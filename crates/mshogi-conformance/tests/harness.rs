//! Deliberately constructed observation tests, not native Tori conformance.
use mshogi_conformance::{
    CORPUS_SHA256, Corpus, FIXTURES, MAX_INPUT_BYTES, OWNERSHIP, assertions::*, properties::*,
};
use serde_json::{Value, json};
use std::{
    fs,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};
fn corpus() -> Corpus {
    Corpus::embedded().unwrap()
}
fn assertion(id: &str, field: &str) -> Assertion {
    plan(&corpus())
        .into_iter()
        .find(|a| a.fixture_id == id && a.field == field)
        .unwrap()
}
fn observe(a: &Assertion, value: Value) -> Observation {
    Observation {
        fixture_id: a.fixture_id.clone(),
        field: a.field.clone(),
        owner: a.owner.clone(),
        value,
    }
}
fn batch(items: Vec<Observation>) -> String {
    serde_json::to_string(&ObservationBatch {
        schema: "mshogi.conformance.observations.v1".into(),
        profile: mshogi_conformance::PROFILE.into(),
        corpus_sha256: CORPUS_SHA256.into(),
        observations: items,
    })
    .unwrap()
}

#[test]
fn all_data_provenance_owners_and_pending_are_accounted() {
    let c = corpus();
    assert_eq!(c.fixtures().len(), 103);
    let r = run(&c, &[]).unwrap();
    assert_eq!(r.fixtures_validated, 103);
    assert_eq!(r.executed, 0);
    assert_eq!(r.passed, 0);
    assert_eq!(r.failed, 0);
    assert!(!r.native_conformance_claimed);
    assert_eq!(r.pending, r.assertions.len());
    assert!(r.pending > 300);
    assert_eq!(r.documentary_annotations, 4);
    assert!(!r.later_consumer_requirements.is_empty());
    for f in c.fixtures() {
        assert!(c.owner(&f.id).is_some());
        assert!(!f.sources.is_empty());
        assert!(!f.expectation_method.is_empty());
    }
    let scopes: std::collections::BTreeSet<_> = r
        .assertions
        .iter()
        .map(|a| format!("{:?}", a.boundary))
        .collect();
    assert_eq!(scopes.len(), 6);
}
#[test]
fn one_constructed_observation_is_not_a_native_fixture_pass() {
    let c = corpus();
    let a = assertion("TORI-CAPTURE-OWNER", "hand_delta");
    let r = run(&c, &[observe(&a, json!({"s:Cr":1}))]).unwrap();
    assert_eq!(r.passed, 1);
    assert_eq!(r.executed, 1);
    assert!(r.pending > 300);
    assert!(!r.native_conformance_claimed);
}
#[test]
fn missing_extra_and_duplicate_legal_actions_are_detected() {
    // Constructed complete-action observation exercises set machinery, not Tori move generation.
    let choices =
        json!([{"kind":"move","from":"4a","to":"5b"},{"kind":"move","from":"4a","to":"5a"}]);
    assert!(
        compare_set(&choices, &json!([{ "kind":"move","from":"4a","to":"5a"}]))
            .unwrap_err()
            .contains("missing=1")
    );
    let mut extra = choices.clone();
    extra
        .as_array_mut()
        .unwrap()
        .push(json!({"kind":"drop","piece":"Sw","to":"4d"}));
    assert!(
        compare_set(&choices, &extra)
            .unwrap_err()
            .contains("extra=1")
    );
    let mut dup = choices.clone();
    dup.as_array_mut().unwrap().push(choices[0].clone());
    assert!(
        compare_set(&choices, &dup)
            .unwrap_err()
            .contains("duplicate")
    );
    let reverse = json!([choices[1], choices[0]]);
    assert!(compare_set(&choices, &reverse).is_ok());
}
#[test]
fn independent_movement_missing_extra_and_wrong_destinations_fail() {
    let a = assertion("TORI-G-SWALLOW", "complete_steps");
    for wrong in [
        json!([]),
        json!([[0, 1], [1, 1]]),
        json!([[0, -1]]),
        json!([[0, 1], [0, 1]]),
    ] {
        assert!(compare(&a, &wrong).is_err());
    }
    assert!(compare(&a, &json!([[0, 1]])).is_ok());
}
#[test]
fn wrong_capture_promotion_demotion_hand_owner_count_history_fail() {
    let cases = [
        ("TORI-CAPTURE-OWNER", "board_removed", json!(["s:Cr:4e"])),
        ("TORI-P-ENTRY", "board_added", json!(["s:Fa:4b"])),
        ("TORI-CAPTURE-DEMOTION", "hand_delta", json!({"s:Ea":1})),
        ("TORI-CAPTURE-GOOSE", "hand_delta", json!({"s:Go":1})),
        ("TORI-CAPTURE-QUAIL", "hand_delta", json!({"s:RQ":1})),
        ("TORI-CAPTURE-OWNER", "hand_delta", json!({"g:Cr":1})),
        ("TORI-CAPTURE-OWNER", "hand_delta", json!({"s:Cr":2})),
        (
            "TORI-H-IDENTITY",
            "each_difference_changes_repetition_key",
            json!(false),
        ),
        ("TORI-H-COUNT", "root_key_occurrences", json!([0, 8, 4])),
    ];
    let c = corpus();
    for (id, field, value) in cases {
        let a = assertion(id, field);
        let r = run(&c, &[observe(&a, value)]).unwrap();
        assert_eq!(r.failed, 1, "{id}");
        assert_eq!(r.executed, 1);
        assert_eq!(r.passed, 0);
        let error = r
            .assertions
            .iter()
            .find(|a| a.status == Status::Failed)
            .unwrap();
        assert_eq!(error.fixture_id, id);
        for key in ["input_context", "expected", "observed", "message"] {
            assert!(error.diagnostic.as_ref().unwrap().get(key).is_some());
        }
    }
}
#[test]
fn actual_reversal_properties_require_snapshots_not_claims() {
    let a = assertion("TORI-CAPTURE-DEMOTION", "reversal_contract/apply_then_undo");
    let before = json!({"board":["g:Ea:4d","s:Cr:4e"],"hands":{},"history":[],"cache":17});
    assert!(compare(&a, &snapshots(&before, &before)).is_ok());
    let mut bad = before.clone();
    bad["cache"] = json!(18);
    assert!(compare(&a, &snapshots(&before, &bad)).is_err());
    assert!(compare(&a, &json!(true)).is_err());
    assert!(compare(&a, &json!({"before":null,"after":null})).is_err());
    let retired = assertion(
        "TORI-CAPTURE-DEMOTION",
        "reversal_contract/retired_execution",
    );
    assert!(
        compare(
            &retired,
            &snapshots(&json!({"generation":3}), &json!({"generation":4}))
        )
        .is_ok()
    );
    assert!(compare(&retired, &snapshots(&before, &before)).is_err());
}
#[test]
fn generated_snapshot_and_material_mutations_are_detected() {
    // Deterministic property-based harness test over2000 generated snapshots, seed fixed.
    let mut seed = 0x126_u64;
    for _ in 0..2000 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let state = json!({"counter":seed,"board":[seed%49],"hands":{"base_falcon":seed%3},"history":[seed%8],"cache":seed>>3});
        assert!(equivalent(&state, &state).is_ok());
        assert!(preserves_material(&state["hands"], &state["hands"]).is_ok());
        let mut bad = state.clone();
        bad["hands"]["base_falcon"] = json!(3);
        assert!(equivalent(&state, &bad).is_err());
        assert!(preserves_material(&state["hands"], &bad["hands"]).is_err());
    }
}
#[test]
fn rotation_is_involution_on_all49_coordinates_and_all103_inputs() {
    for f in 1..=7 {
        for r in b'a'..=b'g' {
            let sq = format!("{f}{}", char::from(r));
            assert_eq!(rotate_square(&rotate_square(&sq).unwrap()).unwrap(), sq);
        }
    }
    for bad in ["0a", "8a", "1h", "1aa", "é", "", "11"] {
        assert!(rotate_square(bad).is_err());
    }
    for f in corpus().fixtures() {
        assert_eq!(rotate_data(&rotate_data(&f.given)), f.given, "{}", f.id);
    }
}
#[test]
fn all35_independently_expected_rotations_correspond_and_detect_mutation() {
    let c = corpus();
    let mut count = 0;
    for f in c.fixtures() {
        if let Some(base) = &f.rotation_of {
            count += 1;
            let original = c.fixtures().iter().find(|r| r.id == *base).unwrap();
            assert_eq!(rotate_data(&original.given), f.given, "{}", f.id);
            let expected = rotate_data(&serde_json::to_value(&original.expected).unwrap());
            for (field, v) in &f.expected {
                assert_eq!(&expected[field], v, "{}:{field}", f.id);
            }
            let mut wrong = f.given.clone();
            wrong["root"]["side_to_move"] = json!("sente");
            assert_ne!(rotate_data(&original.given), wrong);
        }
    }
    assert_eq!(count, 35);
}
#[test]
fn strict_fixture_schema_duplicates_unknown_missing_and_conflicts_reject() {
    let base: Value = serde_json::from_str(FIXTURES).unwrap();
    let mut samples = Vec::new();
    let mut s = base.clone();
    s["fixtures"].as_array_mut().unwrap().pop();
    samples.push(s);
    let mut s = base.clone();
    s["fixtures"][1]["id"] = s["fixtures"][0]["id"].clone();
    samples.push(s);
    let mut s = base.clone();
    s["fixtures"][0]["id"] = json!("TORI-UNKNOWN");
    samples.push(s);
    let mut s = base.clone();
    s["profile"] = json!("mshogi.shogi.csa36.v1");
    samples.push(s);
    let mut s = base.clone();
    s["fixtures"][0]["expected"]["side_to_move"] = json!("gote");
    samples.push(s);
    let mut s = base.clone();
    s["fixtures"][0]["unknown"] = json!(true);
    samples.push(s);
    let mut s = base.clone();
    s["fixtures"][0]["given"]["unknown"] = json!(true);
    samples.push(s);
    let mut s = base.clone();
    s["fixtures"][0].as_object_mut().unwrap().remove("given");
    samples.push(s);
    for s in samples {
        assert!(Corpus::load(&s.to_string(), OWNERSHIP).is_err());
    }
    let duplicate = FIXTURES.replacen(
        "\"revision\":",
        "\"revision\":\"duplicate\",\"revision\":",
        1,
    );
    assert!(
        Corpus::load(&duplicate, OWNERSHIP)
            .unwrap_err()
            .contains("duplicate")
    );
    let nested = FIXTURES.replacen(
        "\"side_to_move\":",
        "\"side_to_move\":\"gote\",\"side_to_move\":",
        1,
    );
    assert!(
        Corpus::load(&nested, OWNERSHIP)
            .unwrap_err()
            .contains("duplicate")
    );
    assert!(
        Corpus::load(&format!("{FIXTURES} "), OWNERSHIP)
            .unwrap_err()
            .contains("SHA256")
    );
    assert!(Corpus::load(&" ".repeat(MAX_INPUT_BYTES + 1), OWNERSHIP).is_err());
    assert!(Corpus::load(&format!("{FIXTURES}{{}}"), OWNERSHIP).is_err());
}
#[test]
fn ownership_missing_duplicate_unknown_wrong_owner_fields_reject() {
    let base: Value = serde_json::from_str(OWNERSHIP).unwrap();
    for mode in 0..5 {
        let mut v = base.clone();
        match mode {
            0 => {
                v["fixtures"].as_array_mut().unwrap().pop();
            }
            1 => v["fixtures"][0]["fixture_id"] = json!("unknown"),
            2 => v["fixtures"][1] = v["fixtures"][0].clone(),
            3 => v["fixtures"][0]["primary_native_owner"] = json!("MSH-42"),
            _ => v["fixtures"][0]["expected_fields"] = json!([]),
        }
        assert!(Corpus::load(FIXTURES, &v.to_string()).is_err());
    }
}
#[test]
fn strict_observations_reject_unknown_wrong_owner_duplicate_nested_and_identity() {
    let c = corpus();
    let a = assertion("TORI-CAPTURE-OWNER", "hand_delta");
    let o = observe(&a, json!({"s:Cr":1}));
    let valid = batch(vec![o.clone()]);
    assert_eq!(observations(&c, &valid).unwrap().len(), 1);
    assert!(observations(&c, &batch(vec![o.clone(), o.clone()])).is_err());
    for (field, value) in [
        ("fixture_id", "unknown"),
        ("field", "unknown"),
        ("owner", "MSH-42"),
    ] {
        let mut v: Value = serde_json::from_str(&valid).unwrap();
        v["observations"][0][field] = json!(value);
        assert!(observations(&c, &v.to_string()).is_err());
    }
    let raw = valid.replace("\"s:Cr\":1", "\"s:Cr\":1,\"s:Cr\":2");
    assert!(observations(&c, &raw).unwrap_err().contains("duplicate"));
    for key in ["schema", "profile", "corpus_sha256"] {
        let mut v: Value = serde_json::from_str(&valid).unwrap();
        v[key] = json!("wrong");
        assert!(observations(&c, &v.to_string()).is_err());
    }
    let mut v: Value = serde_json::from_str(&valid).unwrap();
    v["observations"][0]["unknown"] = json!(true);
    assert!(observations(&c, &v.to_string()).is_err());
}
#[test]
fn adapter_failure_is_failed_and_diagnostics_are_bounded() {
    struct Failed;
    impl NativeAdapter for Failed {
        fn observe(
            &self,
            _: &mshogi_conformance::Fixture,
            _: &Assertion,
        ) -> Result<Option<Value>, String> {
            Err("é".repeat(10000))
        }
    }
    let r = run_adapter(&corpus(), &Failed);
    assert_eq!(r.failed, r.assertions.len());
    assert_eq!(r.pending, 0);
    for a in r.assertions {
        assert!(a.diagnostic.unwrap().to_string().len() < 10000);
    }
}
struct Temp(std::path::PathBuf);
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}
fn temp(text: &str) -> Temp {
    static N: AtomicU64 = AtomicU64::new(0);
    let path = std::env::temp_dir().join(format!(
        "mshogi-conformance-{}-{}.json",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ));
    use std::io::Write;
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .unwrap();
    f.write_all(text.as_bytes()).unwrap();
    Temp(path)
}
#[test]
fn cli_pending_is_honest_and_bad_observation_has_nonzero_diagnostic() {
    let bin = env!("CARGO_BIN_EXE_mshogi-conformance");
    let pending = Command::new(bin).output().unwrap();
    assert!(pending.status.success());
    let r: Value = serde_json::from_slice(&pending.stdout).unwrap();
    assert_eq!(r["passed"], 0);
    assert_eq!(r["fixtures_validated"], 103);
    assert!(
        !Command::new(bin)
            .arg("--require-complete")
            .output()
            .unwrap()
            .status
            .success()
    );
    let a = assertion("TORI-CAPTURE-OWNER", "hand_delta");
    let path = temp(&batch(vec![observe(&a, json!({"s:Cr":2}))]));
    let result = Command::new(bin)
        .args(["--observations", path.0.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(1));
    let r: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(r["failed"], 1);
    let bad = temp("{\"schema\":1,\"schema\":2}");
    let result = Command::new(bin)
        .args(["--observations", bad.0.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2));
    assert!(
        String::from_utf8(result.stderr)
            .unwrap()
            .contains("duplicate")
    );
}

#[test]
fn supplemental_transition_owners_and_owner_exchange_are_real() {
    let c = corpus();
    let a = plan(&c)
        .into_iter()
        .find(|a| {
            a.fixture_id == "TORI-CAPTURE-OWNER" && a.field == "hand_delta" && a.owner == "MSH-127"
        })
        .unwrap();
    assert_eq!(a.boundary, Boundary::Transition);
    let r = run(&c, &[observe(&a, json!({"s:Cr":1}))]).unwrap();
    assert_eq!(r.passed, 1);
    assert!(
        r.assertions
            .iter()
            .any(|a| a.fixture_id == "TORI-CAPTURE-OWNER"
                && a.field == "hand_delta"
                && a.owner == "MSH-128"
                && a.status == Status::Pending)
    );
    assert!(
        plan(&c)
            .iter()
            .any(|a| a.owner == "MSH-127" && a.field == "reversal_contract/apply_then_undo")
    );
    assert_eq!(
        rotate_data(&json!({"owner":"s","opposite_owner":"g"})),
        json!({"owner":"g","opposite_owner":"s"})
    );
    assert_ne!(rotate_data(&json!({"owner":"s"})), json!({"owner":"s"}));
}

#[test]
fn game_commands_and_consumer_fields_never_claim_local_conformance() {
    for a in plan(&corpus()) {
        if a.fixture_id.starts_with("TORI-O-IMPASSE")
            || a.fixture_id.starts_with("TORI-O-AGREEMENT")
            || a.fixture_id.starts_with("TORI-O-RESIGN")
        {
            assert_eq!(a.owner, "MSH-129");
            assert_eq!(a.boundary, Boundary::History);
        }
        if a.field == "conclusion" {
            assert_eq!(a.owner, "MSH-129");
            assert_eq!(a.boundary, Boundary::History);
        }
        if a.field == "live_document_unchanged" {
            assert_eq!(a.owner, "MSH-77");
            assert_eq!(a.boundary, Boundary::Consumer);
        }
    }
    let bin = env!("CARGO_BIN_EXE_mshogi-conformance");
    assert_eq!(
        Command::new(bin)
            .args(["--require-owner", "MSH-127"])
            .output()
            .unwrap()
            .status
            .code(),
        Some(1)
    );
    assert_eq!(
        Command::new(bin)
            .args(["--require-owner", "UNKNOWN"])
            .output()
            .unwrap()
            .status
            .code(),
        Some(2)
    );
}

#[test]
fn native_property_closures_exercise_operations_and_detect_bad_undo_or_rejection() {
    // A vector is an observation-machine example, not a board/rules implementation.
    let mut state = vec![1, 2, 3];
    assert!(
        round_trip(
            &mut state,
            Clone::clone,
            |s| {
                s.push(4);
                Ok::<_, ()>(())
            },
            |s, _| {
                s.pop();
                Ok::<_, ()>(())
            }
        )
        .is_ok()
    );
    assert!(
        round_trip(
            &mut state,
            Clone::clone,
            |s| {
                s.push(4);
                Ok::<_, ()>(())
            },
            |_, _| Ok::<_, ()>(())
        )
        .is_err()
    );
    assert!(rejection(&mut state, Clone::clone, |_| Err::<(), _>("illegal")).is_ok());
    assert!(
        rejection(&mut state, Clone::clone, |s| {
            s.push(9);
            Err::<(), _>("illegal")
        })
        .is_err()
    );
    assert!(rejection(&mut state, Clone::clone, |_| Ok::<_, ()>(())).is_err());
}

#[test]
fn every_assertion_key_is_unique_and_each_field_has_an_executable_or_documentary_role() {
    let c = corpus();
    let p = plan(&c);
    let mut keys = std::collections::BTreeSet::new();
    for a in &p {
        assert!(keys.insert((&a.fixture_id, &a.field, &a.owner)));
    }
    for f in c.fixtures() {
        for field in f.expected.keys() {
            let documentary = matches!(
                field.as_str(),
                "escape_proof"
                    | "defense_proof"
                    | "independent_escape_proof"
                    | "independent_defense_check"
            );
            let executable = p.iter().any(|a| {
                a.fixture_id == f.id
                    && (a.field == *field || a.field.starts_with(&format!("{field}/")))
            });
            assert_ne!(documentary, executable, "{}:{field}", f.id);
        }
    }
}
