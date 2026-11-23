//! Authoritative resource facts, separate from bounded technical diagnostics.
//!
//! A diagnostic is never parsed to discover a resource category. Native admission
//! supplies facts only when it knows them; unknown causes remain
//! representable without inventing a count or a game outcome.

use serde::Serialize;

/// Maximum UTF-8 byte length of a technical resource diagnostic.
pub const MAX_RESOURCE_DIAGNOSTIC_BYTES: usize = 4096;

/// Resource measured by a native admission or file-validation check.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceCategory {
    /// Actual encoded desktop document size.
    EncodedDocumentBytes,
    /// Actual encoded authoritative native record size.
    CanonicalRecordBytes,
    /// Native events, including events that do not advance board ply.
    NativeEvents,
    /// Native replay or validation work units.
    ReplayWork,
    /// Monotonic execution, request, session or document identity space.
    IdentityHeadroom,
    /// Bounded validation time or work allowance.
    ValidationBudget,
    /// The failing resource cannot be established authoritatively.
    Unknown,
}

/// Native ownership of a resource fact, when its check establishes a scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceScope {
    /// A common application or document resource.
    Application,
    /// A Chess-owned native resource.
    Chess,
    /// A Shogi-owned native resource.
    Shogi,
    /// A Xiangqi-owned native resource.
    Xiangqi,
    /// A Tori-owned native/document resource.
    Tori,
    /// A Chu-owned native/document resource.
    Chu,
    /// A Tenjiku-owned native/document resource, qualified by the document profile.
    Tenjiku,
}

/// Unit of a count or allowance; no conversion is implied between units.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceUnit {
    /// Encoded UTF-8 bytes.
    Bytes,
    /// Native event entries.
    Events,
    /// Native implementation-defined replay work units.
    ReplayUnits,
    /// Monotonic identity increments.
    Identities,
    /// Elapsed or reserved milliseconds.
    Milliseconds,
}

/// Which state the measured count describes. A rejected candidate's usage must
/// never be presented as evidence already retained in the accepted document.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceMeasurement {
    /// The current complete accepted document, before starting more work.
    Accepted,
    /// The proposed complete candidate or file being validated for installation.
    Candidate,
    /// No reliable measurement provenance is available.
    Unknown,
}

/// One native-measured resource fact. Missing values mean unknown, never zero.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ResourceFact {
    /// Authoritative category, independent of the diagnostic wording.
    pub category: ResourceCategory,
    /// Which state was measured; never infer this from the diagnostic text.
    pub measurement: ResourceMeasurement,
    /// Measured usage in that state, excluding the separately recorded reserve.
    pub count: Option<u64>,
    /// Additional admission allowance for requested work or mandatory stop/close.
    pub reserved: Option<u64>,
    /// Established supported limit in the same units.
    pub limit: Option<u64>,
    /// Native scope, when known.
    pub scope: Option<ResourceScope>,
    /// Explicit units, when known.
    pub unit: Option<ResourceUnit>,
}

impl ResourceFact {
    /// Construct a measured count and limit without guessing scope or reserves.
    pub fn measured(category: ResourceCategory, count: u64, limit: u64) -> Self {
        Self {
            category,
            measurement: ResourceMeasurement::Candidate,
            count: Some(count),
            reserved: None,
            limit: Some(limit),
            scope: None,
            unit: None,
        }
    }

    /// Construct an unknown cause without a fabricated observation count.
    pub fn unknown() -> Self {
        Self {
            category: ResourceCategory::Unknown,
            measurement: ResourceMeasurement::Unknown,
            count: None,
            reserved: None,
            limit: None,
            scope: None,
            unit: None,
        }
    }

    /// Attach the separately measured mandatory-operation reservation.
    pub fn with_reserved(mut self, reserved: u64) -> Self {
        self.reserved = Some(reserved);
        self
    }

    /// Attach the native owner established by the check.
    pub fn with_scope(mut self, scope: ResourceScope) -> Self {
        self.scope = Some(scope);
        self
    }

    /// Attach explicit measurement units.
    pub fn with_unit(mut self, unit: ResourceUnit) -> Self {
        self.unit = Some(unit);
        self
    }

    /// Identify an accepted-state check rather than a proposed candidate check.
    pub fn with_measurement(mut self, measurement: ResourceMeasurement) -> Self {
        self.measurement = measurement;
        self
    }
}

/// Bounded native capacity refusal; this is neither an illegal move nor outcome.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ResourceFailure {
    fact: ResourceFact,
    diagnostic: String,
}

impl ResourceFailure {
    /// Construct a refusal with one authoritative resource fact.
    pub fn new(fact: ResourceFact, diagnostic: impl AsRef<str>) -> Self {
        let diagnostic = diagnostic.as_ref();
        let mut end = diagnostic.len().min(MAX_RESOURCE_DIAGNOSTIC_BYTES);
        while !diagnostic.is_char_boundary(end) {
            end -= 1;
        }
        Self {
            fact,
            diagnostic: diagnostic[..end].to_owned(),
        }
    }

    /// Construct a truthful fallback when the failing resource is unknown.
    pub fn unknown(diagnostic: impl AsRef<str>) -> Self {
        Self::new(ResourceFact::unknown(), diagnostic)
    }

    /// The authoritative cause; diagnostic text adds no further facts.
    pub fn fact(&self) -> &ResourceFact {
        &self.fact
    }

    /// Bounded technical details, unsuitable for resource-category inference.
    pub fn diagnostic(&self) -> &str {
        &self.diagnostic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejected_candidate_count_is_distinct_from_accepted_count() {
        let candidate = ResourceFact::measured(ResourceCategory::NativeEvents, 257, 256);
        let accepted = ResourceFact::measured(ResourceCategory::NativeEvents, 256, 256)
            .with_measurement(ResourceMeasurement::Accepted);
        assert_eq!(candidate.measurement, ResourceMeasurement::Candidate);
        assert_eq!(accepted.measurement, ResourceMeasurement::Accepted);
        assert_ne!(
            serde_json::to_value(candidate).unwrap(),
            serde_json::to_value(accepted).unwrap()
        );
        assert_eq!(
            ResourceFact::unknown().measurement,
            ResourceMeasurement::Unknown
        );
    }

    #[test]
    fn unknown_cause_does_not_invent_a_specific_resource() {
        let unknown = ResourceFailure::unknown("8192 controls; 256 observations");
        assert_eq!(unknown.fact(), &ResourceFact::unknown());
    }

    #[test]
    fn diagnostic_is_bounded_without_broken_utf8() {
        let text = format!("{}界suffix", "a".repeat(MAX_RESOURCE_DIAGNOSTIC_BYTES - 1));
        let fact = ResourceFact::measured(ResourceCategory::NativeEvents, 11, 10);
        let failure = ResourceFailure::new(fact.clone(), text);
        assert_eq!(
            failure.diagnostic().len(),
            MAX_RESOURCE_DIAGNOSTIC_BYTES - 1
        );
        assert_eq!(failure.fact(), &fact);
        let wire = serde_json::to_value(&failure).unwrap();
        assert_eq!(wire["fact"], serde_json::to_value(fact).unwrap());
        assert_eq!(wire["diagnostic"], failure.diagnostic());
        assert_eq!(wire.as_object().unwrap().len(), 2);
    }
}
