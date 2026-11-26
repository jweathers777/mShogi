//! Generic native session in the sole desktop runtime (MSH-217). Rules and
//! persistence stay native-owned; each game supplies its presentation, limits
//! and the wrappers of the shared runtime enums through [`SessionGame`].
use super::*;
use crate::game_document::{GameSettings, SettingsOf, ValidatedGameDocument};
use crate::native_game::{NativeGame, analysis_limits, validate_limits};
use mshogi_engine::{EngineGame, EngineResult};
use std::sync::Arc;

/// Kind of a native command, as the session needs it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CommandKind {
    Play,
    /// A human's final command: resignation or a formal declaration.
    Terminal,
    Other,
}

/// A change of the saved document.
enum Change<G: NativeGame> {
    Command { actor: G::Side, command: G::Command },
    Settings(SettingsOf<G>),
    Rewind(usize),
}

/// Engine telemetry as the observation display needs it.
pub(crate) struct TelemetryView {
    pub(crate) score: Option<String>,
    pub(crate) completed_depth: u16,
    pub(crate) nodes: u64,
    pub(crate) variation: Vec<String>,
    /// Display label of how the search ended.
    pub(crate) status: &'static str,
}

/// The session-side differences of a native game.
pub(crate) trait SessionGame: NativeGame {
    /// Frontend presentation of the selected and live games.
    type Presentation: Clone;
    const FINISH_CONCLUDED_ANALYSIS: bool = false;
    fn cancellation_phase(_analysis: bool) -> EnginePhase {
        EnginePhase::Cancelled
    }
    fn requires_running(_command: &Self::Command) -> bool {
        false
    }
    const UNDO: bool = true;
    /// The play command of the presented choice identifier `action`.
    fn play(action: &str) -> Result<Self::Command, String>;

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
    /// Telemetry and incompleteness of this game's engine result.
    fn telemetry(result: &Self::EngineResult) -> TelemetryView;
    fn incomplete(result: &Self::EngineResult) -> bool;

    /// A restored session as an opened document.
    fn opened(state: Box<NativeSession<Self>>) -> archive::OpenedNative;
    fn state(session: &ActiveDocument) -> Option<&NativeSession<Self>>;
    fn state_mut(session: &mut ActiveDocument) -> Option<&mut NativeSession<Self>>;

    /// Event index that Take back restores, if a human decision is available.
    fn take_back_target(game: &Self::Game, human: Self::Side, two_humans: bool) -> Option<usize> {
        if two_humans {
            return Self::last_command_start(game);
        }
        let events = Self::events(game);
        if let Some(last) = events.checked_sub(1)
            && Self::event(game, last) == (human, CommandKind::Terminal)
            && Self::side_to_move(game) == human
        {
            return Some(last);
        }
        (0..events)
            .rev()
            .find(|&i| Self::event(game, i) == (human, CommandKind::Play))
    }
    /// The actor of a command of the local player; for most games the local actor.
    fn command_actor(
        _game: &Self::Game,
        _settings: &SettingsOf<Self>,
        _human: Self::Side,
        _command: &Self::Command,
        local: Self::Side,
    ) -> Self::Side {
        local
    }
    /// Add game-specific snapshot fields after the generic ones.
    fn finish_snapshot(_snapshot: &mut SharedSnapshot, _state: &NativeSession<Self>) {}
    /// Advisory declaration facts of both sides, for a game with declarations.
    fn declaration_advice(_game: &Self::Game) -> Vec<DeclarationAdvice> {
        vec![]
    }
    /// The local human may submit a formal declaration now.
    fn can_declare(_game: &Self::Game, _human: Self::Side) -> bool {
        false
    }
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
    pub(crate) game: G::Game,
    viewed: Option<G::Game>,
    pub(crate) browse_stop: Option<BrowseStop>,
    pub(crate) pending: Option<ExecutionContext>,
    pub(crate) phase: EnginePhase,
    pub(crate) paused: bool,
    submitted: bool,
    requested_analysis: Option<AnalysisOperation>,
    pub(crate) analysis: Option<ExecutionObservation>,
    pub(crate) cancelled_analysis: Option<ExecutionContext>,
    /// Engine results of this session, in memory only (not saved).
    pub(crate) observations: Vec<ExecutionObservation>,
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
        Self::from_replayed_game(document, game, stopped)
    }
    fn from_replayed_game(
        document: ValidatedGameDocument<G>,
        game: G::Game,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, OperationFailure> {
        persistence::check(stopped)?;
        let (presentation, status, choices) = present::<G>(&game, None, stopped)?;
        persistence::check(stopped)?;
        Ok(Self {
            document: Arc::new(document),
            game,
            viewed: None,
            browse_stop: None,
            pending: None,
            phase: EnginePhase::Paused,
            paused: true,
            submitted: false,
            requested_analysis: None,
            analysis: None,
            cancelled_analysis: None,
            observations: vec![],
            current_observations: vec![],
            diagnostic: Some("Resume starts play.".into()),
            presentation,
            choices,
            status,
        })
    }
    /// Canonical record of the selected game.
    #[cfg(test)]
    pub(crate) fn record(&self) -> CanonicalRecord {
        G::context(self.selected()).expect("admitted native record")
    }
    /// Select a history position directly, for tests.
    #[cfg(test)]
    pub(crate) fn set_viewed(&mut self, viewed: Option<G::Game>) {
        self.viewed = viewed;
    }
    pub(crate) fn selected(&self) -> &G::Game {
        self.viewed.as_ref().unwrap_or(&self.game)
    }
    pub(crate) fn retire(&mut self) {
        self.pending = None;
        self.submitted = false;
        self.requested_analysis = None;
        self.analysis = None;
        self.cancelled_analysis = None;
        self.browse_stop = None;
        self.phase = EnginePhase::Paused;
        // The native context is never recovered by restoring a canonical record.
        G::retire_context(&mut self.game);
    }
    /// Install fresh identities while preserving New Game's running intent.
    pub(crate) fn prepare_install(&mut self) {
        self.retire();
        if !self.paused {
            self.phase = EnginePhase::Idle;
        }
    }
    fn cancel(&mut self) {
        let cancelled = self
            .pending
            .as_ref()
            .filter(|p| !matches!(p.purpose, ExecutionPurpose::EngineTurn {}))
            .cloned()
            .or_else(|| self.analysis.as_ref().map(|o| o.context.clone()))
            .or_else(|| self.cancelled_analysis.clone());
        self.retire();
        self.phase = G::cancellation_phase(cancelled.is_some());
        self.cancelled_analysis = cancelled;
        self.paused = true;
        self.diagnostic =
            Some("Work cancelled. The game is unchanged; Resume or start another analysis.".into());
    }
    fn local_actor(&self) -> G::Side {
        self.document
            .settings()
            .play_mode
            .actor(self.document.human(), G::side_to_move(&self.game))
    }
    pub(crate) fn take_back_target(&self) -> Option<usize> {
        G::take_back_target(
            &self.game,
            self.document.human(),
            self.document.settings().play_mode.is_two_humans(),
        )
    }
    /// A history position is selected.
    pub(crate) fn historical(&self) -> bool {
        self.viewed.is_some()
    }
    // Only used on the private admission candidate. Any error discards that
    // candidate; publication still requires the captured host context to match.
    fn apply_command(
        &mut self,
        actor: G::Side,
        command: G::Command,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<(), OperationFailure> {
        G::submit(&mut self.game, actor, command, stopped)
            .map_err(crate::game_document::Errors::<G>::native)?;
        self.document = Arc::new(self.document.with_game(&self.game)?);
        Ok(())
    }

    /// Admit a command of `actor` before the native game's own admission.
    fn command(
        &mut self,
        actor: G::Side,
        command: G::Command,
    ) -> Result<Change<G>, OperationFailure> {
        let kind = G::command_kind(&command);
        let play_mode = self.document.settings().play_mode;
        if self.paused
            && (G::requires_running(&command)
                || (play_mode.is_two_humans() && kind == CommandKind::Play))
        {
            return Err("Resume play before moving.".into());
        }
        if self.viewed.is_some() {
            return Err("Return to live before changing the game.".into());
        }
        if !play_mode.owns(self.document.human(), actor) {
            return Err("This command belongs to the other player.".into());
        }
        // Actor/turn/terminal/draw admission belongs to the native Game.
        // The immutable document candidate admits the complete command below.
        if kind == CommandKind::Terminal {
            self.paused = true;
        }
        Ok(Change::Command { actor, command })
    }

    fn apply(
        &mut self,
        operation: SessionOperation,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<bool, OperationFailure> {
        persistence::check(stopped)?;
        let local = self.local_actor();
        let change = match operation {
            SessionOperation::Play { action } => self.command(local, G::play(&action)?)?,
            SessionOperation::Command { command } => {
                let command: G::Command = serde_json::from_value(command)
                    .map_err(|e| format!("Invalid {} command: {e}", G::NAME))?;
                let actor = G::command_actor(
                    &self.game,
                    self.document.settings(),
                    self.document.human(),
                    &command,
                    local,
                );
                self.command(actor, command)?
            }
            SessionOperation::Resign {} => self.command(local, G::resign())?,
            SessionOperation::Preferences { settings } => {
                let orientation = G::side_of(settings.orientation)
                    .ok_or("The orientation side does not belong to this game.")?;
                Change::Settings(GameSettings {
                    play_mode: self.document.settings().play_mode,
                    orientation,
                    analysis_defaults: settings.analysis_defaults,
                })
            }
            SessionOperation::Undo {} | SessionOperation::TakeBack {} => {
                if self.viewed.is_some() {
                    return Err("Return to live before changing history.".into());
                }
                let event_index = if matches!(operation, SessionOperation::Undo {}) {
                    if !G::UNDO {
                        return Err("Undo is unsupported for this game; use Take back.".into());
                    }
                    G::last_command_start(&self.game).ok_or("No event to Undo.")?
                } else {
                    self.take_back_target()
                        .ok_or("No earlier playable human decision is available.")?
                };
                self.paused = true;
                Change::Rewind(event_index)
            }
            SessionOperation::Inspect { event_index } => {
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
                self.viewed = Some(G::to_game(&record, stopped).map_err(record_text)?);
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
                if was_viewed && !stop.paused && !G::concluded(&self.game) {
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
                self.cancel();
                return Ok(false);
            }
            SessionOperation::Analyze { operation } => {
                analysis_limits::<G>(operation)?;
                let stop = if self.viewed.is_some() {
                    self.browse_stop.clone()
                } else {
                    None
                };
                // The settings remember the last analysis limits.
                let settings = *self.document.settings();
                let durable = settings.analysis_defaults != operation;
                if durable {
                    self.document = Arc::new(self.document.with_settings(GameSettings {
                        analysis_defaults: operation,
                        ..settings
                    })?);
                }
                self.retire();
                self.browse_stop = stop;
                self.paused = true;
                self.requested_analysis = Some(operation);
                self.diagnostic = None;
                return Ok(durable);
            }
            SessionOperation::Resume {} | SessionOperation::Retry {} => {
                if self.viewed.is_some() {
                    return Err("Return to live before resuming play.".into());
                }
                let analysis = self.requested_analysis.is_some()
                    || self.analysis.is_some()
                    || self.cancelled_analysis.is_some()
                    || self
                        .pending
                        .as_ref()
                        .is_some_and(|p| !matches!(p.purpose, ExecutionPurpose::EngineTurn {}));
                if G::concluded(&self.game) && !(G::FINISH_CONCLUDED_ANALYSIS && analysis) {
                    return Err("Resume requires a live unconcluded game.".into());
                }
                self.retire();
                self.paused = false;
                self.phase = EnginePhase::Idle;
                self.diagnostic = None;
                return Ok(false);
            }
            SessionOperation::Replace { .. } => {
                return Err(
                    format!("Unsupported operation for the native {} profile.", G::NAME).into(),
                );
            }
        };
        match change {
            // Preferences change no game and keep current work.
            Change::Settings(settings) => {
                self.document = Arc::new(self.document.with_settings(settings)?);
                return Ok(true);
            }
            Change::Command { actor, command } => self.apply_command(actor, command, stopped)?,
            Change::Rewind(event_index) => {
                self.current_observations.clear();
                let document = self.document.with_rewind(event_index, stopped)?;
                self.game = G::to_game(document.record(), stopped).map_err(record_text)?;
                self.document = Arc::new(document);
            }
        }
        self.viewed = None;
        self.retire();
        Ok(true)
    }
    /// Completed work was refused by document admission: retire it and keep
    /// the accepted document.
    pub(crate) fn refuse_completed_work(&mut self, resource: bool) {
        self.retire();
        self.paused = true;
        self.phase = if resource {
            EnginePhase::Paused
        } else {
            EnginePhase::Failed
        };
        self.diagnostic = Some("Completed work refused by document admission; the accepted document is preserved and saveable. See Technical Details for the refusal.".into());
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
            cancelled_analysis: self.cancelled_analysis.clone(),
            observations: self.observations.clone(),
            current_observations: self.current_observations.clone(),
            diagnostic: self.diagnostic.clone(),
            selected: self.viewed.as_ref().map(G::events),
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
    requested_analysis: Option<AnalysisOperation>,
    analysis: Option<ExecutionObservation>,
    cancelled_analysis: Option<ExecutionContext>,
    observations: Vec<ExecutionObservation>,
    current_observations: Vec<u64>,
    diagnostic: Option<String>,
    /// Event count of the viewed history position, if any.
    selected: Option<usize>,
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
            state.cancelled_analysis = draft.cancelled_analysis;
            state.observations = draft.observations;
            state.current_observations = draft.current_observations;
            state.diagnostic = draft.diagnostic;
            if let Some(index) = draft.selected {
                let prefix = G::record_prefix(&state.game, index, stopped).map_err(record_text)?;
                state.viewed = Some(G::to_game(&prefix, stopped).map_err(record_text)?);
            }
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
        let failed = G::failure(&response.result).is_some();
        if !response.matches(pending) || !(failed || result.is_some()) {
            return Err(format!("Stale or unsupported {} observation.", G::NAME).into());
        }
        let analysis = !matches!(pending.purpose, ExecutionPurpose::EngineTurn {});
        let incomplete = result.is_some_and(G::incomplete);
        // Check the result against its exact target, then play an engine move.
        if let Some(result) = result {
            let target = state.selected();
            if G::context(target).map_err(record_text)? != response.context.game {
                return Err("Observation target differs from the admitted candidate.".into());
            }
            G::validate_result(result, &response.context, target, stopped)?;
            if !analysis && let Some((actor, command)) = G::engine_command(result, &state.game) {
                state.apply_command(actor, command, stopped)?;
                durable = true;
            }
        }
        if let Some(evicted) = keep_newest(&mut state.observations, response.clone()) {
            state
                .current_observations
                .retain(|id| *id != evicted.context.request_id);
        }
        let browse_stop = if analysis && !failed && !incomplete {
            state.browse_stop.clone()
        } else {
            None
        };
        state.retire();
        state.browse_stop = browse_stop;
        state.current_observations.push(response.context.request_id);
        let failure = G::failure(&response.result).map(|failure| failure.message.clone());
        if analysis {
            state.analysis = Some(response);
        }
        state.paused = analysis || failed || incomplete;
        state.phase = if failed {
            EnginePhase::Failed
        } else if incomplete {
            EnginePhase::Incomplete
        } else if analysis {
            EnginePhase::Paused
        } else {
            EnginePhase::Idle
        };
        state.diagnostic = if let Some(message) = failure {
            Some(format!(
                "Engine failure: {message} · The game is unchanged; Retry is explicit."
            ))
        } else if incomplete {
            Some("Incomplete search. No move, score or game outcome was created.".into())
        } else {
            None
        };
    }
    let (presentation, status, choices) =
        present::<G>(&state.game, state.viewed.as_ref(), stopped)?;
    state.presentation = presentation;
    state.status = status;
    state.choices = choices;
    persistence::check(stopped)?;
    Ok(archive::NativeCompleted::Runtime {
        state: G::opened(state),
        replacement: work.replacement,
        durable,
    })
}

/// The worker job of a runtime admission.
fn runtime_job<G: SessionGame>(work: RuntimeWork<G>) -> archive::NativeWork
where
    RuntimeWork<G>: Send,
{
    archive::NativeWork::Game(Box::new(move |stopped| execute(work, &mut || stopped())))
}

/// Open on the file worker: decode, validate and restore a saved document.
pub(crate) fn open<G: SessionGame>(
    bytes: &[u8],
    stopped: &mut impl FnMut() -> bool,
) -> Result<archive::OpenedNative, OperationFailure> {
    let (document, game) = ValidatedGameDocument::<G>::decode_with_game(bytes, stopped)?;
    Ok(G::opened(Box::new(NativeSession::from_replayed_game(
        document, game, stopped,
    )?)))
}

/// Results kept in memory per session; the oldest is dropped beyond this.
const KEPT_RESULTS: usize = 256;
fn keep_newest<T>(list: &mut Vec<T>, item: T) -> Option<T> {
    let evicted = (list.len() >= KEPT_RESULTS).then(|| list.remove(0));
    list.push(item);
    evicted
}

/// Save on the file worker: write the accepted document atomically.
pub(crate) fn save<G: NativeGame>(
    document: &ValidatedGameDocument<G>,
    path: &std::path::Path,
    revision: u64,
    identity: u64,
    stopped: &mut impl FnMut() -> bool,
) -> Result<archive::NativeCompleted, OperationFailure> {
    persistence::atomic_write(path, document.bytes(), stopped)?;
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
        // An approved replacement supersedes any pending admission.
        if replacement {
            self.retire_admission();
        }
        self.queue_native_transaction(
            runtime_job(RuntimeWork {
                seed,
                operation,
                response,
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
    /// Install a record as a resumed two-humans document, so that a test can
    /// submit the commands of both sides.
    #[cfg(test)]
    pub(crate) fn replace_two_humans<G: SessionGame>(
        &mut self,
        record: G::Record,
        bottom: G::Side,
    ) {
        let receipt = self
            .queue_game_candidate::<G>(
                RuntimeSeed::Record {
                    record: Box::new(record),
                    human: bottom,
                    play_mode: PlayMode::TwoHumans,
                },
                Some(SessionOperation::Resume {}),
                None,
                true,
            )
            .expect("queued two-humans document");
        while receipt.completion().is_none() {
            self.shared_snapshot();
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        receipt.completion().unwrap().expect("two-humans document");
    }
    pub(super) fn game_command<G: SessionGame>(
        &mut self,
        operation: SessionOperation,
    ) -> Result<SharedSnapshot, String> {
        if matches!(operation, SessionOperation::Cancel {}) {
            self.retire_admission();
            self.bridge.cancel();
            let state = G::state_mut(&mut self.session).expect("native owner");
            state.cancel();
            self.session.version += 1;
            self.session.selection_id += 1;
            self.session.revision += 1;
        }
        // While history is viewed, a command that would change the game is
        // refused at once; it changes nothing.
        if G::state(&self.session).is_some_and(|s| s.viewed.is_some())
            && !matches!(
                operation,
                SessionOperation::Inspect { .. }
                    | SessionOperation::ReturnLive {}
                    | SessionOperation::Analyze { .. }
                    | SessionOperation::Cancel {}
            )
            && !matches!(operation, SessionOperation::Preferences { .. })
        {
            return Err("Return to live before changing the game.".into());
        }
        // Refuse a malformed analysis request at once; it changes nothing.
        if let SessionOperation::Analyze { operation } = &operation {
            analysis_limits::<G>(*operation)?;
        }

        let state = G::state(&self.session).expect("native owner");
        if matches!(operation, SessionOperation::TakeBack {})
            && (state.viewed.is_some() || state.take_back_target().is_none())
        {
            return Err("No live playable human decision is available for Take back.".into());
        }
        let state = state.capture();
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
        let revision = self.session.document_revision;
        let identity = self.session.document_identity;
        let id = self.next_native_io(IoPhase::Saving)?;
        self.files.submit(
            id,
            persistence::Work::Native(Box::new(archive::NativeWork::Game(Box::new(
                move |stopped| save(&document, &path, revision, identity, &mut || stopped()),
            )))),
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
        let selected = G::context(state.selected()).expect("admitted record");
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
        self.start_game_operation::<G>(
            limits,
            analysis.then_some(AnalysisOperation {
                depth: limits.depth,
                nodes: limits.nodes,
            }),
        )
    }
    fn start_game_operation<G: SessionGame>(
        &mut self,
        limits: WorkLimits,
        operation: Option<AnalysisOperation>,
    ) -> Result<(), OperationFailure> {
        if let Some(operation) = operation {
            analysis_limits::<G>(operation)?;
        } else {
            validate_limits::<G>(limits)?;
        }
        self.ensure_runtime_capacity()?;
        let state = G::state(&self.session).expect("native owner");
        let game = G::context(state.selected()).map_err(record_text)?;
        let turn = G::side_to_move(state.selected());
        let historical = state.viewed.is_some();
        self.session.next_request += 1;
        let context = ExecutionContext {
            purpose: if operation.is_some() {
                let target = ExecutionTarget {
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
                };
                ExecutionPurpose::MoveAnalysis { target }
            } else {
                ExecutionPurpose::EngineTurn {}
            },
            request_id: self.session.next_request,
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
            if let Some(operation) = analysis {
                let started = analysis_limits::<G>(operation)
                    .map_err(OperationFailure::from)
                    .and_then(|limits| self.start_game_operation::<G>(limits, Some(operation)));
                if let Err(e) = started {
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
        // The Chess QA incomplete fixture sends a real zero-node request.
        // Normal product work still uses the validated positive play limits.
        let zero_chess = G::KIND == GameKind::Chess && limits.nodes == 0 && !analysis;
        self.start_game_work::<G>(
            if zero_chess {
                WorkLimits { nodes: 1, ..limits }
            } else {
                limits
            },
            analysis,
        )
        .map_err(|e| e.diagnostic)?;
        if zero_chess {
            G::state_mut(&mut self.session)
                .expect("native owner")
                .pending
                .as_mut()
                .expect("started work")
                .limits = limits;
        }
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
        if G::state(&self.session)
            .is_some_and(|s| s.paused && s.phase == EnginePhase::Paused && s.pending.is_none())
        {
            return;
        }
        // Stop all work; closing never changes the saved document.
        self.bridge.cancel();
        let state = G::state_mut(&mut self.session).expect("native owner");
        state.retire();
        state.paused = true;
        self.session.version += 1;
        self.session.selection_id += 1;
        self.session.revision += 1;
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
                undo: !historical && G::last_command_start(&state.game).is_some(),
                engine_turn: !settings.play_mode.is_two_humans(),
                move_analysis: true,
                claims: false,
                declaration: !historical
                    && !ended
                    && G::can_declare(&state.game, settings.play_mode.actor(human, turn)),
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
            execution_purpose: state.pending.as_ref().map(|p| match p.purpose {
                ExecutionPurpose::EngineTurn {} => "engine_turn",
                ExecutionPurpose::MoveAnalysis { .. } => "move_analysis",
            }),
            diagnostic: state.diagnostic.clone(),
            declaration_advice: G::declaration_advice(game),
            context: G::context(game).expect("admitted native record"),
            analysis: state.analysis.as_ref().map(|o| PresentedObservation {
                observation: o.clone(),
                historical: false,
                application_error: None,
            }),
            observations: state
                .observations
                .iter()
                .map(|o| PresentedObservation {
                    observation: o.clone(),
                    historical: !state.current_observations.contains(&o.context.request_id),
                    application_error: None,
                })
                .collect(),
            search_observations: state
                .observations
                .iter()
                .enumerate()
                .map(|(i, o)| display_observation(i, o, state))
                .collect(),
            score_meaning: G::SCORE_MEANING,
            chess: None,
            chess_match: None,
        };
        G::present_in(&mut snapshot, state.presentation.clone());
        G::finish_snapshot(&mut snapshot, state);
        snapshot
    }
}

/// Display label of a shared-search stop reason.
pub(crate) fn stop_label(status: mshogi_engine::StopReason) -> &'static str {
    match status {
        mshogi_engine::StopReason::Complete => "Complete",
        mshogi_engine::StopReason::NodeLimit => "Node limit reached",
        mshogi_engine::StopReason::Stopped => "Search stopped",
        mshogi_engine::StopReason::Unresolved => "Unresolved, unscored",
    }
}

/// Telemetry of a result of a game on the shared search core; `label` names
/// each choice of the reported line.
pub(crate) fn engine_telemetry<E: EngineGame>(
    result: &EngineResult<E>,
    label: impl Fn(&E::Choice) -> String,
) -> TelemetryView {
    let t = result.telemetry();
    TelemetryView {
        score: t.score_label(),
        completed_depth: t.completed_depth,
        nodes: t.nodes,
        variation: t.principal_variation.iter().map(label).collect(),
        status: stop_label(t.status),
    }
}
/// The engine completed no scored work.
pub(crate) fn engine_incomplete<E: EngineGame>(result: &EngineResult<E>) -> bool {
    matches!(
        result,
        EngineResult::Incomplete { .. } | EngineResult::Unresolved { .. }
    )
}
pub(crate) fn display_observation<G: SessionGame>(
    index: usize,
    o: &ExecutionObservation,
    state: &NativeSession<G>,
) -> PresentedSearchObservation {
    let record = &o.context.game;
    let telemetry = G::engine_result(&o.result).map(G::telemetry);
    PresentedSearchObservation {
        selective_depth: None,
        observation_index: index,
        purpose: if matches!(o.context.purpose, ExecutionPurpose::EngineTurn {}) {
            "engine_turn"
        } else {
            "move_analysis"
        },
        event_index: record.event_count(),
        board_ply: o.context.game.known_board_actions(),
        perspective: side(record.profile(), G::native_side(G::context_side(record))).label,
        engine: "mShogi",
        score: telemetry.as_ref().and_then(|t| t.score.clone()),
        completed_depth: telemetry.as_ref().map(|t| t.completed_depth),
        nodes: telemetry.as_ref().map(|t| t.nodes),
        // Native complete route strings retain every landing and promotion; exact typed PV remains in Technical Details.
        variation: telemetry
            .as_ref()
            .map_or_else(Vec::new, |t| t.variation.clone()),
        status: match telemetry.as_ref().map(|t| t.status) {
            Some(status) => status,
            None => match &o.result {
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

/// The wrappers of the shared runtime enums for one game: its worker jobs,
/// active-document variant and snapshot field.
macro_rules! session_wiring {
    ($game:ident, state: $state:ident, $state_mut:ident $(, field: $field:ident)?) => {
        fn opened(state: Box<NativeSession<Self>>) -> archive::OpenedNative {
            archive::OpenedNative::$game(state)
        }
        fn state(session: &ActiveDocument) -> Option<&NativeSession<Self>> {
            session.$state()
        }
        fn state_mut(session: &mut ActiveDocument) -> Option<&mut NativeSession<Self>> {
            session.$state_mut()
        }
        #[allow(unused_variables)]
        fn present_in(snapshot: &mut SharedSnapshot, presentation: Self::Presentation) {
            $(snapshot.$field = Some(presentation);)?
        }
    };
}

/// The Chu and Tenjiku sessions differ only by names and the PV label.
macro_rules! large_shogi_session {
    ($game:ident, $core:ident,
     state: $state:ident, $state_mut:ident, field: $field:ident,
     board: $size:expr, $label:expr, $focus:expr, score_meaning: $meaning:expr,
     pv: |$a:ident| $pv:expr) => {
        impl SessionGame for $game {
            type Presentation = presentation::Presentation;
            session_wiring!($game, state: $state, $state_mut, field: $field);

            fn presentation(selected: &c::Game, live: &c::Game) -> Self::Presentation {
                presentation::Presentation::new(selected, live)
            }
            fn status(selected: &c::Game, has_actions: bool) -> PresentedStatus {
                presentation::status(selected, has_actions)
            }
            fn choice(_game: &c::Game, action: c::Action, all: &[c::Action]) -> PresentedChoice {
                presentation::choice(action, all)
            }
            fn play(action: &str) -> Result<c::GameCommand, String> {
                let action = serde_json::from_str(action).map_err(|e| e.to_string())?;
                Ok(c::GameCommand::Play { action })
            }
            fn command_actor(
                game: &c::Game,
                _settings: &crate::game_document::SettingsOf<Self>,
                _human: c::Side,
                command: &c::GameCommand,
                local: c::Side,
            ) -> c::Side {
                // The draw response belongs to the opponent of the offer.
                match command {
                    c::GameCommand::AcceptDraw | c::GameCommand::DeclineDraw => {
                        game.snapshot().draw_offer.map_or(local, c::Side::opponent)
                    }
                    _ => local,
                }
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
                    c::GameCommand::Resign => CommandKind::Terminal,
                    _ => CommandKind::Other,
                }
            }
            fn telemetry(result: &Self::EngineResult) -> TelemetryView {
                engine_telemetry(result, |$a| $pv)
            }
            fn incomplete(result: &Self::EngineResult) -> bool {
                engine_incomplete(result)
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

#[cfg(test)]
mod retention_tests {
    use super::*;
    use crate::native_game::Tori;
    use mshogi_core::tori;

    #[test]
    fn repeated_results_keep_only_recent_observations_and_current_ids() {
        let record = tori::GameRecord::from_game(&tori::Game::standard()).unwrap();
        let document =
            ValidatedGameDocument::<Tori>::new(record.clone(), tori::Side::Sente).unwrap();
        let saved = document.bytes().to_vec();
        let mut state = Box::new(NativeSession::<Tori>::restore(document, &mut || false).unwrap());
        for id in 1..=(KEPT_RESULTS as u64 + 4) {
            let context = ExecutionContext {
                purpose: ExecutionPurpose::EngineTurn {},
                request_id: id,
                session_version: 1,
                game: CanonicalRecord::tori(record.clone()),
                limits: WorkLimits {
                    time_ms: None,
                    depth: 1,
                    nodes: 1,
                },
            };
            state.pending = Some(context.clone());
            let response = ExecutionObservation::new(
                context,
                ExecutionResult::Failure {
                    failure: mshogi_engine::EngineFailure {
                        kind: mshogi_engine::FailureKind::Search,
                        message: "Constructed failure for retention test".into(),
                    },
                },
            );
            let completed = execute(
                RuntimeWork {
                    seed: RuntimeSeed::Current(Box::new(state.capture())),
                    operation: None,
                    response: Some(response),
                    replacement: false,
                },
                &mut || false,
            )
            .unwrap();
            let archive::NativeCompleted::Runtime {
                state: archive::OpenedNative::Tori(next),
                durable,
                ..
            } = completed
            else {
                panic!("expected Tori runtime completion")
            };
            assert!(!durable);
            state = next;
        }
        assert_eq!(state.observations.len(), KEPT_RESULTS);
        assert_eq!(state.current_observations, (5..=260).collect::<Vec<_>>());
        assert_eq!(state.observations.first().unwrap().context.request_id, 5);
        assert_eq!(state.observations.last().unwrap().context.request_id, 260);
        assert_eq!(state.document.bytes(), saved);
    }
}
