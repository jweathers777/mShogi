//! Tenjiku adapter in the sole desktop runtime. Rules and persistence stay native-owned.
use super::*;
use crate::tenjiku_document::{RewindKind, TenjikuControl, ValidatedTenjikuDocument};
use mshogi_core::tenjiku as c;
use std::sync::Arc;

pub(crate) struct TenjikuSession {
    pub(crate) document: Arc<ValidatedTenjikuDocument>,
    close_document: Arc<ValidatedTenjikuDocument>,
    pub(crate) game: c::Game,
    viewed: Option<c::Game>,
    pub(crate) browse_stop: Option<BrowseStop>,
    pub(crate) pending: Option<ExecutionContext>,
    pub(crate) phase: EnginePhase,
    pub(crate) paused: bool,
    submitted: bool,
    requested_analysis: Option<WorkLimits>,
    analysis: Option<ExecutionObservation>,
    current_observations: Vec<u64>,
    pub(crate) diagnostic: Option<String>,
    presentation: TenjikuPresentation,
    choices: Vec<PresentedChoice>,
    status: PresentedStatus,
}
impl TenjikuSession {
    pub(crate) fn restore(
        document: ValidatedTenjikuDocument,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, OperationFailure> {
        persistence::check(stopped)?;
        let game = document
            .record()
            .to_game_with_stop(stopped)
            .map_err(|e| e.to_string())?;
        persistence::check(stopped)?;
        let close_document =
            Arc::new(document.with_reserved_close(document.retired_execution_floor())?);
        let viewed = if let Some(index) = document.selected_event() {
            let prefix = game
                .record_prefix_with_stop(index, stopped)
                .map_err(|e| e.to_string())?;
            persistence::check(stopped)?;
            Some(
                prefix
                    .to_game_with_stop(stopped)
                    .map_err(|e| e.to_string())?,
            )
        } else {
            None
        };
        persistence::check(stopped)?;
        let selected = viewed.as_ref().unwrap_or(&game);
        let presentation = TenjikuPresentation::new(selected, &game);
        let actions = selected
            .legal_actions_with_stop(stopped)
            .map_err(|e| e.to_string())?;
        let status = presentation::status(selected, !actions.is_empty());
        let choices = actions
            .iter()
            .filter(|_| viewed.is_none())
            .map(|a| presentation::choice(a.clone(), &actions))
            .collect();
        persistence::check(stopped)?;
        Ok(Self {
            document: Arc::new(document),
            close_document,
            game,
            viewed,
            browse_stop: None,
            pending: None,
            phase: EnginePhase::Paused,
            paused: true,
            submitted: false,
            requested_analysis: None,
            analysis: None,
            current_observations: vec![],
            diagnostic: Some("Saved observations are historical. Resume starts play.".into()),
            presentation,
            choices,
            status,
        })
    }
    pub(crate) fn selected(&self) -> &c::Game {
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
            && matches!(last.command, c::GameCommand::Resign)
            && self.game.position().side_to_move() == human
        {
            return Some(last.index);
        }
        events
            .iter()
            .rposition(|e| e.actor == human && matches!(e.command, c::GameCommand::Play { .. }))
    }
    fn apply(
        &mut self,
        operation: SessionOperation,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<bool, OperationFailure> {
        persistence::check(stopped)?;
        let control = match operation {
            SessionOperation::Tenjiku { actor, command } => {
                if self.viewed.is_some() {
                    return Err("Return to live before changing the game.".into());
                }
                // Actor/turn/terminal/draw admission belongs to the native Game.
                // The immutable document candidate admits the complete command below.
                if matches!(
                    command,
                    c::GameCommand::Resign | c::GameCommand::Stop { .. }
                ) {
                    self.paused = true;
                }
                Some(TenjikuControl::Native { actor, command })
            }
            SessionOperation::Resign {} => {
                return self.apply(
                    SessionOperation::Tenjiku {
                        actor: self.document.human(),
                        command: c::GameCommand::Resign,
                    },
                    stopped,
                );
            }
            SessionOperation::TenjikuPreferences { settings } => {
                Some(TenjikuControl::Settings { settings })
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
                Some(TenjikuControl::Rewind {
                    event_index,
                    reason,
                })
            }
            SessionOperation::Inspect { event_index } => {
                let record = self
                    .game
                    .record_prefix_with_stop(event_index, stopped)
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
                self.document = Arc::new(self.document.with_selection(Some(event_index), stopped)?);
                self.viewed = Some(
                    record
                        .to_game_with_stop(stopped)
                        .map_err(|e| e.to_string())?,
                );
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
                self.document = Arc::new(self.document.with_selection(None, stopped)?);
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
                    return Err("Unsupported Tenjiku analysis operation; no Chess Full or Xiangqi exact operation is available.".into());
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
            _ => return Err("Unsupported operation for the native Tenjiku profile.".into()),
        };
        if let Some(control) = control {
            let document = self.document.with_control(control.clone(), stopped)?;
            self.game = document
                .record()
                .to_game_with_stop(stopped)
                .map_err(|e| e.to_string())?;
            if matches!(control, TenjikuControl::Rewind { .. }) {
                self.current_observations.clear();
            }
            self.viewed = if let Some(index) = document.selected_event() {
                Some(
                    self.game
                        .record_prefix_with_stop(index, stopped)
                        .map_err(|e| e.to_string())?
                        .to_game_with_stop(stopped)
                        .map_err(|e| e.to_string())?,
                )
            } else {
                None
            };
            self.document = Arc::new(document);
            self.retire();
        }
        Ok(true)
    }
}

pub(crate) struct TenjikuDraft {
    document: Arc<ValidatedTenjikuDocument>,
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
impl TenjikuSession {
    fn capture(&self) -> TenjikuDraft {
        TenjikuDraft {
            document: self.document.clone(),
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
    Current(Box<TenjikuDraft>),
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
    let mut state = match work.seed {
        RuntimeSeed::Current(draft) => {
            let mut state = Box::new(TenjikuSession::restore((*draft.document).clone(), stopped)?);
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
        RuntimeSeed::Record { record, human } => Box::new(TenjikuSession::restore(
            ValidatedTenjikuDocument::new_with_stop(*record, human, stopped)?,
            stopped,
        )?),
    };
    let mut durable = false;
    if let Some(operation) = work.operation {
        durable = state.apply(operation, stopped)?;
    }
    if let Some(response) = work.response {
        let pending = state
            .pending
            .as_ref()
            .ok_or("No current Tenjiku execution.")?;
        if !response.matches(pending)
            || !matches!(
                response.result,
                ExecutionResult::Failure { .. } | ExecutionResult::Tenjiku { .. }
            )
        {
            return Err("Stale or unsupported Tenjiku observation.".into());
        }
        let analysis = matches!(pending.purpose, ExecutionPurpose::MoveAnalysis { .. });
        let failed = matches!(response.result, ExecutionResult::Failure { .. });
        let incomplete = matches!(
            &response.result,
            ExecutionResult::Tenjiku {
                result: mshogi_engine::TenjikuResult::Incomplete { .. }
            }
        );
        // One worker-owned candidate validates the observation, native continuation,
        // engine action and all persistence headroom before the live state changes.
        state.document = Arc::new(state.document.with_observation(response.clone(), stopped)?);
        state.game = state
            .document
            .record()
            .to_game_with_stop(stopped)
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
    let selected = state.selected();
    let presentation = TenjikuPresentation::new(selected, &state.game);
    let actions = selected
        .legal_actions_with_stop(stopped)
        .map_err(|e| e.to_string())?;
    state.status = presentation::status(selected, !actions.is_empty());
    state.choices = actions
        .iter()
        .filter(|_| state.viewed.is_none())
        .map(|a| presentation::choice(a.clone(), &actions))
        .collect();
    state.presentation = presentation;
    persistence::check(stopped)?;
    Ok(archive::NativeCompleted::TenjikuRuntime {
        state,
        replacement: work.replacement,
        durable,
    })
}

impl DesktopController {
    pub(super) fn queue_tenjiku_candidate(
        &mut self,
        seed: RuntimeSeed,
        operation: Option<SessionOperation>,
        response: Option<ExecutionObservation>,
        replacement: bool,
    ) -> Result<AdmissionReceipt, String> {
        let consumes_response = response.is_some();
        let floor = self.runtime_floor();
        self.queue_native_transaction(
            archive::NativeWork::TenjikuRuntime(Box::new(RuntimeWork {
                seed,
                operation,
                response,
                floor,
                replacement,
            })),
            consumes_response,
        )
    }
    pub(super) fn replace_tenjiku(
        &mut self,
        record: c::GameRecord,
        human: c::Side,
    ) -> Result<SharedSnapshot, String> {
        self.queue_tenjiku_candidate(
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
    pub(super) fn tenjiku_command(
        &mut self,
        operation: SessionOperation,
    ) -> Result<SharedSnapshot, String> {
        if matches!(operation, SessionOperation::Cancel {}) {
            self.retire_admission();
            self.bridge.cancel();
            self.session.tenjiku_mut().unwrap().retire();
            self.session.tenjiku_mut().unwrap().paused = true;
            self.session.version += 1;
            self.session.selection_id += 1;
            self.session.revision += 1;
        }
        let state = self.session.tenjiku().unwrap().capture();
        self.queue_tenjiku_candidate(
            RuntimeSeed::Current(Box::new(state)),
            Some(operation),
            None,
            false,
        )?;
        Ok(self.shared_snapshot())
    }
    pub(crate) fn queue_tenjiku_save(&mut self, path: PathBuf) -> Result<u64, String> {
        if self.admission_token.is_some() {
            return Err("Wait for native document admission before Save.".into());
        }
        let state = self.session.tenjiku().unwrap();
        let document = state.document.clone();
        let floor = self.runtime_floor();
        let revision = self.session.document_revision;
        let identity = self.session.document_identity;
        let id = self.next_native_io(IoPhase::Saving)?;
        self.files.submit(
            id,
            persistence::Work::Native(Box::new(archive::NativeWork::TenjikuSave {
                document,
                floor,
                path,
                revision,
                identity,
            })),
        );
        Ok(id)
    }
    pub(super) fn accept_tenjiku_observation(&mut self, response: ExecutionObservation) -> bool {
        let state = self.session.tenjiku().unwrap();
        let Some(pending) = &state.pending else {
            return false;
        };
        let selected = CanonicalRecord::tenjiku(
            c::GameRecord::from_game(state.selected()).expect("admitted record"),
        );
        if !response.matches(pending)
            || pending.session_version != self.session.version
            || pending.game != selected
            || !matches!(
                response.result,
                ExecutionResult::Failure { .. } | ExecutionResult::Tenjiku { .. }
            )
        {
            return false;
        }
        let seed = RuntimeSeed::Current(Box::new(state.capture()));
        if self
            .queue_tenjiku_candidate(seed, None, Some(response), false)
            .is_err()
        {
            // The bridge already consumed this exact response. Even enqueue
            // headroom refusal must not leave Thinking with no deliverable work.
            self.bridge.cancel();
            let capacity = self.session.capacity.resource.is_some();
            let state = self.session.tenjiku_mut().unwrap();
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
    pub(super) fn start_tenjiku(
        &mut self,
        limits: WorkLimits,
        analysis: bool,
    ) -> Result<(), OperationFailure> {
        validate_limits(limits)?;
        self.ensure_runtime_capacity()?;
        let state = self.session.tenjiku().unwrap();
        let count = state.document.observations().len();
        if count >= MAX_ARCHIVE_OBSERVATIONS {
            return Err(OperationFailure::resource(ResourceFailure::new(
                ResourceFact::measured(
                    ResourceCategory::OrdinaryObservations,
                    count as u64,
                    MAX_ARCHIVE_OBSERVATIONS as u64,
                )
                .with_scope(ResourceScope::Tenjiku)
                .with_unit(ResourceUnit::Observations),
                "Application limit reached: all 256 observation slots are occupied. The complete current game can still be saved.",
            )));
        }
        let game = CanonicalRecord::tenjiku(
            c::GameRecord::from_game(state.selected()).map_err(|e| e.to_string())?,
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
                            "Tenjiku event {}{}",
                            game.event_count(),
                            if historical { " (history)" } else { " (live)" }
                        ),
                        turn: NativeSide::Tenjiku(turn),
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
        let state = self.session.tenjiku_mut().unwrap();
        state.pending = Some(context);
        state.submitted = false;
        state.phase = EnginePhase::Thinking;
        Ok(())
    }
    pub(super) fn poll_tenjiku(&mut self) -> SharedSnapshot {
        if self.admission_token.is_none()
            && let Some(response) = self.bridge.poll()
        {
            self.accept_shared(response);
        }
        if self.admission_token.is_none()
            && !self.lifecycle.close_requested()
            && self.session.capacity.status != CapacityPhase::Paused
        {
            let state = self.session.tenjiku_mut().unwrap();
            let analysis = state.requested_analysis.take();
            let play = state.pending.is_none()
                && !state.paused
                && state.viewed.is_none()
                && state.game.conclusion().is_none()
                && state.game.position().side_to_move() != state.document.human();
            if let Some(limits) = analysis {
                if let Err(e) = self.start_tenjiku(limits, true) {
                    self.pause_native_failure(e);
                }
            } else if play && let Err(e) = self.start_tenjiku(play_limits(), false) {
                self.pause_native_failure(e);
            }
            if self.shared.auto_submit()
                && let Some(context) = self.session.tenjiku().unwrap().pending.clone()
                && !self.session.tenjiku().unwrap().submitted
            {
                self.session.tenjiku_mut().unwrap().submitted = true;
                if let Err(e) = self.bridge.submit(execution_request(context.clone())) {
                    self.accept_tenjiku_observation(ExecutionObservation::new(
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
        self.tenjiku_snapshot()
    }
    /// Verification entry into the same real native supervised request path.
    #[cfg(any(test, feature = "desktop-verification"))]
    pub fn verify_tenjiku_execution(
        &mut self,
        limits: WorkLimits,
        analysis: bool,
    ) -> Result<ExecutionContext, String> {
        if self.admission_token.is_some() || self.lifecycle.close_requested() {
            return Err("Wait for native operation/close.".into());
        }
        self.start_tenjiku(limits, analysis)
            .map_err(|e| e.diagnostic)?;
        let context = self.session.tenjiku().unwrap().pending.clone().unwrap();
        self.bridge
            .submit(execution_request(context.clone()))
            .map_err(|e| format!("Cannot submit native work: {e:?}"))?;
        self.session.tenjiku_mut().unwrap().submitted = true;
        Ok(context)
    }
    pub(super) fn close_tenjiku(&mut self) {
        if self.session.tenjiku().is_some_and(|s| {
            s.paused
                && s.phase == EnginePhase::Paused
                && s.pending.is_none()
                && s.browse_stop.is_none()
                && matches!(s.document.controls().last(), Some(TenjikuControl::Close {}))
        }) {
            return;
        }
        self.bridge.cancel();
        let state = self.session.tenjiku_mut().unwrap();
        state.retire();
        state.paused = true;
        // Prepared on the admission/Open worker; O(1) swap on native close.
        state.document = state.close_document.clone();
        self.shared_changed();
    }
}

#[path = "shared_tenjiku_presentation.rs"]
mod presentation;
pub use presentation::{TenjikuChoiceShape, TenjikuPresentation};
pub(super) fn inspect(game: &c::Game, origin: &str) -> Result<inspection::Inspected, String> {
    presentation::inspect(game, origin)
}
fn play_limits() -> WorkLimits {
    mshogi_engine::TENJIKU_PLAY_LIMITS
}
fn validate_limits(limits: WorkLimits) -> Result<(), OperationFailure> {
    if limits.depth == 0
        || limits.depth > mshogi_engine::TENJIKU_MAX_DEPTH
        || limits.nodes == 0
        || limits.nodes > mshogi_engine::TENJIKU_MAX_NODES
    {
        return Err(
            "Unsupported Tenjiku work limits. Use a positive bounded native depth and visit limit."
                .into(),
        );
    }
    Ok(())
}
fn display_observation(
    index: usize,
    o: &ExecutionObservation,
    state: &TenjikuSession,
) -> PresentedSearchObservation {
    let CanonicalRecord::Tenjiku(record) = &o.context.game else {
        unreachable!("validated native Tenjiku context")
    };
    let telemetry = match &o.result {
        ExecutionResult::Tenjiku { result } => Some(result.telemetry()),
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
        perspective: side(
            record.profile().into(),
            NativeSide::Tenjiku(record.side_to_move()),
        )
        .label,
        engine: "mShogi",
        score: telemetry.and_then(|t| t.score_label()),
        completed_depth: telemetry.map(|t| t.completed_depth),
        nodes: telemetry.map(|t| t.nodes),
        // Native complete route strings retain every landing and promotion; exact typed PV remains in Technical Details.
        variation: telemetry.map_or_else(Vec::new, |t| {
            t.principal_variation
                .iter()
                .map(|a| {
                    if a.kind == c::ActionKind::Igui {
                        return format!(
                            "{}!x{} (stationary capture)",
                            a.from,
                            a.target.expect("validated native igui target")
                        );
                    }
                    format!(
                        "{}→{}{}",
                        a.from,
                        a.route
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join("→"),
                        if a.promote { " (promote)" } else { "" }
                    )
                })
                .collect()
        }),
        status: match telemetry.map(|t| t.status) {
            Some(mshogi_engine::StopReason::Complete) => "Complete",
            Some(mshogi_engine::StopReason::NodeLimit) => "Node limit reached",
            Some(mshogi_engine::StopReason::Stopped) => "Search stopped",
            None => match &o.result {
                ExecutionResult::Failure { failure }
                    if failure.kind == mshogi_engine::FailureKind::UnsupportedMode =>
                {
                    "Unsupported execution · unscored"
                }
                ExecutionResult::Failure { failure }
                    if failure.kind == mshogi_engine::FailureKind::Search =>
                {
                    "Search could not complete · unscored"
                }
                _ => "Engine failure · unscored",
            },
        },
        historical: !state.current_observations.contains(&o.context.request_id),
        selected: state.analysis.as_ref() == Some(o),
    }
}
