//! Xiangqi document replay/admission. No graph verifier or frontend rules execute here.
use super::*;
use crate::persistence;
use serde_json::value::RawValue;

/// Persisted native preferences; execution ownership is never a setting.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct XiangqiSettings {
    /// Native side at the bottom, independent of human ownership.
    pub orientation: x::Side,
    /// Ordinary sound-fast-only analysis defaults.
    pub analysis_defaults: WorkLimits,
}
impl Default for XiangqiSettings {
    fn default() -> Self {
        Self {
            orientation: x::Side::Red,
            analysis_defaults: PLAY_LIMITS,
        }
    }
}
fn valid_limits(limits: WorkLimits) -> Result<(), String> {
    if !(1..=3).contains(&limits.depth) || !(1..=512).contains(&limits.nodes) {
        return Err(
            "Xiangqi analysis requires depth1–3 and1–512 visits; no downgrade was applied.".into(),
        );
    }
    Ok(())
}
fn capacity(message: &str) -> String {
    format!(
        "Application capacity: {message} The complete accepted document is preserved and saveable. Saving/reopening cannot enlarge this limit; start a new document to continue."
    )
}
fn error(e: impl ToString) -> String {
    super::super::archive::bounded(e)
}
fn native(record: &CanonicalRecord) -> Result<&x::GameRecord, String> {
    match record {
        CanonicalRecord::Xiangqi(r) => Ok(r),
        _ => Err("Wrong game in Xiangqi archive.".into()),
    }
}

/// Immutable completed native exact work, never a resumable request or an outcome.
/// On Open this is historical reported evidence, not a newly verified graph proof.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct XiangqiExactEvidence {
    format: String,
    executor: String,
    source: CanonicalRecord,
    purpose: String,
    goal: String,
    event_count: usize,
    historical_target: bool,
    position_json: String,
    native_session: u64,
    native_generation: u64,
    limits: x::ExactLimits,
    report: x::ExactReport,
}
impl XiangqiExactEvidence {
    /// Construct only from completed native work. This does not execute or accept work.
    pub fn from_native(observation: &x::ExactObservation) -> Self {
        let request = observation.request();
        let (native_session, native_generation) = request.identity();
        let (event_count, historical_target) = match request.target() {
            x::ExactTarget::Live => (request.source_record().event_count(), false),
            x::ExactTarget::History { event_count } => (event_count, true),
        };
        Self {
            format: "mshogi.xiangqi.exact-observation.v1".into(),
            executor: "native_in_process".into(),
            source: CanonicalRecord::xiangqi(request.source_record().clone()),
            purpose: "natural_board_win".into(),
            goal: x::NATURAL_WIN_GOAL.into(),
            event_count,
            historical_target,
            position_json: x::PositionRoot::analysis(request.position().clone()).to_json(),
            native_session,
            native_generation,
            limits: request.limits(),
            report: observation.report().clone(),
        }
    }
    /// Exact source history, separate from the analyzed event prefix.
    pub fn source(&self) -> &CanonicalRecord {
        &self.source
    }
    /// Actual reported proof, positive witness, Unknown/cancellation or failure.
    pub fn report(&self) -> &x::ExactReport {
        &self.report
    }
    /// Exact native event selection, not board ply.
    pub fn event_count(&self) -> usize {
        self.event_count
    }
    fn validate(
        &self,
        state: &XiangqiSession,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        persistence::check(stopped)?;
        if self.format != "mshogi.xiangqi.exact-observation.v1"
            || self.executor != "native_in_process"
            || self.purpose != "natural_board_win"
            || self.goal != x::NATURAL_WIN_GOAL
            || self.native_session == 0
            || self.native_generation == 0
            || self.native_session > MAX_SAFE_ID
            || self.native_generation > MAX_SAFE_ID
            || self.source
                != CanonicalRecord::xiangqi(x::GameRecord::from_game(&state.game).map_err(error)?)
            || self.historical_target != state.viewed.is_some()
            || self.event_count
                != state
                    .viewed
                    .as_ref()
                    .map_or(state.game.events().len(), |v| v.event_count)
        {
            return Err("Corrupt exact historical evidence: source, purpose, identity or event target differs.".into());
        }
        let root = x::PositionRoot::from_json(&self.position_json).map_err(error)?;
        if root != x::PositionRoot::analysis(state.selected_position().clone()) {
            return Err("Corrupt exact evidence target position/provenance.".into());
        }
        let unsupported = !self.limits.is_supported();
        if matches!(
            self.report.verdict,
            x::ExactVerdict::Failure {
                reason: x::ExactFailure::UnsupportedLimits
            }
        ) != unsupported
        {
            return Err("Corrupt exact evidence limits/failure contract.".into());
        }
        let work = &self.report.work;
        if work.states > self.limits.states
            || work.expanded > work.states
            || work.transitions > self.limits.transitions
            || work.cache_hits > work.transitions
            || work.memory_bytes > self.limits.memory_bytes
            || matches!(self.report.verdict, x::ExactVerdict::NeitherCanWin)
                && (work.states == 0 || work.expanded != work.states)
        {
            return Err(
                "Corrupt exact evidence work/closure metadata; no graph was executed.".into(),
            );
        }
        if let x::ExactVerdict::NaturalWin {
            winner,
            reason,
            witness,
        } = &self.report.verdict
        {
            if witness.len() > 4096 || witness.len() > self.limits.witness_plies as usize {
                return Err(capacity("Exact witness output exceeds its recorded bound."));
            }
            if witness_work(witness.len()) > MAX_REPLAY_WORK {
                return Err(capacity(
                    "Exact witness validation exceeds replay capacity.",
                ));
            }
            let p = state.selected_position();
            let mut p = x::Position::from_diagram(
                &p.pieces().collect::<Vec<_>>(),
                p.side_to_move(),
                x::Counters::unknown(),
            )
            .map_err(error)?;
            for &action in witness {
                persistence::check(stopped)?;
                if !matches!(p.local_status(), Ok(x::LocalStatus::Ongoing { .. })) {
                    return Err("Corrupt exact witness: action after a natural terminal.".into());
                }
                p.apply(action).map_err(error)?;
            }
            if p.side_to_move().opponent() != *winner
                || p.local_status().map_err(error)?
                    != (x::LocalStatus::NoLegalAction { reason: *reason })
            {
                return Err(
                    "Corrupt exact natural-board witness; repetition penalties are not witnesses."
                        .into(),
                );
            }
        }
        // Negative graph closure is deliberately NOT recomputed here. Reports retain
        // their provenance as historical observations and cannot adjudicate the game.
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct XiangqiControl {
    sequence: usize,
    native_events_before: usize,
    native_board_actions_before: usize,
    #[serde(deserialize_with = "super::super::archive::required_option")]
    command: Option<SessionOperation>,
    #[serde(deserialize_with = "super::super::archive::required_option")]
    observation: Option<usize>,
    #[serde(deserialize_with = "super::super::archive::required_option")]
    exact_observation: Option<usize>,
    #[serde(deserialize_with = "super::super::archive::required_option")]
    execution: Option<ExecutionBinding>,
    close_pause: bool,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ExecutionBinding {
    purpose: SharedPurpose,
    request_id: u64,
    session_version: u64,
    process_generation: u64,
    limits: WorkLimits,
}
impl ExecutionBinding {
    fn of(r: &SharedRequest) -> Self {
        Self {
            purpose: r.purpose.clone(),
            request_id: r.request_id,
            session_version: r.session_version,
            process_generation: r.process_generation,
            limits: r.limits,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct XiangqiArchive {
    engine_protocol: u32,
    play_limits: WorkLimits,
    initial: CanonicalRecord,
    current: CanonicalRecord,
    human: NativeSide,
    initial_settings: XiangqiSettings,
    settings: XiangqiSettings,
    controls: Vec<XiangqiControl>,
    observations: Vec<PresentedObservation>,
    exact_observations: Vec<XiangqiExactEvidence>,
    retired_execution_floor: u64,
}
// The controller only captures already-admitted immutable values. Native replay,
// prefix reconstruction and all candidate transitions happen on the bounded worker.
pub(crate) struct XiangqiDraft {
    saved: XiangqiArchive,
    viewed: Option<usize>,
    paused: bool,
    phase: EnginePhase,
    diagnostic: Option<String>,
    analysis: Option<PresentedObservation>,
    analysis_intent: Option<(CanonicalRecord, WorkLimits)>,
    pending: Option<SharedRequest>,
    submitted: bool,
    durable_needs_stop: bool,
    observation_bytes: usize,
}
impl XiangqiDraft {
    pub(crate) fn replacement(record: &x::GameRecord, human: x::Side) -> Self {
        let settings = XiangqiSettings {
            orientation: human,
            ..XiangqiSettings::default()
        };
        Self {
            saved: XiangqiArchive {
                engine_protocol: SHARED_PROTOCOL_VERSION,
                play_limits: PLAY_LIMITS,
                initial: CanonicalRecord::xiangqi(record.clone()),
                current: CanonicalRecord::xiangqi(record.clone()),
                human: NativeSide::Xiangqi(human),
                initial_settings: settings.clone(),
                settings,
                controls: vec![],
                observations: vec![],
                exact_observations: vec![],
                retired_execution_floor: 0,
            },
            viewed: None,
            paused: true,
            phase: EnginePhase::Paused,
            diagnostic: None,
            analysis: None,
            analysis_intent: None,
            pending: None,
            submitted: false,
            durable_needs_stop: false,
            observation_bytes: 0,
        }
    }
    pub(crate) fn capture(state: &XiangqiSession) -> Self {
        Self {
            saved: archive(state, state.execution_floor),
            viewed: state.viewed.as_ref().map(|v| v.event_count),
            paused: state.paused,
            phase: state.phase,
            diagnostic: state.diagnostic.clone(),
            analysis: state.analysis.clone(),
            analysis_intent: state.analysis_intent.clone(),
            pending: state.pending.clone(),
            submitted: state.submitted,
            durable_needs_stop: state.durable_needs_stop,
            observation_bytes: state.observation_bytes,
        }
    }
    fn materialize(self) -> Result<XiangqiSession, String> {
        let NativeSide::Xiangqi(human) = self.saved.human else {
            return Err("Wrong native human side.".into());
        };
        let (game, undo) = native(&self.saved.current)?
            .to_game_with_undo()
            .map_err(error)?;
        let viewed = self
            .viewed
            .map(|count| game.event_prefix(count).map_err(error))
            .transpose()?;
        let viewed_game = self
            .viewed
            .and_then(|count| game.committed_prefix(count).ok());
        let observation_bytes = self.observation_bytes;
        Ok(XiangqiSession {
            initial_record: native(&self.saved.initial)?.clone(),
            initial_settings: self.saved.initial_settings,
            settings: self.saved.settings,
            controls: self.saved.controls,
            exact_observations: self.saved.exact_observations,
            game,
            undo,
            human,
            viewed,
            viewed_game,
            paused: self.paused,
            pending: self.pending,
            submitted: self.submitted,
            phase: self.phase,
            diagnostic: self.diagnostic,
            analysis: self.analysis,
            analysis_intent: self.analysis_intent,
            durable_needs_stop: self.durable_needs_stop,
            execution_floor: self.saved.retired_execution_floor,
            observations: self.saved.observations,
            observation_bytes,
        })
    }
}
fn archive(state: &XiangqiSession, floor: u64) -> XiangqiArchive {
    XiangqiArchive {
        engine_protocol: SHARED_PROTOCOL_VERSION,
        play_limits: PLAY_LIMITS,
        initial: CanonicalRecord::xiangqi(state.initial_record.clone()),
        current: CanonicalRecord::xiangqi(
            x::GameRecord::from_game(&state.game).expect("native admitted record"),
        ),
        human: NativeSide::Xiangqi(state.human),
        initial_settings: state.initial_settings.clone(),
        settings: state.settings.clone(),
        controls: state.controls.clone(),
        observations: state.observations.clone(),
        exact_observations: state.exact_observations.clone(),
        retired_execution_floor: floor,
    }
}
pub(crate) fn encode(state: &XiangqiSession, floor: u64) -> Result<Vec<u8>, String> {
    #[derive(Serialize)]
    struct Envelope {
        schema_version: u32,
        format: &'static str,
        game_kind: &'static str,
        rules_profile: &'static str,
        profile_edition: &'static str,
        payload: XiangqiArchive,
    }
    serde_json::to_vec_pretty(&Envelope {
        schema_version: 2,
        format: "mshogi.desktop.archive.v2",
        game_kind: "xiangqi",
        rules_profile: x::PROFILE_ID,
        profile_edition: x::PROFILE_EDITION,
        payload: archive(state, floor),
    })
    .map_err(error)
}
pub(crate) fn needs_stop(state: &XiangqiSession) -> bool {
    state.pending.is_some()
        || state.analysis_intent.is_some()
        || !state.paused
        || state.durable_needs_stop
}
const MAX_REPLAY_WORK: usize = 16_000;
const EVENT_WORK: usize = 4;
// Measured WXF replay is materially more costly for board actions than same-ply
// claim audits. Keep the existing budget and charge actual board admissions too.
const BOARD_WORK: usize = 12;
// Uncached root/PV validation measured ~34ms on the local debug host. Charge
// conservatively (32 ordinary board-action equivalents), even at zero history.
const NATIVE_VALIDATION_WORK: usize = 512;
const PV_ACTION_WORK: usize = 16;
// Witness replay performs repeated local terminal/legality checks, not just
// transitions. It is not cached; every saved positive report pays its full cost.
const WITNESS_ACTION_WORK: usize = 64;
fn witness_work(actions: usize) -> usize {
    NATIVE_VALIDATION_WORK.saturating_add(actions.saturating_mul(WITNESS_ACTION_WORK))
}
fn variation(result: &SharedResult) -> &[XiangqiChoice] {
    match result {
        SharedResult::Xiangqi {
            result:
                XiangqiResult::Choice { telemetry, .. }
                | XiangqiResult::Finished { telemetry, .. }
                | XiangqiResult::Incomplete { telemetry }
                | XiangqiResult::Unresolved { telemetry, .. },
        } => &telemetry.principal_variation,
        _ => &[],
    }
}
fn charge_variation(
    seen: &mut std::collections::HashSet<(String, String)>,
    source: String,
    pv: &[XiangqiChoice],
) -> Result<usize, String> {
    if pv.is_empty() || !seen.insert((source, serde_json::to_string(pv).map_err(error)?)) {
        return Ok(0);
    }
    Ok(NATIVE_VALIDATION_WORK.saturating_add(pv.len().saturating_mul(PV_ACTION_WORK)))
}
fn record_work(events: usize, board_actions: usize) -> usize {
    events
        .saturating_mul(EVENT_WORK)
        .saturating_add(board_actions.saturating_mul(BOARD_WORK))
}
fn canonical_work(record: &CanonicalRecord) -> usize {
    record_work(record.event_count(), record.known_board_actions())
}
fn control_work(c: &XiangqiControl) -> usize {
    match c.command {
        Some(
            SessionOperation::Undo {}
            | SessionOperation::TakeBack {}
            | SessionOperation::Inspect { .. },
        ) => record_work(c.native_events_before, c.native_board_actions_before),
        _ => 1,
    }
}
fn exact_work(e: &XiangqiExactEvidence) -> usize {
    canonical_work(&e.source)
        .saturating_add(match &e.report.verdict {
            x::ExactVerdict::NaturalWin { witness, .. } => witness_work(witness.len()),
            _ => 0,
        })
        .saturating_add(8)
}
pub(crate) fn check_capacity(
    state: &XiangqiSession,
    stopped: &mut impl FnMut() -> bool,
) -> Result<(), String> {
    let reserve = usize::from(needs_stop(state));
    if state.controls.len().saturating_add(reserve) > MAX_ARCHIVE_CONTROLS
        || state.observations.len() + state.exact_observations.len() > MAX_ARCHIVE_OBSERVATIONS
    {
        return Err(capacity(
            "The archive retains256 total ordinary/exact observations and8192 controls including stop/close.",
        ));
    }
    let mut work = record_work(
        state.initial_record.event_count(),
        state.initial_record.known_board_actions(),
    )
    .saturating_add(record_work(
        state.game.events().len(),
        state.board_actions(),
    ))
    .saturating_add(reserve);
    for c in &state.controls {
        work = work.saturating_add(control_work(c));
    }
    let mut variations = std::collections::HashSet::new();
    for o in &state.observations {
        work = work.saturating_add(canonical_work(&o.response.context.game) + 8);
        work = work.saturating_add(charge_variation(
            &mut variations,
            o.response.context.game.to_json().map_err(error)?,
            variation(&o.response.result),
        )?);
    }
    for e in &state.exact_observations {
        work = work.saturating_add(exact_work(e));
    }
    if work > MAX_REPLAY_WORK {
        return Err(capacity(
            "The complete document exceeds16,000 weighted native replay work units.",
        ));
    }
    persistence::check(stopped)?;
    // Budget maximum-width future identities and the longer false historical flag.
    let bytes = encode(state, MAX_SAFE_ID - 17)?;
    // false is one byte longer than true; initial accepted flags may be either.
    if bytes
        .len()
        .saturating_add(state.observations.iter().filter(|o| o.historical).count())
        .saturating_add(if reserve != 0 { 1024 } else { 0 })
        > MAX_ARCHIVE_BYTES
    {
        return Err(capacity(
            "The actual complete encoded archive exceeds16MiB including reserved identity/stop/close headroom.",
        ));
    }
    encoded_native_bounds(&bytes)?;
    persistence::check(stopped)?;
    Ok(())
}

// Inspect the actual pretty-encoded wrappers, including their enclosing indentation.
// Borrowed RawValues avoid copying history or invoking native replay on this probe.
fn encoded_native_bounds(bytes: &[u8]) -> Result<(), String> {
    #[derive(Deserialize)]
    struct Envelope<'a> {
        #[serde(borrow)]
        payload: Payload<'a>,
    }
    #[derive(Deserialize)]
    struct Payload<'a> {
        #[serde(borrow)]
        initial: &'a RawValue,
        #[serde(borrow)]
        current: &'a RawValue,
        #[serde(borrow)]
        observations: Vec<&'a RawValue>,
        #[serde(borrow)]
        exact_observations: Vec<&'a RawValue>,
    }
    #[derive(Deserialize)]
    struct Observation<'a> {
        #[serde(borrow)]
        response: Response<'a>,
    }
    #[derive(Deserialize)]
    struct Response<'a> {
        #[serde(borrow)]
        context: Context<'a>,
    }
    #[derive(Deserialize)]
    struct Context<'a> {
        #[serde(borrow)]
        game: &'a RawValue,
    }
    #[derive(Deserialize)]
    struct Exact<'a> {
        #[serde(borrow)]
        source: &'a RawValue,
    }
    fn fits(raw: &RawValue) -> Result<(), String> {
        if raw.get().len() > mshogi_core::native::MAX_RECORD_ENVELOPE_BYTES {
            return Err(capacity(
                "An actual encoded canonical wrapper exceeds1MiB including archive indentation.",
            ));
        }
        Ok(())
    }
    let envelope: Envelope<'_> = serde_json::from_slice(bytes).map_err(error)?;
    fits(envelope.payload.initial)?;
    fits(envelope.payload.current)?;
    for raw in envelope.payload.observations {
        let observation: Observation<'_> = serde_json::from_str(raw.get()).map_err(error)?;
        fits(observation.response.context.game)?;
    }
    for raw in envelope.payload.exact_observations {
        let evidence: Exact<'_> = serde_json::from_str(raw.get()).map_err(error)?;
        fits(evidence.source)?;
    }
    Ok(())
}

impl XiangqiSession {
    // Every seeded control is an actual admitted preference with a replayable journal.
    #[cfg(feature = "desktop-verification")]
    pub(crate) fn verification_control_capacity(&mut self) -> Result<(), String> {
        while self.controls.len() < MAX_ARCHIVE_CONTROLS - 1 {
            let before = (self.game.events().len(), self.board_actions());
            let operation = SessionOperation::XiangqiPreferences {
                settings: self.settings.clone(),
            };
            self.apply_command(operation.clone())?;
            self.journal(before, Some(operation), None, None, false);
        }
        check_capacity(self, &mut || false)
    }
    fn board_actions(&self) -> usize {
        self.game
            .events()
            .iter()
            .filter(|e| matches!(e.event, x::EventKind::BoardAction { .. }))
            .count()
    }
    fn journal(
        &mut self,
        before: (usize, usize),
        command: Option<SessionOperation>,
        observation: Option<usize>,
        exact_observation: Option<usize>,
        close_pause: bool,
    ) {
        self.controls.push(XiangqiControl {
            sequence: self.controls.len() + 1,
            native_events_before: before.0,
            native_board_actions_before: before.1,
            command,
            observation,
            exact_observation,
            execution: None,
            close_pause,
        });
    }
    pub(crate) fn close_pause(&mut self) -> bool {
        if !needs_stop(self) {
            return false;
        }
        let before = (self.game.events().len(), self.board_actions());
        self.retire();
        self.paused = true;
        self.durable_needs_stop = false;
        self.journal(before, None, None, None, true);
        true
    }
    fn apply_command(&mut self, operation: SessionOperation) -> Result<Option<WorkLimits>, String> {
        let mut analysis = None;
        let preferences = matches!(operation, SessionOperation::XiangqiPreferences { .. });
        match operation {
            SessionOperation::Xiangqi { .. } | SessionOperation::Resign {} => {
                let command = if let SessionOperation::Xiangqi { command } = operation { command } else { x::GameCommand::Resign };
                if self.viewed.is_some() { return Err("Historical viewing is read-only. Return to live explicitly.".into()); }
                if self.paused { return Err("Resume live play before submitting a native command.".into()); }
                let submission = self.game.submit(x::GameRequest { expected: self.game.context(), actor: self.human, command }).map_err(error)?;
                self.undo.push(submission.undo); self.retire();
            }
            SessionOperation::TakeBack {} => {
                if self.viewed.is_some() { return Err("Return to live before Take back.".into()); }
                let target = self.game.preceding_playable_decision(self.human).ok_or("No earlier playable human decision is available.")?;
                let prefix = self.game.committed_prefix(target).map_err(error)?;
                let record = x::GameRecord::from_game(&prefix).map_err(error)?;
                (self.game, self.undo) = record.to_game_with_undo().map_err(error)?;
                self.retire(); self.paused = true;
                self.diagnostic = Some("Position restored. Resume play when ready.".into());
                for o in &mut self.observations { o.historical = true; }
            }
            SessionOperation::Undo {} => {
                if self.viewed.is_some() || self.undo.is_empty() { return Err("No live native command to undo.".into()); }
                self.game.undo(self.undo.pop().unwrap()).map_err(error)?; self.retire(); self.paused = true;
            }
            SessionOperation::Inspect { event_index } => {
                let viewed = self.game.event_prefix(event_index).map_err(error)?;
                let viewed_game = self.game.committed_prefix(event_index).ok();
                self.retire(); self.viewed = Some(viewed); self.viewed_game = viewed_game; self.paused = true;
            }
            SessionOperation::ReturnLive {} => { self.retire(); self.viewed = None; self.viewed_game = None; self.paused = true; }
            SessionOperation::Cancel {} => { self.retire(); self.paused = true; self.phase = EnginePhase::Cancelled; }
            SessionOperation::Analyze { operation: AnalysisOperation::Moves { depth, nodes } } => {
                if self.viewed.is_some() && self.viewed_game.is_none() { return Err("This immutable event is inside an atomic native command. Analysis requires a committed command boundary; view and movement inspection remain available.".into()); }
                let limits = WorkLimits { depth, nodes }; valid_limits(limits)?;
                if !self.observation_capacity() { return Err(capacity("All observation capacity is retained; no new analysis started.")); }
                self.retire(); self.paused = true; self.settings.analysis_defaults = limits;
                self.analysis_intent = Some((self.record(), limits)); analysis = Some(limits);
            }
            SessionOperation::Analyze { .. } => return Err("Chess Full is not Xiangqi analysis. Native bounded exact no-win execution is a separate facility, not this ordinary operation.".into()),
            SessionOperation::Resume {} | SessionOperation::Retry {} => {
                if self.viewed.is_some() || self.game.conclusion().is_some() { return Err("Resume requires a live unconcluded game. Return to live first.".into()); }
                if !self.observation_capacity() { return Err(capacity("All retained observation slots are occupied; no new work started.")); }
                self.retire(); self.paused = false; self.phase = EnginePhase::Idle;
            }
            SessionOperation::XiangqiPreferences { settings } => { valid_limits(settings.analysis_defaults)?; self.settings = settings; }
            _ => return Err("Unsupported operation for the native Xiangqi session.".into()),
        }
        if !preferences {
            self.durable_needs_stop =
                self.pending.is_some() || self.analysis_intent.is_some() || !self.paused;
        }
        Ok(analysis)
    }
    fn apply_response(&mut self, response: SharedResponse) -> Result<(), String> {
        self.apply_response_reusing(response, &mut ValidatedVariations::default())
    }
    fn apply_response_reusing(
        &mut self,
        response: SharedResponse,
        variations: &mut ValidatedVariations,
    ) -> Result<(), String> {
        if serde_json::to_vec(&response.context).map_err(error)?.len()
            > mshogi_engine::MAX_REQUEST_BYTES
            || serde_json::to_vec(&response).map_err(error)?.len()
                > mshogi_engine::MAX_RESPONSE_BYTES
        {
            return Err(capacity(
                "Encoded observation exceeds its native transport bounds.",
            ));
        }
        if !response.matches(&response.context) {
            return Err("Corrupt native observation/context metadata.".into());
        }
        if self.observations.len() + self.exact_observations.len() >= MAX_ARCHIVE_OBSERVATIONS {
            return Err(capacity("No retained observation slot remains."));
        }
        let read_only = matches!(response.context.purpose, SharedPurpose::MoveAnalysis { .. });
        let expected = if read_only {
            let (record, limits) = self
                .analysis_intent
                .as_ref()
                .ok_or("Corrupt observation: no preceding analysis intent.")?;
            if *limits != response.context.limits {
                return Err("Corrupt observation analysis work limits.".into());
            }
            record.clone()
        } else {
            if !matches!(response.context.purpose, SharedPurpose::EngineTurn {})
                || self.paused
                || self.viewed.is_some()
                || self.game.conclusion().is_some()
                || self.game.pending_adjudication().is_some()
                || self.game.position().side_to_move() == self.human
                || response.context.limits != PLAY_LIMITS
            {
                return Err(
                    "Corrupt observation: no matching live native engine turn/limits.".into(),
                );
            }
            CanonicalRecord::xiangqi(x::GameRecord::from_game(&self.game).map_err(error)?)
        };
        if response.context.game != expected || self.record() != expected {
            return Err(
                "Corrupt observation: exact target history differs from its journal intent.".into(),
            );
        }
        if let SharedPurpose::MoveAnalysis { target } = &response.context.purpose
            && target.turn != NativeSide::Xiangqi(self.selected_position().side_to_move())
        {
            return Err("Corrupt observation target side.".into());
        }
        let telemetry = match &response.result {
            SharedResult::Xiangqi {
                result:
                    XiangqiResult::Choice { telemetry, .. }
                    | XiangqiResult::Finished { telemetry, .. }
                    | XiangqiResult::Incomplete { telemetry }
                    | XiangqiResult::Unresolved { telemetry, .. },
            } => Some(telemetry),
            SharedResult::Failure { .. } => None,
            _ => return Err("Wrong game result in Xiangqi archive.".into()),
        };
        let mut application_error = None;
        let mut phase = EnginePhase::Paused;
        // Validate the entire proposal before applying even its first action.
        // Rejected native output is retained as explicitly unscored evidence.
        let mut pv_error = None;
        if let Some(t) = telemetry
            && !t.principal_variation.is_empty()
        {
            pv_error = variations.check(&expected, &t.principal_variation)?;
        }
        match &response.result {
            SharedResult::Xiangqi {
                result: XiangqiResult::Choice { choice, .. },
            } if !read_only && pv_error.is_none() => {
                let (actor, command) = native_choice(*choice, self.game.position().side_to_move());
                if actor == self.human {
                    application_error =
                        Some("Engine choice is not an owned live engine turn.".into());
                    phase = EnginePhase::Failed;
                } else {
                    match self.game.submit(x::GameRequest {
                        expected: self.game.context(),
                        actor,
                        command,
                    }) {
                        Ok(submission) => {
                            if submission.result == x::CommandResult::RejectedNaturalCount {
                                application_error = Some("Native engine count claim was rejected; its audit is retained without a game result.".into());
                                phase = EnginePhase::Failed;
                            } else {
                                phase = EnginePhase::Idle;
                            }
                            self.undo.push(submission.undo);
                        }
                        Err(e) => {
                            application_error = Some(e.to_string());
                            phase = EnginePhase::Failed;
                        }
                    }
                }
            }
            SharedResult::Xiangqi {
                result: XiangqiResult::Incomplete { .. },
            } => phase = EnginePhase::Incomplete,
            SharedResult::Failure { .. } => phase = EnginePhase::Failed,
            SharedResult::Xiangqi { .. } => {}
            _ => unreachable!(),
        }
        if let Some(error) = pv_error {
            application_error = Some(format!("Native principal variation rejected: {error}"));
            phase = EnginePhase::Failed;
        }
        let observation = PresentedObservation {
            response,
            application_error,
            historical: false,
        };
        self.pending = None;
        self.submitted = false;
        self.analysis_intent = None;
        self.phase = phase;
        self.paused = read_only || phase != EnginePhase::Idle;
        self.diagnostic =
            observation
                .application_error
                .clone()
                .or_else(|| match &observation.response.result {
                    SharedResult::Failure { failure } => Some(failure.message.clone()),
                    SharedResult::Xiangqi {
                        result: XiangqiResult::Unresolved { pending, .. },
                    } => Some(format!("Unresolved, unscored: {}", pending.reason)),
                    SharedResult::Xiangqi {
                        result: XiangqiResult::Incomplete { .. },
                    } => Some("Incomplete native work; no move or result was invented.".into()),
                    _ => None,
                });
        if read_only && observation.application_error.is_none() {
            self.analysis = Some(observation.clone());
        } else {
            self.analysis = None;
        }
        self.observation_bytes += serde_json::to_vec(&observation.response)
            .map_err(error)?
            .len();
        self.observations.push(observation);
        self.durable_needs_stop = !self.paused;
        Ok(())
    }
}
fn native_choice(choice: XiangqiChoice, turn: x::Side) -> (x::Side, x::GameCommand) {
    match choice {
        XiangqiChoice::Move { action } => (turn, x::GameCommand::Play { action }),
        XiangqiChoice::Claim { actor } => (actor, x::GameCommand::ClaimNaturalCount),
    }
}

pub(crate) enum XiangqiWork {
    Admit {
        draft: Box<XiangqiDraft>,
        command: Option<SessionOperation>,
        response: Option<Box<SharedResponse>>,
        exact: Option<Box<XiangqiExactEvidence>>,
        replacement: bool,
        floor: u64,
    },
    Save {
        draft: Box<XiangqiDraft>,
        path: PathBuf,
        revision: u64,
        identity: u64,
        floor: u64,
    },
}
pub(crate) enum XiangqiCompleted {
    Admitted {
        state: Box<XiangqiSession>,
        analysis: Option<WorkLimits>,
        replacement: bool,
        changed: bool,
    },
}
pub(crate) fn execute(
    work: XiangqiWork,
    stopped: &mut impl FnMut() -> bool,
) -> Result<super::super::archive::NativeCompleted, String> {
    use super::super::archive::NativeCompleted;
    let result: Result<NativeCompleted, String> = match work {
        XiangqiWork::Admit {
            draft,
            command,
            response,
            exact,
            replacement,
            floor,
        } => {
            let mut state = draft.materialize()?;
            let before = (state.game.events().len(), state.board_actions());
            let changed = exact.is_none()
                && !matches!(command, Some(SessionOperation::XiangqiPreferences { .. }))
                && !response.as_ref().is_some_and(|r| {
                    matches!(r.context.purpose, SharedPurpose::MoveAnalysis { .. })
                });
            let mut analysis = None;
            if let Some(command) = command {
                analysis = state.apply_command(command.clone())?;
                state.journal(before, Some(command), None, None, false);
            }
            if let Some(response) = response {
                let submitted = state
                    .pending
                    .as_ref()
                    .ok_or("Native result has no submitted request.")?;
                if !response.matches(submitted) {
                    return Err("Native result differs from submitted context.".into());
                }
                let binding = ExecutionBinding::of(submitted);
                let index = state.observations.len();
                state.apply_response(*response)?;
                state.journal(before, None, Some(index), None, false);
                state.controls.last_mut().unwrap().execution = Some(binding);
            }
            if let Some(exact) = exact {
                exact.validate(&state, stopped)?;
                let index = state.exact_observations.len();
                state.exact_observations.push(*exact);
                state.journal(before, None, None, Some(index), false);
            }
            if floor >= MAX_SAFE_ID - 64 {
                return Err(capacity("Runtime identity headroom exhausted."));
            }
            state.execution_floor = state.execution_floor.max(floor);
            check_capacity(&state, stopped)?;
            Ok(NativeCompleted::Xiangqi(XiangqiCompleted::Admitted {
                state: Box::new(state),
                analysis,
                replacement,
                changed,
            }))
        }
        XiangqiWork::Save {
            draft,
            path,
            revision,
            identity,
            floor,
        } => {
            let state = draft.materialize()?;
            check_capacity(&state, stopped)?;
            let bytes = encode(&state, floor)?;
            persistence::atomic_write(&path, &bytes, stopped)?;
            Ok(NativeCompleted::Saved {
                path,
                revision,
                identity,
            })
        }
    };
    result.map_err(error)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawArchive {
    engine_protocol: u32,
    play_limits: WorkLimits,
    initial: Box<RawValue>,
    current: Box<RawValue>,
    human: NativeSide,
    initial_settings: XiangqiSettings,
    settings: XiangqiSettings,
    controls: Vec<Box<RawValue>>,
    observations: Vec<Box<RawValue>>,
    exact_observations: Vec<Box<RawValue>>,
    retired_execution_floor: u64,
}

/// One Open's already-authoritatively-validated original encodings. Equality is
/// byte equality, never diagram equality or a normalized JSON map. Repeated
/// observations often retain the same complete context hundreds of times.
#[derive(Default)]
struct ValidatedRecords(std::collections::HashMap<String, CanonicalRecord>);
impl ValidatedRecords {
    fn parse(&mut self, raw: &RawValue) -> Result<CanonicalRecord, String> {
        if let Some(record) = self.0.get(raw.get()) {
            return Ok(record.clone());
        }
        let record = CanonicalRecord::from_json(raw.get()).map_err(error)?;
        self.0.insert(raw.get().to_owned(), record.clone());
        Ok(record)
    }
}

/// Pure proposal validation, scoped to one Open (or a single live admission).
/// Transport acceptance and actual engine-turn application are never cached.
#[derive(Default)]
struct ValidatedVariations(std::collections::HashMap<(String, String), Option<String>>);
impl ValidatedVariations {
    fn check(
        &mut self,
        source: &CanonicalRecord,
        variation: &[XiangqiChoice],
    ) -> Result<Option<String>, String> {
        let key = (
            source.to_json().map_err(error)?,
            serde_json::to_string(variation).map_err(error)?,
        );
        if let Some(result) = self.0.get(&key) {
            return Ok(result.clone());
        }
        let (mut probe, _) = native(source)?.to_game_with_undo().map_err(error)?;
        let mut rejection = None;
        for choice in variation {
            let (actor, command) = native_choice(*choice, probe.position().side_to_move());
            if let Err(e) = probe.submit(x::GameRequest {
                expected: probe.context(),
                actor,
                command,
            }) {
                rejection = Some(e.to_string());
                break;
            }
        }
        self.0.insert(key, rejection.clone());
        Ok(rejection)
    }
}

// These concrete archive readers defer only canonical payload parsing. Every
// surrounding field stays strict and every original nested byte reaches the
// authoritative parser before it can enter the per-Open cache.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRequest {
    protocol: u32,
    purpose: SharedPurpose,
    request_id: u64,
    session_version: u64,
    process_generation: u64,
    game: Box<RawValue>,
    limits: WorkLimits,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawResponse {
    context: RawRequest,
    result: SharedResult,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawObservation {
    response: RawResponse,
    #[serde(deserialize_with = "super::super::archive::required_option")]
    application_error: Option<String>,
    historical: bool,
}
impl RawObservation {
    fn validated(self, records: &mut ValidatedRecords) -> Result<PresentedObservation, String> {
        let r = self.response.context;
        Ok(PresentedObservation {
            response: SharedResponse {
                context: SharedRequest {
                    protocol: r.protocol,
                    purpose: r.purpose,
                    request_id: r.request_id,
                    session_version: r.session_version,
                    process_generation: r.process_generation,
                    game: records.parse(&r.game)?,
                    limits: r.limits,
                },
                result: self.response.result,
            },
            application_error: self.application_error,
            historical: self.historical,
        })
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawExactEvidence {
    format: String,
    executor: String,
    source: Box<RawValue>,
    purpose: String,
    goal: String,
    event_count: usize,
    historical_target: bool,
    position_json: String,
    native_session: u64,
    native_generation: u64,
    limits: x::ExactLimits,
    report: x::ExactReport,
}
impl RawExactEvidence {
    fn validated(self, records: &mut ValidatedRecords) -> Result<XiangqiExactEvidence, String> {
        Ok(XiangqiExactEvidence {
            format: self.format,
            executor: self.executor,
            source: records.parse(&self.source)?,
            purpose: self.purpose,
            goal: self.goal,
            event_count: self.event_count,
            historical_target: self.historical_target,
            position_json: self.position_json,
            native_session: self.native_session,
            native_generation: self.native_generation,
            limits: self.limits,
            report: self.report,
        })
    }
}
fn raw_work(text: &str) -> Result<usize, String> {
    if text.len() > mshogi_core::native::MAX_RECORD_ENVELOPE_BYTES {
        return Err(capacity("Saved canonical wrapper exceeds1MiB."));
    }
    #[derive(Deserialize)]
    struct Envelope {
        game_kind: String,
        rules_profile: String,
        profile_edition: String,
        payload: Box<RawValue>,
    }
    #[derive(Deserialize)]
    struct Ledger {
        commands: Vec<Box<RawValue>>,
        events: Vec<Box<RawValue>>,
    }
    #[derive(Deserialize)]
    struct Command {
        command: Kind,
    }
    #[derive(Deserialize)]
    struct Event {
        event: Kind,
    }
    #[derive(Deserialize)]
    struct Kind {
        kind: String,
    }
    let e: Envelope = serde_json::from_str(text).map_err(error)?;
    if e.game_kind != "xiangqi"
        || e.rules_profile != x::PROFILE_ID
        || e.profile_edition != x::PROFILE_EDITION
    {
        return Err("Wrong game/profile/edition in Xiangqi archive native context.".into());
    }
    let ledger: Ledger = serde_json::from_str(e.payload.get()).map_err(error)?;
    if ledger.events.len() > x::MAX_GAME_EVENTS || ledger.commands.len() > x::MAX_GAME_COMMANDS {
        return Err(capacity("Native Xiangqi command/event bound exceeded."));
    }
    let mut command_plays = 0usize;
    for raw in &ledger.commands {
        let c: Command = serde_json::from_str(raw.get()).map_err(error)?;
        command_plays += usize::from(c.command.kind == "play");
    }
    let mut event_plays = 0usize;
    for raw in &ledger.events {
        let e: Event = serde_json::from_str(raw.get()).map_err(error)?;
        event_plays += usize::from(e.event.kind == "board_action");
    }
    // A forged omitted/shortened derived event vector must not undercharge native
    // command replay before the authoritative parser detects the contradiction.
    Ok(record_work(
        ledger.events.len().max(ledger.commands.len()),
        command_plays.max(event_plays),
    ))
}
pub(crate) fn decode(
    payload: &str,
    stopped: &mut impl FnMut() -> bool,
) -> Result<(XiangqiSession, u64), String> {
    let raw: RawArchive = serde_json::from_str(payload).map_err(error)?;
    if raw.play_limits != PLAY_LIMITS {
        return Err("Unsupported Xiangqi saved play limits; no downgrade was applied.".into());
    }
    if raw.engine_protocol != SHARED_PROTOCOL_VERSION {
        return Err("Unsupported Xiangqi archive execution transport version.".into());
    }
    if raw.controls.len() > MAX_ARCHIVE_CONTROLS
        || raw
            .observations
            .len()
            .saturating_add(raw.exact_observations.len())
            > MAX_ARCHIVE_OBSERVATIONS
    {
        return Err(capacity(
            "Saved control/observation count exceeds supported limits.",
        ));
    }
    // Resource probes never replace the original bytes supplied to strict parsers.
    let mut work = raw_work(raw.initial.get())?.saturating_add(raw_work(raw.current.get())?);
    #[derive(Deserialize)]
    struct ObsProbe {
        response: Box<RawValue>,
    }
    #[derive(Deserialize)]
    struct ResponseProbe {
        context: Box<RawValue>,
        result: SharedResult,
    }
    #[derive(Deserialize)]
    struct ContextProbe {
        game: Box<RawValue>,
    }
    let mut variations = std::collections::HashSet::new();
    for text in &raw.observations {
        persistence::check(stopped)?;
        let o: ObsProbe = serde_json::from_str(text.get()).map_err(error)?;
        let r: ResponseProbe = serde_json::from_str(o.response.get()).map_err(error)?;
        let c: ContextProbe = serde_json::from_str(r.context.get()).map_err(error)?;
        work = work.saturating_add(raw_work(c.game.get())? + 8);
        // Original bytes are a conservative preflight identity. Native typed
        // equality and original strict parsing still govern the actual replay.
        work = work.saturating_add(charge_variation(
            &mut variations,
            c.game.get().to_owned(),
            variation(&r.result),
        )?);
    }
    #[derive(Deserialize)]
    struct ExactProbe {
        source: Box<RawValue>,
        report: Box<RawValue>,
    }
    #[derive(Deserialize)]
    struct ReportProbe {
        verdict: Box<RawValue>,
    }
    #[derive(Deserialize)]
    struct VerdictProbe {
        kind: String,
        #[serde(default)]
        witness: Vec<Box<RawValue>>,
    }
    for text in &raw.exact_observations {
        persistence::check(stopped)?;
        let e: ExactProbe = serde_json::from_str(text.get()).map_err(error)?;
        let r: ReportProbe = serde_json::from_str(e.report.get()).map_err(error)?;
        let v: VerdictProbe = serde_json::from_str(r.verdict.get()).map_err(error)?;
        if v.witness.len() > 4096 {
            return Err(capacity("Saved exact witness exceeds4096 actions."));
        }
        work = work.saturating_add(raw_work(e.source.get())? + 8);
        if v.kind == "natural_win" {
            work = work.saturating_add(witness_work(v.witness.len()));
        }
    }
    let mut controls = Vec::with_capacity(raw.controls.len());
    for text in raw.controls {
        persistence::check(stopped)?;
        #[derive(Deserialize)]
        struct Probe {
            command: Option<Box<RawValue>>,
        }
        #[derive(Deserialize)]
        struct Kind {
            kind: String,
        }
        let probe: Probe = serde_json::from_str(text.get()).map_err(error)?;
        if let Some(command) = probe.command {
            let k: Kind = serde_json::from_str(command.get()).map_err(error)?;
            if !matches!(
                k.kind.as_str(),
                "xiangqi"
                    | "xiangqi_preferences"
                    | "resign"
                    | "undo"
                    | "take_back"
                    | "inspect"
                    | "return_live"
                    | "analyze"
                    | "cancel"
                    | "resume"
                    | "retry"
            ) {
                return Err("Unsupported Xiangqi journal command.".into());
            }
        }
        let control: XiangqiControl = serde_json::from_str(text.get()).map_err(error)?;
        if control.native_events_before > x::MAX_GAME_EVENTS
            || control.native_board_actions_before > x::MAX_GAME_COMMANDS
        {
            return Err(capacity("Journal native event count exceeds its bound."));
        }
        work = work.saturating_add(control_work(&control));
        controls.push(control);
    }
    if work > MAX_REPLAY_WORK {
        return Err(capacity(
            "Saved journal exceeds16,000 weighted replay work units.",
        ));
    }
    persistence::check(stopped)?;
    let mut records = ValidatedRecords::default();
    let initial = records.parse(&raw.initial)?;
    persistence::check(stopped)?;
    let current = records.parse(&raw.current)?;
    valid_limits(raw.initial_settings.analysis_defaults)?;
    valid_limits(raw.settings.analysis_defaults)?;
    if raw.retired_execution_floor >= MAX_SAFE_ID - 64 {
        return Err(capacity("Saved identity floor leaves no runtime headroom."));
    }
    let NativeSide::Xiangqi(human) = raw.human else {
        return Err("Wrong game for saved human side.".into());
    };
    let mut state = XiangqiDraft::replacement(native(&initial)?, human).materialize()?;
    state.settings = raw.initial_settings.clone();
    state.initial_settings = raw.initial_settings;
    let mut observations: Vec<PresentedObservation> = Vec::with_capacity(raw.observations.len());
    for text in raw.observations {
        persistence::check(stopped)?;
        let wire: RawObservation = serde_json::from_str(text.get()).map_err(error)?;
        observations.push(wire.validated(&mut records)?);
    }
    let mut exact = Vec::<XiangqiExactEvidence>::with_capacity(raw.exact_observations.len());
    for text in raw.exact_observations {
        persistence::check(stopped)?;
        let wire: RawExactEvidence = serde_json::from_str(text.get()).map_err(error)?;
        exact.push(wire.validated(&mut records)?);
    }
    let mut last_request = 0;
    let mut variations = ValidatedVariations::default();
    for (index, c) in controls.into_iter().enumerate() {
        persistence::check(stopped)?;
        if c.sequence != index + 1
            || c.native_events_before != state.game.events().len()
            || c.native_board_actions_before != state.board_actions()
            || usize::from(c.command.is_some())
                + usize::from(c.observation.is_some())
                + usize::from(c.exact_observation.is_some())
                + usize::from(c.close_pause)
                != 1
            || c.execution.is_some() != c.observation.is_some()
        {
            return Err("Corrupt Xiangqi journal order/type/native event boundary.".into());
        }
        if let Some(command) = &c.command {
            state.apply_command(command.clone())?;
        } else if let Some(index) = c.observation {
            if index != state.observations.len() {
                return Err("Corrupt observation journal order.".into());
            }
            let o = observations
                .get(index)
                .ok_or("Missing journal observation.")?;
            let r = &o.response.context;
            if c.execution.as_ref() != Some(&ExecutionBinding::of(r))
                || r.request_id <= last_request
                || [r.request_id, r.session_version, r.process_generation]
                    .iter()
                    .any(|id| *id > raw.retired_execution_floor)
            {
                return Err("Corrupt observation execution identity/purpose/limits.".into());
            }
            if let SharedPurpose::MoveAnalysis { target } = &r.purpose
                && target.selection_id > raw.retired_execution_floor
            {
                return Err("Corrupt historical selection identity.".into());
            }
            last_request = r.request_id;
            state.apply_response_reusing(o.response.clone(), &mut variations)?;
            let admitted = state.observations.last_mut().unwrap();
            if admitted.application_error != o.application_error {
                return Err("Corrupt observation application-admission evidence.".into());
            }
            admitted.historical = o.historical;
        } else if let Some(index) = c.exact_observation {
            if index != state.exact_observations.len() {
                return Err("Corrupt exact evidence journal order.".into());
            }
            let e = exact
                .get(index)
                .ok_or("Missing exact historical evidence.")?;
            e.validate(&state, stopped)?;
            state.exact_observations.push(e.clone());
        } else {
            state.retire();
            state.paused = true;
            state.durable_needs_stop = false;
        }
        state.controls.push(c);
    }
    if state.observations.len() != observations.len()
        || state.exact_observations.len() != exact.len()
        || CanonicalRecord::xiangqi(x::GameRecord::from_game(&state.game).map_err(error)?)
            != current
        || state.settings != raw.settings
    {
        return Err("Corrupt archive: authoritative Xiangqi journal replay differs from saved ledger/settings/evidence.".into());
    }
    state.execution_floor = raw.retired_execution_floor;
    check_capacity(&state, stopped)?;
    Ok((state, raw.retired_execution_floor))
}

impl DesktopController {
    /// Attach already completed native exact work as historical evidence only.
    /// This issues no request, advertises no execution capability and applies no result.
    pub fn retain_xiangqi_exact_evidence(
        &mut self,
        expected: SessionToken,
        observation: &x::ExactObservation,
    ) -> Result<AdmissionReceipt, String> {
        if expected != self.shared_token() || self.admission_token.is_some() {
            return Err("Stale historical evidence attachment.".into());
        }
        let state = self
            .shared
            .xiangqi
            .as_ref()
            .ok_or("No native Xiangqi document.")?;
        self.queue_xiangqi_candidate(
            XiangqiDraft::capture(state),
            None,
            None,
            Some(XiangqiExactEvidence::from_native(observation)),
            false,
        )
    }
    pub(crate) fn queue_xiangqi_candidate(
        &mut self,
        draft: XiangqiDraft,
        command: Option<SessionOperation>,
        response: Option<SharedResponse>,
        exact: Option<XiangqiExactEvidence>,
        replacement: bool,
    ) -> Result<AdmissionReceipt, String> {
        self.session.ensure_capacity()?;
        if self.session.capacity.id >= MAX_SAFE_ID - 64 {
            return Err(capacity("Admission identity headroom exhausted."));
        }
        let preferences =
            exact.is_some() || matches!(command, Some(SessionOperation::XiangqiPreferences { .. }));
        let rollback = self.shared.xiangqi.as_ref().map(|s| {
            (
                s.paused,
                s.phase,
                s.diagnostic.clone(),
                s.analysis.clone(),
                s.pending.is_some() || s.analysis_intent.is_some(),
            )
        });
        self.retire_admission();
        self.shared.admission_consumes_response = response.is_some();
        self.shared.rollback = rollback;
        if replacement {
            self.retire_native_files()?;
        }
        if matches!(command, Some(SessionOperation::TakeBack {})) {
            // Takeback owns execution immediately; the accepted native game is
            // retained until candidate admission, but its abandoned work cannot
            // race the reversal or become current again after a refusal.
            self.bridge.cancel();
            if let Some(state) = &mut self.shared.xiangqi {
                state.retire();
                state.paused = true;
            }
        }
        // Other operations keep their running request until admission.
        // Keep the accepted game AND its running request untouched until admission.
        // The controller holds result acceptance while the candidate is checked.
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
            persistence::Work::Native(Box::new(super::super::archive::NativeWork::Xiangqi(
                XiangqiWork::Admit {
                    draft: Box::new(draft),
                    command,
                    response: response.map(Box::new),
                    exact: exact.map(Box::new),
                    replacement,
                    floor,
                },
            ))),
        );
        Ok(receipt)
    }
    pub(crate) fn install_xiangqi(
        &mut self,
        mut state: Box<XiangqiSession>,
        floor: u64,
        path: PathBuf,
    ) -> Result<String, String> {
        let next = self
            .runtime_floor()
            .max(floor)
            .checked_add(1)
            .ok_or("Runtime identity exhausted.")?;
        if next >= MAX_SAFE_ID - 64
            || self.shared.epoch >= MAX_SAFE_ID - 64
            || self.session.document_identity >= MAX_SAFE_ID - 65
            || self.session.document_revision >= MAX_SAFE_ID - 65
        {
            return Err(capacity("Reopened runtime identities leave no headroom."));
        }
        self.retire_admission();
        self.last_command_receipt = None;
        self.bridge.cancel();
        state.retire();
        state.paused = true;
        for o in &mut state.observations {
            o.historical = true;
        }
        state.diagnostic = Some("Document reopened with work stopped. Saved ordinary/exact observations are historical; Resume explicitly.".into());
        self.shared.shogi = None;
        self.shared.xiangqi = Some(*state);
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
        Ok(
            "Opened native Xiangqi archivev2 with fresh runtime identities and work stopped."
                .into(),
        )
    }
    pub(crate) fn finish_xiangqi_admission(
        &mut self,
        completed: XiangqiCompleted,
        id: u64,
        receipt: Option<AdmissionReceipt>,
    ) {
        let XiangqiCompleted::Admitted {
            state,
            analysis,
            replacement,
            changed,
        } = completed;
        if state.pending.is_none() || replacement {
            self.bridge.cancel();
        }
        if replacement {
            self.session.pending = None;
            self.session.retire_analysis();
            self.session.play_paused = true;
        }
        self.shared.shogi = None;
        self.shared.xiangqi = Some(*state);
        if changed {
            self.shared_changed();
        } else {
            self.session.document_revision += 1;
            self.session.revision += 1;
            self.session.archive_revision = self.session.revision.min(MAX_SAFE_ID - 65);
        }
        if replacement {
            self.shared.epoch += 1;
            self.session.document_identity += 1;
            self.session.document_path = None;
            self.session.saved_revision = 0;
        }
        let floor = self.runtime_floor();
        self.shared.xiangqi.as_mut().unwrap().execution_floor = self
            .shared
            .xiangqi
            .as_ref()
            .unwrap()
            .execution_floor
            .max(floor);
        self.session.capacity = CapacityState {
            id,
            ..CapacityState::default()
        };
        if let Some(limits) = analysis
            && let Err(e) = self.start_xiangqi(limits, true)
        {
            self.pause_native_capacity(e);
        }
        if let Some(r) = receipt {
            r.finish(Ok(()));
        }
    }
}

#[cfg(test)]
#[path = "shared_xiangqi_archive_tests.rs"]
mod tests;
