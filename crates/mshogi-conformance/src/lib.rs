//! Development-only, engine-independent conformance machinery. No game rules run here.
pub mod assertions;
pub mod properties;
mod strict;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

/// Audited MSH-81 bytes, not generated from implementation output.
pub const CORPUS_SHA256: &str = "3f9e5ca3b541fc72f562ddd8231408e17e7bd5ba3ccee118e643ce0f543731e3";
/// Exact selected profile.
pub const PROFILE: &str = "mshogi.tori.modern.v1";
/// Native fixture specification bytes, unchanged from MSH-81.
pub const FIXTURES: &str = include_str!("../../../docs/m4/tori/fixtures.json");
/// Complete native and later consumer ownership.
pub const OWNERSHIP: &str = include_str!("../../../docs/m4/tori/native-fixture-ownership.json");
/// Bound each input; serde also enforces its normal nesting limit.
pub const MAX_INPUT_BYTES: usize = 512 * 1024;

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotation_of: Option<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireCorpus {
    revision: String,
    profile: String,
    profile_status: String,
    native_conformance_executed: bool,
    fixture_schema: String,
    notation: Value,
    source_ids: Vec<String>,
    preserved_seed_ids: Vec<String>,
    fixtures: Vec<Fixture>,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[allow(missing_docs)]
pub struct CheckOwner {
    pub owner: String,
    pub scope: String,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[allow(missing_docs)]
pub struct Ownership {
    pub fixture_id: String,
    pub corpus_owner: String,
    pub primary_native_owner: String,
    pub native_boundary: String,
    pub expected_fields: Vec<String>,
    pub supplementary_native_checks: Vec<CheckOwner>,
    pub later_consumer_checks: Vec<CheckOwner>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireOwnership {
    schema: String,
    profile: String,
    corpus_revision: String,
    corpus_sha256: String,
    corpus_unchanged: bool,
    coordinator: String,
    corpus_owner: String,
    native_components: Vec<String>,
    policy: String,
    fixtures: Vec<Ownership>,
}
/// Immutable admitted independently expected fixture data and named owners.
#[derive(Clone, Debug)]
pub struct Corpus {
    fixtures: Vec<Fixture>,
    ownership: BTreeMap<String, Ownership>,
}
impl Corpus {
    /// Load the checked-in corpus without external programs or network access.
    pub fn embedded() -> Result<Self, String> {
        Self::load(FIXTURES, OWNERSHIP)
    }
    /// Strictly admit the exact reviewed corpus and ownership, preserving nested duplicates.
    pub fn load(fixtures: &str, ownership: &str) -> Result<Self, String> {
        let wire: WireCorpus = strict::parse(fixtures, MAX_INPUT_BYTES)?;
        let owners: WireOwnership = strict::parse(ownership, MAX_INPUT_BYTES)?;
        let anchor: WireCorpus = strict::parse(FIXTURES, MAX_INPUT_BYTES)?;
        let owner_anchor: WireOwnership = strict::parse(OWNERSHIP, MAX_INPUT_BYTES)?;
        let mut ids = BTreeSet::new();
        for f in &wire.fixtures {
            if !ids.insert(f.id.as_str()) {
                return Err(format!("duplicate fixture ID: {}", f.id));
            }
        }
        let required: BTreeSet<_> = anchor.fixtures.iter().map(|f| f.id.as_str()).collect();
        if ids != required {
            return Err("missing or unknown fixture ID".into());
        }
        if wire.profile != PROFILE
            || wire.fixture_schema != anchor.fixture_schema
            || wire.revision != anchor.revision
            || wire.profile_status != anchor.profile_status
            || wire.native_conformance_executed
            || wire.notation != anchor.notation
            || wire.source_ids != anchor.source_ids
            || wire.preserved_seed_ids != anchor.preserved_seed_ids
        {
            return Err("conflicting corpus identity, provenance or coverage".into());
        }
        if wire.fixtures != anchor.fixtures {
            return Err("fixture definition differs from independent audited expectation".into());
        }
        if format!("{:x}", Sha256::digest(fixtures.as_bytes())) != CORPUS_SHA256 {
            return Err("corpus SHA256 differs from audited bytes".into());
        }
        if owners.schema != owner_anchor.schema
            || owners.profile != PROFILE
            || owners.corpus_revision != wire.revision
            || owners.corpus_sha256 != CORPUS_SHA256
            || !owners.corpus_unchanged
            || owners.coordinator != "MSH-85"
            || owners.corpus_owner != "MSH-126"
            || owners.native_components != owner_anchor.native_components
            || owners.policy != owner_anchor.policy
        {
            return Err("conflicting ownership identity".into());
        }
        let mut mapping = BTreeMap::new();
        for owner in owners.fixtures {
            let id = owner.fixture_id.clone();
            if mapping.insert(id.clone(), owner).is_some() {
                return Err(format!("duplicate ownership ID: {id}"));
            }
        }
        let original: BTreeMap<_, _> = owner_anchor
            .fixtures
            .into_iter()
            .map(|o| (o.fixture_id.clone(), o))
            .collect();
        if mapping != original {
            return Err("missing, unknown or conflicting fixture ownership/fields".into());
        }
        if mapping.len() != 103 || wire.fixtures.len() != 103 {
            return Err("expected exactly103 fixtures".into());
        }
        for f in &wire.fixtures {
            let o = &mapping[&f.id];
            let fields: BTreeSet<_> = f.expected.keys().collect();
            if fields != o.expected_fields.iter().collect() {
                return Err(format!("expected-field ownership mismatch: {}", f.id));
            }
        }
        Ok(Self {
            fixtures: wire.fixtures,
            ownership: mapping,
        })
    }
    /// Read-only audited fixture specifications.
    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }
    /// Full native/supplementary/later-consumer ownership, not a completion claim.
    pub fn owner(&self, id: &str) -> Option<&Ownership> {
        self.ownership.get(id)
    }
}
