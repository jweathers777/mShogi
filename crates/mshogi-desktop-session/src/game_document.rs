//! Generic native document in the existing desktop envelope (MSH-217). No
//! runtime or file owner. Each game supplies its native calls through
//! [`NativeGame`]; the journal, observations, branches and capacity
//! accounting are the same for every game that uses this module.
use crate::native_game::{NativeFailure, NativeGame, NativeResource, RecordFailure};
use crate::{document, *};
use mshogi_core::native::{CanonicalRecord, RulesProfile};
use mshogi_engine::{ExecutionObservation, ExecutionPurpose, ExecutionResult};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::marker::PhantomData;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
/// Supported document orientation; bounded execution limits are explicit per request.
pub struct GameSettings<S> {
    /// Persisted ownership of the two native participants.
    #[serde(default)]
    pub play_mode: crate::PlayMode,
    /// Native side displayed at the bottom.
    pub orientation: S,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Durable reason for retiring a complete native branch.
pub enum RewindKind {
    /// Retire one native event.
    Undo,
    /// Restore a previous human decision while retaining the full branch.
    TakeBack,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
/// Document journal operations; native commands retain exact complete types.
pub enum GameControl<S, C> {
    /// Admit a complete native command with its native actor.
    Native {
        /// Native participant or administrative actor.
        actor: S,
        /// Complete native command.
        command: C,
    },
    /// Persist presentation settings without modifying rules.
    Settings {
        /// Native orientation.
        settings: GameSettings<S>,
    },
    /// Retain the abandoned branch and restore an exact event prefix.
    Rewind {
        /// Exact native event prefix.
        event_index: usize,
        /// Human intent for retirement.
        reason: RewindKind,
    },
    /// Reference the next exact-context historical observation.
    Observation {
        /// Sequential observation ledger index.
        index: usize,
    },
    /// Record native application closure, never a game conclusion.
    Close {},
}
/// The settings of game `G`.
pub type SettingsOf<G> = GameSettings<<G as NativeGame>::Side>;
/// The journal control of game `G`.
pub type ControlOf<G> = GameControl<<G as NativeGame>::Side, <G as NativeGame>::Command>;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct Retired {
    pub(crate) control_index: usize,
    pub(crate) record: CanonicalRecord,
}
#[derive(Serialize)]
#[serde(bound(serialize = ""))]
pub(crate) struct Payload<G: NativeGame> {
    pub(crate) mode: String,
    pub(crate) initial: CanonicalRecord,
    pub(crate) current: CanonicalRecord,
    pub(crate) human: G::Side,
    pub(crate) initial_settings: SettingsOf<G>,
    pub(crate) settings: SettingsOf<G>,
    pub(crate) controls: Vec<ControlOf<G>>,
    pub(crate) observations: Vec<ExecutionObservation>,
    pub(crate) retired: Vec<Retired>,
    pub(crate) retired_execution_floor: u64,
    pub(crate) selected_event: Option<usize>,
}
impl<G: NativeGame> Clone for Payload<G> {
    fn clone(&self) -> Self {
        Self {
            mode: self.mode.clone(),
            initial: self.initial.clone(),
            current: self.current.clone(),
            human: self.human,
            initial_settings: self.initial_settings.clone(),
            settings: self.settings.clone(),
            controls: self.controls.clone(),
            observations: self.observations.clone(),
            retired: self.retired.clone(),
            retired_execution_floor: self.retired_execution_floor,
            selected_event: self.selected_event,
        }
    }
}
#[derive(Clone, Debug, Serialize)]
/// Measured whole-document usage, including repeated contexts.
pub struct GameDocumentUsage {
    /// Actual pretty-encoded envelope bytes.
    pub encoded_bytes: usize,
    /// All native events retained in records, branches and observations.
    pub retained_events: usize,
    /// Durable document journal entries.
    pub controls: usize,
    /// Retained historical execution results.
    pub observations: usize,
    /// Supported complete decode and runtime reconstruction path, not elapsed time.
    pub replay_work: u64,
    /// Actual additional complete Stop/Resign/Close envelope bytes reserved.
    pub reserved_bytes: usize,
    /// Additional complete closing and reopen allowance.
    pub reserved_work: u64,
}
#[derive(Clone, Debug)]
/// Typed bounded document refusal, separate from a game outcome.
pub struct GameDocumentError {
    /// Bounded technical diagnostic.
    pub diagnostic: String,
    /// Authoritative capacity facts when known.
    pub resource: Option<ResourceFailure>,
    /// Cooperative cancellation/deadline interruption; worker classifies elapsed deadline separately.
    pub stopped: bool,
}
impl fmt::Display for GameDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.diagnostic)
    }
}
impl std::error::Error for GameDocumentError {}
impl From<OperationFailure> for GameDocumentError {
    fn from(e: OperationFailure) -> Self {
        Self {
            diagnostic: e.diagnostic,
            resource: e.resource,
            stopped: false,
        }
    }
}
impl From<GameDocumentError> for OperationFailure {
    fn from(e: GameDocumentError) -> Self {
        Self {
            diagnostic: e.diagnostic,
            resource: e.resource,
        }
    }
}

/// Error helpers of game `G`: diagnostics name the game and resources its scope.
pub(crate) struct Errors<G>(PhantomData<G>);
impl<G: NativeGame> Errors<G> {
    fn invalid(e: impl fmt::Display) -> GameDocumentError {
        GameDocumentError {
            diagnostic: document::diagnostic(format!("Invalid {} document: {e}", G::NAME)),
            resource: None,
            stopped: false,
        }
    }
    fn stopped() -> GameDocumentError {
        GameDocumentError {
            diagnostic: format!("{} document operation stopped before acceptance.", G::NAME),
            resource: None,
            stopped: true,
        }
    }
    fn check(stopped: &mut impl FnMut() -> bool) -> Result<(), GameDocumentError> {
        if stopped() {
            Err(Self::stopped())
        } else {
            Ok(())
        }
    }
    fn unknown_resource(diagnostic: String) -> GameDocumentError {
        OperationFailure::resource(ResourceFailure::new(
            ResourceFact::unknown().with_scope(G::SCOPE),
            diagnostic,
        ))
        .into()
    }
    fn limit(
        category: ResourceCategory,
        count: u64,
        cap: u64,
        reserve: u64,
        unit: ResourceUnit,
    ) -> GameDocumentError {
        OperationFailure::resource(ResourceFailure::new(
            ResourceFact::measured(category, count, cap)
                .with_reserved(reserve)
                .with_scope(G::SCOPE)
                .with_unit(unit),
            "Application limit reached. The complete previously accepted game can still be saved.",
        ))
        .into()
    }
    fn canonical(error: mshogi_core::native::EnvelopeError) -> GameDocumentError {
        match error {
            mshogi_core::native::EnvelopeError::Stopped => Self::stopped(),
            mshogi_core::native::EnvelopeError::Capacity => {
                Self::unknown_resource(error.to_string())
            }
            error => Self::invalid(error),
        }
    }
    fn record(error: RecordFailure) -> GameDocumentError {
        match error {
            RecordFailure::Stopped => Self::stopped(),
            RecordFailure::Capacity(diagnostic) => Self::unknown_resource(diagnostic),
            RecordFailure::Invalid(diagnostic) => Self::invalid(diagnostic),
        }
    }
    fn native(error: NativeFailure) -> GameDocumentError {
        let (resource, diagnostic) = match error {
            NativeFailure::Stopped => return Self::stopped(),
            NativeFailure::Invalid(diagnostic) => return Self::invalid(diagnostic),
            NativeFailure::Capacity(resource, diagnostic) => (resource, diagnostic),
        };
        let (category, cap, unit) = match resource {
            NativeResource::EncodedBytes => (
                ResourceCategory::CanonicalRecordBytes,
                G::MAX_RECORD_BYTES,
                ResourceUnit::Bytes,
            ),
            NativeResource::Events => (
                ResourceCategory::NativeEvents,
                G::MAX_EVENTS,
                ResourceUnit::Events,
            ),
            NativeResource::ReplayWork => (
                ResourceCategory::ReplayWork,
                G::MAX_REPLAY_WORK,
                ResourceUnit::ReplayUnits,
            ),
            // Native Counter covers both declared/accepted board counters
            // and runtime identities. It cannot identify one specific limit.
            NativeResource::Counter => return Self::unknown_resource(diagnostic),
        };
        let mut fact = ResourceFact::measured(category, 0, cap)
            .with_scope(G::SCOPE)
            .with_unit(unit);
        fact.count = None;
        OperationFailure::resource(ResourceFailure::new(fact, diagnostic)).into()
    }
    pub(crate) fn record_of(r: &CanonicalRecord) -> Result<&G::Record, GameDocumentError> {
        G::native(r).ok_or_else(|| Self::invalid(format!("record must be native {}", G::NAME)))
    }
    pub(crate) fn charge(total: &mut u64, n: u64) -> Result<(), GameDocumentError> {
        *total = total.saturating_add(n);
        if *total > G::MAX_DOCUMENT_WORK {
            return Err(Self::limit(
                ResourceCategory::ReplayWork,
                *total,
                G::MAX_DOCUMENT_WORK,
                0,
                ResourceUnit::ReplayUnits,
            ));
        }
        Ok(())
    }
}

fn counts<G: NativeGame>(p: &Payload<G>, bytes: usize) -> Result<(), GameDocumentError> {
    for (category, count, cap, unit) in [
        (
            ResourceCategory::EncodedDocumentBytes,
            bytes as u64,
            MAX_ARCHIVE_BYTES as u64,
            ResourceUnit::Bytes,
        ),
        (
            ResourceCategory::Controls,
            p.controls.len() as u64,
            MAX_ARCHIVE_CONTROLS as u64,
            ResourceUnit::Controls,
        ),
        (
            ResourceCategory::OrdinaryObservations,
            p.observations.len() as u64,
            MAX_ARCHIVE_OBSERVATIONS as u64,
            ResourceUnit::Observations,
        ),
        (
            ResourceCategory::IdentityHeadroom,
            p.retired_execution_floor,
            MAX_SAFE_ID - 65,
            ResourceUnit::Identities,
        ),
    ] {
        if count > cap {
            return Err(Errors::<G>::limit(category, count, cap, 0, unit));
        }
    }
    if p.retired_execution_floor == 0 {
        return Err(Errors::<G>::invalid("zero runtime floor"));
    }
    Ok(())
}

/// An immutable validated candidate. Only DesktopController installs it.
pub struct ValidatedGameDocument<G: NativeGame> {
    pub(crate) payload: Payload<G>,
    pub(crate) bytes: Vec<u8>,
    pub(crate) usage: GameDocumentUsage,
}
impl<G: NativeGame> Clone for ValidatedGameDocument<G> {
    fn clone(&self) -> Self {
        Self {
            payload: self.payload.clone(),
            bytes: self.bytes.clone(),
            usage: self.usage.clone(),
        }
    }
}
impl<G: NativeGame> fmt::Debug for ValidatedGameDocument<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValidatedGameDocument")
            .field("game", &G::NAME)
            .field("usage", &self.usage)
            .finish_non_exhaustive()
    }
}
impl<G: NativeGame> ValidatedGameDocument<G> {
    /// Create a stopped candidate from an authoritative native root/history.
    pub fn new(record: G::Record, human: G::Side) -> Result<Self, GameDocumentError> {
        Self::new_with_stop(record, human, &mut || false)
    }
    pub(crate) fn new_with_stop(
        record: G::Record,
        human: G::Side,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, GameDocumentError> {
        Self::new_with_mode(record, human, crate::PlayMode::default(), stopped)
    }
    pub(crate) fn new_with_mode(
        record: G::Record,
        human: G::Side,
        play_mode: crate::PlayMode,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, GameDocumentError> {
        Errors::<G>::check(stopped)?;
        let settings = GameSettings {
            orientation: human,
            play_mode,
        };
        let record = G::canonical(record);
        validate(
            Payload {
                mode: "untimed".into(),
                initial: record.clone(),
                current: record,
                human,
                initial_settings: settings.clone(),
                settings,
                controls: vec![],
                observations: vec![],
                retired: vec![],
                retired_execution_floor: 1,
                selected_event: None,
            },
            stopped,
        )
    }
    /// Strict raw decode and native replay on the existing bounded worker.
    pub fn decode(
        bytes: &[u8],
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, GameDocumentError> {
        type E<G> = Errors<G>;
        E::<G>::check(stopped)?;
        if bytes.len() > MAX_ARCHIVE_BYTES {
            return Err(E::<G>::limit(
                ResourceCategory::EncodedDocumentBytes,
                bytes.len() as u64,
                MAX_ARCHIVE_BYTES as u64,
                0,
                ResourceUnit::Bytes,
            ));
        }
        document::reject_duplicates(bytes).map_err(E::<G>::invalid)?;
        let e = document::DocumentEnvelope::parse(bytes).map_err(E::<G>::invalid)?;
        if e.game_kind != G::KIND {
            return Err(E::<G>::invalid("wrong desktop game"));
        }
        // Shape/count/identity probes precede every expensive native parser.
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields, bound(deserialize = ""))]
        struct Wire<G: NativeGame> {
            mode: String,
            initial: Box<serde_json::value::RawValue>,
            current: Box<serde_json::value::RawValue>,
            human: G::Side,
            initial_settings: SettingsOf<G>,
            settings: SettingsOf<G>,
            controls: Vec<Box<serde_json::value::RawValue>>,
            observations: Vec<Box<serde_json::value::RawValue>>,
            retired: Vec<Branch>,
            retired_execution_floor: u64,
            #[serde(deserialize_with = "shared::archive::required_option")]
            selected_event: Option<usize>,
        }
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Branch {
            control_index: usize,
            record: Box<serde_json::value::RawValue>,
        }
        let w: Wire<G> = serde_json::from_str(e.payload.get()).map_err(E::<G>::invalid)?;
        if w.controls.len() > MAX_ARCHIVE_CONTROLS {
            return Err(E::<G>::limit(
                ResourceCategory::Controls,
                w.controls.len() as u64,
                MAX_ARCHIVE_CONTROLS as u64,
                0,
                ResourceUnit::Controls,
            ));
        }
        if w.observations.len() > MAX_ARCHIVE_OBSERVATIONS {
            return Err(E::<G>::limit(
                ResourceCategory::OrdinaryObservations,
                w.observations.len() as u64,
                MAX_ARCHIVE_OBSERVATIONS as u64,
                0,
                ResourceUnit::Observations,
            ));
        }
        // A malformed file may consume at most one native-bounded replay beyond
        // the aggregate allowance; stop/deadline checks separate all invocations.
        let mut parsed_work = 0;
        let mut parse = |raw: &serde_json::value::RawValue| {
            E::<G>::check(stopped)?;
            let result = CanonicalRecord::from_json_with_stop(raw.get(), stopped);
            E::<G>::check(stopped)?;
            let r = result.map_err(E::<G>::canonical)?;
            E::<G>::charge(
                &mut parsed_work,
                G::reconstruction_work(E::<G>::record_of(&r)?),
            )?;
            E::<G>::check(stopped)?;
            Ok::<_, GameDocumentError>(r)
        };
        let initial = parse(&w.initial)?;
        let current = parse(&w.current)?;
        if e.rules_profile != G::profile(E::<G>::record_of(&initial)?) {
            return Err(E::<G>::invalid(
                "outer profile does not match native payload profile",
            ));
        }
        let mut retired = vec![];
        for b in w.retired {
            retired.push(Retired {
                control_index: b.control_index,
                record: parse(&b.record)?,
            });
        }
        let mut observations = vec![];
        for raw in w.observations {
            E::<G>::check(stopped)?;
            let result = ExecutionObservation::from_json_with_stop_typed(raw.get(), stopped);
            E::<G>::check(stopped)?;
            let o = result.map_err(|error| match error {
                mshogi_engine::ObservationDecodeError::Stopped => E::<G>::stopped(),
                mshogi_engine::ObservationDecodeError::Native(error) => E::<G>::canonical(error),
                error => E::<G>::invalid(error),
            })?;
            E::<G>::charge(
                &mut parsed_work,
                G::reconstruction_work(E::<G>::record_of(&o.context.game)?),
            )?;
            E::<G>::check(stopped)?;
            observations.push(o);
        }
        let mut controls = Vec::with_capacity(w.controls.len());
        for raw in w.controls {
            E::<G>::check(stopped)?;
            let control: ControlOf<G> = serde_json::from_str(raw.get()).map_err(E::<G>::invalid)?;
            if G::STRICT_CONTROL_FIELDS {
                let input: serde_json::Value =
                    serde_json::from_str(raw.get()).map_err(E::<G>::invalid)?;
                if input != serde_json::to_value(&control).map_err(E::<G>::invalid)? {
                    return Err(E::<G>::invalid(
                        "control has unknown, missing or noncanonical native fields",
                    ));
                }
            }
            controls.push(control);
        }
        validate(
            Payload {
                mode: w.mode,
                initial,
                current,
                human: w.human,
                initial_settings: w.initial_settings,
                settings: w.settings,
                controls,
                observations,
                retired,
                retired_execution_floor: w.retired_execution_floor,
                selected_event: w.selected_event,
            },
            stopped,
        )
    }
    /// Canonical current-envelope bytes for the existing atomic writer.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Authoritative measured capacity accounting.
    pub fn usage(&self) -> &GameDocumentUsage {
        &self.usage
    }
    /// Complete current native record.
    pub fn record(&self) -> &G::Record {
        G::native(&self.payload.current).expect("validated native record")
    }
    /// Complete initial native history and provenance.
    pub fn initial_record(&self) -> &G::Record {
        G::native(&self.payload.initial).expect("validated native record")
    }
    /// Actual immutable profile; never inferred from board dimensions or player side.
    pub fn profile(&self) -> RulesProfile {
        G::profile(self.record())
    }
    /// Recorded native human assignment.
    pub fn human(&self) -> G::Side {
        self.payload.human
    }
    /// Accepted orientation settings.
    pub fn settings(&self) -> &SettingsOf<G> {
        &self.payload.settings
    }
    /// Exact durable control journal.
    pub fn controls(&self) -> &[ControlOf<G>] {
        &self.payload.controls
    }
    /// Historical results with complete target context; never pending work.
    pub fn observations(&self) -> &[ExecutionObservation] {
        &self.payload.observations
    }
    /// Exact currently selected event prefix, independent of board ply.
    pub fn selected_event(&self) -> Option<usize> {
        self.payload.selected_event
    }
    /// Saved high-water evidence; restoration allocates fresh runtime IDs.
    pub fn retired_execution_floor(&self) -> u64 {
        self.payload.retired_execution_floor
    }
    /// Lossless abandoned branches and their rewind journal indices.
    pub fn retired_branches(&self) -> impl Iterator<Item = (usize, &G::Record)> {
        self.payload.retired.iter().map(|b| {
            (
                b.control_index,
                G::native(&b.record).expect("validated native record"),
            )
        })
    }
    /// Validate a complete candidate before replacing this immutable document.
    pub fn with_control(
        &self,
        control: ControlOf<G>,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, GameDocumentError> {
        type E<G> = Errors<G>;
        E::<G>::check(stopped)?;
        let mut p = self.payload.clone();
        let mut game = G::to_game(self.record(), stopped).map_err(E::<G>::record)?;
        E::<G>::check(stopped)?;
        match &control {
            GameControl::Native { actor, command } => {
                G::submit(&mut game, *actor, command.clone(), stopped).map_err(E::<G>::native)?;
            }
            GameControl::Settings { settings } => {
                if settings.play_mode != p.settings.play_mode {
                    return Err(E::<G>::invalid(
                        "participant mode can only be selected for a new game",
                    ));
                }
                p.settings = settings.clone();
            }
            GameControl::Rewind { event_index, .. } => {
                if *event_index >= G::events(&game) {
                    return Err(E::<G>::invalid("rewind must retire an event"));
                }
                p.retired.push(Retired {
                    control_index: p.controls.len(),
                    record: p.current.clone(),
                });
                let prefix =
                    G::record_prefix(&game, *event_index, stopped).map_err(E::<G>::record)?;
                E::<G>::check(stopped)?;
                game = G::to_game(&prefix, stopped).map_err(E::<G>::record)?;
                E::<G>::check(stopped)?;
            }
            GameControl::Observation { .. } => {
                return Err(E::<G>::invalid("use atomic observation admission"));
            }
            GameControl::Close {} => {}
        }
        p.current = G::canonical(G::from_game(&game).map_err(E::<G>::record)?);
        if matches!(
            control,
            GameControl::Native { .. } | GameControl::Rewind { .. }
        ) {
            p.selected_event = None;
        }
        p.controls.push(control);
        validate(p, stopped)
    }
    /// Validate exact historical selection without modifying native play.
    pub fn with_selection(
        &self,
        event: Option<usize>,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, GameDocumentError> {
        let mut p = self.payload.clone();
        p.selected_event = event;
        validate(p, stopped)
    }
    /// Atomically retain a validated result and, for engine play, its exact native action.
    /// Analysis retains observations only and never commits a board command.
    pub fn with_observation(
        &self,
        observation: ExecutionObservation,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, GameDocumentError> {
        type E<G> = Errors<G>;
        let mut p = self.payload.clone();
        let ctx = &observation.context;
        p.retired_execution_floor = p
            .retired_execution_floor
            .max(ctx.request_id)
            .max(ctx.session_version)
            .max(ctx.process_generation);
        if let ExecutionPurpose::MoveAnalysis { target } = &ctx.purpose {
            p.retired_execution_floor = p.retired_execution_floor.max(target.selection_id);
        }
        p.controls.push(GameControl::Observation {
            index: p.observations.len(),
        });
        if matches!(observation.context.purpose, ExecutionPurpose::EngineTurn {})
            && let Some(action) = G::engine_result(&observation.result).and_then(G::engine_choice)
        {
            let mut game =
                G::to_game(E::<G>::record_of(&p.current)?, stopped).map_err(E::<G>::record)?;
            E::<G>::check(stopped)?;
            let actor = G::side_to_move(&game);
            let command = G::play(action.clone());
            G::submit(&mut game, actor, command.clone(), stopped).map_err(E::<G>::native)?;
            p.controls.push(GameControl::Native { actor, command });
            p.current = G::canonical(G::from_game(&game).map_err(E::<G>::record)?);
            p.selected_event = None;
        }
        p.observations.push(observation);
        validate(p, stopped)
    }
    pub(crate) fn with_runtime_floor(&self, floor: u64) -> Result<Self, GameDocumentError> {
        let mut doc = self.clone();
        doc.payload.retired_execution_floor = doc.payload.retired_execution_floor.max(floor);
        if doc.payload.retired_execution_floor > MAX_SAFE_ID - 65 {
            return Err(Errors::<G>::limit(
                ResourceCategory::IdentityHeadroom,
                doc.payload.retired_execution_floor,
                MAX_SAFE_ID - 65,
                0,
                ResourceUnit::Identities,
            ));
        }
        let previous_bytes = doc.bytes.len();
        doc.bytes = document::encode(doc.profile(), &doc.payload).map_err(Errors::<G>::invalid)?;
        doc.usage.reserved_bytes = doc
            .usage
            .reserved_bytes
            .saturating_sub(doc.bytes.len().saturating_sub(previous_bytes));
        // Every admission reserves the widest floor spelling.
        doc.usage.encoded_bytes = doc.bytes.len();
        if doc.bytes.len() + doc.usage.reserved_bytes > MAX_ARCHIVE_BYTES {
            return Err(Errors::<G>::limit(
                ResourceCategory::EncodedDocumentBytes,
                doc.bytes.len() as u64,
                MAX_ARCHIVE_BYTES as u64,
                doc.usage.reserved_bytes as u64,
                ResourceUnit::Bytes,
            ));
        }
        Ok(doc)
    }
    /// Consume a prevalidated close reserve without native replay on Quit.
    pub(crate) fn with_reserved_close(&self, floor: u64) -> Result<Self, GameDocumentError> {
        let mut d = self.clone();
        if !matches!(d.payload.controls.last(), Some(GameControl::Close {})) {
            d.payload.controls.push(GameControl::Close {});
            d.usage.controls += 1;
            d.usage.replay_work += 1;
            // Space was admitted from actual complete closing candidates.
            // Recompute remaining byte allowance from the already admitted maximum.
            let previous = d.bytes.len();
            let encoded =
                document::encode(d.profile(), &d.payload).map_err(Errors::<G>::invalid)?;
            let next = encoded.len();
            d.bytes = encoded;
            d.usage.encoded_bytes = next;
            d.usage.reserved_bytes = d
                .usage
                .reserved_bytes
                .saturating_sub(next.saturating_sub(previous));
            d.usage.reserved_work = d.usage.reserved_work.saturating_sub(1);
        }
        d.with_runtime_floor(floor)
    }
}

pub(crate) fn validate<G: NativeGame>(
    p: Payload<G>,
    stopped: &mut impl FnMut() -> bool,
) -> Result<ValidatedGameDocument<G>, GameDocumentError> {
    type E<G> = Errors<G>;
    E::<G>::check(stopped)?;
    // Known journal/observation/identity bounds precede serialization of any
    // retained context. The actual encoded-byte check follows encoding below.
    counts(&p, 0)?;
    let selected_profile = G::profile(E::<G>::record_of(&p.initial)?);
    // Canonical envelope limits are distinct from native raw record and document
    // limits. Preflight their actual serialization so a wrapper capacity refusal
    // remains typed instead of being flattened into a serde diagnostic string.
    let records = || {
        std::iter::once(&p.initial)
            .chain(std::iter::once(&p.current))
            .chain(p.retired.iter().map(|b| &b.record))
            .chain(p.observations.iter().map(|o| &o.context.game))
    };
    for record in records() {
        E::<G>::check(stopped)?;
        if G::profile(E::<G>::record_of(record)?) != selected_profile {
            return Err(E::<G>::invalid(format!(
                "mixed immutable {} profiles",
                G::NAME
            )));
        }
        record.to_json().map_err(E::<G>::canonical)?;
    }
    let bytes = document::encode(selected_profile, &p).map_err(E::<G>::invalid)?;
    counts(&p, bytes.len())?;
    if p.mode != "untimed" {
        return Err(E::<G>::invalid(format!(
            "only untimed {} documents are supported",
            G::NAME
        )));
    }
    let mut usage = GameDocumentUsage {
        encoded_bytes: bytes.len(),
        retained_events: 0,
        controls: p.controls.len(),
        observations: p.observations.len(),
        replay_work: p.controls.len() as u64,
        reserved_bytes: 0,
        reserved_work: 0,
    };
    // Charge raw parsing of every separately retained context, even when the
    // candidate happened to reuse immutable typed records in this invocation.
    for r in records() {
        let r = E::<G>::record_of(r)?;
        usage.retained_events += G::event_count(r);
        E::<G>::charge(&mut usage.replay_work, G::reconstruction_work(r))?;
    }
    if usage.retained_events > MAX_ARCHIVE_EVENTS {
        return Err(E::<G>::limit(
            ResourceCategory::NativeEvents,
            usage.retained_events as u64,
            MAX_ARCHIVE_EVENTS as u64,
            0,
            ResourceUnit::Events,
        ));
    }
    let initial = E::<G>::record_of(&p.initial)?;
    E::<G>::charge(&mut usage.replay_work, G::reconstruction_work(initial))?;
    E::<G>::check(stopped)?;
    let mut game = G::to_game(initial, stopped).map_err(E::<G>::record)?;
    E::<G>::check(stopped)?;
    let mut settings = p.initial_settings.clone();
    let (mut retired, mut obs) = (0, 0);
    let mut required_action: Option<(G::Side, G::Command)> = None;
    for (i, control) in p.controls.iter().enumerate() {
        E::<G>::check(stopped)?;
        if let Some((actor, command)) = required_action.take()
            && !matches!(control, GameControl::Native { actor: a, command: b } if *a == actor && *b == command)
        {
            return Err(E::<G>::invalid(
                "engine choice must be followed by its exact complete native action",
            ));
        }
        match control {
            GameControl::Native { actor, command } => {
                let before = G::replay_work(&game);
                G::submit(&mut game, *actor, command.clone(), stopped).map_err(E::<G>::native)?;
                E::<G>::charge(
                    &mut usage.replay_work,
                    G::replay_work(&game).saturating_sub(before),
                )?;
            }
            GameControl::Settings { settings: next } => {
                if next.play_mode != settings.play_mode {
                    return Err(E::<G>::invalid(
                        "participant mode changed within a document",
                    ));
                }
                settings = next.clone();
            }
            GameControl::Rewind { event_index, .. } => {
                if *event_index >= G::events(&game) {
                    return Err(E::<G>::invalid("invalid rewind prefix"));
                }
                let saved = p
                    .retired
                    .get(retired)
                    .ok_or_else(|| E::<G>::invalid("missing retired branch"))?;
                if saved.control_index != i
                    || E::<G>::record_of(&saved.record)?
                        != &G::from_game(&game).map_err(E::<G>::record)?
                {
                    return Err(E::<G>::invalid("retired branch disagrees with journal"));
                }
                let prefix =
                    G::record_prefix(&game, *event_index, stopped).map_err(E::<G>::record)?;
                E::<G>::charge(&mut usage.replay_work, 2 * G::reconstruction_work(&prefix))?;
                E::<G>::check(stopped)?;
                game = G::to_game(&prefix, stopped).map_err(E::<G>::record)?;
                E::<G>::check(stopped)?;
                retired += 1;
            }
            GameControl::Observation { index } => {
                if *index != obs {
                    return Err(E::<G>::invalid("observation order disagrees"));
                }
                let o = p
                    .observations
                    .get(obs)
                    .ok_or_else(|| E::<G>::invalid("missing observation"))?;
                if settings.play_mode.is_two_humans()
                    && matches!(o.context.purpose, ExecutionPurpose::EngineTurn {})
                {
                    return Err(E::<G>::invalid(
                        "two-human documents cannot contain engine turns",
                    ));
                }
                let result = G::engine_result(&o.result);
                if !o.matches(&o.context)
                    || !(matches!(o.result, ExecutionResult::Failure { .. }) || result.is_some())
                {
                    return Err(E::<G>::invalid(format!(
                        "invalid or unsupported {} execution observation",
                        G::NAME
                    )));
                }
                let event = match &o.context.purpose {
                    ExecutionPurpose::EngineTurn {} => G::events(&game),
                    ExecutionPurpose::MoveAnalysis { target } => {
                        usize::try_from(target.event_index).map_err(E::<G>::invalid)?
                    }
                    _ => {
                        return Err(E::<G>::invalid(format!(
                            "no Chess Full or other analysis operation is available for {}",
                            G::NAME
                        )));
                    }
                };
                let prefix = G::record_prefix(&game, event, stopped).map_err(E::<G>::record)?;
                E::<G>::charge(&mut usage.replay_work, G::reconstruction_work(&prefix))?;
                if &prefix != E::<G>::record_of(&o.context.game)? {
                    return Err(E::<G>::invalid(
                        "observation is not the exact journal history prefix",
                    ));
                }
                if let Some(result) = result {
                    E::<G>::charge(&mut usage.replay_work, G::reconstruction_work(&prefix))?;
                    let target_game = G::to_game(&prefix, stopped).map_err(E::<G>::record)?;
                    E::<G>::check(stopped)?;
                    let validated = G::validate_result(result, &target_game, stopped);
                    E::<G>::check(stopped)?;
                    E::<G>::charge(&mut usage.replay_work, validated.map_err(E::<G>::invalid)?)?;
                    if matches!(o.context.purpose, ExecutionPurpose::EngineTurn {})
                        && let Some(action) = G::engine_choice(result)
                    {
                        required_action = Some((G::side_to_move(&game), G::play(action.clone())));
                    }
                }
                for id in [
                    o.context.request_id,
                    o.context.session_version,
                    o.context.process_generation,
                    match &o.context.purpose {
                        ExecutionPurpose::MoveAnalysis { target } => target.selection_id,
                        _ => o.context.session_version,
                    },
                ] {
                    if id == 0 || id > p.retired_execution_floor {
                        return Err(E::<G>::invalid("observation execution floor disagrees"));
                    }
                }
                let request = mshogi_engine::ExecutionRequest {
                    protocol: mshogi_engine::PROTOCOL_VERSION,
                    context: o.context.clone(),
                };
                let response = mshogi_engine::ExecutionResponse {
                    protocol: mshogi_engine::PROTOCOL_VERSION,
                    observation: o.clone(),
                };
                if serde_json::to_vec(&request).map_err(E::<G>::invalid)?.len()
                    > mshogi_engine::MAX_REQUEST_BYTES
                    || serde_json::to_vec(&response)
                        .map_err(E::<G>::invalid)?
                        .len()
                        > mshogi_engine::MAX_RESPONSE_BYTES
                {
                    return Err(E::<G>::invalid(
                        "observation exceeds execution transport bounds",
                    ));
                }
                obs += 1;
            }
            GameControl::Close {} => {}
        }
    }
    E::<G>::check(stopped)?;
    let current = E::<G>::record_of(&p.current)?;
    if required_action.is_some()
        || retired != p.retired.len()
        || obs != p.observations.len()
        || settings != p.settings
        || current != &G::from_game(&game).map_err(E::<G>::record)?
    {
        return Err(E::<G>::invalid(
            "native journal/current/settings/branches/observations disagree",
        ));
    }
    // Prepay actual runtime installation after decode (fresh game, no pending work).
    E::<G>::charge(&mut usage.replay_work, G::reconstruction_work(current))?;
    if let Some(index) = p.selected_event {
        let prefix = G::record_prefix(&game, index, stopped).map_err(E::<G>::record)?;
        // One validation construction plus runtime prefix construction/replay.
        E::<G>::charge(&mut usage.replay_work, 3 * G::reconstruction_work(&prefix))?;
    }
    reserve(&p, &mut game, &mut usage, stopped)?;
    E::<G>::check(stopped)?;
    Ok(ValidatedGameDocument {
        payload: p,
        bytes,
        usage,
    })
}

/// Reserve the largest complete closing sequence: an early close and reopen,
/// the native ending commands, and the final close.
fn reserve<G: NativeGame>(
    p: &Payload<G>,
    game: &mut G::Game,
    usage: &mut GameDocumentUsage,
    stopped: &mut impl FnMut() -> bool,
) -> Result<(), GameDocumentError> {
    type E<G> = Errors<G>;
    let current_work = G::replay_work(game);
    let mut max_bytes = usage.encoded_bytes;
    let mut max_work = usage.replay_work;
    let mut max_controls = usage.controls;
    let mut max_events = usage.retained_events;
    // Only a game that admits a Stop after its conclusion has a concluding Stop.
    let concluding: &[Option<G::Side>] = if G::STOP_AFTER_CONCLUSION {
        &[Some(G::SIDES[0]), Some(G::SIDES[1])]
    } else {
        &[None]
    };
    for stop_actor in G::SIDES {
        for resign_actor in G::SIDES {
            for &concluding_actor in concluding {
                E::<G>::check(stopped)?;
                let mut q = p.clone();
                let mut undos = vec![];
                let ongoing = !G::concluded(game);
                let needs_stop = !G::stop_recorded(game);
                // A close/reopen before ending, plus final Close, are actual controls.
                if ongoing && !matches!(q.controls.last(), Some(GameControl::Close {})) {
                    q.controls.push(GameControl::Close {});
                }
                let mut add = |actor, command: G::Command| -> Result<(), GameDocumentError> {
                    E::<G>::check(stopped)?;
                    undos.push(
                        G::submit(game, actor, command.clone(), stopped).map_err(E::<G>::native)?,
                    );
                    q.controls.push(GameControl::Native { actor, command });
                    Ok(())
                };
                if ongoing {
                    if needs_stop {
                        add(stop_actor, G::engine_failure_stop())?;
                    }
                    add(resign_actor, G::resign())?;
                    if let Some(actor) = concluding_actor {
                        add(actor, G::engine_failure_stop())?;
                    }
                } else if needs_stop && let Some(actor) = concluding_actor {
                    add(actor, G::engine_failure_stop())?;
                }
                let native_added = undos.len();
                q.current = G::canonical(G::from_game(game).map_err(E::<G>::record)?);
                q.current.to_json().map_err(E::<G>::canonical)?;
                if !matches!(q.controls.last(), Some(GameControl::Close {})) {
                    q.controls.push(GameControl::Close {});
                }
                q.retired_execution_floor = MAX_SAFE_ID - 65;
                // A null selection (4 bytes) is as long as every supported numeric
                // prefix. Endings return selection to live; no invented selection.
                q.selected_event = None;
                max_bytes = max_bytes.max(
                    document::encode(G::profile(E::<G>::record_of(&q.current)?), &q)
                        .map_err(E::<G>::invalid)?
                        .len(),
                );
                let added_controls = q.controls.len() - p.controls.len();
                // The changed current record appears in raw decode, journal replay
                // and runtime installation; each actual native delta is prepaid.
                let delta = G::replay_work(game).saturating_sub(current_work);
                max_work = max_work.max(
                    usage
                        .replay_work
                        .saturating_add(delta.saturating_mul(3))
                        .saturating_add(added_controls as u64),
                );
                max_controls = max_controls.max(q.controls.len());
                max_events = max_events.max(usage.retained_events + native_added);
                for undo in undos.iter_mut().rev() {
                    G::undo(game, undo).map_err(E::<G>::native)?;
                }
            }
        }
    }
    usage.reserved_bytes = max_bytes - usage.encoded_bytes;
    usage.reserved_work = max_work - usage.replay_work;
    for (category, count, cap, reserved, unit) in [
        (
            ResourceCategory::EncodedDocumentBytes,
            usage.encoded_bytes as u64,
            MAX_ARCHIVE_BYTES as u64,
            usage.reserved_bytes as u64,
            ResourceUnit::Bytes,
        ),
        (
            ResourceCategory::Controls,
            usage.controls as u64,
            MAX_ARCHIVE_CONTROLS as u64,
            (max_controls - usage.controls) as u64,
            ResourceUnit::Controls,
        ),
        (
            ResourceCategory::NativeEvents,
            usage.retained_events as u64,
            MAX_ARCHIVE_EVENTS as u64,
            (max_events - usage.retained_events) as u64,
            ResourceUnit::Events,
        ),
        (
            ResourceCategory::ReplayWork,
            usage.replay_work,
            G::MAX_DOCUMENT_WORK,
            usage.reserved_work,
            ResourceUnit::ReplayUnits,
        ),
    ] {
        if count.saturating_add(reserved) > cap {
            return Err(E::<G>::limit(category, count, cap, reserved, unit));
        }
    }
    Ok(())
}
