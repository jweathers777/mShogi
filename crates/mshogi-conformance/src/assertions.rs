//! Typed observation admission and honest assertion accounting; no move generation.
use crate::{CORPUS_SHA256, Corpus, Fixture, MAX_INPUT_BYTES, PROFILE, strict};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeSet;

/// An actual result projection; future adapters compute values from native results.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(missing_docs)]
pub struct Observation {
    pub fixture_id: String,
    pub field: String,
    pub owner: String,
    pub value: Value,
}
/// Bounded strict observation file for the CLI or a native test driver.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(missing_docs)]
pub struct ObservationBatch {
    pub schema: String,
    pub profile: String,
    pub corpus_sha256: String,
    pub observations: Vec<Observation>,
}
/// A field's assertion boundary. Geometry is not full legal action admission.
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Boundary {
    Geometry,
    Transition,
    LocalLegality,
    History,
    Record,
    Consumer,
}
/// Comparison required by the independently reviewed input, not by an adapter.
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Predicate {
    Equal,
    CompleteSet,
    EquivalentSnapshots,
    DifferentSnapshots,
}
/// One executable expectation. Documentary escape proofs are annotations instead.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct Assertion {
    pub fixture_id: String,
    pub field: String,
    pub owner: String,
    pub boundary: Boundary,
    pub predicate: Predicate,
    pub expected: Value,
}
/// Honest execution state; absent native observations remain Pending.
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Status {
    Passed,
    Failed,
    Pending,
}
/// Bounded result details for one assertion.
#[derive(Debug, Serialize)]
#[allow(missing_docs)]
pub struct AssertionResult {
    pub predicate: Predicate,
    pub fixture_id: String,
    pub field: String,
    pub owner: String,
    pub boundary: Boundary,
    pub status: Status,
    pub diagnostic: Option<Value>,
}
/// Test infrastructure report, never a declaration of Tori native conformance.
#[derive(Debug, Serialize)]
#[allow(missing_docs)]
pub struct Report {
    pub profile: String,
    pub corpus_sha256: String,
    pub fixtures_validated: usize,
    pub native_conformance_claimed: bool,
    pub executed: usize,
    pub passed: usize,
    pub pending: usize,
    pub failed: usize,
    pub documentary_annotations: usize,
    pub assertions: Vec<AssertionResult>,
    pub later_consumer_requirements: Vec<Value>,
}
/// Native test code can project actual outputs without a process or another rules engine.
/// None means unimplemented/pending; Err is an execution failure, never a chess result.
pub trait NativeAdapter {
    /// Compute this field from an actual native operation. Never echo the expected answer.
    fn observe(&self, fixture: &Fixture, assertion: &Assertion) -> Result<Option<Value>, String>;
}
fn boundary(f: &Fixture) -> Boundary {
    if matches!(
        f.given["command"]["kind"].as_str(),
        Some("resign" | "declare_impasse" | "agree_draw")
    ) {
        return Boundary::History;
    }
    match f.scope.as_str() {
        "exhaustive_local_geometry" | "orientation_geometry" => Boundary::Geometry,
        "initial_position" => Boundary::Transition,
        "root_admission" | "terminal_position" => Boundary::LocalLegality,
        "strict_dispatch_contract" | "notation_contract" => Boundary::Record,
        "execution_contract" | "match_contract" => Boundary::Consumer,
        s if s.starts_with("native_admission") => Boundary::LocalLegality,
        _ => Boundary::History,
    }
}
/// Compile expectations without running rules, using fixed field semantics.
pub fn plan(corpus: &Corpus) -> Vec<Assertion> {
    let mut out = Vec::new();
    for f in corpus.fixtures() {
        for (field, expected) in &f.expected {
            // These are hand-written explanations, not observable native fields.
            if matches!(
                field.as_str(),
                "escape_proof"
                    | "defense_proof"
                    | "independent_escape_proof"
                    | "independent_defense_check"
            ) {
                continue;
            }
            let mut b = boundary(f);
            if field == "conclusion" {
                b = Boundary::History;
            }
            if field == "runtime_work_never_revived" {
                b = Boundary::Consumer;
            }
            if field == "live_document_unchanged" {
                b = Boundary::Consumer;
            }
            let owner = if matches!(
                field.as_str(),
                "state_events_and_runtime_unchanged" | "conclusion"
            ) {
                b = Boundary::History;
                "MSH-129"
            } else if b == Boundary::Consumer {
                "MSH-77"
            } else if f.id == "TORI-C-NOTATION" && field == "display_is_not_action_authority" {
                "MSH-42"
            } else {
                &corpus
                    .owner(&f.id)
                    .expect("admitted ownership")
                    .primary_native_owner
            };
            if field == "reversal_contract" {
                for (part, predicate) in [
                    ("apply_then_undo", Predicate::EquivalentSnapshots),
                    ("redo", Predicate::EquivalentSnapshots),
                    ("retired_execution", Predicate::DifferentSnapshots),
                ] {
                    out.push(Assertion {
                        fixture_id: f.id.clone(),
                        field: format!("{field}/{part}"),
                        owner: "MSH-129".into(),
                        boundary: Boundary::History,
                        predicate,
                        expected: expected[part].clone(),
                    });
                }
            } else {
                let predicate = if field.starts_with("complete_") && expected.is_array()
                    || matches!(
                        field.as_str(),
                        "board"
                            | "board_added"
                            | "board_removed"
                            | "board_unchanged"
                            | "intermediate_unchanged"
                            | "forbidden_departures"
                            | "forbidden_at10"
                            | "forbidden_at16"
                    ) {
                    Predicate::CompleteSet
                } else {
                    Predicate::Equal
                };
                out.push(Assertion {
                    fixture_id: f.id.clone(),
                    field: field.clone(),
                    owner: owner.into(),
                    boundary: if owner == "MSH-42" {
                        Boundary::Consumer
                    } else {
                        b
                    },
                    predicate,
                    expected: expected.clone(),
                });
            }
        }
        if f.id == "TORI-C-IDENTITY" {
            out.push(Assertion {fixture_id:f.id.clone(),field:"native_context/retired".into(),owner:"MSH-129".into(),boundary:Boundary::History,predicate:Predicate::DifferentSnapshots,expected:Value::String("fresh native full-history context/lineage after retirement; real process acceptance belongs77".into())});
        }
        let owner = corpus.owner(&f.id).expect("admitted owner");
        // A structural projection is not full legality; the manifest assigns it to127.
        let structural = owner
            .supplementary_native_checks
            .iter()
            .any(|c| c.owner == "MSH-127");
        if structural {
            for field in [
                "board_added",
                "board_removed",
                "board_unchanged",
                "intermediate_unchanged",
                "hand_delta",
                "side_to_move",
                "promotion_timer",
                "no_extra_promotion_privilege",
                "resulting_goose_does_not_attack_straight_forward_one",
            ] {
                if let Some(expected) = f.expected.get(field) {
                    let predicate = if expected.is_array() {
                        Predicate::CompleteSet
                    } else {
                        Predicate::Equal
                    };
                    out.push(Assertion {
                        fixture_id: f.id.clone(),
                        field: field.into(),
                        owner: "MSH-127".into(),
                        boundary: Boundary::Transition,
                        predicate,
                        expected: expected.clone(),
                    });
                }
            }
        }
        if f.expected.contains_key("reversal_contract") {
            for (native_owner, boundary) in [
                ("MSH-127", Boundary::Transition),
                ("MSH-128", Boundary::LocalLegality),
            ] {
                if native_owner == "MSH-127" && !structural
                    || f.given["command"]["kind"] == "resign"
                {
                    continue;
                }
                for part in ["apply_then_undo", "redo"] {
                    out.push(Assertion {
                        fixture_id: f.id.clone(),
                        field: format!("reversal_contract/{part}"),
                        owner: native_owner.into(),
                        boundary,
                        predicate: Predicate::EquivalentSnapshots,
                        expected: f.expected["reversal_contract"][part].clone(),
                    });
                }
            }
        }
        if f.expected
            .contains_key("state_events_and_runtime_unchanged")
            && matches!(f.given["command"]["kind"].as_str(), Some("move" | "drop"))
        {
            out.push(Assertion {
                fixture_id: f.id.clone(),
                field: "rejection/local_state".into(),
                owner: "MSH-128".into(),
                boundary: Boundary::LocalLegality,
                predicate: Predicate::EquivalentSnapshots,
                expected: Value::String("unchanged authoritative local pre/post snapshots".into()),
            });
        }
    }
    out
}
/// Complete-set comparison rejects duplicates, missing and extra actions/destinations.
pub fn compare_set(expected: &Value, observed: &Value) -> Result<(), String> {
    let to_set = |v: &Value| -> Result<BTreeSet<String>, String> {
        let a = v
            .as_array()
            .ok_or("expected an array for complete-set comparison")?;
        let set: BTreeSet<_> = a.iter().map(Value::to_string).collect();
        if set.len() != a.len() {
            return Err("duplicate member in complete action/geometry set".into());
        }
        Ok(set)
    };
    let e = to_set(expected)?;
    let o = to_set(observed)?;
    if e == o {
        return Ok(());
    }
    Err(format!(
        "missing={} extra={}",
        e.difference(&o).count(),
        o.difference(&e).count()
    ))
}
/// Check one actual result. Properties take real snapshots, not pass/fail booleans.
pub fn compare(assertion: &Assertion, observed: &Value) -> Result<(), String> {
    match assertion.predicate {
        Predicate::Equal => {
            if assertion.expected == *observed {
                Ok(())
            } else {
                Err("expected/observed mismatch".into())
            }
        }
        Predicate::CompleteSet => compare_set(&assertion.expected, observed),
        Predicate::EquivalentSnapshots | Predicate::DifferentSnapshots => {
            let o = observed
                .as_object()
                .ok_or("property requires before/after snapshots")?;
            if o.len() != 2
                || !o.contains_key("before")
                || !o.contains_key("after")
                || o["before"].is_null()
                || o["after"].is_null()
            {
                return Err(
                    "property requires exactly two non-null snapshots: before/after".into(),
                );
            }
            let equal = o["before"] == o["after"];
            if equal == matches!(assertion.predicate, Predicate::EquivalentSnapshots) {
                Ok(())
            } else {
                Err("snapshot property mismatch".into())
            }
        }
    }
}
/// Parse before any map could erase duplicate observations or nested fields.
pub fn observations(corpus: &Corpus, input: &str) -> Result<Vec<Observation>, String> {
    let batch: ObservationBatch = strict::parse(input, MAX_INPUT_BYTES)?;
    if batch.schema != "mshogi.conformance.observations.v1"
        || batch.profile != PROFILE
        || batch.corpus_sha256 != CORPUS_SHA256
    {
        return Err("conflicting observation identity".into());
    }
    validate_observations(corpus, &batch.observations)?;
    Ok(batch.observations)
}
fn validate_observations(corpus: &Corpus, items: &[Observation]) -> Result<(), String> {
    let plan = plan(corpus);
    let known: BTreeSet<_> = plan
        .iter()
        .map(|a| (a.fixture_id.as_str(), a.field.as_str(), a.owner.as_str()))
        .collect();
    let mut seen = BTreeSet::new();
    for o in items {
        let key = (o.fixture_id.as_str(), o.field.as_str(), o.owner.as_str());
        if !seen.insert(key) {
            return Err("duplicate observation assertion".into());
        }
        if !known.contains(&key) {
            return Err("unknown or conflicting observation fixture/field/owner".into());
        }
    }
    Ok(())
}
struct Observations<'a>(&'a [Observation]);
impl NativeAdapter for Observations<'_> {
    fn observe(&self, _f: &Fixture, a: &Assertion) -> Result<Option<Value>, String> {
        Ok(self
            .0
            .iter()
            .find(|o| o.fixture_id == a.fixture_id && o.field == a.field && o.owner == a.owner)
            .map(|o| o.value.clone()))
    }
}
/// Run admitted projections. Missing fields remain pending, never filled from expected data.
pub fn run(corpus: &Corpus, items: &[Observation]) -> Result<Report, String> {
    validate_observations(corpus, items)?;
    Ok(run_adapter(corpus, &Observations(items)))
}
fn clip(v: &Value) -> String {
    v.to_string().chars().take(768).collect()
}
/// Run a native test adapter; records every assertion even if an earlier one fails.
pub fn run_adapter(corpus: &Corpus, adapter: &impl NativeAdapter) -> Report {
    let mut report = Report {
        profile: PROFILE.into(),
        corpus_sha256: CORPUS_SHA256.into(),
        fixtures_validated: corpus.fixtures().len(),
        native_conformance_claimed: false,
        executed: 0,
        passed: 0,
        pending: 0,
        failed: 0,
        documentary_annotations: 0,
        assertions: Vec::new(),
        later_consumer_requirements: Vec::new(),
    };
    for f in corpus.fixtures() {
        report.documentary_annotations += f
            .expected
            .keys()
            .filter(|k| {
                matches!(
                    k.as_str(),
                    "escape_proof"
                        | "defense_proof"
                        | "independent_escape_proof"
                        | "independent_defense_check"
                )
            })
            .count();
        for requirement in &corpus
            .owner(&f.id)
            .expect("admitted owner")
            .later_consumer_checks
        {
            report.later_consumer_requirements.push(json!({"fixture_id":f.id,"owner":requirement.owner,"scope":requirement.scope,"status":"pending","reason":"real consumer adapter required; native projection does not establish this"}));
        }
    }
    for a in plan(corpus) {
        let f = corpus
            .fixtures()
            .iter()
            .find(|f| f.id == a.fixture_id)
            .expect("planned fixture");
        let observation = adapter.observe(f, &a);
        let (status, diagnostic) = match observation {
            Ok(None) => {
                report.pending += 1;
                (Status::Pending, None)
            }
            Ok(Some(value)) => match compare(&a, &value) {
                Ok(()) => {
                    report.passed += 1;
                    (Status::Passed, None)
                }
                Err(e) => {
                    report.failed += 1;
                    (
                        Status::Failed,
                        Some(
                            json!({"message":e,"input_context":clip(&f.given),"expected":clip(&a.expected),"observed":clip(&value)}),
                        ),
                    )
                }
            },
            Err(e) => {
                report.failed += 1;
                (
                    Status::Failed,
                    Some(
                        json!({"message":e.chars().take(768).collect::<String>(),"input_context":clip(&f.given),"expected":clip(&a.expected),"observed":"adapter execution failed"}),
                    ),
                )
            }
        };
        report.assertions.push(AssertionResult {
            predicate: a.predicate,
            fixture_id: a.fixture_id,
            field: a.field,
            owner: a.owner,
            boundary: a.boundary,
            status,
            diagnostic,
        });
    }
    report.executed = report.passed + report.failed;
    report
}
