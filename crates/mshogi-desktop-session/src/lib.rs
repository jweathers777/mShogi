//! Authoritative untimed desktop sessions. Framework-free commands and snapshots.
#[cfg(feature = "desktop-verification")]
mod synthetic;
#[cfg(feature = "desktop-verification")]
pub use synthetic::*;
mod participants;
pub use participants::PlayMode;
mod shared;
pub use shared::*;
mod moves;
pub use moves::{
    CapturedPiece, CompletedActionGeometry, MoveEffect, MovesProjection, MovesRoot,
    NativeCompletedEffects, PresentedMove, TenjikuEffectPhase, TenjikuEffectPiece,
};
mod capacity;
pub use capacity::*;
mod chess_observation;
/// Minimal accepted-command Chess records for desktop persistence.
pub mod chess_record;
/// Immutable Chu candidates for the same document envelope and file worker.
pub mod chu_document;
mod document;
/// Generic native document of the games behind `native_game::NativeGame`.
pub mod game_document;
mod native_chess;
/// Per-game boundary of the generic document and session.
pub mod native_game;
pub(crate) use native_game::{Chess, Chu, Shogi, Tenjiku, Tori, Xiangqi};
mod resource;
/// Immutable two-profile Tenjiku candidates for the existing document and worker.
pub mod tenjiku_document;
/// Tori document: the generic native document for Tori.
pub mod tori_document;
pub use resource::*;
mod lifecycle;
pub use lifecycle::*;
mod persistence;
pub use persistence::*;
mod analysis;
mod takeback;
pub use analysis::*;
use mshogi_core::chess::*;
use mshogi_core::native::CanonicalRecord;
use mshogi_engine::{
    ChessResult, EngineBridge, EngineClaim, ExecutionContext, ExecutionObservation,
    ExecutionResult, WorkLimits,
};
#[cfg(test)]
use mshogi_engine::{ExecutionRequest, ExecutionResponse};
use serde::{Deserialize, Serialize};
use std::{io, path::PathBuf, time::Instant};

// A Chess presentation retains native Chess meanings; this is a domain
// projection, never a second execution or compatibility transport.
fn chess_observation_result(observation: &ExecutionObservation) -> Option<ChessResult> {
    match &observation.result {
        ExecutionResult::Chess { result } => Some(result.clone()),
        ExecutionResult::Failure { failure } => Some(ChessResult::Failure {
            failure: failure.clone(),
        }),
        _ => None,
    }
}
const MAX_SAFE_ID: u64 = 9_007_199_254_740_991;
/// Representation guard, not an ordinary search-effort budget. This is the
/// largest integer preserved exactly by the JSON/JavaScript desktop boundary;
/// no supported timed search can practically exhaust it.
pub const PLAY_NODE_CEILING: u64 = MAX_SAFE_ID;
/// Time-primary Chess play. Depth128 remains the native recursion safeguard;
/// both technical ceilings are recorded in the exact request identity.
pub const PLAY_LIMITS: WorkLimits = WorkLimits {
    time_ms: Some(1_000),
    depth: 128,
    nodes: PLAY_NODE_CEILING,
};
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
/// Pure serialized rules/session state; the controller supplies process results.
/// Monotonic runtime identity and document lifecycle metadata. Native game state is
/// held only by the selected `ActiveDocument` variant.
#[derive(Clone)]
pub struct RuntimeMetadata {
    document_epoch: u64,
    version: u64,
    revision: u64,
    next_request: u64,
    document_revision: u64,
    document_identity: u64,
    saved_revision: u64,
    document_path: Option<PathBuf>,
    selection_id: u64,
    capacity: CapacityState,
}
/// Runtime stop meaning retained across read-only browsing; never an execution request.
#[derive(Clone)]
struct BrowseStop {
    paused: bool,
    phase: EnginePhase,
    diagnostic: Option<String>,
}
/// Exactly one authoritative native game/document is installed at a time.
pub(crate) enum ActiveDocument {
    Chess {
        runtime: RuntimeMetadata,
        state: Box<shared::chess::ChessSession>,
    },
    /// Native Shogi game journal and observations.
    Shogi {
        runtime: RuntimeMetadata,
        state: Box<shared::shogi::ShogiSession>,
    },
    /// Authoritative Tori game and admitted immutable document.
    Tori {
        runtime: RuntimeMetadata,
        state: Box<shared::tori::ToriSession>,
    },
    /// Native Chu game and immutable admitted current document.
    Chu {
        runtime: RuntimeMetadata,
        state: Box<shared::chu::ChuSession>,
    },
    /// Authoritative two-profile Tenjiku document, in the same runtime.
    Tenjiku {
        runtime: RuntimeMetadata,
        state: Box<shared::tenjiku::TenjikuSession>,
    },
    /// Native Xiangqi WXF game journal and observations.
    Xiangqi {
        runtime: RuntimeMetadata,
        state: Box<shared::xiangqi::XiangqiSession>,
    },
}
impl std::ops::Deref for ActiveDocument {
    type Target = RuntimeMetadata;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Chess { runtime, .. }
            | Self::Shogi { runtime, .. }
            | Self::Xiangqi { runtime, .. }
            | Self::Tori { runtime, .. }
            | Self::Chu { runtime, .. }
            | Self::Tenjiku { runtime, .. } => runtime,
        }
    }
}
impl std::ops::DerefMut for ActiveDocument {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Chess { runtime, .. }
            | Self::Shogi { runtime, .. }
            | Self::Xiangqi { runtime, .. }
            | Self::Tori { runtime, .. }
            | Self::Chu { runtime, .. }
            | Self::Tenjiku { runtime, .. } => runtime,
        }
    }
}
impl ActiveDocument {
    fn chess(&self) -> Option<&shared::chess::ChessSession> {
        match self {
            Self::Chess { state, .. } => Some(state),
            _ => None,
        }
    }
    fn chess_mut(&mut self) -> Option<&mut shared::chess::ChessSession> {
        match self {
            Self::Chess { state, .. } => Some(state),
            _ => None,
        }
    }

    fn shogi(&self) -> Option<&shared::shogi::ShogiSession> {
        match self {
            Self::Shogi { state, .. } => Some(state),
            _ => None,
        }
    }
    fn shogi_mut(&mut self) -> Option<&mut shared::shogi::ShogiSession> {
        match self {
            Self::Shogi { state, .. } => Some(state),
            _ => None,
        }
    }
    fn xiangqi(&self) -> Option<&shared::xiangqi::XiangqiSession> {
        match self {
            Self::Xiangqi { state, .. } => Some(state),
            _ => None,
        }
    }
    fn xiangqi_mut(&mut self) -> Option<&mut shared::xiangqi::XiangqiSession> {
        match self {
            Self::Xiangqi { state, .. } => Some(state),
            _ => None,
        }
    }
    fn chu(&self) -> Option<&shared::chu::ChuSession> {
        if let Self::Chu { state, .. } = self {
            Some(state)
        } else {
            None
        }
    }
    fn chu_mut(&mut self) -> Option<&mut shared::chu::ChuSession> {
        if let Self::Chu { state, .. } = self {
            Some(state)
        } else {
            None
        }
    }
    fn tenjiku(&self) -> Option<&shared::tenjiku::TenjikuSession> {
        if let Self::Tenjiku { state, .. } = self {
            Some(state)
        } else {
            None
        }
    }
    fn tenjiku_mut(&mut self) -> Option<&mut shared::tenjiku::TenjikuSession> {
        if let Self::Tenjiku { state, .. } = self {
            Some(state)
        } else {
            None
        }
    }
    fn profile(&self) -> mshogi_core::native::RulesProfile {
        use mshogi_core::native::RulesProfile;
        match self {
            Self::Chess { .. } => RulesProfile::ChessMatchV1,
            Self::Shogi { .. } => RulesProfile::ShogiCsa36V1,
            Self::Xiangqi { .. } => RulesProfile::XiangqiWxf2018V1,
            Self::Tori { .. } => RulesProfile::ToriModernV1,
            Self::Chu { .. } => RulesProfile::ChuJcsa2019V1,
            Self::Tenjiku { state, .. } => state.document.profile(),
        }
    }
    fn tori(&self) -> Option<&shared::tori::ToriSession> {
        match self {
            Self::Tori { state, .. } => Some(state),
            _ => None,
        }
    }
    fn tori_mut(&mut self) -> Option<&mut shared::tori::ToriSession> {
        match self {
            Self::Tori { state, .. } => Some(state),
            _ => None,
        }
    }
    fn selected_game(&self) -> &AdjudicatedGame {
        self.chess().expect("Chess document").selected().game()
    }
    fn clear_browse_intent(&mut self) {
        match self {
            Self::Chess { state, .. } => {
                state.browse_stop = None;
            }
            Self::Shogi { state, .. } => {
                state.browse_stop = None;
            }
            Self::Tori { state, .. } => {
                state.browse_stop = None;
            }
            Self::Chu { state, .. } => {
                state.browse_stop = None;
            }
            Self::Tenjiku { state, .. } => {
                state.browse_stop = None;
            }
            Self::Xiangqi { state, .. } => {
                state.browse_stop = None;
            }
        }
    }
    fn retire_execution(&mut self) {
        self.clear_browse_intent();
        match self {
            Self::Chess { state, .. } => {
                state.retire();
                state.paused = true;
            }
            Self::Shogi { state, .. } => {
                state.retire();
                state.paused = true;
            }
            Self::Tori { state, .. } => {
                state.retire();
                state.paused = true;
            }
            Self::Chu { state, .. } => {
                state.retire();
                state.paused = true;
            }
            Self::Tenjiku { state, .. } => {
                state.retire();
                state.paused = true;
            }
            Self::Xiangqi { state, .. } => {
                state.retire();
                state.paused = true;
            }
        }
    }

    fn ensure_capacity(&self) -> Result<(), OperationFailure> {
        std::ops::Deref::deref(self).ensure_capacity()
    }
    /// Refuse completed work of a game on the generic session.
    fn refuse_completed_work(&mut self, resource: bool) {
        match self {
            Self::Shogi { state, .. } => state.refuse_completed_work(resource),
            Self::Tori { state, .. } => state.refuse_completed_work(resource),
            Self::Chu { state, .. } => state.refuse_completed_work(resource),
            Self::Tenjiku { state, .. } => state.refuse_completed_work(resource),
            Self::Xiangqi { state, .. } => state.refuse_completed_work(resource),
            Self::Chess { state, .. } => state.refuse_completed_work(resource),
        }
    }
}

impl Default for RuntimeMetadata {
    fn default() -> Self {
        RuntimeMetadata {
            document_epoch: 0,
            version: 1,
            revision: 1,
            next_request: 0,
            document_revision: 0,
            document_identity: 1,
            saved_revision: 0,
            document_path: None,
            selection_id: 1,
            capacity: CapacityState::default(),
        }
    }
}
impl RuntimeMetadata {
    fn ensure_capacity(&self) -> Result<(), OperationFailure> {
        if self.version >= MAX_SAFE_ID - 64
            || self.revision >= MAX_SAFE_ID - 64
            || self.next_request >= MAX_SAFE_ID - 64
            || self.document_revision >= MAX_SAFE_ID - 64
            || self.selection_id >= MAX_SAFE_ID - 64
        {
            Err(resource_refusal(
                vec![resource_fact(
                    ResourceCategory::IdentityHeadroom,
                    self.version
                        .max(self.revision)
                        .max(self.next_request)
                        .max(self.document_revision)
                        .max(self.selection_id) as usize,
                    MAX_SAFE_ID as usize,
                    64,
                    ResourceScope::Application,
                    ResourceUnit::Identities,
                )],
                "Application capacity: session identity headroom exhausted. Save the complete current document then restart.",
            ))
        } else {
            Ok(())
        }
    }
}

/// Authoritative native application runtime. The host serializes short operations;
/// native mutation, replay and admission run on bounded workers.
pub struct DesktopController {
    shared: shared::SharedState,
    session: ActiveDocument,
    lifecycle: std::sync::Arc<ApplicationLifecycle>,
    bridge: EngineBridge,
    files: persistence::FileWorker,
    io_state: IoState,
    admissions: persistence::FileWorker,
    admission_token: Option<AdmissionToken>,
    admission_receipt: Option<AdmissionReceipt>,
    last_command_receipt: Option<AdmissionReceipt>,
}
impl DesktopController {
    fn close_requires_save(&self) -> bool {
        let document = self.shared_document();
        self.lifecycle.close_requires_save(
            self.shared_open_token(),
            document.dirty || document.path.is_none(),
        )
    }
    /// Native application lifecycle authority, shared with host reservation guards.
    pub fn lifecycle(&self) -> std::sync::Arc<ApplicationLifecycle> {
        self.lifecycle.clone()
    }
    /// Launch the supervisor worker, without starting a child or doing pipe I/O.
    pub fn new(engine_path: PathBuf) -> io::Result<Self> {
        Ok(Self {
            shared: shared::SharedState::default(),
            lifecycle: std::sync::Arc::new(ApplicationLifecycle::default()),
            session: {
                let document = game_document::ValidatedGameDocument::<Chess>::new(
                    chess_record::ChessRecord::new(
                        &AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record(),
                    ),
                    Color::White,
                )
                .expect("standard Chess document");
                let mut state = shared::chess::ChessSession::restore(document, &mut || false)
                    .expect("standard Chess game");
                state.paused = false;
                state.phase = EnginePhase::Idle;
                ActiveDocument::Chess {
                    runtime: RuntimeMetadata::default(),
                    state: Box::new(state),
                }
            },
            bridge: EngineBridge::new(engine_path)?,
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
        controller.verification_chess(fixture)?;
        if fixture.starts_with("tenjiku-") {
            #[path = "../examples/support/tenjiku.rs"]
            mod roots;
            fn send(
                controller: &mut DesktopController,
                operation: SessionOperation,
            ) -> Result<(), String> {
                let game = controller.shared_snapshot().game;
                let (_, receipt) =
                    controller.shared_command_with_receipt(SharedDesktopCommand {
                        expected: controller.shared_token(),
                        game,
                        operation,
                    })?;
                let deadline = Instant::now() + std::time::Duration::from_secs(20);
                loop {
                    controller.shared_snapshot();
                    if let Some(done) = receipt.completion() {
                        return done;
                    }
                    if Instant::now() >= deadline {
                        return Err("Tenjiku verification admission did not complete".into());
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
            }
            let (profile, scenario) = roots::selected(fixture)?;
            let game = roots::game(profile, scenario)?;
            send(
                &mut controller,
                SessionOperation::Replace {
                    record: CanonicalRecord::tenjiku(
                        mshogi_core::tenjiku::GameRecord::from_game(&game)
                            .map_err(|e| e.to_string())?,
                    ),
                    human: NativeSide::Tenjiku(if matches!(scenario, "gote" | "engine-played") {
                        mshogi_core::tenjiku::Side::Gote
                    } else {
                        mshogi_core::tenjiku::Side::Sente
                    }),
                },
            )?;
            if matches!(
                scenario,
                "burned"
                    | "iguied"
                    | "promoted"
                    | "declined"
                    | "compound-played"
                    | "selected-root"
                    | "unsupported"
                    | "hawk-played"
                    | "area-played"
                    | "ranked-jump-played"
            ) {
                send(
                    &mut controller,
                    SessionOperation::Tenjiku {
                        actor: mshogi_core::tenjiku::Side::Sente,
                        command: mshogi_core::tenjiku::GameCommand::Play {
                            action: roots::action(scenario)?,
                        },
                    },
                )?;
            }
            if matches!(scenario, "selected-root" | "unsupported") {
                send(
                    &mut controller,
                    SessionOperation::Inspect { event_index: 0 },
                )?;
            }
            if scenario == "ending-stopped" {
                send(
                    &mut controller,
                    SessionOperation::Tenjiku {
                        actor: mshogi_core::tenjiku::Side::Sente,
                        command: mshogi_core::tenjiku::GameCommand::Resign,
                    },
                )?;
                send(&mut controller, SessionOperation::Cancel {})?;
            }
            if scenario == "unsupported" {
                let before = controller.shared_snapshot().observations.len();
                controller.verify_tenjiku_execution(
                    WorkLimits {
                        time_ms: None,
                        depth: 1,
                        nodes: 1,
                    },
                    true,
                )?;
                let deadline = Instant::now() + std::time::Duration::from_secs(20);
                while controller.shared_snapshot().observations.len() == before {
                    if Instant::now() >= deadline {
                        return Err("Tenjiku unsupported observation did not complete".into());
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
            }
            if matches!(scenario, "engine-played" | "analysis") {
                let before = controller.shared_snapshot().observations.len();
                controller.verify_tenjiku_execution(
                    WorkLimits {
                        time_ms: None,
                        depth: 1,
                        nodes: 1024,
                    },
                    scenario == "analysis",
                )?;
                let deadline = Instant::now() + std::time::Duration::from_secs(20);
                while controller.shared_snapshot().observations.len() == before {
                    if Instant::now() >= deadline {
                        return Err("Tenjiku real engine checkpoint did not complete".into());
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                let snapshot = controller.shared_snapshot();
                let result = &snapshot
                    .observations
                    .last()
                    .ok_or("Missing real engine observation")?
                    .observation
                    .result;
                if !matches!(result,mshogi_engine::ExecutionResult::Tenjiku {result:mshogi_engine::TenjikuResult::Choice{telemetry,..}} if telemetry.completed_depth==1 && telemetry.score.is_some())
                {
                    return Err(format!(
                        "Tenjiku checkpoint requires completed native engine work: {result:?}"
                    ));
                }
                let expected_ply = usize::from(scenario == "engine-played");
                if snapshot.board_ply != expected_ply as u64
                    || (scenario == "engine-played" && snapshot.turn.id != snapshot.human.id)
                {
                    return Err(
                        "Tenjiku checkpoint action/analysis state disagrees with native work"
                            .into(),
                    );
                }
                if scenario == "analysis"
                    && snapshot.context
                        != CanonicalRecord::tenjiku(
                            mshogi_core::tenjiku::GameRecord::from_game(
                                &mshogi_core::tenjiku::Game::standard(profile),
                            )
                            .map_err(|e| e.to_string())?,
                        )
                {
                    return Err("Read-only Tenjiku checkpoint changed the native game".into());
                }
            }
        }
        if fixture.starts_with("chu-") {
            let game = match fixture {
                "chu-contract" => chu_fixture_roots::contract_game(),
                "chu-standard" | "chu-gote" => mshogi_core::chu::Game::standard(),
                "chu-captures" | "chu-captured" | "chu-promotion" => {
                    let f = chu_fixture_roots::fixture(if fixture == "chu-promotion" {
                        "CHU-PROMOTE-DE-YES"
                    } else {
                        "CHU-LION-C2-1"
                    });
                    let mut g = chu_fixture_roots::root(&f["given"]["root"]);
                    if fixture == "chu-captured" {
                        g.submit(mshogi_core::chu::GameRequest {
                            expected: g.context(),
                            actor: g.position().side_to_move(),
                            command: mshogi_core::chu::GameCommand::Play {
                                action: chu_fixture_roots::action(&f["given"]["commands"][0]),
                            },
                        })
                        .map_err(|e| e.to_string())?;
                    }
                    g
                }
                "chu-promoted" | "chu-royals" => {
                    use mshogi_core::chu as c;
                    let pieces: Vec<_> = c::Position::standard()
                        .pieces()
                        .map(|(at, mut p)| {
                            if p.side == c::Side::Sente
                                && p.base.promotable()
                                && (fixture == "chu-promoted" || p.base == c::Base::Elephant)
                            {
                                p.promoted = true;
                            }
                            (at, p)
                        })
                        .collect();
                    c::Game::from_root(
                        c::Position::from_diagram(
                            &pieces,
                            c::Side::Sente,
                            c::RootMetadata::imported("Chu artwork verification diagram".into(), 0)
                                .map_err(|e| e.to_string())?,
                        )
                        .map_err(|e| e.to_string())?,
                    )
                    .map_err(|e| e.to_string())?
                }
                _ => return Err("Unknown Chu verification fixture".into()),
            };
            let (_, receipt) = controller.shared_command_with_receipt(SharedDesktopCommand {
                expected: controller.shared_token(),
                game: GameKind::Chess,
                operation: SessionOperation::Replace {
                    record: CanonicalRecord::chu(
                        mshogi_core::chu::GameRecord::from_game(&game)
                            .map_err(|e| e.to_string())?,
                    ),
                    human: NativeSide::Chu(if fixture == "chu-gote" {
                        mshogi_core::chu::Side::Gote
                    } else {
                        mshogi_core::chu::Side::Sente
                    }),
                },
            })?;
            while receipt.completion().is_none() {
                controller.shared_snapshot();
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            receipt.completion().unwrap()?;
        }
        if fixture.starts_with("tori-") {
            let game = match fixture {
                "tori-contract" => tori_fixture_roots::contract_game(),
                "tori-standard" => mshogi_core::tori::Game::standard(),
                _ => return Err("Unknown Tori verification fixture".into()),
            };
            let (_, receipt) = controller.shared_command_with_receipt(SharedDesktopCommand {
                expected: controller.shared_token(),
                game: GameKind::Chess,
                operation: SessionOperation::Replace {
                    record: CanonicalRecord::tori(
                        mshogi_core::tori::GameRecord::from_game(&game)
                            .map_err(|e| e.to_string())?,
                    ),
                    human: NativeSide::Tori(mshogi_core::tori::Side::Sente),
                },
            })?;
            while receipt.completion().is_none() {
                controller.shared_snapshot();
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            receipt.completion().unwrap()?;
        }
        if fixture.starts_with("shogi-") {
            controller.verification_shogi(fixture)?;
        }
        if fixture.starts_with("xiangqi-") {
            controller.verification_xiangqi(fixture)?;
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
    /// Stop workers and the engine supervisor before application exit.
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
fn test_request(context: ExecutionContext) -> ExecutionRequest {
    ExecutionRequest {
        protocol: mshogi_engine::PROTOCOL_VERSION,
        context,
    }
}
#[cfg(test)]
mod runtime_tests;

#[cfg(any(test, feature = "desktop-verification"))]
#[path = "../examples/support/tori.rs"]
mod tori_fixture_roots;

#[cfg(any(test, feature = "desktop-verification"))]
#[path = "../examples/support/chu.rs"]
mod chu_fixture_roots;

#[cfg(test)]
fn test_single_profile(game: GameKind) -> RulesProfile {
    match game {
        GameKind::Chess => RulesProfile::ChessMatchV1,
        GameKind::Shogi => RulesProfile::ShogiCsa36V1,
        GameKind::Xiangqi => RulesProfile::XiangqiWxf2018V1,
        GameKind::Tori => RulesProfile::ToriModernV1,
        GameKind::Chu => RulesProfile::ChuJcsa2019V1,
        GameKind::Tenjiku => panic!("Tenjiku tests must supply an explicit whole profile"),
    }
}

#[cfg(test)]
impl DesktopController {
    fn test_chess_operation(
        &mut self,
        operation: SessionOperation,
    ) -> Result<(SharedSnapshot, AdmissionReceipt), String> {
        self.shared_command_with_receipt(SharedDesktopCommand {
            expected: self.shared_token(),
            game: GameKind::Chess,
            operation,
        })
    }
    fn test_chess_record(&mut self, record: GameRecord, human: Color) {
        let (_, receipt) = self
            .shared_command_with_receipt(SharedDesktopCommand {
                expected: self.shared_token(),
                game: self.session.profile().game(),
                operation: SessionOperation::Replace {
                    record: CanonicalRecord::chess(record),
                    human: NativeSide::Chess(human),
                },
            })
            .unwrap();
        let deadline = Instant::now() + std::time::Duration::from_secs(10);
        while receipt.completion().is_none() {
            self.poll_native_admission();
            assert!(Instant::now() < deadline);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        receipt.completion().unwrap().unwrap();
    }
}
