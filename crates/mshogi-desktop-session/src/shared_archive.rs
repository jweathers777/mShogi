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
pub(super) fn required_option<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    d: D,
) -> Result<Option<T>, D::Error> {
    Option::<T>::deserialize(d)
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ArchiveV2 {
    schema_version: u32,
    format: String,
    game_kind: String,
    rules_profile: String,
    profile_edition: String,
    payload: Box<RawValue>,
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
pub(crate) enum NativeWork {
    Xiangqi(xiangqi::archive::XiangqiWork),
    Admit {
        candidate: Box<ShogiSession>,
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
        allow_xiangqi: bool,
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
fn capacity(message: &str) -> String {
    format!(
        "Application capacity: {message} No canonical history or observation was discarded. Save the complete current document, then start a new document to continue."
    )
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
    let payload = serde_json::value::to_raw_value(&archive(state, floor)).map_err(bounded)?;
    let envelope = ArchiveV2 {
        schema_version: 2,
        format: "mshogi.desktop.archive.v2".into(),
        game_kind: "shogi".into(),
        rules_profile: s::PROFILE_ID.into(),
        profile_edition: s::PROFILE_EDITION.into(),
        payload,
    };
    serde_json::to_vec_pretty(&envelope).map_err(bounded)
}
fn needs_stop(state: &ShogiSession) -> bool {
    state.pending.is_some() || state.analysis_intent.is_some() || !state.paused
}

/// Bound total replay, including semantic Undo reconstruction, independently of wall time.
const MAX_REPLAY_ACTIONS: usize = 16_000;
// Native event admission/reconstruction costs more than a cheap desktop control.
const NATIVE_EVENT_WORK: usize = 4;
fn counts(state: &ShogiSession, reserve: bool) -> Result<(), String> {
    if state.controls.len() + usize::from(reserve) > MAX_ARCHIVE_CONTROLS
        || state.observations.len() > MAX_ARCHIVE_OBSERVATIONS
    {
        return Err(capacity(
            "The supported shogi archive retains256 observations and8192 controls, including a stop/close reservation.",
        ));
    }
    if state.game.events().len() > s::MAX_RECORD_EVENTS
        || state.initial_record.event_count() > s::MAX_RECORD_EVENTS
    {
        return Err(capacity(
            "Shogi history exceeds its native321-event parser bound.",
        ));
    }
    Ok(())
}
fn check_capacity(state: &ShogiSession, stopped: &mut impl FnMut() -> bool) -> Result<(), String> {
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
        return Err(capacity(
            "The complete shogi archive would exceed16MiB, including identity and stop/close headroom.",
        ));
    }
    persistence::check(stopped)?;
    replay_work(&state)?;
    Ok(())
}
fn replay_work(state: &ShogiSession) -> Result<(), String> {
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
        Err(capacity(
            "The document exceeds the bounded16,000 native replay-action work budget.",
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
        check_capacity(self, &mut || false)
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
    fn apply_command(&mut self, operation: SessionOperation) -> Result<Option<WorkLimits>, String> {
        if matches!(
            operation,
            SessionOperation::Analyze { .. }
                | SessionOperation::Resume {}
                | SessionOperation::Retry {}
                | SessionOperation::Play { .. }
        ) && self.observations.len() >= MAX_ARCHIVE_OBSERVATIONS
        {
            return Err(capacity(
                "All256 observation slots are retained; no new work was started.",
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
    fn apply_response(&mut self, response: ExecutionObservation) -> Result<(), String> {
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
            return Err(capacity(
                "Encoded observation context/result exceeds native transport limits.",
            ));
        }
        if !response.matches(&response.context) {
            return Err("Corrupt observation: invalid native context/result metadata.".into());
        }
        if self.observations.len() >= MAX_ARCHIVE_OBSERVATIONS {
            return Err(capacity("No observation slot remains."));
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
fn raw_event_count(text: &str) -> Result<usize, String> {
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
        return Err(capacity("Native shogi event bound exceeded."));
    }
    Ok(ledger.events.len())
}
fn parse_shogi_archive(
    payload: &str,
    stopped: &mut impl FnMut() -> bool,
) -> Result<ShogiArchive, String> {
    let raw: RawShogiArchive = serde_json::from_str(payload)
        .map_err(|e| format!("Corrupt shogi archive: {}", bounded(e)))?;
    if raw.controls.len() > MAX_ARCHIVE_CONTROLS
        || raw.observations.len() > MAX_ARCHIVE_OBSERVATIONS
    {
        return Err(capacity(
            "Saved control/observation counts exceed supported limits.",
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
        return Err(capacity("Saved journal exceeds16,000 replay actions."));
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
) -> Result<(ShogiSession, u64), String> {
    let saved = parse_shogi_archive(payload, stopped)?;
    let NativeSide::Shogi(human) = saved.human else {
        return Err("Wrong game: shogi archive human side is chess.".into());
    };
    if saved.retired_execution_floor >= MAX_SAFE_ID - 64 {
        return Err(capacity(
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
        return Err(capacity(
            "Saved shogi control/observation counts exceed supported limits.",
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
                return Err(capacity("Saved journal exceeds16,000 replay actions."));
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
                return Err(capacity("Saved observations exceed16,000 replay actions."));
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
fn version(bytes: &[u8]) -> Result<u32, String> {
    #[derive(Deserialize)]
    struct Header {
        schema_version: u32,
    }
    serde_json::from_slice::<Header>(bytes)
        .map(|h| h.schema_version)
        .map_err(|e| format!("Corrupt archive header: {}", bounded(e)))
}
fn envelope(bytes: &[u8]) -> Result<ArchiveV2, String> {
    let value: ArchiveV2 = serde_json::from_slice(bytes)
        .map_err(|e| format!("Corrupt tagged archive: {}", bounded(e)))?;
    if value.schema_version != 2 || value.format != "mshogi.desktop.archive.v2" {
        return Err("Unsupported desktop archive version/format.".into());
    }
    let profile = match value.game_kind.as_str() {
        "chess" => RulesProfile::ChessMatchV1,
        "shogi" => RulesProfile::ShogiCsa36V1,
        "xiangqi" => RulesProfile::XiangqiWxf2018V1,
        other => return Err(format!("Unsupported archive game: {}.", bounded(other))),
    };
    if value.rules_profile != profile.id() || value.profile_edition != profile.edition() {
        return Err(
            "Wrong profile/edition: archive identity disagrees with its explicit game.".into(),
        );
    }
    Ok(value)
}
pub(crate) fn decode_chess_compatible(
    bytes: &[u8],
    stopped: &mut impl FnMut() -> bool,
) -> Result<persistence::Opened, String> {
    match version(bytes)? {
        1 | 3 => persistence::decode_chess_v1(bytes, stopped),
        2 => {
            let value = envelope(bytes)?;
            if value.game_kind != "chess" {
                return Err("Wrong game: the legacy chess document consumer cannot install shogi; use the shared native document consumer.".into());
            }
            persistence::decode_chess_v1(value.payload.get().as_bytes(), stopped)
        }
        _ => Err("Unsupported desktop archive version.".into()),
    }
}
fn read(path: &std::path::Path, stopped: &mut impl FnMut() -> bool) -> Result<Vec<u8>, String> {
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
            return Err(capacity("The encoded desktop archive exceeds16MiB."));
        }
        bytes.extend_from_slice(&buffer[..n]);
    }
    Ok(bytes)
}
pub(crate) fn execute(
    work: NativeWork,
    stopped: &mut impl FnMut() -> bool,
) -> Result<NativeCompleted, String> {
    let result = (|| match work {
        NativeWork::Xiangqi(work) => xiangqi::archive::execute(work, stopped),
        NativeWork::Admit {
            mut candidate,
            command,
            response,
            replacement,
            floor,
        } => {
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
                return Err(capacity("Runtime identity headroom exhausted."));
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
                return Err(capacity("The complete archive exceeds16MiB."));
            }
            persistence::atomic_write(&path, &bytes, stopped)?;
            Ok(NativeCompleted::Saved {
                path,
                revision,
                identity,
            })
        }
        NativeWork::Open {
            path,
            token,
            allow_xiangqi,
        } => {
            let bytes = read(&path, stopped)?;
            let game = match version(&bytes)? {
                1 | 3 => {
                    OpenedNative::Chess(Box::new(persistence::decode_chess_v1(&bytes, stopped)?))
                }
                2 => {
                    let value = envelope(&bytes)?;
                    match value.game_kind.as_str() {
                        "chess" => OpenedNative::Chess(Box::new(persistence::decode_chess_v1(
                            value.payload.get().as_bytes(),
                            stopped,
                        )?)),
                        "shogi" => {
                            let (state, floor) = decode_shogi(value.payload.get(), stopped)?;
                            OpenedNative::Shogi {
                                state: Box::new(state),
                                floor,
                            }
                        }
                        "xiangqi" => {
                            if !allow_xiangqi {
                                return Err("Xiangqi product UI is not available yet; current document preserved. Use the native archive consumer.".into());
                            }
                            let (state, floor) =
                                xiangqi::archive::decode(value.payload.get(), stopped)?;
                            OpenedNative::Xiangqi {
                                state: Box::new(state),
                                floor,
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                _ => return Err("Unsupported desktop archive version.".into()),
            };
            persistence::check(stopped)?;
            Ok(NativeCompleted::Opened { game, path, token })
        }
    })();
    result.map_err(bounded)
}
impl DesktopController {
    /// Stable shared replacement token; callers confirm unsaved changes against it.
    pub fn shared_open_token(&self) -> SharedOpenToken {
        SharedOpenToken {
            document: self.shared.epoch,
            version: self.session.version,
            document_revision: self.session.document_revision,
        }
    }
    pub(crate) fn shared_document(&self) -> SharedDocumentState {
        let settings = if let Some(state) = &self.shared.xiangqi {
            NativeDesktopSettings::Xiangqi {
                orientation: state.settings.orientation,
                analysis_defaults: state.settings.analysis_defaults,
            }
        } else if let Some(state) = &self.shared.shogi {
            NativeDesktopSettings::Shogi {
                settings: state.settings.clone(),
            }
        } else {
            NativeDesktopSettings::Chess {
                settings: self.session.settings.clone(),
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
    /// Queue a complete native document. Legacy chess continues to write strict v1.
    pub fn queue_shared_save(&mut self, path: PathBuf) -> Result<u64, String> {
        if let Some(state) = &self.shared.xiangqi {
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
        let Some(state) = &self.shared.shogi else {
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
    /// Queue strict v1/v2 native dispatch. Open never installs resumable work.
    pub fn queue_shared_open(
        &mut self,
        path: PathBuf,
        token: SharedOpenToken,
    ) -> Result<u64, String> {
        self.queue_open_admission(path, token, true)
    }
    /// Product Open uses the same strict native dispatch and stopped restoration.
    pub fn queue_product_open(
        &mut self,
        path: PathBuf,
        token: SharedOpenToken,
    ) -> Result<u64, String> {
        self.queue_open_admission(path, token, true)
    }
    fn queue_open_admission(
        &mut self,
        path: PathBuf,
        token: SharedOpenToken,
        allow_xiangqi: bool,
    ) -> Result<u64, String> {
        if token != self.shared_open_token() {
            return Err("Stale Open: review the current document again.".into());
        }
        let id = self.next_native_io(IoPhase::Opening)?;
        self.files.submit(
            id,
            Work::Native(Box::new(NativeWork::Open {
                path,
                token,
                allow_xiangqi,
            })),
        );
        Ok(id)
    }
    pub(crate) fn retire_native_files(&mut self) -> Result<(), String> {
        self.next_native_io(IoPhase::Idle)?;
        self.files.cancel();
        Ok(())
    }
    fn next_native_io(&mut self, phase: IoPhase) -> Result<u64, String> {
        if self.io_state.id >= MAX_SAFE_ID - 1 || self.session.revision >= MAX_SAFE_ID - 2 {
            return Err(capacity("File-operation identity headroom exhausted."));
        }
        self.io_state = IoState {
            id: self.io_state.id + 1,
            phase,
            message: None,
        };
        self.session.revision += 1;
        Ok(self.io_state.id)
    }
    pub(super) fn runtime_floor(&self) -> u64 {
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
                match game {
                    OpenedNative::Xiangqi { state, floor } => {
                        self.install_xiangqi(state, floor, path)
                    }
                    OpenedNative::Chess(opened) => self.install_chess_archive(*opened, path),
                    OpenedNative::Shogi { mut state, floor } => {
                        let next = self.runtime_floor().max(floor) + 1;
                        if next >= MAX_SAFE_ID - 64
                            || self.shared.epoch >= MAX_SAFE_ID - 64
                            || self.session.document_identity >= MAX_SAFE_ID - 65
                            || self.session.document_revision >= MAX_SAFE_ID - 65
                        {
                            return Err(capacity(
                                "Reopened runtime identities leave no safe headroom.",
                            ));
                        }
                        self.retire_admission();
                        self.bridge.cancel();
                        state.retire();
                        state.execution_floor = floor;
                        state.paused = true;
                        for o in &mut state.observations {
                            o.historical = true;
                        }
                        state.diagnostic=Some("Document reopened with work stopped. Saved observations are historical; Resume explicitly.".into());
                        self.shared.xiangqi = None;
                        self.shared.shogi = Some(*state);
                        self.shared.epoch += 1;
                        self.session.pending = None;
                        self.session.retire_analysis();
                        self.session.version = next;
                        self.session.revision = next;
                        self.session.next_request = next;
                        self.session.selection_id = next;
                        self.session.document_identity += 1;
                        self.session.document_revision += 1;
                        self.session.saved_revision = self.session.document_revision;
                        self.session.document_path = Some(path);
                        self.session.capacity = CapacityState::default();
                        Ok("Opened native shogi archivev2 with fresh runtime identities and work stopped.".into())
                    }
                }
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
        self.session.ensure_capacity()?;
        if self.session.capacity.id >= MAX_SAFE_ID - 64 {
            return Err(capacity("Admission identity headroom exhausted."));
        }
        let preferences = matches!(command, Some(SessionOperation::ShogiPreferences { .. }));
        let rollback = self.shared.shogi.as_ref().map(|s| {
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
            if let Some(state) = &mut self.shared.xiangqi {
                state.retire();
                state.paused = true;
            }
            self.bridge.cancel();
            if let Some(state) = &mut self.shared.shogi {
                state.retire();
                state.paused = true;
            } else {
                self.session.pending = None;
                self.session.retire_analysis();
                self.session.play_paused = true;
                self.session.engine.status = EnginePhase::Paused;
            }
        }
        self.session.capacity.id += 1;
        self.session.capacity.status = CapacityPhase::Checking;
        self.session.capacity.message = None;
        self.session.revision += 1;
        let receipt = AdmissionReceipt::pending();
        self.admission_receipt = Some(receipt.clone());
        self.admission_token = Some(self.open_token());
        self.shared.admission = Some((self.shared.epoch, preferences));
        let floor = self.runtime_floor();
        self.admissions.submit(
            self.session.capacity.id,
            Work::Native(Box::new(NativeWork::Admit {
                candidate: Box::new(candidate),
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
        if token != Some(self.open_token()) || epoch != self.shared.epoch {
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
                    self.shared.xiangqi = None;
                    self.shared.shogi = Some(*state);
                    if preferences {
                        self.session.document_revision += 1;
                        self.session.revision += 1;
                        self.session.archive_revision = self.session.revision.min(MAX_SAFE_ID - 65);
                    } else {
                        self.shared_changed();
                    }
                    if replacement {
                        self.shared.epoch += 1;
                        self.session.document_identity += 1;
                        self.session.document_path = None;
                        self.session.saved_revision = 0;
                        self.session.pending = None;
                        self.session.retire_analysis();
                    }
                    self.session.capacity = CapacityState {
                        id,
                        ..CapacityState::default()
                    };
                    if let Some(limits) = analysis
                        && let Err(e) = self.start_shogi(limits, true)
                    {
                        self.pause_native_capacity(e);
                    }
                    if let Some(r) = receipt {
                        r.finish(Ok(()));
                    }
                }
                _ => unreachable!("dedicated admission worker"),
            },
            Ok(Completed::Admitted(mut candidate)) => {
                self.shared.xiangqi = None;
                self.shared.shogi = None;
                self.shared.epoch += 1;
                candidate.revision = candidate.revision.max(self.session.revision + 1);
                candidate.capacity = CapacityState {
                    id,
                    ..CapacityState::default()
                };
                self.session = *candidate;
                if let Some(r) = receipt {
                    r.finish(Ok(()));
                }
            }
            Err(error) => {
                if error.starts_with("Application capacity:")
                    || error.starts_with("Document operation")
                {
                    self.pause_native_capacity(error.clone());
                } else {
                    self.session.capacity = CapacityState {
                        id,
                        ..CapacityState::default()
                    };
                    if let (Some(state), Some((paused, phase, diagnostic, analysis, had_work))) =
                        (&mut self.shared.shogi, rollback.clone())
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
                        (&mut self.shared.xiangqi, rollback)
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
                    r.finish(Err(error));
                }
            }
            _ => unreachable!("native admission completion"),
        }
    }
    pub(crate) fn pause_native_capacity(&mut self, error: String) {
        self.bridge.cancel();
        if let Some(state) = &mut self.shared.shogi {
            state.retire();
            state.paused = true;
        }
        if let Some(state) = &mut self.shared.xiangqi {
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
