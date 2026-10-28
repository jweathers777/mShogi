//! Read-only native assessment resource ownership, separate from engine execution.
use super::{ExactLimits, ExactReport, Game, GameContext, GameError, GameRecord, Position};
use std::sync::atomic::{AtomicU64, Ordering};
static NEXT_ANALYSIS: AtomicU64 = AtomicU64::new(1);
/// Exact historical event selection; viewing never branches the actual game.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExactTarget {
    /// Current complete live context.
    Live,
    /// Immutable event prefix; future moves never enter the target board.
    History {
        /// Number of included native events, not board ply.
        event_count: usize,
    },
}
/// Actual independently bounded operation purpose; not ordinary move analysis.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExactPurpose {
    /// Intrinsic existential natural-board reachability, never an engine turn.
    NaturalBoardWin,
}
/// Immutable accepted native request. Construction requires the native adapter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactRequest {
    context: GameContext,
    record: GameRecord,
    position: Position,
    target: ExactTarget,
    limits: ExactLimits,
    session: u64,
    generation: u64,
}
impl ExactRequest {
    /// This operation cannot enter a move-application result handler.
    pub const fn purpose(&self) -> ExactPurpose {
        ExactPurpose::NaturalBoardWin
    }
    /// Exact submitted bounded configuration.
    pub const fn limits(&self) -> ExactLimits {
        self.limits
    }
    /// Session identity plus never-rewound execution generation/request identity.
    /// This in-process core operation creates no engine process generation.
    pub const fn identity(&self) -> (u64, u64) {
        (self.session, self.generation)
    }
    /// Historical/live selection at submission.
    pub const fn target(&self) -> ExactTarget {
        self.target
    }
    /// Complete native source record, retaining full history and root provenance.
    pub fn source_record(&self) -> &GameRecord {
        &self.record
    }
    /// Exact target board and declared counters; computation normalizes only its private copy.
    pub fn position(&self) -> &Position {
        &self.position
    }
    /// Separately bounded read-only work. Cancellation must be signalled by the
    /// native host; retiring a request independently prevents late acceptance.
    pub fn run(&self, cancelled: impl FnMut() -> bool) -> ExactObservation {
        ExactObservation {
            request: self.clone(),
            report: self.position.verify_natural_win(self.limits, cancelled),
        }
    }
}
/// Historical observation bound to all submitted context; never pending work or a game command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactObservation {
    request: ExactRequest,
    report: ExactReport,
}
impl ExactObservation {
    /// Complete identity/provenance of this immutable result.
    pub fn request(&self) -> &ExactRequest {
        &self.request
    }
    /// Actual proof, witness, unknown or execution failure.
    pub fn report(&self) -> &ExactReport {
        &self.report
    }
}
/// One native exact-analysis resource. Selection/replacement/cancel consumes its
/// generation; game changes and undo are additionally guarded by GameContext.
/// Does not launch a process, advertise engine capabilities or mutate Game.
#[derive(Debug)]
pub struct ExactAnalysis {
    session: u64,
    generation: u64,
    target: ExactTarget,
    active: Option<ExactRequest>,
}
impl ExactAnalysis {
    /// Create fresh runtime execution identity; saved observations never resume work.
    pub fn new() -> Result<Self, GameError> {
        let session = NEXT_ANALYSIS
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |v| v.checked_add(1))
            .map_err(|_| GameError::Capacity)?;
        Ok(Self {
            session,
            generation: 0,
            target: ExactTarget::Live,
            active: None,
        })
    }
    /// Retire pending work on cancellation or replacement. No generation rewinds.
    pub fn retire(&mut self) -> Result<(), GameError> {
        // Invalidate even if a representation ceiling prevents further requests.
        self.active = None;
        self.generation = self.generation.checked_add(1).ok_or(GameError::Capacity)?;
        Ok(())
    }
    /// Native host must route every historical selection through this operation,
    /// including returning to an equal/live board, so retired results cannot revive.
    pub fn select(&mut self, target: ExactTarget) -> Result<(), GameError> {
        self.retire()?;
        self.target = target;
        Ok(())
    }
    /// Bind exact target plus the complete known game. Reconstruction is not done
    /// here: target snapshots and the native record are already bounded values.
    /// Canonical record restoration retains its existing strict fast-only mode;
    /// a saved record cannot silently start this advanced operation.
    pub fn begin(&mut self, game: &Game, limits: ExactLimits) -> Result<ExactRequest, GameError> {
        self.retire()?;
        let position = match self.target {
            ExactTarget::Live => game.position().clone(),
            ExactTarget::History { event_count } => game.event_prefix(event_count)?.position,
        };
        let record = GameRecord::from_game(game).map_err(|_| GameError::Capacity)?;
        let request = ExactRequest {
            context: game.context(),
            record,
            position,
            target: self.target,
            limits,
            session: self.session,
            generation: self.generation,
        };
        self.active = Some(request.clone());
        Ok(request)
    }
    /// Accept a result only once for precisely the submitted target/history/work.
    /// Returning a report does not install a game outcome. A caller requiring this
    /// assessment for adjudication must leave Unknown unscored and unresolved.
    pub fn accept(
        &mut self,
        game: &Game,
        observation: &ExactObservation,
    ) -> Result<ExactReport, GameError> {
        if self.active.as_ref() != Some(&observation.request)
            || observation.request.context != game.context()
        {
            return Err(GameError::StaleContext);
        }
        self.retire()?;
        Ok(observation.report.clone())
    }
}
