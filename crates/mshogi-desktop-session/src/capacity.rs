//! Transactional document admission. Encoding/replay happens on a bounded worker.
use super::*;
use std::sync::{Arc, Mutex};

/// Internal worker refusal keeps semantic/native errors distinct from resource
/// facts. Public legacy error text is only a bounded diagnostic projection.
#[derive(Clone, Debug)]
pub(crate) struct OperationFailure {
    pub diagnostic: String,
    pub resource: Option<ResourceFailure>,
}
impl OperationFailure {
    pub(crate) fn resource(failure: ResourceFailure) -> Self {
        Self {
            diagnostic: failure.diagnostic().into(),
            resource: Some(failure),
        }
    }
}
impl From<String> for OperationFailure {
    fn from(value: String) -> Self {
        Self {
            diagnostic: crate::document::diagnostic(value),
            resource: None,
        }
    }
}
impl From<&str> for OperationFailure {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}
impl From<OperationFailure> for String {
    fn from(value: OperationFailure) -> Self {
        value.diagnostic
    }
}
impl std::fmt::Display for OperationFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.diagnostic.fmt(f)
    }
}
impl std::ops::Deref for OperationFailure {
    type Target = str;
    fn deref(&self) -> &str {
        &self.diagnostic
    }
}
pub(crate) fn resource_fact(
    category: ResourceCategory,
    count: usize,
    limit: usize,
    reserved: usize,
    scope: ResourceScope,
    unit: ResourceUnit,
) -> ResourceFact {
    ResourceFact::measured(category, count as u64, limit as u64)
        .with_reserved(reserved as u64)
        .with_scope(scope)
        .with_unit(unit)
}
pub(crate) fn resource_refusal(
    fact: ResourceFact,
    diagnostic: impl AsRef<str>,
) -> OperationFailure {
    OperationFailure::resource(ResourceFailure::new(fact, diagnostic))
}
/// Resource admission is independent of engine execution and chess outcomes.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CapacityPhase {
    /// The last candidate was accepted; ordinary work may proceed.
    #[default]
    Available,
    /// A complete immutable candidate is being checked off the interaction path.
    Checking,
    /// A candidate was rejected; the complete previously accepted document remains.
    Paused,
}
/// Monotonic native admission identity and resource diagnostic.
#[derive(Clone, Debug, Default, Serialize)]
pub struct CapacityState {
    /// Identifies this admission, including its eventual completion/rejection.
    pub id: u64,
    /// Current native admission phase.
    pub status: CapacityPhase,
    /// Actionable resource diagnostic, separate from chess and engine failures.
    pub message: Option<String>,
    /// Authoritative native category/count/limit; absent for non-resource errors.
    pub resource: Option<ResourceFailure>,
}
/// A bounded, caller-owned receipt survives later automatic result admissions.
#[derive(Clone, Debug)]
pub struct AdmissionReceipt(Arc<Mutex<Option<Result<(), String>>>>);
impl AdmissionReceipt {
    /// None while pending; otherwise the exact command's accepted/rejected result.
    pub fn completion(&self) -> Option<Result<(), String>> {
        self.0.lock().expect("admission receipt").clone()
    }
    pub(super) fn pending() -> Self {
        Self(Arc::new(Mutex::new(None)))
    }
    pub(super) fn finish(&self, result: Result<(), String>) {
        let mut value = self.0.lock().expect("admission receipt");
        if value.is_none() {
            *value = Some(result);
        }
    }
}
impl DesktopController {
    pub(crate) fn ensure_runtime_capacity(&mut self) -> Result<(), OperationFailure> {
        if let Err(error) = self.session.ensure_capacity() {
            self.pause_native_failure(error.clone());
            Err(error)
        } else {
            Ok(())
        }
    }
    pub(crate) fn identity_refusal(
        &mut self,
        count: u64,
        limit: u64,
        file: bool,
        measurement: ResourceMeasurement,
        diagnostic: &str,
    ) -> String {
        let resource = ResourceFailure::new(
            ResourceFact::measured(ResourceCategory::IdentityHeadroom, count, limit)
                .with_measurement(measurement)
                .with_scope(ResourceScope::Application)
                .with_unit(ResourceUnit::Identities),
            diagnostic,
        );
        if file {
            self.io_state.resource = Some(resource);
            self.io_state.message = Some(diagnostic.into());
            self.io_state.phase = IoPhase::Failed;
        } else {
            self.session.capacity.resource = Some(resource);
            self.session.capacity.message = Some(diagnostic.into());
            self.session.capacity.status = CapacityPhase::Paused;
        }
        diagnostic.into()
    }
    pub(super) fn retire_admission(&mut self) {
        self.shared.admission = None;
        self.shared.admission_consumes_response = false;
        self.shared.transactional_admission = false;
        if let Some(receipt) = self.admission_receipt.take() {
            receipt.finish(Err(
                "The pending command was superseded before acceptance.".into()
            ));
        }
        self.admissions.cancel();
        self.admission_token = None;
        self.session.capacity.id += 1;
        if self.session.capacity.status == CapacityPhase::Checking {
            self.session.capacity.status = CapacityPhase::Available;
            self.session.capacity.message = None;
            self.session.capacity.resource = None;
        }
        self.session.revision += 1;
    }
}
