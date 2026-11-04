//! One authoritative Tenjiku history owner. Local geometry is not live permission.
use super::*;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

/// Native event bound; native acceptance additionally reserves real ending events.
pub const MAX_GAME_EVENTS: usize = 1024;
/// One native event is retained per accepted command.
pub const MAX_GAME_COMMANDS: usize = MAX_GAME_EVENTS;
/// Deterministic native rule/key/encoded-byte units, not elapsed milliseconds.
pub const MAX_REPLAY_WORK: u64 = 512_000_000;
static NEXT_INSTANCE: AtomicU64 = AtomicU64::new(1);

/// Native conclusions are separate from search, cancellation and operational failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
#[allow(missing_docs)]
pub enum Conclusion {
    RoyalCapture { winner: Side },
    SimultaneousRoyalLoss,
    BareRoyalLoss { winner: Side },
    BothBareRoyals,
    Resignation { winner: Side },
    Agreement,
}
impl Conclusion {
    /// A drawn conclusion has no winner.
    pub const fn winner(self) -> Option<Side> {
        match self {
            Self::RoyalCapture { winner }
            | Self::BareRoyalLoss { winner }
            | Self::Resignation { winner } => Some(winner),
            _ => None,
        }
    }
}
/// Administrative interruption is not a game outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum StopReason {
    Cancelled,
    ResourceLimit,
    EngineFailure,
    WorkLimit,
}
/// Complete native participant/administrative commands; no rule switches exist.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
#[allow(missing_docs)]
pub enum GameCommand {
    Play { action: Action },
    Resign,
    OfferDraw,
    AcceptDraw,
    DeclineDraw,
    Stop { reason: StopReason },
}
impl<'de> Deserialize<'de> for GameCommand {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
        enum Wire {
            Play { action: Action },
            Resign {},
            OfferDraw {},
            AcceptDraw {},
            DeclineDraw {},
            Stop { reason: StopReason },
        }
        Ok(match Wire::deserialize(d)? {
            Wire::Play { action } => Self::Play { action },
            Wire::Resign {} => Self::Resign,
            Wire::OfferDraw {} => Self::OfferDraw,
            Wire::AcceptDraw {} => Self::AcceptDraw,
            Wire::DeclineDraw {} => Self::DeclineDraw,
            Wire::Stop { reason } => Self::Stop { reason },
        })
    }
}
/// Resource refusal leaves the accepted complete record intact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum Resource {
    EncodedBytes,
    Events,
    ReplayWork,
    Counter,
}
/// Typed atomic native refusal.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum GameError {
    Root(RootError),
    NotRoot,
    StaleContext,
    WrongActor,
    Action(ActionError),
    Repetition,
    Concluded,
    DrawOffer,
    InvalidToken,
    Prefix,
    Capacity(Resource),
    Stopped,
    Record,
}
impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tenjiku game refused: {self:?}")
    }
}
impl Error for GameError {}
impl From<ActionError> for GameError {
    fn from(e: ActionError) -> Self {
        if e == ActionError::Stopped {
            Self::Stopped
        } else if e == ActionError::Capacity {
            Self::Capacity(Resource::Counter)
        } else {
            Self::Action(e)
        }
    }
}
/// Immutable event facts are always re-derived on canonical replay.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(missing_docs)]
pub struct GameEvent {
    pub index: usize,
    pub board_ply: usize,
    pub actor: Side,
    pub command: GameCommand,
    pub effects: Option<Effects>,
    pub conclusion: Option<Conclusion>,
    pub pending_bare: Option<Side>,
    pub draw_offer: Option<Side>,
    pub incomplete: Option<StopReason>,
}
/// A complete immutable committed native prefix; it owns no execution authority.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct HistorySnapshot {
    state: StructuralState,
    pub event_count: usize,
    pub board_ply: usize,
    pub conclusion: Option<Conclusion>,
    pub pending_bare: Option<Side>,
    pub draw_offer: Option<Side>,
    pub incomplete: Option<StopReason>,
    work: u64,
}
impl HistorySnapshot {
    /// Exact logical board, original identities, promotion rights and provenance.
    pub fn state(&self) -> &StructuralState {
        &self.state
    }
    /// Read-only local queries on this prefix, without future game events.
    pub fn local_position(&self) -> LocalPosition {
        LocalPosition::admit(Position::install(self.state.clone()).expect("admitted snapshot"))
            .expect("admitted snapshot")
    }
}
/// Board-and-turn repetition excludes original serials and promotion-refusal rights.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepetitionKey {
    profile: Profile,
    side: Side,
    board: Vec<Option<(Side, Base, bool)>>,
}
impl RepetitionKey {
    /// The selected profile is part of identity even when boards match.
    pub fn from_position(p: &Position) -> Self {
        Self {
            profile: p.profile(),
            side: p.side_to_move(),
            board: p
                .state()
                .board()
                .iter()
                .map(|p| p.map(|p| (p.side, p.base, p.promoted)))
                .collect(),
        }
    }
    fn compared(&self, other: &Self) -> (bool, u64) {
        let mut work = 1;
        if self.profile != other.profile || self.side != other.side {
            return (false, work);
        }
        for (a, b) in self.board.iter().zip(&other.board) {
            work += 1;
            if a != b {
                return (false, work);
            }
        }
        (true, work)
    }
}
/// Fresh runtime authority, distinct from saved/repetition/search semantics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeToken {
    instance: u64,
    revision: u64,
}
/// Exact immutable native context; later consumers add request/purpose/process limits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameContext {
    root: StructuralState,
    events: Vec<GameEvent>,
    runtime: RuntimeToken,
}
impl GameContext {
    /// Full saved semantic equality, deliberately excluding retired runtime authority.
    pub fn same_semantics(&self, other: &Self) -> bool {
        self.root == other.root && self.events == other.events
    }
    /// Explicit whole-profile identity.
    pub fn profile(&self) -> Profile {
        self.root.profile()
    }
    /// Captured runtime identity for diagnostics/equality.
    pub fn runtime(&self) -> RuntimeToken {
        self.runtime
    }
}
/// Context-checked complete request.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct GameRequest {
    pub expected: GameContext,
    pub actor: Side,
    pub command: GameCommand,
}
/// Single-use compact LIFO handle; snapshots are retained once by the game, not per undo.
#[derive(Debug)]
pub struct GameUndo {
    instance: u64,
    before_node: u64,
    after_node: u64,
    before_len: usize,
    used: bool,
}
/// Single-use redo, retired by a new native command branch.
#[derive(Debug)]
pub struct GameRedo {
    instance: u64,
    branch: u64,
    before: GameContext,
    actor: Side,
    command: GameCommand,
    used: bool,
}
/// Actual current native capacity facts plus prevalidated ending headroom.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct CapacityUsage {
    pub encoded_bytes: usize,
    pub events: usize,
    pub replay_work: u64,
    pub reserved_bytes: usize,
    pub reserved_events: usize,
    pub reserved_work: u64,
}
/// Unscored no-action is not an invented mate, forfeit or draw.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum GameStatus {
    Ongoing,
    BareReply { side: Side },
    NoBoardActions,
    Concluded(Conclusion),
}
/// Sole native history/adjudication owner; profile can never be changed in place.
#[derive(Debug)]
pub struct Game {
    root: StructuralState,
    local: LocalPosition,
    events: Vec<GameEvent>,
    keys: Vec<RepetitionKey>,
    snapshots: Vec<Arc<HistorySnapshot>>,
    root_snapshot: Arc<HistorySnapshot>,
    runtime: RuntimeToken,
    node: u64,
    next_node: u64,
    branch: u64,
    pub(super) events_encoded_bytes: usize,
}
impl Game {
    #[cfg(test)]
    pub(super) fn set_test_authority(&mut self, revision: u64, next_node: u64, branch: u64) {
        self.runtime.revision = revision;
        self.next_node = next_node;
        self.branch = branch;
    }
    /// Admit a declared known-history root; no earlier history is invented.
    pub fn from_root(root: Position) -> Result<Self, GameError> {
        Self::from_root_with_stop(root, &mut || false)
    }
    /// Root admission with cancellation before finite native bare-response queries.
    pub fn from_root_with_stop(
        root: Position,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self, GameError> {
        check(stop)?;
        if root.accepted_plies() != 0 {
            return Err(GameError::NotRoot);
        }
        if root.root().kind() == RootKind::Standard
            && root.state() != Position::standard(root.profile()).state()
        {
            return Err(GameError::Root(RootError::InvalidProvenance));
        }
        let local = LocalPosition::admit(root).map_err(GameError::Root)?;
        let root = local.position().state().clone();
        let key = RepetitionKey::from_position(local.position());
        let instance = NEXT_INSTANCE
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
            .map_err(|_| GameError::Capacity(Resource::Counter))?;
        let mut snapshot = HistorySnapshot {
            state: root.clone(),
            event_count: 0,
            board_ply: 0,
            conclusion: None,
            pending_bare: None,
            draw_offer: None,
            incomplete: None,
            work: 256,
        };
        let (conclusion, pending, work) = adjudicate(&local, std::slice::from_ref(&key), stop)?;
        snapshot.conclusion = conclusion;
        snapshot.pending_bare = pending;
        snapshot.work += work;
        let mut game = Self {
            root,
            local,
            events: vec![],
            keys: vec![key],
            snapshots: vec![],
            root_snapshot: Arc::new(snapshot),
            runtime: RuntimeToken {
                instance,
                revision: 0,
            },
            node: 0,
            next_node: 1,
            branch: 0,
            events_encoded_bytes: 0,
        };
        let bytes = record::encoded_size(&game).map_err(|_| GameError::Record)?;
        Arc::make_mut(&mut game.root_snapshot).work += bytes as u64;
        game.ensure_capacity(stop)?;
        Ok(game)
    }
    /// Exact standard setup under an explicitly selected whole profile.
    pub fn standard(profile: Profile) -> Self {
        Self::from_root(Position::standard(profile)).expect("standard native root")
    }
    /// Immutable actual local owner, not game/history permission.
    pub fn local_position(&self) -> &LocalPosition {
        &self.local
    }
    /// Current accepted position.
    pub fn position(&self) -> &Position {
        self.local.position()
    }
    /// Immutable selected whole profile.
    pub fn profile(&self) -> Profile {
        self.position().profile()
    }
    /// Original admitted structural root and declared provenance.
    pub fn native_root(&self) -> &StructuralState {
        &self.root
    }
    /// Initial adjudication at root, without future information.
    pub fn root_snapshot(&self) -> &HistorySnapshot {
        &self.root_snapshot
    }
    /// Exact current committed prefix.
    pub fn snapshot(&self) -> &HistorySnapshot {
        self.snapshots.last().map_or(&self.root_snapshot, |s| s)
    }
    /// Actual command/event ledger in native order.
    pub fn events(&self) -> &[GameEvent] {
        &self.events
    }
    /// Exact completed native outcome only.
    pub fn conclusion(&self) -> Option<Conclusion> {
        self.snapshot().conclusion
    }
    /// Actual deterministic reconstruction charge, excluding a refused candidate.
    pub fn replay_work(&self) -> u64 {
        self.snapshot().work
    }
    /// Full saved semantics plus never-rewound runtime authority.
    pub fn context(&self) -> GameContext {
        GameContext {
            root: self.root.clone(),
            events: self.events.clone(),
            runtime: self.runtime,
        }
    }
    /// Full context suitable for search identity; never a board-only cache key.
    pub fn search_identity(&self) -> GameContext {
        self.context()
    }
    /// Repetition-only native identity, excluding serials/counters/refusal flags.
    pub fn repetition_key(&self) -> &RepetitionKey {
        self.keys.last().expect("known root")
    }
    /// All surviving royal identities and positions for one owner.
    pub fn royals(&self, side: Side) -> Vec<(Square, Piece)> {
        self.position()
            .pieces()
            .filter(|(_, p)| p.side == side && p.is_royal())
            .collect()
    }
    /// Exact immutable event prefix; future outcomes/rights cannot leak backwards.
    pub fn event_prefix(&self, count: usize) -> Result<HistorySnapshot, GameError> {
        if count == 0 {
            Ok((*self.root_snapshot).clone())
        } else {
            self.snapshots
                .get(count - 1)
                .map(|s| (**s).clone())
                .ok_or(GameError::Prefix)
        }
    }
    /// Retire execution while selecting an immutable native history prefix.
    pub fn select_prefix(&mut self, count: usize) -> Result<HistorySnapshot, GameError> {
        let value = self.event_prefix(count)?;
        self.retire_context()?;
        Ok(value)
    }
    /// Advance runtime authority without rewriting any saved semantic state.
    pub fn retire_context(&mut self) -> Result<(), GameError> {
        let reserve = self.capacity_usage()?.reserved_events as u64;
        let revision = self
            .runtime
            .revision
            .checked_add(1)
            .ok_or(GameError::Capacity(Resource::Counter))?;
        if revision.checked_add(reserve).is_none() {
            return Err(GameError::Capacity(Resource::Counter));
        }
        self.runtime.revision = revision;
        Ok(())
    }
    /// Canonical exact prefix reconstructed with fresh native authority.
    pub fn record_prefix(&self, count: usize) -> Result<GameRecord, RecordError> {
        if count > self.events.len() {
            return Err(RecordError::Prefix);
        }
        let mut game =
            Self::from_root(Position::install(self.root.clone()).map_err(|_| RecordError::Root)?)
                .map_err(|_| RecordError::Root)?;
        for event in &self.events[..count] {
            game.submit(GameRequest {
                expected: game.context(),
                actor: event.actor,
                command: event.command.clone(),
            })
            .map_err(|_| RecordError::Derived)?;
        }
        GameRecord::from_game(&game)
    }
    /// Complete current native choices, with repetition admission separate from geometry.
    pub fn legal_actions(&self) -> Vec<Action> {
        self.legal_actions_with_stop(&mut || false)
            .expect("finite admitted native query")
    }
    /// Cancellable native Game choices; no process, frontend or implied outcome.
    pub fn legal_actions_with_stop(
        &self,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Vec<Action>, GameError> {
        check(stop)?;
        if self.conclusion().is_some() {
            return Ok(vec![]);
        }
        let mut out = vec![];
        for action in self.local.legal_actions_with_stop(&mut *stop)? {
            check(stop)?;
            let (next, _) = self.local.simulate_with_stop(&action, &mut *stop)?;
            let key = RepetitionKey::from_position(next.position());
            let mut repeated = false;
            for old in &self.keys {
                check(stop)?;
                if old == &key {
                    repeated = true;
                    break;
                }
            }
            if !repeated {
                out.push(action)
            }
        }
        Ok(out)
    }
    /// Actual game status, including explicitly unscored no-action cases.
    pub fn status(&self) -> GameStatus {
        if let Some(c) = self.conclusion() {
            GameStatus::Concluded(c)
        } else if let Some(side) = self.snapshot().pending_bare {
            GameStatus::BareReply { side }
        } else if self.legal_actions().is_empty() {
            GameStatus::NoBoardActions
        } else {
            GameStatus::Ongoing
        }
    }
    fn copy_for_transaction(&self) -> Self {
        Self {
            root: self.root.clone(),
            local: self.local.clone(),
            events: self.events.clone(),
            keys: self.keys.clone(),
            snapshots: self.snapshots.clone(),
            root_snapshot: self.root_snapshot.clone(),
            runtime: self.runtime,
            node: self.node,
            next_node: self.next_node,
            branch: self.branch,
            events_encoded_bytes: self.events_encoded_bytes,
        }
    }
    /// Atomic full-context admission; all effects/capacity validate before publication.
    pub fn submit(&mut self, request: GameRequest) -> Result<GameUndo, GameError> {
        self.submit_with_stop(request, &mut || false)
    }
    /// Cancellable atomic admission; a stopped candidate never changes accepted authority.
    pub fn submit_with_stop(
        &mut self,
        request: GameRequest,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<GameUndo, GameError> {
        if request.expected != self.context() {
            return Err(GameError::StaleContext);
        }
        self.commit(request.actor, request.command, true, stop)
    }
    fn commit(
        &mut self,
        actor: Side,
        command: GameCommand,
        new_branch: bool,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<GameUndo, GameError> {
        check(stop)?;
        if self.conclusion().is_some() && !matches!(command, GameCommand::Stop { .. }) {
            return Err(GameError::Concluded);
        }
        let mut candidate = self.copy_for_transaction();
        let mut work = 1;
        let mut effects = None;
        let mut board_ply = self.snapshot().board_ply;
        let mut conclusion = self.conclusion();
        let mut pending = self.snapshot().pending_bare;
        let mut offer = self.snapshot().draw_offer;
        let mut incomplete = self.snapshot().incomplete;
        match &command {
            GameCommand::Play { action } => {
                if actor != self.position().side_to_move() {
                    return Err(GameError::WrongActor);
                }
                let (e, _) = candidate.local.apply_with_stop(action, &mut *stop)?;
                work += e.work_units;
                let key = RepetitionKey::from_position(candidate.position());
                for old in &self.keys {
                    check(stop)?;
                    let (equal, n) = old.compared(&key);
                    work += n;
                    if equal {
                        return Err(GameError::Repetition);
                    }
                }
                candidate.keys.push(key);
                board_ply += 1;
                effects = Some(e);
                offer = None;
                incomplete = None;
                let (c, p, n) = adjudicate(&candidate.local, &candidate.keys, stop)?;
                work += n;
                conclusion = c;
                pending = p;
            }
            GameCommand::Resign => {
                conclusion = Some(Conclusion::Resignation {
                    winner: actor.opponent(),
                });
                pending = None;
                offer = None;
                incomplete = None;
            }
            GameCommand::OfferDraw => {
                if offer.is_some() {
                    return Err(GameError::DrawOffer);
                }
                offer = Some(actor);
            }
            GameCommand::AcceptDraw => {
                if offer != Some(actor.opponent()) {
                    return Err(GameError::DrawOffer);
                }
                conclusion = Some(Conclusion::Agreement);
                pending = None;
                offer = None;
                incomplete = None;
            }
            GameCommand::DeclineDraw => {
                if offer != Some(actor.opponent()) {
                    return Err(GameError::DrawOffer);
                }
                offer = None;
            }
            GameCommand::Stop { reason } => {
                incomplete = Some(*reason);
            }
        }
        candidate.push_event(
            actor, command, effects, board_ply, conclusion, pending, offer, incomplete, work,
        )?;
        let bytes = record::encoded_size(&candidate).map_err(|_| GameError::Record)?;
        let total = candidate
            .replay_work()
            .checked_add(bytes as u64)
            .ok_or(GameError::Capacity(Resource::ReplayWork))?;
        Arc::make_mut(candidate.snapshots.last_mut().expect("new snapshot")).work = total;
        candidate.ensure_capacity(stop)?;
        check(stop)?;
        let counter_reserve = candidate.capacity_usage_with_stop(stop)?.reserved_events as u64;
        for value in [self.runtime.revision, self.next_node, self.branch] {
            if value
                .checked_add(1)
                .and_then(|v| v.checked_add(counter_reserve))
                .is_none()
            {
                return Err(GameError::Capacity(Resource::Counter));
            }
        }
        let revision = self
            .runtime
            .revision
            .checked_add(1)
            .ok_or(GameError::Capacity(Resource::Counter))?;
        let next_node = self
            .next_node
            .checked_add(1)
            .ok_or(GameError::Capacity(Resource::Counter))?;
        let branch = self
            .branch
            .checked_add(u64::from(new_branch))
            .ok_or(GameError::Capacity(Resource::Counter))?;
        let token = GameUndo {
            instance: self.runtime.instance,
            before_node: self.node,
            after_node: self.next_node,
            before_len: self.events.len(),
            used: false,
        };
        candidate.runtime.revision = revision;
        candidate.node = self.next_node;
        candidate.next_node = next_node;
        candidate.branch = branch;
        *self = candidate;
        Ok(token)
    }
    #[allow(clippy::too_many_arguments)]
    fn push_event(
        &mut self,
        actor: Side,
        command: GameCommand,
        effects: Option<Effects>,
        board_ply: usize,
        conclusion: Option<Conclusion>,
        pending_bare: Option<Side>,
        draw_offer: Option<Side>,
        incomplete: Option<StopReason>,
        work: u64,
    ) -> Result<(), GameError> {
        let event = GameEvent {
            index: self.events.len(),
            board_ply,
            actor,
            command,
            effects,
            conclusion,
            pending_bare,
            draw_offer,
            incomplete,
        };
        let size = serde_json::to_vec(&event)
            .map_err(|_| GameError::Record)?
            .len();
        self.events_encoded_bytes = self
            .events_encoded_bytes
            .checked_add(size)
            .ok_or(GameError::Capacity(Resource::EncodedBytes))?;
        let charged = self
            .replay_work()
            .checked_add(work)
            .ok_or(GameError::Capacity(Resource::ReplayWork))?;
        self.events.push(event);
        self.snapshots.push(Arc::new(HistorySnapshot {
            state: self.position().state().clone(),
            event_count: self.events.len(),
            board_ply,
            conclusion,
            pending_bare,
            draw_offer,
            incomplete,
            work: charged,
        }));
        Ok(())
    }
    fn ending(&mut self, actor: Side, command: GameCommand) -> Result<u64, GameError> {
        let (s, c, p, o, i) = (
            self.snapshot().board_ply,
            self.conclusion(),
            self.snapshot().pending_bare,
            self.snapshot().draw_offer,
            self.snapshot().incomplete,
        );
        let (conclusion, pending, offer, incomplete) = match command {
            GameCommand::Resign => (
                Some(Conclusion::Resignation {
                    winner: actor.opponent(),
                }),
                None,
                None,
                None,
            ),
            GameCommand::Stop { reason } => (c, p, o, Some(reason)),
            _ => (c, p, o, i),
        };
        self.push_event(
            actor, command, None, s, conclusion, pending, offer, incomplete, 1,
        )?;
        Ok(record::encoded_size(self).map_err(|_| GameError::Record)? as u64 + 1)
    }
    /// Actual encoded complete document and widest actual ending sequence headroom.
    pub fn capacity_usage(&self) -> Result<CapacityUsage, GameError> {
        self.capacity_usage_with_stop(&mut || false)
    }
    fn capacity_usage_with_stop(
        &self,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<CapacityUsage, GameError> {
        let bytes = record::encoded_size(self).map_err(|_| GameError::Record)?;
        let mut largest_bytes = bytes;
        let mut largest_events = self.events.len();
        let mut largest_work = self.replay_work();
        // All actors are native choices. Only reserve a first Stop while ongoing and not already stopped.
        for first in [Side::Sente, Side::Gote] {
            for resign in [Side::Sente, Side::Gote] {
                for last in [Side::Sente, Side::Gote] {
                    check(stop)?;
                    let mut candidate = self.copy_for_transaction();
                    let mut extra = 0;
                    if candidate.conclusion().is_none() && candidate.snapshot().incomplete.is_none()
                    {
                        extra += candidate.ending(
                            first,
                            GameCommand::Stop {
                                reason: StopReason::ResourceLimit,
                            },
                        )?;
                    }
                    if candidate.conclusion().is_none() {
                        extra += candidate.ending(resign, GameCommand::Resign)?;
                    }
                    if candidate.snapshot().incomplete.is_none() {
                        extra += candidate.ending(
                            last,
                            GameCommand::Stop {
                                reason: StopReason::ResourceLimit,
                            },
                        )?;
                    }
                    largest_bytes = largest_bytes
                        .max(record::encoded_size(&candidate).map_err(|_| GameError::Record)?);
                    largest_events = largest_events.max(candidate.events.len());
                    largest_work = largest_work.max(
                        self.replay_work()
                            .checked_add(extra)
                            .ok_or(GameError::Capacity(Resource::ReplayWork))?,
                    );
                }
            }
        }
        Ok(CapacityUsage {
            encoded_bytes: bytes,
            events: self.events.len(),
            replay_work: self.replay_work(),
            reserved_bytes: largest_bytes - bytes,
            reserved_events: largest_events - self.events.len(),
            reserved_work: largest_work - self.replay_work(),
        })
    }
    fn ensure_capacity(&self, stop: &mut impl FnMut() -> bool) -> Result<(), GameError> {
        let u = self.capacity_usage_with_stop(stop)?;
        if u.encoded_bytes + u.reserved_bytes > MAX_GAME_RECORD_BYTES {
            return Err(GameError::Capacity(Resource::EncodedBytes));
        }
        if u.events + u.reserved_events > MAX_GAME_EVENTS {
            return Err(GameError::Capacity(Resource::Events));
        }
        if u.replay_work + u.reserved_work > MAX_REPLAY_WORK {
            return Err(GameError::Capacity(Resource::ReplayWork));
        }
        Ok(())
    }
    /// LIFO exact logical undo; runtime revisions advance rather than rewind.
    pub fn undo(&mut self, undo: &mut GameUndo) -> Result<GameRedo, GameError> {
        if undo.used
            || undo.instance != self.runtime.instance
            || undo.after_node != self.node
            || self.events.len() != undo.before_len + 1
        {
            return Err(GameError::InvalidToken);
        }
        let revision = self
            .runtime
            .revision
            .checked_add(1)
            .ok_or(GameError::Capacity(Resource::Counter))?;
        let before = self.event_prefix(undo.before_len)?;
        let event = self.events.last().expect("undo current").clone();
        let mut candidate = self.copy_for_transaction();
        candidate.events.truncate(undo.before_len);
        candidate.snapshots.truncate(undo.before_len);
        candidate.keys.truncate(before.board_ply + 1);
        candidate.local =
            LocalPosition::admit(Position::install(before.state).map_err(GameError::Root)?)
                .map_err(GameError::Root)?;
        candidate.events_encoded_bytes = candidate
            .events
            .iter()
            .map(|e| serde_json::to_vec(e).expect("accepted event").len())
            .sum();
        let reserve = candidate.capacity_usage()?.reserved_events as u64;
        if revision.checked_add(reserve).is_none()
            || candidate.next_node.checked_add(reserve).is_none()
            || candidate.branch.checked_add(reserve).is_none()
        {
            return Err(GameError::Capacity(Resource::Counter));
        }
        candidate.runtime.revision = revision;
        candidate.node = undo.before_node;
        let redo = GameRedo {
            instance: candidate.runtime.instance,
            branch: candidate.branch,
            before: candidate.context(),
            actor: event.actor,
            command: event.command,
            used: false,
        };
        *self = candidate;
        undo.used = true;
        Ok(redo)
    }
    /// Redo without reviving old runtime requests; new commands retire the redo branch.
    pub fn redo(&mut self, redo: &mut GameRedo) -> Result<GameUndo, GameError> {
        if redo.used
            || redo.instance != self.runtime.instance
            || redo.branch != self.branch
            || !redo.before.same_semantics(&self.context())
        {
            return Err(GameError::InvalidToken);
        }
        let undo = self.commit(redo.actor, redo.command.clone(), false, &mut || false)?;
        redo.used = true;
        Ok(undo)
    }
}
fn check(stop: &mut impl FnMut() -> bool) -> Result<(), GameError> {
    if stop() {
        Err(GameError::Stopped)
    } else {
        Ok(())
    }
}
fn royalty(p: &Position) -> [usize; 2] {
    let mut n = [0; 2];
    for (_, piece) in p.pieces() {
        if piece.is_royal() {
            n[piece.side.index()] += 1
        }
    }
    n
}
fn terminal_royals(p: &Position) -> Option<Conclusion> {
    match royalty(p) {
        [0, 0] => Some(Conclusion::SimultaneousRoyalLoss),
        [0, _] => Some(Conclusion::RoyalCapture { winner: Side::Gote }),
        [_, 0] => Some(Conclusion::RoyalCapture {
            winner: Side::Sente,
        }),
        _ => None,
    }
}
fn bare(p: &Position, side: Side) -> bool {
    let pieces = p
        .pieces()
        .filter(|(_, p)| p.side == side)
        .map(|(_, p)| p)
        .collect::<Vec<_>>();
    pieces.len() == 1 && pieces[0].is_royal()
}
fn adjudicate(
    local: &LocalPosition,
    keys: &[RepetitionKey],
    stop: &mut impl FnMut() -> bool,
) -> Result<(Option<Conclusion>, Option<Side>, u64), GameError> {
    check(stop)?;
    let p = local.position();
    if let Some(c) = terminal_royals(p) {
        return Ok((Some(c), None, 256));
    }
    let b = [bare(p, Side::Sente), bare(p, Side::Gote)];
    if b == [true, true] {
        return Ok((Some(Conclusion::BothBareRoyals), None, 256));
    }
    let Some(victim) = [Side::Sente, Side::Gote].into_iter().find(|s| b[s.index()]) else {
        return Ok((None, None, 256));
    };
    if victim != p.side_to_move() {
        return Ok((
            Some(Conclusion::BareRoyalLoss {
                winner: victim.opponent(),
            }),
            None,
            256,
        ));
    }
    let query = local.legal_actions_metered(&mut *stop)?;
    let mut work = query.work_units + 256;
    for action in query.value {
        check(stop)?;
        let (next, e) = local.simulate_with_stop(&action, &mut *stop)?;
        work += e.work_units;
        let key = RepetitionKey::from_position(next.position());
        let mut repeated = false;
        for old in keys {
            check(stop)?;
            let (equal, n) = old.compared(&key);
            work += n;
            if equal {
                repeated = true;
                break;
            }
        }
        if repeated {
            continue;
        }
        let escape = terminal_royals(next.position())
            .is_some_and(|c| c.winner() != Some(victim.opponent()))
            || (bare(next.position(), Side::Sente) && bare(next.position(), Side::Gote));
        if escape {
            return Ok((None, Some(victim), work));
        }
    }
    Ok((
        Some(Conclusion::BareRoyalLoss {
            winner: victim.opponent(),
        }),
        None,
        work,
    ))
}
