//! Second, independently anchored corpus boundary. Rules remain in `mshogi-core`.
//! The Tori corpus and its 512 KiB limit are unchanged.
use crate::assertions::{self, Assertion, AssertionResult, Boundary, Predicate, Status};
use crate::strict;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

/// Exact MSH-82 ratified profile identity.
pub const PROFILE: &str = "mshogi.chu.jcsa2019.v1";
/// Independently reviewed immutable corpus digest from MSH-82's audit lock.
pub const CORPUS_SHA256: &str = "545f23e7c805949e7d67c64c3e2fd39927d5bdae2401065d4d5f5bfc0c9fea2a";
/// Independently expected input; never implementation observations.
pub const FIXTURES: &str = include_str!("../../../docs/m4/chu/fixtures.json");
/// Development-only input bound: measured corpus is 563,575 bytes, exceeding Tori's bound.
/// This does not change any native record, transport or archive limit.
pub const MAX_INPUT_BYTES: usize = 1024 * 1024;

/// One source-derived fixture, including explicitly pending consumer scope.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[allow(missing_docs)]
pub struct Fixture {
    pub id: String,
    pub clauses: Vec<String>,
    pub sources: Vec<String>,
    pub scope: String,
    pub given: Value,
    pub expected: BTreeMap<String, Value>,
    pub expectation_method: String,
    pub requires_owner_decisions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation_of: Option<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireCorpus {
    schema: String,
    revision: String,
    profile: String,
    profile_status: String,
    native_conformance_executed: bool,
    source_ids: Vec<String>,
    notation: Value,
    fixtures: Vec<Fixture>,
}
/// Strictly admitted immutable Chu expectations.
pub struct Corpus {
    fixtures: Vec<Fixture>,
}
impl Corpus {
    /// Admit checked-in bytes without network access or external engines.
    pub fn embedded() -> Result<Self, String> {
        Self::load(FIXTURES)
    }
    /// Duplicate-preserving parsing precedes maps and audited-byte comparison.
    pub fn load(input: &str) -> Result<Self, String> {
        let wire: WireCorpus = strict::parse(input, MAX_INPUT_BYTES)?;
        if wire.schema != "mshogi.chu.fixtures.v1"
            || wire.revision != "MSH82-2026-11-02"
            || wire.profile != PROFILE
            || wire.profile_status != "ratified"
            || wire.native_conformance_executed
            || wire.source_ids.len() != 8
            || !wire.notation.is_object()
        {
            return Err("conflicting Chu corpus identity or provenance".into());
        }
        let mut ids = BTreeSet::new();
        for fixture in &wire.fixtures {
            if !ids.insert(&fixture.id) {
                return Err(format!("duplicate fixture ID: {}", fixture.id));
            }
            if !fixture.requires_owner_decisions.is_empty()
                || !matches!(fixture.scope.as_str(), "native" | "consumer")
            {
                return Err(format!(
                    "unresolved or unknown fixture scope: {}",
                    fixture.id
                ));
            }
        }
        if wire.fixtures.len() != 250
            || wire.fixtures.iter().filter(|f| f.scope == "native").count() != 240
        {
            return Err("expected 240 native and 10 consumer Chu fixtures".into());
        }
        let digest = format!("{:x}", Sha256::digest(input.as_bytes()));
        if digest != CORPUS_SHA256 {
            return Err(format!(
                "Chu corpus differs from independently audited bytes: expected {CORPUS_SHA256}, observed {digest}, bytes {}",
                input.len()
            ));
        }
        Ok(Self {
            fixtures: wire.fixtures,
        })
    }
    /// Immutable fixtures, including later consumers rather than invented native passes.
    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }
    /// Resolve a deliberate fixture reference without duplicating input definitions.
    pub fn fixture(&self, id: &str) -> Option<&Fixture> {
        self.fixtures.iter().find(|f| f.id == id)
    }
}

/// Explanations/alternative-rule comparisons are retained but not reported as execution.
pub fn documentary(field: &str) -> bool {
    matches!(
        field,
        "offset_basis"
            | "derivation"
            | "count_derivation"
            | "broader_bare_king_alternative_would_win"
    )
}
fn explanatory_lion_reason(field: &str, expected: &Value) -> bool {
    field == "reason"
        && matches!(
            expected.as_str(),
            Some(
                "go_between_only_defender_removed_midroute_is_not_exception"
                    | "intermediate_step_does_not_create_adjacent_lion_exception"
            )
        )
}
fn predicate(field: &str) -> Predicate {
    match field {
        "apply_then_undo" | "redo" | "roundtrip" => Predicate::EquivalentSnapshots,
        "runtime_authority" => Predicate::DifferentSnapshots,
        "board"
        | "step_offsets"
        | "jump_offsets"
        | "ray_directions"
        | "actor_complete_actions"
        | "complete_actions"
        | "divide_depth2"
        | "protected_lions_after_first"
        | "transient_protection_after_first"
        | "remaining_sente_royals"
        | "final_royals"
        | "responsible_sides"
        | "ignored_for_count_only" => Predicate::CompleteSet,
        _ => Predicate::Equal,
    }
}
fn boundary(f: &Fixture) -> Boundary {
    if f.scope == "consumer" {
        return Boundary::Consumer;
    }
    if f.clauses.iter().any(|c| c == "C10") {
        return Boundary::Record;
    }
    if f.clauses
        .iter()
        .any(|c| matches!(c.as_str(), "C06" | "C07" | "C08"))
    {
        return Boundary::History;
    }
    if f.id.starts_with("CHU-MOVE-") {
        Boundary::Geometry
    } else {
        Boundary::LocalLegality
    }
}
/// Assertions reuse the common comparator, including complete-set duplicate rejection.
pub fn plan(corpus: &Corpus) -> Vec<Assertion> {
    corpus
        .fixtures
        .iter()
        .flat_map(|f| {
            f.expected
                .iter()
                .filter(|(field, _)| !documentary(field))
                .map(|(field, expected)| Assertion {
                    fixture_id: f.id.clone(),
                    // These source explanations do not promise a verbose native
                    // diagnostic. Execute its authoritative error class separately.
                    field: if explanatory_lion_reason(field, expected) {
                        "native_refusal_kind".into()
                    } else {
                        field.clone()
                    },
                    owner: if f.scope == "consumer" {
                        "MSH-78/MSH-43"
                    } else {
                        "MSH-86"
                    }
                    .into(),
                    boundary: boundary(f),
                    predicate: predicate(field),
                    expected: if explanatory_lion_reason(field, expected) {
                        json!("lion_law")
                    } else {
                        expected.clone()
                    },
                })
        })
        .collect()
}
/// Actual production projections only; absence remains pending.
pub trait NativeAdapter {
    /// Compute from inputs and native operations. Never copy expected values.
    fn observe(&self, fixture: &Fixture, assertion: &Assertion) -> Result<Option<Value>, String>;
}
/// Counts distinguish rows, assertions, annotations and later consumer obligations.
#[derive(Serialize)]
#[allow(missing_docs)]
pub struct Report {
    pub profile: String,
    pub corpus_sha256: String,
    pub fixtures_validated: usize,
    pub native_fixture_rows: usize,
    pub native_rows_executed: usize,
    pub native_rows_passed: usize,
    pub native_rows_failed: usize,
    pub native_rows_pending: usize,
    pub executed: usize,
    pub passed: usize,
    pub failed: usize,
    pub pending: usize,
    pub documentary_annotations: usize,
    pub assertions: Vec<AssertionResult>,
    pub later_consumer_requirements: Vec<Value>,
}
/// Run all assertions; one failure does not hide later mismatches.
pub fn run_adapter(corpus: &Corpus, adapter: &impl NativeAdapter) -> Report {
    let mut report = Report {
        profile: PROFILE.into(),
        corpus_sha256: CORPUS_SHA256.into(),
        fixtures_validated: corpus.fixtures.len(),
        native_fixture_rows: 240,
        native_rows_executed: 0,
        native_rows_passed: 0,
        native_rows_failed: 0,
        native_rows_pending: 0,
        executed: 0,
        passed: 0,
        failed: 0,
        pending: 0,
        documentary_annotations: corpus
            .fixtures
            .iter()
            .map(|f| {
                f.expected
                    .iter()
                    .filter(|(k, v)| documentary(k) || explanatory_lion_reason(k, v))
                    .count()
            })
            .sum(),
        assertions: Vec::new(),
        later_consumer_requirements: Vec::new(),
    };
    for f in corpus.fixtures.iter().filter(|f| f.scope == "consumer") {
        report.later_consumer_requirements.push(json!({"fixture_id":f.id,"owner":"MSH-78/MSH-43","status":"pending","requirements":f.expected}));
    }
    for a in plan(corpus) {
        let f = corpus.fixture(&a.fixture_id).expect("admitted fixture");
        if f.scope == "consumer" {
            continue;
        }
        let (status, diagnostic) = match adapter.observe(f, &a) {
            Ok(None) => {
                report.pending += 1;
                (Status::Pending, None)
            }
            Ok(Some(v)) => match assertions::compare(&a, &v) {
                Ok(()) => {
                    report.passed += 1;
                    (Status::Passed, None)
                }
                Err(e) => {
                    report.failed += 1;
                    (
                        Status::Failed,
                        Some(
                            json!({"message":e,"input":clip(&f.given),"expected":clip(&a.expected),"observed":clip(&v)}),
                        ),
                    )
                }
            },
            Err(e) => {
                report.failed += 1;
                (
                    Status::Failed,
                    Some(
                        json!({"message":e.chars().take(768).collect::<String>(),"input":clip(&f.given),"expected":clip(&a.expected)}),
                    ),
                )
            }
        };
        report.assertions.push(AssertionResult {
            fixture_id: a.fixture_id,
            field: a.field,
            owner: a.owner,
            boundary: a.boundary,
            predicate: a.predicate,
            status,
            diagnostic,
        });
    }
    for f in corpus.fixtures.iter().filter(|f| f.scope == "native") {
        let rows: Vec<_> = report
            .assertions
            .iter()
            .filter(|a| a.fixture_id == f.id)
            .collect();
        if rows.iter().any(|a| a.status == Status::Failed) {
            report.native_rows_failed += 1;
            report.native_rows_executed += 1;
        } else if rows.is_empty() || rows.iter().any(|a| a.status == Status::Pending) {
            report.native_rows_pending += 1;
        } else {
            report.native_rows_passed += 1;
            report.native_rows_executed += 1;
        }
    }
    report.executed = report.passed + report.failed;
    report
}
fn clip(v: &Value) -> String {
    v.to_string().chars().take(768).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Pending;
    impl NativeAdapter for Pending {
        fn observe(&self, _: &Fixture, _: &Assertion) -> Result<Option<Value>, String> {
            Ok(None)
        }
    }
    #[test]
    fn anchored_second_corpus_and_honest_pending() {
        let corpus = Corpus::embedded().unwrap();
        let report = run_adapter(&corpus, &Pending);
        assert_eq!(report.fixtures_validated, 250);
        assert_eq!(report.native_rows_pending, 240);
        assert_eq!(report.native_rows_executed, 0);
        assert_eq!(report.later_consumer_requirements.len(), 10);
        assert_eq!(report.passed, 0);
        assert_eq!(crate::MAX_INPUT_BYTES, 512 * 1024);
        assert_eq!(crate::Corpus::embedded().unwrap().fixtures().len(), 103);
    }
    #[test]
    fn reject_duplicate_outer_nested_unknown_missing_and_rewritten_expectations() {
        let mut missing: Value = serde_json::from_str(FIXTURES).unwrap();
        missing.as_object_mut().unwrap().remove("schema");
        assert!(Corpus::load(&missing.to_string()).is_err());
        for changed in [
            FIXTURES.replacen("\"schema\":", "\"schema\":0,\"schema\":", 1),
            FIXTURES.replacen(
                "\"promote\": false",
                "\"promote\":true,\"promote\": false",
                1,
            ),
            FIXTURES.replacen("\"schema\":", "\"unknown\":0,\"schema\":", 1),
            FIXTURES.replacen("\"ratified\"", "\"proposed\"", 1),
            FIXTURES.replacen("\"piece_count\": 92", "\"piece_count\": 91", 1),
        ] {
            assert!(Corpus::load(&changed).is_err());
        }
        assert!(Corpus::load(&" ".repeat(MAX_INPUT_BYTES + 1)).is_err());
    }
    #[test]
    fn wrong_actual_value_has_bounded_actionable_diagnostic() {
        struct Wrong;
        impl NativeAdapter for Wrong {
            fn observe(&self, f: &Fixture, a: &Assertion) -> Result<Option<Value>, String> {
                Ok((f.id == "CHU-SETUP" && a.field == "piece_count").then(|| json!(91)))
            }
        }
        let report = run_adapter(&Corpus::embedded().unwrap(), &Wrong);
        assert_eq!(report.failed, 1);
        assert_eq!(report.native_rows_failed, 1);
        let failure = report
            .assertions
            .iter()
            .find(|a| a.status == Status::Failed)
            .unwrap();
        assert_eq!(failure.fixture_id, "CHU-SETUP");
        assert_eq!(failure.diagnostic.as_ref().unwrap()["observed"], "91");
        assert_eq!(failure.diagnostic.as_ref().unwrap()["expected"], "92");
    }
    #[test]
    fn shared_complete_set_reports_missing_extra_and_duplicates() {
        for observed in [json!([1]), json!([1, 2, 3]), json!([1, 2, 2])] {
            assert!(assertions::compare_set(&json!([1, 2]), &observed).is_err());
        }
        assert!(assertions::compare_set(&json!([1, 2]), &json!([2, 1])).is_ok());
    }
    #[test]
    fn byte_identity_does_not_silently_normalize_changed_line_endings() {
        let crlf = FIXTURES.replace('\n', "\r\n");
        let error = Corpus::load(&crlf)
            .err()
            .expect("changed bytes must reject");
        assert!(error.contains("expected "));
        assert!(error.contains("observed "));
        assert!(error.contains("bytes "));
    }
}
