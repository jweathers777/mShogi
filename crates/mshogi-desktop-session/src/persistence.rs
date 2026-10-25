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
    pub response: EngineResponse,
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
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Archive {
    schema_version: u32,
    profile: String,
    match_record_json: String,
    game_record_json: String,
    human: Color,
    settings: DesktopSettings,
    play_limits: WorkLimits,
    observations: Vec<ArchivedObservation>,
    controls: Vec<ControlEntry>,
    version: u64,
    revision: u64,
    next_request: u64,
    document_revision: u64,
}
struct Opened {
    archive: Archive,
    game: ChessMatch,
}
pub(super) struct Viewed {
    game: AdjudicatedGame,
    conclusion: Option<MatchConclusion>,
    index: usize,
}
enum Work {
    Save {
        archive: Box<Archive>,
        record: MatchRecord,
        game_record: Box<GameRecord>,
        path: PathBuf,
        revision: u64,
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
enum Completed {
    Saved {
        path: PathBuf,
        revision: u64,
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
                    let result = execute(job.work, stopped);
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
    fn submit(&self, id: u64, work: Work) {
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
    fn poll(&self) -> Option<(u64, Result<Completed, String>)> {
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
fn check(stopped: &mut impl FnMut() -> bool) -> Result<(), String> {
    if stopped() {
        Err("Document operation cancelled or exceeded its 15-second validation budget.".into())
    } else {
        Ok(())
    }
}
fn execute(work: Work, mut stopped: impl FnMut() -> bool) -> Result<Completed, String> {
    match work {
        Work::Save {
            mut archive,
            record,
            game_record,
            path,
            revision,
        } => {
            archive.match_record_json = record.to_json();
            archive.game_record_json = game_record.to_json();
            preflight(&archive)?;
            check(&mut stopped)?;
            let bytes = serde_json::to_vec_pretty(&archive).map_err(|e| e.to_string())?;
            if bytes.len() > MAX_ARCHIVE_BYTES {
                return Err(
                    "Document exceeds the 16 MiB archive byte limit; no data was truncated.".into(),
                );
            }
            atomic_write(&path, &bytes, &mut stopped)?;
            Ok(Completed::Saved { path, revision })
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
            let archive: Archive = serde_json::from_slice(&bytes)
                .map_err(|e| format!("Invalid desktop archive: {e}"))?;
            let record = preflight(&archive)?;
            check(&mut stopped)?;
            let game = record
                .replay_with_stop(&mut stopped)
                .map_err(|e| e.to_string())?;
            let final_record =
                GameRecord::from_json(&archive.game_record_json).map_err(|e| e.to_string())?;
            if game.game().record() != final_record {
                return Err(
                    "Desktop final GameRecord disagrees with complete MatchRecord replay.".into(),
                );
            }
            for observation in &archive.observations {
                check(&mut stopped)?;
                validate_observation(observation, &final_record, &mut stopped)?;
            }
            Ok(Completed::Opened {
                opened: Box::new(Opened { archive, game }),
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
fn preflight(archive: &Archive) -> Result<MatchRecord, String> {
    if archive.schema_version != 1 || archive.profile != "mshogi.desktop.archive.v1" {
        return Err("Unsupported desktop archive schema/profile.".into());
    }
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
            return Err("Desktop archive v1 does not support witnessed match commands; supplied certificates were not replayed or discarded.".into());
        }
    }
    for (i, observation) in archive.observations.iter().enumerate() {
        if let mshogi_engine::RequestPurpose::MoveAnalysis { target }
        | mshogi_engine::RequestPurpose::FullReachability { target, .. } =
            &observation.response.context.purpose
            && (target.selection_id == 0
                || target.selection_id > archive.next_request
                || target.selection_id >= MAX_SAFE_ID)
        {
            return Err("Observation selection identity exceeds its saved watermark.".into());
        }
        let context = &observation.response.context;
        if observation.sequence != (i + 1) as u64
            || context.request_id > archive.next_request
            || context.process_generation > archive.next_request
            || context.session_version > archive.version
            || observation.label.is_empty()
            || observation.label.len() > 256
            || context.request_id == 0
            || context.process_generation == 0
            || !context.valid_limits()
            || context.protocol != mshogi_engine::PROTOCOL_VERSION
            || context.game.adjudication_mode() != AdjudicationMode::M2
            || context.game.events().len() > MAX_ARCHIVE_EVENTS
        {
            return Err("Invalid observation ordering, identity or live-mode context.".into());
        }
    }
    Ok(record)
}
fn validate_observation(
    observation: &ArchivedObservation,
    final_record: &GameRecord,
    stopped: &mut impl FnMut() -> bool,
) -> Result<(), String> {
    let context = &observation.response.context;
    let replay = AdjudicatedGame::replay_record_with_stop(&context.game, &mut *stopped)
        .map_err(|e| e.to_string())?;
    if final_record.prefix(context.game.events().len()).as_ref() != Some(&context.game) {
        return Err("Observation target history is not a prefix of this match.".into());
    }
    if let mshogi_engine::RequestPurpose::MoveAnalysis { target }
    | mshogi_engine::RequestPurpose::FullReachability { target, .. } = &context.purpose
        && (target.turn != replay.position().side_to_move()
            || target.ply != replay.history().ply_count() as u64
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
    use mshogi_engine::{EngineChoice, RequestPurpose, StopReason};
    let context = &observation.response.context;
    let result = &observation.response.result;
    let full = matches!(context.purpose, RequestPurpose::FullReachability { .. });
    // Rejected native choices remain explicit historical failure evidence.
    if let Some(message) = &observation.application_error {
        if message.is_empty() || message.len() > mshogi_engine::MAX_STDERR_BYTES {
            return Err("Invalid observation application diagnostic.".into());
        }
        let invalid = match result {
            EngineResult::Move { action, .. } if !full => game.clone().apply(*action).is_err(),
            EngineResult::Claim { claim, .. } if !full => {
                game.clone().claim((*claim).into()).is_err()
            }
            EngineResult::Finished { outcome, .. } if !full => game.outcome() != Some(*outcome),
            EngineResult::Reachability { .. } => !full,
            _ => full,
        };
        return if invalid {
            Ok(())
        } else {
            Err("Observation marks a valid choice as an application failure.".into())
        };
    }
    if let EngineResult::Failure { failure } = result {
        return if failure.message.len() <= mshogi_engine::MAX_STDERR_BYTES {
            Ok(())
        } else {
            Err("Failure diagnostic exceeds its byte limit.".into())
        };
    }
    if let EngineResult::Reachability { result } = result {
        let RequestPurpose::FullReachability { limits, .. } = context.purpose else {
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
                let RequestPurpose::FullReachability { mate, .. } = context.purpose else {
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
        EngineResult::Move { telemetry, .. }
        | EngineResult::Claim { telemetry, .. }
        | EngineResult::Finished { telemetry, .. }
        | EngineResult::Incomplete { telemetry } => telemetry,
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
        EngineResult::Incomplete { .. }
            if telemetry.completed_depth != 0 || telemetry.status == StopReason::Complete =>
        {
            return Err("Incomplete result contains a completed iteration.".into());
        }
        EngineResult::Move { action, .. }
            if telemetry.principal_variation.first()
                != Some(&EngineChoice::Move { action: *action }) =>
        {
            return Err("Move decision disagrees with its continuation.".into());
        }
        EngineResult::Claim { claim, .. }
            if telemetry.principal_variation.first()
                != Some(&EngineChoice::Claim { claim: *claim }) =>
        {
            return Err("Claim decision disagrees with its continuation.".into());
        }
        EngineResult::Finished { outcome, .. }
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
fn atomic_write(
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

impl DesktopSession {
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
    pub(super) fn remember(&mut self, response: EngineResponse) {
        let label = match &response.context.purpose {
            mshogi_engine::RequestPurpose::EngineTurn => format!(
                "Engine turn after {} plies",
                response
                    .context
                    .game
                    .events()
                    .iter()
                    .filter(|event| matches!(event.kind, RecordEventKind::Move(_)))
                    .count()
            ),
            mshogi_engine::RequestPurpose::MoveAnalysis { target }
            | mshogi_engine::RequestPurpose::FullReachability { target, .. } => {
                target.label.clone()
            }
        };
        self.observations.push(ArchivedObservation {
            sequence: self.observations.len() as u64 + 1,
            label,
            analyzed_turn: match &response.context.purpose {
                mshogi_engine::RequestPurpose::EngineTurn => {
                    self.game.game().position().side_to_move()
                }
                mshogi_engine::RequestPurpose::MoveAnalysis { target }
                | mshogi_engine::RequestPurpose::FullReachability { target, .. } => target.turn,
            },
            response,
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
        if self.session.controls.len() > MAX_ARCHIVE_CONTROLS
            || self.session.observations.len() > MAX_ARCHIVE_OBSERVATIONS
            || self.session.initial_record.events().len() + self.session.game.record().event_count()
                > MAX_ARCHIVE_EVENTS
        {
            return Err("Document resource limits exceeded (4096 events, 256 observations, 8192 controls); no history was truncated or file changed.".into());
        }
        let id = self.next_io(IoPhase::Saving)?;
        let archive = Archive {
            schema_version: 1,
            profile: "mshogi.desktop.archive.v1".into(),
            match_record_json: String::new(),
            game_record_json: String::new(),
            human: self.session.human,
            settings: self.session.settings.clone(),
            play_limits: self.session.play_limits,
            observations: self.session.observations.clone(),
            controls: self.session.controls.clone(),
            version: self.session.version,
            revision: self.session.revision,
            next_request: self.session.next_request.max(self.session.selection_id),
            document_revision: self.session.document_revision,
        };
        self.files.submit(
            id,
            Work::Save {
                archive: Box::new(archive),
                record: self.session.game.record().clone(),
                game_record: Box::new(self.session.game.game().record()),
                path,
                revision: self.session.document_revision,
            },
        );
        Ok(id)
    }
    /// Queue strict bounded validation. A stale token cannot replace any state.
    pub fn queue_open(&mut self, path: PathBuf, token: OpenToken) -> Result<u64, String> {
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
            Ok(Completed::Saved { path, revision }) => {
                if self.session.document_revision == revision {
                    self.session.document_path = Some(path);
                    self.session.saved_revision = revision;
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
                    self.session.document_revision =
                        self.session.document_revision.max(saved.document_revision) + 1;
                    self.session.saved_revision = self.session.document_revision;
                    self.session.document_path = Some(path);
                    self.session.version = next;
                    self.session.revision = next;
                    self.session.next_request = next;
                    self.session.selection_id = next;
                    self.session.pending = None;
                    self.session.retire_analysis();
                    self.session.viewed = None;
                    self.session.play_paused = true;
                    self.session.engine = EngineState { status: EnginePhase::Paused, message: Some("Document reopened with work stopped. Saved observations are historical. Resume play explicitly.".into()), ..EngineState::default() };
                    Ok("Opened the complete document. Play remains paused until Resume.".into())
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
                    Ok("Viewing an exact read-only history prefix.".into())
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
    /// Stabilize a close decision by retiring work without inventing results.
    /// Repeated preflight is idempotent; an untouched initial game stays clean.
    pub fn pause_for_close(&mut self) -> Snapshot {
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
    fn archive(session: &DesktopSession) -> Archive {
        Archive {
            schema_version: 1,
            profile: "mshogi.desktop.archive.v1".into(),
            match_record_json: session.game.record().to_json(),
            game_record_json: session.game.game().record().to_json(),
            human: session.human,
            settings: session.settings.clone(),
            play_limits: session.play_limits,
            observations: session.observations.clone(),
            controls: session.controls.clone(),
            version: session.version,
            revision: session.revision,
            next_request: session.next_request.max(session.selection_id),
            document_revision: session.document_revision,
        }
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
                source
                    .command(DesktopCommand::Claim {
                        version: source.session.version,
                        claim: choice,
                    })
                    .unwrap();
            } else {
                source
                    .command(DesktopCommand::Resign {
                        version: source.session.version,
                    })
                    .unwrap();
            }
            let core_record = source.session.game.record().to_json();
            let conclusion = source.session.game.conclusion();
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
            .command(DesktopCommand::Preferences {
                orientation: Color::Black,
                analysis_defaults: AnalysisOperation::Moves {
                    depth: 2,
                    nodes: 20_000,
                },
            })
            .unwrap();
        let snapshot = wait(&mut controller, save);
        assert_eq!(snapshot.io.phase, IoPhase::Complete);
        assert!(snapshot.document.dirty);
        assert!(snapshot.document.path.is_none());
        let token = controller.open_token();
        let open = controller.queue_open(path, token).unwrap();
        controller
            .command(DesktopCommand::Preferences {
                orientation: Color::White,
                analysis_defaults: AnalysisOperation::Moves {
                    depth: 3,
                    nodes: 10_000,
                },
            })
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
        let request = session.analysis_request().unwrap();
        let mut input = serde_json::to_vec(&request).unwrap();
        input.push(b'\n');
        let mut output = Vec::new();
        mshogi_engine::serve_once(std::io::Cursor::new(input), &mut output).unwrap();
        assert!(session.accept_analysis(serde_json::from_slice(&output).unwrap()));
        let good = archive(&session);
        let final_record = session.game.game().record();
        assert!(validate_observation(&good.observations[0], &final_record, &mut || false).is_ok());
        let mut bad = good.clone();
        bad.observations[0].response.context.limits.depth = 9;
        assert!(preflight(&bad).is_err());
        let mut bad = good.observations[0].clone();
        if let EngineResult::Move { telemetry, .. } = &mut bad.response.result {
            telemetry.score = Some(i32::MIN);
        }
        assert!(validate_observation(&bad, &final_record, &mut || false).is_err());
        let mut bad = good.observations[0].clone();
        let mut wire: serde_json::Value =
            serde_json::from_str(&bad.response.context.game.to_json()).unwrap();
        wire["claim_policy"] = "auto_claim".into();
        bad.response.context.game = GameRecord::from_json(&wire.to_string()).unwrap();
        assert!(validate_observation(&bad, &final_record, &mut || false).is_err());
        let directory = directory();
        let path = directory.join("timestamps.mshogi");
        fs::write(&path, serde_json::to_vec(&good).unwrap()).unwrap();
        let mut restored = controller();
        let token = restored.open_token();
        let id = restored.queue_open(path, token).unwrap();
        assert_eq!(wait(&mut restored, id).io.phase, IoPhase::Complete);
        assert!(restored.at_ns() >= 100_000_000_000);
        restored
            .command(DesktopCommand::Resume {
                version: restored.session.version,
            })
            .unwrap();
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
        let request = session.analysis_request().unwrap();
        let mut bytes = serde_json::to_vec(&request).unwrap();
        bytes.push(b'\n');
        let mut output = Vec::new();
        mshogi_engine::serve_once(std::io::Cursor::new(bytes), &mut output).unwrap();
        assert!(session.accept_analysis(serde_json::from_slice(&output).unwrap()));
        let good = session.observations[0].clone();
        assert!(matches!(
            good.response.result,
            EngineResult::Reachability {
                result: MateReachability::Possible { .. }
            }
        ));
        assert!(validate_observation(&good, &live.record(), &mut || false).is_ok());
        for action in ["g6g5", "g6f6"] {
            let mut bad = good.clone();
            if let EngineResult::Reachability {
                result: MateReachability::Possible { witness, .. },
            } = &mut bad.response.result
            {
                *witness = vec![action.parse().unwrap()];
            }
            assert!(
                validate_observation(&bad, &live.record(), &mut || false).is_err(),
                "corrupt witness {action}"
            );
        }
        let mut wrong_side = good.clone();
        if let mshogi_engine::RequestPurpose::FullReachability { mate, .. } =
            &mut wrong_side.response.context.purpose
        {
            *mate = mshogi_engine::ReachabilityTarget::Black;
        }
        assert!(validate_observation(&wrong_side, &live.record(), &mut || false).is_err());
        let mut either = good.clone();
        if let mshogi_engine::RequestPurpose::FullReachability { mate, .. } =
            &mut either.response.context.purpose
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
        if let mshogi_engine::RequestPurpose::FullReachability { target, .. } =
            &mut overflow.observations[0].response.context.purpose
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
        let claim = controller.session.snapshot().claims[0];
        controller
            .command(DesktopCommand::Claim { version: 1, claim })
            .unwrap();
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
                0,
            )
            .unwrap();
        let request = controller.session.analysis_request().unwrap();
        assert!(request.game.events().is_empty());
        assert_eq!(request.game, game.record());
        let target = match &request.purpose {
            mshogi_engine::RequestPurpose::MoveAnalysis { target } => target,
            _ => panic!(),
        };
        assert_eq!(target.selection_id, controller.session.selection_id);
        controller
            .session
            .command(
                DesktopCommand::ReturnLive {
                    version: controller.session.version,
                },
                0,
            )
            .unwrap();
        assert!(controller.session.analysis_pending.is_none());
        assert_eq!(controller.session.game.record().to_json(), before);
        assert!(controller.session.viewed.is_none());
    }
}
