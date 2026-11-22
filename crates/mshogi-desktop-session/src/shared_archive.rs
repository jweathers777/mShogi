//! Versioned lossless archives for the concrete shared desktop consumers.
use super::*;
use crate::persistence::{self, Completed, Work};
use std::{fs, io::Read};

/// Native shogi preferences; presentation does not prescribe artwork or layout.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShogiSettings {
    /// Durable local participant arrangement.
    #[serde(default)]
    pub play_mode: PlayMode,
    /// Native side placed at the bottom by a presentation consumer.
    pub orientation: s::Side,
    /// Explicit default bounded ordinary analysis settings.
    pub analysis_defaults: WorkLimits,
}
impl Default for ShogiSettings {
    fn default() -> Self {
        Self {
            play_mode: PlayMode::HumanEngine,
            orientation: s::Side::Sente,
            analysis_defaults: WorkLimits {
                time_ms: None,
                depth: 1,
                nodes: 1000,
            },
        }
    }
}
impl crate::native_game::DocumentSettings for ShogiSettings {
    type Side = s::Side;
    fn new(orientation: s::Side, play_mode: PlayMode) -> Self {
        Self {
            play_mode,
            orientation,
            ..Self::default()
        }
    }
    fn play_mode(&self) -> PlayMode {
        self.play_mode
    }
    fn set_play_mode(&mut self, play_mode: PlayMode) {
        self.play_mode = play_mode;
    }
    fn validate(&self) -> Result<(), String> {
        valid_limits(self.analysis_defaults)
    }
    fn with_analysis_defaults(&self, limits: WorkLimits) -> Option<Self> {
        Some(Self {
            analysis_defaults: limits,
            ..self.clone()
        })
    }
}

/// Game-qualified settings without a common board or rules representation.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "game_kind", rename_all = "snake_case")]
pub enum NativeDesktopSettings {
    /// Native Tenjiku orientation; the immutable profile stays in the record.
    Tenjiku {
        /// Persisted native orientation.
        settings: crate::tenjiku_document::TenjikuSettings,
    },
    /// Native Chu document orientation; no execution defaults implied.
    Chu {
        /// Persisted native orientation.
        settings: crate::chu_document::ChuSettings,
    },
    /// Native Tori settings; no search capabilities implied.
    Tori {
        /// Persisted native orientation without engine defaults.
        settings: crate::tori_document::ToriSettings,
    },
    /// Native Xiangqi document preferences.
    Xiangqi {
        /// Explicit native board orientation.
        orientation: mshogi_core::xiangqi::Side,
        /// Supported ordinary sound-fast-only analysis defaults.
        analysis_defaults: WorkLimits,
    },
    /// Unchanged native chess settings, including explicit Full defaults.
    Chess {
        /// Existing persisted chess preferences.
        settings: DesktopSettings,
    },
    /// Native shogi settings; chess Full is not an available operation.
    Shogi {
        /// Supported untimed shogi preferences.
        settings: ShogiSettings,
    },
}
/// Complete shared document metadata; execution ownership is never a saved setting.
#[derive(Clone, Debug, Serialize)]
pub struct SharedDocumentState {
    /// Monotonic accepted document-content revision.
    pub revision: u64,
    /// True until the exact accepted revision is successfully saved.
    pub dirty: bool,
    /// Native path of the current document, if any.
    pub path: Option<String>,
    /// Native game-qualified desktop preferences.
    pub settings: NativeDesktopSettings,
}
/// Stable replacement precondition, unaffected by I/O progress snapshots.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedOpenToken {
    /// Active document epoch; prevents cross-game ABA replacement.
    pub document: u64,
    /// Current live session version, independent of board ply.
    pub version: u64,
    /// Exact accepted content revision approved for replacement.
    pub document_revision: u64,
}
pub(crate) fn required_option<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    Option::<T>::deserialize(d)
}
/// A generic game's runtime or save job on the worker.
pub(crate) type GameJob =
    Box<dyn FnOnce(&mut dyn FnMut() -> bool) -> Result<NativeCompleted, OperationFailure> + Send>;
pub(crate) enum NativeWork {
    /// Runtime admission or save of a game on the generic session.
    Game(GameJob),
    Xiangqi(xiangqi::archive::XiangqiWork),
    Open {
        path: PathBuf,
        token: SharedOpenToken,
    },
}
pub(crate) enum OpenedNative {
    Tenjiku(Box<tenjiku::TenjikuSession>),
    Chu(Box<chu::ChuSession>),
    Tori(Box<tori::ToriSession>),
    Xiangqi {
        state: Box<xiangqi::XiangqiSession>,
        floor: u64,
    },
    Chess(Box<persistence::Opened>),
    Shogi(Box<shogi::ShogiSession>),
}
impl OpenedNative {
    /// A generic game's session still holds pending work.
    pub(crate) fn has_pending_work(&self) -> bool {
        match self {
            Self::Shogi(state) => state.pending.is_some(),
            Self::Tori(state) => state.pending.is_some(),
            Self::Chu(state) => state.pending.is_some(),
            Self::Tenjiku(state) => state.pending.is_some(),
            Self::Xiangqi { .. } | Self::Chess(_) => false,
        }
    }
}
pub(crate) enum NativeCompleted {
    /// A generic game's session after runtime admission.
    Runtime {
        state: OpenedNative,
        replacement: bool,
        durable: bool,
    },
    Xiangqi(xiangqi::archive::XiangqiCompleted),
    Saved {
        path: PathBuf,
        revision: u64,
        identity: u64,
    },
    Opened {
        game: OpenedNative,
        path: PathBuf,
        token: SharedOpenToken,
    },
}
pub(super) fn bounded(error: impl ToString) -> String {
    let mut message = error.to_string();
    let mut end = message.len().min(mshogi_engine::MAX_STDERR_BYTES);
    while !message.is_char_boundary(end) {
        end -= 1;
    }
    message.truncate(end);
    message
}

fn valid_limits(limits: WorkLimits) -> Result<(), String> {
    // The same bounds as Analyze (native_session::validate_limits).
    if !(1..=mshogi_engine::SHOGI_MAX_DEPTH).contains(&limits.depth)
        || !(1..=mshogi_engine::SHOGI_MAX_NODES).contains(&limits.nodes)
        || limits.time_ms.is_some()
    {
        Err(
            "Unsupported Shogi limits: depth 1–64 and 1 to 5,000,000 nodes; untimed play only."
                .into(),
        )
    } else {
        Ok(())
    }
}

pub(crate) fn decode_chess_document(
    bytes: &[u8],
    stopped: &mut impl FnMut() -> bool,
) -> Result<persistence::Opened, OperationFailure> {
    let value = crate::document::DocumentEnvelope::parse(bytes)?;
    if value.game_kind != GameKind::Chess {
        return Err("Wrong game: the Chess consumer cannot install another native game.".into());
    }
    persistence::decode_chess_payload(value.payload.get().as_bytes(), stopped)
}
fn read(
    path: &std::path::Path,
    stopped: &mut impl FnMut() -> bool,
) -> Result<Vec<u8>, OperationFailure> {
    if !fs::metadata(path)
        .map_err(|e| format!("Open metadata failed: {e}"))?
        .is_file()
    {
        return Err("Only regular desktop archive files may be opened.".into());
    }
    let mut file = fs::File::open(path).map_err(|e| format!("Open failed: {e}"))?;
    if !file.metadata().map_err(bounded)?.is_file() {
        return Err("Only regular desktop archive files may be opened.".into());
    }
    let mut bytes = Vec::new();
    let mut buffer = [0; 8192];
    loop {
        persistence::check(stopped)?;
        let n = file.read(&mut buffer).map_err(bounded)?;
        if n == 0 {
            break;
        }
        if bytes.len() + n > MAX_ARCHIVE_BYTES {
            return Err(resource_refusal(
                vec![resource_fact(
                    ResourceCategory::EncodedDocumentBytes,
                    bytes.len() + n,
                    MAX_ARCHIVE_BYTES,
                    0,
                    ResourceScope::Application,
                    ResourceUnit::Bytes,
                )],
                "Application capacity: encoded desktop archive exceeds16MiB.",
            ));
        }
        bytes.extend_from_slice(&buffer[..n]);
    }
    Ok(bytes)
}
pub(crate) fn execute(
    work: NativeWork,
    stopped: &mut impl FnMut() -> bool,
) -> Result<NativeCompleted, OperationFailure> {
    match work {
        NativeWork::Game(job) => job(stopped),
        NativeWork::Xiangqi(work) => xiangqi::archive::execute(work, stopped),
        NativeWork::Open { path, token } => {
            let bytes = read(&path, stopped)?;
            let value = crate::document::DocumentEnvelope::parse(&bytes)?;
            let game = match value.game_kind {
                GameKind::Tenjiku => native_session::open::<Tenjiku>(&bytes, stopped)?,
                GameKind::Chu => native_session::open::<Chu>(&bytes, stopped)?,
                GameKind::Tori => native_session::open::<Tori>(&bytes, stopped)?,
                GameKind::Chess => OpenedNative::Chess(Box::new(
                    persistence::decode_chess_payload(value.payload.get().as_bytes(), stopped)?,
                )),
                GameKind::Shogi => native_session::open::<Shogi>(&bytes, stopped)?,
                GameKind::Xiangqi => {
                    let (state, floor) = xiangqi::archive::decode(value.payload.get(), stopped)?;
                    OpenedNative::Xiangqi {
                        state: Box::new(state),
                        floor,
                    }
                }
            };
            persistence::check(stopped)?;
            Ok(NativeCompleted::Opened { game, path, token })
        }
    }
}
impl DesktopController {
    /// Stable shared replacement token; callers confirm unsaved changes against it.
    pub fn shared_open_token(&self) -> SharedOpenToken {
        SharedOpenToken {
            document: self.session.document_epoch,
            version: self.session.version,
            document_revision: self.session.document_revision,
        }
    }
    pub(crate) fn shared_document(&self) -> SharedDocumentState {
        let settings = if let Some(state) = self.session.tenjiku() {
            NativeDesktopSettings::Tenjiku {
                settings: state.document.settings().clone(),
            }
        } else if let Some(state) = self.session.chu() {
            NativeDesktopSettings::Chu {
                settings: state.document.settings().clone(),
            }
        } else if let Some(state) = self.session.tori() {
            NativeDesktopSettings::Tori {
                settings: state.document.settings().clone(),
            }
        } else if let Some(state) = self.session.xiangqi() {
            NativeDesktopSettings::Xiangqi {
                orientation: state.settings.orientation,
                analysis_defaults: state.settings.analysis_defaults,
            }
        } else if let Some(state) = self.session.shogi() {
            NativeDesktopSettings::Shogi {
                settings: state.document.settings().clone(),
            }
        } else {
            NativeDesktopSettings::Chess {
                settings: self.session.chess().settings.clone(),
            }
        };
        SharedDocumentState {
            revision: self.session.document_revision,
            dirty: self.session.document_revision != self.session.saved_revision,
            path: self
                .session
                .document_path
                .as_ref()
                .map(|p| p.to_string_lossy().into_owned()),
            settings,
        }
    }
    /// Queue a complete current-format document using native payload encoding.
    pub fn queue_shared_save(&mut self, path: PathBuf) -> Result<u64, String> {
        if self.session.tenjiku().is_some() {
            return self.queue_game_save::<Tenjiku>(path);
        }
        if self.session.chu().is_some() {
            return self.queue_game_save::<Chu>(path);
        }
        if self.session.tori().is_some() {
            return self.queue_game_save::<Tori>(path);
        }
        if let Some(state) = self.session.xiangqi() {
            let work = xiangqi::archive::XiangqiWork::Save {
                draft: Box::new(xiangqi::archive::XiangqiDraft::capture(state)),
                path,
                revision: self.session.document_revision,
                identity: self.session.document_identity,
                floor: state.execution_floor.max(self.runtime_floor()),
            };
            let id = self.next_native_io(IoPhase::Saving)?;
            self.files
                .submit(id, Work::Native(Box::new(NativeWork::Xiangqi(work))));
            return Ok(id);
        }
        if self.session.shogi().is_some() {
            return self.queue_game_save::<Shogi>(path);
        }
        self.queue_save(path)
    }
    /// Queue strict current-envelope native dispatch. Open never installs resumable work.
    pub fn queue_shared_open(
        &mut self,
        path: PathBuf,
        token: SharedOpenToken,
    ) -> Result<u64, String> {
        self.queue_open_admission(path, token)
    }
    /// Product Open uses the same strict native dispatch and stopped restoration.
    pub fn queue_product_open(
        &mut self,
        path: PathBuf,
        token: SharedOpenToken,
    ) -> Result<u64, String> {
        self.queue_open_admission(path, token)
    }
    fn queue_open_admission(
        &mut self,
        path: PathBuf,
        token: SharedOpenToken,
    ) -> Result<u64, String> {
        if self.lifecycle.close_requested() {
            return Err("Close is pending. Cancel close before opening another document.".into());
        }
        if token != self.shared_open_token() {
            return Err("Stale Open: review the current document again.".into());
        }
        let id = self.next_native_io(IoPhase::Opening)?;
        self.files
            .submit(id, Work::Native(Box::new(NativeWork::Open { path, token })));
        Ok(id)
    }
    pub(crate) fn retire_native_files(&mut self) -> Result<(), String> {
        self.next_native_io(IoPhase::Idle)?;
        self.files.cancel();
        Ok(())
    }
    pub(crate) fn next_native_io(&mut self, phase: IoPhase) -> Result<u64, String> {
        if self.io_state.id >= MAX_SAFE_ID - 1 || self.session.revision >= MAX_SAFE_ID - 2 {
            let (count, limit) = if self.io_state.id >= MAX_SAFE_ID - 1 {
                (self.io_state.id, MAX_SAFE_ID - 2)
            } else {
                (self.session.revision, MAX_SAFE_ID - 3)
            };
            return Err(self.identity_refusal(
                count,
                limit,
                true,
                ResourceMeasurement::Accepted,
                "Application capacity: file-operation identity headroom exhausted.",
            ));
        }
        self.io_state = IoState {
            id: self.io_state.id + 1,
            phase,
            message: None,
            resource: None,
        };
        self.session.revision += 1;
        Ok(self.io_state.id)
    }
    pub(crate) fn runtime_floor(&self) -> u64 {
        [
            self.session.version,
            self.session.archive_revision,
            self.session.next_request,
            self.session.selection_id,
        ]
        .into_iter()
        .max()
        .unwrap()
    }
    pub(crate) fn finish_native_file(
        &mut self,
        completed: NativeCompleted,
    ) -> Result<String, String> {
        match completed {
            NativeCompleted::Runtime { .. } | NativeCompleted::Xiangqi(_) => {
                unreachable!("dedicated admission worker")
            }
            NativeCompleted::Saved {
                path,
                revision,
                identity,
            } => {
                if self.session.document_identity == identity {
                    self.session.document_path = Some(path);
                    if self.session.document_revision == revision {
                        self.session.saved_revision = revision;
                    }
                }
                Ok("Saved complete native desktop archive.".into())
            }
            NativeCompleted::Opened { game, path, token } => {
                if token != self.shared_open_token() {
                    return Err("Stale Open: current native session preserved.".into());
                }
                self.install_document(game, path)
            }
        }
    }
    pub(crate) fn poll_native_admission(&mut self) {
        if self.shared.admission.is_none() {
            return;
        }
        let Some((id, result)) = self.admissions.poll() else {
            return;
        };
        if id != self.session.capacity.id {
            return;
        }
        let Some((epoch, _preferences)) = self.shared.admission.take() else {
            return;
        };
        let transactional_admission = std::mem::take(&mut self.shared.transactional_admission);
        let consumes_response = std::mem::take(&mut self.shared.admission_consumes_response);
        let rollback = self.shared.rollback.take();
        let token = self.admission_token.take();
        let receipt = self.admission_receipt.take();
        if token != Some(self.open_token()) || epoch != self.session.document_epoch {
            if let Some(r) = receipt {
                r.finish(Err("Native admission superseded by document change.".into()));
            }
            return;
        }
        match result {
            Ok(Completed::Native(value)) => match *value {
                NativeCompleted::Runtime {
                    state,
                    replacement,
                    durable,
                } => {
                    self.finish_native_transaction(state, replacement, durable, id, receipt);
                }
                NativeCompleted::Xiangqi(completed) => {
                    self.finish_xiangqi_admission(completed, id, receipt)
                }
                _ => unreachable!("dedicated admission worker"),
            },
            Ok(Completed::Admitted(mut candidate)) => {
                if self.session.tori().is_some()
                    || self.session.chu().is_some()
                    || self.session.tenjiku().is_some()
                {
                    self.bridge.cancel();
                }
                if candidate.document_identity == self.session.document_identity {
                    candidate.document_path = self.session.document_path.clone();
                    candidate.saved_revision = self.session.saved_revision;
                } else {
                    candidate.document_epoch = self.session.document_epoch + 1;
                }
                candidate.validated_observations = candidate.observations.len();
                candidate.revision = candidate.revision.max(self.session.revision + 1);
                candidate.capacity = CapacityState {
                    id,
                    ..CapacityState::default()
                };
                self.session = ActiveDocument::Chess(candidate);
                if let Some(r) = receipt {
                    r.finish(Ok(()));
                }
            }
            Err(error) => {
                if transactional_admission {
                    if error.resource.is_some() {
                        // A capacity refusal supersedes browse-only running intent,
                        // even when the rejected candidate cannot alter the game.
                        self.session.clear_browse_intent();
                    }
                    if consumes_response {
                        self.session.refuse_completed_work(error.resource.is_some());
                    }
                    // Rejected candidates cannot alter accepted game or execution.
                    self.session.capacity = CapacityState {
                        id,
                        status: if error.resource.is_some() {
                            CapacityPhase::Paused
                        } else {
                            CapacityPhase::Available
                        },
                        message: Some(error.diagnostic.clone()),
                        resource: error.resource.clone(),
                    };
                    self.session.revision += 1;
                } else if error.resource.is_some() {
                    self.pause_native_capacity(error.diagnostic.clone());
                    self.session.capacity.resource = error.resource.clone();
                } else {
                    if matches!(&self.session, ActiveDocument::Chess(_)) {
                        self.bridge.cancel();
                        self.session.retire_execution();
                    }
                    self.session.capacity = CapacityState {
                        id,
                        ..CapacityState::default()
                    };
                    if let (Some(state), Some((paused, phase, diagnostic, analysis, had_work))) =
                        (self.session.xiangqi_mut(), rollback)
                    {
                        if consumes_response {
                            // The bridge has already delivered this result. Never
                            // restore Thinking for execution that has ended.
                            self.bridge.cancel();
                            state.retire();
                            state.paused = true;
                            state.phase = EnginePhase::Failed;
                            state.diagnostic = Some(bounded(format!(
                                "Native result rejected by document admission: {error} Retry explicitly; the accepted game is preserved."
                            )));
                        } else if had_work && state.pending.is_none() {
                            state.paused = true;
                            state.diagnostic = Some(
                                "Command rejected; prior execution was retired. Resume explicitly."
                                    .into(),
                            );
                        } else {
                            state.paused = paused;
                            state.phase = phase;
                            state.diagnostic = diagnostic;
                            state.analysis = analysis;
                        }
                    }
                    self.session.revision += 1;
                }
                if let Some(r) = receipt {
                    r.finish(Err(error.diagnostic));
                }
            }
            _ => unreachable!("native admission completion"),
        }
    }
    pub(crate) fn pause_native_failure(&mut self, error: OperationFailure) {
        self.session.clear_browse_intent();
        if error.resource.is_some() {
            self.pause_native_capacity(error.diagnostic);
            self.session.capacity.resource = error.resource;
        } else {
            self.bridge.cancel();
            self.session.retire_execution();
            self.session.revision += 1;
            match &mut self.session {
                ActiveDocument::Chess(state) => state.engine.message = Some(error.diagnostic),
                ActiveDocument::Shogi { state, .. } => state.diagnostic = Some(error.diagnostic),
                ActiveDocument::Xiangqi { state, .. } => state.diagnostic = Some(error.diagnostic),
                ActiveDocument::Tori { state, .. } => state.diagnostic = Some(error.diagnostic),
                ActiveDocument::Chu { state, .. } => state.diagnostic = Some(error.diagnostic),
                ActiveDocument::Tenjiku { state, .. } => state.diagnostic = Some(error.diagnostic),
            }
        }
    }
    pub(crate) fn pause_native_capacity(&mut self, error: String) {
        self.session.clear_browse_intent();
        self.bridge.cancel();
        if matches!(&self.session, ActiveDocument::Chess(_)) {
            self.session.retire_execution();
        }
        if let Some(state) = self.session.shogi_mut() {
            state.retire();
            state.paused = true;
        }
        if let Some(state) = self.session.xiangqi_mut() {
            state.retire();
            state.paused = true;
        }
        if let Some(state) = self.session.tori_mut() {
            state.retire();
            state.paused = true;
        }
        if let Some(state) = self.session.chu_mut() {
            state.retire();
            state.paused = true;
        }
        if let Some(state) = self.session.tenjiku_mut() {
            state.retire();
            state.paused = true;
        }
        self.session.capacity.status = CapacityPhase::Paused;
        self.session.capacity.message = Some(error);
        self.session.revision += 1;
    }
    /// Queue an exact shared command and obtain its stable admission receipt.
    pub fn shared_command_with_receipt(
        &mut self,
        command: SharedDesktopCommand,
    ) -> Result<(SharedSnapshot, AdmissionReceipt), String> {
        let snapshot = self.shared_command(command)?;
        let receipt = self.last_command_receipt.take().unwrap_or_else(|| {
            let r = AdmissionReceipt::pending();
            r.finish(Ok(()));
            r
        });
        Ok((snapshot, receipt))
    }
}

#[cfg(test)]
#[path = "shared_archive_tests.rs"]
mod tests;
