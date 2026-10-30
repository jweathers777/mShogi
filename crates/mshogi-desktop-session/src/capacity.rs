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
    facts: Vec<ResourceFact>,
    diagnostic: impl AsRef<str>,
) -> OperationFailure {
    OperationFailure::resource(ResourceFailure::multiple(facts, diagnostic))
}
pub(crate) fn document_count_failure(
    controls: usize,
    ordinary: usize,
    exact: usize,
    combined: bool,
    scope: ResourceScope,
    reserve: usize,
) -> OperationFailure {
    let mut facts = Vec::new();
    if controls.saturating_add(reserve) > MAX_ARCHIVE_CONTROLS {
        facts.push(resource_fact(
            ResourceCategory::Controls,
            controls,
            MAX_ARCHIVE_CONTROLS,
            reserve,
            scope,
            ResourceUnit::Controls,
        ));
    }
    if ordinary.saturating_add(exact) > MAX_ARCHIVE_OBSERVATIONS {
        facts.push(
            resource_fact(
                if combined {
                    ResourceCategory::CombinedObservations
                } else {
                    ResourceCategory::OrdinaryObservations
                },
                ordinary.saturating_add(exact),
                MAX_ARCHIVE_OBSERVATIONS,
                0,
                scope,
                ResourceUnit::Observations,
            )
            .with_observations(ordinary as u64, exact as u64),
        );
    }
    resource_refusal(
        facts,
        "Application capacity: saved native document counts exceed supported limits; current document preserved.",
    )
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
impl DesktopSession {
    pub(super) fn observation_slot(&self) -> Result<(), OperationFailure> {
        if self.observations.len() >= MAX_ARCHIVE_OBSERVATIONS {
            Err(resource_refusal(
                vec![
                    resource_fact(
                        ResourceCategory::OrdinaryObservations,
                        self.observations.len(),
                        MAX_ARCHIVE_OBSERVATIONS,
                        0,
                        ResourceScope::Chess,
                        ResourceUnit::Observations,
                    )
                    .with_measurement(ResourceMeasurement::Accepted),
                ],
                "Application capacity: all256 retained observation slots are occupied. Current document remains saveable; start a new document to continue work.",
            ))
        } else {
            Ok(())
        }
    }
    pub(super) fn capacity_pause(&mut self, error: String) {
        self.pending = None;
        if self.analysis_pending.take().is_some() {
            self.analysis.status = AnalysisPhase::Cancelled;
            self.analysis.result = None;
            self.analysis.message = Some("Work stopped before observation acceptance because the document has no remaining capacity.".into());
        }
        self.play_paused = true;
        self.engine.status = EnginePhase::Paused;
        self.engine.message = None;
        self.capacity.status = CapacityPhase::Paused;
        self.capacity.message = Some(if error.starts_with("Application capacity:") {
            error
        } else {
            format!(
                "Application capacity: {error} Save the complete current document, then start a new game/document to continue."
            )
        });
        self.revision += 1;
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
    /// Queue a command and return a stable, caller-owned completion receipt.
    /// Continue polling snapshots off the UI thread until completion is present.
    pub fn command_with_receipt(
        &mut self,
        command: DesktopCommand,
    ) -> Result<(Snapshot, AdmissionReceipt), String> {
        self.last_command_receipt = None;
        let snapshot = self.command(command)?;
        let receipt = self.last_command_receipt.take().unwrap_or_else(|| {
            let value = AdmissionReceipt::pending();
            value.finish(Ok(()));
            value
        });
        Ok((snapshot, receipt))
    }

    #[cfg(test)]
    pub(super) fn queue_admission(&mut self, candidate: DesktopSession) -> Result<(), String> {
        self.queue_admission_operation(candidate, None)
    }
    pub(super) fn queue_admission_operation(
        &mut self,
        candidate: DesktopSession,
        take_back_at: Option<u64>,
    ) -> Result<(), String> {
        self.queue_chess_candidate(
            candidate,
            take_back_at.map(|at_ns| persistence::ChessMutation::TakeBack { at_ns }),
        )
    }
    pub(super) fn queue_chess_candidate(
        &mut self,
        mut candidate: DesktopSession,
        mutation: Option<persistence::ChessMutation>,
    ) -> Result<(), String> {
        candidate.document_epoch = self.session.document_epoch;
        if self.session.capacity.id >= MAX_SAFE_ID - 32 {
            return Err(self.identity_refusal(self.session.capacity.id,MAX_SAFE_ID-33,false,ResourceMeasurement::Accepted,"Application capacity: admission identity space exhausted. Current document remains saveable."));
        }
        if let Some(receipt) = self.admission_receipt.take() {
            receipt.finish(Err(
                "The pending command was superseded before acceptance.".into()
            ));
        }
        self.admission_receipt = Some(AdmissionReceipt::pending());
        self.session.capacity.id += 1;
        self.session.capacity.status = CapacityPhase::Checking;
        self.session.capacity.message = None;
        self.session.capacity.resource = None;
        self.session.revision += 1;
        candidate.capacity.id = self.session.capacity.id;
        self.admission_token = Some(self.open_token());
        let preferences = matches!(
            &mutation,
            Some(persistence::ChessMutation::Command {
                command: DesktopCommand::Preferences { .. },
                ..
            })
        );
        self.shared.admission = Some((self.session.document_epoch, preferences));
        self.admissions.submit(
            self.session.capacity.id,
            persistence::Work::Admit {
                candidate: Box::new(candidate),
                mutation,
            },
        );
        Ok(())
    }
    pub(super) fn poll_admission(&mut self) {
        self.poll_native_admission();
    }
    pub(super) fn retire_admission(&mut self) {
        self.shared.admission = None;
        self.shared.admission_consumes_response = false;
        self.shared.rollback = None;
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

#[cfg(feature = "desktop-verification")]
impl DesktopSession {
    pub(super) fn verification_capacity() -> Result<Self, String> {
        let mut session = Self::new();
        // Native zero-node evaluations supply real incomplete observations. This
        // fixture initializer and its name do not exist in normal distributions.
        for index in 0..MAX_ARCHIVE_OBSERVATIONS - 1 {
            session.command_unchecked(
                DesktopCommand::Analyze {
                    version: session.version,
                    operation: AnalysisOperation::Moves { depth: 1, nodes: 0 },
                },
                index as u64,
            )?;
            let request = session
                .analysis_request()
                .expect("fixture analysis request");
            let mut request_bytes = serde_json::to_vec(&request).map_err(|e| e.to_string())?;
            request_bytes.push(b'\n');
            let mut response_bytes = Vec::new();
            mshogi_engine::serve_once(std::io::Cursor::new(request_bytes), &mut response_bytes)
                .map_err(|e| e.to_string())?;
            let response: ExecutionResponse =
                serde_json::from_slice(&response_bytes).map_err(|e| e.to_string())?;
            if !response.matches(&request)
                || !session.accept_analysis_unchecked(response.observation)
            {
                return Err("Verification observation was not accepted.".into());
            }
        }
        session.check_archive_capacity(&mut || false)?;
        session.validated_observations = session.observations.len();
        Ok(session)
    }
}

#[cfg(all(test, feature = "desktop-verification"))]
mod verification_tests {
    use super::*;
    #[test]
    fn chess_capacity_fixture_retains_current_semantic_observations_from_real_executor() {
        let session = DesktopSession::verification_capacity().unwrap();
        assert_eq!(session.observations.len(), MAX_ARCHIVE_OBSERVATIONS - 1);
        assert!(
            session
                .observations
                .iter()
                .all(|entry| entry.observation.matches(&entry.observation.context))
        );
        assert!(session.check_archive_capacity(&mut || false).is_ok());
        assert!(session.analysis_pending.is_none());
        assert!(session.pending.is_none());
    }
}
