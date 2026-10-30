//! Versioned desktop archives and one bounded off-path file/replay worker.
use super::*;
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};
/// Maximum desktop archive bytes; exceeding a resource cap never truncates data.
pub const MAX_ARCHIVE_BYTES: usize = 16 * 1024 * 1024;
/// Maximum combined initial-history and match events per document.
pub const MAX_ARCHIVE_EVENTS: usize = 4096;
/// Maximum retained accepted observations in a saved document.
pub const MAX_ARCHIVE_OBSERVATIONS: usize = 256;
/// Maximum accepted desktop controls in a saved document.
pub const MAX_ARCHIVE_CONTROLS: usize = 8192;
/// Native preferences persist independently of game/search identity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesktopSettings {
    /// View orientation, independent of player color.
    pub orientation: Color,
    /// Initial analysis form choices; actual requests retain their own limits.
    pub analysis_defaults: AnalysisOperation,
}
impl Default for DesktopSettings {
    fn default() -> Self {
        Self {
            orientation: Color::White,
            analysis_defaults: AnalysisOperation::Moves {
                depth: 4,
                nodes: 100_000,
            },
        }
    }
}
/// An accepted final observation; restored observations are historical only.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArchivedObservation {
    /// Ordered document observation index.
    pub sequence: u64,
    /// Native target description.
    pub label: String,
    /// Exact immutable request and actual final result.
    pub observation: ExecutionObservation,
    /// True after restore; never resumes old work.
    pub historical: bool,
    /// Root perspective supplied natively, never inferred by the frontend.
    pub analyzed_turn: Color,
    /// Invalid returned choice/operation retained as failed evidence, never applied.
    pub application_error: Option<String>,
}
/// Document revision/dirty state and native preferences.
#[derive(Clone, Debug, Serialize)]
pub struct DocumentState {
    /// Changes only when document contents change, not on I/O progress.
    pub revision: u64,
    /// Current contents differ from the last successfully saved/opened snapshot.
    pub dirty: bool,
    /// Display path; actual filesystem paths remain native.
    pub path: Option<String>,
    /// Persisted native preferences.
    pub settings: DesktopSettings,
}
/// A stable open precondition, unaffected by I/O progress snapshots.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenToken {
    /// Expected live command version.
    pub version: u64,
    /// Expected document contents revision.
    pub document_revision: u64,
}
/// File/inspection operation phase, separate from chess outcomes.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IoPhase {
    /// No operation yet.
    #[default]
    Idle,
    /// Atomic file save in progress.
    Saving,
    /// File validation/replay in progress.
    Opening,
    /// Historical prefix reconstruction in progress.
    Viewing,
    /// Current operation completed.
    Complete,
    /// Current operation failed; live game preserved.
    Failed,
}
/// Bounded worker operation identity and diagnostic.
#[derive(Clone, Debug, Default, Serialize)]
pub struct IoState {
    /// Monotonic operation identity.
    pub id: u64,
    /// Current phase.
    pub phase: IoPhase,
    /// Plain completion/failure detail.
    pub message: Option<String>,
}
/// One native event boundary, including nonmove commands.
#[derive(Clone, Debug, Serialize)]
pub struct HistoryPoint {
    /// Exact prefix event count.
    pub index: usize,
    /// Native event label.
    pub label: String,
    /// Played plies at this boundary.
    pub ply: u64,
}
/// Read-only viewed position, explicitly separate from the live game.
#[derive(Clone, Debug, Serialize)]
pub struct ViewState {
    /// True when an earlier event boundary is selected.
    pub historical: bool,
    /// Selected exact event boundary.
    pub event_index: usize,
    /// Total event boundaries in this match.
    pub event_count: usize,
    /// Native position/event label.
    pub label: String,
    /// Viewed canonical position.
    pub fen: String,
    /// Native viewed pieces.
    pub pieces: Vec<BoardPiece>,
    /// Viewed side to move.
    pub turn: Color,
    /// Outcome established at this prefix, excluding future events.
    pub conclusion: Option<MatchConclusion>,
    /// Viewed played ply count.
    pub ply: u64,
    /// Runtime selection identity.
    pub selection_id: u64,
    /// Native event timeline.
    pub timeline: Vec<HistoryPoint>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ControlEntry {
    sequence: u64,
    at_ns: u64,
    version: u64,
    command: Option<DesktopCommand>,
    close_pause: bool,
}
/// Complete abandoned authoritative lineage, tied to its desktop transaction.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct TakeBackEvidence {
    pub(super) control_sequence: u64,
    pub(super) match_record_json: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ChessDocument {
    match_record_json: String,
    game_record_json: String,
    human: Color,
    settings: DesktopSettings,
    play_limits: WorkLimits,
    observations: Vec<ArchivedObservation>,
    controls: Vec<ControlEntry>,
    takebacks: Vec<TakeBackEvidence>,
    version: u64,
    revision: u64,
    next_request: u64,
    document_revision: u64,
    #[serde(deserialize_with = "super::shared::archive::required_option")]
    selected_event: Option<usize>,
}
pub(super) struct Opened {
    archive: ChessDocument,
    game: ChessMatch,
    viewed: Option<Viewed>,
}
#[derive(Clone)]
pub(super) struct Viewed {
    game: AdjudicatedGame,
    conclusion: Option<MatchConclusion>,
    index: usize,
}
pub(super) enum Work {
    Native(Box<super::shared::archive::NativeWork>),
    Admit(Box<DesktopSession>),
    TakeBack {
        candidate: Box<DesktopSession>,
        at_ns: u64,
    },
    Save {
        archive: Box<ChessDocument>,
        record: MatchRecord,
        game_record: Box<GameRecord>,
        path: PathBuf,
        revision: u64,
        document_identity: u64,
    },
    Open {
        path: PathBuf,
        token: OpenToken,
    },
    View {
        record: MatchRecord,
        index: usize,
        token: OpenToken,
    },
}
pub(super) enum Completed {
    Native(Box<super::shared::archive::NativeCompleted>),
    Admitted(Box<DesktopSession>),
    Saved {
        path: PathBuf,
        revision: u64,
        document_identity: u64,
    },
    Opened {
        opened: Box<Opened>,
        path: PathBuf,
        token: OpenToken,
    },
    Viewed {
        view: Box<Viewed>,
        token: OpenToken,
    },
}
struct Job {
    id: u64,
    work: Work,
    stop: Arc<AtomicBool>,
}
#[derive(Default)]
struct WorkerState {
    pending: Option<Job>,
    result: Option<(u64, Result<Completed, String>)>,
    current: Option<Arc<AtomicBool>>,
    shutdown: bool,
}
pub(super) struct WorkerShared {
    state: Mutex<WorkerState>,
    wake: Condvar,
    pub(super) exited: AtomicBool,
}
pub(super) struct FileWorker {
    pub(super) shared: Arc<WorkerShared>,
}
impl FileWorker {
    pub(super) fn new() -> io::Result<Self> {
        let shared = Arc::new(WorkerShared {
            state: Mutex::new(WorkerState::default()),
            wake: Condvar::new(),
            exited: AtomicBool::new(false),
        });
        let worker = shared.clone();
        thread::Builder::new()
            .name("mshogi-document".into())
            .spawn(move || {
                loop {
                    let job = {
                        let mut state = worker.state.lock().expect("document worker metadata");
                        while state.pending.is_none() && !state.shutdown {
                            state = worker.wake.wait(state).expect("document worker metadata");
                        }
                        if state.shutdown {
                            break;
                        }
                        state.pending.take().expect("pending document job")
                    };
                    let deadline = Instant::now() + std::time::Duration::from_secs(15);
                    let stopped = || job.stop.load(Ordering::Acquire) || Instant::now() >= deadline;
                    let result = execute(job.work, stopped).map_err(crate::document::diagnostic);
                    let mut state = worker.state.lock().expect("document worker metadata");
                    if !state.shutdown
                        && !job.stop.load(Ordering::Acquire)
                        && state
                            .current
                            .as_ref()
                            .is_some_and(|current| Arc::ptr_eq(current, &job.stop))
                    {
                        state.result = Some((job.id, result));
                    }
                }
                worker.exited.store(true, Ordering::Release);
            })?;
        Ok(Self { shared })
    }
    pub(super) fn submit_admission(&self, id: u64, candidate: DesktopSession) {
        self.submit(id, Work::Admit(Box::new(candidate)));
    }
    pub(super) fn poll_admission(&self) -> Option<(u64, Result<DesktopSession, String>)> {
        self.poll().map(|(id, result)| {
            (
                id,
                result.map(|completed| match completed {
                    Completed::Admitted(candidate) => *candidate,
                    _ => unreachable!("dedicated admission worker"),
                }),
            )
        })
    }
    pub(super) fn cancel(&self) {
        let mut state = self.shared.state.lock().expect("document worker metadata");
        if let Some(current) = &state.current {
            current.store(true, Ordering::Release);
        }
        state.pending = None;
        state.result = None;
    }
    pub(super) fn submit(&self, id: u64, work: Work) {
        let stop = Arc::new(AtomicBool::new(false));
        let mut state = self.shared.state.lock().expect("document worker metadata");
        if let Some(current) = &state.current {
            current.store(true, Ordering::Release);
        }
        let retired = (
            state.pending.replace(Job {
                id,
                work,
                stop: stop.clone(),
            }),
            state.result.take(),
        );
        state.current = Some(stop);
        drop(state);
        self.shared.wake.notify_one();
        drop(retired);
    }
    pub(super) fn poll(&self) -> Option<(u64, Result<Completed, String>)> {
        self.shared
            .state
            .lock()
            .expect("document worker metadata")
            .result
            .take()
    }
    pub(super) fn shutdown(&self) {
        let mut state = self.shared.state.lock().expect("document worker metadata");
        state.shutdown = true;
        if let Some(current) = &state.current {
            current.store(true, Ordering::Release);
        }
        let retired = (state.pending.take(), state.result.take());
        drop(state);
        self.shared.wake.notify_one();
        drop(retired);
    }
}
impl Drop for FileWorker {
    fn drop(&mut self) {
        self.shutdown();
    }
}
pub(super) fn check(stopped: &mut impl FnMut() -> bool) -> Result<(), String> {
    if stopped() {
        Err("Document operation cancelled or exceeded its 15-second validation budget.".into())
    } else {
        Ok(())
    }
}
fn execute(work: Work, mut stopped: impl FnMut() -> bool) -> Result<Completed, String> {
    match work {
        Work::Native(work) => super::shared::archive::execute(*work, &mut stopped)
            .map(|value| Completed::Native(Box::new(value))),
        Work::TakeBack {
            mut candidate,
            at_ns,
        } => {
            candidate.ensure_capacity()?;
            candidate.take_back_with_stop(&mut stopped)?;
            candidate.journal(
                DesktopCommand::TakeBack {
                    version: candidate.version - 1,
                },
                at_ns,
            );
            candidate.check_archive_capacity(&mut stopped)?;
            Ok(Completed::Admitted(candidate))
        }
        Work::Admit(candidate) => {
            candidate.check_archive_capacity(&mut stopped)?;
            Ok(Completed::Admitted(candidate))
        }
        Work::Save {
            mut archive,
            record,
            game_record,
            path,
            revision,
            document_identity,
        } => {
            archive.match_record_json = record.to_json();
            archive.game_record_json = game_record.to_json();
            preflight(&archive)?;
            check(&mut stopped)?;
            let bytes = encode_chess(&archive)?;
            if bytes.len() > MAX_ARCHIVE_BYTES {
                return Err(
                    "Document exceeds the 16 MiB archive byte limit; no data was truncated.".into(),
                );
            }
            atomic_write(&path, &bytes, &mut stopped)?;
            Ok(Completed::Saved {
                path,
                revision,
                document_identity,
            })
        }
        Work::Open { path, token } => {
            if !fs::metadata(&path)
                .map_err(|e| format!("Open metadata failed: {e}"))?
                .is_file()
            {
                return Err("Only regular desktop document files can be opened.".into());
            }
            let mut file = fs::File::open(&path).map_err(|e| format!("Open failed: {e}"))?;
            if !file.metadata().map_err(|e| e.to_string())?.is_file() {
                return Err("Only regular desktop document files can be opened.".into());
            }
            let mut bytes = Vec::new();
            let mut chunk = [0; 8192];
            loop {
                check(&mut stopped)?;
                let n = file.read(&mut chunk).map_err(|e| e.to_string())?;
                if n == 0 {
                    break;
                }
                if bytes.len() + n > MAX_ARCHIVE_BYTES {
                    return Err("Document exceeds the 16 MiB archive byte limit.".into());
                }
                bytes.extend_from_slice(&chunk[..n]);
            }
            let opened = super::shared::archive::decode_chess_document(&bytes, &mut stopped)?;
            Ok(Completed::Opened {
                opened: Box::new(opened),
                path,
                token,
            })
        }
        Work::View {
            record,
            index,
            token,
        } => {
            let view = view_prefix(&record, index, &mut stopped)?;
            Ok(Completed::Viewed {
                view: Box::new(view),
                token,
            })
        }
    }
}
fn encode_chess(payload: &ChessDocument) -> Result<Vec<u8>, String> {
    crate::document::encode(mshogi_core::native::RulesProfile::ChessMatchV1, payload)
}
pub(super) fn decode_chess_payload(
    bytes: &[u8],
    stopped: &mut impl FnMut() -> bool,
) -> Result<Opened, String> {
    let archive: ChessDocument =
        serde_json::from_slice(bytes).map_err(|e| format!("Invalid desktop archive: {e}"))?;
    let record = preflight(&archive)?;
    let mut resave = archive.clone();
    resave.version = MAX_SAFE_ID - 17;
    resave.revision = MAX_SAFE_ID - 17;
    resave.next_request = MAX_SAFE_ID - 17;
    resave.document_revision = MAX_SAFE_ID - 17;
    if encode_chess(&resave)?.len() > MAX_ARCHIVE_BYTES {
        return Err("Application capacity: reopened runtime identities would exceed the complete 16 MiB archive budget. The current document was preserved.".into());
    }
    check(stopped)?;
    let game = record
        .replay_with_stop(&mut *stopped)
        .map_err(|e| e.to_string())?;
    let final_record =
        GameRecord::from_json(&archive.game_record_json).map_err(|e| e.to_string())?;
    if game.game().record() != final_record {
        return Err("Desktop final GameRecord disagrees with complete MatchRecord replay.".into());
    }
    let lineage = validate_takebacks(&archive, &record, stopped)?;
    for observation in &archive.observations {
        check(stopped)?;
        validate_observation_lineage(observation, &final_record, &lineage, stopped)?;
    }
    let viewed = archive
        .selected_event
        .map(|index| view_prefix(&record, index, stopped))
        .transpose()?;
    Ok(Opened {
        archive,
        game,
        viewed,
    })
}
fn preflight(archive: &ChessDocument) -> Result<MatchRecord, String> {
    preflight_reserved(archive, 0)
}
fn preflight_reserved(
    archive: &ChessDocument,
    reserve_controls: usize,
) -> Result<MatchRecord, String> {
    archive.settings.analysis_defaults.validate()?;
    if archive.play_limits != PLAY_LIMITS {
        return Err("Unsupported live play limits; no settings were downgraded.".into());
    }
    if [
        archive.version,
        archive.revision,
        archive.next_request,
        archive.document_revision,
    ]
    .iter()
    .any(|value| *value >= MAX_SAFE_ID - 16)
    {
        return Err("Saved runtime identity watermark exceeds the supported range.".into());
    }
    if archive.observations.len() > MAX_ARCHIVE_OBSERVATIONS
        || archive.controls.len() > MAX_ARCHIVE_CONTROLS
    {
        return Err(
            "Archive observation/control resource limit exceeded; no history was truncated.".into(),
        );
    }
    let record = MatchRecord::from_json(&archive.match_record_json).map_err(|e| e.to_string())?;
    let initial = record.initial_record().map_err(|e| e.to_string())?;
    let final_record =
        GameRecord::from_json(&archive.game_record_json).map_err(|e| e.to_string())?;
    if record.time_control() != TimeControl::Untimed {
        return Err(
            "Timed live-session restoration is unsupported; clocks were not changed.".into(),
        );
    }
    if initial.adjudication_mode() != AdjudicationMode::M2
        || final_record.adjudication_mode() != AdjudicationMode::M2
    {
        return Err("Full live-session restoration is unsupported; mode and limits were not downgraded or replayed.".into());
    }
    if initial.events().len() + record.event_count() > MAX_ARCHIVE_EVENTS
        || final_record.events().len() > MAX_ARCHIVE_EVENTS
    {
        return Err("Archive exceeds 4096 game/match events.".into());
    }
    let current_history_events = initial.events().len() + record.event_count();
    if archive
        .selected_event
        .is_some_and(|index| index > current_history_events)
    {
        return Err("Selected history event is outside the native match timeline.".into());
    }
    let mut retained_events = current_history_events;
    // Reserve reconstruction of any selectable live-history prefix even when
    // currently viewing live. Selecting history must not make an admitted
    // retained-branch document exceed its existing replay budget on Save/Open.
    let mut replay_work = retained_events
        .saturating_mul(4)
        .saturating_add(current_history_events.saturating_mul(4))
        .saturating_add(archive.controls.len())
        .saturating_add(reserve_controls);
    for entry in &archive.takebacks {
        let prior = MatchRecord::from_json(&entry.match_record_json).map_err(|e| e.to_string())?;
        let prior_initial = prior.initial_record().map_err(|e| e.to_string())?;
        if prior.time_control() != TimeControl::Untimed || prior_initial != initial {
            return Err("Unsupported or conflicting takeback root/mode/time control.".into());
        }
        if prior.events().any(|(request, _)| {
            matches!(
                request.command,
                MatchCommand::Resign { witness: Some(_) }
                    | MatchCommand::Observe { witness: Some(_) }
            )
        }) {
            return Err("Desktop takeback archives do not support witnessed match commands; evidence was not replayed or discarded.".into());
        }
        let events = prior
            .event_count()
            .saturating_add(prior_initial.events().len());
        retained_events = retained_events.saturating_add(events);
        // Open validates the complete pre-takeback match, its target and the
        // restored prefix. Charge all three native passes conservatively.
        replay_work = replay_work.saturating_add(events.saturating_mul(12));
    }
    for observation in &archive.observations {
        let continuation = match chess_observation_result(&observation.observation).as_ref() {
            Some(ChessResult::Move { telemetry, .. })
            | Some(ChessResult::Claim { telemetry, .. })
            | Some(ChessResult::Finished { telemetry, .. })
            | Some(ChessResult::Incomplete { telemetry }) => telemetry.principal_variation.len(),
            Some(ChessResult::Reachability {
                result: MateReachability::Possible { witness, .. },
            }) => witness.len(),
            _ => 0,
        };
        replay_work = replay_work.saturating_add(
            observation
                .observation
                .context
                .game
                .event_count()
                .saturating_add(continuation)
                .saturating_mul(4)
                .saturating_add(8),
        );
    }
    if retained_events > MAX_ARCHIVE_EVENTS
        || (!archive.takebacks.is_empty() && replay_work > 16_000)
    {
        return Err("Application capacity: complete retained takeback lineage exceeds the 4096-event/16000 replay-work budget.".into());
    }
    let mut last_control_at = 0;
    for (i, control) in archive.controls.iter().enumerate() {
        if control.at_ns < last_control_at {
            return Err("Desktop control journal timestamps moved backwards.".into());
        }
        last_control_at = control.at_ns;
        if let Some(command) = &control.command {
            match command {
                DesktopCommand::Analyze { operation, .. } => operation.validate()?,
                DesktopCommand::Preferences {
                    analysis_defaults, ..
                } => analysis_defaults.validate()?,
                DesktopCommand::NewGame {
                    document_revision, ..
                } if *document_revision > archive.document_revision => {
                    return Err(
                        "New-game journal precondition exceeds saved document revision.".into(),
                    );
                }
                DesktopCommand::Inspect { .. } | DesktopCommand::ReturnLive { .. } => {
                    return Err("Read-only selection is not a mutating document control.".into());
                }
                _ => {}
            }
        }
        if control.sequence != (i + 1) as u64
            || control.version > archive.version
            || control.close_pause == control.command.is_some()
        {
            return Err("Invalid desktop control journal ordering/identity.".into());
        }
    }
    for (request, _) in record.events() {
        if matches!(
            &request.command,
            MatchCommand::Resign { witness: Some(_) } | MatchCommand::Observe { witness: Some(_) }
        ) {
            return Err("Current desktop restoration does not support witnessed match commands; supplied certificates were not replayed or discarded.".into());
        }
    }
    for (i, observation) in archive.observations.iter().enumerate() {
        if let mshogi_engine::ExecutionPurpose::MoveAnalysis { target }
        | mshogi_engine::ExecutionPurpose::ChessFullReachability { target, .. } =
            &observation.observation.context.purpose
            && (target.selection_id == 0
                || target.selection_id > archive.next_request
                || target.selection_id >= MAX_SAFE_ID)
        {
            return Err("Observation selection identity exceeds its saved watermark.".into());
        }
        let context = &observation.observation.context;
        if observation.sequence != (i + 1) as u64
            || context.request_id > archive.next_request
            || context.process_generation > archive.next_request
            || context.session_version > archive.version
            || observation.label.is_empty()
            || observation.label.len() > 256
            || context.request_id == 0
            || context.process_generation == 0
            || !context.valid_limits()
            || !observation.observation.matches(context)
            || chess_record(&context.game)?.adjudication_mode() != AdjudicationMode::M2
            || context.game.event_count() > MAX_ARCHIVE_EVENTS
        {
            return Err("Invalid observation ordering, identity or live-mode context.".into());
        }
    }
    Ok(record)
}
fn validate_takebacks(
    archive: &ChessDocument,
    current: &MatchRecord,
    stopped: &mut impl FnMut() -> bool,
) -> Result<Vec<GameRecord>, String> {
    let controls: Vec<_> = archive
        .controls
        .iter()
        .filter(|c| matches!(c.command, Some(DesktopCommand::TakeBack { .. })))
        .collect();
    if controls.len() != archive.takebacks.len() {
        return Err("Takeback evidence/control count disagrees.".into());
    }
    let mut lineage = Vec::new();
    let mut previous: Option<MatchRecord> = None;
    for (control, evidence) in controls.into_iter().zip(&archive.takebacks) {
        check(stopped)?;
        if control.sequence != evidence.control_sequence {
            return Err("Takeback evidence is bound to another control.".into());
        }
        let before =
            MatchRecord::from_json(&evidence.match_record_json).map_err(|e| e.to_string())?;
        if before.initial_record().map_err(|e| e.to_string())?
            != current.initial_record().map_err(|e| e.to_string())?
        {
            return Err("Takeback lineage changed native root/provenance.".into());
        }
        if let Some(prior) = &previous
            && before.prefix(prior.event_count()).map(|p| p.to_json()) != Some(prior.to_json())
        {
            return Err(
                "Takeback lineage is not an extension of its previous restored decision.".into(),
            );
        }
        if before.timestamp_floor() > control.at_ns {
            return Err("Takeback control precedes its authoritative history.".into());
        }
        let admitted = before
            .replay_with_stop(&mut *stopped)
            .map_err(|e| e.to_string())?;
        lineage.push(admitted.game().record());
        let restored = super::takeback::restore(&before, archive.human, stopped)?;
        previous = Some(restored.record().clone());
    }
    if let Some(prior) = &previous
        && current.prefix(prior.event_count()).map(|p| p.to_json()) != Some(prior.to_json())
    {
        return Err("Current match disagrees with the last restored native decision.".into());
    }
    Ok(lineage)
}
fn validate_observation_lineage(
    observation: &ArchivedObservation,
    final_record: &GameRecord,
    lineage: &[GameRecord],
    stopped: &mut impl FnMut() -> bool,
) -> Result<(), String> {
    let source = chess_record(&observation.observation.context.game)?;
    let record = std::iter::once(final_record)
        .chain(lineage)
        .find(|record| record.prefix(source.events().len()).as_ref() == Some(source))
        .ok_or(
            "Observation target history is not an admitted current or abandoned native lineage.",
        )?;
    validate_observation(observation, record, stopped)
}
fn validate_observation(
    observation: &ArchivedObservation,
    final_record: &GameRecord,
    stopped: &mut impl FnMut() -> bool,
) -> Result<(), String> {
    let context = &observation.observation.context;
    let replay =
        AdjudicatedGame::replay_record_with_stop(chess_record(&context.game)?, &mut *stopped)
            .map_err(|e| e.to_string())?;
    if final_record.prefix(context.game.event_count()).as_ref()
        != Some(chess_record(&context.game)?)
    {
        return Err("Observation target history is not a prefix of this match.".into());
    }
    if let mshogi_engine::ExecutionPurpose::MoveAnalysis { target }
    | mshogi_engine::ExecutionPurpose::ChessFullReachability { target, .. } = &context.purpose
        && (target.turn != NativeSide::Chess(replay.position().side_to_move())
            || target.board_ply != replay.history().ply_count() as u64
            || target.session_version != context.session_version)
    {
        return Err("Observation target provenance disagrees with its game context.".into());
    }
    if observation.analyzed_turn != replay.position().side_to_move() {
        return Err("Observation score perspective disagrees with its exact target.".into());
    }
    validate_result(observation, &replay, stopped)?;
    Ok(())
}
fn validate_result(
    observation: &ArchivedObservation,
    game: &AdjudicatedGame,
    stopped: &mut impl FnMut() -> bool,
) -> Result<(), String> {
    use mshogi_engine::{EngineChoice, ExecutionPurpose, StopReason};
    let context = &observation.observation.context;
    let native_result = chess_observation_result(&observation.observation)
        .ok_or("Wrong native observation result family.")?;
    let result = &native_result;
    let full = matches!(
        context.purpose,
        ExecutionPurpose::ChessFullReachability { .. }
    );
    // Rejected native choices remain explicit historical failure evidence.
    if let Some(message) = &observation.application_error {
        if message.is_empty() || message.len() > mshogi_engine::MAX_STDERR_BYTES {
            return Err("Invalid observation application diagnostic.".into());
        }
        let invalid = match result {
            ChessResult::Move { action, .. } if !full => game.clone().apply(*action).is_err(),
            ChessResult::Claim { claim, .. } if !full => {
                game.clone().claim((*claim).into()).is_err()
            }
            ChessResult::Finished { outcome, .. } if !full => game.outcome() != Some(*outcome),
            ChessResult::Reachability { .. } => !full,
            _ => full,
        };
        return if invalid {
            Ok(())
        } else {
            Err("Observation marks a valid choice as an application failure.".into())
        };
    }
    if let ChessResult::Failure { failure } = result {
        return if failure.message.len() <= mshogi_engine::MAX_STDERR_BYTES {
            Ok(())
        } else {
            Err("Failure diagnostic exceeds its byte limit.".into())
        };
    }
    if let ChessResult::Reachability { result } = result {
        let ExecutionPurpose::ChessFullReachability { limits, .. } = context.purpose else {
            return Err("Reachability observations require the Full purpose.".into());
        };
        let work = match result {
            MateReachability::Possible { witness, work } => {
                if work.positions == 0
                    || work.expanded == 0
                    || witness.len() as u64 >= work.positions
                    || witness.len() as u64 > work.transitions
                    || witness.len() as u64 > limits.max_positions
                {
                    return Err("Reachability witness exceeds recorded work evidence.".into());
                }
                let ExecutionPurpose::ChessFullReachability { mate, .. } = context.purpose else {
                    unreachable!()
                };
                let targets: &[Color] = match mate {
                    mshogi_engine::ReachabilityTarget::Either => &[Color::White, Color::Black],
                    mshogi_engine::ReachabilityTarget::White => &[Color::White],
                    mshogi_engine::ReachabilityTarget::Black => &[Color::Black],
                };
                let mut valid = false;
                for side in targets {
                    check(stopped)?;
                    match mating_feasibility_with_stop(game.position(), *side, Some(witness), &mut *stopped) {
                        Ok(MatingFeasibility::Possible) => { valid = true; break; },
                        Err("mating witness verification interrupted") => return Err("Stored witness verification interrupted by the operation budget/cancellation.".into()),
                        _ => {},
                    }
                }
                check(stopped)?;
                if !valid {
                    return Err(
                        "Stored cooperative witness does not establish the requested legal mate."
                            .into(),
                    );
                }
                work
            }
            MateReachability::Impossible { work } | MateReachability::Unknown { work, .. } => work,
        };
        if work.positions > limits.max_positions
            || work.transitions > limits.max_transitions
            || work.expanded > work.positions
            || work.static_leaves > work.expanded
        {
            return Err("Reachability observations exceed their recorded work limits.".into());
        }
        return Ok(());
    }
    if full {
        return Err("Full purpose has minimax observations.".into());
    }
    let telemetry = match result {
        ChessResult::Move { telemetry, .. }
        | ChessResult::Claim { telemetry, .. }
        | ChessResult::Finished { telemetry, .. }
        | ChessResult::Incomplete { telemetry } => telemetry,
        _ => unreachable!(),
    };
    if telemetry.completed_depth > context.limits.depth
        || telemetry.nodes > context.limits.nodes
        || telemetry.principal_variation.len() > usize::from(telemetry.completed_depth)
        || telemetry.score.is_some() != (telemetry.completed_depth > 0)
        || telemetry
            .score
            .is_some_and(|score| score.unsigned_abs() > 1_000_000)
    {
        return Err("Minimax observations disagree with recorded limits/iteration shape.".into());
    }
    match result {
        ChessResult::Incomplete { .. }
            if telemetry.completed_depth != 0 || telemetry.status == StopReason::Complete =>
        {
            return Err("Incomplete result contains a completed iteration.".into());
        }
        ChessResult::Move { action, .. }
            if telemetry.principal_variation.first()
                != Some(&EngineChoice::Move { action: *action }) =>
        {
            return Err("Move decision disagrees with its continuation.".into());
        }
        ChessResult::Claim { claim, .. }
            if telemetry.principal_variation.first()
                != Some(&EngineChoice::Claim { claim: *claim }) =>
        {
            return Err("Claim decision disagrees with its continuation.".into());
        }
        ChessResult::Finished { outcome, .. }
            if game.outcome() != Some(*outcome) || !telemetry.principal_variation.is_empty() =>
        {
            return Err("Finished observation disagrees with native root outcome.".into());
        }
        _ => {}
    }
    let mut continuation = game.clone();
    for choice in &telemetry.principal_variation {
        match choice {
            EngineChoice::Move { action } => {
                continuation.apply(*action).map_err(|e| e.to_string())?
            }
            EngineChoice::Claim { claim } => continuation
                .claim((*claim).into())
                .map_err(|e| e.to_string())?,
        }
    }
    Ok(())
}
pub(super) fn atomic_write(
    path: &std::path::Path,
    bytes: &[u8],
    stopped: &mut impl FnMut() -> bool,
) -> Result<(), String> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| std::path::Path::new("."));
    let mut created = None;
    for attempt in 0..32 {
        let name = format!(
            ".mshogi-save-{}-{}-{attempt}.tmp",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        );
        let temporary = parent.join(name);
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
        {
            Ok(file) => {
                created = Some((temporary, file));
                break;
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(format!("Save temporary file failed: {error}")),
        }
    }
    let (temporary, mut file) =
        created.ok_or("Cannot create an exclusive same-directory save file.")?;
    let result = (|| {
        for chunk in bytes.chunks(8192) {
            check(stopped)?;
            file.write_all(chunk)
                .map_err(|e| format!("Save write failed: {e}"))?;
        }
        file.flush().map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;
        check(stopped)?;
        drop(file);
        fs::rename(&temporary, path).map_err(|e| format!("Atomic save replacement failed: {e}"))
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}
fn view_prefix(
    record: &MatchRecord,
    index: usize,
    stopped: &mut impl FnMut() -> bool,
) -> Result<Viewed, String> {
    let initial = record.initial_record().map_err(|e| e.to_string())?;
    if initial.adjudication_mode() != AdjudicationMode::M2 {
        return Err("Unsupported Full historical reconstruction.".into());
    }
    if index <= initial.events().len() {
        let prefix = initial
            .prefix(index)
            .ok_or("History boundary is out of range.")?;
        let game = AdjudicatedGame::replay_record_with_stop(&prefix, stopped)
            .map_err(|e| e.to_string())?;
        let conclusion = game.outcome().map(MatchConclusion::Chess);
        return Ok(Viewed {
            game,
            conclusion,
            index,
        });
    }
    let prefix = record
        .prefix(index - initial.events().len())
        .ok_or("History boundary is out of range.")?;
    let game = prefix
        .replay_with_stop(stopped)
        .map_err(|e| e.to_string())?;
    Ok(Viewed {
        game: game.game().clone(),
        conclusion: game.conclusion(),
        index,
    })
}

fn counts_fit(
    events: usize,
    game_events: usize,
    observations: usize,
    controls: usize,
    reserve_controls: usize,
) -> bool {
    events <= MAX_ARCHIVE_EVENTS
        && game_events <= MAX_ARCHIVE_EVENTS
        && observations <= MAX_ARCHIVE_OBSERVATIONS
        && controls.saturating_add(reserve_controls) <= MAX_ARCHIVE_CONTROLS
}
fn bytes_fit(bytes: usize, reserve: usize) -> bool {
    bytes.saturating_add(reserve) <= MAX_ARCHIVE_BYTES
}
impl DesktopSession {
    pub(super) fn check_archive_capacity(
        &self,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        let reserve_stop = self.pending.is_some()
            || self.analysis_pending.is_some()
            || (!self.play_paused
                && self.game.conclusion().is_none()
                && self.game.game().position().side_to_move() != self.human);
        let reserve_controls = usize::from(reserve_stop);
        if !counts_fit(
            self.initial_record.events().len() + self.game.record().event_count(),
            self.game.game().record().events().len(),
            self.observations.len(),
            self.controls.len(),
            reserve_controls,
        ) {
            return Err("Application capacity: this operation would exceed 4096 events, 256 observations or 8192 controls (including a stop/close reservation). No document history or observations were accepted or discarded. Save the complete current document, then start a new game/document to continue.".into());
        }
        check(stopped)?;
        let mut archive = self.archive_snapshot();
        archive.match_record_json = self.game.record().to_json();
        archive.game_record_json = self.game.game().record().to_json();
        // Immutable accepted lineage was already admitted natively. Do not
        // reconstruct every abandoned branch on each ordinary action; Open
        // performs complete authoritative lineage replay before installation.
        preflight_reserved(&archive, reserve_controls)?;
        // Budget each archive watermark at its maximum supported decimal width.
        // Reopening and runtime-only selection/work/I/O cannot grow this encoding.
        archive.version = MAX_SAFE_ID - 17;
        archive.revision = MAX_SAFE_ID - 17;
        archive.next_request = MAX_SAFE_ID - 17;
        archive.document_revision = MAX_SAFE_ID - 17;
        let bytes = encode_chess(&archive)?;
        let reserve_bytes = if reserve_stop { 1024 } else { 0 };
        if !bytes_fit(bytes.len(), reserve_bytes) {
            return Err("Application capacity: this operation would exceed the 16 MiB complete archive budget, including runtime identity and stop/close reservations. No new history or observations were accepted. Save the complete current document, then start a new game/document to continue.".into());
        }
        check(stopped)?;
        // Validate new raw evidence before accepting it; malformed or oversized
        // engine failure/PV/witness payloads must not make the document unsaveable.
        let final_record = self.game.game().record();
        for observation in self.observations.iter().skip(self.validated_observations) {
            validate_observation(observation, &final_record, stopped)?;
        }
        Ok(())
    }
    fn archive_snapshot(&self) -> ChessDocument {
        ChessDocument {
            match_record_json: String::new(),
            game_record_json: String::new(),
            human: self.human,
            settings: self.settings.clone(),
            play_limits: self.play_limits,
            observations: self.observations.clone(),
            controls: self.controls.clone(),
            takebacks: self.takebacks.clone(),
            version: self.version,
            revision: self.archive_revision,
            next_request: self.next_request.max(self.selection_id),
            document_revision: self.document_revision,
            selected_event: self.viewed.as_ref().map(|v| v.index),
        }
    }
    pub(super) fn document_state(&self) -> DocumentState {
        DocumentState {
            revision: self.document_revision,
            dirty: self.document_revision != self.saved_revision,
            path: self
                .document_path
                .as_ref()
                .map(|path| path.to_string_lossy().into_owned()),
            settings: self.settings.clone(),
        }
    }
    pub(super) fn remember(&mut self, response: ExecutionObservation) {
        let label = match &response.context.purpose {
            mshogi_engine::ExecutionPurpose::EngineTurn {} => format!(
                "Engine turn after {} plies",
                response.context.game.known_board_actions()
            ),
            mshogi_engine::ExecutionPurpose::MoveAnalysis { target }
            | mshogi_engine::ExecutionPurpose::ChessFullReachability { target, .. } => {
                target.label.clone()
            }
        };
        self.observations.push(ArchivedObservation {
            sequence: self.observations.len() as u64 + 1,
            label,
            analyzed_turn: match &response.context.purpose {
                mshogi_engine::ExecutionPurpose::EngineTurn {} => {
                    self.game.game().position().side_to_move()
                }
                mshogi_engine::ExecutionPurpose::MoveAnalysis { target }
                | mshogi_engine::ExecutionPurpose::ChessFullReachability { target, .. } => {
                    match target.turn {
                        NativeSide::Chess(turn) => turn,
                        _ => unreachable!("Chess observation checked before retention"),
                    }
                }
            },
            observation: response,
            historical: false,
            application_error: None,
        });
        self.document_revision += 1;
    }
    pub(super) fn journal(&mut self, command: DesktopCommand, at_ns: u64) {
        self.controls.push(ControlEntry {
            sequence: self.controls.len() as u64 + 1,
            at_ns,
            version: self.version,
            command: Some(command),
            close_pause: false,
        });
    }
    pub(super) fn view_state(&self, live_pieces: &[BoardPiece]) -> ViewState {
        let initial_count = self.initial_record.events().len();
        let mut timeline = vec![HistoryPoint {
            index: 0,
            label: "Initial known position".into(),
            ply: 0,
        }];
        let mut ply = 0;
        for (i, event) in self.initial_record.events().iter().enumerate() {
            if matches!(event.kind, RecordEventKind::Move(_)) {
                ply += 1;
            }
            timeline.push(HistoryPoint {
                index: i + 1,
                label: match &event.kind {
                    RecordEventKind::Move(action) => action.to_string(),
                    _ => "Initial history claim/observation".into(),
                },
                ply,
            });
        }
        for (i, (request, reply)) in self.game.record().events().enumerate() {
            if matches!(request.command, MatchCommand::Move(_)) && reply.rejection.is_none() {
                ply += 1;
            }
            let label = match &request.command {
                MatchCommand::Move(action) => action.to_string(),
                MatchCommand::Start => "Match started".into(),
                MatchCommand::Claim { intended, .. } => format!(
                    "Draw claim{}",
                    intended
                        .map(|a| format!(" with intended {a}"))
                        .unwrap_or_default()
                ),
                MatchCommand::Resign { .. } => "Resignation".into(),
                _ => "Match command".into(),
            };
            timeline.push(HistoryPoint {
                index: initial_count + i + 1,
                label,
                ply,
            });
        }
        let count = initial_count + self.game.record().event_count();
        let (historical, index, game, conclusion, pieces) = if let Some(view) = &self.viewed {
            (
                true,
                view.index,
                &view.game,
                view.conclusion,
                board_pieces(view.game.position()),
            )
        } else {
            (
                false,
                count,
                self.game.game(),
                self.game.conclusion(),
                live_pieces.to_vec(),
            )
        };
        ViewState {
            historical,
            event_index: index,
            event_count: count,
            label: if historical {
                format!("Read-only history event {index} of {count}")
            } else {
                "Live game".into()
            },
            fen: game.position().to_fen(),
            pieces,
            turn: game.position().side_to_move(),
            conclusion,
            ply: game.history().ply_count() as u64,
            selection_id: self.selection_id,
            timeline,
        }
    }
    pub(super) fn selected_game(&self) -> &AdjudicatedGame {
        self.viewed
            .as_ref()
            .map_or(self.game.game(), |view| &view.game)
    }
}
impl DesktopController {
    /// Capture a current replacement precondition without doing filesystem work.
    pub fn open_token(&self) -> OpenToken {
        OpenToken {
            version: self.session.version,
            document_revision: self.session.document_revision,
        }
    }
    fn next_io(&mut self, phase: IoPhase) -> Result<u64, String> {
        if self.io_state.id >= MAX_SAFE_ID - 1 || self.session.revision >= MAX_SAFE_ID - 2 {
            return Err("Document operation identity exhausted.".into());
        }
        self.io_state = IoState {
            id: self.io_state.id + 1,
            phase,
            message: None,
        };
        self.session.revision += 1;
        Ok(self.io_state.id)
    }
    /// Queue a complete save snapshot; serialization and all file I/O run off-path.
    pub fn queue_save(&mut self, path: PathBuf) -> Result<u64, String> {
        self.require_chess()?;
        if self.session.controls.len() > MAX_ARCHIVE_CONTROLS
            || self.session.observations.len() > MAX_ARCHIVE_OBSERVATIONS
            || self.session.initial_record.events().len() + self.session.game.record().event_count()
                > MAX_ARCHIVE_EVENTS
        {
            return Err("Document resource limits exceeded (4096 events, 256 observations, 8192 controls); no history was truncated or file changed.".into());
        }
        let id = self.next_io(IoPhase::Saving)?;
        let archive = ChessDocument {
            match_record_json: String::new(),
            game_record_json: String::new(),
            human: self.session.human,
            settings: self.session.settings.clone(),
            play_limits: self.session.play_limits,
            observations: self.session.observations.clone(),
            controls: self.session.controls.clone(),
            takebacks: self.session.takebacks.clone(),
            version: self.session.version,
            revision: self.session.archive_revision,
            next_request: self.session.next_request.max(self.session.selection_id),
            document_revision: self.session.document_revision,
            selected_event: self.session.viewed.as_ref().map(|v| v.index),
        };
        self.files.submit(
            id,
            Work::Save {
                archive: Box::new(archive),
                record: self.session.game.record().clone(),
                game_record: Box::new(self.session.game.game().record()),
                path,
                revision: self.session.document_revision,
                document_identity: self.session.document_identity,
            },
        );
        Ok(id)
    }
    /// Queue strict bounded validation. A stale token cannot replace any state.
    pub fn queue_open(&mut self, path: PathBuf, token: OpenToken) -> Result<u64, String> {
        self.require_chess()?;
        if token != self.open_token() {
            return Err(
                "The document changed before Open. Confirm the current document again.".into(),
            );
        }
        let id = self.next_io(IoPhase::Opening)?;
        self.files.submit(id, Work::Open { path, token });
        Ok(id)
    }
    pub(super) fn queue_view(&mut self, version: u64, index: usize) -> Result<Snapshot, String> {
        if version != self.session.version {
            return Err("The game changed before history selection.".into());
        }
        let count =
            self.session.initial_record.events().len() + self.session.game.record().event_count();
        if index > count || count > MAX_ARCHIVE_EVENTS {
            return Err("Unsupported history event boundary/resource limit.".into());
        }
        self.session.ensure_capacity()?;
        self.retire_admission();
        self.bridge.cancel();
        self.session.pending = None;
        self.session.retire_analysis();
        self.session.play_paused = true;
        self.session.engine.status = EnginePhase::Paused;
        self.session.version += 1;
        self.session.revision += 1;
        self.session.selection_id += 1;
        let token = self.open_token();
        let id = self.next_io(IoPhase::Viewing)?;
        self.files.submit(
            id,
            Work::View {
                record: self.session.game.record().clone(),
                index,
                token,
            },
        );
        Ok(self.snapshot())
    }
    pub(super) fn poll_files(&mut self) {
        let Some((id, result)) = self.files.poll() else {
            return;
        };
        if id != self.io_state.id {
            return;
        }
        let result = match result {
            Ok(Completed::Native(value)) => self.finish_native_file(*value),
            Ok(Completed::Admitted(_)) => unreachable!("dedicated file worker"),
            Ok(Completed::Saved {
                path,
                revision,
                document_identity,
            }) => {
                if self.session.document_identity == document_identity {
                    self.session.document_path = Some(path);
                    if self.session.document_revision == revision {
                        self.session.saved_revision = revision;
                    }
                }
                Ok("Saved a complete desktop document.".into())
            }
            Ok(Completed::Opened {
                opened,
                path,
                token,
            }) => {
                if token != self.open_token() {
                    Err("The live document changed during validation. Open was not applied.".into())
                } else {
                    self.install_chess_archive(*opened, path)
                }
            }
            Ok(Completed::Viewed { view, token }) => {
                if token != self.open_token() {
                    Err(
                        "The game changed during historical reconstruction; view was not applied."
                            .into(),
                    )
                } else {
                    self.session.viewed = Some(*view);
                    Ok("Reconstructed an exact read-only history prefix.".into())
                }
            }
            Err(error) => Err(error),
        };
        self.io_state.phase = if result.is_ok() {
            IoPhase::Complete
        } else {
            IoPhase::Failed
        };
        self.io_state.message = Some(result.unwrap_or_else(|error| error));
        self.session.revision += 1;
    }
    pub(super) fn install_chess_archive(
        &mut self,
        opened: Opened,
        path: PathBuf,
    ) -> Result<String, String> {
        let saved = opened.archive;
        let next = [
            self.session.version,
            self.session.revision,
            self.session.next_request,
            self.session.selection_id,
            saved.version,
            saved.revision,
            saved.next_request,
        ]
        .into_iter()
        .max()
        .unwrap_or(0)
            + 1;
        if next >= MAX_SAFE_ID - 64
            || self.shared.epoch >= MAX_SAFE_ID - 64
            || self.session.document_revision.max(saved.document_revision) >= MAX_SAFE_ID - 65
        {
            self.io_state.phase = IoPhase::Failed;
            self.io_state.message = Some("Application capacity: reopened identities leave no safe save/runtime headroom. Current document preserved.".into());
            self.session.revision += 1;
            return Err("Application capacity: reopened identity headroom exhausted.".into());
        }
        self.retire_admission();
        self.shared.xiangqi = None;
        self.shared.shogi = None;
        self.shared.epoch += 1;
        self.session.capacity.status = CapacityPhase::Available;
        self.session.capacity.message = None;
        self.bridge.cancel();
        self.clock_floor = opened.game.record().timestamp_floor().max(
            saved
                .controls
                .iter()
                .map(|control| control.at_ns)
                .max()
                .unwrap_or(0),
        );
        self.origin = Instant::now();
        self.session.game = opened.game;
        self.session.initial_record = self
            .session
            .game
            .record()
            .initial_record()
            .expect("worker validated initial record");
        self.session.human = saved.human;
        self.session.settings = saved.settings;
        self.session.play_limits = saved.play_limits;
        self.session.observations = saved.observations;
        for observation in &mut self.session.observations {
            observation.historical = true;
        }
        self.session.controls = saved.controls;
        self.session.takebacks = saved.takebacks;
        self.session.validated_observations = self.session.observations.len();
        self.session.document_revision =
            self.session.document_revision.max(saved.document_revision) + 1;
        self.session.saved_revision = self.session.document_revision;
        self.session.document_path = Some(path);
        self.session.document_identity = next;
        self.session.version = next;
        self.session.revision = next;
        self.session.archive_revision = next;
        self.session.next_request = next;
        self.session.selection_id = next;
        self.session.pending = None;
        self.session.retire_analysis();
        self.session.viewed = opened.viewed;
        self.session.play_paused = true;
        self.session.engine = EngineState { status: EnginePhase::Paused, message: Some("Document reopened with work stopped. Saved observations are historical. Resume play explicitly.".into()), ..EngineState::default() };
        Ok("Opened the complete document with work stopped.".into())
    }
    /// Stabilize a close decision by retiring work without inventing results.
    /// Repeated preflight is idempotent; an untouched initial game stays clean.
    pub fn pause_for_close(&mut self) -> Snapshot {
        self.pause_shared_for_close();
        self.retire_admission();
        if self.session.pending.is_some() || self.session.analysis_pending.is_some() {
            self.bridge.cancel();
            self.session.pending = None;
            self.session.retire_analysis();
            self.session.play_paused = true;
            self.session.engine.status = EnginePhase::Paused;
            self.session.changed();
            self.session.controls.push(ControlEntry {
                sequence: self.session.controls.len() as u64 + 1,
                at_ns: self.at_ns(),
                version: self.session.version,
                command: None,
                close_pause: true,
            });
        }
        self.snapshot()
    }
}
fn board_pieces(position: &Position) -> Vec<BoardPiece> {
    let mut pieces = Vec::new();
    for rank in 0..8 {
        for file in 0..8 {
            let square = Square::new(file, rank).expect("bounded square");
            if let Some(piece) = position.piece_at(square) {
                let kind = format!("{:?}", piece.kind).to_lowercase();
                pieces.push(BoardPiece {
                    square: square.to_string(),
                    color: piece.color,
                    label: format!("{square}, {:?} {kind}", piece.color),
                    kind,
                });
            }
        }
    }
    pieces
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU64;
    static NEXT_TEMP: AtomicU64 = AtomicU64::new(1);
    fn directory() -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "mshogi-doc-{}-{}",
            std::process::id(),
            NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).unwrap();
        path
    }
    fn controller() -> DesktopController {
        DesktopController::new(PathBuf::from("/missing/test-engine")).unwrap()
    }
    fn wait(controller: &mut DesktopController, id: u64) -> Snapshot {
        let deadline = Instant::now() + std::time::Duration::from_secs(20);
        loop {
            let snapshot = controller.snapshot();
            if snapshot.io.id == id
                && matches!(snapshot.io.phase, IoPhase::Complete | IoPhase::Failed)
            {
                return snapshot;
            }
            assert!(Instant::now() < deadline);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    }
    fn settled(controller: &mut DesktopController, command: DesktopCommand) -> Snapshot {
        let (mut snapshot, receipt) = controller.command_with_receipt(command).unwrap();
        let deadline = Instant::now() + std::time::Duration::from_secs(5);
        loop {
            if let Some(result) = receipt.completion() {
                result.unwrap();
                return snapshot;
            }
            assert!(Instant::now() < deadline);
            thread::sleep(std::time::Duration::from_millis(1));
            snapshot = controller.snapshot();
        }
    }
    fn archive(session: &DesktopSession) -> ChessDocument {
        ChessDocument {
            match_record_json: session.game.record().to_json(),
            game_record_json: session.game.game().record().to_json(),
            human: session.human,
            settings: session.settings.clone(),
            play_limits: session.play_limits,
            observations: session.observations.clone(),
            controls: session.controls.clone(),
            takebacks: session.takebacks.clone(),
            version: session.version,
            revision: session.archive_revision,
            next_request: session.next_request.max(session.selection_id),
            document_revision: session.document_revision,
            selected_event: session.viewed.as_ref().map(|v| v.index),
        }
    }
    #[test]
    fn shared_poll_that_commits_shogi_open_publishes_the_matching_game_family() {
        use crate::{SessionOperation, SharedDesktopCommand};
        use mshogi_core::{
            native::{CanonicalRecord, GameKind, NativeSide},
            shogi,
        };

        let directory = directory();
        let path = directory.join("native-shogi.mshogi");
        let mut source = controller();
        source.shared.manual_engine = true;
        let (_, receipt) = source
            .shared_command_with_receipt(SharedDesktopCommand {
                expected: source.shared_token(),
                game: GameKind::Chess,
                operation: SessionOperation::Replace {
                    record: CanonicalRecord::shogi(shogi::GameRecord::from_game(
                        &shogi::Game::new(),
                    )),
                    human: NativeSide::Shogi(shogi::Side::Sente),
                },
            })
            .unwrap();
        let deadline = Instant::now() + std::time::Duration::from_secs(20);
        while receipt.completion().is_none() {
            source.shared_snapshot();
            assert!(Instant::now() < deadline);
            thread::sleep(std::time::Duration::from_millis(1));
        }
        receipt.completion().unwrap().unwrap();
        let id = source.queue_shared_save(path.clone()).unwrap();
        assert_eq!(wait(&mut source, id).io.phase, IoPhase::Complete);

        for legacy_first in [false, true] {
            let mut opened = controller();
            // An eligible old Chess engine turn must not start after Open has
            // committed Shogi, even when the legacy view polls first.
            opened.session.human = Color::Black;
            let before = opened.shared_token();
            opened
                .queue_shared_open(path.clone(), opened.shared_open_token())
                .unwrap();
            // Wait for the real file worker WITHOUT polling the controller:
            // the chosen first poll must commit the completed cross-game Open.
            let deadline = Instant::now() + std::time::Duration::from_secs(20);
            while opened.files.shared.state.lock().unwrap().result.is_none() {
                assert!(Instant::now() < deadline);
                thread::sleep(std::time::Duration::from_millis(1));
            }
            assert!(opened.shared.shogi.is_none());
            if legacy_first {
                opened.snapshot();
            }
            let first = opened.shared_snapshot();
            assert_eq!(first.io.phase, IoPhase::Complete);
            assert!(first.token.document > before.document);
            assert_eq!(first.game, GameKind::Shogi);
            assert_eq!(first.board.files.len(), 9);
            assert_eq!(first.human.id, NativeSide::Shogi(shogi::Side::Sente));
            assert!(first.paused);
            assert!(opened.session.pending.is_none());
            let next = opened.shared_snapshot();
            assert_eq!(first.token, next.token);
            assert_eq!(first.game, next.game);
        }
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn claims_and_resignation_roundtrip_complete_match_not_just_moves() {
        for claim in [false, true] {
            let directory = directory();
            let path = directory.join("game.mshogi");
            let mut source = controller();
            if claim {
                let game = AdjudicatedGame::from_fen(
                    "7k/8/8/8/8/8/R7/K7 w - - 99 1",
                    ClaimPolicy::ExplicitOnly,
                )
                .unwrap();
                source.session = DesktopSession::from_record(&game.record(), Color::White).unwrap();
                let choice = source
                    .session
                    .snapshot()
                    .claims
                    .into_iter()
                    .find(|claim| claim.intended.is_some())
                    .unwrap();
                let version = source.session.version;
                settled(
                    &mut source,
                    DesktopCommand::Claim {
                        version,
                        claim: choice,
                    },
                );
            } else {
                let version = source.session.version;
                settled(&mut source, DesktopCommand::Resign { version });
            }
            let core_record = source.session.game.record().to_json();
            let conclusion = source.session.game.conclusion();
            assert!(conclusion.is_some());
            let saved_version = source.session.version;
            let id = source.queue_save(path.clone()).unwrap();
            assert_eq!(wait(&mut source, id).io.phase, IoPhase::Complete);
            let mut restored = controller();
            let token = restored.open_token();
            let id = restored.queue_open(path.clone(), token).unwrap();
            let snapshot = wait(&mut restored, id);
            assert_eq!(
                snapshot.io.phase,
                IoPhase::Complete,
                "{:?}",
                snapshot.io.message
            );
            assert_eq!(restored.session.game.record().to_json(), core_record);
            assert_eq!(snapshot.conclusion, conclusion);
            assert!(snapshot.history.is_empty());
            assert!(snapshot.play_paused);
            assert!(!snapshot.document.dirty);
            assert!(snapshot.version > saved_version);
            assert!(restored.at_ns() >= restored.session.game.record().timestamp_floor());
            // The preceding event has the same board/ply, but no future claim or resignation.
            let final_index = snapshot.view.event_count;
            let before = restored
                .command(DesktopCommand::Inspect {
                    version: snapshot.version,
                    event_index: final_index - 1,
                })
                .unwrap();
            let viewed = wait(&mut restored, before.io.id);
            assert!(viewed.view.historical);
            assert!(viewed.view.conclusion.is_none());
            assert_eq!(viewed.view.ply, 0);
            assert_eq!(restored.session.game.conclusion(), conclusion);
            // Same diagram/ply, distinct native terminal-event prefix: saving
            // an inspected prefix must not silently reopen the live conclusion.
            let save = restored.queue_save(path.clone()).unwrap();
            assert_eq!(wait(&mut restored, save).io.phase, IoPhase::Complete);
            let mut selected = controller();
            let open = selected
                .queue_open(path.clone(), selected.open_token())
                .unwrap();
            let selected_view = wait(&mut selected, open);
            assert_eq!(selected_view.io.phase, IoPhase::Complete);
            assert!(selected_view.view.historical && selected_view.play_paused);
            assert_eq!(selected_view.view.event_index, viewed.view.event_index);
            assert_eq!(selected_view.view.ply, viewed.view.ply);
            assert_eq!(selected_view.view.fen, viewed.view.fen);
            assert!(selected_view.view.conclusion.is_none());
            assert_eq!(selected.session.game.record().to_json(), core_record);
            assert_eq!(selected.session.game.conclusion(), conclusion);
            assert!(
                selected.session.pending.is_none() && selected.session.analysis_pending.is_none()
            );
            assert!(selected_view.version > viewed.version);
            let live = settled(
                &mut selected,
                DesktopCommand::ReturnLive {
                    version: selected_view.version,
                },
            );
            assert!(!live.view.historical && live.play_paused);
            assert_eq!(live.conclusion, conclusion);
            fs::remove_dir_all(directory).unwrap();
        }
    }
    #[test]
    fn interrupted_atomic_write_keeps_prior_file_and_removes_temporary() {
        let directory = directory();
        let path = directory.join("prior.mshogi");
        fs::write(&path, b"prior valid document").unwrap();
        let mut calls = 0;
        let result = atomic_write(&path, &vec![b'x'; 20_000], &mut || {
            calls += 1;
            calls > 1
        });
        assert!(result.is_err());
        assert_eq!(fs::read(&path).unwrap(), b"prior valid document");
        assert_eq!(fs::read_dir(&directory).unwrap().count(), 1);
        atomic_write(&path, b"complete replacement", &mut || false).unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"complete replacement");
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn stale_save_and_stale_open_do_not_mark_or_replace_current_document() {
        let directory = directory();
        let path = directory.join("game.mshogi");
        let mut controller = controller();
        let save = controller.queue_save(path.clone()).unwrap();
        controller
            .session
            .command(
                DesktopCommand::Preferences {
                    orientation: Color::Black,
                    analysis_defaults: AnalysisOperation::Moves {
                        depth: 2,
                        nodes: 20_000,
                    },
                },
                1,
            )
            .unwrap();
        let snapshot = wait(&mut controller, save);
        assert_eq!(snapshot.io.phase, IoPhase::Complete);
        assert!(snapshot.document.dirty);
        assert_eq!(
            snapshot.document.path,
            Some(path.to_string_lossy().into_owned())
        );
        let token = controller.open_token();
        let open = controller.queue_open(path, token).unwrap();
        controller
            .session
            .command(
                DesktopCommand::Preferences {
                    orientation: Color::White,
                    analysis_defaults: AnalysisOperation::Moves {
                        depth: 3,
                        nodes: 10_000,
                    },
                },
                2,
            )
            .unwrap();
        let before = controller.session.game.record().to_json();
        let version = controller.session.version;
        let snapshot = wait(&mut controller, open);
        assert_eq!(snapshot.io.phase, IoPhase::Failed);
        assert_eq!(snapshot.version, version);
        assert_eq!(controller.session.game.record().to_json(), before);
        assert!(snapshot.document.dirty);
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn preflight_rejects_timed_full_identity_and_resource_limits_before_replay() {
        let session = DesktopSession::new();
        let valid = archive(&session);
        assert!(preflight(&valid).is_ok());
        let mut timed = valid.clone();
        let mut value: serde_json::Value = serde_json::from_str(&timed.match_record_json).unwrap();
        value["time_control"] =
            serde_json::json!({"increment":{"initial_ns":[100,100],"increment_ns":1}});
        timed.match_record_json = value.to_string();
        assert!(preflight(&timed).unwrap_err().contains("Timed"));
        let mut full = valid.clone();
        let mut game: serde_json::Value = serde_json::from_str(&full.game_record_json).unwrap();
        game["schema_version"] = 2.into();
        game["adjudication_mode"] =
            serde_json::json!({"full":{"limits":{"max_positions":null,"max_transitions":null}}});
        full.game_record_json = game.to_string();
        let mut record: serde_json::Value = serde_json::from_str(&full.match_record_json).unwrap();
        record["schema_version"] = 2.into();
        record["initial_record_json"] = full.game_record_json.clone().into();
        full.match_record_json = record.to_string();
        assert!(preflight(&full).unwrap_err().contains("Full"));
        let mut identity = valid.clone();
        identity.next_request = MAX_SAFE_ID;
        assert!(preflight(&identity).unwrap_err().contains("identity"));
        let mut controls = valid.clone();
        let entry = ControlEntry {
            sequence: 1,
            at_ns: 0,
            version: 1,
            command: None,
            close_pause: true,
        };
        controls.controls = vec![entry; MAX_ARCHIVE_CONTROLS + 1];
        assert!(preflight(&controls).unwrap_err().contains("resource"));
        assert!(
            session
                .game
                .record()
                .replay_with_stop(|| true)
                .unwrap_err()
                .to_string()
                .contains("interrupted")
        );
        assert!(matches!(
            AdjudicatedGame::replay_record_with_stop(&session.game.game().record(), || true),
            Err(LiveReplayError::Stopped)
        ));
    }
    #[test]
    fn observation_corruption_and_future_control_timestamps_are_validated() {
        let mut session = DesktopSession::new();
        session
            .command(
                DesktopCommand::Analyze {
                    version: 1,
                    operation: AnalysisOperation::Moves {
                        depth: 1,
                        nodes: 1000,
                    },
                },
                100_000_000_000,
            )
            .unwrap();
        let request = session.analysis_request().unwrap().context;
        let mut input = serde_json::to_vec(&test_request(request.clone())).unwrap();
        input.push(b'\n');
        let mut output = Vec::new();
        mshogi_engine::serve_once(std::io::Cursor::new(input), &mut output).unwrap();
        assert!(session.accept_analysis(serde_json::from_slice(&output).unwrap()));
        let good = archive(&session);
        let final_record = session.game.game().record();
        assert!(validate_observation(&good.observations[0], &final_record, &mut || false).is_ok());
        let mut bad = good.clone();
        bad.observations[0].observation.context.limits.depth = 9;
        assert!(preflight(&bad).is_err());
        let mut bad = good.observations[0].clone();
        if let ExecutionResult::Chess {
            result: ChessResult::Move { telemetry, .. },
        } = &mut bad.observation.result
        {
            telemetry.score = Some(i32::MIN);
        }
        assert!(validate_observation(&bad, &final_record, &mut || false).is_err());
        let mut bad = good.observations[0].clone();
        let mut wire: serde_json::Value =
            serde_json::to_value(&bad.observation.context.game).unwrap();
        wire["payload"]["claim_policy"] = "auto_claim".into();
        bad.observation.context.game = CanonicalRecord::from_json(&wire.to_string()).unwrap();
        assert!(validate_observation(&bad, &final_record, &mut || false).is_err());
        let directory = directory();
        let path = directory.join("timestamps.mshogi");
        fs::write(&path, encode_chess(&good).unwrap()).unwrap();
        let mut restored = controller();
        let token = restored.open_token();
        let id = restored.queue_open(path, token).unwrap();
        assert_eq!(wait(&mut restored, id).io.phase, IoPhase::Complete);
        assert!(restored.at_ns() >= 100_000_000_000);
        let version = restored.session.version;
        settled(&mut restored, DesktopCommand::Resume { version });
        assert!(matches!(
            restored.session.controls.last().unwrap().command,
            Some(DesktopCommand::Resume { .. })
        ));
        assert!(restored.session.controls.last().unwrap().at_ns >= 100_000_000_000);
        assert!(preflight(&archive(&restored.session)).is_ok());
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn stored_full_witnesses_require_legal_requested_mate_and_cooperative_budget() {
        let live =
            AdjudicatedGame::from_fen("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", ClaimPolicy::ExplicitOnly)
                .unwrap();
        let mut session = DesktopSession::from_record(&live.record(), Color::White).unwrap();
        session
            .command(
                DesktopCommand::Analyze {
                    version: 1,
                    operation: AnalysisOperation::FullReachability {
                        target: mshogi_engine::ReachabilityTarget::White,
                        max_positions: 1000,
                        max_transitions: 10_000,
                    },
                },
                1,
            )
            .unwrap();
        let request = session.analysis_request().unwrap().context;
        let mut bytes = serde_json::to_vec(&test_request(request.clone())).unwrap();
        bytes.push(b'\n');
        let mut output = Vec::new();
        mshogi_engine::serve_once(std::io::Cursor::new(bytes), &mut output).unwrap();
        assert!(session.accept_analysis(serde_json::from_slice(&output).unwrap()));
        let good = session.observations[0].clone();
        assert!(matches!(
            good.observation.result,
            ExecutionResult::Chess {
                result: ChessResult::Reachability {
                    result: MateReachability::Possible { .. }
                }
            }
        ));
        assert!(validate_observation(&good, &live.record(), &mut || false).is_ok());
        for action in ["g6g5", "g6f6"] {
            let mut bad = good.clone();
            if let ExecutionResult::Chess {
                result:
                    ChessResult::Reachability {
                        result: MateReachability::Possible { witness, .. },
                    },
            } = &mut bad.observation.result
            {
                *witness = vec![action.parse().unwrap()];
            }
            assert!(
                validate_observation(&bad, &live.record(), &mut || false).is_err(),
                "corrupt witness {action}"
            );
        }
        let mut wrong_side = good.clone();
        if let mshogi_engine::ExecutionPurpose::ChessFullReachability { mate, .. } =
            &mut wrong_side.observation.context.purpose
        {
            *mate = mshogi_engine::ReachabilityTarget::Black;
        }
        assert!(validate_observation(&wrong_side, &live.record(), &mut || false).is_err());
        let mut either = good.clone();
        if let mshogi_engine::ExecutionPurpose::ChessFullReachability { mate, .. } =
            &mut either.observation.context.purpose
        {
            *mate = mshogi_engine::ReachabilityTarget::Either;
        }
        assert!(validate_observation(&either, &live.record(), &mut || false).is_ok());
        let mut polls = 0;
        let interrupted = validate_observation(&good, &live.record(), &mut || {
            polls += 1;
            polls >= 4
        });
        assert!(interrupted.unwrap_err().contains("interrupt"));
        let mut overflow = archive(&session);
        if let mshogi_engine::ExecutionPurpose::ChessFullReachability { target, .. } =
            &mut overflow.observations[0].observation.context.purpose
        {
            target.selection_id = u64::MAX;
        }
        assert!(preflight(&overflow).unwrap_err().contains("selection"));
        let mut certificate = archive(&DesktopSession::new());
        let mut record: serde_json::Value =
            serde_json::from_str(&certificate.match_record_json).unwrap();
        record["events"][0]["request"]["command"] =
            serde_json::json!({"resign":{"witness":["g1f3","g8f6","f3g1","f6g8"]}});
        certificate.match_record_json = record.to_string();
        assert!(preflight(&certificate).unwrap_err().contains("witnessed"));
    }
    #[test]
    fn historical_analysis_excludes_future_claim_and_survives_only_its_selection() {
        let game =
            AdjudicatedGame::from_fen("7k/8/8/8/8/8/R7/K7 w - - 99 1", ClaimPolicy::ExplicitOnly)
                .unwrap();
        let mut controller = controller();
        controller.session = DesktopSession::from_record(&game.record(), Color::White).unwrap();
        // Force a positive controller timestamp so mixing a hardcoded zero
        // with this real clock would fail deterministically.
        controller.clock_floor = 100_000_000_000;
        let claim = controller.session.snapshot().claims[0];
        let concluded = settled(&mut controller, DesktopCommand::Claim { version: 1, claim });
        assert!(concluded.conclusion.is_some());
        assert_eq!(controller.session.game.record().event_count(), 2);
        let before = controller.session.game.record().to_json();
        let inspected = controller
            .command(DesktopCommand::Inspect {
                version: controller.session.version,
                event_index: 0,
            })
            .unwrap();
        wait(&mut controller, inspected.io.id);
        controller
            .session
            .command(
                DesktopCommand::Analyze {
                    version: controller.session.version,
                    operation: AnalysisOperation::Moves {
                        depth: 1,
                        nodes: 1000,
                    },
                },
                controller.at_ns(),
            )
            .unwrap();
        let request = controller.session.analysis_request().unwrap().context;
        assert_eq!(request.game.event_count(), 0);
        assert_eq!(request.game, CanonicalRecord::chess(game.record()));
        let target = match &request.purpose {
            mshogi_engine::ExecutionPurpose::MoveAnalysis { target } => target,
            _ => panic!(),
        };
        assert_eq!(target.selection_id, controller.session.selection_id);
        controller
            .session
            .command(
                DesktopCommand::ReturnLive {
                    version: controller.session.version,
                },
                controller.at_ns(),
            )
            .unwrap();
        assert!(controller.session.analysis_pending.is_none());
        assert_eq!(controller.session.game.record().to_json(), before);
        assert!(controller.session.viewed.is_none());
    }
    fn analysis_observations(count: usize) -> DesktopSession {
        let mut session = DesktopSession::new();
        for i in 0..count {
            session
                .command(
                    DesktopCommand::Analyze {
                        version: session.version,
                        operation: AnalysisOperation::Moves { depth: 1, nodes: 0 },
                    },
                    i as u64,
                )
                .unwrap();
            let context = session.analysis_request().unwrap().context;
            assert!(session.accept_analysis(test_response(
                context,
                ChessResult::Incomplete {
                    telemetry: mshogi_engine::Telemetry {
                        score: None,
                        completed_depth: 0,
                        nodes: 0,
                        principal_variation: vec![],
                        status: mshogi_engine::StopReason::NodeLimit
                    },
                }
            )));
        }
        session
    }
    #[test]
    fn archive_capacity_exact_count_and_byte_boundaries_include_stop_reservations() {
        assert!(counts_fit(4096, 4096, 256, 8192, 0));
        assert!(!counts_fit(4097, 0, 0, 0, 0));
        assert!(!counts_fit(0, 4097, 0, 0, 0));
        assert!(!counts_fit(0, 0, 257, 0, 0));
        assert!(!counts_fit(0, 0, 0, 8193, 0));
        assert!(counts_fit(0, 0, 0, 8191, 1));
        assert!(!counts_fit(0, 0, 0, 8192, 1));
        assert!(bytes_fit(16 * 1024 * 1024, 0));
        assert!(!bytes_fit(16 * 1024 * 1024 + 1, 0));
        assert!(bytes_fit(16 * 1024 * 1024 - 1024, 1024));
        assert!(!bytes_fit(16 * 1024 * 1024 - 1023, 1024));
    }
    #[test]
    fn observation_256_is_accepted_257_rejected_and_save_open_preserves_every_observation() {
        let mut source = controller();
        source.session = analysis_observations(256);
        let match_before = source.session.game.record().to_json();
        let document_before = source.session.document_revision;
        let version = source.session.version;
        let error = source
            .command(DesktopCommand::Analyze {
                version,
                operation: AnalysisOperation::Moves { depth: 1, nodes: 0 },
            })
            .unwrap_err();
        assert!(error.starts_with("Application capacity:"));
        assert_eq!(source.session.capacity.status, CapacityPhase::Paused);
        assert_eq!(source.session.game.record().to_json(), match_before);
        assert_eq!(source.session.document_revision, document_before);
        assert_eq!(source.session.observations.len(), 256);
        let directory = directory();
        let path = directory.join("complete.mshogi");
        let id = source.queue_save(path.clone()).unwrap();
        assert_eq!(wait(&mut source, id).io.phase, IoPhase::Complete);
        let mut reopened = controller();
        let id = reopened.queue_open(path, reopened.open_token()).unwrap();
        let snapshot = wait(&mut reopened, id);
        assert_eq!(snapshot.io.phase, IoPhase::Complete);
        assert_eq!(snapshot.observations.len(), 256);
        assert!(snapshot.observations.iter().all(|entry| entry.historical));
        assert!(
            reopened
                .command(DesktopCommand::Analyze {
                    version: snapshot.version,
                    operation: AnalysisOperation::Moves { depth: 1, nodes: 0 }
                })
                .unwrap_err()
                .starts_with("Application capacity:")
        );
        assert_eq!(reopened.session.observations.len(), 256);
        let second = directory.join("reopened.mshogi");
        let id = reopened.queue_save(second).unwrap();
        assert_eq!(wait(&mut reopened, id).io.phase, IoPhase::Complete);
        assert_eq!(reopened.pause_for_close().observations.len(), 256);
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn control_boundary_rejects_unchanged_and_reserved_close_is_saveable() {
        let mut source = controller();
        source.session.human = Color::Black;
        for i in 0..8191 {
            source.session.journal(
                DesktopCommand::Preferences {
                    orientation: Color::White,
                    analysis_defaults: AnalysisOperation::Moves { depth: 1, nodes: 0 },
                },
                i,
            );
        }
        assert!(source.session.check_archive_capacity(&mut || false).is_ok());
        let request = source.session.engine_request().unwrap().context;
        let closed = source.pause_for_close();
        assert!(closed.play_paused);
        assert_eq!(source.session.controls.len(), 8192);
        assert!(source.session.pending.is_none());
        assert!(!source.session.accept(
            test_response(
                request,
                ChessResult::Failure {
                    failure: mshogi_engine::EngineFailure {
                        kind: mshogi_engine::FailureKind::Timeout,
                        message: "obsolete".into()
                    }
                }
            ),
            8192
        ));
        let before = archive(&source.session);
        let error = source
            .session
            .command(
                DesktopCommand::Preferences {
                    orientation: Color::Black,
                    analysis_defaults: AnalysisOperation::Moves { depth: 1, nodes: 0 },
                },
                8193,
            )
            .unwrap_err();
        assert!(error.starts_with("Application capacity:"));
        assert_eq!(source.session.controls.len(), 8192);
        assert_eq!(source.session.settings.orientation, Color::White);
        assert_eq!(source.session.document_revision, before.document_revision);
        assert!(source.session.check_archive_capacity(&mut || false).is_ok());
        let directory = directory();
        let id = source.queue_save(directory.join("closed.mshogi")).unwrap();
        assert_eq!(wait(&mut source, id).io.phase, IoPhase::Complete);
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn oversized_raw_engine_failure_is_rejected_before_observation_or_game_commit() {
        let mut session = DesktopSession::new();
        session.human = Color::Black;
        let context = session.engine_request().unwrap().context;
        let before = session.game.record().to_json();
        let revision = session.document_revision;
        let capacity = session.capacity.clone();
        let pending = session.pending.clone();
        assert!(!session.accept(
            test_response(
                context.clone(),
                ChessResult::Failure {
                    failure: mshogi_engine::EngineFailure {
                        kind: mshogi_engine::FailureKind::Timeout,
                        message: "x".repeat(MAX_ARCHIVE_BYTES)
                    }
                }
            ),
            1
        ));
        // Invalid wire diagnostics are rejected before resource admission.
        assert_eq!(
            serde_json::to_value(&session.capacity).unwrap(),
            serde_json::to_value(&capacity).unwrap()
        );
        assert_eq!(session.pending, pending);
        assert_eq!(session.game.record().to_json(), before);
        assert_eq!(session.document_revision, revision);
        assert!(session.observations.is_empty());
        assert!(session.check_archive_capacity(&mut || false).is_ok());
        // The malformed response did not retire the legitimate outstanding work.
        assert!(session.accept(
            test_response(
                context,
                ChessResult::Failure {
                    failure: mshogi_engine::EngineFailure {
                        kind: mshogi_engine::FailureKind::Timeout,
                        message: "bounded watchdog diagnostic".into()
                    }
                }
            ),
            2
        ));
        assert_eq!(session.observations.len(), 1);
        assert_eq!(session.engine.status, EnginePhase::Failed);
        assert_eq!(session.game.record().to_json(), before);
    }
    #[test]
    fn command_receipts_survive_following_work_and_close_supersedes_pending_admission() {
        let mut source = controller();
        let (_, receipt) = source
            .command_with_receipt(DesktopCommand::Play {
                version: 1,
                action: "e2e4".parse().unwrap(),
            })
            .unwrap();
        let deadline = Instant::now() + std::time::Duration::from_secs(5);
        while receipt.completion().is_none() {
            source.snapshot();
            assert!(Instant::now() < deadline);
            thread::sleep(std::time::Duration::from_millis(1));
        }
        assert_eq!(receipt.completion(), Some(Ok(())));
        assert_eq!(source.session.snapshot().history, ["e2e4"]);
        source.pause_for_close();
        assert_eq!(receipt.completion(), Some(Ok(())));
        let mut candidate = source.session.clone();
        candidate
            .command_unchecked(
                DesktopCommand::Analyze {
                    version: source.session.version,
                    operation: AnalysisOperation::Moves { depth: 1, nodes: 0 },
                },
                source.at_ns(),
            )
            .unwrap();
        // command_with_receipt() polls before returning and can commit a fast
        // candidate. Queue directly so only validation runs before close.
        source.queue_admission(candidate).unwrap();
        let pending = source.admission_receipt.clone().unwrap();
        let deadline = Instant::now() + std::time::Duration::from_secs(5);
        while source
            .admissions
            .shared
            .state
            .lock()
            .unwrap()
            .result
            .is_none()
        {
            assert!(Instant::now() < deadline);
            thread::sleep(std::time::Duration::from_millis(1));
        }
        assert_eq!(pending.completion(), None);
        let document_revision = source.session.document_revision;
        source.pause_for_close();
        assert!(pending.completion().unwrap().is_err());
        assert_eq!(source.session.document_revision, document_revision);
        source.snapshot();
        assert!(source.session.analysis_pending.is_none());
        assert!(source.session.check_archive_capacity(&mut || false).is_ok());
    }

    #[test]
    fn save_during_admission_keeps_new_contents_dirty_and_successful_open_retires_candidate() {
        let directory = directory();
        let path = directory.join("initial.mshogi");
        let mut source = controller();
        let mut candidate = source.session.clone();
        candidate
            .command_unchecked(
                DesktopCommand::Preferences {
                    orientation: Color::Black,
                    analysis_defaults: AnalysisOperation::Moves { depth: 1, nodes: 0 },
                },
                1,
            )
            .unwrap();
        source.queue_admission(candidate).unwrap();
        let receipt = source.admission_receipt.clone().unwrap();
        // Save captures the accepted original, while the independent admission
        // worker checks a newer immutable candidate.
        let id = source.queue_save(path.clone()).unwrap();
        let snapshot = wait(&mut source, id);
        assert_eq!(snapshot.io.phase, IoPhase::Complete);
        let deadline = Instant::now() + std::time::Duration::from_secs(5);
        while receipt.completion().is_none() {
            source.snapshot();
            assert!(Instant::now() < deadline);
            thread::sleep(std::time::Duration::from_millis(1));
        }
        assert_eq!(receipt.completion(), Some(Ok(())));
        assert!(source.session.document_state().dirty);
        assert_eq!(source.session.document_path.as_ref(), Some(&path));
        let bytes = fs::read(&path).unwrap();
        let document = crate::document::DocumentEnvelope::parse(&bytes).unwrap();
        let file: ChessDocument = serde_json::from_str(document.payload.get()).unwrap();
        assert_eq!(file.settings.orientation, Color::White);
        assert_eq!(source.session.settings.orientation, Color::Black);
        let mut opened = controller();
        let id = opened.queue_open(path, opened.open_token()).unwrap();
        let mut candidate = opened.session.clone();
        candidate
            .command_unchecked(
                DesktopCommand::Preferences {
                    orientation: Color::Black,
                    analysis_defaults: AnalysisOperation::Moves {
                        depth: 2,
                        nodes: 10,
                    },
                },
                1,
            )
            .unwrap();
        opened.queue_admission(candidate).unwrap();
        let obsolete = opened.admission_receipt.clone().unwrap();
        // Consume a completed Open before polling admission, deterministically
        // exercising the race where replacement wins.
        let deadline = Instant::now() + std::time::Duration::from_secs(5);
        while opened.files.shared.state.lock().unwrap().result.is_none() {
            assert!(Instant::now() < deadline);
            thread::sleep(std::time::Duration::from_millis(1));
        }
        opened.poll_files();
        assert_eq!(opened.io_state.id, id);
        assert_eq!(opened.io_state.phase, IoPhase::Complete);
        assert!(obsolete.completion().unwrap().is_err());
        opened.poll_admission();
        assert_eq!(opened.session.settings.orientation, Color::White);
        assert_eq!(
            opened.session.settings.analysis_defaults,
            file.settings.analysis_defaults
        );
        assert_ne!(opened.session.capacity.status, CapacityPhase::Paused);
        assert!(opened.session.check_archive_capacity(&mut || false).is_ok());
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn final_engine_observation_256_commits_once_then_further_engine_work_resource_pauses() {
        let mut session = analysis_observations(255);
        session.retire_analysis();
        session.human = Color::Black;
        session.play_paused = false;
        session.engine = EngineState::default();
        let context = session.engine_request().unwrap().context;
        let action = "e2e4".parse().unwrap();
        let response = test_response(
            context,
            ChessResult::Move {
                action,
                telemetry: mshogi_engine::Telemetry {
                    score: Some(0),
                    completed_depth: 1,
                    nodes: 21,
                    principal_variation: vec![mshogi_engine::EngineChoice::Move { action }],
                    status: mshogi_engine::StopReason::Complete,
                },
            },
        );
        assert!(session.accept(response.clone(), 256));
        assert!(!session.accept(response, 256));
        assert_eq!(session.observations.len(), 256);
        assert_eq!(session.snapshot().history, ["e2e4"]);
        session
            .command(
                DesktopCommand::Play {
                    version: session.version,
                    action: "e7e5".parse().unwrap(),
                },
                257,
            )
            .unwrap();
        let before = session.game.record().to_json();
        assert!(session.engine_request().is_none());
        assert_eq!(session.capacity.status, CapacityPhase::Paused);
        assert_eq!(session.game.record().to_json(), before);
        assert_eq!(session.observations.len(), 256);
        assert!(session.game.conclusion().is_none());
        assert!(session.check_archive_capacity(&mut || false).is_ok());
    }
    #[test]
    fn cancellation_retires_an_uncommitted_engine_result_candidate() {
        let mut source = controller();
        source.session.human = Color::Black;
        let context = source.session.engine_request().unwrap().context;
        let action = "e2e4".parse().unwrap();
        let mut candidate = source.session.clone();
        assert!(
            candidate.accept_unchecked(
                test_response(
                    context,
                    ChessResult::Move {
                        action,
                        telemetry: mshogi_engine::Telemetry {
                            score: Some(0),
                            completed_depth: 1,
                            nodes: 21,
                            principal_variation: vec![mshogi_engine::EngineChoice::Move { action }],
                            status: mshogi_engine::StopReason::Complete
                        }
                    }
                )
                .observation,
                1
            )
        );
        source.queue_admission(candidate).unwrap();
        let result_receipt = source.admission_receipt.clone().unwrap();
        let (_, cancel_receipt) = source
            .command_with_receipt(DesktopCommand::Cancel {
                version: source.session.version,
            })
            .unwrap();
        let deadline = Instant::now() + std::time::Duration::from_secs(5);
        while cancel_receipt.completion().is_none() {
            source.snapshot();
            assert!(Instant::now() < deadline);
            thread::sleep(std::time::Duration::from_millis(1));
        }
        assert!(result_receipt.completion().unwrap().is_err());
        assert_eq!(cancel_receipt.completion(), Some(Ok(())));
        assert!(source.session.snapshot().history.is_empty());
        assert!(source.session.observations.is_empty());
        assert_eq!(source.session.engine.status, EnginePhase::Cancelled);
        assert!(source.session.check_archive_capacity(&mut || false).is_ok());
    }
    #[test]
    fn concurrent_save_as_bookkeeping_survives_later_candidate_commit() {
        let directory = directory();
        let path = directory.join("new-path.mshogi");
        let mut source = controller();
        let mut candidate = source.session.clone();
        candidate
            .command_unchecked(
                DesktopCommand::Preferences {
                    orientation: Color::Black,
                    analysis_defaults: AnalysisOperation::Moves { depth: 1, nodes: 0 },
                },
                1,
            )
            .unwrap();
        source.queue_admission(candidate).unwrap();
        let receipt = source.admission_receipt.clone().unwrap();
        let id = source.queue_save(path.clone()).unwrap();
        let deadline = Instant::now() + std::time::Duration::from_secs(5);
        while source.files.shared.state.lock().unwrap().result.is_none() {
            assert!(Instant::now() < deadline);
            thread::sleep(std::time::Duration::from_millis(1));
        }
        source.poll_files();
        assert_eq!(source.io_state.id, id);
        assert_eq!(source.io_state.phase, IoPhase::Complete);
        assert_eq!(source.session.document_path.as_ref(), Some(&path));
        assert!(!source.session.document_state().dirty);
        while receipt.completion().is_none() {
            source.poll_admission();
            assert!(Instant::now() < deadline);
            thread::sleep(std::time::Duration::from_millis(1));
        }
        assert_eq!(receipt.completion(), Some(Ok(())));
        assert_eq!(source.session.document_path.as_ref(), Some(&path));
        assert_eq!(source.session.saved_revision, 0);
        assert!(source.session.document_state().dirty);
        assert_eq!(source.session.settings.orientation, Color::Black);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn event_4096_commits_but_engine_event_4097_rejects_before_move_or_observation() {
        let mut source = controller();
        for index in 1..4095 {
            let reply = source.session.game.submit(MatchRequest {
                at_ns: index,
                version: source.session.game.version(),
                actor: Color::White,
                command: MatchCommand::Observe { witness: None },
            });
            assert!(reply.rejection.is_none());
        }
        assert_eq!(source.session.game.record().event_count(), 4095);
        source
            .session
            .command(
                DesktopCommand::Play {
                    version: 1,
                    action: "e2e4".parse().unwrap(),
                },
                4095,
            )
            .unwrap();
        assert_eq!(source.session.game.record().event_count(), 4096);
        let context = source.session.engine_request().unwrap().context;
        let action = "e7e5".parse().unwrap();
        let before = source.session.game.record().to_json();
        let revision = source.session.document_revision;
        assert!(!source.session.accept(
            test_response(
                context,
                ChessResult::Move {
                    action,
                    telemetry: mshogi_engine::Telemetry {
                        score: Some(0),
                        completed_depth: 1,
                        nodes: 21,
                        principal_variation: vec![mshogi_engine::EngineChoice::Move { action }],
                        status: mshogi_engine::StopReason::Complete
                    }
                }
            ),
            4096
        ));
        assert_eq!(source.session.game.record().to_json(), before);
        assert_eq!(source.session.document_revision, revision);
        assert_eq!(source.session.capacity.status, CapacityPhase::Paused);
        assert!(source.session.observations.is_empty());
        let directory = directory();
        let path = directory.join("event-boundary.mshogi");
        let id = source.queue_save(path.clone()).unwrap();
        assert_eq!(wait(&mut source, id).io.phase, IoPhase::Complete);
        let mut reopened = controller();
        let id = reopened.queue_open(path, reopened.open_token()).unwrap();
        assert_eq!(wait(&mut reopened, id).io.phase, IoPhase::Complete);
        assert_eq!(reopened.session.game.record().event_count(), 4096);
        assert_eq!(reopened.session.game.record().to_json(), before);
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn io_and_rejection_revisions_cannot_consume_the_persisted_save_watermark() {
        let mut source = controller();
        source.session.revision = MAX_SAFE_ID - 80;
        source.session.archive_revision = 1;
        source.session.version = 1;
        let directory = directory();
        let path = directory.join("watermark.mshogi");
        // Runtime-only Save progress crosses both the ordinary mutation guard
        // and the archive preflight watermark. Persisted revision stays stable.
        for _ in 0..24 {
            source
                .session
                .capacity_pause("Application capacity: test refusal".into());
            let id = source.queue_save(path.clone()).unwrap();
            assert_eq!(wait(&mut source, id).io.phase, IoPhase::Complete);
        }
        assert!(source.session.revision >= MAX_SAFE_ID - 16);
        let bytes = fs::read(&path).unwrap();
        let document = crate::document::DocumentEnvelope::parse(&bytes).unwrap();
        let saved: ChessDocument = serde_json::from_str(document.payload.get()).unwrap();
        assert_eq!(saved.revision, 1);
        assert!(preflight(&saved).is_ok());
        let mut reopened = controller();
        let id = reopened
            .queue_open(path.clone(), reopened.open_token())
            .unwrap();
        assert_eq!(wait(&mut reopened, id).io.phase, IoPhase::Complete);
        let mut unsupported = saved;
        unsupported.version = MAX_SAFE_ID - 34;
        unsupported.revision = MAX_SAFE_ID - 34;
        unsupported.next_request = MAX_SAFE_ID - 34;
        unsupported.document_revision = MAX_SAFE_ID - 34;
        fs::write(&path, encode_chess(&unsupported).unwrap()).unwrap();
        let token = reopened.open_token();
        let before = reopened.session.game.record().to_json();
        let id = reopened.queue_open(path, token).unwrap();
        assert_eq!(wait(&mut reopened, id).io.phase, IoPhase::Failed);
        assert_eq!(reopened.open_token(), token);
        assert_eq!(reopened.session.game.record().to_json(), before);
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn real_bounded_witness_observations_reach_byte_capacity_and_remain_saveable() {
        let game =
            AdjudicatedGame::from_fen("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", ClaimPolicy::ExplicitOnly)
                .unwrap();
        let mut source = controller();
        source.session = DesktopSession::from_record(&game.record(), Color::White).unwrap();
        // A legal cooperative cycle, then Qg7 mate. Every result is below the
        // engine's 2 MiB wire cap and obeys its recorded Full work bounds.
        let cycle: Vec<Action> = ["g6g5", "h8h7", "g5g6", "h7h8"]
            .into_iter()
            .map(|text| text.parse().unwrap())
            .collect();
        let mut witness = cycle.repeat(10_000);
        witness.push("g6g7".parse().unwrap());
        assert_eq!(
            mating_feasibility(game.position(), Color::White, Some(&witness)).unwrap(),
            MatingFeasibility::Possible
        );
        let work = ReachabilityWork {
            positions: witness.len() as u64 + 1,
            transitions: witness.len() as u64,
            expanded: witness.len() as u64,
            ..ReachabilityWork::default()
        };
        let mut accepted = 0;
        loop {
            source
                .session
                .command(
                    DesktopCommand::Analyze {
                        version: source.session.version,
                        operation: AnalysisOperation::FullReachability {
                            target: mshogi_engine::ReachabilityTarget::White,
                            max_positions: 100_000,
                            max_transitions: 1_000_000,
                        },
                    },
                    accepted as u64,
                )
                .unwrap();
            let context = source.session.analysis_request().unwrap().context;
            let response = test_response(
                context,
                ChessResult::Reachability {
                    result: MateReachability::Possible {
                        witness: witness.clone(),
                        work,
                    },
                },
            );
            assert!(
                serde_json::to_vec(&response).unwrap().len() < mshogi_engine::MAX_RESPONSE_BYTES
            );
            let before = source.session.game.record().to_json();
            let revision = source.session.document_revision;
            if !source.session.accept_analysis(response) {
                assert_eq!(source.session.capacity.status, CapacityPhase::Paused);
                assert!(
                    source
                        .session
                        .capacity
                        .message
                        .as_deref()
                        .unwrap()
                        .contains("16 MiB")
                );
                assert_eq!(source.session.game.record().to_json(), before);
                assert_eq!(source.session.document_revision, revision);
                assert_eq!(source.session.observations.len(), accepted);
                break;
            }
            accepted += 1;
            assert!(
                accepted < 256,
                "the byte limit must precede the observation count for this fixture"
            );
        }
        assert!(accepted > 1);
        let bytes = encode_chess(&archive(&source.session)).unwrap();
        assert!(bytes.len() <= MAX_ARCHIVE_BYTES);
        // An independently encoded next observation crosses the actual encoded
        // archive cap; this is not merely a count or helper boundary assertion.
        let mut too_large = archive(&source.session);
        let mut next = too_large.observations.last().unwrap().clone();
        next.sequence += 1;
        too_large.observations.push(next);
        assert!(encode_chess(&too_large).unwrap().len() > MAX_ARCHIVE_BYTES);
        let directory = directory();
        let path = directory.join("byte-boundary.mshogi");
        let id = source.queue_save(path.clone()).unwrap();
        assert_eq!(wait(&mut source, id).io.phase, IoPhase::Complete);
        let mut reopened = controller();
        let id = reopened.queue_open(path, reopened.open_token()).unwrap();
        assert_eq!(wait(&mut reopened, id).io.phase, IoPhase::Complete);
        assert_eq!(reopened.session.observations.len(), accepted);
        assert_eq!(
            reopened.session.game.record().to_json(),
            source.session.game.record().to_json()
        );
        fs::remove_dir_all(directory).unwrap();
    }
}

#[cfg(test)]
mod takeback_tests {
    use super::*;
    fn tick(s: &DesktopSession) -> u64 {
        s.controls
            .last()
            .map_or(0, |c| c.at_ns)
            .max(s.game.record().timestamp_floor())
            + 1
    }
    fn play(s: &mut DesktopSession, action: &str) {
        s.command(
            DesktopCommand::Play {
                version: s.version,
                action: action.parse().unwrap(),
            },
            tick(s),
        )
        .unwrap();
    }
    fn engine(s: &mut DesktopSession) -> ExecutionObservation {
        let request = s.engine_request().unwrap().context;
        let mut bytes = Vec::new();
        mshogi_engine::serve_once(
            std::io::Cursor::new(format!(
                "{}\n",
                serde_json::to_string(&test_request(request.clone())).unwrap()
            )),
            &mut bytes,
        )
        .unwrap();
        let response = serde_json::from_slice(&bytes).unwrap();
        assert!(s.accept(response, tick(s)));
        s.observations.last().unwrap().observation.clone()
    }
    fn take(s: &mut DesktopSession) {
        s.command(DesktopCommand::TakeBack { version: s.version }, tick(s))
            .unwrap();
    }
    fn encoded(s: &DesktopSession) -> Vec<u8> {
        let mut archive = s.archive_snapshot();
        archive.match_record_json = s.game.record().to_json();
        archive.game_record_json = s.game.game().record().to_json();
        encode_chess(&archive).unwrap()
    }
    #[test]
    fn takeback_chess_pairs_pending_cancelled_results_and_repeated_native_decisions() {
        let mut s = DesktopSession::new();
        assert!(!s.snapshot().can_take_back);
        let before = encoded(&s);
        assert!(
            s.command(DesktopCommand::TakeBack { version: s.version }, 1)
                .is_err()
        );
        assert_eq!(encoded(&s), before);
        play(&mut s, "e2e4");
        let abandoned = s.engine_request().unwrap().context;
        let mut output = Vec::new();
        mshogi_engine::serve_once(
            std::io::Cursor::new(format!(
                "{}\n",
                serde_json::to_string(&test_request(abandoned.clone())).unwrap()
            )),
            &mut output,
        )
        .unwrap();
        let late: ExecutionResponse = serde_json::from_slice(&output).unwrap();
        let old_version = s.version;
        let old_selection = s.selection_id;
        take(&mut s);
        assert!(s.play_paused);
        assert!(s.version > old_version);
        assert!(s.selection_id > old_selection);
        assert!(!s.accept(late, 5));
        assert!(s.snapshot().history.is_empty());
        assert!(s.engine_request().is_none());
        s.command(DesktopCommand::Resume { version: s.version }, 6)
            .unwrap();
        play(&mut s, "e2e4");
        engine(&mut s);
        let paired = s.game.game().record();
        assert_eq!(s.snapshot().history.len(), 2);
        let action = s.snapshot().legal_choices[0].action;
        s.command(
            DesktopCommand::Play {
                version: s.version,
                action,
            },
            10,
        )
        .unwrap();
        engine(&mut s);
        take(&mut s);
        assert_eq!(s.game.game().record(), paired);
        take(&mut s);
        assert!(s.snapshot().history.is_empty());
        assert_eq!(s.observations.len(), 2);
        assert!(s.observations.iter().all(|o| o.historical));
        let bytes = encoded(&s);
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(value["envelope_version"], 1);
        assert_eq!(value["format"], crate::document::FORMAT);
        assert_eq!(value["game_kind"], "chess");
        assert_eq!(value["native_schema_version"], 1);
        assert!(value["payload"].get("schema_version").is_none());
        assert!(value["payload"].get("profile").is_none());
        let reopened =
            super::super::shared::archive::decode_chess_document(&bytes, &mut || false).unwrap();
        assert_eq!(reopened.game.game().record(), s.game.game().record());
        assert_eq!(reopened.archive.observations.len(), 2);
        assert_eq!(reopened.archive.takebacks.len(), 3);
    }
    #[test]
    fn takeback_chess_engine_opening_survives_and_terminal_commands_restore_native_turn() {
        let root = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record();
        let mut s = DesktopSession::from_record(&root, Color::Black).unwrap();
        engine(&mut s);
        let opening = s.game.game().record();
        assert!(!s.snapshot().can_take_back);
        let action = s.snapshot().legal_choices[0].action;
        s.command(
            DesktopCommand::Play {
                version: s.version,
                action,
            },
            2,
        )
        .unwrap();
        engine(&mut s);
        take(&mut s);
        assert_eq!(s.game.game().record(), opening);
        assert_eq!(s.snapshot().history.len(), 1);
        assert!(!s.snapshot().can_take_back);
        let mut own = DesktopSession::new();
        own.command(
            DesktopCommand::Resign {
                version: own.version,
            },
            1,
        )
        .unwrap();
        assert!(own.snapshot().can_take_back);
        take(&mut own);
        assert!(own.game.conclusion().is_none());
        assert!(own.snapshot().history.is_empty());
        let mut off = DesktopSession::new();
        play(&mut off, "e2e4");
        off.command(
            DesktopCommand::Resign {
                version: off.version,
            },
            2,
        )
        .unwrap();
        take(&mut off);
        assert!(off.snapshot().history.is_empty());
        assert!(off.game.conclusion().is_none());
    }
    #[test]
    fn takeback_chess_claim_and_checkmate_preserve_correct_history_semantics() {
        let mut game =
            AdjudicatedGame::from_fen("7k/8/8/8/8/8/R7/K7 w - - 100 1", ClaimPolicy::ExplicitOnly)
                .unwrap();
        let mut s = DesktopSession::from_record(&game.record(), Color::White).unwrap();
        let claim = s
            .snapshot()
            .claims
            .into_iter()
            .find(|c| c.intended.is_none())
            .unwrap();
        s.command(
            DesktopCommand::Claim {
                version: s.version,
                claim,
            },
            1,
        )
        .unwrap();
        assert!(s.game.conclusion().is_some());
        take(&mut s);
        assert!(s.game.conclusion().is_none());
        assert!(!s.snapshot().claims.is_empty());
        assert_eq!(s.game.game().record(), game.record());
        game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        let mut mate = DesktopSession::from_record(&game.record(), Color::White).unwrap();
        for (i, action) in ["f2f3", "e7e5", "g2g4", "d8h4"].into_iter().enumerate() {
            let actor = mate.game.game().position().side_to_move();
            mate.apply_match(
                actor,
                MatchCommand::Move(action.parse().unwrap()),
                i as u64 + 1,
            )
            .unwrap();
        }
        assert!(mate.game.conclusion().is_some());
        take(&mut mate);
        assert_eq!(mate.snapshot().history, ["f2f3", "e7e5"]);
        assert!(mate.game.conclusion().is_none());
        super::super::shared::archive::decode_chess_document(&encoded(&mate), &mut || false)
            .unwrap();
    }
    #[test]
    fn takeback_chess_strict_lineage_conflicts_capacity_and_interruption_preserve_current() {
        let mut s = DesktopSession::new();
        play(&mut s, "e2e4");
        engine(&mut s);
        take(&mut s);
        let good = encoded(&s);
        let parsed: serde_json::Value = serde_json::from_slice(&good).unwrap();
        for edit in 0..4 {
            let mut bad = parsed.clone();
            match edit {
                0 => bad["payload"]["takebacks"][0]["control_sequence"] = 0.into(),
                1 => bad["native_schema_version"] = 2.into(),
                2 => bad["payload"]["takebacks"] = serde_json::json!([]),
                _ => bad["payload"]["takebacks"][0]["match_record_json"] = "{}".into(),
            }
            assert!(
                super::super::shared::archive::decode_chess_document(
                    &serde_json::to_vec(&bad).unwrap(),
                    &mut || false
                )
                .is_err()
            );
        }
        for (field, value) in [
            ("envelope_version", parsed["envelope_version"].to_string()),
            (
                "control_sequence",
                parsed["payload"]["takebacks"][0]["control_sequence"].to_string(),
            ),
        ] {
            let original = String::from_utf8(good.clone()).unwrap();
            let pretty = format!("\"{field}\": {value}");
            let compact = format!("\"{field}\":{value}");
            let needle = if original.contains(&pretty) {
                pretty
            } else {
                compact
            };
            assert!(
                original.contains(&needle),
                "duplicate fixture must edit a real field"
            );
            let duplicate =
                original.replacen(&needle, &format!("{needle}, \"{field}\": {value}"), 1);
            assert!(
                super::super::shared::archive::decode_chess_document(
                    duplicate.as_bytes(),
                    &mut || false
                )
                .is_err()
            );
        }
        // A complete native payload alone and retired archive identities are
        // rejected rather than guessed into the current game document.
        let naked = serde_json::to_vec(&parsed["payload"]).unwrap();
        assert!(
            super::super::shared::archive::decode_chess_document(&naked, &mut || false).is_err()
        );
        for old in [
            "mshogi.desktop.archive.v1",
            "mshogi.desktop.archive.v2",
            "mshogi.desktop.archive.v3",
        ] {
            let mut retired = parsed.clone();
            retired["format"] = old.into();
            assert!(
                super::super::shared::archive::decode_chess_document(
                    &serde_json::to_vec(&retired).unwrap(),
                    &mut || false
                )
                .is_err()
            );
        }
        assert!(super::super::shared::archive::decode_chess_document(&good, &mut || true).is_err());
        let mut full = DesktopSession::new();
        play(&mut full, "e2e4");
        let original = full.controls[0].clone();
        full.controls = (0..MAX_ARCHIVE_CONTROLS)
            .map(|i| ControlEntry {
                sequence: i as u64 + 1,
                command: Some(DesktopCommand::Preferences {
                    orientation: Color::White,
                    analysis_defaults: AnalysisOperation::Moves { depth: 1, nodes: 1 },
                }),
                ..original.clone()
            })
            .collect();
        let record = full.game.record().to_json();
        let observations = full.observations.len();
        assert!(
            full.command(
                DesktopCommand::TakeBack {
                    version: full.version
                },
                2
            )
            .unwrap_err()
            .starts_with("Application capacity:")
        );
        assert_eq!(full.game.record().to_json(), record);
        assert!(full.takebacks.is_empty());
        assert_eq!(full.observations.len(), observations);
    }
    #[test]
    fn takeback_chess_current_replay_boundary_reserves_actual_close_control() {
        let mut s = DesktopSession::new();
        play(&mut s, "e2e4");
        for at_ns in 2..1322 {
            s.apply_match(Color::Black, MatchCommand::Observe { witness: None }, at_ns)
                .unwrap();
        }
        take(&mut s);
        s.command(DesktopCommand::Resume { version: s.version }, tick(&s))
            .unwrap();
        play(&mut s, "e2e4");
        s.engine_request().unwrap();
        let mut accepted = 0;
        loop {
            let prior = s.clone();
            let result = s.command(
                DesktopCommand::Preferences {
                    orientation: Color::White,
                    analysis_defaults: AnalysisOperation::Moves { depth: 1, nodes: 1 },
                },
                tick(&s),
            );
            if let Err(error) = result {
                assert!(error.starts_with("Application capacity:"), "{error}");
                s = prior;
                break;
            }
            accepted += 1;
            assert!(accepted < MAX_ARCHIVE_CONTROLS);
        }
        assert!(s.pending.is_some());
        assert!(accepted > 10);
        let mut c = DesktopController::new("/unused/msh113-close-engine".into()).unwrap();
        c.session = s;
        let before = c.session.controls.len();
        c.pause_for_close();
        assert_eq!(c.session.controls.len(), before + 1);
        assert!(c.session.controls.last().unwrap().close_pause);
        c.session.check_archive_capacity(&mut || false).unwrap();
        let bytes = encoded(&c.session);
        super::super::shared::archive::decode_chess_document(&bytes, &mut || false).unwrap();
        let last = c.session.initial_record.events().len() + c.session.game.record().event_count();
        c.session.viewed = Some(view_prefix(c.session.game.record(), last, &mut || false).unwrap());
        c.session.check_archive_capacity(&mut || false).unwrap();
        let selected = encoded(&c.session);
        let reopened =
            super::super::shared::archive::decode_chess_document(&selected, &mut || false).unwrap();
        assert_eq!(reopened.viewed.unwrap().index, last);
        println!(
            "MSH-117 current envelope lineage boundary: {} accepted preferences, {} controls, {} encoded bytes; mandatory close retained and reopened",
            accepted,
            c.session.controls.len(),
            bytes.len()
        );
    }
}
