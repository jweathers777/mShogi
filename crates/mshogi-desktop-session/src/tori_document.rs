//! Immutable Tori candidates for the one current desktop envelope.
//!
//! This is not an active document or an execution owner. The native game admits
//! every board/terminal command. Installation is owned solely by DesktopController/persistence, not this codec.
use crate::{document, persistence, *};
use mshogi_core::{
    native::{CanonicalRecord, GameKind, RulesProfile},
    tori as t,
};
use mshogi_engine::{ExecutionObservation, ExecutionPurpose, ExecutionResult};
use serde::{
    Deserialize, Serialize,
    de::{self, MapAccess, SeqAccess, Visitor},
};
use serde_json::value::RawValue;
use std::{collections::BTreeSet, fmt};

/// Whole-document deterministic validation allowance, distinct from native caps.
pub const MAX_TORI_DOCUMENT_WORK: u64 = 262_144;
// A native root admission/serialization costs work even without any events.
const ROOT_WORK: u64 = 128;
const NATIVE_RESERVED_BYTES: usize = 8192;
const CLOSE_RESERVED_BYTES: usize = 1024;
const NATIVE_RESERVED_WORK: u64 = 512;
const CLOSE_RESERVED_WORK: u64 = 16;
// Reservations are consumed by Stop, then Resign, then close; not reset to three.
#[derive(Clone, Copy)]
struct Reserve {
    controls: usize,
    native: usize,
}

/// Persisted Tori orientation. Baseline play limits are fixed by the native engine version.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToriSettings {
    /// Native side placed at the bottom.
    pub orientation: t::Side,
}
/// Why a previous complete branch was retired; never a pending operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RewindKind {
    /// Explicit semantic event undo.
    Undo,
    /// Restore a prior human decision, with the full abandoned branch retained.
    TakeBack,
}
/// Ordered durable document operations. Native commands retain their native type.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ToriControl {
    /// Complete native command, never a partially selected move.
    Native {
        /// Native participant or administrative actor.
        actor: t::Side,
        /// Native command admitted by Game::submit.
        command: t::GameCommand,
    },
    /// Document preferences, separate from rules state.
    Settings {
        /// Supported native document settings.
        settings: ToriSettings,
    },
    /// Retire the complete current branch and restore its exact event prefix.
    Rewind {
        /// Exact native event count, not board ply.
        event_index: usize,
        /// Semantic rewind intent.
        reason: RewindKind,
    },
    /// Retain one actual observation at the current committed history prefix.
    Observation {
        /// Index in the document's observation ledger.
        index: usize,
    },
    /// Persisted close/pause bookkeeping, not a native game outcome.
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
    human: t::Side,
    initial_settings: ToriSettings,
    settings: ToriSettings,
    controls: Vec<ToriControl>,
    observations: Vec<ExecutionObservation>,
    retired: Vec<Retired>,
    retired_execution_floor: u64,
}
/// Actual complete-file resource usage. Native units remain per-record facts.
#[derive(Clone, Debug, Serialize)]
pub struct ToriDocumentUsage {
    /// Actual pretty-encoded current envelope bytes.
    pub encoded_bytes: usize,
    /// All encoded native events, including duplicate retained contexts.
    pub retained_events: usize,
    /// All durable desktop controls, including board commands.
    pub controls: usize,
    /// Historical execution observations, never pending work.
    pub observations: usize,
    /// Conservative aggregate native parsing/reconstruction/encoding charges.
    pub replay_work: u64,
}
/// Bounded refusal; a resource cause is explicit and never parsed from its text.
#[derive(Clone, Debug)]
pub struct ToriDocumentError {
    /// Bounded technical diagnostic.
    pub diagnostic: String,
    /// Authoritative native resource facts, when known.
    pub resource: Option<ResourceFailure>,
}
impl fmt::Display for ToriDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.diagnostic)
    }
}
impl std::error::Error for ToriDocumentError {}
impl From<OperationFailure> for ToriDocumentError {
    fn from(e: OperationFailure) -> Self {
        Self {
            diagnostic: e.diagnostic,
            resource: e.resource,
        }
    }
}
impl From<ToriDocumentError> for OperationFailure {
    fn from(e: ToriDocumentError) -> Self {
        Self {
            diagnostic: e.diagnostic,
            resource: e.resource,
        }
    }
}
fn invalid(e: impl fmt::Display) -> ToriDocumentError {
    ToriDocumentError {
        diagnostic: document::diagnostic(format!("Invalid Tori document: {e}")),
        resource: None,
    }
}
fn limit(
    category: ResourceCategory,
    count: u64,
    cap: u64,
    reserve: u64,
    unit: ResourceUnit,
) -> ToriDocumentError {
    OperationFailure::resource(ResourceFailure::new(
        ResourceFact::measured(category, count, cap)
            .with_reserved(reserve)
            .with_scope(ResourceScope::Tori)
            .with_unit(unit),
        "Application capacity reached. The complete previously accepted document remains saveable.",
    ))
    .into()
}
/// Replay-validated immutable current-format candidate. No runtime identity is installed.
#[derive(Clone)]
pub struct ValidatedToriDocument {
    payload: Payload,
    bytes: Vec<u8>,
    usage: ToriDocumentUsage,
}
impl fmt::Debug for ValidatedToriDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValidatedToriDocument")
            .field("usage", &self.usage)
            .finish_non_exhaustive()
    }
}
impl ValidatedToriDocument {
    /// Construct a stopped document from an actual native initial record.
    pub fn new(record: t::GameRecord, human: t::Side) -> Result<Self, ToriDocumentError> {
        let settings = ToriSettings { orientation: human };
        let record = CanonicalRecord::tori(record);
        validate_payload(
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
            },
            &mut || false,
        )
    }
    /// Strict decode and native replay on a caller's bounded/cancellable worker.
    pub fn decode(
        bytes: &[u8],
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, ToriDocumentError> {
        let usage = preflight(bytes, stopped)?;
        let envelope = document::DocumentEnvelope::parse(bytes).map_err(invalid)?;
        check(stopped)?;
        let payload = parse_payload(envelope.payload.get(), stopped)?;
        check(stopped)?;
        validate_semantics(&payload, stopped)?;
        // Canonical output can grow (whitespace, normalized enums, wider identifiers).
        let encoded = document::encode(RulesProfile::ToriModernV1, &payload).map_err(invalid)?;
        let mut usage = usage;
        usage.encoded_bytes = encoded.len();
        check_limits(
            &usage,
            payload.retired_execution_floor,
            reservation(&payload),
        )?;
        Ok(Self {
            payload,
            bytes: encoded,
            usage,
        })
    }
    /// Complete current envelope bytes, suitable only for the existing atomic writer.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Immutable measured usage.
    pub fn usage(&self) -> &ToriDocumentUsage {
        &self.usage
    }
    /// Exact native live record; observation targets are separately retained.
    pub fn record(&self) -> &t::GameRecord {
        native(&self.payload.current).expect("validated Tori")
    }
    /// Exact admitted initial native history, including imported root provenance.
    pub fn initial_record(&self) -> &t::GameRecord {
        native(&self.payload.initial).expect("validated Tori")
    }
    /// Assigned native human side.
    pub fn human(&self) -> t::Side {
        self.payload.human
    }
    /// Accepted document settings.
    pub fn settings(&self) -> &ToriSettings {
        &self.payload.settings
    }
    /// Ordered durable controls; native non-board events are not Moves rows.
    pub fn controls(&self) -> &[ToriControl] {
        &self.payload.controls
    }
    /// Retired execution high-water evidence; restoration must issue fresh identities.
    pub fn retired_execution_floor(&self) -> u64 {
        self.payload.retired_execution_floor
    }
    /// Complete retired branches paired with their journal rewind control index.
    pub fn retired_branches(&self) -> impl Iterator<Item = (usize, &t::GameRecord)> {
        self.payload
            .retired
            .iter()
            .map(|r| (r.control_index, native(&r.record).expect("validated Tori")))
    }
    /// All historical observations, never execution intents.
    pub fn observations(&self) -> &[ExecutionObservation] {
        &self.payload.observations
    }
    /// Prepare a new complete candidate, leaving this accepted value unchanged on refusal.
    pub fn with_control(
        &self,
        control: ToriControl,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, ToriDocumentError> {
        check(stopped)?;
        let mut payload = self.payload.clone();
        let mut game = native(&payload.current)?.to_game().map_err(invalid)?;
        match &control {
            ToriControl::Native { actor, command } => {
                game.submit(t::GameRequest {
                    expected: game.context(),
                    actor: *actor,
                    command: *command,
                })
                .map_err(native_error)?;
            }
            ToriControl::Settings { settings } => payload.settings = settings.clone(),
            ToriControl::Rewind { event_index, .. } => {
                if *event_index >= game.events().len() {
                    return Err(invalid("rewind must retire at least one event"));
                }
                payload.retired.push(Retired {
                    control_index: payload.controls.len(),
                    record: payload.current.clone(),
                });
                game = game
                    .record_prefix(*event_index)
                    .map_err(invalid)?
                    .to_game()
                    .map_err(invalid)?;
            }
            ToriControl::Observation { .. } => {
                return Err(invalid(
                    "use with_observation for atomic observation admission",
                ));
            }
            ToriControl::Close {} => {}
        }
        payload.current = CanonicalRecord::tori(t::GameRecord::from_game(&game).map_err(invalid)?);
        payload.controls.push(control);
        validate_payload(payload, stopped)
    }
    /// Refresh retired runtime high-water evidence without changing game semantics.
    /// Only validated immutable fields change; there is no new native replay.
    pub(crate) fn with_runtime_floor(&self, floor: u64) -> Result<Self, ToriDocumentError> {
        let mut doc = self.clone();
        doc.payload.retired_execution_floor = doc.payload.retired_execution_floor.max(floor);
        doc.bytes = document::encode(RulesProfile::ToriModernV1, &doc.payload).map_err(invalid)?;
        doc.usage.encoded_bytes = doc.bytes.len();
        // A later Save may widen only this monotonic decimal field. Reserve its
        // maximum supported spelling even after close/terminal space is consumed.
        check_runtime_bytes(
            &doc.usage,
            doc.payload.retired_execution_floor,
            reservation(&doc.payload),
        )?;
        check_limits(
            &doc.usage,
            doc.payload.retired_execution_floor,
            reservation(&doc.payload),
        )?;
        Ok(doc)
    }
    /// Consume already admitted close headroom. No rules/history reconstruction or
    /// native action runs on the close path; unchanged native validation is reused.
    pub(crate) fn with_reserved_close(&self, floor: u64) -> Result<Self, ToriDocumentError> {
        if matches!(self.payload.controls.last(), Some(ToriControl::Close {})) {
            return self.with_runtime_floor(floor);
        }
        let mut doc = self.clone();
        doc.payload.controls.push(ToriControl::Close {});
        doc.usage.controls += 1;
        doc.usage.replay_work += 1;
        doc.with_runtime_floor(floor)
    }
    /// Atomically retain an actual result and, for an engine turn, its re-admitted action.
    /// Read-only analysis never appends a native command.
    pub fn with_observation(
        &self,
        observation: ExecutionObservation,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, ToriDocumentError> {
        check(stopped)?;
        let mut payload = self.payload.clone();
        payload.retired_execution_floor = payload
            .retired_execution_floor
            .max(observation.context.request_id)
            .max(observation.context.process_generation)
            .max(observation.context.session_version);
        if let ExecutionPurpose::MoveAnalysis { target } = &observation.context.purpose {
            payload.retired_execution_floor =
                payload.retired_execution_floor.max(target.selection_id);
        }
        payload.controls.push(ToriControl::Observation {
            index: payload.observations.len(),
        });
        if let (
            ExecutionPurpose::EngineTurn {},
            ExecutionResult::Tori {
                result: mshogi_engine::ToriResult::Choice { action, .. },
            },
        ) = (&observation.context.purpose, &observation.result)
        {
            let mut game = native(&payload.current)?.to_game().map_err(invalid)?;
            let actor = game.position().side_to_move();
            let command = t::GameCommand::Play { action: *action };
            game.submit(t::GameRequest {
                expected: game.context(),
                actor,
                command,
            })
            .map_err(native_error)?;
            payload
                .controls
                .push(ToriControl::Native { actor, command });
            payload.current =
                CanonicalRecord::tori(t::GameRecord::from_game(&game).map_err(invalid)?);
        }
        payload.observations.push(observation);
        validate_payload(payload, stopped)
    }
}
fn native(record: &CanonicalRecord) -> Result<&t::GameRecord, ToriDocumentError> {
    if let CanonicalRecord::Tori(r) = record {
        Ok(r)
    } else {
        Err(invalid("record game must be Tori"))
    }
}
pub(crate) fn native_error(e: t::GameError) -> ToriDocumentError {
    if let t::GameError::Capacity(r) = e {
        let (cat, cap, unit) = match r {
            t::Resource::EncodedBytes => (
                ResourceCategory::CanonicalRecordBytes,
                t::MAX_GAME_RECORD_BYTES as u64,
                ResourceUnit::Bytes,
            ),
            t::Resource::Events => (
                ResourceCategory::NativeEvents,
                t::MAX_GAME_EVENTS as u64,
                ResourceUnit::Events,
            ),
            t::Resource::ReplayWork => (
                ResourceCategory::ReplayWork,
                t::MAX_REPLAY_WORK,
                ResourceUnit::ReplayUnits,
            ),
            t::Resource::Counter => (
                ResourceCategory::IdentityHeadroom,
                u64::MAX,
                ResourceUnit::Identities,
            ),
        };
        let mut fact = ResourceFact::measured(cat, 0, cap)
            .with_scope(ResourceScope::Tori)
            .with_unit(unit);
        fact.count = None; // Native refusal provides category but no candidate measurement.
        return OperationFailure::resource(ResourceFailure::new(fact, e.to_string())).into();
    }
    invalid(e)
}
fn check(stopped: &mut impl FnMut() -> bool) -> Result<(), ToriDocumentError> {
    persistence::check(stopped).map_err(invalid)
}
fn validate_payload(
    payload: Payload,
    stopped: &mut impl FnMut() -> bool,
) -> Result<ValidatedToriDocument, ToriDocumentError> {
    check(stopped)?;
    let bytes = document::encode(RulesProfile::ToriModernV1, &payload).map_err(invalid)?;
    ValidatedToriDocument::decode(&bytes, stopped)
}
fn reservation(payload: &Payload) -> Reserve {
    let raw =
        serde_json::value::to_raw_value(&payload.current).expect("validated serializable record");
    let close = matches!(payload.controls.last(), Some(ToriControl::Close {}));
    reserve_for(&raw, close).expect("validated native record")
}
fn reserve_for(raw: &RawValue, close: bool) -> Result<Reserve, ToriDocumentError> {
    #[derive(Deserialize)]
    struct Status {
        conclusion: Option<Box<RawValue>>,
        incomplete: Option<Box<RawValue>>,
    }
    let e: ProbeEnvelope = serde_json::from_str(raw.get()).map_err(invalid)?;
    let status: Status = serde_json::from_str(e.payload.get()).map_err(invalid)?;
    let native = if status.conclusion.is_some() {
        0
    } else {
        1 + usize::from(status.incomplete.is_none())
    };
    Ok(Reserve {
        controls: native + usize::from(!close),
        native,
    })
}
fn check_runtime_bytes(
    u: &ToriDocumentUsage,
    floor: u64,
    reserve: Reserve,
) -> Result<(), ToriDocumentError> {
    let width = (MAX_SAFE_ID - 65)
        .to_string()
        .len()
        .saturating_sub(floor.to_string().len());
    let extra = reserve.native * NATIVE_RESERVED_BYTES
        + (reserve.controls - reserve.native) * CLOSE_RESERVED_BYTES
        + width;
    if u.encoded_bytes.saturating_add(extra) > MAX_ARCHIVE_BYTES {
        return Err(limit(
            ResourceCategory::EncodedDocumentBytes,
            u.encoded_bytes as u64,
            MAX_ARCHIVE_BYTES as u64,
            extra as u64,
            ResourceUnit::Bytes,
        ));
    }
    Ok(())
}
fn check_limits(
    u: &ToriDocumentUsage,
    floor: u64,
    reserve: Reserve,
) -> Result<(), ToriDocumentError> {
    let close = reserve.controls - reserve.native;
    for (category, count, cap, extra, unit) in [
        (
            ResourceCategory::EncodedDocumentBytes,
            u.encoded_bytes as u64,
            MAX_ARCHIVE_BYTES as u64,
            (reserve.native * NATIVE_RESERVED_BYTES + close * CLOSE_RESERVED_BYTES) as u64,
            ResourceUnit::Bytes,
        ),
        (
            ResourceCategory::NativeEvents,
            u.retained_events as u64,
            MAX_ARCHIVE_EVENTS as u64,
            reserve.native as u64,
            ResourceUnit::Events,
        ),
        (
            ResourceCategory::Controls,
            u.controls as u64,
            MAX_ARCHIVE_CONTROLS as u64,
            reserve.controls as u64,
            ResourceUnit::Controls,
        ),
        (
            ResourceCategory::OrdinaryObservations,
            u.observations as u64,
            MAX_ARCHIVE_OBSERVATIONS as u64,
            0,
            ResourceUnit::Observations,
        ),
        (
            ResourceCategory::ReplayWork,
            u.replay_work,
            MAX_TORI_DOCUMENT_WORK,
            reserve.native as u64 * NATIVE_RESERVED_WORK + close as u64 * CLOSE_RESERVED_WORK,
            ResourceUnit::ReplayUnits,
        ),
        (
            ResourceCategory::IdentityHeadroom,
            floor,
            MAX_SAFE_ID - 1,
            64,
            ResourceUnit::Identities,
        ),
    ] {
        if count.saturating_add(extra) > cap {
            return Err(limit(category, count, cap, extra, unit));
        }
    }
    Ok(())
}
// Strict shape-only probes before native Deserialize performs any expensive replay.
#[derive(Deserialize)]
struct ProbeRecord {
    events: Vec<ProbeEvent>,
}
#[derive(Deserialize)]
struct ProbeEvent {
    command: ProbeCommand,
}
#[derive(Deserialize)]
struct ProbeCommand {
    kind: String,
}
#[derive(Deserialize)]
struct ProbeEnvelope {
    envelope_version: u32,
    game_kind: GameKind,
    rules_profile: RulesProfile,
    profile_edition: String,
    native_schema_version: u32,
    payload: Box<RawValue>,
}
fn record_charge(raw: &RawValue) -> Result<(usize, u64), ToriDocumentError> {
    if raw.get().len() > mshogi_core::native::MAX_RECORD_ENVELOPE_BYTES {
        return Err(limit(
            ResourceCategory::CanonicalRecordBytes,
            raw.get().len() as u64,
            mshogi_core::native::MAX_RECORD_ENVELOPE_BYTES as u64,
            0,
            ResourceUnit::Bytes,
        ));
    }
    let envelope: ProbeEnvelope = serde_json::from_str(raw.get()).map_err(invalid)?;
    if envelope.envelope_version != mshogi_core::native::RECORD_ENVELOPE_VERSION
        || envelope.game_kind != GameKind::Tori
        || envelope.rules_profile != RulesProfile::ToriModernV1
        || envelope.profile_edition != t::PROFILE_EDITION
        || envelope.native_schema_version != t::GAME_RECORD_SCHEMA_VERSION
    {
        return Err(invalid(
            "conflicting native canonical game/profile/edition/schema/envelope identity",
        ));
    }
    if envelope.payload.get().len() > t::MAX_GAME_RECORD_BYTES {
        return Err(limit(
            ResourceCategory::CanonicalRecordBytes,
            envelope.payload.get().len() as u64,
            t::MAX_GAME_RECORD_BYTES as u64,
            0,
            ResourceUnit::Bytes,
        ));
    }
    let record: ProbeRecord = serde_json::from_str(envelope.payload.get()).map_err(invalid)?;
    if record.events.len() > t::MAX_GAME_EVENTS {
        return Err(limit(
            ResourceCategory::NativeEvents,
            record.events.len() as u64,
            t::MAX_GAME_EVENTS as u64,
            0,
            ResourceUnit::Events,
        ));
    }
    let mut boards = 0u64;
    let mut work = ROOT_WORK;
    for e in &record.events {
        work += 1;
        match e.command.kind.as_str() {
            "play" => {
                boards += 1;
                work += boards;
            }
            "resign" | "stop" => {}
            _ => return Err(invalid("unsupported native command in record")),
        }
    }
    if work - ROOT_WORK + 1 > t::MAX_REPLAY_WORK {
        return Err(limit(
            ResourceCategory::ReplayWork,
            work - ROOT_WORK + 1,
            t::MAX_REPLAY_WORK,
            0,
            ResourceUnit::ReplayUnits,
        ));
    }
    Ok((record.events.len(), work))
}
#[derive(Deserialize)]
struct RawPayload {
    initial: Box<RawValue>,
    current: Box<RawValue>,
    controls: Vec<Box<RawValue>>,
    observations: Vec<Box<RawValue>>,
    retired: Vec<RawRetired>,
    retired_execution_floor: u64,
}
#[derive(Deserialize)]
struct RawRetired {
    record: Box<RawValue>,
}
fn preflight(
    bytes: &[u8],
    stopped: &mut impl FnMut() -> bool,
) -> Result<ToriDocumentUsage, ToriDocumentError> {
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
    unique(bytes)?;
    let envelope = document::DocumentEnvelope::parse(bytes).map_err(invalid)?;
    if envelope.game_kind != GameKind::Tori {
        return Err(invalid("wrong desktop game"));
    }
    let p: RawPayload = serde_json::from_str(envelope.payload.get()).map_err(invalid)?;
    let close = p.controls.last().is_some_and(|c| {
        serde_json::from_str::<ToriControl>(c.get())
            .is_ok_and(|c| matches!(c, ToriControl::Close {}))
    });
    let reserve = reserve_for(&p.current, close)?;
    let mut usage = ToriDocumentUsage {
        encoded_bytes: bytes.len(),
        retained_events: 0,
        controls: p.controls.len(),
        observations: p.observations.len(),
        replay_work: 0,
    };
    check_limits(&usage, p.retired_execution_floor, reserve)?;
    let (initial_events, initial_work) = record_charge(&p.initial)?;
    let (current_events, current_work) = record_charge(&p.current)?;
    if p.retired_execution_floor == 0 {
        return Err(invalid("zero retired execution floor"));
    }
    for (category, count, cap, extra, unit) in [
        (
            ResourceCategory::NativeEvents,
            current_events as u64,
            t::MAX_GAME_EVENTS as u64,
            reserve.native as u64,
            ResourceUnit::Events,
        ),
        (
            ResourceCategory::ReplayWork,
            current_work - ROOT_WORK + 1,
            t::MAX_REPLAY_WORK,
            reserve.native as u64,
            ResourceUnit::ReplayUnits,
        ),
    ] {
        if count + extra > cap {
            return Err(limit(category, count, cap, extra, unit));
        }
    }
    let e: ProbeEnvelope = serde_json::from_str(p.current.get()).map_err(invalid)?;
    let native_bytes = e.payload.get().len();
    if native_bytes + reserve.native * 4096 > t::MAX_GAME_RECORD_BYTES {
        return Err(limit(
            ResourceCategory::CanonicalRecordBytes,
            native_bytes as u64,
            t::MAX_GAME_RECORD_BYTES as u64,
            (reserve.native * 4096) as u64,
            ResourceUnit::Bytes,
        ));
    }
    usage.retained_events = initial_events + current_events;
    // initial parse + reconstruct; current parse; later final encode. Journal native
    // command replay and every prefix reconstruction are separately charged below.
    usage.replay_work = 2 * initial_work + current_work;
    for r in &p.retired {
        let (n, w) = record_charge(&r.record)?;
        usage.retained_events += n;
        usage.replay_work += w;
    }
    let mut board_ply = 0u64;
    // Upper bound includes the imported initial history; never rely on stored work.
    let initial: ProbeEnvelope = serde_json::from_str(p.initial.get()).map_err(invalid)?;
    let initial: ProbeRecord = serde_json::from_str(initial.payload.get()).map_err(invalid)?;
    board_ply += initial
        .events
        .iter()
        .filter(|e| e.command.kind == "play")
        .count() as u64;
    for c in &p.controls {
        check(stopped)?;
        let c: ToriControl = serde_json::from_str(c.get()).map_err(invalid)?;
        usage.replay_work += 1;
        match c {
            ToriControl::Native {
                command: t::GameCommand::Play { .. },
                ..
            } => {
                board_ply += 1;
                usage.replay_work += board_ply + 1;
            }
            ToriControl::Rewind { .. } => {
                // Conservative: every rewind may reconstruct any admitted native
                // record. Does not trust the target or cached retired ledger.
                usage.replay_work += 2 * ROOT_WORK + t::MAX_REPLAY_WORK;
            }
            _ => {}
        }
        check_limits(&usage, p.retired_execution_floor, reserve)?;
    }
    #[derive(Deserialize)]
    struct Obs {
        context: Context,
        result: Box<RawValue>,
    }
    #[derive(Deserialize)]
    struct Context {
        game: Box<RawValue>,
    }
    for o in &p.observations {
        check(stopped)?;
        let o: Obs = serde_json::from_str(o.get()).map_err(invalid)?;
        let (n, w) = record_charge(&o.context.game)?;
        usage.retained_events += n;
        usage.replay_work += w + ROOT_WORK; // parse + exact prefix projection
        // Actual result validation reconstructs its native root and re-admits every
        // complete PV action. Count this before any expensive typed deserialization.
        #[derive(Deserialize)]
        struct ResultProbe {
            kind: String,
            result: Option<NativeResultProbe>,
        }
        #[derive(Deserialize)]
        struct NativeResultProbe {
            telemetry: TelemetryProbe,
        }
        #[derive(Deserialize)]
        struct TelemetryProbe {
            principal_variation: Vec<Box<RawValue>>,
        }
        let result: ResultProbe = serde_json::from_str(o.result.get()).map_err(invalid)?;
        if result.kind == "tori" {
            let pv = result
                .result
                .ok_or_else(|| invalid("missing Tori result"))?
                .telemetry
                .principal_variation
                .len();
            if pv > usize::from(mshogi_engine::TORI_MAX_DEPTH) {
                return Err(invalid("Tori PV exceeds supported depth"));
            }
            // Charge native context parse, exact prefix encoding, target reconstruction
            // and private validation-branch reconstruction conservatively (4 roots
            // in total). Each PV submit serializes its growing native history.
            usage.replay_work += 3 * w + (pv as u64) * (n as u64 + pv as u64 + ROOT_WORK);
        }
        check_limits(&usage, p.retired_execution_floor, reserve)?;
    }
    check_limits(&usage, p.retired_execution_floor, reserve)?;
    Ok(usage)
}
// Deserialize each expensive native context separately so the existing worker
// can retire/cancel between native replay invocations (each still native-bounded).
fn parse_payload(
    text: &str,
    stopped: &mut impl FnMut() -> bool,
) -> Result<Payload, ToriDocumentError> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Wire {
        mode: String,
        initial: Box<RawValue>,
        current: Box<RawValue>,
        human: t::Side,
        initial_settings: ToriSettings,
        settings: ToriSettings,
        controls: Vec<ToriControl>,
        observations: Vec<Box<RawValue>>,
        retired: Vec<Branch>,
        retired_execution_floor: u64,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Branch {
        control_index: usize,
        record: Box<RawValue>,
    }
    let wire: Wire = serde_json::from_str(text).map_err(invalid)?;
    let mut record = |r: &RawValue| {
        check(stopped)?;
        let r = CanonicalRecord::from_json(r.get()).map_err(invalid)?;
        check(stopped)?;
        Ok::<_, ToriDocumentError>(r)
    };
    let initial = record(&wire.initial)?;
    let current = record(&wire.current)?;
    let mut retired = Vec::new();
    for r in wire.retired {
        retired.push(Retired {
            control_index: r.control_index,
            record: record(&r.record)?,
        });
    }
    let mut observations = Vec::new();
    for o in wire.observations {
        check(stopped)?;
        observations.push(serde_json::from_str(o.get()).map_err(invalid)?);
        check(stopped)?;
    }
    Ok(Payload {
        mode: wire.mode,
        initial,
        current,
        human: wire.human,
        initial_settings: wire.initial_settings,
        settings: wire.settings,
        controls: wire.controls,
        observations,
        retired,
        retired_execution_floor: wire.retired_execution_floor,
    })
}
fn validate_semantics(
    p: &Payload,
    stopped: &mut impl FnMut() -> bool,
) -> Result<(), ToriDocumentError> {
    if p.mode != "untimed" {
        return Err(invalid("only untimed Tori documents are supported"));
    }
    let mut game = native(&p.initial)?.to_game().map_err(invalid)?;
    native(&p.current)?;
    let mut settings = p.initial_settings.clone();
    let mut retired = 0;
    let mut observation = 0;
    let mut required_action: Option<(t::Side, t::GameCommand)> = None;
    for (i, c) in p.controls.iter().enumerate() {
        check(stopped)?;
        if let Some((actor, command)) = required_action.take()
            && !matches!(c, ToriControl::Native { actor: a, command: b } if *a == actor && *b == command)
        {
            return Err(invalid(
                "engine choice must be followed by its exact native action",
            ));
        }
        match c {
            ToriControl::Native { actor, command } => {
                game.submit(t::GameRequest {
                    expected: game.context(),
                    actor: *actor,
                    command: *command,
                })
                .map_err(native_error)?;
            }
            ToriControl::Settings { settings: s } => settings = s.clone(),
            ToriControl::Rewind { event_index, .. } => {
                if *event_index >= game.events().len() {
                    return Err(invalid("invalid rewind prefix"));
                }
                let saved = p
                    .retired
                    .get(retired)
                    .ok_or_else(|| invalid("missing retired branch"))?;
                if saved.control_index != i
                    || native(&saved.record)?
                        != &t::GameRecord::from_game(&game).map_err(invalid)?
                {
                    return Err(invalid("retired branch disagrees with native journal"));
                }
                game = game
                    .record_prefix(*event_index)
                    .map_err(invalid)?
                    .to_game()
                    .map_err(invalid)?;
                retired += 1;
            }
            ToriControl::Observation { index } => {
                if *index != observation {
                    return Err(invalid("observation ordering disagrees"));
                }
                let o = p
                    .observations
                    .get(*index)
                    .ok_or_else(|| invalid("missing observation"))?;
                if !o.matches(&o.context)
                    || !matches!(
                        o.result,
                        ExecutionResult::Failure { .. } | ExecutionResult::Tori { .. }
                    )
                {
                    return Err(invalid("unsupported or invalid Tori observation"));
                }
                let n = native(&o.context.game)?;
                let event_index = match &o.context.purpose {
                    ExecutionPurpose::EngineTurn {} => game.events().len(),
                    ExecutionPurpose::MoveAnalysis { target } => {
                        usize::try_from(target.event_index).map_err(invalid)?
                    }
                    _ => return Err(invalid("unsupported Tori analysis operation")),
                };
                if &game.record_prefix(event_index).map_err(invalid)? != n {
                    return Err(invalid("observation target is not the exact known prefix"));
                }
                if let ExecutionResult::Tori { result } = &o.result {
                    let target_game = n.to_game().map_err(invalid)?;
                    result.validate(&target_game).map_err(invalid)?;
                    if let (
                        ExecutionPurpose::EngineTurn {},
                        mshogi_engine::ToriResult::Choice { action, .. },
                    ) = (&o.context.purpose, result)
                    {
                        required_action = Some((
                            game.position().side_to_move(),
                            t::GameCommand::Play { action: *action },
                        ));
                    }
                }
                for id in [
                    match &o.context.purpose {
                        ExecutionPurpose::MoveAnalysis { target } => target.selection_id,
                        _ => o.context.session_version,
                    },
                    o.context.request_id,
                    o.context.session_version,
                    o.context.process_generation,
                ] {
                    if id == 0 || id > p.retired_execution_floor {
                        return Err(invalid("observation execution identity/floor disagrees"));
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
                    return Err(invalid("observation exceeds execution encoded limits"));
                }
                observation += 1;
            }
            ToriControl::Close {} => {}
        }
    }
    if required_action.is_some()
        || observation != p.observations.len()
        || retired != p.retired.len()
        || settings != p.settings
        || native(&p.current)? != &t::GameRecord::from_game(&game).map_err(invalid)?
    {
        return Err(invalid(
            "native journal/current/settings/retained evidence disagree",
        ));
    }
    Ok(())
}
// Validate duplicate fields before any internally tagged enum buffering or native
// deserialization can normalize them. This has no game-specific rule knowledge.
fn unique(bytes: &[u8]) -> Result<(), ToriDocumentError> {
    struct Unique;
    impl<'de> Deserialize<'de> for Unique {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            d.deserialize_any(UVisitor)
        }
    }
    struct UVisitor;
    impl<'de> Visitor<'de> for UVisitor {
        type Value = Unique;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("duplicate-free JSON")
        }
        fn visit_map<M: MapAccess<'de>>(self, mut m: M) -> Result<Unique, M::Error> {
            let mut keys = BTreeSet::new();
            while let Some(k) = m.next_key::<String>()? {
                if !keys.insert(k) {
                    return Err(de::Error::custom("duplicate field"));
                }
                m.next_value::<Unique>()?;
            }
            Ok(Unique)
        }
        fn visit_seq<S: SeqAccess<'de>>(self, mut s: S) -> Result<Unique, S::Error> {
            while s.next_element::<Unique>()?.is_some() {}
            Ok(Unique)
        }
        fn visit_bool<E: de::Error>(self, _: bool) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_u64<E: de::Error>(self, _: u64) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_i64<E: de::Error>(self, _: i64) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_f64<E: de::Error>(self, _: f64) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_str<E: de::Error>(self, _: &str) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_unit<E: de::Error>(self) -> Result<Unique, E> {
            Ok(Unique)
        }
    }
    let mut d = serde_json::Deserializer::from_slice(bytes);
    Unique::deserialize(&mut d).map_err(invalid)?;
    d.end().map_err(invalid)
}

/// Result of a pure codec/file operation, never permission to install live state.
pub enum ToriFileResult {
    /// Complete validated immutable candidate.
    Validated(Box<ValidatedToriDocument>),
    /// Atomic rename committed for this exact revision and document identity.
    Saved {
        /// Actual path successfully replaced.
        path: std::path::PathBuf,
        /// Exact content revision submitted, not a clean-state instruction.
        revision: u64,
        /// Exact document identity submitted, not a runtime restoration.
        identity: u64,
    },
}
/// Thin codec-only access to the existing bounded FileWorker and its completion
/// family. Holds no live document, session, game process or application state.
pub struct ToriDocumentWorker {
    worker: persistence::FileWorker,
    last_id: u64,
}
impl ToriDocumentWorker {
    /// Start the existing file worker; no engine or application is launched.
    pub fn new() -> std::io::Result<Self> {
        Ok(Self {
            worker: persistence::FileWorker::new()?,
            last_id: 0,
        })
    }
    fn submit(&mut self, id: u64, work: CandidateWork) -> Result<(), ToriDocumentError> {
        let bytes = match &work {
            CandidateWork::Validate(bytes) | CandidateWork::Save { bytes, .. } => bytes.len(),
        };
        if bytes > MAX_ARCHIVE_BYTES {
            return Err(limit(
                ResourceCategory::EncodedDocumentBytes,
                bytes as u64,
                MAX_ARCHIVE_BYTES as u64,
                0,
                ResourceUnit::Bytes,
            ));
        }
        if id <= self.last_id || id >= MAX_SAFE_ID - 64 {
            return Err(invalid("stale or exhausted file operation identity"));
        }
        self.last_id = id;
        self.worker.submit(
            id,
            persistence::Work::Native(Box::new(shared::archive::NativeWork::Tori(work))),
        );
        Ok(())
    }
    /// Replace any older validation/save job; old completions cannot reappear.
    pub fn validate(&mut self, id: u64, bytes: Vec<u8>) -> Result<(), ToriDocumentError> {
        self.submit(id, CandidateWork::Validate(bytes))
    }
    /// Queue complete candidate validation followed by the existing atomic writer.
    pub fn save(
        &mut self,
        id: u64,
        bytes: Vec<u8>,
        path: std::path::PathBuf,
        revision: u64,
        identity: u64,
    ) -> Result<(), ToriDocumentError> {
        self.submit(
            id,
            CandidateWork::Save {
                bytes,
                path,
                revision,
                identity,
            },
        )
    }
    /// Retire work and its completion without mutating accepted document contents.
    pub fn cancel(&self) {
        self.worker.cancel();
    }
    /// One bounded completion, keyed by operation identity. A Saved receipt must
    /// still be checked by the runtime against its current identity and revision.
    pub fn poll(&self) -> Option<(u64, Result<ToriFileResult, ToriDocumentError>)> {
        self.worker.poll().map(|(id, result)| {
            (
                id,
                result
                    .map_err(ToriDocumentError::from)
                    .and_then(|c| match c {
                        persistence::Completed::Native(c) => match *c {
                            shared::archive::NativeCompleted::ToriValidated(doc) => {
                                Ok(ToriFileResult::Validated(doc))
                            }
                            shared::archive::NativeCompleted::Saved {
                                path,
                                revision,
                                identity,
                            } => Ok(ToriFileResult::Saved {
                                path,
                                revision,
                                identity,
                            }),
                            _ => Err(invalid("unexpected native file completion")),
                        },
                        _ => Err(invalid("unexpected file completion")),
                    }),
            )
        })
    }
}

// Pure candidate work only. No OpenedNative or ActiveDocument variant is added.
pub(crate) enum CandidateWork {
    Validate(Vec<u8>),
    Save {
        bytes: Vec<u8>,
        path: std::path::PathBuf,
        revision: u64,
        identity: u64,
    },
}
pub(crate) fn execute(
    work: CandidateWork,
    stopped: &mut impl FnMut() -> bool,
) -> Result<shared::archive::NativeCompleted, OperationFailure> {
    match work {
        CandidateWork::Validate(bytes) => Ok(shared::archive::NativeCompleted::ToriValidated(
            Box::new(ValidatedToriDocument::decode(&bytes, stopped)?),
        )),
        CandidateWork::Save {
            bytes,
            path,
            revision,
            identity,
        } => {
            if revision == 0
                || identity == 0
                || revision >= MAX_SAFE_ID - 64
                || identity >= MAX_SAFE_ID - 64
            {
                return Err(limit(
                    ResourceCategory::IdentityHeadroom,
                    revision.max(identity),
                    MAX_SAFE_ID - 65,
                    0,
                    ResourceUnit::Identities,
                )
                .into());
            }
            let doc = ValidatedToriDocument::decode(&bytes, stopped)?;
            persistence::atomic_write(&path, doc.bytes(), stopped)?;
            Ok(shared::archive::NativeCompleted::Saved {
                path,
                revision,
                identity,
            })
        }
    }
}
#[cfg(test)]
mod tests;
