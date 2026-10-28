//! Authoritative untimed desktop sessions. Framework-free commands and snapshots.
#[cfg(feature = "desktop-verification")]
mod synthetic;
#[cfg(feature = "desktop-verification")]
pub use synthetic::*;
mod shared;
pub use shared::*;
mod capacity;
pub use capacity::*;
mod persistence;
pub use persistence::*;
mod analysis;
pub use analysis::*;
use mshogi_core::chess::*;
use mshogi_engine::{
    EngineBridge, EngineClaim, EngineRequest, EngineResponse, EngineResult, WorkLimits,
};
use serde::{Deserialize, Serialize};
use std::{io, path::PathBuf, time::Instant};

const MAX_SAFE_ID: u64 = 9_007_199_254_740_991;
/// Initial interactive work limits; recorded on every engine request.
pub const PLAY_LIMITS: WorkLimits = WorkLimits {
    depth: 2,
    nodes: 20_000,
};

/// One complete host command. A frontend version is mandatory for mutations.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DesktopCommand {
    /// Replace the live game, invalidating every previous search.
    NewGame {
        /// Expected current session version.
        version: u64,
        /// Exact document revision approved for replacement.
        document_revision: u64,
        /// Player color in the new game.
        human: Color,
    },
    /// Set preferences without changing game identity or cancelling work.
    Preferences {
        /// View orientation.
        orientation: Color,
        /// Default analysis form choices.
        analysis_defaults: AnalysisOperation,
    },
    /// Select an exact read-only history event boundary.
    Inspect {
        /// Current game version.
        version: u64,
        /// Native event boundary.
        event_index: usize,
    },
    /// Explicitly return to the live position, with play still paused.
    ReturnLive {
        /// Current game version.
        version: u64,
    },
    /// Commit one native complete action on the human's live turn.
    Play {
        /// Expected current session version.
        version: u64,
        /// Native complete action identifier.
        action: Action,
    },
    /// Exercise a native optional claim; its intended action is not played.
    Claim {
        /// Expected current session version.
        version: u64,
        /// Native claim choice.
        claim: EngineClaim,
    },
    /// Start or replace explicitly bounded, read-only analysis.
    Analyze {
        /// Current authoritative session version.
        version: u64,
        /// Purpose-specific deterministic work budgets.
        operation: AnalysisOperation,
    },
    /// Cancel analysis while keeping play paused.
    CancelAnalysis {
        /// Current session version.
        version: u64,
    },
    /// Retire analysis and explicitly resume ordinary play.
    Resume {
        /// Current session version.
        version: u64,
    },
    /// Explicit human resignation, including while the engine is thinking.
    /// M2 proof limits can leave the concluded match unresolved and unscored.
    Resign {
        /// Expected current session version.
        version: u64,
    },
    /// Stop work without a fallback move or chess result.
    Cancel {
        /// Expected current session version.
        version: u64,
    },
    /// Explicitly retry a paused engine turn with a fresh identity/generation.
    Retry {
        /// Expected current session version.
        version: u64,
    },
}
impl DesktopCommand {
    fn version(&self) -> Option<u64> {
        match self {
            Self::Preferences { .. } => None,
            Self::Inspect { version, .. }
            | Self::ReturnLive { version }
            | Self::NewGame { version, .. }
            | Self::Play { version, .. }
            | Self::Claim { version, .. }
            | Self::Resign { version }
            | Self::Analyze { version, .. }
            | Self::CancelAnalysis { version }
            | Self::Resume { version }
            | Self::Cancel { version }
            | Self::Retry { version } => Some(*version),
        }
    }
}
/// Native piece presentation; coordinates are ordinary chess square names.
#[derive(Clone, Debug, Serialize)]
pub struct BoardPiece {
    /// Square occupied by the piece.
    pub square: String,
    /// Piece owner.
    pub color: Color,
    /// Lowercase orthodox piece name.
    pub kind: String,
    /// Accessible square and piece description.
    pub label: String,
}
/// One legal complete action, split only for UI selection.
#[derive(Clone, Debug, Serialize)]
pub struct LegalChoice {
    /// Canonical complete action identifier.
    pub action: Action,
    /// Source coordinate.
    pub from: String,
    /// Destination coordinate.
    pub to: String,
    /// Explicit promotion label when required.
    pub promotion: Option<String>,
}
/// Engine activity is separate from the game's conclusion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EnginePhase {
    /// No work is running.
    Idle,
    /// A bounded request is outstanding.
    Thinking,
    /// Analysis owns execution; explicit Resume is required.
    Paused,
    /// User cancelled; explicit retry is needed.
    Cancelled,
    /// A process/search/protocol failure paused the engine turn.
    Failed,
    /// No full iteration was completed; no fallback is applied.
    Incomplete,
}
/// Last actual engine observations, never synthetic progress.
#[derive(Clone, Debug, Serialize)]
pub struct EngineState {
    /// Current process/search lifecycle.
    pub status: EnginePhase,
    /// Last final result, kept distinct from current position evaluation.
    pub last: Option<EngineResult>,
    /// Side whose perspective the last score describes.
    pub last_turn: Option<Color>,
    /// Session version searched by the last result.
    pub searched_version: Option<u64>,
    /// Plain diagnostic separate from any chess result.
    pub message: Option<String>,
}
impl Default for EngineState {
    fn default() -> Self {
        Self {
            status: EnginePhase::Idle,
            last: None,
            last_turn: None,
            searched_version: None,
            message: None,
        }
    }
}
/// Immutable native view. Revision protects async frontend responses.
#[derive(Clone, Debug, Serialize)]
pub struct Snapshot {
    /// Monotonic observation revision, including engine lifecycle changes.
    pub revision: u64,
    /// Monotonic live-session command version, independent of ply count.
    pub version: u64,
    /// Color controlled by the human.
    pub human: Color,
    /// Current side to move.
    pub turn: Color,
    /// Canonical current position, for inspection only.
    pub fen: String,
    /// Native piece list.
    pub pieces: Vec<BoardPiece>,
    /// Complete legal actions, only actionable on the human live turn.
    pub legal_choices: Vec<LegalChoice>,
    /// Claim opportunities; these are not concluded outcomes.
    pub claims: Vec<EngineClaim>,
    /// Complete played actions, in order.
    pub history: Vec<String>,
    /// Actual match conclusion, possibly explicitly unresolved/unscored.
    pub conclusion: Option<MatchConclusion>,
    /// Rules-owned check state.
    pub in_check: bool,
    /// Process status and final search observations.
    pub engine: EngineState,
    /// Complete desktop document state.
    pub document: DocumentState,
    /// Native archive admission, separate from chess and engine failures.
    pub capacity: CapacityState,
    /// Current bounded file/history operation.
    pub io: IoState,
    /// Read-only view, distinct from the live state above.
    pub view: ViewState,
    /// Accepted observations, historical after restoration.
    pub observations: Vec<ArchivedObservation>,
    /// Read-only analysis provenance and actual observations.
    pub analysis: AnalysisState,
    /// Analysis pauses play until explicit Resume; ended games stay ended.
    pub play_paused: bool,
    /// Explicit supported interactive adjudication identity.
    pub rules_mode: &'static str,
    /// Deterministic interactive work limits.
    pub limits: WorkLimits,
}

/// Pure serialized rules/session state; the controller supplies process results.
#[derive(Clone)]
pub struct DesktopSession {
    game: ChessMatch,
    human: Color,
    version: u64,
    revision: u64,
    archive_revision: u64,
    next_request: u64,
    play_limits: WorkLimits,
    pending: Option<EngineRequest>,
    engine: EngineState,
    analysis: AnalysisState,
    analysis_pending: Option<EngineRequest>,
    analysis_submitted: bool,
    play_paused: bool,
    document_revision: u64,
    document_identity: u64,
    saved_revision: u64,
    document_path: Option<PathBuf>,
    settings: DesktopSettings,
    observations: Vec<ArchivedObservation>,
    controls: Vec<persistence::ControlEntry>,
    initial_record: GameRecord,
    viewed: Option<persistence::Viewed>,
    selection_id: u64,
    capacity: CapacityState,
    validated_observations: usize,
}
impl Default for DesktopSession {
    fn default() -> Self {
        Self::new()
    }
}
impl DesktopSession {
    /// New untimed M2 game with optional claims and a human playing White.
    pub fn new() -> Self {
        Self {
            game: new_match(),
            human: Color::White,
            version: 1,
            revision: 1,
            archive_revision: 1,
            next_request: 0,
            play_limits: PLAY_LIMITS,
            pending: None,
            engine: EngineState::default(),
            analysis: AnalysisState::default(),
            analysis_pending: None,
            analysis_submitted: false,
            play_paused: false,
            document_revision: 0,
            document_identity: 1,
            saved_revision: 0,
            document_path: None,
            settings: DesktopSettings::default(),
            observations: Vec::new(),
            controls: Vec::new(),
            initial_record: AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record(),
            viewed: None,
            selection_id: 1,
            capacity: CapacityState::default(),
            validated_observations: 0,
        }
    }
    /// Restore supported live chess history without silently changing its mode.
    /// Persistence/UI file handling remains a later milestone deliverable.
    pub fn from_record(record: &GameRecord, human: Color) -> Result<Self, String> {
        if record.events().len() >= MAX_ARCHIVE_EVENTS {
            return Err("Application capacity: imported game history leaves no room for its live match start event.".into());
        }
        if record.adjudication_mode() != AdjudicationMode::M2 {
            return Err("Full adjudication restoration is unsupported by this untimed play slice; its mode and limits were not changed.".into());
        }
        let live = AdjudicatedGame::replay_record(record).map_err(|e| e.to_string())?;
        let mut session = Self::new();
        session.initial_record = record.clone();
        session.game = ChessMatch::new(live, TimeControl::Untimed).map_err(|e| e.to_string())?;
        if session.game.conclusion().is_some() {
            session.human = human;
            return Ok(session);
        }
        let reply = session.game.submit(MatchRequest {
            at_ns: 0,
            version: 0,
            actor: session.game.game().position().side_to_move(),
            command: MatchCommand::Start,
        });
        if let Some(rejection) = reply.rejection {
            return Err(rejection.detail);
        }
        session.human = human;
        Ok(session)
    }
    fn changed(&mut self) {
        // Checked before every external mutation; exhaustion never wraps.
        self.version += 1;
        self.revision += 1;
        self.document_revision += 1;
        self.archive_revision = self.revision.min(MAX_SAFE_ID - 65);
    }
    fn ensure_capacity(&self) -> Result<(), String> {
        if self.version >= MAX_SAFE_ID - 64
            || self.revision >= MAX_SAFE_ID - 64
            || self.next_request >= MAX_SAFE_ID - 64
            || self.document_revision >= MAX_SAFE_ID - 64
            || self.selection_id >= MAX_SAFE_ID - 64
        {
            Err("Application capacity: session identity space exhausted. Save the complete current document, then restart the application.".into())
        } else {
            Ok(())
        }
    }
    /// Apply an atomic human command. Rejection preserves the accepted document;
    /// resource refusals also pause execution and expose a capacity diagnostic.
    pub fn command(&mut self, command: DesktopCommand, at_ns: u64) -> Result<(), String> {
        let mut candidate = self.clone();
        if let Err(error) = candidate.command_unchecked(command, at_ns) {
            if error.starts_with("Application capacity:") {
                self.capacity_pause(error.clone());
            }
            return Err(error);
        }
        if let Err(error) = candidate.check_archive_capacity(&mut || false) {
            self.capacity_pause(error.clone());
            return Err(error);
        }
        candidate.validated_observations = candidate.observations.len();
        candidate.capacity = CapacityState::default();
        *self = candidate;
        Ok(())
    }
    fn command_unchecked(&mut self, command: DesktopCommand, at_ns: u64) -> Result<(), String> {
        if command
            .version()
            .is_some_and(|version| version != self.version)
        {
            return Err("The game changed. Use the current position and try again.".into());
        }
        self.ensure_capacity()?;
        if matches!(
            command,
            DesktopCommand::Analyze { .. }
                | DesktopCommand::Resume { .. }
                | DesktopCommand::Retry { .. }
        ) {
            self.observation_slot()?;
        }
        let recorded = command.clone();
        match command {
            DesktopCommand::NewGame {
                human,
                document_revision,
                ..
            } => {
                if document_revision != self.document_revision {
                    return Err(
                        "The document changed before New game. Confirm the current document again."
                            .into(),
                    );
                }
                self.initial_record = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record();
                self.observations.clear();
                self.validated_observations = 0;
                self.controls.clear();
                self.document_path = None;
                self.viewed = None;
                self.selection_id += 1;
                self.retire_analysis();
                self.play_paused = false;
                self.game = new_match();
                self.human = human;
                self.play_limits = PLAY_LIMITS;
                self.pending = None;
                self.engine = EngineState::default();
                self.changed();
                self.document_identity = self.version;
            }
            DesktopCommand::Preferences {
                orientation,
                analysis_defaults,
            } => {
                analysis_defaults.validate()?;
                self.settings = DesktopSettings {
                    orientation,
                    analysis_defaults,
                };
                self.document_revision += 1;
                self.revision += 1;
                self.archive_revision = self.revision.min(MAX_SAFE_ID - 65);
            }
            DesktopCommand::Inspect { .. } => {
                return Err("History selection requires the native controller worker.".into());
            }
            DesktopCommand::ReturnLive { .. } => {
                self.retire_analysis();
                self.pending = None;
                self.viewed = None;
                self.selection_id += 1;
                self.play_paused = true;
                self.engine.status = EnginePhase::Paused;
                self.version += 1;
                self.revision += 1;
            }
            DesktopCommand::Play { action, .. } => {
                self.human_turn()?;
                self.apply_match(self.human, MatchCommand::Move(action), at_ns)?;
                self.pending = None;
                self.engine.status = EnginePhase::Idle;
                self.engine.message = None;
                self.changed();
            }
            DesktopCommand::Claim { claim, .. } => {
                self.human_turn()?;
                if claim.actor != self.human {
                    return Err("The claim must belong to the player to move.".into());
                }
                self.apply_match(
                    claim.actor,
                    MatchCommand::Claim {
                        reason: claim.reason,
                        intended: claim.intended,
                    },
                    at_ns,
                )?;
                self.pending = None;
                self.engine.status = EnginePhase::Idle;
                self.engine.message = None;
                self.changed();
            }
            DesktopCommand::Analyze { operation, .. } => self.start_analysis(operation)?,
            DesktopCommand::CancelAnalysis { .. } => self.cancel_analysis()?,
            DesktopCommand::Resume { .. } => self.resume()?,
            DesktopCommand::Resign { .. } => {
                if self.game.conclusion().is_some() {
                    return Err("This game has concluded. Start a new game to play again.".into());
                }
                self.apply_match(self.human, MatchCommand::Resign { witness: None }, at_ns)?;
                self.retire_analysis();
                self.play_paused = false;
                self.pending = None;
                self.engine.status = EnginePhase::Idle;
                self.engine.message = None;
                self.changed();
            }
            DesktopCommand::Cancel { .. } => {
                if self.game.conclusion().is_some()
                    || self.pending.is_none()
                    || self.engine.status != EnginePhase::Thinking
                {
                    return Err("There is no active engine search to cancel.".into());
                }
                self.pending = None;
                self.engine.status = EnginePhase::Cancelled;
                self.engine.message =
                    Some("Engine work cancelled. No move or chess result was applied.".into());
                self.changed();
            }
            DesktopCommand::Retry { .. } => {
                if self.play_paused
                    || self.game.conclusion().is_some()
                    || self.game.game().position().side_to_move() == self.human
                    || self.pending.is_some()
                    || !matches!(
                        self.engine.status,
                        EnginePhase::Cancelled | EnginePhase::Failed | EnginePhase::Incomplete
                    )
                {
                    return Err("There is no paused engine turn to retry.".into());
                }
                self.play_limits = PLAY_LIMITS;
                self.pending = None;
                self.engine.status = EnginePhase::Idle;
                self.engine.message = None;
                self.changed();
            }
        }
        if !matches!(recorded, DesktopCommand::ReturnLive { .. }) {
            self.journal(recorded, at_ns);
        }
        Ok(())
    }
    fn human_turn(&self) -> Result<(), String> {
        if self.play_paused || self.viewed.is_some() {
            return Err(
                "Play is paused for analysis. Resume play explicitly before moving or claiming."
                    .into(),
            );
        }
        if self.game.conclusion().is_some() {
            return Err("This game has concluded. Start a new game to play again.".into());
        }
        if self.game.game().position().side_to_move() != self.human {
            return Err("It is mShogi's turn.".into());
        }
        Ok(())
    }
    fn apply_match(
        &mut self,
        actor: Color,
        command: MatchCommand,
        at_ns: u64,
    ) -> Result<(), String> {
        // ChessMatch logs rejected requests. A UI rejection is atomic, so commit
        // this private copy only after acceptance.
        let mut candidate = self.game.clone();
        let reply = candidate.submit(MatchRequest {
            at_ns,
            version: candidate.version(),
            actor,
            command,
        });
        if let Some(rejection) = reply.rejection {
            return Err(rejection.detail);
        }
        self.game = candidate;
        Ok(())
    }
    /// Create one immutable exact engine envelope if the live engine turn is ready.
    pub fn engine_request(&mut self) -> Option<EngineRequest> {
        if self.capacity.status != CapacityPhase::Available
            || self.play_paused
            || self.pending.is_some()
            || self.engine.status != EnginePhase::Idle
            || self.game.conclusion().is_some()
            || self.game.game().position().side_to_move() == self.human
        {
            return None;
        }
        if let Err(error) = self.ensure_capacity() {
            self.capacity_pause(error);
            return None;
        }
        if let Err(error) = self.observation_slot() {
            self.capacity_pause(error);
            return None;
        }
        self.next_request += 1;
        let request = EngineRequest {
            protocol: mshogi_engine::PROTOCOL_VERSION,
            purpose: mshogi_engine::RequestPurpose::EngineTurn,
            request_id: self.next_request,
            session_version: self.version,
            process_generation: self.next_request,
            game: self.game.game().record(),
            limits: self.play_limits,
        };
        #[cfg(feature = "desktop-verification")]
        if self.play_limits.nodes == 0 {
            // The verification budget affects this one recorded execution only;
            // the live document retains supported ordinary resume/save limits.
            self.play_limits = PLAY_LIMITS;
        }
        self.pending = Some(request.clone());
        self.engine.status = EnginePhase::Thinking;
        self.revision += 1;
        Some(request)
    }
    /// Accept and apply only the exact outstanding live context. Stale responses do nothing.
    pub fn accept(&mut self, response: EngineResponse, at_ns: u64) -> bool {
        if self.pending.as_ref() == Some(&response.context)
            && let Err(error) = self.ensure_capacity()
        {
            self.capacity_pause(error);
            return false;
        }
        let mut candidate = self.clone();
        if !candidate.accept_unchecked(response, at_ns) {
            return false;
        }
        if let Err(error) = candidate.check_archive_capacity(&mut || false) {
            self.capacity_pause(error);
            return false;
        }
        candidate.validated_observations = candidate.observations.len();
        *self = candidate;
        true
    }
    fn accept_unchecked(&mut self, response: EngineResponse, at_ns: u64) -> bool {
        if self.play_paused
            || response.context.purpose != mshogi_engine::RequestPurpose::EngineTurn
            || self.pending.as_ref() != Some(&response.context)
            || response.context.session_version != self.version
            || response.context.game != self.game.game().record()
            || self.game.conclusion().is_some()
            || self.game.game().position().side_to_move() == self.human
        {
            return false;
        }
        if let Err(error) = self.ensure_capacity() {
            self.pending = None;
            self.fail(error);
            return false;
        }
        self.remember(response.clone());
        self.pending = None;
        self.engine.last_turn = Some(self.game.game().position().side_to_move());
        self.engine.searched_version = Some(self.version);
        self.engine.last = Some(response.result.clone());
        let applied = match response.result {
            EngineResult::Move { action, .. } => {
                self.apply_match(self.human.opponent(), MatchCommand::Move(action), at_ns)
            }
            EngineResult::Claim { claim, .. } => {
                if claim.actor != self.human.opponent() {
                    Err("Engine returned a claim for the wrong player.".into())
                } else {
                    self.apply_match(
                        claim.actor,
                        MatchCommand::Claim {
                            reason: claim.reason,
                            intended: claim.intended,
                        },
                        at_ns,
                    )
                }
            }
            EngineResult::Finished { outcome, .. } => {
                if self.game.game().outcome() != Some(outcome) {
                    Err("Engine reported a conclusion absent from the authoritative game.".into())
                } else {
                    Ok(())
                }
            }
            EngineResult::Reachability { .. } => {
                Err("Engine turn returned analysis-only reachability observations.".into())
            }
            EngineResult::Incomplete { .. } => {
                self.engine.status = EnginePhase::Incomplete;
                self.engine.message = Some("Search ended before a complete iteration. No move was applied. Retry or start a new game.".into());
                self.changed();
                return true;
            }
            EngineResult::Failure { failure } => {
                self.fail(format!(
                    "Engine failure ({:?}): {}",
                    failure.kind, failure.message
                ));
                return true;
            }
        };
        match applied {
            Ok(()) => {
                self.engine.status = EnginePhase::Idle;
                self.engine.message = None;
                self.changed();
            }
            Err(error) => {
                if let Some(observation) = self.observations.last_mut() {
                    observation.application_error = Some(error.clone());
                }
                self.fail(format!("Invalid engine choice: {error}"));
            }
        }
        true
    }
    /// Pause for an application/process failure without assigning a chess score.
    pub fn fail(&mut self, message: String) {
        self.pending = None;
        self.engine.status = EnginePhase::Failed;
        self.engine.message = Some(message);
        if self.ensure_capacity().is_ok() {
            self.changed();
        }
    }
    /// Build a rules-owned immutable view. This never starts search or changes the game.
    pub fn snapshot(&self) -> Snapshot {
        let position = self.game.game().position();
        let mut pieces = Vec::new();
        for rank in 0..8 {
            for file in 0..8 {
                let square = Square::new(file, rank).expect("bounded coordinates");
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
        let legal_choices = if self.game.conclusion().is_some() {
            Vec::new()
        } else {
            position
                .legal_actions()
                .into_iter()
                .map(|action| LegalChoice {
                    from: action.from.to_string(),
                    to: action.to.to_string(),
                    promotion: action.promotion.map(|p| format!("{p:?}").to_lowercase()),
                    action,
                })
                .collect()
        };
        Snapshot {
            revision: self.revision,
            version: self.version,
            human: self.human,
            turn: position.side_to_move(),
            fen: position.to_fen(),
            view: self.view_state(&pieces),
            pieces,
            legal_choices,
            claims: self
                .game
                .game()
                .available_claims()
                .into_iter()
                .map(EngineClaim::from)
                .collect(),
            history: self
                .game
                .game()
                .history()
                .actions()
                .map(|a| a.to_string())
                .collect(),
            conclusion: self.game.conclusion(),
            in_check: position.in_check(),
            engine: self.engine.clone(),
            document: self.document_state(),
            capacity: self.capacity.clone(),
            io: IoState::default(),
            observations: self.observations.clone(),
            analysis: self.analysis.clone(),
            play_paused: self.play_paused,
            rules_mode: "m2",
            limits: self.play_limits,
        }
    }
}
fn new_match() -> ChessMatch {
    let mut game = ChessMatch::new(
        AdjudicatedGame::new(ClaimPolicy::ExplicitOnly),
        TimeControl::Untimed,
    )
    .expect("valid untimed match");
    let reply = game.submit(MatchRequest {
        at_ns: 0,
        version: 0,
        actor: Color::White,
        command: MatchCommand::Start,
    });
    assert!(reply.rejection.is_none());
    game
}

/// Tauri-owned serialized controller. Process work runs on the bridge worker.
pub struct DesktopController {
    shared: shared::SharedState,
    session: DesktopSession,
    bridge: EngineBridge,
    origin: Instant,
    clock_floor: u64,
    files: persistence::FileWorker,
    io_state: IoState,
    admissions: persistence::FileWorker,
    admission_token: Option<OpenToken>,
    admission_receipt: Option<AdmissionReceipt>,
    last_command_receipt: Option<AdmissionReceipt>,
}
impl DesktopController {
    /// Launch the supervisor worker, without starting a child or doing pipe I/O.
    pub fn new(engine_path: PathBuf) -> io::Result<Self> {
        Ok(Self {
            shared: shared::SharedState::default(),
            session: DesktopSession::new(),
            bridge: EngineBridge::new(engine_path)?,
            origin: Instant::now(),
            clock_floor: 0,
            files: persistence::FileWorker::new()?,
            io_state: IoState::default(),
            admissions: persistence::FileWorker::new()?,
            admission_token: None,
            admission_receipt: None,
            last_command_receipt: None,
        })
    }
    /// Construct difficult native sessions for a separately labeled QA package.
    /// This API and its fixture names do not exist in normal distributables.
    /// Incomplete work is produced by the actual child with a zero-node budget;
    /// explicit retry restores ordinary play limits and launches fresh work.
    #[cfg(feature = "desktop-verification")]
    pub fn verification(engine_path: PathBuf, fixture: &str) -> Result<Self, String> {
        let mut controller = Self::new(engine_path).map_err(|error| error.to_string())?;
        controller.session = match fixture {
            "standard" | "shared-contract" => DesktopSession::new(),
            fixture if fixture.starts_with("shogi-") => DesktopSession::new(),
            "capacity" => DesktopSession::verification_capacity()?,
            "promotion" => {
                let game = AdjudicatedGame::from_fen("7k/P7/8/8/8/8/8/7K w - - 0 1", ClaimPolicy::ExplicitOnly).map_err(|error| error.to_string())?;
                DesktopSession::from_record(&game.record(), Color::White)?
            }
            "claim" | "current-claim" => {
                let fen = if fixture == "claim" { "7k/8/8/8/8/8/R7/K7 w - - 99 1" } else { "7k/8/8/8/8/8/R7/K7 w - - 100 1" };
                let game = AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).map_err(|error| error.to_string())?;
                DesktopSession::from_record(&game.record(), Color::White)?
            }
            "concluded" => {
                let game = AdjudicatedGame::from_fen("7k/6Q1/5K2/8/8/8/8/8 b - - 0 1", ClaimPolicy::ExplicitOnly).map_err(|error| error.to_string())?;
                DesktopSession::from_record(&game.record(), Color::Black)?
            }
            "incomplete" => {
                let game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
                let mut session = DesktopSession::from_record(&game.record(), Color::Black)?;
                session.play_limits.nodes = 0;
                session
            }
            _ => return Err("Unknown desktop verification fixture. Use standard, promotion, claim, current-claim, concluded, incomplete or capacity.".into()),
        };
        if fixture.starts_with("shogi-") {
            controller.verification_shogi(fixture)?;
        }
        if fixture == "shared-contract" {
            use mshogi_core::{
                native::{CanonicalRecord, GameKind, NativeSide},
                shogi,
            };
            let (_, receipt) = controller.shared_command_with_receipt(SharedDesktopCommand {
                expected: controller.shared_token(),
                game: GameKind::Chess,
                operation: SessionOperation::Replace {
                    record: CanonicalRecord::shogi(shogi::GameRecord::from_game(
                        &shogi::Game::new(),
                    )),
                    human: NativeSide::Shogi(shogi::Side::Sente),
                },
            })?;
            while receipt.completion().is_none() {
                controller.shared_snapshot();
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            receipt.completion().unwrap()?;
        }
        Ok(controller)
    }
    fn at_ns(&self) -> u64 {
        self.clock_floor
            .saturating_add(u64::try_from(self.origin.elapsed().as_nanos()).unwrap_or(u64::MAX))
    }
    /// Queue an explicit candidate before polling completions; cancellation wins races.
    /// Use `command_with_receipt` when the caller needs its accepted/rejected outcome.
    pub fn command(&mut self, command: DesktopCommand) -> Result<Snapshot, String> {
        self.require_chess()?;
        if matches!(
            command,
            DesktopCommand::Analyze { .. }
                | DesktopCommand::Resume { .. }
                | DesktopCommand::ReturnLive { .. }
        ) && matches!(self.io_state.phase, IoPhase::Viewing | IoPhase::Opening)
        {
            return Err("Wait for the current document/history operation before changing its analysis target.".into());
        }
        if let DesktopCommand::Inspect {
            version,
            event_index,
        } = command
        {
            return self.queue_view(version, event_index);
        }
        if self.admission_token.is_some()
            && !matches!(
                command,
                DesktopCommand::Cancel { .. }
                    | DesktopCommand::CancelAnalysis { .. }
                    | DesktopCommand::NewGame { .. }
                    | DesktopCommand::ReturnLive { .. }
            )
        {
            return Err("Wait for native document capacity admission to complete.".into());
        }
        if self.admission_token.is_some()
            && matches!(command, DesktopCommand::CancelAnalysis { .. })
            && self.session.analysis_pending.is_none()
        {
            if command.version() != Some(self.session.version) {
                return Err("The game changed. Use the current position and try again.".into());
            }
            self.retire_admission();
            self.bridge.cancel();
            self.session.pending = None;
            self.session.play_paused = true;
            self.session.engine.status = EnginePhase::Paused;
            self.session.analysis.status = AnalysisPhase::Cancelled;
            self.session.analysis.message = Some("Pending analysis admission cancelled before any command or observation was accepted.".into());
            return Ok(self.session.snapshot());
        }
        let preferences = matches!(command, DesktopCommand::Preferences { .. });
        let at_ns = self.at_ns();
        let mut candidate = self.session.clone();
        if let Err(error) = candidate.command_unchecked(command, at_ns) {
            if error.starts_with("Application capacity:") {
                self.retire_admission();
                self.bridge.cancel();
                self.session.capacity_pause(error.clone());
            }
            return Err(error);
        }
        self.queue_admission(candidate)?;
        self.last_command_receipt = self.admission_receipt.clone();
        if !preferences {
            self.bridge.cancel();
        }
        Ok(self.snapshot())
    }
    /// Poll the bounded completion slot, accept it atomically, and start eligible work.
    pub fn snapshot(&mut self) -> Snapshot {
        if self.shared.admission.is_some() {
            let mut snapshot = self.session.snapshot();
            snapshot.io = self.io_state.clone();
            return snapshot;
        }
        if self.shared.xiangqi.is_some() {
            let mut snapshot = self.session.snapshot();
            snapshot.io = self.io_state.clone();
            return snapshot;
        }
        if self.shared.shogi.is_some() {
            self.poll_files();
            if self.shared.shogi.is_none() {
                return self.snapshot();
            }
            // Legacy read adapter cannot poll or start hidden chess work.
            let mut snapshot = self.session.snapshot();
            snapshot.document = self.session.document_state();
            snapshot.io = self.io_state.clone();
            return snapshot;
        }
        self.poll_admission();
        self.poll_files();
        // Open can replace Chess with Shogi while polling. Return its read
        // adapter before polling or starting any now-retired Chess execution.
        if self.shared.shogi.is_some() {
            return self.snapshot();
        }
        if self.admission_token.is_none()
            && (self.session.pending.is_some() || self.session.analysis_pending.is_some())
            && let Err(error) = self.session.ensure_capacity()
        {
            self.bridge.cancel();
            self.session.capacity_pause(error);
        }
        if self.admission_token.is_none()
            && let Some(response) = self.bridge.poll()
        {
            let at_ns = self.at_ns();
            if matches!(
                response.context.purpose,
                mshogi_engine::RequestPurpose::EngineTurn
            ) {
                let mut candidate = self.session.clone();
                if candidate.accept_unchecked(response, at_ns) {
                    let _ = self.queue_admission(candidate);
                }
            } else {
                let mut candidate = self.session.clone();
                if candidate.accept_analysis_unchecked(response) {
                    let _ = self.queue_admission(candidate);
                }
            }
        }
        if self.admission_token.is_none()
            && let Some(request) = self.session.analysis_request()
            && let Err(error) = self.bridge.submit(request.clone())
        {
            let mut candidate = self.session.clone();
            if candidate.accept_analysis_unchecked(EngineResponse {
                context: request,
                result: EngineResult::Failure {
                    failure: mshogi_engine::EngineFailure {
                        kind: mshogi_engine::FailureKind::Start,
                        message: format!("Cannot start analysis work: {error:?}"),
                    },
                },
            }) {
                let _ = self.queue_admission(candidate);
            }
        }
        if self.admission_token.is_none()
            && let Some(request) = self.session.engine_request()
            && let Err(error) = self.bridge.submit(request)
        {
            self.session
                .fail(format!("Cannot start engine work: {error:?}"));
        }
        let mut snapshot = self.session.snapshot();
        snapshot.io = self.io_state.clone();
        snapshot
    }
    /// Signal process cleanup before a GUI runtime exits without dropping state.
    /// This does not join or wait on the interaction path.
    pub fn begin_shutdown(&mut self) {
        self.bridge.shutdown();
        self.files.shutdown();
        self.admissions.shutdown();
    }
    /// Observe off-path supervisor cleanup before allowing application exit.
    pub fn shutdown_acknowledged(&self) -> bool {
        self.admissions
            .shared
            .exited
            .load(std::sync::atomic::Ordering::Acquire)
            && self.bridge.shutdown_acknowledged()
            && self
                .files
                .shared
                .exited
                .load(std::sync::atomic::Ordering::Acquire)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mshogi_engine::{StopReason, Telemetry};
    fn telemetry() -> Telemetry {
        Telemetry {
            score: Some(0),
            completed_depth: 1,
            nodes: 21,
            principal_variation: vec![],
            status: StopReason::Complete,
        }
    }
    fn response(context: EngineRequest, action: &str) -> EngineResponse {
        EngineResponse {
            context,
            result: EngineResult::Move {
                action: action.parse().unwrap(),
                telemetry: Telemetry {
                    principal_variation: vec![mshogi_engine::EngineChoice::Move {
                        action: action.parse().unwrap(),
                    }],
                    ..telemetry()
                },
            },
        }
    }
    fn human_black() -> DesktopSession {
        let mut s = DesktopSession::new();
        s.command(
            DesktopCommand::NewGame {
                document_revision: s.document_revision,
                version: 1,
                human: Color::Black,
            },
            0,
        )
        .unwrap();
        s
    }
    #[test]
    fn native_snapshot_and_atomic_human_rejection() {
        let mut s = DesktopSession::new();
        assert_eq!(s.snapshot().pieces.len(), 32);
        assert_eq!(s.snapshot().legal_choices.len(), 20);
        let before = s.game.record().to_json();
        assert!(
            s.command(
                DesktopCommand::Play {
                    version: 1,
                    action: "e2e5".parse().unwrap()
                },
                1
            )
            .is_err()
        );
        assert_eq!(s.game.record().to_json(), before);
        assert_eq!(s.version, 1);
        s.command(
            DesktopCommand::Play {
                version: 1,
                action: "e2e4".parse().unwrap(),
            },
            1,
        )
        .unwrap();
        assert_eq!(s.snapshot().history, ["e2e4"]);
        assert!(
            s.command(
                DesktopCommand::Play {
                    version: 1,
                    action: "d2d4".parse().unwrap()
                },
                2
            )
            .is_err()
        );
        assert_eq!(s.snapshot().history, ["e2e4"]);
    }
    #[test]
    fn engine_applies_once_and_keeps_actual_observations() {
        let mut s = human_black();
        let request = s.engine_request().unwrap();
        let result = response(request, "e2e4");
        assert!(s.accept(result.clone(), 1));
        assert!(!s.accept(result, 2));
        assert_eq!(s.snapshot().history, ["e2e4"]);
        assert_eq!(s.engine.last_turn, Some(Color::White));
        assert_eq!(s.engine.searched_version, Some(2));
        assert!(s.engine_request().is_none());
    }
    #[test]
    fn new_game_aba_and_generation_changes_reject_old_results() {
        let mut s = human_black();
        let old = s.engine_request().unwrap();
        let mut wrong_generation = old.clone();
        wrong_generation.process_generation += 1;
        assert!(!s.accept(response(wrong_generation, "e2e4"), 1));
        s.command(
            DesktopCommand::NewGame {
                document_revision: s.document_revision,
                version: s.version,
                human: Color::Black,
            },
            2,
        )
        .unwrap();
        let current = s.engine_request().unwrap();
        assert_eq!(current.game, old.game);
        assert_ne!(current.session_version, old.session_version);
        assert!(current.process_generation > old.process_generation);
        assert!(!s.accept(response(old, "e2e4"), 3));
        assert!(s.accept(response(current, "d2d4"), 4));
        assert_eq!(s.snapshot().history, ["d2d4"]);
    }
    #[test]
    fn exact_same_fen_with_different_known_history_cannot_accept() {
        let mut live = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        for action in ["g1f3", "g8f6", "f3g1", "f6g8"] {
            live.apply(action.parse().unwrap()).unwrap();
        }
        let fen = live.position().to_fen();
        let imported = AdjudicatedGame::from_fen(&fen, ClaimPolicy::ExplicitOnly).unwrap();
        assert_eq!(imported.position(), live.position());
        let mut s = DesktopSession::from_record(&live.record(), Color::Black).unwrap();
        let request = s.engine_request().unwrap();
        let mut incorrect = request.clone();
        incorrect.game = imported.record();
        assert_ne!(incorrect.game, request.game);
        assert!(!s.accept(response(incorrect, "e2e4"), 1));
        assert_eq!(s.snapshot().history.len(), 4);
        assert!(s.accept(response(request, "e2e4"), 2));
    }
    #[test]
    fn cancel_wins_even_if_a_complete_response_has_arrived() {
        let mut s = human_black();
        let old = s.engine_request().unwrap();
        s.command(DesktopCommand::Cancel { version: s.version }, 1)
            .unwrap();
        assert!(!s.accept(response(old.clone(), "e2e4"), 2));
        assert!(s.snapshot().history.is_empty());
        assert!(s.engine_request().is_none());
        s.command(DesktopCommand::Retry { version: s.version }, 3)
            .unwrap();
        let fresh = s.engine_request().unwrap();
        assert!(fresh.request_id > old.request_id);
        assert!(s.accept(response(fresh, "e2e4"), 4));
    }
    #[test]
    fn invalid_engine_choice_is_unscored_and_atomic() {
        let mut s = human_black();
        let request = s.engine_request().unwrap();
        let before = s.game.record().to_json();
        assert!(s.accept(response(request, "e2e5"), 1));
        assert_eq!(s.game.record().to_json(), before);
        assert_eq!(s.engine.status, EnginePhase::Failed);
        assert!(s.snapshot().conclusion.is_none());
    }
    #[test]
    fn incomplete_search_never_falls_back_to_a_move() {
        let mut s = human_black();
        let context = s.engine_request().unwrap();
        let mut observation = telemetry();
        observation.score = None;
        observation.completed_depth = 0;
        observation.nodes = 0;
        observation.status = StopReason::NodeLimit;
        assert!(s.accept(
            EngineResponse {
                context,
                result: EngineResult::Incomplete {
                    telemetry: observation
                }
            },
            1
        ));
        assert_eq!(s.engine.status, EnginePhase::Incomplete);
        assert!(s.snapshot().history.is_empty());
        assert!(s.snapshot().conclusion.is_none());
    }
    #[test]
    fn intended_claim_is_an_opportunity_then_conclusion_without_a_move() {
        let live =
            AdjudicatedGame::from_fen("7k/8/8/8/8/8/R7/K7 w - - 99 1", ClaimPolicy::ExplicitOnly)
                .unwrap();
        let mut s = DesktopSession::from_record(&live.record(), Color::White).unwrap();
        let claim = s
            .snapshot()
            .claims
            .into_iter()
            .find(|c| c.reason == DrawClaim::FiftyMove && c.intended.is_some())
            .unwrap();
        assert!(s.snapshot().conclusion.is_none());
        s.command(
            DesktopCommand::Claim {
                version: s.version,
                claim,
            },
            1,
        )
        .unwrap();
        assert_eq!(
            s.snapshot().conclusion,
            Some(MatchConclusion::Chess(ChessOutcome::Draw(
                DrawReason::Claimed(DrawClaim::FiftyMove)
            )))
        );
        assert!(s.snapshot().history.is_empty());
        assert!(s.snapshot().legal_choices.is_empty());
    }
    #[test]
    fn engine_claim_remains_a_claim() {
        let live =
            AdjudicatedGame::from_fen("7k/8/8/8/8/8/R7/K7 w - - 100 1", ClaimPolicy::ExplicitOnly)
                .unwrap();
        let mut s = DesktopSession::from_record(&live.record(), Color::Black).unwrap();
        let claim = s
            .snapshot()
            .claims
            .into_iter()
            .find(|c| c.intended.is_none())
            .unwrap();
        let context = s.engine_request().unwrap();
        assert!(s.accept(
            EngineResponse {
                context,
                result: EngineResult::Claim {
                    claim,
                    telemetry: Telemetry {
                        principal_variation: vec![mshogi_engine::EngineChoice::Claim { claim }],
                        ..telemetry()
                    }
                }
            },
            1
        ));
        assert!(s.snapshot().history.is_empty());
        assert!(s.snapshot().conclusion.is_some());
    }
    #[test]
    fn promotion_choices_and_automatic_checkmate_are_native() {
        let live =
            AdjudicatedGame::from_fen("7k/P7/8/8/8/8/8/7K w - - 0 1", ClaimPolicy::ExplicitOnly)
                .unwrap();
        let s = DesktopSession::from_record(&live.record(), Color::White).unwrap();
        assert_eq!(
            s.snapshot()
                .legal_choices
                .iter()
                .filter(|c| c.from == "a7")
                .count(),
            4
        );
        let mut live = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        for action in ["f2f3", "e7e5", "g2g4", "d8h4"] {
            live.apply(action.parse().unwrap()).unwrap();
        }
        let s = DesktopSession::from_record(&live.record(), Color::White).unwrap();
        assert_eq!(
            s.snapshot().conclusion,
            Some(MatchConclusion::Chess(ChessOutcome::Checkmate {
                winner: Color::Black
            }))
        );
    }
    #[test]
    fn resignation_is_unresolved_with_no_proof_on_either_players_turn() {
        for engine_turn in [false, true] {
            let mut session = DesktopSession::new();
            let old = if engine_turn {
                session
                    .command(
                        DesktopCommand::Play {
                            version: session.version,
                            action: "e2e4".parse().unwrap(),
                        },
                        1,
                    )
                    .unwrap();
                session.engine_request()
            } else {
                None
            };
            let before_game = session.game.game().record().to_json();
            let before_history = session.snapshot().history;
            session
                .command(
                    DesktopCommand::Resign {
                        version: session.version,
                    },
                    2,
                )
                .unwrap();
            let conclusion = Some(MatchConclusion::Unresolved {
                actor: Color::White,
                trigger: TerminationTrigger::Resignation,
            });
            assert_eq!(session.snapshot().conclusion, conclusion);
            assert_eq!(
                session.snapshot().conclusion.unwrap().white_score_twice(),
                None
            );
            assert_eq!(session.snapshot().history, before_history);
            assert_eq!(session.game.game().record().to_json(), before_game);
            assert_eq!(
                session.game.record().replay().unwrap().conclusion(),
                conclusion
            );
            assert!(session.snapshot().legal_choices.is_empty());
            assert!(session.engine_request().is_none());
            if let Some(old) = old {
                assert!(!session.accept(response(old, "a7a6"), 3));
            }
            let record = session.game.record().to_json();
            for command in [
                DesktopCommand::Resign {
                    version: session.version,
                },
                DesktopCommand::Cancel {
                    version: session.version,
                },
                DesktopCommand::Retry {
                    version: session.version,
                },
            ] {
                assert!(session.command(command, 4).is_err());
                assert_eq!(session.game.record().to_json(), record);
            }
        }
    }
    #[test]
    fn resignation_is_a_draw_only_when_opponents_mate_is_proven_impossible() {
        let game =
            AdjudicatedGame::from_fen("7k/8/8/8/8/8/R7/K7 w - - 0 1", ClaimPolicy::ExplicitOnly)
                .unwrap();
        let mut session = DesktopSession::from_record(&game.record(), Color::White).unwrap();
        assert!(session.snapshot().conclusion.is_none());
        session
            .command(
                DesktopCommand::Resign {
                    version: session.version,
                },
                1,
            )
            .unwrap();
        assert_eq!(
            session.snapshot().conclusion,
            Some(MatchConclusion::Draw {
                trigger: Some(TerminationTrigger::Resignation)
            })
        );
        assert_eq!(
            session.snapshot().conclusion.unwrap().white_score_twice(),
            Some(1)
        );
        assert!(session.snapshot().history.is_empty());
        assert_eq!(
            session.game.record().replay().unwrap().conclusion(),
            session.snapshot().conclusion
        );
    }
    #[test]
    fn recovery_preserves_exact_match_and_game_records_and_retires_old_results() {
        for failure in [false, true] {
            let mut game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
            for action in ["g1f3", "g8f6"] {
                game.apply(action.parse().unwrap()).unwrap();
            }
            let mut session = DesktopSession::from_record(&game.record(), Color::Black).unwrap();
            let old = session.engine_request().unwrap();
            let before_game = session.game.game().record().to_json();
            let before_match = session.game.record().to_json();
            let result = if failure {
                EngineResult::Failure {
                    failure: mshogi_engine::EngineFailure {
                        kind: mshogi_engine::FailureKind::Search,
                        message: "test native search failure".into(),
                    },
                }
            } else {
                EngineResult::Incomplete {
                    telemetry: Telemetry {
                        score: None,
                        completed_depth: 0,
                        nodes: 0,
                        principal_variation: vec![],
                        status: StopReason::Stopped,
                    },
                }
            };
            assert!(session.accept(
                EngineResponse {
                    context: old.clone(),
                    result
                },
                1
            ));
            assert_eq!(
                session.engine.status,
                if failure {
                    EnginePhase::Failed
                } else {
                    EnginePhase::Incomplete
                }
            );
            assert!(session.snapshot().conclusion.is_none());
            assert_eq!(session.game.record().to_json(), before_match);
            session
                .command(
                    DesktopCommand::Retry {
                        version: session.version,
                    },
                    2,
                )
                .unwrap();
            let fresh = session.engine_request().unwrap();
            assert!(fresh.request_id > old.request_id);
            assert!(fresh.process_generation > old.process_generation);
            assert_eq!(fresh.game, old.game);
            assert_eq!(session.game.game().record().to_json(), before_game);
            assert_eq!(session.game.record().to_json(), before_match);
            assert!(!session.accept(response(old, "e2e4"), 3));
            session
                .command(
                    DesktopCommand::Cancel {
                        version: session.version,
                    },
                    4,
                )
                .unwrap();
            assert_eq!(session.game.record().to_json(), before_match);
            assert!(!session.accept(response(fresh, "e2e4"), 5));
            session
                .command(
                    DesktopCommand::Retry {
                        version: session.version,
                    },
                    6,
                )
                .unwrap();
            let resumed = session.engine_request().unwrap();
            assert!(session.accept(response(resumed, "e2e4"), 7));
            assert_eq!(session.snapshot().history, ["g1f3", "g8f6", "e2e4"]);
        }
    }
    #[test]
    fn cancellation_and_retry_reject_invalid_states_atomically() {
        let mut session = DesktopSession::new();
        let before = session.game.record().to_json();
        let version = session.version;
        assert!(
            session
                .command(DesktopCommand::Cancel { version }, 1)
                .is_err()
        );
        assert!(
            session
                .command(DesktopCommand::Retry { version }, 1)
                .is_err()
        );
        assert_eq!(session.version, version);
        assert_eq!(session.game.record().to_json(), before);
        session
            .command(
                DesktopCommand::Play {
                    version,
                    action: "e2e4".parse().unwrap(),
                },
                2,
            )
            .unwrap();
        let request = session.engine_request().unwrap();
        let version = session.version;
        assert!(
            session
                .command(DesktopCommand::Retry { version }, 3)
                .is_err()
        );
        assert_eq!(session.pending, Some(request));
        assert_eq!(session.version, version);
    }
    #[cfg(feature = "desktop-verification")]
    #[test]
    fn verification_states_are_native_and_zero_work_retry_restores_play_limits() {
        let path = PathBuf::from("/unused/verification-engine");
        let claim = DesktopController::verification(path.clone(), "claim").unwrap();
        assert!(
            claim
                .session
                .snapshot()
                .claims
                .iter()
                .any(|claim| claim.intended.is_some())
        );
        let current_claim = DesktopController::verification(path.clone(), "current-claim").unwrap();
        assert!(
            current_claim
                .session
                .snapshot()
                .claims
                .iter()
                .any(|claim| claim.intended.is_none())
        );
        let concluded = DesktopController::verification(path.clone(), "concluded").unwrap();
        assert_eq!(
            concluded.session.snapshot().conclusion,
            Some(MatchConclusion::Chess(ChessOutcome::Checkmate {
                winner: Color::White
            }))
        );
        let mut incomplete = DesktopController::verification(path.clone(), "incomplete").unwrap();
        let request = incomplete.session.engine_request().unwrap();
        assert_eq!(request.limits.nodes, 0);
        assert_eq!(incomplete.session.snapshot().limits, PLAY_LIMITS);
        incomplete
            .session
            .command(
                DesktopCommand::Cancel {
                    version: incomplete.session.version,
                },
                1,
            )
            .unwrap();
        incomplete
            .session
            .command(
                DesktopCommand::Retry {
                    version: incomplete.session.version,
                },
                2,
            )
            .unwrap();
        assert_eq!(
            incomplete.session.engine_request().unwrap().limits,
            PLAY_LIMITS
        );
        assert!(DesktopController::verification(path, "unknown").is_err());
    }
    #[cfg(feature = "desktop-verification")]
    #[test]
    fn verification_promotion_has_four_native_choices_and_roundtrips_complete_archive() {
        fn wait_document(controller: &mut DesktopController, id: u64) -> Snapshot {
            let deadline = Instant::now() + std::time::Duration::from_secs(5);
            loop {
                let snapshot = controller.snapshot();
                if snapshot.io.id == id
                    && matches!(snapshot.io.phase, IoPhase::Complete | IoPhase::Failed)
                {
                    assert_eq!(snapshot.io.phase, IoPhase::Complete, "{:?}", snapshot.io);
                    return snapshot;
                }
                assert!(Instant::now() < deadline);
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        }
        let engine = PathBuf::from("/unused/verification-engine");
        let mut source = DesktopController::verification(engine.clone(), "promotion").unwrap();
        let snapshot = source.snapshot();
        assert_eq!(snapshot.human, Color::White);
        assert_eq!(snapshot.fen, "7k/P7/8/8/8/8/8/7K w - - 0 1");
        assert!(snapshot.conclusion.is_none());
        let choices: std::collections::BTreeSet<_> = snapshot
            .legal_choices
            .iter()
            .filter(|choice| choice.from == "a7" && choice.to == "a8")
            .map(|choice| {
                (
                    choice.action.to_string(),
                    choice.promotion.as_deref().unwrap(),
                )
            })
            .collect();
        assert_eq!(
            choices,
            [
                ("a7a8b".into(), "bishop"),
                ("a7a8n".into(), "knight"),
                ("a7a8q".into(), "queen"),
                ("a7a8r".into(), "rook"),
            ]
            .into_iter()
            .collect()
        );
        let match_record = source.session.game.record().to_json();
        let game_record = source.session.game.game().record();
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "mshogi-promotion-{}-{unique}.mshogi",
            std::process::id()
        ));
        let id = source.queue_save(path.clone()).unwrap();
        assert!(!wait_document(&mut source, id).document.dirty);
        let mut restored = DesktopController::new(engine).unwrap();
        let id = restored
            .queue_open(path.clone(), restored.open_token())
            .unwrap();
        let reopened = wait_document(&mut restored, id);
        assert_eq!(reopened.fen, snapshot.fen);
        assert_eq!(reopened.human, Color::White);
        assert!(reopened.play_paused);
        assert!(reopened.version > snapshot.version);
        assert_eq!(restored.session.game.record().to_json(), match_record);
        assert_eq!(restored.session.game.game().record(), game_record);
        assert!(reopened.observations.is_empty());
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn full_mode_restoration_is_explicitly_rejected_before_solver_execution() {
        let live = AdjudicatedGame::from_fen_with_mode(
            "7k/8/8/8/8/8/R7/K7 w - - 0 1",
            ClaimPolicy::ExplicitOnly,
            AdjudicationMode::Full {
                limits: ReachabilityLimits {
                    max_positions: Some(0),
                    max_transitions: Some(0),
                },
            },
        )
        .unwrap();
        let error = DesktopSession::from_record(&live.record(), Color::White)
            .err()
            .unwrap();
        assert!(error.contains("unsupported"));
        assert!(error.contains("mode and limits were not changed"));
    }
}
