//! Bounded off-path document file and replay worker.
use super::*;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};
/// Maximum desktop archive bytes; exceeding a resource cap never truncates data.
pub const MAX_ARCHIVE_BYTES: usize = 16 * 1024 * 1024;
/// Maximum retained accepted observations in the current session.
pub const MAX_ARCHIVE_OBSERVATIONS: usize = 256;
const VALIDATION_BUDGET: std::time::Duration = std::time::Duration::from_secs(15);
/// Native preferences persist independently of game/search identity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesktopSettings {
    /// Persisted ownership of the two native participants.
    pub play_mode: crate::PlayMode,
    /// View orientation, independent of player color.
    pub orientation: Color,
    /// Initial analysis form choices; actual requests retain their own limits.
    pub analysis_defaults: AnalysisOperation,
}
impl Default for DesktopSettings {
    fn default() -> Self {
        Self {
            play_mode: crate::PlayMode::default(),
            orientation: Color::White,
            analysis_defaults: AnalysisOperation::Moves {
                depth: 4,
                nodes: 100_000,
            },
        }
    }
}
/// A stable open precondition, unaffected by I/O progress snapshots.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct AdmissionToken {
    /// Expected live command version.
    pub version: u64,
    /// Expected document contents revision.
    pub document_revision: u64,
}
/// File/inspection operation phase, separate from chess outcomes.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IoPhase {
    /// No operation yet.
    #[default]
    Idle,
    /// Atomic file save in progress.
    Saving,
    /// File validation/replay in progress.
    Opening,
    /// Historical prefix reconstruction in progress.
    Viewing,
    /// Current operation completed.
    Complete,
    /// Current operation failed; live game preserved.
    Failed,
}
/// Bounded worker operation identity and diagnostic.
#[derive(Clone, Debug, Default, Serialize)]
pub struct IoState {
    /// Monotonic operation identity.
    pub id: u64,
    /// Current phase.
    pub phase: IoPhase,
    /// Plain completion/failure detail.
    pub message: Option<String>,
    /// Native resource facts when the failed file operation establishes them.
    pub resource: Option<ResourceFailure>,
}
pub(super) enum Work {
    Native(Box<shared::archive::NativeWork>),
}
pub(super) enum Completed {
    Native(Box<shared::archive::NativeCompleted>),
}
struct Job {
    id: u64,
    work: Work,
    stop: Arc<AtomicBool>,
}
#[derive(Default)]
struct WorkerState {
    pending: Option<Job>,
    result: Option<(u64, Result<Completed, OperationFailure>)>,
    current: Option<Arc<AtomicBool>>,
    shutdown: bool,
}
pub(super) struct WorkerShared {
    state: Mutex<WorkerState>,
    wake: Condvar,
    pub(super) exited: AtomicBool,
}
pub(super) struct FileWorker {
    pub(super) shared: Arc<WorkerShared>,
    // Hold only result observation; the real worker still executes normally.
    #[cfg(test)]
    hold_results: AtomicBool,
}
impl FileWorker {
    pub(super) fn new() -> io::Result<Self> {
        let shared = Arc::new(WorkerShared {
            state: Mutex::new(WorkerState::default()),
            wake: Condvar::new(),
            exited: AtomicBool::new(false),
        });
        let worker = shared.clone();
        thread::Builder::new()
            .name("mshogi-document".into())
            .spawn(move || {
                loop {
                    let job = {
                        let mut state = worker.state.lock().expect("document worker metadata");
                        while state.pending.is_none() && !state.shutdown {
                            state = worker.wake.wait(state).expect("document worker metadata");
                        }
                        if state.shutdown {
                            break;
                        }
                        state.pending.take().expect("pending document job")
                    };
                    let began = Instant::now();
                    let deadline = began + VALIDATION_BUDGET;
                    let stopped = || job.stop.load(Ordering::Acquire) || Instant::now() >= deadline;
                    let result = finish_validation_budget(
                        execute(job.work, stopped),
                        began.elapsed(),
                        VALIDATION_BUDGET,
                        job.stop.load(Ordering::Acquire),
                    );
                    let mut state = worker.state.lock().expect("document worker metadata");
                    if !state.shutdown
                        && !job.stop.load(Ordering::Acquire)
                        && state
                            .current
                            .as_ref()
                            .is_some_and(|current| Arc::ptr_eq(current, &job.stop))
                    {
                        state.result = Some((job.id, result));
                    }
                }
                worker.exited.store(true, Ordering::Release);
            })?;
        Ok(Self {
            shared,
            #[cfg(test)]
            hold_results: AtomicBool::new(false),
        })
    }
    pub(super) fn cancel(&self) {
        let mut state = self.shared.state.lock().expect("document worker metadata");
        if let Some(current) = &state.current {
            current.store(true, Ordering::Release);
        }
        state.pending = None;
        state.result = None;
    }
    pub(super) fn submit(&self, id: u64, work: Work) {
        let stop = Arc::new(AtomicBool::new(false));
        let mut state = self.shared.state.lock().expect("document worker metadata");
        if let Some(current) = &state.current {
            current.store(true, Ordering::Release);
        }
        let retired = (
            state.pending.replace(Job {
                id,
                work,
                stop: stop.clone(),
            }),
            state.result.take(),
        );
        state.current = Some(stop);
        drop(state);
        self.shared.wake.notify_one();
        drop(retired);
    }
    pub(super) fn poll(&self) -> Option<(u64, Result<Completed, OperationFailure>)> {
        #[cfg(test)]
        if self.hold_results.load(Ordering::Relaxed) {
            return None;
        }
        self.shared
            .state
            .lock()
            .expect("document worker metadata")
            .result
            .take()
    }
    #[cfg(test)]
    pub(super) fn hold_results_for_test(&self, hold: bool) {
        self.hold_results.store(hold, Ordering::Relaxed);
    }
    pub(super) fn shutdown(&self) {
        let mut state = self.shared.state.lock().expect("document worker metadata");
        state.shutdown = true;
        if let Some(current) = &state.current {
            current.store(true, Ordering::Release);
        }
        let retired = (state.pending.take(), state.result.take());
        drop(state);
        self.shared.wake.notify_one();
        drop(retired);
    }
}
impl Drop for FileWorker {
    fn drop(&mut self) {
        self.shutdown();
    }
}
fn finish_validation_budget(
    result: Result<Completed, OperationFailure>,
    elapsed: std::time::Duration,
    budget: std::time::Duration,
    cancelled: bool,
) -> Result<Completed, OperationFailure> {
    // Atomic rename is the Save commit point. Its successful completion cannot
    // subsequently become a failed Save because scheduling crossed the budget.
    // atomic_write checks the deadline/cancellation predicate before that point.
    let committed_save = match &result {
        Ok(Completed::Native(value)) => matches!(
            value.as_ref(),
            shared::archive::NativeCompleted::Saved { .. }
        ),
        _ => false,
    };
    if !committed_save && !cancelled && elapsed >= budget {
        Err(resource_refusal(
            resource_fact(
                ResourceCategory::ValidationBudget,
                elapsed.as_millis().min(u64::MAX as u128) as usize,
                budget.as_millis() as usize,
                0,
                ResourceScope::Application,
                ResourceUnit::Milliseconds,
            ),
            "Application capacity: native document validation exceeded its bounded15-second budget. The accepted current document is preserved.",
        ))
    } else {
        result
    }
}
/// Deterministic clock injection into the actual worker deadline classifier.
/// Verification payloads only: this does not measure replay performance.
#[cfg(feature = "desktop-verification")]
pub(super) fn verification_deadline_failure() -> OperationFailure {
    finish_validation_budget(
        Err("Verification: interrupted validation before commit.".into()),
        VALIDATION_BUDGET,
        VALIDATION_BUDGET,
        false,
    )
    .err()
    .expect("injected elapsed validation budget")
}
pub(super) fn check(stopped: &mut impl FnMut() -> bool) -> Result<(), String> {
    if stopped() {
        Err("Document operation cancelled or exceeded its 15-second validation budget.".into())
    } else {
        Ok(())
    }
}
fn execute(work: Work, mut stopped: impl FnMut() -> bool) -> Result<Completed, OperationFailure> {
    let Work::Native(work) = work;
    shared::archive::execute(*work, &mut stopped).map(|value| Completed::Native(Box::new(value)))
}
pub(super) fn atomic_write(
    path: &std::path::Path,
    bytes: &[u8],
    stopped: &mut impl FnMut() -> bool,
) -> Result<(), String> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| std::path::Path::new("."));
    let mut created = None;
    for attempt in 0..32 {
        let name = format!(
            ".mshogi-save-{}-{}-{attempt}.tmp",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        );
        let temporary = parent.join(name);
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
        {
            Ok(file) => {
                created = Some((temporary, file));
                break;
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(format!("Save temporary file failed: {error}")),
        }
    }
    let (temporary, mut file) =
        created.ok_or("Cannot create an exclusive same-directory save file.")?;
    let result = (|| {
        for chunk in bytes.chunks(8192) {
            check(stopped)?;
            file.write_all(chunk)
                .map_err(|e| format!("Save write failed: {e}"))?;
        }
        file.flush().map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;
        check(stopped)?;
        drop(file);
        fs::rename(&temporary, path).map_err(|e| format!("Atomic save replacement failed: {e}"))
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}
impl DesktopController {
    /// Capture a current replacement precondition without doing filesystem work.
    pub(super) fn open_token(&self) -> AdmissionToken {
        AdmissionToken {
            version: self.session.version,
            document_revision: self.session.document_revision,
        }
    }
    pub(super) fn poll_files(&mut self) {
        let Some((id, result)) = self.files.poll() else {
            return;
        };
        if id != self.io_state.id {
            return;
        }
        let result = match result {
            Ok(Completed::Native(value)) => self.finish_native_file(*value),
            Err(error) => {
                self.session.clear_browse_intent();
                self.io_state.resource = error.resource;
                Err(error.diagnostic)
            }
        };
        self.io_state.phase = if result.is_ok() {
            IoPhase::Complete
        } else {
            IoPhase::Failed
        };
        self.io_state.message = Some(result.unwrap_or_else(|error| error));
        self.session.revision += 1;
    }
    /// The sole current-document installation authority. Every payload has already
    /// passed strict native replay on the bounded document worker. This method
    /// assigns fresh identities and stops execution before the atomic owner swap.
    pub(crate) fn install_document(
        &mut self,
        opened: shared::archive::OpenedNative,
        path: PathBuf,
    ) -> Result<String, String> {
        use shared::archive::OpenedNative;
        let next = self
            .runtime_floor()
            .max(self.session.revision)
            .checked_add(1)
            .ok_or("Runtime identity exhausted.")?;
        let revision = self.session.document_revision;
        if next >= MAX_SAFE_ID - 64
            || self.session.document_epoch >= MAX_SAFE_ID - 64
            || revision >= MAX_SAFE_ID - 65
            || self.session.document_identity >= MAX_SAFE_ID - 65
        {
            let (count, limit) = if next >= MAX_SAFE_ID - 64 {
                (next, MAX_SAFE_ID - 65)
            } else if self.session.document_epoch >= MAX_SAFE_ID - 64 {
                (self.session.document_epoch, MAX_SAFE_ID - 65)
            } else if revision >= MAX_SAFE_ID - 65 {
                (revision, MAX_SAFE_ID - 66)
            } else {
                (self.session.document_identity, MAX_SAFE_ID - 66)
            };
            return Err(self.identity_refusal(count,limit,true,ResourceMeasurement::Candidate,"Application capacity: reopened runtime identities leave no safe headroom. Current document preserved."));
        }
        let runtime = RuntimeMetadata {
            document_epoch: self.session.document_epoch + 1,
            version: next,
            revision: next,
            next_request: next,
            document_revision: revision + 1,
            document_identity: next,
            saved_revision: revision + 1,
            document_path: Some(path),
            selection_id: next,
            capacity: CapacityState::default(),
        };
        let candidate = match opened {
            OpenedNative::Chess(mut state) => {
                state.prepare_install();
                ActiveDocument::Chess { runtime, state }
            }
            OpenedNative::Shogi(mut state) => {
                state.prepare_install();
                ActiveDocument::Shogi { runtime, state }
            }
            OpenedNative::Tenjiku(mut state) => {
                state.prepare_install();
                ActiveDocument::Tenjiku { runtime, state }
            }
            OpenedNative::Chu(mut state) => {
                state.prepare_install();
                ActiveDocument::Chu { runtime, state }
            }
            OpenedNative::Tori(mut state) => {
                state.prepare_install();
                ActiveDocument::Tori { runtime, state }
            }
            OpenedNative::Xiangqi(mut state) => {
                state.prepare_install();
                ActiveDocument::Xiangqi { runtime, state }
            }
        };
        self.retire_admission();
        self.last_command_receipt = None;
        self.bridge.cancel();
        self.session = candidate;
        Ok(
            "Opened complete native document with fresh runtime identities and work stopped."
                .into(),
        )
    }
    /// Stop the actual native document before host close confirmation. No other
    /// game is constructed and no result is invented.
    pub fn application_pause_for_close(&mut self) -> SharedSnapshot {
        // Repeated close must retain an existing exact stop-only exemption.
        let dirty_before = self.close_requires_save();
        self.pause_application();
        self.lifecycle
            .record_close_guard(self.shared_open_token(), dirty_before);
        self.shared_snapshot()
    }
    fn pause_application(&mut self) {
        self.session.clear_browse_intent();
        self.pause_shared_for_close();
        self.retire_admission();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validation_deadline_is_distinct_from_cancellation_and_preserves_committed_saves() {
        let budget = std::time::Duration::from_secs(15);
        for (elapsed, cancelled, resource) in [
            (14999, false, false),
            (15001, true, false),
            (15001, false, true),
        ] {
            let error = finish_validation_budget(
                Err("Native operation stopped".into()),
                std::time::Duration::from_millis(elapsed),
                budget,
                cancelled,
            )
            .err()
            .unwrap();
            assert_eq!(error.resource.is_some(), resource);
            if let Some(failure) = error.resource {
                let fact = &failure.fact();
                assert_eq!(fact.category, ResourceCategory::ValidationBudget);
                assert_eq!(fact.count, Some(15001));
                assert_eq!(fact.limit, Some(15000));
                assert_eq!(fact.unit, Some(ResourceUnit::Milliseconds));
            }
        }
        let saved = Completed::Native(Box::new(shared::archive::NativeCompleted::Saved {
            path: "committed.mshogi".into(),
            revision: 7,
            identity: 9,
        }));
        assert!(
            finish_validation_budget(
                Ok(saved),
                budget + std::time::Duration::from_millis(1),
                budget,
                false
            )
            .is_ok()
        );
    }
    #[test]
    fn interrupted_atomic_write_preserves_the_prior_file_and_removes_temporary() {
        let dir = std::env::temp_dir().join(format!("msh225-atomic-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("game.mshogi");
        fs::write(&path, b"prior").unwrap();
        let mut checks = 0;
        assert!(
            atomic_write(&path, b"replacement", &mut || {
                checks += 1;
                checks >= 2
            })
            .is_err()
        );
        assert_eq!(fs::read(&path).unwrap(), b"prior");
        assert_eq!(fs::read_dir(&dir).unwrap().count(), 1);
        fs::remove_dir_all(dir).unwrap();
    }
}
