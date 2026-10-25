//! Transactional document admission. Encoding/replay happens on a bounded worker.
use super::*;
use std::sync::{Arc, Mutex};

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
}
/// A bounded, caller-owned receipt survives later automatic result admissions.
#[derive(Clone, Debug)]
pub struct AdmissionReceipt(Arc<Mutex<Option<Result<(), String>>>>);
impl AdmissionReceipt {
    /// None while pending; otherwise the exact command's accepted/rejected result.
    pub fn completion(&self) -> Option<Result<(), String>> {
        self.0.lock().expect("admission receipt").clone()
    }
    fn pending() -> Self {
        Self(Arc::new(Mutex::new(None)))
    }
    fn finish(&self, result: Result<(), String>) {
        let mut value = self.0.lock().expect("admission receipt");
        if value.is_none() {
            *value = Some(result);
        }
    }
}
impl DesktopSession {
    pub(super) fn observation_slot(&self) -> Result<(), String> {
        if self.observations.len() >= MAX_ARCHIVE_OBSERVATIONS {
            Err("Application capacity: all 256 observation slots are retained. Save this complete document, then start a new game/document to continue analysis or engine play.".into())
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

    pub(super) fn queue_admission(&mut self, mut candidate: DesktopSession) -> Result<(), String> {
        if self.session.capacity.id >= MAX_SAFE_ID - 32 {
            return Err("Application capacity: admission identity space exhausted. Save the current document and restart.".into());
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
        self.session.revision += 1;
        candidate.capacity.id = self.session.capacity.id;
        self.admission_token = Some(self.open_token());
        self.admissions
            .submit_admission(self.session.capacity.id, candidate);
        Ok(())
    }
    pub(super) fn poll_admission(&mut self) {
        let Some((id, result)) = self.admissions.poll_admission() else {
            return;
        };
        if id != self.session.capacity.id {
            return;
        }
        let Some(token) = self.admission_token.take() else {
            return;
        };
        if token != self.open_token() {
            if let Some(receipt) = self.admission_receipt.take() {
                receipt.finish(Err(
                    "The live document changed before command acceptance.".into()
                ));
            }
            self.session.capacity.status = CapacityPhase::Available;
            return;
        }
        match result {
            Ok(mut candidate) => {
                if let Some(receipt) = self.admission_receipt.take() {
                    receipt.finish(Ok(()));
                }
                if candidate.document_identity == self.session.document_identity {
                    candidate.document_path = self.session.document_path.clone();
                    candidate.saved_revision = self.session.saved_revision;
                }
                candidate.validated_observations = candidate.observations.len();
                candidate.revision = candidate.revision.max(self.session.revision + 1);
                candidate.capacity = CapacityState {
                    id,
                    ..CapacityState::default()
                };
                self.session = candidate;
            }
            Err(error) => {
                self.bridge.cancel();
                self.session.capacity_pause(error);
                if let Some(receipt) = self.admission_receipt.take() {
                    receipt.finish(Err(self
                        .session
                        .capacity
                        .message
                        .clone()
                        .expect("capacity diagnostic")));
                }
            }
        }
    }
    pub(super) fn retire_admission(&mut self) {
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
            if !session.accept_analysis_unchecked(
                serde_json::from_slice(&response_bytes).map_err(|e| e.to_string())?,
            ) {
                return Err("Verification observation was not accepted.".into());
            }
        }
        session.check_archive_capacity(&mut || false)?;
        session.validated_observations = session.observations.len();
        Ok(session)
    }
}
