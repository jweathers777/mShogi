//! Frozen two-profile Tenjiku expectations. This loader executes no game rules.
use crate::{Fixture, strict};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
/// Exact reviewed MSH-83 native and downstream expectation bytes.
pub const FIXTURES: &str = include_str!("../../../docs/m4/tenjiku/fixtures.json");
/// Reviewed two-profile corpus hash, not generated from native output.
pub const CORPUS_SHA256: &str = "2005e098b507f73248b8f9bd442f7ffe0c2c0c5ae205428d9a3d2371a7c08e82";
/// Complete immutable profile identities.
pub const PROFILES: [&str; 2] = [
    "mshogi.tenjiku.adams1999-modern.v1",
    "mshogi.tenjiku.muller-completed.v1",
];
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Wire {
    schema: String,
    revision: String,
    profiles: Vec<String>,
    profile_status: String,
    native_conformance_executed: bool,
    source_ids: Vec<String>,
    notation: String,
    fixtures: Vec<Fixture>,
}
/// Admitted source-derived fixture rows, not actual observations.
pub struct Corpus {
    fixtures: Vec<Fixture>,
}
impl Corpus {
    /// Load immutable data without network access or a comparator engine.
    pub fn embedded() -> Result<Self, String> {
        Self::load(FIXTURES)
    }
    /// Strictly admit only the reviewed input, including duplicate-free nested values.
    pub fn load(text: &str) -> Result<Self, String> {
        let w: Wire = strict::parse(text, 2 * 1024 * 1024)?;
        if format!("{:x}", Sha256::digest(text.as_bytes())) != CORPUS_SHA256 {
            return Err("Tenjiku independent corpus bytes changed".into());
        }
        if w.schema != "mshogi.tenjiku.fixtures.v1"
            || w.revision != "MSH83-2026-11-04"
            || w.profiles != PROFILES
            || w.profile_status != "ratified"
            || w.native_conformance_executed
            || w.source_ids != ["A", "M", "S"]
            || w.notation.is_empty()
        {
            return Err("Tenjiku corpus identity/provenance".into());
        }
        let mut ids = BTreeSet::new();
        let mut counts = [0; 3];
        for f in &w.fixtures {
            if !ids.insert(&f.id)
                || f.expected
                    .keys()
                    .map(String::as_str)
                    .collect::<BTreeSet<_>>()
                    != PROFILES.into_iter().collect()
            {
                return Err("Tenjiku fixture identity/applicability".into());
            }
            counts[match f.scope.as_str() {
                "native" => 0,
                "annotation" => 1,
                "consumer" => 2,
                _ => return Err("Tenjiku fixture scope".into()),
            }] += 1;
        }
        if counts != [204, 129, 17] || w.fixtures.len() != 350 {
            return Err("Tenjiku fixture scope counts".into());
        }
        Ok(Self {
            fixtures: w.fixtures,
        })
    }
    /// Exact immutable source expectations; adapters supply independent actual observations.
    pub fn fixtures(&self) -> &[Fixture] {
        &self.fixtures
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frozen_two_profile_admission_rejects_mutations() {
        assert_eq!(Corpus::embedded().unwrap().fixtures().len(), 350);
        for text in [
            FIXTURES.replacen("ratified", "native_passed", 1),
            FIXTURES.replacen("\"schema\":", "\"schema\":0,\"schema\":", 1),
            format!("{FIXTURES}{{}}"),
        ] {
            assert!(Corpus::load(&text).is_err())
        }
    }
}
