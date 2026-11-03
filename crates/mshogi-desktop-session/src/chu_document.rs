//! Pure Chu candidates in the existing desktop envelope. No runtime or file owner.
use crate::{document, persistence, *};
use mshogi_core::{
    chu as c,
    native::{CanonicalRecord, GameKind, RulesProfile},
};
use mshogi_engine::{ExecutionObservation, ExecutionPurpose, ExecutionResult};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Aggregate authoritative native replay units, including retained targets and installation.
/// This is independent of native per-record limits and the file worker's 15-second deadline.
pub const MAX_CHU_DOCUMENT_WORK: u64 = 12_000_000;
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
/// Supported Chu document orientation; engine preferences remain unavailable.
pub struct ChuSettings {
    /// Native side displayed at the bottom; no search defaults are invented.
    pub orientation: c::Side,
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
pub enum ChuControl {
    /// Admit a complete native command with its native actor.
    Native {
        /// Native participant or administrative actor.
        actor: c::Side,
        /// Complete native command.
        command: c::GameCommand,
    },
    /// Persist presentation settings without modifying rules.
    Settings {
        /// Native orientation.
        settings: ChuSettings,
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
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Retired {
    control_index: usize,
    record: CanonicalRecord,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Payload {
    mode: String,
    initial: CanonicalRecord,
    current: CanonicalRecord,
    human: c::Side,
    initial_settings: ChuSettings,
    settings: ChuSettings,
    controls: Vec<ChuControl>,
    observations: Vec<ExecutionObservation>,
    retired: Vec<Retired>,
    retired_execution_floor: u64,
    #[serde(deserialize_with = "shared::archive::required_option")]
    selected_event: Option<usize>,
}
#[derive(Clone, Debug, Serialize)]
/// Measured whole-document usage, including repeated contexts.
pub struct ChuDocumentUsage {
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
pub struct ChuDocumentError {
    /// Bounded technical diagnostic.
    pub diagnostic: String,
    /// Authoritative capacity facts when known.
    pub resource: Option<ResourceFailure>,
}
impl fmt::Display for ChuDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.diagnostic)
    }
}
impl std::error::Error for ChuDocumentError {}
impl From<OperationFailure> for ChuDocumentError {
    fn from(e: OperationFailure) -> Self {
        Self {
            diagnostic: e.diagnostic,
            resource: e.resource,
        }
    }
}
impl From<ChuDocumentError> for OperationFailure {
    fn from(e: ChuDocumentError) -> Self {
        Self {
            diagnostic: e.diagnostic,
            resource: e.resource,
        }
    }
}
fn invalid(e: impl fmt::Display) -> ChuDocumentError {
    ChuDocumentError {
        diagnostic: document::diagnostic(format!("Invalid Chu document: {e}")),
        resource: None,
    }
}
fn check(stopped: &mut impl FnMut() -> bool) -> Result<(), ChuDocumentError> {
    persistence::check(stopped).map_err(invalid)
}
fn limit(
    category: ResourceCategory,
    count: u64,
    cap: u64,
    reserve: u64,
    unit: ResourceUnit,
) -> ChuDocumentError {
    OperationFailure::resource(ResourceFailure::new(
        ResourceFact::measured(category, count, cap)
            .with_reserved(reserve)
            .with_scope(ResourceScope::Chu)
            .with_unit(unit),
        "Application limit reached. The complete previously accepted game can still be saved.",
    ))
    .into()
}
pub(crate) fn native_error(e: c::GameError) -> ChuDocumentError {
    if let c::GameError::Capacity(r) = e {
        let (category, cap, unit) = match r {
            c::Resource::EncodedBytes => (
                ResourceCategory::CanonicalRecordBytes,
                c::MAX_GAME_RECORD_BYTES as u64,
                ResourceUnit::Bytes,
            ),
            c::Resource::Events => (
                ResourceCategory::NativeEvents,
                c::MAX_GAME_EVENTS as u64,
                ResourceUnit::Events,
            ),
            c::Resource::ReplayWork => (
                ResourceCategory::ReplayWork,
                c::MAX_REPLAY_WORK,
                ResourceUnit::ReplayUnits,
            ),
            c::Resource::Counter => {
                // Native Counter covers both declared/accepted board counters
                // and runtime identities. It cannot identify one specific limit.
                return OperationFailure::resource(ResourceFailure::new(
                    ResourceFact::unknown().with_scope(ResourceScope::Chu),
                    e.to_string(),
                ))
                .into();
            }
        };
        let mut fact = ResourceFact::measured(category, 0, cap)
            .with_scope(ResourceScope::Chu)
            .with_unit(unit);
        fact.count = None;
        return OperationFailure::resource(ResourceFailure::new(fact, e.to_string())).into();
    }
    invalid(e)
}
fn native(r: &CanonicalRecord) -> Result<&c::GameRecord, ChuDocumentError> {
    if let CanonicalRecord::Chu(r) = r {
        Ok(r)
    } else {
        Err(invalid("record must be native Chu"))
    }
}
fn charge(total: &mut u64, n: u64) -> Result<(), ChuDocumentError> {
    *total = total.saturating_add(n);
    if *total > MAX_CHU_DOCUMENT_WORK {
        return Err(limit(
            ResourceCategory::ReplayWork,
            *total,
            MAX_CHU_DOCUMENT_WORK,
            0,
            ResourceUnit::ReplayUnits,
        ));
    }
    Ok(())
}
fn counts(p: &Payload, bytes: usize) -> Result<(), ChuDocumentError> {
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
            return Err(limit(category, count, cap, 0, unit));
        }
    }
    if p.retired_execution_floor == 0 {
        return Err(invalid("zero runtime floor"));
    }
    Ok(())
}
/// An immutable validated candidate. Only DesktopController installs it.
#[derive(Clone, Debug)]
pub struct ValidatedChuDocument {
    payload: Payload,
    bytes: Vec<u8>,
    usage: ChuDocumentUsage,
}
impl ValidatedChuDocument {
    /// Create a stopped candidate from an authoritative native root/history.
    pub fn new(record: c::GameRecord, human: c::Side) -> Result<Self, ChuDocumentError> {
        Self::new_with_stop(record, human, &mut || false)
    }
    pub(crate) fn new_with_stop(
        record: c::GameRecord,
        human: c::Side,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, ChuDocumentError> {
        check(stopped)?;
        let settings = ChuSettings { orientation: human };
        let record = CanonicalRecord::chu(record);
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
    ) -> Result<Self, ChuDocumentError> {
        check(stopped)?;
        if bytes.len() > MAX_ARCHIVE_BYTES {
            return Err(limit(
                ResourceCategory::EncodedDocumentBytes,
                bytes.len() as u64,
                MAX_ARCHIVE_BYTES as u64,
                0,
                ResourceUnit::Bytes,
            ));
        }
        document::reject_duplicates(bytes).map_err(invalid)?;
        let e = document::DocumentEnvelope::parse(bytes).map_err(invalid)?;
        if e.game_kind != GameKind::Chu {
            return Err(invalid("wrong desktop game"));
        }
        // Shape/count/identity probes precede every expensive native parser.
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            mode: String,
            initial: Box<serde_json::value::RawValue>,
            current: Box<serde_json::value::RawValue>,
            human: c::Side,
            initial_settings: ChuSettings,
            settings: ChuSettings,
            controls: Vec<ChuControl>,
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
        let w: Wire = serde_json::from_str(e.payload.get()).map_err(invalid)?;
        if w.controls.len() > MAX_ARCHIVE_CONTROLS {
            return Err(limit(
                ResourceCategory::Controls,
                w.controls.len() as u64,
                MAX_ARCHIVE_CONTROLS as u64,
                0,
                ResourceUnit::Controls,
            ));
        }
        if w.observations.len() > MAX_ARCHIVE_OBSERVATIONS {
            return Err(limit(
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
            check(stopped)?;
            let r = CanonicalRecord::from_json(raw.get()).map_err(invalid)?;
            charge(&mut parsed_work, native(&r)?.reconstruction_work())?;
            check(stopped)?;
            Ok::<_, ChuDocumentError>(r)
        };
        let initial = parse(&w.initial)?;
        let current = parse(&w.current)?;
        let mut retired = vec![];
        for b in w.retired {
            retired.push(Retired {
                control_index: b.control_index,
                record: parse(&b.record)?,
            });
        }
        let mut observations = vec![];
        for raw in w.observations {
            check(stopped)?;
            let o: ExecutionObservation = serde_json::from_str(raw.get()).map_err(invalid)?;
            charge(
                &mut parsed_work,
                native(&o.context.game)?.reconstruction_work(),
            )?;
            check(stopped)?;
            observations.push(o);
        }
        validate(
            Payload {
                mode: w.mode,
                initial,
                current,
                human: w.human,
                initial_settings: w.initial_settings,
                settings: w.settings,
                controls: w.controls,
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
    pub fn usage(&self) -> &ChuDocumentUsage {
        &self.usage
    }
    /// Complete current native record.
    pub fn record(&self) -> &c::GameRecord {
        native(&self.payload.current).expect("validated Chu")
    }
    /// Complete initial native history and provenance.
    pub fn initial_record(&self) -> &c::GameRecord {
        native(&self.payload.initial).expect("validated Chu")
    }
    /// Recorded native human assignment.
    pub fn human(&self) -> c::Side {
        self.payload.human
    }
    /// Accepted orientation settings.
    pub fn settings(&self) -> &ChuSettings {
        &self.payload.settings
    }
    /// Exact durable control journal.
    pub fn controls(&self) -> &[ChuControl] {
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
    pub fn retired_branches(&self) -> impl Iterator<Item = (usize, &c::GameRecord)> {
        self.payload
            .retired
            .iter()
            .map(|b| (b.control_index, native(&b.record).expect("validated Chu")))
    }
    /// Validate a complete candidate before replacing this immutable document.
    pub fn with_control(
        &self,
        control: ChuControl,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, ChuDocumentError> {
        check(stopped)?;
        let mut p = self.payload.clone();
        let mut game = self.record().to_game().map_err(invalid)?;
        check(stopped)?;
        match &control {
            ChuControl::Native { actor, command } => {
                game.submit(c::GameRequest {
                    expected: game.context(),
                    actor: *actor,
                    command: command.clone(),
                })
                .map_err(native_error)?;
            }
            ChuControl::Settings { settings } => p.settings = settings.clone(),
            ChuControl::Rewind { event_index, .. } => {
                if *event_index >= game.events().len() {
                    return Err(invalid("rewind must retire an event"));
                }
                p.retired.push(Retired {
                    control_index: p.controls.len(),
                    record: p.current.clone(),
                });
                let prefix = game.record_prefix(*event_index).map_err(invalid)?;
                check(stopped)?;
                game = prefix.to_game().map_err(invalid)?;
                check(stopped)?;
            }
            ChuControl::Observation { .. } => {
                return Err(invalid("use atomic observation admission"));
            }
            ChuControl::Close {} => {}
        }
        p.current = CanonicalRecord::chu(c::GameRecord::from_game(&game).map_err(invalid)?);
        if matches!(
            control,
            ChuControl::Native { .. } | ChuControl::Rewind { .. }
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
    ) -> Result<Self, ChuDocumentError> {
        let mut p = self.payload.clone();
        p.selected_event = event;
        validate(p, stopped)
    }
    /// Atomically retain an actual unscored result and its exact target.
    pub fn with_observation(
        &self,
        observation: ExecutionObservation,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, ChuDocumentError> {
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
        p.controls.push(ChuControl::Observation {
            index: p.observations.len(),
        });
        p.observations.push(observation);
        validate(p, stopped)
    }
    pub(crate) fn with_runtime_floor(&self, floor: u64) -> Result<Self, ChuDocumentError> {
        let mut doc = self.clone();
        doc.payload.retired_execution_floor = doc.payload.retired_execution_floor.max(floor);
        if doc.payload.retired_execution_floor > MAX_SAFE_ID - 65 {
            return Err(limit(
                ResourceCategory::IdentityHeadroom,
                doc.payload.retired_execution_floor,
                MAX_SAFE_ID - 65,
                0,
                ResourceUnit::Identities,
            ));
        }
        let previous_bytes = doc.bytes.len();
        doc.bytes = document::encode(RulesProfile::ChuJcsa2019V1, &doc.payload).map_err(invalid)?;
        doc.usage.reserved_bytes = doc
            .usage
            .reserved_bytes
            .saturating_sub(doc.bytes.len().saturating_sub(previous_bytes));
        // Every admission reserves the widest floor spelling.
        doc.usage.encoded_bytes = doc.bytes.len();
        if doc.bytes.len() + doc.usage.reserved_bytes > MAX_ARCHIVE_BYTES {
            return Err(limit(
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
    pub(crate) fn with_reserved_close(&self, floor: u64) -> Result<Self, ChuDocumentError> {
        let mut d = self.clone();
        if !matches!(d.payload.controls.last(), Some(ChuControl::Close {})) {
            d.payload.controls.push(ChuControl::Close {});
            d.usage.controls += 1;
            d.usage.replay_work += 1;
            // Space was admitted from actual complete closing candidates.
            // Recompute remaining byte allowance from the already admitted maximum.
            let previous = d.bytes.len();
            let encoded =
                document::encode(RulesProfile::ChuJcsa2019V1, &d.payload).map_err(invalid)?;
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

fn validate(
    p: Payload,
    stopped: &mut impl FnMut() -> bool,
) -> Result<ValidatedChuDocument, ChuDocumentError> {
    check(stopped)?;
    let bytes = document::encode(RulesProfile::ChuJcsa2019V1, &p).map_err(invalid)?;
    counts(&p, bytes.len())?;
    if p.mode != "untimed" {
        return Err(invalid("only untimed Chu documents are supported"));
    }
    let mut usage = ChuDocumentUsage {
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
    let records = std::iter::once(&p.initial)
        .chain(std::iter::once(&p.current))
        .chain(p.retired.iter().map(|b| &b.record))
        .chain(p.observations.iter().map(|o| &o.context.game));
    for r in records {
        let r = native(r)?;
        usage.retained_events += r.event_count();
        charge(&mut usage.replay_work, r.reconstruction_work())?;
    }
    if usage.retained_events > MAX_ARCHIVE_EVENTS {
        return Err(limit(
            ResourceCategory::NativeEvents,
            usage.retained_events as u64,
            MAX_ARCHIVE_EVENTS as u64,
            0,
            ResourceUnit::Events,
        ));
    }
    charge(
        &mut usage.replay_work,
        native(&p.initial)?.reconstruction_work(),
    )?;
    check(stopped)?;
    let mut game = native(&p.initial)?.to_game().map_err(invalid)?;
    check(stopped)?;
    let mut settings = p.initial_settings.clone();
    let (mut retired, mut obs) = (0, 0);
    for (i, control) in p.controls.iter().enumerate() {
        check(stopped)?;
        match control {
            ChuControl::Native { actor, command } => {
                let before = game.replay_work();
                game.submit(c::GameRequest {
                    expected: game.context(),
                    actor: *actor,
                    command: command.clone(),
                })
                .map_err(native_error)?;
                charge(
                    &mut usage.replay_work,
                    game.replay_work().saturating_sub(before),
                )?;
            }
            ChuControl::Settings { settings: next } => settings = next.clone(),
            ChuControl::Rewind { event_index, .. } => {
                if *event_index >= game.events().len() {
                    return Err(invalid("invalid rewind prefix"));
                }
                let saved = p
                    .retired
                    .get(retired)
                    .ok_or_else(|| invalid("missing retired branch"))?;
                if saved.control_index != i
                    || native(&saved.record)?
                        != &c::GameRecord::from_game(&game).map_err(invalid)?
                {
                    return Err(invalid("retired branch disagrees with journal"));
                }
                let prefix = game.record_prefix(*event_index).map_err(invalid)?;
                charge(&mut usage.replay_work, 2 * prefix.reconstruction_work())?;
                check(stopped)?;
                game = prefix.to_game().map_err(invalid)?;
                check(stopped)?;
                retired += 1;
            }
            ChuControl::Observation { index } => {
                if *index != obs {
                    return Err(invalid("observation order disagrees"));
                }
                let o = p
                    .observations
                    .get(obs)
                    .ok_or_else(|| invalid("missing observation"))?;
                if !o.matches(&o.context) || !matches!(o.result, ExecutionResult::Failure { .. }) {
                    return Err(invalid(
                        "Chu execution is unsupported; only unscored failure observations may be retained",
                    ));
                }
                let event = match &o.context.purpose {
                    ExecutionPurpose::EngineTurn {} => game.events().len(),
                    ExecutionPurpose::MoveAnalysis { target } => {
                        usize::try_from(target.event_index).map_err(invalid)?
                    }
                    _ => {
                        return Err(invalid(
                            "no Chess Full or other analysis operation is available for Chu",
                        ));
                    }
                };
                let prefix = game.record_prefix(event).map_err(invalid)?;
                charge(&mut usage.replay_work, prefix.reconstruction_work())?;
                if &prefix != native(&o.context.game)? {
                    return Err(invalid(
                        "observation is not the exact journal history prefix",
                    ));
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
                        return Err(invalid("observation execution floor disagrees"));
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
                if serde_json::to_vec(&request).map_err(invalid)?.len()
                    > mshogi_engine::MAX_REQUEST_BYTES
                    || serde_json::to_vec(&response).map_err(invalid)?.len()
                        > mshogi_engine::MAX_RESPONSE_BYTES
                {
                    return Err(invalid("observation exceeds execution transport bounds"));
                }
                obs += 1;
            }
            ChuControl::Close {} => {}
        }
    }
    check(stopped)?;
    if retired != p.retired.len()
        || obs != p.observations.len()
        || settings != p.settings
        || native(&p.current)? != &c::GameRecord::from_game(&game).map_err(invalid)?
    {
        return Err(invalid(
            "native journal/current/settings/branches/observations disagree",
        ));
    }
    // Prepay actual runtime installation after decode (fresh game, no pending work).
    charge(
        &mut usage.replay_work,
        native(&p.current)?.reconstruction_work(),
    )?;
    if let Some(index) = p.selected_event {
        let prefix = game.record_prefix(index).map_err(invalid)?;
        charge(&mut usage.replay_work, 2 * prefix.reconstruction_work())?;
    }
    reserve(&p, &mut game, &mut usage, stopped)?;
    check(stopped)?;
    Ok(ValidatedChuDocument {
        payload: p,
        bytes,
        usage,
    })
}

fn reserve(
    p: &Payload,
    game: &mut c::Game,
    usage: &mut ChuDocumentUsage,
    stopped: &mut impl FnMut() -> bool,
) -> Result<(), ChuDocumentError> {
    let current_work = game.replay_work();
    let mut max_bytes = usage.encoded_bytes;
    let mut max_work = usage.replay_work;
    let mut max_controls = usage.controls;
    let mut max_events = usage.retained_events;
    for stop_actor in [c::Side::Sente, c::Side::Gote] {
        for resign_actor in [c::Side::Sente, c::Side::Gote] {
            check(stopped)?;
            let mut q = p.clone();
            let mut undo = vec![];
            let ongoing = game.conclusion().is_none();
            let needs_stop = game.snapshot().incomplete.is_none();
            let closed = matches!(p.controls.last(), Some(ChuControl::Close {}));
            // Reserve an early close/reopen followed by the ending and final close.
            if ongoing && !closed {
                q.controls.push(ChuControl::Close {});
            }
            let mut add = |actor, command: c::GameCommand| -> Result<(), ChuDocumentError> {
                check(stopped)?;
                undo.push(
                    game.submit(c::GameRequest {
                        expected: game.context(),
                        actor,
                        command: command.clone(),
                    })
                    .map_err(native_error)?,
                );
                q.controls.push(ChuControl::Native { actor, command });
                Ok(())
            };
            if ongoing {
                if needs_stop {
                    add(
                        stop_actor,
                        c::GameCommand::Stop {
                            reason: c::StopReason::EngineFailure,
                        },
                    )?;
                }
                add(resign_actor, c::GameCommand::Resign)?;
            }
            let native_added = undo.len();
            q.current = CanonicalRecord::chu(c::GameRecord::from_game(game).map_err(invalid)?);
            if !matches!(q.controls.last(), Some(ChuControl::Close {})) {
                q.controls.push(ChuControl::Close {});
            }
            q.retired_execution_floor = MAX_SAFE_ID - 65;
            // null (4 bytes) is longer than every supported numeric prefix (0..512).
            // Native ending commands also return selection to live.
            q.selected_event = None;
            max_bytes = max_bytes.max(
                document::encode(RulesProfile::ChuJcsa2019V1, &q)
                    .map_err(invalid)?
                    .len(),
            );
            let added_controls = q.controls.len() - p.controls.len();
            // Current parse + journal replay + runtime reconstruction each grow.
            max_work = max_work.max(
                usage.replay_work + 3 * (game.replay_work() - current_work) + added_controls as u64,
            );
            max_controls = max_controls.max(q.controls.len());
            max_events = max_events.max(usage.retained_events + native_added);
            for token in undo.iter_mut().rev() {
                game.undo(token).map_err(native_error)?;
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
            MAX_CHU_DOCUMENT_WORK,
            usage.reserved_work,
            ResourceUnit::ReplayUnits,
        ),
    ] {
        if count.saturating_add(reserved) > cap {
            return Err(limit(category, count, cap, reserved, unit));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
