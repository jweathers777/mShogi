//! Versioned lossless archives for the concrete shared desktop consumers.
use super::*;
use crate::persistence::{self, Completed, Work};
use serde_json::value::RawValue;
use std::{fs, io::Read};

/// Native shogi preferences; presentation does not prescribe artwork or layout.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShogiSettings {
    /// Native side placed at the bottom by a presentation consumer.
    pub orientation: s::Side,
    /// Explicit default bounded ordinary analysis settings.
    pub analysis_defaults: WorkLimits,
}
impl Default for ShogiSettings {
    fn default() -> Self {
        Self {
            orientation: s::Side::Sente,
            analysis_defaults: WorkLimits {
                depth: 1,
                nodes: 1000,
            },
        }
    }
}
impl ShogiSettings {
    fn validate(&self) -> Result<(), String> {
        valid_limits(self.analysis_defaults)
    }
}
/// Game-qualified settings without a common board or rules representation.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "game_kind", rename_all = "snake_case")]
pub enum NativeDesktopSettings {
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
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ShogiControl {
    sequence: usize,
    #[serde(deserialize_with = "required_option")]
    command: Option<SessionOperation>,
    #[serde(deserialize_with = "required_option")]
    observation: Option<usize>,
    close_pause: bool,
}
pub(crate) fn required_option<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    Option::<T>::deserialize(d)
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ShogiArchive {
    initial: CanonicalRecord,
    current: CanonicalRecord,
    human: NativeSide,
    initial_settings: ShogiSettings,
    settings: ShogiSettings,
    controls: Vec<ShogiControl>,
    observations: Vec<PresentedObservation>,
    // A retired high-water mark is evidence/headroom, never pending execution.
    retired_execution_floor: u64,
}
pub(crate) enum ShogiSeed {
    Current(Box<ShogiSession>),
    Record {
        record: Box<s::GameRecord>,
        human: s::Side,
        orient_human: bool,
    },
}
pub(crate) enum NativeWork {
    Xiangqi(xiangqi::archive::XiangqiWork),
    Admit {
        candidate: ShogiSeed,
        command: Option<SessionOperation>,
        response: Option<Box<ExecutionObservation>>,
        replacement: bool,
        floor: u64,
    },
    Save {
        candidate: Box<ShogiSession>,
        path: PathBuf,
        revision: u64,
        identity: u64,
        floor: u64,
    },
    Open {
        path: PathBuf,
        token: SharedOpenToken,
    },
}
pub(crate) enum OpenedNative {
    Xiangqi {
        state: Box<xiangqi::XiangqiSession>,
        floor: u64,
    },
    Chess(Box<persistence::Opened>),
    Shogi {
        state: Box<ShogiSession>,
        floor: u64,
    },
}
pub(crate) enum NativeCompleted {
    Xiangqi(xiangqi::archive::XiangqiCompleted),
    Admitted {
        state: Box<ShogiSession>,
        analysis: Option<WorkLimits>,
        replacement: bool,
    },
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
    if !(1..=4).contains(&limits.depth) || limits.nodes > 20_000 {
        Err("Unsupported shogi operation/limits: ordinary depth1–4 and at most20,000 nodes; untimed play only.".into())
    } else {
        Ok(())
    }
}
fn native(record: &CanonicalRecord) -> Result<&s::GameRecord, String> {
    match record {
        CanonicalRecord::Shogi(record) => Ok(record),
        _ => Err("Wrong game: a shogi archive contains a non-shogi native record.".into()),
    }
}
fn archive(state: &ShogiSession, floor: u64) -> ShogiArchive {
    ShogiArchive {
        initial: CanonicalRecord::shogi(state.initial_record.clone()),
        current: CanonicalRecord::shogi(s::GameRecord::from_game(&state.game)),
        human: NativeSide::Shogi(state.human),
        initial_settings: state.initial_settings.clone(),
        settings: state.settings.clone(),
        controls: state.controls.clone(),
        observations: state.observations.clone(),
        retired_execution_floor: floor,
    }
}
fn encode(state: &ShogiSession, floor: u64) -> Result<Vec<u8>, String> {
    crate::document::encode(RulesProfile::ShogiCsa36V1, &archive(state, floor))
}
fn needs_stop(state: &ShogiSession) -> bool {
    state.pending.is_some() || state.analysis_intent.is_some() || !state.paused
}

/// Bound total replay, including semantic Undo reconstruction, independently of wall time.
const MAX_REPLAY_ACTIONS: usize = 16_000;
// Native event admission/reconstruction costs more than a cheap desktop control.
const NATIVE_EVENT_WORK: usize = 4;
fn counts(state: &ShogiSession, reserve: bool) -> Result<(), OperationFailure> {
    if state.controls.len() + usize::from(reserve) > MAX_ARCHIVE_CONTROLS
        || state.observations.len() > MAX_ARCHIVE_OBSERVATIONS
    {
        let mut facts = Vec::new();
        if state.controls.len() + usize::from(reserve) > MAX_ARCHIVE_CONTROLS {
            facts.push(resource_fact(
                ResourceCategory::Controls,
                state.controls.len(),
                MAX_ARCHIVE_CONTROLS,
                usize::from(reserve),
                ResourceScope::Shogi,
                ResourceUnit::Controls,
            ));
        }
        if state.observations.len() > MAX_ARCHIVE_OBSERVATIONS {
            facts.push(resource_fact(
                ResourceCategory::OrdinaryObservations,
                state.observations.len(),
                MAX_ARCHIVE_OBSERVATIONS,
                0,
                ResourceScope::Shogi,
                ResourceUnit::Observations,
            ));
        }
        return Err(resource_refusal(
            facts,
            "Application capacity: native Shogi document counts exceed supported limits including stop/close reservation. Current accepted document remains saveable.",
        ));
    }
    if state.game.events().len() > s::MAX_RECORD_EVENTS
        || state.initial_record.event_count() > s::MAX_RECORD_EVENTS
    {
        return Err(resource_refusal(
            vec![resource_fact(
                ResourceCategory::NativeEvents,
                state
                    .game
                    .events()
                    .len()
                    .max(state.initial_record.event_count()),
                s::MAX_RECORD_EVENTS,
                0,
                ResourceScope::Shogi,
                ResourceUnit::Events,
            )],
            "Application capacity: native Shogi event limit reached.",
        ));
    }
    Ok(())
}
fn check_capacity(
    state: &ShogiSession,
    stopped: &mut impl FnMut() -> bool,
) -> Result<(), OperationFailure> {
    let reserve = needs_stop(state) || state.durable_needs_stop;
    counts(state, reserve)?;
    persistence::check(stopped)?;
    // Historical flags become true on Open: budget the longer false spelling now.
    let mut state = state.clone();
    for o in &mut state.observations {
        o.historical = false;
    }
    let bytes = encode(&state, MAX_SAFE_ID - 17)?;
    if bytes.len() + if reserve { 1024 } else { 0 } > MAX_ARCHIVE_BYTES {
        return Err(resource_refusal(
            vec![resource_fact(
                ResourceCategory::EncodedDocumentBytes,
                bytes.len(),
                MAX_ARCHIVE_BYTES,
                if reserve { 1024 } else { 0 },
                ResourceScope::Shogi,
                ResourceUnit::Bytes,
            )],
            "Application capacity: complete Shogi document exceeds16MiB including stop/close headroom.",
        ));
    }
    persistence::check(stopped)?;
    replay_work(&state)?;
    Ok(())
}
fn replay_work(state: &ShogiSession) -> Result<(), OperationFailure> {
    let mut work = (state.initial_record.event_count() + state.game.events().len())
        * NATIVE_EVENT_WORK
        + usize::from(needs_stop(state) || state.durable_needs_stop);
    for c in &state.controls {
        work = work.saturating_add(match &c.command {
            Some(SessionOperation::Undo {} | SessionOperation::TakeBack {}) => {
                s::MAX_RECORD_EVENTS * NATIVE_EVENT_WORK
            }
            Some(SessionOperation::Inspect { .. }) => s::MAX_RECORD_EVENTS * NATIVE_EVENT_WORK,
            _ => 1,
        });
    }
    for o in &state.observations {
        work =
            work.saturating_add(o.observation.context.game.event_count() * NATIVE_EVENT_WORK + 8);
    }
    if work > MAX_REPLAY_ACTIONS {
        Err(resource_refusal(
            vec![resource_fact(
                ResourceCategory::ReplayWork,
                work.saturating_sub(usize::from(needs_stop(state) || state.durable_needs_stop)),
                MAX_REPLAY_ACTIONS,
                usize::from(needs_stop(state) || state.durable_needs_stop),
                ResourceScope::Shogi,
                ResourceUnit::ReplayUnits,
            )],
            "Application capacity: complete Shogi document exceeds16,000 native replay work units.",
        ))
    } else {
        Ok(())
    }
}
impl ShogiSession {
    // Seed a complete cheap-control journal only in a separately labeled QA build.
    #[cfg(feature = "desktop-verification")]
    pub(crate) fn verification_control_capacity(&mut self) -> Result<(), String> {
        while self.controls.len() < MAX_ARCHIVE_CONTROLS - 1 {
            let operation = SessionOperation::ShogiPreferences {
                settings: self.settings.clone(),
            };
            self.apply_command(operation.clone())?;
            self.control(Some(operation), None, false);
        }
        check_capacity(self, &mut || false).map_err(String::from)
    }
    pub(crate) fn restored(record: &s::GameRecord, human: s::Side) -> Result<Self, String> {
        let settings = ShogiSettings::default();
        Ok(Self {
            initial_record: record.clone(),
            initial_settings: settings.clone(),
            settings,
            controls: vec![],
            analysis_intent: None,
            durable_needs_stop: false,
            execution_floor: 0,
            game: record
                .to_game()
                .map_err(|e| format!("Corrupt native shogi record: {e}"))?,
            human,
            viewed: None,
            paused: true,
            pending: None,
            submitted: false,
            phase: EnginePhase::Paused,
            diagnostic: None,
            analysis: None,
            observations: vec![],
        })
    }
    pub(crate) fn control(
        &mut self,
        command: Option<SessionOperation>,
        observation: Option<usize>,
        close_pause: bool,
    ) {
        if close_pause {
            self.durable_needs_stop = false;
        }
        self.controls.push(ShogiControl {
            sequence: self.controls.len() + 1,
            command,
            observation,
            close_pause,
        });
    }
    fn apply_command(
        &mut self,
        operation: SessionOperation,
    ) -> Result<Option<WorkLimits>, OperationFailure> {
        if matches!(
            operation,
            SessionOperation::Analyze { .. }
                | SessionOperation::Resume {}
                | SessionOperation::Retry {}
                | SessionOperation::Play { .. }
        ) && self.observations.len() >= MAX_ARCHIVE_OBSERVATIONS
        {
            return Err(resource_refusal(
                vec![
                    resource_fact(
                        ResourceCategory::OrdinaryObservations,
                        self.observations.len(),
                        MAX_ARCHIVE_OBSERVATIONS,
                        0,
                        ResourceScope::Shogi,
                        ResourceUnit::Observations,
                    )
                    .with_measurement(ResourceMeasurement::Accepted),
                ],
                "Application capacity: all256 observation slots are retained; no new work started.",
            ));
        }
        let preferences = matches!(operation, SessionOperation::ShogiPreferences { .. });
        let mut analysis = None;
        match operation {
            SessionOperation::Play{action}=>{
                if self.paused||self.viewed.is_some()||self.game.position().side_to_move()!=self.human{return Err("Native board action requires the resumed human live turn.".into());}
                self.game.submit(s::GameRequest{expected:self.game.context(),actor:self.human,command:s::GameCommand::Play(action)}).map_err(|e|format!("Native action rejected: {e:?}"))?;
                self.retire();self.paused=false;self.phase=EnginePhase::Idle;
            }
            SessionOperation::Declare{}|SessionOperation::Resign{}=>{
                if self.viewed.is_some(){return Err("Return to live before a terminal command.".into());}
                let command=if matches!(operation,SessionOperation::Declare{}){s::GameCommand::DeclareEnteringKing}else{s::GameCommand::Resign};
                self.game.submit(s::GameRequest{expected:self.game.context(),actor:self.human,command}).map_err(|e|format!("Native terminal command rejected: {e:?}"))?;
                self.retire();self.paused=true;
            }
            SessionOperation::TakeBack{}=>{
                if self.viewed.is_some(){return Err("Return to live before Take back.".into());}
                let target=self.take_back_target().ok_or("No earlier playable human decision is available.")?;
                self.game=self.game.prefix(target).map_err(|e|format!("Native Take back rejected: {e:?}"))?;
                self.retire();self.paused=true;
                self.diagnostic=Some("Position restored. Resume play when ready.".into());
                for o in &mut self.observations{o.historical=true;}
            }
            SessionOperation::Undo{}=>{
                if self.viewed.is_some()||self.game.events().is_empty(){return Err("Undo requires a live accepted event.".into());}
                self.game=self.game.prefix(self.game.events().len()-1).map_err(|e|format!("Native undo rejected: {e:?}"))?;self.retire();self.paused=true;
            }
            SessionOperation::Inspect{event_index}=>{self.viewed=Some(self.game.prefix(event_index).map_err(|e|format!("Native history rejected: {e:?}"))?);self.retire();self.paused=true;}
            SessionOperation::ReturnLive{}=>{self.viewed=None;self.retire();self.paused=true;}
            SessionOperation::Cancel{}=>{self.retire();self.paused=true;self.phase=EnginePhase::Cancelled;}
            SessionOperation::Analyze{operation:AnalysisOperation::Moves{depth,nodes}}=>{
                let limits=WorkLimits{depth,nodes};valid_limits(limits)?;self.retire();self.paused=true;self.settings.analysis_defaults=limits;self.analysis_intent=Some((self.record(),limits));analysis=Some(limits);
            }
            SessionOperation::Resume{}|SessionOperation::Retry{}=>{
                if self.viewed.is_some()||self.game.conclusion().is_some(){return Err("Resume requires a live unconcluded game.".into());}
                self.retire();self.paused=false;self.phase=EnginePhase::Idle;
            }
            SessionOperation::ShogiPreferences{settings}=>{settings.validate()?;self.settings=settings;}
            _=>return Err("Unsupported shogi operation: chess Full/claims/chess commands are not shogi capabilities.".into()),
        }
        if !preferences {
            self.durable_needs_stop = needs_stop(self);
        }
        Ok(analysis)
    }
    fn apply_response(&mut self, response: ExecutionObservation) -> Result<(), OperationFailure> {
        if serde_json::to_vec(&execution_request(response.context.clone()))
            .map_err(bounded)?
            .len()
            > mshogi_engine::MAX_REQUEST_BYTES
            || serde_json::to_vec(&ExecutionResponse {
                protocol: mshogi_engine::PROTOCOL_VERSION,
                observation: response.clone(),
            })
            .map_err(bounded)?
            .len()
                > mshogi_engine::MAX_RESPONSE_BYTES
        {
            return Err(OperationFailure::resource(ResourceFailure::unknown(
                "Application capacity: encoded observation context/result exceeds native transport limits.",
            )));
        }
        if !response.matches(&response.context) {
            return Err("Corrupt observation: invalid native context/result metadata.".into());
        }
        if self.observations.len() >= MAX_ARCHIVE_OBSERVATIONS {
            return Err(resource_refusal(
                vec![
                    resource_fact(
                        ResourceCategory::OrdinaryObservations,
                        self.observations.len(),
                        MAX_ARCHIVE_OBSERVATIONS,
                        0,
                        ResourceScope::Shogi,
                        ResourceUnit::Observations,
                    )
                    .with_measurement(ResourceMeasurement::Accepted),
                ],
                "Application capacity: all256 observation slots are retained; no new observation accepted.",
            ));
        }
        let analysis = matches!(
            response.context.purpose,
            ExecutionPurpose::MoveAnalysis { .. }
        );
        let expected = if analysis {
            let (game, limits) = self
                .analysis_intent
                .as_ref()
                .ok_or("Corrupt observation: no corresponding analysis command.")?;
            if *limits != response.context.limits {
                return Err(
                    "Corrupt observation: analysis limits disagree with its command.".into(),
                );
            }
            game.clone()
        } else {
            if !matches!(response.context.purpose, ExecutionPurpose::EngineTurn {})
                || self.paused
                || self.viewed.is_some()
                || self.game.conclusion().is_some()
                || self.game.position().side_to_move() == self.human
            {
                return Err("Corrupt observation: no corresponding native engine turn.".into());
            }
            CanonicalRecord::shogi(s::GameRecord::from_game(&self.game))
        };
        if response.context.game != expected {
            return Err("Corrupt observation: exact native history/target differs from its command context.".into());
        }
        // Validate the short PV in private native state, never on the live game.
        // The journal already owns this exact native prefix. Clone that proven
        // state instead of replaying hundreds of events for every empty/short PV.
        // Strict native record parsing remains mandatory at the archive boundary.
        if CanonicalRecord::shogi(s::GameRecord::from_game(self.selected())) != expected {
            return Err(
                "Corrupt observation: selected native prefix disagrees with command intent.".into(),
            );
        }
        let mut probe = self.selected().clone();
        if let ExecutionPurpose::MoveAnalysis { target } = &response.context.purpose {
            if target.turn != NativeSide::Shogi(probe.position().side_to_move()) {
                return Err(
                    "Corrupt observation: analysis side disagrees with the exact native target."
                        .into(),
                );
            }
        } else if response.context.limits
            != (WorkLimits {
                depth: 2,
                nodes: 5000,
            })
        {
            return Err(
                "Unsupported saved shogi engine-turn limits; no downgrade was applied.".into(),
            );
        }
        let result = match &response.result {
            ExecutionResult::Shogi { result } => Some(result),
            ExecutionResult::Failure { .. } => None,
            _ => return Err("Wrong game: chess observation in a shogi document.".into()),
        };
        let mut pv_error = None;
        if let Some(
            ShogiResult::Choice { telemetry, .. }
            | ShogiResult::Finished { telemetry, .. }
            | ShogiResult::Incomplete { telemetry },
        ) = result
        {
            for choice in &telemetry.principal_variation {
                let (actor, command) = match choice {
                    ShogiChoice::Move { action } => (
                        probe.position().side_to_move(),
                        s::GameCommand::Play(*action),
                    ),
                    ShogiChoice::Declare { actor } => (*actor, s::GameCommand::DeclareEnteringKing),
                };
                if let Err(e) = probe.submit(s::GameRequest {
                    expected: probe.context(),
                    actor,
                    command,
                }) {
                    pv_error = Some(format!("Native engine choice rejected: {e:?}"));
                    break;
                }
            }
        }
        let mut error = None;
        self.phase = match &response.result {
            ExecutionResult::Failure { failure } => {
                self.diagnostic = Some(failure.message.clone());
                EnginePhase::Failed
            }
            ExecutionResult::Shogi {
                result: ShogiResult::Incomplete { .. },
            } => EnginePhase::Incomplete,
            ExecutionResult::Shogi {
                result: ShogiResult::Finished { .. },
            } => EnginePhase::Idle,
            ExecutionResult::Shogi {
                result: ShogiResult::Choice { choice, .. },
            } => {
                if !analysis {
                    let (actor, command) = match choice {
                        ShogiChoice::Move { action } => (
                            self.game.position().side_to_move(),
                            s::GameCommand::Play(*action),
                        ),
                        ShogiChoice::Declare { actor } => {
                            (*actor, s::GameCommand::DeclareEnteringKing)
                        }
                    };
                    if let Err(e) = self.game.submit(s::GameRequest {
                        expected: self.game.context(),
                        actor,
                        command,
                    }) {
                        error = Some(format!("Native engine choice rejected: {e:?}"));
                    }
                }
                if error.is_some() {
                    EnginePhase::Failed
                } else {
                    EnginePhase::Idle
                }
            }
            _ => unreachable!(),
        };
        if pv_error.is_some() && error.is_none() {
            return Err(
                "Corrupt observation: native principal variation contains an inadmissible choice."
                    .into(),
            );
        }
        if let Some(e) = &error {
            self.diagnostic = Some(e.clone());
        }
        let observation = PresentedObservation {
            observation: response,
            application_error: error,
            historical: false,
        };
        if analysis {
            self.analysis = Some(observation.clone());
            self.analysis_intent = None;
        } else if self.phase != EnginePhase::Idle {
            self.paused = true;
        }
        self.pending = None;
        self.submitted = false;
        self.observations.push(observation);
        self.durable_needs_stop = needs_stop(self);
        Ok(())
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawShogiArchive {
    initial: Box<RawValue>,
    current: Box<RawValue>,
    human: NativeSide,
    initial_settings: ShogiSettings,
    settings: ShogiSettings,
    controls: Vec<Box<RawValue>>,
    observations: Vec<Box<RawValue>>,
    retired_execution_floor: u64,
}
fn raw_event_count(text: &str) -> Result<usize, OperationFailure> {
    #[derive(Deserialize)]
    struct Envelope {
        game_kind: String,
        rules_profile: String,
        profile_edition: String,
        payload: Box<RawValue>,
    }
    #[derive(Deserialize)]
    struct Ledger {
        events: Vec<Box<RawValue>>,
    }
    let record: Envelope = serde_json::from_str(text).map_err(bounded)?;
    if record.game_kind != "shogi" {
        return Err("Wrong game: shogi archive contains a non-shogi native context.".into());
    }
    if record.rules_profile != s::PROFILE_ID || record.profile_edition != s::PROFILE_EDITION {
        return Err(
            "Wrong profile/edition: native shogi context disagrees with the selected archive."
                .into(),
        );
    }
    let ledger: Ledger = serde_json::from_str(record.payload.get()).map_err(bounded)?;
    if ledger.events.len() > s::MAX_RECORD_EVENTS {
        return Err(resource_refusal(
            vec![resource_fact(
                ResourceCategory::NativeEvents,
                ledger.events.len(),
                s::MAX_RECORD_EVENTS,
                0,
                ResourceScope::Shogi,
                ResourceUnit::Events,
            )],
            "Application capacity: native Shogi event bound exceeded.",
        ));
    }
    Ok(ledger.events.len())
}
fn parse_shogi_archive(
    payload: &str,
    stopped: &mut impl FnMut() -> bool,
) -> Result<ShogiArchive, OperationFailure> {
    let raw: RawShogiArchive = serde_json::from_str(payload)
        .map_err(|e| format!("Corrupt shogi archive: {}", bounded(e)))?;
    if raw.controls.len() > MAX_ARCHIVE_CONTROLS
        || raw.observations.len() > MAX_ARCHIVE_OBSERVATIONS
    {
        return Err(document_count_failure(
            raw.controls.len(),
            raw.observations.len(),
            0,
            false,
            ResourceScope::Shogi,
            0,
        ));
    }
    // This is only a resource probe, never an alternate parser. Every original
    // raw value subsequently reaches its strict native parser unchanged.
    let mut work = (raw_event_count(raw.initial.get())? + raw_event_count(raw.current.get())?)
        * NATIVE_EVENT_WORK;
    #[derive(Deserialize)]
    struct ObservationProbe {
        observation: Box<RawValue>,
    }
    #[derive(Deserialize)]
    struct ResponseProbe {
        context: Box<RawValue>,
    }
    #[derive(Deserialize)]
    struct ContextProbe {
        game: Box<RawValue>,
    }
    for text in &raw.observations {
        persistence::check(stopped)?;
        let observation: ObservationProbe = serde_json::from_str(text.get()).map_err(bounded)?;
        let response: ResponseProbe =
            serde_json::from_str(observation.observation.get()).map_err(bounded)?;
        let context: ContextProbe =
            serde_json::from_str(response.context.get()).map_err(bounded)?;
        work = work.saturating_add(raw_event_count(context.game.get())? * NATIVE_EVENT_WORK + 8);
    }
    let mut controls = Vec::with_capacity(raw.controls.len());
    for text in raw.controls {
        persistence::check(stopped)?;
        // Journal replacement is not a supported command. Reject it before its
        // embedded canonical record could trigger expensive native validation.
        #[derive(Deserialize)]
        struct CommandProbe {
            command: Option<Box<RawValue>>,
        }
        #[derive(Deserialize)]
        struct KindProbe {
            kind: String,
        }
        let probe: CommandProbe = serde_json::from_str(text.get()).map_err(bounded)?;
        if let Some(command) = probe.command {
            let kind: KindProbe = serde_json::from_str(command.get()).map_err(bounded)?;
            if matches!(kind.kind.as_str(), "replace" | "chess") {
                return Err("Unsupported shogi journal command; native record preserved.".into());
            }
        }
        let control: ShogiControl = serde_json::from_str(text.get()).map_err(bounded)?;
        work = work.saturating_add(match control.command {
            Some(
                SessionOperation::Undo {}
                | SessionOperation::TakeBack {}
                | SessionOperation::Inspect { .. },
            ) => s::MAX_RECORD_EVENTS * NATIVE_EVENT_WORK,
            _ => 1,
        });
        controls.push(control);
    }
    if work > MAX_REPLAY_ACTIONS {
        return Err(resource_refusal(
            vec![resource_fact(
                ResourceCategory::ReplayWork,
                work,
                MAX_REPLAY_ACTIONS,
                0,
                ResourceScope::Shogi,
                ResourceUnit::ReplayUnits,
            )],
            "Application capacity: Saved journal exceeds16,000 replay actions.",
        ));
    }
    persistence::check(stopped)?;
    let initial = serde_json::from_str(raw.initial.get()).map_err(bounded)?;
    persistence::check(stopped)?;
    let current = serde_json::from_str(raw.current.get()).map_err(bounded)?;
    let mut observations = Vec::with_capacity(raw.observations.len());
    for text in raw.observations {
        persistence::check(stopped)?;
        observations.push(serde_json::from_str(text.get()).map_err(bounded)?);
    }
    persistence::check(stopped)?;
    Ok(ShogiArchive {
        initial,
        current,
        human: raw.human,
        initial_settings: raw.initial_settings,
        settings: raw.settings,
        controls,
        observations,
        retired_execution_floor: raw.retired_execution_floor,
    })
}
fn decode_shogi(
    payload: &str,
    stopped: &mut impl FnMut() -> bool,
) -> Result<(ShogiSession, u64), OperationFailure> {
    let saved = parse_shogi_archive(payload, stopped)?;
    let NativeSide::Shogi(human) = saved.human else {
        return Err("Wrong game: shogi archive human side is chess.".into());
    };
    if saved.retired_execution_floor >= MAX_SAFE_ID - 64 {
        return Err(resource_refusal(
            vec![resource_fact(
                ResourceCategory::IdentityHeadroom,
                saved.retired_execution_floor as usize,
                MAX_SAFE_ID as usize,
                64,
                ResourceScope::Shogi,
                ResourceUnit::Identities,
            )],
            "Saved identity floor leaves no safe runtime headroom.",
        ));
    }
    saved.settings.validate()?;
    saved.initial_settings.validate()?;
    let mut state = ShogiSession::restored(native(&saved.initial)?, human)?;
    state.settings = saved.initial_settings.clone();
    state.initial_settings = saved.initial_settings;
    if saved.controls.len() > MAX_ARCHIVE_CONTROLS
        || saved.observations.len() > MAX_ARCHIVE_OBSERVATIONS
    {
        return Err(document_count_failure(
            saved.controls.len(),
            saved.observations.len(),
            0,
            false,
            ResourceScope::Shogi,
            0,
        ));
    }
    let mut work = state.initial_record.event_count() * NATIVE_EVENT_WORK;
    for (index, control) in saved.controls.into_iter().enumerate() {
        persistence::check(stopped)?;
        if control.sequence != index + 1
            || usize::from(control.command.is_some())
                + usize::from(control.observation.is_some())
                + usize::from(control.close_pause)
                != 1
        {
            return Err("Corrupt shogi control journal sequence/type.".into());
        }
        if let Some(operation) = &control.command {
            work = work.saturating_add(match operation {
                SessionOperation::Undo {}
                | SessionOperation::TakeBack {}
                | SessionOperation::Inspect { .. } => state.game.events().len() * NATIVE_EVENT_WORK,
                _ => 1,
            });
            if work > MAX_REPLAY_ACTIONS {
                return Err(resource_refusal(
                    vec![resource_fact(
                        ResourceCategory::ReplayWork,
                        work,
                        MAX_REPLAY_ACTIONS,
                        0,
                        ResourceScope::Shogi,
                        ResourceUnit::ReplayUnits,
                    )],
                    "Application capacity: Saved journal exceeds16,000 replay actions.",
                ));
            }
            state.apply_command(operation.clone())?;
        } else if let Some(observation) = control.observation {
            if observation != state.observations.len() {
                return Err("Corrupt observation/control ordering.".into());
            }
            let archived = saved
                .observations
                .get(observation)
                .ok_or("Corrupt missing observation.")?;
            work = work.saturating_add(
                archived.observation.context.game.event_count() * NATIVE_EVENT_WORK + 8,
            );
            if work > MAX_REPLAY_ACTIONS {
                return Err(resource_refusal(
                    vec![resource_fact(
                        ResourceCategory::ReplayWork,
                        work,
                        MAX_REPLAY_ACTIONS,
                        0,
                        ResourceScope::Shogi,
                        ResourceUnit::ReplayUnits,
                    )],
                    "Application capacity: Saved observations exceed16,000 replay actions.",
                ));
            }
            if [
                archived.observation.context.request_id,
                archived.observation.context.session_version,
                archived.observation.context.process_generation,
            ]
            .iter()
            .any(|id| *id > saved.retired_execution_floor)
            {
                return Err("Corrupt observation: identity exceeds retired evidence floor.".into());
            }
            if let ExecutionPurpose::MoveAnalysis { target } = &archived.observation.context.purpose
                && target.selection_id > saved.retired_execution_floor
            {
                return Err("Corrupt observation selection exceeds retired floor.".into());
            }
            state.apply_response(archived.observation.clone())?;
            let admitted = state.observations.last_mut().unwrap();
            if admitted.application_error != archived.application_error {
                return Err("Corrupt observation application-admission evidence.".into());
            }
            admitted.historical = archived.historical;
        } else {
            state.retire();
            state.paused = true;
            state.durable_needs_stop = false;
        }
        state.controls.push(control);
    }
    if state.observations.len() != saved.observations.len() {
        return Err("Corrupt unreferenced observation.".into());
    }
    if s::GameRecord::from_game(&state.game) != *native(&saved.current)?
        || state.settings != saved.settings
    {
        return Err("Corrupt archive: native replay disagrees with saved record/settings.".into());
    }
    check_capacity(&state, stopped)?;
    Ok((state, saved.retired_execution_floor))
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
        NativeWork::Xiangqi(work) => xiangqi::archive::execute(work, stopped),
        NativeWork::Admit {
            candidate,
            command,
            response,
            replacement,
            floor,
        } => {
            let mut candidate = match candidate {
                ShogiSeed::Current(state) => state,
                ShogiSeed::Record {
                    record,
                    human,
                    orient_human,
                } => {
                    let mut state = ShogiSession::restored(&record, human)?;
                    if orient_human {
                        state.settings.orientation = human;
                        state.initial_settings = state.settings.clone();
                    }
                    Box::new(state)
                }
            };
            let mut analysis = None;
            if let Some(command) = command {
                analysis = candidate.apply_command(command.clone())?;
                candidate.control(Some(command), None, false);
            }
            if let Some(response) = response {
                let index = candidate.observations.len();
                candidate.apply_response(*response)?;
                candidate.control(None, Some(index), false);
            }
            // Check raw encoded context bounds as well as archive composition.
            if floor >= MAX_SAFE_ID - 64 {
                return Err(resource_refusal(
                    vec![resource_fact(
                        ResourceCategory::IdentityHeadroom,
                        floor as usize,
                        (MAX_SAFE_ID - 65) as usize,
                        0,
                        ResourceScope::Application,
                        ResourceUnit::Identities,
                    )],
                    "Application capacity: runtime identity headroom exhausted.",
                ));
            }
            check_capacity(&candidate, stopped)?;
            Ok(NativeCompleted::Admitted {
                state: candidate,
                analysis,
                replacement,
            })
        }
        NativeWork::Save {
            candidate,
            path,
            revision,
            identity,
            floor,
        } => {
            check_capacity(&candidate, stopped)?;
            let bytes = encode(&candidate, floor)?;
            if bytes.len() > MAX_ARCHIVE_BYTES {
                return Err(resource_refusal(
                    vec![resource_fact(
                        ResourceCategory::EncodedDocumentBytes,
                        bytes.len(),
                        MAX_ARCHIVE_BYTES,
                        0,
                        ResourceScope::Shogi,
                        ResourceUnit::Bytes,
                    )],
                    "Application capacity: complete archive exceeds16MiB.",
                ));
            }
            persistence::atomic_write(&path, &bytes, stopped)?;
            Ok(NativeCompleted::Saved {
                path,
                revision,
                identity,
            })
        }
        NativeWork::Open { path, token } => {
            let bytes = read(&path, stopped)?;
            let value = crate::document::DocumentEnvelope::parse(&bytes)?;
            let game = match value.game_kind {
                GameKind::Chess => OpenedNative::Chess(Box::new(
                    persistence::decode_chess_payload(value.payload.get().as_bytes(), stopped)?,
                )),
                GameKind::Shogi => {
                    let (state, floor) = decode_shogi(value.payload.get(), stopped)?;
                    OpenedNative::Shogi {
                        state: Box::new(state),
                        floor,
                    }
                }
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
        let settings = if let Some(state) = self.session.xiangqi() {
            NativeDesktopSettings::Xiangqi {
                orientation: state.settings.orientation,
                analysis_defaults: state.settings.analysis_defaults,
            }
        } else if let Some(state) = self.session.shogi() {
            NativeDesktopSettings::Shogi {
                settings: state.settings.clone(),
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
        let Some(state) = self.session.shogi() else {
            return self.queue_save(path);
        };
        let candidate = Box::new(state.clone());
        let revision = self.session.document_revision;
        let identity = self.session.document_identity;
        let floor = state.execution_floor;
        let id = self.next_native_io(IoPhase::Saving)?;
        self.files.submit(
            id,
            Work::Native(Box::new(NativeWork::Save {
                candidate,
                path,
                revision,
                identity,
                floor,
            })),
        );
        Ok(id)
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
            NativeCompleted::Admitted { .. } | NativeCompleted::Xiangqi(_) => {
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
    pub(crate) fn queue_native_candidate(
        &mut self,
        candidate: ShogiSession,
        command: Option<SessionOperation>,
        response: Option<ExecutionObservation>,
        replacement: bool,
    ) -> Result<AdmissionReceipt, String> {
        self.queue_shogi_seed(
            ShogiSeed::Current(Box::new(candidate)),
            command,
            response,
            replacement,
        )
    }
    pub(crate) fn queue_shogi_seed(
        &mut self,
        candidate: ShogiSeed,
        command: Option<SessionOperation>,
        response: Option<ExecutionObservation>,
        replacement: bool,
    ) -> Result<AdmissionReceipt, String> {
        self.ensure_runtime_capacity()?;
        if self.session.capacity.id >= MAX_SAFE_ID - 64 {
            return Err(self.identity_refusal(
                self.session.capacity.id,
                MAX_SAFE_ID - 65,
                false,
                ResourceMeasurement::Accepted,
                "Application capacity: admission identity headroom exhausted.",
            ));
        }
        let preferences = matches!(command, Some(SessionOperation::ShogiPreferences { .. }));
        let rollback = self.session.shogi().map(|s| {
            (
                s.paused,
                s.phase,
                s.diagnostic.clone(),
                s.analysis.clone(),
                s.pending.is_some() || s.analysis_intent.is_some(),
            )
        });
        self.retire_admission();
        self.shared.rollback = rollback;
        if replacement {
            self.retire_native_files()?;
        }
        if !preferences {
            if let Some(state) = self.session.xiangqi_mut() {
                state.retire();
                state.paused = true;
            }
            self.bridge.cancel();
            if let Some(state) = self.session.shogi_mut() {
                state.retire();
                state.paused = true;
            } else {
                self.session.retire_execution();
            }
        }
        self.session.capacity.id += 1;
        self.session.capacity.status = CapacityPhase::Checking;
        self.session.capacity.message = None;
        self.session.capacity.resource = None;
        self.session.revision += 1;
        let receipt = AdmissionReceipt::pending();
        self.admission_receipt = Some(receipt.clone());
        self.admission_token = Some(self.open_token());
        self.shared.admission = Some((self.session.document_epoch, preferences));
        let floor = self.runtime_floor();
        self.admissions.submit(
            self.session.capacity.id,
            Work::Native(Box::new(NativeWork::Admit {
                candidate,
                command,
                response: response.map(Box::new),
                replacement,
                floor,
            })),
        );
        Ok(receipt)
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
        let Some((epoch, preferences)) = self.shared.admission.take() else {
            return;
        };
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
                NativeCompleted::Xiangqi(completed) => {
                    self.finish_xiangqi_admission(completed, id, receipt)
                }
                NativeCompleted::Admitted {
                    state,
                    analysis,
                    replacement,
                } => {
                    self.session.install_shogi(state);
                    if preferences {
                        self.session.document_revision += 1;
                        self.session.revision += 1;
                        self.session.archive_revision = self.session.revision.min(MAX_SAFE_ID - 65);
                    } else {
                        self.shared_changed();
                    }
                    if replacement {
                        self.session.document_epoch += 1;
                        self.session.document_identity += 1;
                        self.session.document_path = None;
                        self.session.saved_revision = 0;
                    }
                    self.session.capacity = CapacityState {
                        id,
                        ..CapacityState::default()
                    };
                    if let Some(limits) = analysis
                        && let Err(e) = self.start_shogi(limits, true)
                    {
                        self.pause_native_failure(e);
                    }
                    if let Some(r) = receipt {
                        r.finish(Ok(()));
                    }
                }
                _ => unreachable!("dedicated admission worker"),
            },
            Ok(Completed::Admitted(mut candidate)) => {
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
                if error.resource.is_some() {
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
                        (self.session.shogi_mut(), rollback.clone())
                    {
                        if !had_work || preferences {
                            state.paused = paused;
                            state.phase = phase;
                            state.diagnostic = diagnostic;
                            state.analysis = analysis;
                        } else {
                            state.diagnostic = Some(
                                "Command rejected; prior execution was retired. Resume explicitly."
                                    .into(),
                            );
                        }
                    }
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
            }
        }
    }
    pub(crate) fn pause_native_capacity(&mut self, error: String) {
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

#[cfg(test)]
#[path = "shared_capacity_tests.rs"]
mod capacity_tests;
