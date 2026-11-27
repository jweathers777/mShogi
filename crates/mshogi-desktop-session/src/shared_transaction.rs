//! Transaction plumbing for immutable document consumers.
//! Game-specific admission stays in their adapters; this owns no live document.
use super::*;
impl DesktopController {
    pub(super) fn queue_transaction(
        &mut self,
        work: archive::GameWork,
        consumes_response: bool,
    ) -> Result<AdmissionReceipt, String> {
        self.ensure_runtime_capacity()?;
        if self.admission_token.is_some() {
            return Err("Wait for document admission.".into());
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
            persistence::Work::Archive(Box::new(work)),
        );
        self.last_command_receipt = Some(receipt.clone());
        Ok(receipt)
    }
    pub(super) fn finish_transaction(
        &mut self,
        opened: archive::OpenedGame,
        replacement: bool,
        durable: bool,
        consumes_response: bool,
        id: u64,
        receipt: Option<AdmissionReceipt>,
    ) {
        let result = if replacement {
            self.install_document(opened, PathBuf::new()).map(|_| {
                self.session.document_path = None;
                self.session.saved_revision = 0;
            })
        } else {
            // A preference change keeps current work; every other change retires it.
            let keeps_work = opened.has_pending_work();
            if !keeps_work {
                self.bridge.cancel();
            }
            match (&mut self.session, opened) {
                (ActiveDocument::Chess { state, .. }, archive::OpenedGame::Chess(next)) => {
                    *state = next;
                }
                (ActiveDocument::Shogi { state, .. }, archive::OpenedGame::Shogi(next)) => {
                    *state = next
                }
                (ActiveDocument::Xiangqi { state, .. }, archive::OpenedGame::Xiangqi(next)) => {
                    *state = next
                }
                (ActiveDocument::Tori { state, .. }, archive::OpenedGame::Tori(next)) => {
                    *state = next
                }
                (ActiveDocument::Chu { state, .. }, archive::OpenedGame::Chu(next)) => {
                    *state = next
                }
                (ActiveDocument::Tenjiku { state, .. }, archive::OpenedGame::Tenjiku(next)) => {
                    *state = next
                }
                _ => unreachable!("serialized transaction owner"),
            }
            // A read-only result changes revision, not its selected target.
            if !keeps_work && !(consumes_response && !durable) {
                self.session.version += 1;
                self.session.selection_id += 1;
            }
            self.session.revision += 1;
            if durable {
                self.session.document_revision += 1;
            }
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
