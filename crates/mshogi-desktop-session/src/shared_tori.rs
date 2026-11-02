//! Tori adapter in the sole desktop runtime. Rules and persistence stay native-owned.
use super::*;
use crate::tori_document::{RewindKind, ToriControl, ValidatedToriDocument};
use mshogi_core::tori as t;
use std::sync::Arc;

pub(crate) struct ToriSession {
    pub(crate) document: Arc<ValidatedToriDocument>,
    close_document: Arc<ValidatedToriDocument>,
    pub(crate) game: t::Game,
    viewed: Option<t::Game>,
    pub(crate) browse_stop: Option<BrowseStop>,
    pub(crate) pending: Option<ExecutionContext>,
    pub(crate) phase: EnginePhase,
    pub(crate) paused: bool,
    submitted: bool,
    requested_analysis: Option<WorkLimits>,
    analysis: Option<ExecutionObservation>,
    current_observations: Vec<u64>,
    pub(crate) diagnostic: Option<String>,
}
impl ToriSession {
    pub(crate) fn restore(
        document: ValidatedToriDocument,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, OperationFailure> {
        persistence::check(stopped)?;
        let game = document.record().to_game().map_err(|e| e.to_string())?;
        persistence::check(stopped)?;
        let close_document =
            Arc::new(document.with_reserved_close(document.retired_execution_floor())?);
        Ok(Self { document: Arc::new(document), close_document, game, viewed: None, browse_stop: None, pending: None,
            phase: EnginePhase::Paused, paused: true, submitted: false, requested_analysis: None, analysis: None, current_observations: vec![], diagnostic: Some("Tori document ready. Saved observations are historical; Resume starts native play.".into()) })
    }
    pub(crate) fn selected(&self) -> &t::Game {
        self.viewed.as_ref().unwrap_or(&self.game)
    }
    pub(crate) fn retire(&mut self) {
        self.pending = None;
        self.submitted = false;
        self.requested_analysis = None;
        self.analysis = None;
        self.browse_stop = None;
        self.phase = EnginePhase::Paused;
        // The native context is never recovered by restoring a canonical record.
        self.game
            .retire_context()
            .expect("bounded native runtime identity");
    }
    fn take_back_target(&self) -> Option<usize> {
        let human = self.document.human();
        let events = self.game.events();
        if let Some(last) = events.last()
            && last.actor == human
            && matches!(last.command, t::GameCommand::Resign)
            && self.game.position().side_to_move() == human
            && !self.game.local_position().legal_actions().is_empty()
        {
            return Some(last.index);
        }
        events
            .iter()
            .rposition(|e| e.actor == human && matches!(e.command, t::GameCommand::Play { .. }))
    }
    fn apply(
        &mut self,
        operation: SessionOperation,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<bool, OperationFailure> {
        persistence::check(stopped)?;
        let control = match operation {
            SessionOperation::Tori { actor, command } => {
                if self.viewed.is_some() {
                    return Err("Return to live before changing the game.".into());
                }
                if actor != self.document.human() {
                    return Err("The actor is not the assigned native human.".into());
                }
                // Admission is native Game::submit, not local inspection membership.
                self.game
                    .submit(t::GameRequest {
                        expected: self.game.context(),
                        actor,
                        command,
                    })
                    .map_err(crate::tori_document::native_error)?;
                // Native verification may admit a manual board action while
                // engine play is paused. It must never implicitly Resume a
                // reopened/cancelled/inspected document's execution resource.
                if !matches!(command, t::GameCommand::Play { .. }) {
                    self.paused = true;
                }
                Some(ToriControl::Native { actor, command })
            }
            SessionOperation::Resign {} => {
                return self.apply(
                    SessionOperation::Tori {
                        actor: self.document.human(),
                        command: t::GameCommand::Resign,
                    },
                    stopped,
                );
            }
            SessionOperation::ToriPreferences { settings } => {
                Some(ToriControl::Settings { settings })
            }
            SessionOperation::Undo {} | SessionOperation::TakeBack {} => {
                if self.viewed.is_some() {
                    return Err("Return to live before changing history.".into());
                }
                let (event_index, reason) = if matches!(operation, SessionOperation::Undo {}) {
                    (
                        self.game
                            .events()
                            .len()
                            .checked_sub(1)
                            .ok_or("No event to Undo.")?,
                        RewindKind::Undo,
                    )
                } else {
                    (
                        self.take_back_target()
                            .ok_or("No earlier playable human decision is available.")?,
                        RewindKind::TakeBack,
                    )
                };
                self.paused = true;
                Some(ToriControl::Rewind {
                    event_index,
                    reason,
                })
            }
            SessionOperation::Inspect { event_index } => {
                let record = self
                    .game
                    .record_prefix(event_index)
                    .map_err(|e| e.to_string())?;
                let stop = if self.viewed.is_none() {
                    Some(BrowseStop {
                        paused: self.paused,
                        phase: self.phase,
                        diagnostic: self.diagnostic.clone(),
                    })
                } else {
                    self.browse_stop.clone()
                };
                self.viewed = Some(record.to_game().map_err(|e| e.to_string())?);
                self.retire();
                self.browse_stop = stop;
                self.paused = true;
                self.diagnostic = Some(
                    "Historical position. Analysis is read-only. Return to live explicitly.".into(),
                );
                return Ok(false);
            }
            SessionOperation::ReturnLive {} => {
                let was_viewed = self.viewed.is_some();
                let stop = self.browse_stop.take().unwrap_or(BrowseStop {
                    paused: true,
                    phase: self.phase,
                    diagnostic: self.diagnostic.clone(),
                });
                self.viewed = None;
                self.retire();
                self.paused = true;
                if was_viewed
                    && !stop.paused
                    && self.game.conclusion().is_none()
                    && self.game.snapshot().incomplete.is_none()
                {
                    // Use the same worker-owned Resume admission; a browse intent
                    // never contains or restores an execution request.
                    return self.apply(SessionOperation::Resume {}, stopped);
                }
                self.phase = if matches!(stop.phase, EnginePhase::Idle | EnginePhase::Thinking) {
                    EnginePhase::Paused
                } else {
                    stop.phase
                };
                self.diagnostic = stop.diagnostic;
                return Ok(false);
            }
            SessionOperation::Cancel {} => {
                self.retire();
                self.paused = true;
                self.phase = EnginePhase::Cancelled;
                self.diagnostic = Some(
                    "Work cancelled. The game is unchanged; Resume or start another analysis."
                        .into(),
                );
                return Ok(false);
            }
            SessionOperation::Analyze { operation } => {
                let AnalysisOperation::Moves { depth, nodes } = operation else {
                    return Err("Unsupported Tori analysis operation; no Chess Full or Xiangqi exact operation is available.".into());
                };
                let limits = WorkLimits { depth, nodes };
                validate_limits(limits)?;
                let stop = if self.viewed.is_some() {
                    self.browse_stop.clone()
                } else {
                    None
                };
                self.retire();
                self.browse_stop = stop;
                self.paused = true;
                self.requested_analysis = Some(limits);
                self.diagnostic = None;
                return Ok(false);
            }
            SessionOperation::Resume {} | SessionOperation::Retry {} => {
                if self.viewed.is_some() {
                    return Err("Return to live before resuming play.".into());
                }
                self.retire();
                self.paused = false;
                self.diagnostic = None;
                return Ok(false);
            }
            _ => return Err("Unsupported operation for the native Tori profile.".into()),
        };
        if let Some(control) = control {
            let document = self.document.with_control(control.clone(), stopped)?;
            if matches!(control, ToriControl::Rewind { .. }) {
                self.game = document.record().to_game().map_err(|e| e.to_string())?;
                // Preserve abandoned observations losslessly, but never present
                // retired continuation work as the current engine turn.
                self.current_observations.clear();
            }
            debug_assert_eq!(
                t::GameRecord::from_game(&self.game).ok().as_ref(),
                Some(document.record())
            );
            self.document = Arc::new(document);
            self.viewed = None;
            self.retire();
        }
        Ok(true)
    }
}

pub(crate) struct ToriDraft {
    document: Arc<ValidatedToriDocument>,
    selected: Option<usize>,
    browse_stop: Option<BrowseStop>,
    pending: Option<ExecutionContext>,
    phase: EnginePhase,
    paused: bool,
    submitted: bool,
    requested_analysis: Option<WorkLimits>,
    analysis: Option<ExecutionObservation>,
    current_observations: Vec<u64>,
    diagnostic: Option<String>,
}
impl ToriSession {
    fn capture(&self) -> ToriDraft {
        ToriDraft {
            document: self.document.clone(),
            selected: self.viewed.as_ref().map(|g| g.events().len()),
            browse_stop: self.browse_stop.clone(),
            pending: self.pending.clone(),
            phase: self.phase,
            paused: self.paused,
            submitted: self.submitted,
            requested_analysis: self.requested_analysis,
            analysis: self.analysis.clone(),
            current_observations: self.current_observations.clone(),
            diagnostic: self.diagnostic.clone(),
        }
    }
}
pub(crate) enum RuntimeSeed {
    Current(Box<ToriDraft>),
    Record {
        record: Box<t::GameRecord>,
        human: t::Side,
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
    let mut state = match work.seed {
        RuntimeSeed::Current(draft) => {
            let mut state = Box::new(ToriSession::restore((*draft.document).clone(), stopped)?);
            if let Some(index) = draft.selected {
                state.viewed = Some(
                    state
                        .game
                        .record_prefix(index)
                        .map_err(|e| e.to_string())?
                        .to_game()
                        .map_err(|e| e.to_string())?,
                );
            }
            state.browse_stop = draft.browse_stop;
            state.pending = draft.pending;
            state.phase = draft.phase;
            state.paused = draft.paused;
            state.submitted = draft.submitted;
            state.requested_analysis = draft.requested_analysis;
            state.analysis = draft.analysis;
            state.current_observations = draft.current_observations;
            state.diagnostic = draft.diagnostic;
            state
        }
        RuntimeSeed::Record { record, human } => Box::new(ToriSession::restore(
            ValidatedToriDocument::new(*record, human)?,
            stopped,
        )?),
    };
    let mut durable = false;
    if let Some(operation) = work.operation {
        durable = state.apply(operation, stopped)?;
    }
    if let Some(response) = work.response {
        let pending = state.pending.as_ref().ok_or("No current Tori execution.")?;
        if !response.matches(pending)
            || !matches!(
                response.result,
                ExecutionResult::Failure { .. } | ExecutionResult::Tori { .. }
            )
        {
            return Err("Stale or unsupported Tori observation.".into());
        }
        let analysis = matches!(pending.purpose, ExecutionPurpose::MoveAnalysis { .. });
        let failed = matches!(response.result, ExecutionResult::Failure { .. });
        let incomplete = matches!(
            response.result,
            ExecutionResult::Tori {
                result: mshogi_engine::ToriResult::Incomplete { .. }
            }
        );
        // One worker-owned candidate validates the observation, native continuation,
        // engine action and all persistence headroom before the live state changes.
        state.document = Arc::new(state.document.with_observation(response.clone(), stopped)?);
        state.game = state
            .document
            .record()
            .to_game()
            .map_err(|e| e.to_string())?;
        let browse_stop = if analysis && !failed && !incomplete {
            state.browse_stop.clone()
        } else {
            None
        };
        state.retire();
        state.browse_stop = browse_stop;
        state.current_observations.push(response.context.request_id);
        if analysis {
            state.analysis = Some(response);
        }
        state.paused = analysis || failed || incomplete;
        state.phase = if failed {
            EnginePhase::Failed
        } else if incomplete {
            EnginePhase::Incomplete
        } else {
            EnginePhase::Paused
        };
        state.diagnostic = if failed {
            Some("Engine failure. The game is unchanged; Retry is explicit.".into())
        } else if incomplete {
            Some("Incomplete search. No move, score or game outcome was created.".into())
        } else {
            None
        };
        durable = true;
    }
    state.document = Arc::new(state.document.with_runtime_floor(work.floor)?);
    state.close_document = Arc::new(state.document.with_reserved_close(work.floor)?);
    persistence::check(stopped)?;
    Ok(archive::NativeCompleted::ToriRuntime {
        state,
        replacement: work.replacement,
        durable,
    })
}

impl DesktopController {
    pub(super) fn queue_tori_candidate(
        &mut self,
        seed: RuntimeSeed,
        operation: Option<SessionOperation>,
        response: Option<ExecutionObservation>,
        replacement: bool,
    ) -> Result<AdmissionReceipt, String> {
        self.ensure_runtime_capacity()?;
        if self.admission_token.is_some() {
            return Err("Wait for native document admission.".into());
        }
        if self.session.capacity.id >= MAX_SAFE_ID - 64 {
            return Err(self.identity_refusal(self.session.capacity.id, MAX_SAFE_ID - 65, false, ResourceMeasurement::Accepted, "Application capacity: Tori admission identity exhausted. The complete accepted document remains saveable."));
        }
        // Do not retire accepted execution or files until a replacement succeeds.
        self.session.capacity.id += 1;
        self.session.capacity.status = CapacityPhase::Checking;
        self.session.capacity.message = None;
        self.session.capacity.resource = None;
        self.session.revision += 1;
        let receipt = AdmissionReceipt::pending();
        self.shared.tori_admission = true;
        self.shared.admission_consumes_response = response.is_some();
        self.admission_receipt = Some(receipt.clone());
        self.admission_token = Some(self.open_token());
        self.shared.admission = Some((self.session.document_epoch, false));
        self.admissions.submit(
            self.session.capacity.id,
            persistence::Work::Native(Box::new(archive::NativeWork::ToriRuntime(Box::new(
                RuntimeWork {
                    seed,
                    operation,
                    response,
                    floor: self.runtime_floor(),
                    replacement,
                },
            )))),
        );
        self.last_command_receipt = Some(receipt.clone());
        Ok(receipt)
    }
    pub(super) fn replace_tori(
        &mut self,
        record: t::GameRecord,
        human: t::Side,
    ) -> Result<SharedSnapshot, String> {
        self.queue_tori_candidate(
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
    pub(super) fn tori_command(
        &mut self,
        operation: SessionOperation,
    ) -> Result<SharedSnapshot, String> {
        if matches!(operation, SessionOperation::Cancel {}) {
            self.retire_admission();
            self.bridge.cancel();
            self.session.tori_mut().unwrap().retire();
            self.session.tori_mut().unwrap().paused = true;
            self.session.version += 1;
            self.session.selection_id += 1;
            self.session.revision += 1;
        }
        let state = self.session.tori().unwrap().capture();
        self.queue_tori_candidate(
            RuntimeSeed::Current(Box::new(state)),
            Some(operation),
            None,
            false,
        )?;
        Ok(self.shared_snapshot())
    }
    pub(crate) fn finish_tori_admission(
        &mut self,
        state: Box<ToriSession>,
        replacement: bool,
        durable: bool,
        id: u64,
        receipt: Option<AdmissionReceipt>,
    ) {
        let result = if replacement {
            self.install_document(archive::OpenedNative::Tori(state), PathBuf::new())
                .map(|_| {
                    self.session.document_path = None;
                    self.session.saved_revision = 0;
                })
        } else {
            self.bridge.cancel();
            let ActiveDocument::Tori { state: current, .. } = &mut self.session else {
                unreachable!()
            };
            *current = state;
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
    pub(crate) fn queue_tori_save(&mut self, path: PathBuf) -> Result<u64, String> {
        if self.admission_token.is_some() {
            return Err("Wait for native document admission before Save.".into());
        }
        let state = self.session.tori().unwrap();
        let document = state.document.clone();
        let floor = self.runtime_floor();
        let revision = self.session.document_revision;
        let identity = self.session.document_identity;
        let id = self.next_native_io(IoPhase::Saving)?;
        self.files.submit(
            id,
            persistence::Work::Native(Box::new(archive::NativeWork::ToriSave {
                document,
                floor,
                path,
                revision,
                identity,
            })),
        );
        Ok(id)
    }
    pub(super) fn accept_tori_observation(&mut self, response: ExecutionObservation) -> bool {
        let state = self.session.tori().unwrap();
        let Some(pending) = &state.pending else {
            return false;
        };
        let selected = CanonicalRecord::tori(
            t::GameRecord::from_game(state.selected()).expect("admitted record"),
        );
        if !response.matches(pending)
            || pending.session_version != self.session.version
            || pending.game != selected
            || !matches!(
                response.result,
                ExecutionResult::Failure { .. } | ExecutionResult::Tori { .. }
            )
        {
            return false;
        }
        let seed = RuntimeSeed::Current(Box::new(state.capture()));
        if self
            .queue_tori_candidate(seed, None, Some(response), false)
            .is_err()
        {
            // The bridge already consumed this exact response. Even enqueue
            // headroom refusal must not leave Thinking with no deliverable work.
            self.bridge.cancel();
            let capacity = self.session.capacity.resource.is_some();
            let state = self.session.tori_mut().unwrap();
            state.retire();
            state.paused = true;
            state.phase = if capacity {
                EnginePhase::Paused
            } else {
                EnginePhase::Failed
            };
            return false;
        }
        true
    }
    fn start_tori(&mut self, limits: WorkLimits, analysis: bool) -> Result<(), OperationFailure> {
        validate_limits(limits)?;
        self.ensure_runtime_capacity()?;
        let state = self.session.tori().unwrap();
        let count = state.document.observations().len();
        if count >= MAX_ARCHIVE_OBSERVATIONS {
            return Err(OperationFailure::resource(ResourceFailure::new(
                ResourceFact::measured(
                    ResourceCategory::OrdinaryObservations,
                    count as u64,
                    MAX_ARCHIVE_OBSERVATIONS as u64,
                )
                .with_scope(ResourceScope::Tori)
                .with_unit(ResourceUnit::Observations),
                "Application limit reached: all 256 observation slots are occupied. The complete current game can still be saved.",
            )));
        }
        let game = CanonicalRecord::tori(
            t::GameRecord::from_game(state.selected()).map_err(|e| e.to_string())?,
        );
        let turn = state.selected().position().side_to_move();
        let historical = state.viewed.is_some();
        self.session.next_request += 1;
        let context = ExecutionContext {
            purpose: if analysis {
                ExecutionPurpose::MoveAnalysis {
                    target: ExecutionTarget {
                        selection_id: self.session.selection_id,
                        event_index: game.event_count() as u64,
                        board_ply: game.known_board_actions() as u64,
                        session_version: self.session.version,
                        label: format!(
                            "Tori event {}{}",
                            game.event_count(),
                            if historical { " (history)" } else { " (live)" }
                        ),
                        turn: NativeSide::Tori(turn),
                    },
                }
            } else {
                ExecutionPurpose::EngineTurn {}
            },
            request_id: self.session.next_request,
            process_generation: self.session.next_request,
            session_version: self.session.version,
            game,
            limits,
        };
        let state = self.session.tori_mut().unwrap();
        state.pending = Some(context);
        state.submitted = false;
        state.phase = EnginePhase::Thinking;
        Ok(())
    }
    pub(super) fn poll_tori(&mut self) -> SharedSnapshot {
        if self.admission_token.is_none()
            && let Some(response) = self.bridge.poll()
        {
            self.accept_shared(response);
        }
        if self.admission_token.is_none() {
            let state = self.session.tori_mut().unwrap();
            let analysis = state.requested_analysis.take();
            let play = state.pending.is_none()
                && !state.paused
                && state.viewed.is_none()
                && state.game.conclusion().is_none()
                && state.game.position().side_to_move() != state.document.human();
            if let Some(limits) = analysis {
                if let Err(e) = self.start_tori(limits, true) {
                    self.pause_native_failure(e);
                }
            } else if play && let Err(e) = self.start_tori(play_limits(), false) {
                self.pause_native_failure(e);
            }
            if self.shared.auto_submit()
                && let Some(context) = self.session.tori().unwrap().pending.clone()
                && !self.session.tori().unwrap().submitted
            {
                self.session.tori_mut().unwrap().submitted = true;
                if let Err(e) = self.bridge.submit(execution_request(context.clone())) {
                    self.accept_tori_observation(ExecutionObservation::new(
                        context,
                        ExecutionResult::Failure {
                            failure: mshogi_engine::EngineFailure {
                                kind: mshogi_engine::FailureKind::Start,
                                message: format!("Cannot start native work: {e:?}"),
                            },
                        },
                    ));
                }
            }
        }
        self.tori_snapshot()
    }
    /// Hold technical fixture play; excluded from ordinary distributables.
    #[cfg(feature = "desktop-verification")]
    pub fn verify_tori_pause_play(&mut self) -> Result<(), String> {
        self.bridge.cancel();
        let state = self
            .session
            .tori_mut()
            .ok_or("Requires native Tori document.")?;
        state.retire();
        state.paused = true;
        Ok(())
    }
    /// Project the actual submitted pending state without polling completion.
    #[cfg(feature = "desktop-verification")]
    pub fn verify_tori_pending_snapshot(&self) -> Result<SharedSnapshot, String> {
        if self.session.tori().is_none_or(|s| s.pending.is_none()) {
            return Err("Requires actual pending Tori verification work.".into());
        }
        Ok(self.tori_snapshot())
    }
    /// Verification-only elapsed-clock injection; canonical state is unchanged.
    #[cfg(feature = "desktop-verification")]
    pub fn verify_tori_validation_deadline(
        &mut self,
        file: bool,
    ) -> Result<SharedSnapshot, String> {
        if self.session.tori().is_none() || self.admission_token.is_some() {
            return Err("Requires an idle native Tori document.".into());
        }
        let failure = persistence::verification_deadline_failure();
        if file {
            self.io_state.id += 1;
            self.io_state.phase = IoPhase::Failed;
            self.io_state.message = Some(failure.diagnostic);
            self.io_state.resource = failure.resource;
        } else {
            self.session.capacity.id += 1;
            self.session.capacity.status = CapacityPhase::Paused;
            self.session.capacity.message = Some(failure.diagnostic);
            self.session.capacity.resource = failure.resource;
        }
        self.session.revision += 1;
        Ok(self.tori_snapshot())
    }
    /// Verification entry into the same real native supervised request path.
    #[cfg(any(test, feature = "desktop-verification"))]
    pub fn verify_tori_execution(
        &mut self,
        limits: WorkLimits,
        analysis: bool,
    ) -> Result<ExecutionContext, String> {
        if self.admission_token.is_some() || self.lifecycle.close_requested() {
            return Err("Wait for native operation/close.".into());
        }
        self.start_tori(limits, analysis)
            .map_err(|e| e.diagnostic)?;
        let context = self.session.tori().unwrap().pending.clone().unwrap();
        self.bridge
            .submit(execution_request(context.clone()))
            .map_err(|e| format!("Cannot submit native work: {e:?}"))?;
        self.session.tori_mut().unwrap().submitted = true;
        Ok(context)
    }
    pub(super) fn close_tori(&mut self) {
        if self.session.tori().is_some_and(|s| {
            s.pending.is_none()
                && matches!(s.document.controls().last(), Some(ToriControl::Close {}))
        }) {
            return;
        }
        self.bridge.cancel();
        let state = self.session.tori_mut().unwrap();
        state.retire();
        // Prepared on the admission/Open worker; O(1) swap on native close.
        state.document = state.close_document.clone();
        self.shared_changed();
    }
}

pub(super) fn choice(action: t::Action, actor: t::Side, position: &t::Position) -> PresentedChoice {
    let (origin, from, to, completion) = match action {
        t::Action::Move {
            from,
            to,
            result_face,
        } => (
            ChoiceOrigin::Board {
                square: from.to_string(),
            },
            from.to_string(),
            to,
            position
                .piece(from)
                .filter(|p| p.face != result_face)
                .map(|_| Completion {
                    label: format!("Compulsory promotion to {}", face_name(result_face)),
                    glyph: None,
                }),
        ),
        t::Action::Drop { face, to } => (
            ChoiceOrigin::Hand {
                owner: side(NativeSide::Tori(actor)).key.clone(),
                piece_kind: face.code().into(),
            },
            hand_origin(actor, face),
            to,
            None,
        ),
    };
    PresentedChoice {
        origin,
        action: serde_json::to_string(&action).expect("native action"),
        from,
        steps: vec![to.to_string()],
        completion,
    }
}
fn hand_origin(actor: t::Side, face: t::Face) -> String {
    format!("hand:{}:{}", side(NativeSide::Tori(actor)).key, face.code())
}
pub(super) fn inspect(game: &t::Game, origin: &str) -> Result<inspection::Inspected, String> {
    let (native, actor, face) = if let Ok(square) = origin.parse::<t::Square>() {
        let piece = game
            .position()
            .piece(square)
            .ok_or("No native piece at this origin.")?;
        (t::InspectionOrigin::Board(square), piece.side, piece.face)
    } else {
        let (actor, face) = [t::Side::Sente, t::Side::Gote]
            .into_iter()
            .find_map(|actor| {
                t::Face::ALL.into_iter().find_map(|face| {
                    (origin == hand_origin(actor, face)
                        && game.position().hand(actor).count(face) > 0)
                        .then_some((actor, face))
                })
            })
            .ok_or("Unknown or empty Tori hand origin.")?;
        (t::InspectionOrigin::Hand { side: actor, face }, actor, face)
    };
    let choices = game
        .local_position()
        .inspect(native)
        .into_iter()
        .map(|c| choice(c.action, actor, game.position()))
        .collect();
    Ok((
        GameKind::Tori,
        NativeSide::Tori(actor),
        format!(
            "{} {}{}",
            side(NativeSide::Tori(actor)).label,
            face_name(face),
            if matches!(native, t::InspectionOrigin::Hand { .. }) {
                " in hand"
            } else {
                ""
            }
        ),
        choices,
    ))
}
impl DesktopController {
    pub(crate) fn tori_snapshot(&self) -> SharedSnapshot {
        let state = self.session.tori().unwrap();
        let game = state.selected();
        let position = game.position();
        let record =
            CanonicalRecord::tori(t::GameRecord::from_game(game).expect("admitted native record"));
        let files: Vec<_> = (1..=7).rev().map(|n| n.to_string()).collect();
        let ranks: Vec<_> = ('a'..='g').map(|c| c.to_string()).collect();
        let sides: Vec<_> = [t::Side::Sente, t::Side::Gote]
            .into_iter()
            .map(|s| side(NativeSide::Tori(s)))
            .collect();
        let pieces = t::Square::all()
            .filter_map(|square| {
                position.piece(square).map(|p| PresentedPiece {
                    square: square.to_string(),
                    owner: side(NativeSide::Tori(p.side)).key,
                    kind: p.face.code().into(),
                    promoted: p.face.is_promoted(),
                    text: p.face.code().into(),
                    label: format!(
                        "{square}, {} {}",
                        side(NativeSide::Tori(p.side)).label,
                        face_name(p.face)
                    ),
                })
            })
            .collect();
        let hands = [t::Side::Sente, t::Side::Gote]
            .into_iter()
            .flat_map(|actor| {
                t::Face::ALL
                    .into_iter()
                    .filter(|f| !f.is_promoted() && *f != t::Face::Phoenix)
                    .map(move |face| PresentedHand {
                        origin: hand_origin(actor, face),
                        owner: side(NativeSide::Tori(actor)).key,
                        kind: face.code().into(),
                        count: position.hand(actor).count(face),
                        text: face.code().into(),
                        label: format!(
                            "{} {} in hand, {}",
                            side(NativeSide::Tori(actor)).label,
                            face_name(face),
                            position.hand(actor).count(face)
                        ),
                    })
            })
            .collect();
        let historical = state.viewed.is_some();
        let ended = game.conclusion().is_some();
        SharedSnapshot {
            moves: self.present_moves(game.events().len()),
            xiangqi: None,
            tori: Some(ToriPresentation::new(game, &state.game)),
            open_token: self.shared_open_token(),
            document: self.shared_document(),
            io: self.io_state.clone(),
            capacity: self.session.capacity.clone(),
            token: self.shared_token(),
            game: GameKind::Tori,
            profile: RulesProfile::ToriModernV1,
            edition: t::PROFILE_EDITION,
            human: side(NativeSide::Tori(state.document.human())),
            turn: side(NativeSide::Tori(position.side_to_move())),
            sides: sides.clone(),
            board: BoardDescriptor {
                intersections: None,
                files: files.clone(),
                ranks: ranks.clone(),
                label: "Tori Shogi 7×7 board".into(),
                initial_focus: "4e".into(),
            },
            orientations: vec![
                Orientation {
                    side: sides[0].clone(),
                    files: files.clone(),
                    ranks: ranks.clone(),
                },
                Orientation {
                    side: sides[1].clone(),
                    files: files.into_iter().rev().collect(),
                    ranks: ranks.into_iter().rev().collect(),
                },
            ],
            pieces,
            hands,
            choices: if historical {
                vec![]
            } else {
                game.legal_actions()
                    .into_iter()
                    .map(|a| choice(a, position.side_to_move(), position))
                    .collect()
            },
            capabilities: CommandCapabilities {
                board_action: !historical
                    && !ended
                    && position.side_to_move() == state.document.human(),
                drops: true,
                promotion: true,
                take_back: !historical && state.take_back_target().is_some(),
                undo: !historical && !state.game.events().is_empty(),
                engine_turn: true,
                move_analysis: true,
                chess_full: false,
                claims: false,
                declaration: false,
                resign: !historical && !ended,
                hands: true,
                history: true,
                save: true,
            },
            status: PresentedStatus {
                concluded: ended,
                label: status_label(game),
                score_half_points: game.conclusion().map(|c| {
                    let winner = match c {
                        t::Conclusion::Checkmate { winner }
                        | t::Conclusion::Resignation { winner } => winner,
                        t::Conclusion::StalemateLoss { loser }
                        | t::Conclusion::RepetitionForfeit { loser } => loser.opponent(),
                    };
                    if winner == t::Side::Sente {
                        [2, 0]
                    } else {
                        [0, 2]
                    }
                }),
            },
            events: state
                .game
                .events()
                .iter()
                .map(|e| PresentedEvent {
                    index: e.index + 1,
                    board_ply: e.board_ply,
                    label: event_label(e),
                })
                .collect(),
            event_index: game.events().len(),
            board_ply: record.known_board_actions() as u64,
            play_limits: play_limits(),
            historical,
            paused: state.paused,
            close_requires_save: self.close_requires_save(),
            phase: state.phase,
            execution_purpose: state.pending.as_ref().map(|p| {
                if matches!(p.purpose, ExecutionPurpose::EngineTurn {}) {
                    "engine_turn"
                } else {
                    "move_analysis"
                }
            }),
            diagnostic: state.diagnostic.clone(),
            declaration_advice: vec![],
            context: record,
            analysis: state.analysis.as_ref().map(|o| PresentedObservation {
                observation: o.clone(),
                historical: false,
                application_error: None,
            }),
            observations: state
                .document
                .observations()
                .iter()
                .map(|o| PresentedObservation {
                    observation: o.clone(),
                    historical: !state.current_observations.contains(&o.context.request_id),
                    application_error: None,
                })
                .collect(),
            search_observations: state
                .document
                .observations()
                .iter()
                .enumerate()
                .map(|(index, o)| display_observation(index, o, state))
                .collect(),
            score_meaning: "Native root-side evaluation in hundredths of a Swallow. Win/loss distances count board actions; failures and incomplete work are unscored.",
            chess: None,
            chess_match: None,
        }
    }
}
#[cfg(test)]
#[path = "shared_tori_tests.rs"]
mod tests;

#[path = "shared_tori_presentation.rs"]
mod presentation;
pub use presentation::ToriPresentation;
use presentation::{event_label, face_name, status_label};

fn play_limits() -> WorkLimits {
    mshogi_engine::TORI_PLAY_LIMITS
}
fn validate_limits(limits: WorkLimits) -> Result<(), OperationFailure> {
    if limits.depth == 0
        || limits.depth > mshogi_engine::TORI_MAX_DEPTH
        || limits.nodes == 0
        || limits.nodes > mshogi_engine::TORI_MAX_NODES
    {
        return Err(
            "Unsupported Tori work limits. Use a positive bounded native depth and visit limit."
                .into(),
        );
    }
    Ok(())
}

fn display_observation(
    index: usize,
    o: &ExecutionObservation,
    state: &ToriSession,
) -> PresentedSearchObservation {
    let CanonicalRecord::Tori(record) = &o.context.game else {
        unreachable!("validated Tori observation")
    };
    let telemetry = match &o.result {
        ExecutionResult::Tori { result } => Some(result.telemetry()),
        _ => None,
    };
    PresentedSearchObservation {
        observation_index: index,
        purpose: if matches!(o.context.purpose, ExecutionPurpose::EngineTurn {}) {
            "engine_turn"
        } else {
            "move_analysis"
        },
        event_index: record.event_count(),
        board_ply: record.known_board_actions(),
        perspective: side(NativeSide::Tori(record.side_to_move())).label,
        engine: "mShogi",
        score: telemetry.and_then(|t| t.score_label()),
        completed_depth: telemetry.map(|t| t.completed_depth),
        nodes: telemetry.map(|t| t.nodes),
        // Complete native encodings remain honest technical continuation data;
        // readable action notation is projected by the native Moves adapter.
        variation: telemetry.map_or_else(Vec::new, |t| {
            t.principal_variation
                .iter()
                .map(|a| serde_json::to_string(a).expect("native action"))
                .collect()
        }),
        status: match telemetry.map(|t| t.status) {
            Some(mshogi_engine::StopReason::Complete) => "Complete",
            Some(mshogi_engine::StopReason::NodeLimit) => "Node limit reached",
            Some(mshogi_engine::StopReason::Stopped) => "Search stopped",
            None => "Engine failure",
        },
        historical: !state.current_observations.contains(&o.context.request_id),
        selected: state.analysis.as_ref() == Some(o),
    }
}
