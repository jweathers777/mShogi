//! Native Chu adapter in the existing application controller and file worker.
use super::*;
use crate::chu_document::{ChuControl, RewindKind, ValidatedChuDocument};
use mshogi_core::chu as c;
use std::sync::Arc;

pub(crate) struct ChuSession {
    pub(crate) document: Arc<ValidatedChuDocument>,
    close_document: Arc<ValidatedChuDocument>,
    pub(crate) game: c::Game,
    viewed: Option<c::Game>,
    pub(crate) pending: Option<ExecutionContext>,
    pub(crate) phase: EnginePhase,
    pub(crate) diagnostic: Option<String>,
    presentation: ChuPresentation,
    choices: Vec<PresentedChoice>,
    status: PresentedStatus,
}
impl ChuSession {
    pub(crate) fn restore(
        document: ValidatedChuDocument,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, OperationFailure> {
        persistence::check(stopped)?;
        let game = document.record().to_game().map_err(|e| e.to_string())?;
        persistence::check(stopped)?;
        let viewed = if let Some(i) = document.selected_event() {
            let prefix = game.record_prefix(i).map_err(|e| e.to_string())?;
            persistence::check(stopped)?;
            let viewed = prefix.to_game().map_err(|e| e.to_string())?;
            persistence::check(stopped)?;
            Some(viewed)
        } else {
            None
        };
        let close_document =
            Arc::new(document.with_reserved_close(document.retired_execution_floor())?);
        persistence::check(stopped)?;
        let selected = viewed.as_ref().unwrap_or(&game);
        let presentation = ChuPresentation::new(selected, &game);
        let actions = if viewed.is_some() {
            vec![]
        } else {
            selected.legal_actions()
        };
        let choices = actions
            .iter()
            .map(|a| presentation::choice(a.clone(), &actions))
            .collect();
        let status = presentation::status(selected);
        persistence::check(stopped)?;
        Ok(Self { presentation, choices, status, document: Arc::new(document), close_document, game, viewed, pending: None,
            phase: EnginePhase::Paused, diagnostic: Some("Chu shared contract. Engine play and analysis are not implemented; saved observations are historical.".into()) })
    }
    pub(crate) fn selected(&self) -> &c::Game {
        self.viewed.as_ref().unwrap_or(&self.game)
    }
    pub(crate) fn retire(&mut self) {
        self.pending = None;
        self.phase = EnginePhase::Paused;
        self.game
            .retire_context()
            .expect("bounded native runtime identity");
    }
    fn take_back_target(&self) -> Option<usize> {
        let human = self.document.human();
        let events = self.game.events();
        if let Some(e) = events.last()
            && e.actor == human
            && matches!(e.command, c::GameCommand::Resign)
            && self.game.position().side_to_move() == human
        {
            return Some(e.index);
        }
        events
            .iter()
            .rposition(|e| e.actor == human && matches!(e.command, c::GameCommand::Play { .. }))
    }
}
pub(crate) enum RuntimeSeed {
    Current {
        document: Arc<ValidatedChuDocument>,
        pending: Option<ExecutionContext>,
        phase: EnginePhase,
        diagnostic: Option<String>,
    },
    Record {
        record: Box<c::GameRecord>,
        human: c::Side,
    },
}
pub(crate) struct RuntimeWork {
    pub(crate) seed: RuntimeSeed,
    pub(crate) operation: Option<SessionOperation>,
    pub(crate) response: Option<ExecutionObservation>,
    pub(crate) floor: u64,
    pub(crate) replacement: bool,
}
pub(crate) fn execute(
    work: RuntimeWork,
    stopped: &mut impl FnMut() -> bool,
) -> Result<archive::NativeCompleted, OperationFailure> {
    persistence::check(stopped)?;
    let (mut doc, pending, mut phase, mut diagnostic) = match work.seed {
        RuntimeSeed::Record { record, human } => (
            ValidatedChuDocument::new_with_stop(*record, human, stopped)?,
            None,
            EnginePhase::Paused,
            None,
        ),
        RuntimeSeed::Current {
            document,
            pending,
            phase,
            diagnostic,
        } => ((*document).clone(), pending, phase, diagnostic),
    };
    let mut durable = false;
    if let Some(operation) = work.operation {
        if matches!(phase, EnginePhase::Thinking | EnginePhase::Idle) {
            phase = EnginePhase::Paused;
        }
        let mut control = None;
        match operation {
            SessionOperation::Chu { actor, command } => {
                if doc.selected_event().is_some() {
                    return Err("Return to live before changing the game.".into());
                }
                // This gated infrastructure consumer admits explicit native actors.
                // Board turn and terminal/offer semantics belong exclusively to Game.
                control = Some(ChuControl::Native { actor, command });
            }
            SessionOperation::Resign {} => {
                if doc.selected_event().is_some() {
                    return Err("Return to live before resignation.".into());
                }
                control = Some(ChuControl::Native {
                    actor: doc.human(),
                    command: c::GameCommand::Resign,
                });
            }
            SessionOperation::ChuPreferences { settings } => {
                control = Some(ChuControl::Settings { settings })
            }
            SessionOperation::Undo {} | SessionOperation::TakeBack {} => {
                if doc.selected_event().is_some() {
                    return Err("Return to live before changing history.".into());
                }
                let game = doc.record().to_game().map_err(|e| e.to_string())?;
                persistence::check(stopped)?;
                let index = if matches!(operation, SessionOperation::Undo {}) {
                    game.events()
                        .len()
                        .checked_sub(1)
                        .ok_or("No native event to Undo.")?
                } else {
                    let human = doc.human();
                    if let Some(e) = game.events().last()
                        && e.actor == human
                        && matches!(e.command, c::GameCommand::Resign)
                        && game.position().side_to_move() == human
                    {
                        e.index
                    } else {
                        game.events()
                            .iter()
                            .rposition(|e| {
                                e.actor == human && matches!(e.command, c::GameCommand::Play { .. })
                            })
                            .ok_or("No earlier human decision is available.")?
                    }
                };
                control = Some(ChuControl::Rewind {
                    event_index: index,
                    reason: if matches!(operation, SessionOperation::Undo {}) {
                        RewindKind::Undo
                    } else {
                        RewindKind::TakeBack
                    },
                });
            }
            SessionOperation::Inspect { event_index } => {
                doc = doc.with_selection(Some(event_index), stopped)?;
            }
            SessionOperation::ReturnLive {} => {
                doc = doc.with_selection(None, stopped)?;
            }
            SessionOperation::Cancel {} => {
                phase = EnginePhase::Cancelled;
                diagnostic = Some("Work cancelled. No move or game outcome was created.".into());
            }
            SessionOperation::Resume {}
            | SessionOperation::Retry {}
            | SessionOperation::Analyze { .. } => {
                return Err("Chu execution is unsupported until MSH-43; no move, analysis, score or outcome was created.".into());
            }
            _ => return Err("Unsupported command for the native Chu profile.".into()),
        }
        if let Some(control) = control {
            doc = doc.with_control(control, stopped)?;
            durable = true;
        }
    }
    if let Some(response) = work.response {
        let pending = pending
            .as_ref()
            .ok_or("No current Chu verification request.")?;
        if !response.matches(pending) || !matches!(response.result, ExecutionResult::Failure { .. })
        {
            return Err("Stale or unsupported Chu execution response.".into());
        }
        diagnostic = Some(match &response.result {
            ExecutionResult::Failure { failure }
                if failure.kind == mshogi_engine::FailureKind::UnsupportedMode =>
            {
                "Chu execution is unsupported. The game is unchanged and unscored.".into()
            }
            ExecutionResult::Failure { failure } => archive::bounded(format!(
                "Native process failure · unscored. The game is unchanged. {}",
                failure.message
            )),
            _ => unreachable!("validated failure-only response"),
        });
        doc = doc.with_observation(response, stopped)?;
        phase = EnginePhase::Failed;
        durable = true;
    }
    let mut state = ChuSession::restore(doc.with_runtime_floor(work.floor)?, stopped)?;
    state.phase = phase;
    if diagnostic.is_some() {
        state.diagnostic = diagnostic;
    }
    Ok(archive::NativeCompleted::ChuRuntime {
        state: Box::new(state),
        replacement: work.replacement,
        durable,
    })
}
impl DesktopController {
    fn chu_seed(&self) -> RuntimeSeed {
        let state = self.session.chu().expect("Chu owner");
        RuntimeSeed::Current {
            document: state.document.clone(),
            pending: state.pending.clone(),
            phase: state.phase,
            diagnostic: state.diagnostic.clone(),
        }
    }
    pub(super) fn replace_chu(
        &mut self,
        record: c::GameRecord,
        human: c::Side,
    ) -> Result<SharedSnapshot, String> {
        self.queue_chu_candidate(
            RuntimeSeed::Record {
                record: Box::new(record),
                human,
            },
            None,
            None,
            true,
        )?;
        Ok(self.shared_snapshot())
    }
    fn queue_chu_candidate(
        &mut self,
        seed: RuntimeSeed,
        operation: Option<SessionOperation>,
        response: Option<ExecutionObservation>,
        replacement: bool,
    ) -> Result<AdmissionReceipt, String> {
        let consumes_response = response.is_some();
        let work = archive::NativeWork::ChuRuntime(Box::new(RuntimeWork {
            seed,
            operation,
            response,
            replacement,
            floor: self.runtime_floor(),
        }));
        self.queue_native_transaction(work, consumes_response)
    }
    pub(super) fn chu_command(
        &mut self,
        operation: SessionOperation,
    ) -> Result<SharedSnapshot, String> {
        if matches!(operation, SessionOperation::Cancel {}) {
            self.retire_admission();
            self.bridge.cancel();
            self.session.chu_mut().unwrap().retire();
            self.session.version += 1;
            self.session.selection_id += 1;
            self.session.revision += 1;
        }
        self.queue_chu_candidate(self.chu_seed(), Some(operation), None, false)?;
        Ok(self.shared_snapshot())
    }
    pub(crate) fn queue_chu_save(&mut self, path: PathBuf) -> Result<u64, String> {
        if self.admission_token.is_some() {
            return Err("Wait for native admission before Save.".into());
        }
        let document = self.session.chu().unwrap().document.clone();
        let floor = self.runtime_floor();
        let revision = self.session.document_revision;
        let identity = self.session.document_identity;
        let id = self.next_native_io(IoPhase::Saving)?;
        self.files.submit(
            id,
            persistence::Work::Native(Box::new(archive::NativeWork::ChuSave {
                document,
                floor,
                path,
                revision,
                identity,
            })),
        );
        Ok(id)
    }
    pub(super) fn accept_chu_observation(&mut self, response: ExecutionObservation) -> bool {
        let state = self.session.chu().unwrap();
        let Some(pending) = &state.pending else {
            return false;
        };
        let selected = CanonicalRecord::chu(
            c::GameRecord::from_game(state.selected()).expect("admitted native game"),
        );
        if pending.session_version != self.session.version
            || pending.game != selected
            || !response.matches(pending)
            || !matches!(response.result, ExecutionResult::Failure { .. })
        {
            return false;
        }
        if self
            .queue_chu_candidate(self.chu_seed(), None, Some(response), false)
            .is_err()
        {
            self.bridge.cancel();
            self.session.chu_mut().unwrap().retire();
            return false;
        }
        true
    }
    /// Explicit verification-only entry into the actual supervised unsupported path.
    #[cfg(any(test, feature = "desktop-verification"))]
    pub fn verify_chu_execution(
        &mut self,
        limits: WorkLimits,
        analysis: bool,
    ) -> Result<ExecutionContext, String> {
        if self.admission_token.is_some() || self.lifecycle.close_requested() {
            return Err("Wait for admission or close.".into());
        }
        self.ensure_runtime_capacity()?;
        let state = self
            .session
            .chu()
            .ok_or("Requires a native Chu document.")?;
        if state.document.observations().len() >= MAX_ARCHIVE_OBSERVATIONS {
            return Err(
                "Application observation capacity reached; the current game is saveable.".into(),
            );
        }
        let game = CanonicalRecord::chu(
            c::GameRecord::from_game(state.selected()).map_err(|e| e.to_string())?,
        );
        let turn = NativeSide::Chu(state.selected().position().side_to_move());
        let event_index = game.event_count() as u64;
        let board_ply = game.known_board_actions() as u64;
        let next_request = self.session.next_request + 1;
        let context = ExecutionContext {
            purpose: if analysis {
                ExecutionPurpose::MoveAnalysis {
                    target: ExecutionTarget {
                        selection_id: self.session.selection_id,
                        event_index,
                        board_ply,
                        session_version: self.session.version,
                        label: format!("Chu event {event_index}"),
                        turn,
                    },
                }
            } else {
                ExecutionPurpose::EngineTurn {}
            },
            request_id: next_request,
            session_version: self.session.version,
            process_generation: next_request,
            game,
            limits,
        };
        let request = execution_request(context.clone());
        if !request.valid() {
            return Err("Unsupported Chu verification request limits or target.".into());
        }
        self.bridge.cancel();
        self.session.chu_mut().unwrap().retire();
        self.session.next_request = next_request;
        self.bridge
            .submit(request)
            .map_err(|e| format!("Cannot submit native work: {e:?}"))?;
        let state = self.session.chu_mut().unwrap();
        state.pending = Some(context.clone());
        state.phase = EnginePhase::Thinking;
        self.session.revision += 1;
        Ok(context)
    }
    pub(super) fn poll_chu(&mut self) -> SharedSnapshot {
        self.poll_files();
        if self.session.chu().is_none() {
            return self.shared_snapshot();
        }
        if self.admission_token.is_none()
            && let Some(response) = self.bridge.poll()
        {
            self.accept_shared(response);
        }
        self.chu_snapshot()
    }
    pub(super) fn close_chu(&mut self) {
        let state = self.session.chu().unwrap();
        if state.pending.is_none()
            && matches!(state.document.controls().last(), Some(ChuControl::Close {}))
        {
            return;
        }
        self.bridge.cancel();
        let state = self.session.chu_mut().unwrap();
        state.retire();
        state.document = state.close_document.clone();
        self.shared_changed();
    }
}

#[path = "shared_chu_presentation.rs"]
mod presentation;
pub use presentation::ChuPresentation;
pub(super) fn inspect(game: &c::Game, origin: &str) -> Result<inspection::Inspected, String> {
    presentation::inspect(game, origin)
}
