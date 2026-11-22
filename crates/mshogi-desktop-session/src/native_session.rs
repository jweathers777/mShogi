//! Generic native session in the sole desktop runtime (MSH-217). Rules and
//! persistence stay native-owned; each game supplies its presentation, limits
//! and the wrappers of the shared runtime enums through [`SessionGame`].
use super::*;
use crate::game_document::{ControlOf, GameControl, RewindKind, SettingsOf, ValidatedGameDocument};
use crate::native_game::NativeGame;
use std::sync::Arc;

/// Kind of a native command, as the session needs it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CommandKind {
    Play,
    Resign,
    Stop,
    Other,
}

/// A game-specific session operation.
pub(crate) enum GameOperation<G: NativeGame> {
    /// A complete native command of an actor.
    Command { actor: G::Side, command: G::Command },
    /// Orientation preferences; the play mode stays the document's.
    Preferences { settings: SettingsOf<G> },
}

/// Engine telemetry as the observation display needs it.
pub(crate) struct TelemetryView {
    pub(crate) score: Option<String>,
    pub(crate) completed_depth: u16,
    pub(crate) nodes: u64,
    pub(crate) variation: Vec<String>,
    pub(crate) status: mshogi_engine::StopReason,
}

/// The session-side differences of a native game.
pub(crate) trait SessionGame: NativeGame {
    /// Frontend presentation of the selected and live games.
    type Presentation: Clone;
    const PLAY_LIMITS: WorkLimits;
    const MAX_DEPTH: u16;
    const MAX_NODES: u64;

    fn presentation(selected: &Self::Game, live: &Self::Game) -> Self::Presentation;
    fn status(selected: &Self::Game, has_actions: bool) -> PresentedStatus;
    /// A legal action of `game` as a frontend choice.
    fn choice(game: &Self::Game, action: Self::Action, all: &[Self::Action]) -> PresentedChoice;
    fn legal_actions(
        game: &Self::Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Vec<Self::Action>, String>;
    /// Retire the native runtime context; a restored record never recovers it.
    fn retire_context(game: &mut Self::Game);
    /// Actor and command kind of event `index`.
    fn event(game: &Self::Game, index: usize) -> (Self::Side, CommandKind);
    fn command_kind(command: &Self::Command) -> CommandKind;
    fn native_side(side: Self::Side) -> NativeSide;
    /// Telemetry and incompleteness of this game's engine result.
    fn telemetry(result: &Self::EngineResult) -> TelemetryView;
    fn incomplete(result: &Self::EngineResult) -> bool;

    /// This game's operation, or the operation back if it is not one.
    fn operation(operation: SessionOperation) -> Result<GameOperation<Self>, SessionOperation>;
    fn command_operation(actor: Self::Side, command: Self::Command) -> SessionOperation;
    fn runtime_work(work: RuntimeWork<Self>) -> archive::NativeWork;
    fn save_work(
        document: Arc<ValidatedGameDocument<Self>>,
        floor: u64,
        path: PathBuf,
        revision: u64,
        identity: u64,
    ) -> archive::NativeWork;
    fn completed(
        state: Box<NativeSession<Self>>,
        replacement: bool,
        durable: bool,
    ) -> archive::NativeCompleted;
    /// A restored session as an opened document.
    fn opened(state: Box<NativeSession<Self>>) -> archive::OpenedNative;
    fn state(session: &ActiveDocument) -> Option<&NativeSession<Self>>;
    fn state_mut(session: &mut ActiveDocument) -> Option<&mut NativeSession<Self>>;

    /// The game has pieces in hand and drops.
    const HANDS: bool = false;
    /// Pieces in hand, for a game with hands.
    fn hands(_game: &Self::Game) -> Vec<PresentedHand> {
        vec![]
    }
    /// Board width and height in squares.
    const BOARD_SIZE: u8;
    const BOARD_LABEL: &'static str;
    const INITIAL_FOCUS: &'static str;
    const SCORE_MEANING: &'static str;
    fn edition(game: &Self::Game) -> &'static str;
    fn pieces(game: &Self::Game) -> Vec<PresentedPiece>;
    fn presented_events(game: &Self::Game) -> Vec<PresentedEvent>;
    fn board_ply(game: &Self::Game) -> u64;
    /// Put this game's presentation into its snapshot field.
    fn present_in(snapshot: &mut SharedSnapshot, presentation: Self::Presentation);
}

fn record_text(e: crate::native_game::RecordFailure) -> String {
    match e {
        crate::native_game::RecordFailure::Stopped => {
            "Native operation stopped before acceptance.".into()
        }
        crate::native_game::RecordFailure::Capacity(d)
        | crate::native_game::RecordFailure::Invalid(d) => d,
    }
}

pub(crate) struct NativeSession<G: SessionGame> {
    pub(crate) document: Arc<ValidatedGameDocument<G>>,
    close_document: Arc<ValidatedGameDocument<G>>,
    pub(crate) game: G::Game,
    viewed: Option<G::Game>,
    pub(crate) browse_stop: Option<BrowseStop>,
    pub(crate) pending: Option<ExecutionContext>,
    pub(crate) phase: EnginePhase,
    pub(crate) paused: bool,
    submitted: bool,
    requested_analysis: Option<WorkLimits>,
    pub(crate) analysis: Option<ExecutionObservation>,
    current_observations: Vec<u64>,
    pub(crate) diagnostic: Option<String>,
    presentation: G::Presentation,
    choices: Vec<PresentedChoice>,
    status: PresentedStatus,
}
impl<G: SessionGame> NativeSession<G> {
    pub(crate) fn restore(
        document: ValidatedGameDocument<G>,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, OperationFailure> {
        persistence::check(stopped)?;
        let game = G::to_game(document.record(), stopped).map_err(record_text)?;
        persistence::check(stopped)?;
        let close_document =
            Arc::new(document.with_reserved_close(document.retired_execution_floor())?);
        let viewed = if let Some(index) = document.selected_event() {
            let prefix = G::record_prefix(&game, index, stopped).map_err(record_text)?;
            persistence::check(stopped)?;
            Some(G::to_game(&prefix, stopped).map_err(record_text)?)
        } else {
            None
        };
        persistence::check(stopped)?;
        let (presentation, status, choices) = present::<G>(&game, viewed.as_ref(), stopped)?;
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
    pub(crate) fn selected(&self) -> &G::Game {
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
        G::retire_context(&mut self.game);
    }
    fn local_actor(&self) -> G::Side {
        self.document
            .settings()
            .play_mode
            .actor(self.document.human(), G::side_to_move(&self.game))
    }
    fn take_back_target(&self) -> Option<usize> {
        let human = self.document.human();
        let events = G::events(&self.game);
        if self.document.settings().play_mode.is_two_humans() {
            return events.checked_sub(1);
        }
        if let Some(last) = events.checked_sub(1)
            && G::event(&self.game, last) == (human, CommandKind::Resign)
            && G::side_to_move(&self.game) == human
        {
            return Some(last);
        }
        (0..events)
            .rev()
            .find(|&i| G::event(&self.game, i) == (human, CommandKind::Play))
    }
    fn apply(
        &mut self,
        operation: SessionOperation,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<bool, OperationFailure> {
        persistence::check(stopped)?;
        let control: Option<ControlOf<G>> = match G::operation(operation) {
            Ok(GameOperation::Command { actor, command }) => {
                let kind = G::command_kind(&command);
                if self.document.settings().play_mode.is_two_humans()
                    && self.paused
                    && kind == CommandKind::Play
                {
                    return Err("Resume play before moving.".into());
                }
                if self.viewed.is_some() {
                    return Err("Return to live before changing the game.".into());
                }
                // Actor/turn/terminal/draw admission belongs to the native Game.
                // The immutable document candidate admits the complete command below.
                if matches!(kind, CommandKind::Resign | CommandKind::Stop) {
                    self.paused = true;
                }
                Some(GameControl::Native { actor, command })
            }
            Ok(GameOperation::Preferences { mut settings }) => {
                settings.play_mode = self.document.settings().play_mode;
                Some(GameControl::Settings { settings })
            }
            Err(SessionOperation::Resign {}) => {
                return self.apply(
                    G::command_operation(self.local_actor(), G::resign()),
                    stopped,
                );
            }
            Err(operation @ (SessionOperation::Undo {} | SessionOperation::TakeBack {})) => {
                if self.viewed.is_some() {
                    return Err("Return to live before changing history.".into());
                }
                let (event_index, reason) = if matches!(operation, SessionOperation::Undo {}) {
                    (
                        G::events(&self.game)
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
                Some(GameControl::Rewind {
                    event_index,
                    reason,
                })
            }
            Err(SessionOperation::Inspect { event_index }) => {
                let record =
                    G::record_prefix(&self.game, event_index, stopped).map_err(record_text)?;
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
                self.viewed = Some(G::to_game(&record, stopped).map_err(record_text)?);
                self.retire();
                self.browse_stop = stop;
                self.paused = true;
                self.diagnostic = Some(
                    "Historical position. Analysis is read-only. Return to live explicitly.".into(),
                );
                return Ok(false);
            }
            Err(SessionOperation::ReturnLive {}) => {
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
                    && !G::concluded(&self.game)
                    && !G::stop_recorded(&self.game)
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
            Err(SessionOperation::Cancel {}) => {
                self.retire();
                self.paused = true;
                self.phase = EnginePhase::Cancelled;
                self.diagnostic = Some(
                    "Work cancelled. The game is unchanged; Resume or start another analysis."
                        .into(),
                );
                return Ok(false);
            }
            Err(SessionOperation::Analyze { operation }) => {
                let AnalysisOperation::Moves { depth, nodes } = operation else {
                    return Err(format!(
                        "Unsupported {} analysis operation; no Chess Full or Xiangqi exact operation is available.",
                        G::NAME
                    )
                    .into());
                };
                let limits = WorkLimits {
                    time_ms: None,
                    depth,
                    nodes,
                };
                validate_limits::<G>(limits)?;
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
            Err(SessionOperation::Resume {} | SessionOperation::Retry {}) => {
                if self.viewed.is_some() {
                    return Err("Return to live before resuming play.".into());
                }
                self.retire();
                self.paused = false;
                self.diagnostic = None;
                return Ok(false);
            }
            Err(_) => {
                return Err(
                    format!("Unsupported operation for the native {} profile.", G::NAME).into(),
                );
            }
        };
        if let Some(control) = control {
            let rewind = matches!(control, GameControl::Rewind { .. });
            let document = self.document.with_control(control, stopped)?;
            self.game = G::to_game(document.record(), stopped).map_err(record_text)?;
            if rewind {
                self.current_observations.clear();
            }
            self.viewed = if let Some(index) = document.selected_event() {
                let prefix = G::record_prefix(&self.game, index, stopped).map_err(record_text)?;
                Some(G::to_game(&prefix, stopped).map_err(record_text)?)
            } else {
                None
            };
            self.document = Arc::new(document);
            self.retire();
        }
        Ok(true)
    }
    fn capture(&self) -> Draft<G> {
        Draft {
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

/// Presentation, status and choices of the selected game; choices only live.
#[allow(clippy::type_complexity)]
fn present<G: SessionGame>(
    game: &G::Game,
    viewed: Option<&G::Game>,
    stopped: &mut impl FnMut() -> bool,
) -> Result<(G::Presentation, PresentedStatus, Vec<PresentedChoice>), OperationFailure> {
    let selected = viewed.unwrap_or(game);
    let presentation = G::presentation(selected, game);
    let actions = G::legal_actions(selected, stopped)?;
    let status = G::status(selected, !actions.is_empty());
    let choices = actions
        .iter()
        .filter(|_| viewed.is_none())
        .map(|a| G::choice(selected, a.clone(), &actions))
        .collect();
    Ok((presentation, status, choices))
}

pub(crate) struct Draft<G: SessionGame> {
    document: Arc<ValidatedGameDocument<G>>,
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
pub(crate) enum RuntimeSeed<G: SessionGame> {
    Current(Box<Draft<G>>),
    Record {
        record: Box<G::Record>,
        human: G::Side,
        play_mode: PlayMode,
    },
}
pub(crate) struct RuntimeWork<G: SessionGame> {
    pub(crate) seed: RuntimeSeed<G>,
    pub(crate) operation: Option<SessionOperation>,
    pub(crate) response: Option<ExecutionObservation>,
    pub(crate) floor: u64,
    pub(crate) replacement: bool,
}
pub(crate) fn execute<G: SessionGame>(
    work: RuntimeWork<G>,
    stopped: &mut impl FnMut() -> bool,
) -> Result<archive::NativeCompleted, OperationFailure> {
    persistence::check(stopped)?;
    let mut state = match work.seed {
        RuntimeSeed::Current(draft) => {
            let mut state = Box::new(NativeSession::restore((*draft.document).clone(), stopped)?);
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
        RuntimeSeed::Record {
            record,
            human,
            play_mode,
        } => Box::new(NativeSession::restore(
            ValidatedGameDocument::new_with_mode(*record, human, play_mode, stopped)?,
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
            .ok_or_else(|| format!("No current {} execution.", G::NAME))?;
        let result = G::engine_result(&response.result);
        let failed = matches!(response.result, ExecutionResult::Failure { .. });
        if !response.matches(pending) || !(failed || result.is_some()) {
            return Err(format!("Stale or unsupported {} observation.", G::NAME).into());
        }
        let analysis = matches!(pending.purpose, ExecutionPurpose::MoveAnalysis { .. });
        let incomplete = result.is_some_and(G::incomplete);
        // One worker-owned candidate validates the observation, native continuation,
        // engine action and all persistence headroom before the live state changes.
        state.document = Arc::new(state.document.with_observation(response.clone(), stopped)?);
        state.game = G::to_game(state.document.record(), stopped).map_err(record_text)?;
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
    let (presentation, status, choices) =
        present::<G>(&state.game, state.viewed.as_ref(), stopped)?;
    state.presentation = presentation;
    state.status = status;
    state.choices = choices;
    persistence::check(stopped)?;
    Ok(G::completed(state, work.replacement, durable))
}

/// Open on the file worker: decode, validate and restore a saved document.
pub(crate) fn open<G: SessionGame>(
    bytes: &[u8],
    stopped: &mut impl FnMut() -> bool,
) -> Result<archive::OpenedNative, OperationFailure> {
    let document = ValidatedGameDocument::<G>::decode(bytes, stopped)?;
    Ok(G::opened(Box::new(NativeSession::restore(
        document, stopped,
    )?)))
}

/// Save on the file worker: refresh the runtime floor, validate the complete
/// candidate again and write it atomically.
pub(crate) fn save<G: NativeGame>(
    document: &ValidatedGameDocument<G>,
    floor: u64,
    path: &std::path::Path,
    revision: u64,
    identity: u64,
    stopped: &mut impl FnMut() -> bool,
) -> Result<archive::NativeCompleted, OperationFailure> {
    let doc = document.with_runtime_floor(floor)?;
    let doc = ValidatedGameDocument::<G>::decode(doc.bytes(), stopped)?;
    persistence::atomic_write(path, doc.bytes(), stopped)?;
    Ok(archive::NativeCompleted::Saved {
        path: path.to_path_buf(),
        revision,
        identity,
    })
}

impl DesktopController {
    pub(super) fn queue_game_candidate<G: SessionGame>(
        &mut self,
        seed: RuntimeSeed<G>,
        operation: Option<SessionOperation>,
        response: Option<ExecutionObservation>,
        replacement: bool,
    ) -> Result<AdmissionReceipt, String> {
        let consumes_response = response.is_some();
        let floor = self.runtime_floor();
        self.queue_native_transaction(
            G::runtime_work(RuntimeWork {
                seed,
                operation,
                response,
                floor,
                replacement,
            }),
            consumes_response,
        )
    }
    pub(super) fn replace_game<G: SessionGame>(
        &mut self,
        record: G::Record,
        human: G::Side,
    ) -> Result<SharedSnapshot, String> {
        self.queue_game_candidate::<G>(
            RuntimeSeed::Record {
                record: Box::new(record),
                human,
                play_mode: PlayMode::HumanEngine,
            },
            None,
            None,
            true,
        )?;
        Ok(self.shared_snapshot())
    }
    pub(super) fn game_command<G: SessionGame>(
        &mut self,
        operation: SessionOperation,
    ) -> Result<SharedSnapshot, String> {
        if matches!(operation, SessionOperation::Cancel {}) {
            self.retire_admission();
            self.bridge.cancel();
            let state = G::state_mut(&mut self.session).expect("native owner");
            state.retire();
            state.paused = true;
            self.session.version += 1;
            self.session.selection_id += 1;
            self.session.revision += 1;
        }
        let state = G::state(&self.session).expect("native owner").capture();
        self.queue_game_candidate::<G>(
            RuntimeSeed::Current(Box::new(state)),
            Some(operation),
            None,
            false,
        )?;
        Ok(self.shared_snapshot())
    }
    pub(crate) fn queue_game_save<G: SessionGame>(&mut self, path: PathBuf) -> Result<u64, String> {
        if self.admission_token.is_some() {
            return Err("Wait for native document admission before Save.".into());
        }
        let document = G::state(&self.session)
            .expect("native owner")
            .document
            .clone();
        let floor = self.runtime_floor();
        let revision = self.session.document_revision;
        let identity = self.session.document_identity;
        let id = self.next_native_io(IoPhase::Saving)?;
        self.files.submit(
            id,
            persistence::Work::Native(Box::new(G::save_work(
                document, floor, path, revision, identity,
            ))),
        );
        Ok(id)
    }
    pub(super) fn accept_game_observation<G: SessionGame>(
        &mut self,
        response: ExecutionObservation,
    ) -> bool {
        let state = G::state(&self.session).expect("native owner");
        let Some(pending) = &state.pending else {
            return false;
        };
        let selected = G::canonical(G::from_game(state.selected()).expect("admitted record"));
        if !response.matches(pending)
            || pending.session_version != self.session.version
            || pending.game != selected
            || !(matches!(response.result, ExecutionResult::Failure { .. })
                || G::engine_result(&response.result).is_some())
        {
            return false;
        }
        let seed = RuntimeSeed::Current(Box::new(state.capture()));
        if self
            .queue_game_candidate::<G>(seed, None, Some(response), false)
            .is_err()
        {
            // The bridge already consumed this exact response. Even enqueue
            // headroom refusal must not leave Thinking with no deliverable work.
            self.bridge.cancel();
            let capacity = self.session.capacity.resource.is_some();
            let state = G::state_mut(&mut self.session).expect("native owner");
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
    pub(super) fn start_game_work<G: SessionGame>(
        &mut self,
        limits: WorkLimits,
        analysis: bool,
    ) -> Result<(), OperationFailure> {
        validate_limits::<G>(limits)?;
        self.ensure_runtime_capacity()?;
        let state = G::state(&self.session).expect("native owner");
        let count = state.document.observations().len();
        if count >= MAX_ARCHIVE_OBSERVATIONS {
            return Err(OperationFailure::resource(ResourceFailure::new(
                ResourceFact::measured(
                    ResourceCategory::OrdinaryObservations,
                    count as u64,
                    MAX_ARCHIVE_OBSERVATIONS as u64,
                )
                .with_scope(G::SCOPE)
                .with_unit(ResourceUnit::Observations),
                "Application limit reached: all 256 observation slots are occupied. The complete current game can still be saved.",
            )));
        }
        let game = G::canonical(G::from_game(state.selected()).map_err(record_text)?);
        let turn = G::side_to_move(state.selected());
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
                            "{} event {}{}",
                            G::NAME,
                            game.event_count(),
                            if historical { " (history)" } else { " (live)" }
                        ),
                        turn: G::native_side(turn),
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
        let state = G::state_mut(&mut self.session).expect("native owner");
        state.pending = Some(context);
        state.submitted = false;
        state.phase = EnginePhase::Thinking;
        Ok(())
    }
    pub(super) fn poll_game<G: SessionGame>(&mut self) -> SharedSnapshot {
        if self.admission_token.is_none()
            && let Some(response) = self.bridge.poll()
        {
            self.accept_shared(response);
        }
        if self.admission_token.is_none()
            && !self.lifecycle.close_requested()
            && self.session.capacity.status != CapacityPhase::Paused
        {
            let state = G::state_mut(&mut self.session).expect("native owner");
            let analysis = state.requested_analysis.take();
            let play = state.pending.is_none()
                && !state.paused
                && state.viewed.is_none()
                && !G::concluded(&state.game)
                && state
                    .document
                    .settings()
                    .play_mode
                    .engine_turn(state.document.human(), G::side_to_move(&state.game));
            if let Some(limits) = analysis {
                if let Err(e) = self.start_game_work::<G>(limits, true) {
                    self.pause_native_failure(e);
                }
            } else if play && let Err(e) = self.start_game_work::<G>(G::PLAY_LIMITS, false) {
                self.pause_native_failure(e);
            }
            let state = G::state(&self.session).expect("native owner");
            if self.shared.auto_submit()
                && let Some(context) = state.pending.clone()
                && !state.submitted
            {
                G::state_mut(&mut self.session)
                    .expect("native owner")
                    .submitted = true;
                if let Err(e) = self.bridge.submit(execution_request(context.clone())) {
                    self.accept_game_observation::<G>(ExecutionObservation::new(
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
        self.game_snapshot::<G>()
    }
    /// Verification entry into the same real native supervised request path.
    #[cfg(any(test, feature = "desktop-verification"))]
    pub(crate) fn verify_game_execution<G: SessionGame>(
        &mut self,
        limits: WorkLimits,
        analysis: bool,
    ) -> Result<ExecutionContext, String> {
        if self.admission_token.is_some() || self.lifecycle.close_requested() {
            return Err("Wait for native operation/close.".into());
        }
        self.start_game_work::<G>(limits, analysis)
            .map_err(|e| e.diagnostic)?;
        let context = G::state(&self.session)
            .expect("native owner")
            .pending
            .clone()
            .expect("started work");
        self.bridge
            .submit(execution_request(context.clone()))
            .map_err(|e| format!("Cannot submit native work: {e:?}"))?;
        G::state_mut(&mut self.session)
            .expect("native owner")
            .submitted = true;
        Ok(context)
    }
    /// Hold technical fixture play; excluded from ordinary distributables.
    #[cfg(feature = "desktop-verification")]
    pub(crate) fn verify_game_pause_play<G: SessionGame>(&mut self) -> Result<(), String> {
        self.bridge.cancel();
        let state = G::state_mut(&mut self.session)
            .ok_or_else(|| format!("Requires native {} document.", G::NAME))?;
        state.retire();
        state.paused = true;
        Ok(())
    }
    /// Project the actual submitted pending state without polling completion.
    #[cfg(feature = "desktop-verification")]
    pub(crate) fn verify_game_pending_snapshot<G: SessionGame>(
        &self,
    ) -> Result<SharedSnapshot, String> {
        if G::state(&self.session).is_none_or(|s| s.pending.is_none()) {
            return Err(format!(
                "Requires actual pending {} verification work.",
                G::NAME
            ));
        }
        Ok(self.game_snapshot::<G>())
    }
    /// Verification-only elapsed-clock injection; canonical state is unchanged.
    #[cfg(feature = "desktop-verification")]
    pub(crate) fn verify_game_validation_deadline<G: SessionGame>(
        &mut self,
        file: bool,
    ) -> Result<SharedSnapshot, String> {
        if G::state(&self.session).is_none() || self.admission_token.is_some() {
            return Err(format!("Requires an idle native {} document.", G::NAME));
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
        Ok(self.game_snapshot::<G>())
    }
    pub(super) fn close_game<G: SessionGame>(&mut self) {
        if G::state(&self.session).is_some_and(|s| {
            s.paused
                && s.phase == EnginePhase::Paused
                && s.pending.is_none()
                && s.browse_stop.is_none()
                && matches!(s.document.controls().last(), Some(GameControl::Close {}))
        }) {
            return;
        }
        self.bridge.cancel();
        let state = G::state_mut(&mut self.session).expect("native owner");
        state.retire();
        state.paused = true;
        // Prepared on the admission/Open worker; O(1) swap on native close.
        state.document = state.close_document.clone();
        self.shared_changed();
    }
}

impl DesktopController {
    pub(crate) fn game_snapshot<G: SessionGame>(&self) -> SharedSnapshot {
        let state = G::state(&self.session).expect("native owner");
        let game = state.selected();
        let profile = G::game_profile(game);
        let turn = G::side_to_move(game);
        let sides: Vec<_> = G::SIDES.map(|s| side(profile, G::native_side(s))).into();
        let files: Vec<_> = (1..=G::BOARD_SIZE).rev().map(|n| n.to_string()).collect();
        let ranks: Vec<_> = (b'a'..b'a' + G::BOARD_SIZE)
            .map(|n| char::from(n).to_string())
            .collect();
        let historical = state.viewed.is_some();
        let ended = G::concluded(game);
        let settings = state.document.settings();
        let human = state.document.human();
        let mut snapshot = SharedSnapshot {
            moves: self.present_moves(G::events(game)),
            xiangqi: None,
            tori: None,
            chu: None,
            tenjiku: None,
            open_token: self.shared_open_token(),
            document: self.shared_document(),
            io: self.io_state.clone(),
            capacity: self.session.capacity.clone(),
            token: self.shared_token(),
            game: G::KIND,
            profile,
            edition: G::edition(game),
            play_mode: settings.play_mode,
            local_actor: side(
                profile,
                G::native_side(settings.play_mode.actor(human, turn)),
            ),
            human: side(profile, G::native_side(human)),
            turn: side(profile, G::native_side(turn)),
            sides: sides.clone(),
            board: BoardDescriptor {
                intersections: None,
                files: files.clone(),
                ranks: ranks.clone(),
                label: G::BOARD_LABEL.into(),
                initial_focus: G::INITIAL_FOCUS.into(),
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
            pieces: G::pieces(game),
            hands: G::hands(game),
            choices: state.choices.clone(),
            capabilities: CommandCapabilities {
                board_action: !historical
                    && (!settings.play_mode.is_two_humans() || !state.paused)
                    && !ended
                    && settings.play_mode.owns(human, turn),
                drops: G::HANDS,
                promotion: true,
                take_back: !historical && state.take_back_target().is_some(),
                undo: !historical && G::events(&state.game) > 0,
                engine_turn: !settings.play_mode.is_two_humans(),
                move_analysis: true,
                chess_full: false,
                claims: false,
                declaration: false,
                resign: !historical && !ended,
                hands: G::HANDS,
                history: true,
                save: true,
            },
            status: state.status.clone(),
            events: G::presented_events(&state.game),
            event_index: G::events(game),
            board_ply: G::board_ply(game),
            play_limits: G::PLAY_LIMITS,
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
            context: G::canonical(G::from_game(game).expect("admitted native record")),
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
                .map(|(i, o)| display_observation(i, o, state))
                .collect(),
            score_meaning: G::SCORE_MEANING,
            chess: None,
            chess_match: None,
        };
        G::present_in(&mut snapshot, state.presentation.clone());
        snapshot
    }
}

fn validate_limits<G: SessionGame>(limits: WorkLimits) -> Result<(), OperationFailure> {
    if limits.depth == 0
        || limits.depth > G::MAX_DEPTH
        || limits.nodes == 0
        || limits.nodes > G::MAX_NODES
    {
        return Err(format!(
            "Unsupported {} work limits. Use a positive bounded native depth and visit limit.",
            G::NAME
        )
        .into());
    }
    Ok(())
}

pub(crate) fn display_observation<G: SessionGame>(
    index: usize,
    o: &ExecutionObservation,
    state: &NativeSession<G>,
) -> PresentedSearchObservation {
    let record = G::native(&o.context.game).expect("validated native context");
    let telemetry = G::engine_result(&o.result).map(G::telemetry);
    PresentedSearchObservation {
        selective_depth: None,
        observation_index: index,
        purpose: if matches!(o.context.purpose, ExecutionPurpose::EngineTurn {}) {
            "engine_turn"
        } else {
            "move_analysis"
        },
        event_index: G::event_count(record),
        board_ply: o.context.game.known_board_actions(),
        perspective: side(
            G::profile(record),
            G::native_side(G::record_side_to_move(record)),
        )
        .label,
        engine: "mShogi",
        score: telemetry.as_ref().and_then(|t| t.score.clone()),
        completed_depth: telemetry.as_ref().map(|t| t.completed_depth),
        nodes: telemetry.as_ref().map(|t| t.nodes),
        // Native complete route strings retain every landing and promotion; exact typed PV remains in Technical Details.
        variation: telemetry
            .as_ref()
            .map_or_else(Vec::new, |t| t.variation.clone()),
        status: match telemetry.as_ref().map(|t| t.status) {
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

/// The wrappers of the shared runtime enums for one game: its session
/// operations, worker jobs, active-document variant and snapshot field.
macro_rules! session_wiring {
    ($game:ident, $core:ident, operation: $op:ident, preferences: $prefs:ident, runtime: $runtime:ident,
     save: $save:ident, state: $state:ident, $state_mut:ident, field: $field:ident) => {
        fn native_side(side: $core::Side) -> NativeSide {
            NativeSide::$game(side)
        }
        fn operation(operation: SessionOperation) -> Result<GameOperation<Self>, SessionOperation> {
            match operation {
                SessionOperation::$op { actor, command } => {
                    Ok(GameOperation::Command { actor, command })
                }
                SessionOperation::$prefs { settings } => {
                    Ok(GameOperation::Preferences { settings })
                }
                operation => Err(operation),
            }
        }
        fn command_operation(actor: $core::Side, command: $core::GameCommand) -> SessionOperation {
            SessionOperation::$op { actor, command }
        }
        fn runtime_work(work: RuntimeWork<Self>) -> archive::NativeWork {
            archive::NativeWork::$runtime(Box::new(work))
        }
        fn save_work(
            document: Arc<ValidatedGameDocument<Self>>,
            floor: u64,
            path: PathBuf,
            revision: u64,
            identity: u64,
        ) -> archive::NativeWork {
            archive::NativeWork::$save {
                document,
                floor,
                path,
                revision,
                identity,
            }
        }
        fn completed(
            state: Box<NativeSession<Self>>,
            replacement: bool,
            durable: bool,
        ) -> archive::NativeCompleted {
            archive::NativeCompleted::$runtime {
                state,
                replacement,
                durable,
            }
        }
        fn opened(state: Box<NativeSession<Self>>) -> archive::OpenedNative {
            archive::OpenedNative::$game(state)
        }
        fn state(session: &ActiveDocument) -> Option<&NativeSession<Self>> {
            session.$state()
        }
        fn state_mut(session: &mut ActiveDocument) -> Option<&mut NativeSession<Self>> {
            session.$state_mut()
        }
        fn present_in(snapshot: &mut SharedSnapshot, presentation: Self::Presentation) {
            snapshot.$field = Some(presentation);
        }
    };
}

/// The Chu and Tenjiku sessions differ only by names and the PV label.
macro_rules! large_shogi_session {
    ($game:ident, $core:ident, $engine:ident, $limits:ident, $max_depth:ident, $max_nodes:ident,
     operation: $op:ident, preferences: $prefs:ident, runtime: $runtime:ident, save: $save:ident,
     state: $state:ident, $state_mut:ident, field: $field:ident,
     board: $size:expr, $label:expr, $focus:expr, score_meaning: $meaning:expr,
     pv: |$a:ident| $pv:expr) => {
        impl SessionGame for $game {
            type Presentation = presentation::Presentation;
            const PLAY_LIMITS: WorkLimits = mshogi_engine::$limits;
            const MAX_DEPTH: u16 = mshogi_engine::$max_depth;
            const MAX_NODES: u64 = mshogi_engine::$max_nodes;

            session_wiring!($game, c, operation: $op, preferences: $prefs, runtime: $runtime,
                save: $save, state: $state, $state_mut, field: $field);

            fn presentation(selected: &c::Game, live: &c::Game) -> Self::Presentation {
                presentation::Presentation::new(selected, live)
            }
            fn status(selected: &c::Game, has_actions: bool) -> PresentedStatus {
                presentation::status(selected, has_actions)
            }
            fn choice(_game: &c::Game, action: c::Action, all: &[c::Action]) -> PresentedChoice {
                presentation::choice(action, all)
            }
            fn legal_actions(
                game: &c::Game,
                stop: &mut impl FnMut() -> bool,
            ) -> Result<Vec<c::Action>, String> {
                game.legal_actions_with_stop(stop)
                    .map_err(|e| e.to_string())
            }
            fn retire_context(game: &mut c::Game) {
                game.retire_context()
                    .expect("bounded native runtime identity");
            }
            fn event(game: &c::Game, index: usize) -> (c::Side, CommandKind) {
                let event = &game.events()[index];
                (event.actor, Self::command_kind(&event.command))
            }
            fn command_kind(command: &c::GameCommand) -> CommandKind {
                match command {
                    c::GameCommand::Play { .. } => CommandKind::Play,
                    c::GameCommand::Resign => CommandKind::Resign,
                    c::GameCommand::Stop { .. } => CommandKind::Stop,
                    _ => CommandKind::Other,
                }
            }
            fn telemetry(result: &mshogi_engine::$engine) -> TelemetryView {
                let t = result.telemetry();
                TelemetryView {
                    score: t.score_label(),
                    completed_depth: t.completed_depth,
                    nodes: t.nodes,
                    variation: t.principal_variation.iter().map(|$a| $pv).collect(),
                    status: t.status,
                }
            }
            fn incomplete(result: &mshogi_engine::$engine) -> bool {
                matches!(result, mshogi_engine::$engine::Incomplete { .. })
            }
            const BOARD_SIZE: u8 = $size;
            const BOARD_LABEL: &'static str = $label;
            const INITIAL_FOCUS: &'static str = $focus;
            const SCORE_MEANING: &'static str = $meaning;
            fn edition(game: &c::Game) -> &'static str {
                presentation::edition(game)
            }
            fn pieces(game: &c::Game) -> Vec<PresentedPiece> {
                presentation::pieces(game)
            }
            fn presented_events(game: &c::Game) -> Vec<PresentedEvent> {
                presentation::events(game)
            }
            fn board_ply(game: &c::Game) -> u64 {
                game.snapshot().board_ply as u64
            }
        }
    };
}
