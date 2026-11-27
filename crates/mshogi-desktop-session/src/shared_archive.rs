//! Versioned lossless archives for the concrete shared desktop consumers.
use super::*;
use crate::persistence::{self, Completed, Work};
use std::{fs, io::Read};

/// Complete shared document metadata; execution ownership is never a saved setting.
#[derive(Clone, Debug, Serialize)]
pub struct SharedDocumentState {
    /// Monotonic accepted document-content revision.
    pub revision: u64,
    /// True until the exact accepted revision is successfully saved.
    pub dirty: bool,
    /// Path of the current document, if any.
    pub path: Option<String>,
    /// Document settings, the same shape in every game.
    pub settings: crate::game_document::GameSettings<TaggedSide>,
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
    Box<dyn FnOnce(&mut dyn FnMut() -> bool) -> Result<GameCompleted, OperationFailure> + Send>;
pub(crate) enum GameWork {
    /// Runtime admission or save of a game on the generic session.
    Game(GameJob),
    Open {
        path: PathBuf,
        token: SharedOpenToken,
    },
}
pub(crate) enum OpenedGame {
    Chess(Box<chess::ChessSession>),
    Tenjiku(Box<tenjiku::TenjikuSession>),
    Chu(Box<chu::ChuSession>),
    Tori(Box<tori::ToriSession>),
    Xiangqi(Box<xiangqi::XiangqiSession>),
    Shogi(Box<shogi::ShogiSession>),
}
impl OpenedGame {
    /// A generic game's session still holds pending work.
    pub(crate) fn has_pending_work(&self) -> bool {
        match self {
            Self::Shogi(state) => state.pending.is_some(),
            Self::Tori(state) => state.pending.is_some(),
            Self::Chu(state) => state.pending.is_some(),
            Self::Tenjiku(state) => state.pending.is_some(),
            Self::Xiangqi(state) => state.pending.is_some(),
            Self::Chess(state) => state.pending.is_some(),
        }
    }
}
pub(crate) enum GameCompleted {
    /// A generic game's session after runtime admission.
    Runtime {
        state: OpenedGame,
        replacement: bool,
        durable: bool,
    },
    Saved {
        path: PathBuf,
        revision: u64,
        identity: u64,
    },
    Opened {
        game: OpenedGame,
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
        bytes.extend_from_slice(&buffer[..n]);
    }
    Ok(bytes)
}
pub(crate) fn execute(
    work: GameWork,
    stopped: &mut impl FnMut() -> bool,
) -> Result<GameCompleted, OperationFailure> {
    match work {
        GameWork::Game(job) => job(stopped),
        GameWork::Open { path, token } => {
            let bytes = read(&path, stopped)?;
            let value = crate::document::DocumentEnvelope::parse(&bytes)?;
            let game = with_game!(value.game_kind, G => game_session::open::<G>(&bytes, stopped)?);
            persistence::check(stopped)?;
            Ok(GameCompleted::Opened { game, path, token })
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
        let settings = with_game!(self.session.kind(), G => G::state(&self.session)
            .expect("rules owner")
            .document
            .settings()
            .map(G::tagged_side));
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
    /// Queue a complete current-format document using game payload encoding.
    pub fn queue_shared_save(&mut self, path: PathBuf) -> Result<u64, String> {
        with_game!(self.session.kind(), G => self.queue_game_save::<G>(path))
    }
    /// Queue strict current-envelope game dispatch. Open never installs resumable work.
    pub fn queue_shared_open(
        &mut self,
        path: PathBuf,
        token: SharedOpenToken,
    ) -> Result<u64, String> {
        self.queue_open_admission(path, token)
    }
    /// Product Open uses the same strict game dispatch and stopped restoration.
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
        let id = self.next_io(IoPhase::Opening)?;
        self.files
            .submit(id, Work::Archive(Box::new(GameWork::Open { path, token })));
        Ok(id)
    }

    pub(crate) fn next_io(&mut self, phase: IoPhase) -> Result<u64, String> {
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
    #[cfg(test)]
    pub(crate) fn retire_files(&mut self) -> Result<(), String> {
        self.next_io(IoPhase::Idle)?;
        self.files.cancel();
        Ok(())
    }
    pub(crate) fn runtime_floor(&self) -> u64 {
        [
            self.session.version,
            self.session.next_request,
            self.session.selection_id,
        ]
        .into_iter()
        .max()
        .unwrap()
    }
    pub(crate) fn finish_file(&mut self, completed: GameCompleted) -> Result<String, String> {
        match completed {
            GameCompleted::Runtime { .. } => {
                unreachable!("dedicated admission worker")
            }
            GameCompleted::Saved {
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
                Ok("Saved complete desktop archive.".into())
            }
            GameCompleted::Opened { game, path, token } => {
                if token != self.shared_open_token() {
                    return Err("Stale Open: current game session preserved.".into());
                }
                self.install_document(game, path)
            }
        }
    }
    pub(crate) fn poll_admission(&mut self) {
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
        let token = self.admission_token.take();
        let receipt = self.admission_receipt.take();
        if token != Some(self.open_token()) || epoch != self.session.document_epoch {
            if let Some(r) = receipt {
                r.finish(Err("Game admission superseded by document change.".into()));
            }
            return;
        }
        match result {
            Ok(Completed::Archive(value)) => match *value {
                GameCompleted::Runtime {
                    state,
                    replacement,
                    durable,
                } => {
                    self.finish_transaction(
                        state,
                        replacement,
                        durable,
                        consumes_response,
                        id,
                        receipt,
                    );
                }
                _ => unreachable!("dedicated admission worker"),
            },

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
                    self.pause_capacity(error.diagnostic.clone());
                    self.session.capacity.resource = error.resource.clone();
                } else {
                    self.session.capacity = CapacityState {
                        id,
                        ..CapacityState::default()
                    };
                    self.session.revision += 1;
                }
                if let Some(r) = receipt {
                    r.finish(Err(error.diagnostic));
                }
            }
        }
    }
    pub(crate) fn pause_failure(&mut self, error: OperationFailure) {
        self.session.clear_browse_intent();
        if error.resource.is_some() {
            self.pause_capacity(error.diagnostic);
            self.session.capacity.resource = error.resource;
        } else {
            self.bridge.cancel();
            self.session.retire_execution();
            self.session.revision += 1;
            match &mut self.session {
                ActiveDocument::Chess { state, .. } => state.diagnostic = Some(error.diagnostic),
                ActiveDocument::Shogi { state, .. } => state.diagnostic = Some(error.diagnostic),
                ActiveDocument::Xiangqi { state, .. } => state.diagnostic = Some(error.diagnostic),
                ActiveDocument::Tori { state, .. } => state.diagnostic = Some(error.diagnostic),
                ActiveDocument::Chu { state, .. } => state.diagnostic = Some(error.diagnostic),
                ActiveDocument::Tenjiku { state, .. } => state.diagnostic = Some(error.diagnostic),
            }
        }
    }
    pub(crate) fn pause_capacity(&mut self, error: String) {
        self.session.clear_browse_intent();
        self.bridge.cancel();

        if let Some(state) = self.session.chess_mut() {
            state.retire();
            state.paused = true;
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
