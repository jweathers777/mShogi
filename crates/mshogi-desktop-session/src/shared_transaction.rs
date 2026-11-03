//! Transaction plumbing for the two immutable native document consumers.
//! Game-specific admission stays in their adapters; this owns no live document.
use super::*;
impl DesktopController {
    pub(super) fn queue_native_transaction(
        &mut self,
        work: archive::NativeWork,
        consumes_response: bool,
    ) -> Result<AdmissionReceipt, String> {
        self.ensure_runtime_capacity()?;
        if self.admission_token.is_some() {
            return Err("Wait for native document admission.".into());
        }
        if self.session.capacity.id >= MAX_SAFE_ID - 64 {
            return Err(self.identity_refusal(self.session.capacity.id, MAX_SAFE_ID - 65, false, ResourceMeasurement::Accepted,
                "Application admission limit reached. The complete accepted document remains saveable."));
        }
        self.session.capacity.id += 1;
        self.session.capacity.status = CapacityPhase::Checking;
        self.session.capacity.message = None;
        self.session.capacity.resource = None;
        self.session.revision += 1;
        let receipt = AdmissionReceipt::pending();
        self.shared.transactional_admission = true;
        self.shared.admission_consumes_response = consumes_response;
        self.admission_receipt = Some(receipt.clone());
        self.admission_token = Some(self.open_token());
        self.shared.admission = Some((self.session.document_epoch, false));
        self.admissions.submit(
            self.session.capacity.id,
            persistence::Work::Native(Box::new(work)),
        );
        self.last_command_receipt = Some(receipt.clone());
        Ok(receipt)
    }
    pub(super) fn finish_native_transaction(
        &mut self,
        opened: archive::OpenedNative,
        replacement: bool,
        durable: bool,
        id: u64,
        receipt: Option<AdmissionReceipt>,
    ) {
        let result = if replacement {
            self.install_document(opened, PathBuf::new()).map(|_| {
                self.session.document_path = None;
                self.session.saved_revision = 0;
            })
        } else {
            self.bridge.cancel();
            match (&mut self.session, opened) {
                (ActiveDocument::Tori { state, .. }, archive::OpenedNative::Tori(next)) => {
                    *state = next
                }
                (ActiveDocument::Chu { state, .. }, archive::OpenedNative::Chu(next)) => {
                    *state = next
                }
                _ => unreachable!("serialized native transaction owner"),
            }
            self.session.version += 1;
            self.session.selection_id += 1;
            self.session.revision += 1;
            if durable {
                self.session.document_revision += 1;
            }
            self.session.archive_revision = self.session.revision;
            Ok(())
        };
        self.session.capacity = CapacityState {
            id,
            ..CapacityState::default()
        };
        if let Some(receipt) = receipt {
            receipt.finish(result);
        }
    }
}
