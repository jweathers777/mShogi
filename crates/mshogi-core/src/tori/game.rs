//! Authoritative Tori history and admission; local board rules remain in LocalPosition.
use super::*;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

/// Total committed commands/events, including reserved concluding administration.
pub const MAX_GAME_COMMANDS: usize = 512;
/// One atomic native event per accepted command; derived facts are in that event.
pub const MAX_GAME_EVENTS: usize = 512;
/// Deterministic replay units: root=1; each command=1 plus prior keys scanned by a board action.
pub const MAX_REPLAY_WORK: u64 = 65_536;
const RESERVED_BYTES_PER_EVENT: usize = 4096;

static NEXT_INSTANCE: AtomicU64 = AtomicU64::new(1);

/// Identity for repetition only: every native face/owner, both hands and turn.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepetitionKey {
    board: [Option<Piece>; 49],
    hands: [Hand; 2],
    side: Side,
}
impl RepetitionKey {
    /// Counters, provenance, events and runtime identities deliberately do not enter this key.
    pub fn from_position(p: &Position) -> Self {
        Self {
            board: p.state.board,
            hands: p.state.hands,
            side: p.state.side,
        }
    }
}
/// Obligation at the latest third-or-later occurrence, never a retroactive arrival refusal.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Obligation {
    /// Side moving OUT of the repeated key.
    pub side: Side,
    /// Latest three zero-based board-position indices, including the known root.
    pub occurrences: [usize; 3],
    /// Deduplicated actual complete departures from the first two indices.
    pub forbidden_departures: Vec<Action>,
}
impl Obligation {
    /// Current arrival index.
    pub fn at_ply(&self) -> usize {
        self.occurrences[2]
    }
    /// History permission, separate from local board legality.
    pub fn permits(&self, action: Action) -> bool {
        !self.forbidden_departures.contains(&action)
    }
    pub(super) fn derive(
        side: Side,
        occurrences: &[usize],
        departure: impl Fn(usize) -> Action,
    ) -> Option<Self> {
        let last: [usize; 3] = occurrences
            .get(occurrences.len().checked_sub(3)?..)?
            .try_into()
            .ok()?;
        let mut forbidden_departures = vec![departure(last[0])];
        let other = departure(last[1]);
        if !forbidden_departures.contains(&other) {
            forbidden_departures.push(other);
        }
        Some(Self {
            side,
            occurrences: last,
            forbidden_departures,
        })
    }
    pub(super) fn forces_loss(&self, local: &[Action]) -> bool {
        !local.is_empty() && local.iter().all(|a| !self.permits(*a))
    }
}
/// The selected profile has wins/losses only; operational failures never enter this type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Conclusion {
    /// Checked Phoenix with no local action.
    Checkmate {
        /// Native winner.
        winner: Side,
    },
    /// Quiet no-action is a loss under T05.
    StalemateLoss {
        /// Immobile native side.
        loser: Side,
    },
    /// All local departures repeat one of the forbidden choices.
    RepetitionForfeit {
        /// Responsible departure side.
        loser: Side,
    },
    /// Explicit live resignation, irrespective of board turn.
    Resignation {
        /// Opponent of the resigning actor.
        winner: Side,
    },
}
/// Recorded unscored operational reason, not a chess-family conclusion or raw diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    /// External work cancelled.
    Cancelled,
    /// Resource admission refused elsewhere.
    ResourceLimit,
    /// Engine/process failed.
    EngineFailure,
    /// Experiment/search work exhausted.
    WorkLimit,
}
/// Native participant/administrative command. Unsupported declarations remain explicit refusals.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum GameCommand {
    /// Complete board choice.
    Play {
        /// Actual game-owned action.
        action: Action,
    },
    /// Actor resigns, including out of turn.
    Resign,
    /// Unsupported by this selected profile.
    AgreeDraw,
    /// Unsupported by this selected profile.
    DeclareImpasse,
    /// Record incomplete work; does not conclude the game or authorize engine work.
    Stop {
        /// Bounded unscored reason.
        reason: StopReason,
    },
}
/// Exact ordered command plus all authoritative derived facts at its committed prefix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameEvent {
    /// Zero-based event index.
    pub index: usize,
    /// Accepted board actions (not the declared imported counter).
    pub board_ply: usize,
    /// Native command actor, even when not the side to move.
    pub actor: Side,
    /// Original complete submitted command.
    pub command: GameCommand,
    /// Native board effects; null for non-board events.
    pub effects: Option<Effects>,
    /// All known occurrences of the current board key.
    pub occurrences: Vec<usize>,
    /// Current departure obligation, including when it forces a conclusion.
    pub obligation: Option<Obligation>,
    /// Actual native conclusion only.
    pub conclusion: Option<Conclusion>,
    /// Latest incomplete operation; a later accepted board action/resignation clears it.
    pub incomplete: Option<StopReason>,
    /// Exact accumulated deterministic replay admission charge.
    pub replay_work: u64,
}
/// Immutable historical data; no action application or playable branch method.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistorySnapshot {
    /// All board/hand/turn/root/counter semantics at this exact prefix.
    state: StructuralState,
    /// Exact event prefix length.
    pub event_count: usize,
    /// Known board-action prefix length.
    pub board_ply: usize,
    /// Current key's root-indexed occurrences, without future knowledge.
    pub occurrences: Vec<usize>,
    /// Current obligation.
    pub obligation: Option<Obligation>,
    /// Conclusion known at this prefix.
    pub conclusion: Option<Conclusion>,
    /// Incomplete operation known at this prefix.
    pub incomplete: Option<StopReason>,
}
impl HistorySnapshot {
    /// Exact native logical state. External callers cannot replace this admitted
    /// state with an unvalidated diagram or construct forged history views.
    pub fn state(&self) -> &StructuralState {
        &self.state
    }
    /// Read-only local choices/inspection are available without creating a playable Game.
    pub fn local_position(&self) -> LocalPosition {
        // Already admitted native state; historical moving-side check is permitted.
        LocalPosition::admit(Position::install(self.state.clone()))
            .expect("accepted historical state")
    }
}
/// Never-rewound native command authority, deliberately absent from canonical records.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeToken {
    instance: u64,
    revision: u64,
}
/// Full native context; process/request/purpose/limits remain later consumers' ownership.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameContext {
    root: StructuralState,
    events: Vec<GameEvent>,
    runtime: RuntimeToken,
}
impl GameContext {
    /// Compare canonical semantics independently of execution retirement.
    pub fn same_semantics(&self, other: &Self) -> bool {
        self.root == other.root && self.events == other.events
    }
    /// Native runtime authority, for technical diagnostics and exact equality checks.
    pub fn runtime(&self) -> RuntimeToken {
        self.runtime
    }
}
/// Context-checked complete request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRequest {
    /// Exact full-history and current native runtime context.
    pub expected: GameContext,
    /// Native actor; turn validation applies only to Play.
    pub actor: Side,
    /// Complete command.
    pub command: GameCommand,
}
/// Exhausted native resource, never an illegal move or a game outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resource {
    /// Complete encoded native bytes.
    EncodedBytes,
    /// Native command/event count.
    Events,
    /// Deterministic replay charge.
    ReplayWork,
    /// Counter or monotonic native authority exhausted.
    Counter,
}
/// Atomic native admission/reversal error; failures do not commit audit events.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameError {
    /// T07 local root failure.
    Root(AdmissionError),
    /// A live root cannot silently reset an already played position's known history.
    NotRoot,
    /// Wrong history or retired runtime.
    StaleContext,
    /// Board action actor differs from native turn.
    WrongActor,
    /// Selected profile has no agreement/impasse declaration.
    UnsupportedCommand,
    /// An actual native conclusion already exists.
    Concluded,
    /// This complete board action is forbidden by T06.
    RepetitionDeparture,
    /// Complete local rule refusal.
    Action(ActionError),
    /// Duplicate stop without intervening board work.
    AlreadyStopped,
    /// Explicit unscored admission refusal.
    Capacity(Resource),
    /// Invalid historical index/reversal token.
    InvalidToken,
}
impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tori game request refused: {self:?}")
    }
}
impl Error for GameError {}
/// Exact native persistence/replay metadata for later outer document admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapacityUsage {
    /// Actual full encoded native bytes.
    pub encoded_bytes: usize,
    /// Native events and commands (one-to-one).
    pub events: usize,
    /// Deterministic replay units.
    pub replay_work: u64,
}
#[derive(Debug)]
/// Single-use instance/order-bound full-game reversal, not serialized history.
pub struct GameUndo {
    instance: u64,
    before_node: u64,
    after_node: u64,
    before_len: usize,
    used: bool,
}
#[derive(Debug)]
/// Forward semantic restoration; new participant commands retire outstanding redos.
pub struct GameRedo {
    instance: u64,
    branch: u64,
    before: GameContext,
    actor: Side,
    command: GameCommand,
    used: bool,
}
/// Authoritative game owner. Neither LocalPosition nor a repetition key grants live permission.
#[derive(Debug)]
pub struct Game {
    pub(super) root: StructuralState,
    local: LocalPosition,
    pub(super) events: Vec<GameEvent>,
    keys: Vec<RepetitionKey>,
    actions: Vec<Action>,
    snapshots: Vec<HistorySnapshot>,
    root_snapshot: HistorySnapshot,
    runtime: RuntimeToken,
    node: u64,
    branch: u64,
}
impl Game {
    #[cfg(test)]
    pub(super) fn transaction_for_test(&self) -> Self {
        self.transaction_copy()
    }
    #[cfg(test)]
    pub(super) fn cache_snapshot(&self) -> String {
        format!(
            "{:?}",
            (
                &self.root,
                self.position().state(),
                &self.events,
                &self.keys,
                &self.actions,
                &self.snapshots,
                &self.root_snapshot
            )
        )
    }
    /// Admit a complete known-history root, including immediate T05 terminal outcomes.
    pub fn from_root(root: Position) -> Result<Self, GameError> {
        if root.accepted_plies() != 0 {
            return Err(GameError::NotRoot);
        }
        let local = LocalPosition::admit(root).map_err(GameError::Root)?;
        let root = local.position().state().clone();
        let conclusion = board_conclusion(local.status(), local.position().side_to_move());
        let root_snapshot = HistorySnapshot {
            state: root.clone(),
            event_count: 0,
            board_ply: 0,
            occurrences: vec![0],
            obligation: None,
            conclusion,
            incomplete: None,
        };
        let instance = NEXT_INSTANCE
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
            .map_err(|_| GameError::Capacity(Resource::Counter))?;
        let key = RepetitionKey::from_position(local.position());
        Ok(Self {
            root,
            local,
            events: vec![],
            keys: vec![key],
            actions: vec![],
            snapshots: vec![],
            root_snapshot,
            runtime: RuntimeToken {
                instance,
                revision: 0,
            },
            node: 0,
            branch: 0,
        })
    }
    /// Full standard game with empty known history and the standard root as occurrence1.
    pub fn standard() -> Self {
        Self::from_root(Position::standard()).expect("standard root and available runtime identity")
    }
    /// Current immutable local board owner; legal board choices are not game permission.
    pub fn local_position(&self) -> &LocalPosition {
        &self.local
    }
    /// Current native position and provenance.
    pub fn position(&self) -> &Position {
        self.local.position()
    }
    /// Full ordered accepted native events.
    pub fn events(&self) -> &[GameEvent] {
        &self.events
    }
    /// Exact semantic snapshot at the current committed prefix.
    pub fn snapshot(&self) -> &HistorySnapshot {
        self.snapshots.last().unwrap_or(&self.root_snapshot)
    }
    /// Actual concluded native result.
    pub fn conclusion(&self) -> Option<Conclusion> {
        self.snapshot().conclusion
    }
    /// Current history-dependent departure obligation.
    pub fn obligation(&self) -> Option<&Obligation> {
        self.snapshot().obligation.as_ref()
    }
    /// Full semantic context plus fresh runtime authority.
    pub fn context(&self) -> GameContext {
        GameContext {
            root: self.root.clone(),
            events: self.events.clone(),
            runtime: self.runtime,
        }
    }
    /// Current repetition-only identity; not a saved game/context identity.
    pub fn repetition_key(&self) -> &RepetitionKey {
        self.keys.last().expect("known root")
    }
    /// Current known key occurrences, including root where appropriate.
    pub fn occurrences(&self) -> &[usize] {
        &self.snapshot().occurrences
    }
    /// Complete permitted native choices; concluded games have none even if local choices exist.
    pub fn legal_actions(&self) -> Vec<Action> {
        if self.conclusion().is_some() {
            return vec![];
        }
        self.local
            .legal_actions()
            .into_iter()
            .filter(|a| self.obligation().is_none_or(|o| o.permits(*a)))
            .collect()
    }
    /// Immutable historical query. Selection retirement is explicitly separate below.
    pub fn event_prefix(&self, count: usize) -> Result<HistorySnapshot, GameError> {
        if count == 0 {
            return Ok(self.root_snapshot.clone());
        }
        self.snapshots
            .get(count - 1)
            .cloned()
            .ok_or(GameError::InvalidToken)
    }
    /// Complete canonical history at an exact committed event prefix, for read-only
    /// historical analysis/inspection. Future commands and obligations are absent.
    /// This does not branch, rewind or replace the live Game; restoring the record
    /// later always creates fresh native authority.
    pub fn record_prefix(&self, count: usize) -> Result<GameRecord, RecordError> {
        let snapshot = self.event_prefix(count).map_err(|_| RecordError::Prefix)?;
        let mut prefix = self.transaction_copy();
        prefix.events.truncate(count);
        prefix.snapshots.truncate(count);
        prefix.actions.truncate(snapshot.board_ply);
        prefix.keys.truncate(snapshot.board_ply + 1);
        prefix.local = LocalPosition::admit(Position::install(snapshot.state))
            .expect("native historical state");
        GameRecord::from_game(&prefix)
    }
    /// Select an immutable historical prefix and retire current native command authority.
    /// The view has no Game constructor/branch operation and cannot submit a live action.
    pub fn select_prefix(&mut self, count: usize) -> Result<HistorySnapshot, GameError> {
        let snapshot = self.event_prefix(count)?;
        self.retire_context()?;
        Ok(snapshot)
    }
    /// Retire native authority without altering canonical semantics (e.g. Return to live).
    pub fn retire_context(&mut self) -> Result<(), GameError> {
        self.runtime.revision = self.next_revision()?;
        Ok(())
    }
    fn next_revision(&self) -> Result<u64, GameError> {
        self.runtime
            .revision
            .checked_add(1)
            .ok_or(GameError::Capacity(Resource::Counter))
    }
    fn replay_work(&self) -> u64 {
        self.events.last().map_or(1, |e| e.replay_work)
    }
    /// Actual native encoded bytes and deterministic bounded replay accounting.
    pub fn capacity_usage(&self) -> Result<CapacityUsage, GameError> {
        Ok(CapacityUsage {
            encoded_bytes: record::encode(self)
                .map_err(|_| GameError::Capacity(Resource::EncodedBytes))?
                .len(),
            events: self.events.len(),
            replay_work: self.replay_work(),
        })
    }
    fn transaction_copy(&self) -> Self {
        Self {
            root: self.root.clone(),
            local: self.local.clone(),
            events: self.events.clone(),
            keys: self.keys.clone(),
            actions: self.actions.clone(),
            snapshots: self.snapshots.clone(),
            root_snapshot: self.root_snapshot.clone(),
            runtime: self.runtime,
            node: self.node,
            branch: self.branch,
        }
    }
    /// Atomic participant admission. Capacity is checked on a complete candidate before commit.
    pub fn submit(&mut self, request: GameRequest) -> Result<GameUndo, GameError> {
        if request.expected != self.context() {
            return Err(GameError::StaleContext);
        }
        self.commit(request.actor, request.command, true)
    }
    fn commit(
        &mut self,
        actor: Side,
        command: GameCommand,
        new_branch: bool,
    ) -> Result<GameUndo, GameError> {
        if self.conclusion().is_some() {
            return Err(GameError::Concluded);
        }
        let revision = self.next_revision()?;
        let branch = self
            .branch
            .checked_add(u64::from(new_branch))
            .ok_or(GameError::Capacity(Resource::Counter))?;
        let mut candidate = self.transaction_copy();
        let mut effects = None;
        let mut occurrences = self.occurrences().to_vec();
        let mut obligation = self.obligation().cloned();
        let mut conclusion = None;
        let mut incomplete = None;
        let work = self
            .replay_work()
            .checked_add(
                1 + if matches!(command, GameCommand::Play { .. }) {
                    self.keys.len() as u64
                } else {
                    0
                },
            )
            .ok_or(GameError::Capacity(Resource::ReplayWork))?;
        match command {
            GameCommand::Play { action } => {
                if actor != self.position().side_to_move() {
                    return Err(GameError::WrongActor);
                }
                // Validate local rules first so history cannot excuse an illegal choice.
                if !self.local.is_legal(action) {
                    return Err(GameError::Action(
                        self.local
                            .clone()
                            .apply(action)
                            .expect_err("not locally legal"),
                    ));
                }
                if obligation.as_ref().is_some_and(|o| !o.permits(action)) {
                    return Err(GameError::RepetitionDeparture);
                }
                effects = Some(
                    candidate
                        .local
                        .apply(action)
                        .map_err(|e| {
                            if e == ActionError::Capacity {
                                GameError::Capacity(Resource::Counter)
                            } else {
                                GameError::Action(e)
                            }
                        })?
                        .0,
                );
                candidate.actions.push(action);
                candidate
                    .keys
                    .push(RepetitionKey::from_position(candidate.position()));
                occurrences = candidate.current_occurrences();
                let local_choices = candidate.local.legal_actions();
                let side = candidate.position().side_to_move();
                let status = match (candidate.local.is_in_check(side), local_choices.is_empty()) {
                    (true, true) => LocalStatus::Checkmate,
                    (false, true) => LocalStatus::QuietNoAction,
                    (true, false) => LocalStatus::Check,
                    (false, false) => LocalStatus::Ongoing,
                };
                conclusion = board_conclusion(status, side);
                obligation = None;
                if conclusion.is_none() {
                    obligation = Obligation::derive(side, &occurrences, |i| candidate.actions[i]);
                    if obligation
                        .as_ref()
                        .is_some_and(|o| o.forces_loss(&local_choices))
                    {
                        conclusion = Some(Conclusion::RepetitionForfeit { loser: side });
                    }
                }
            }
            GameCommand::Resign => {
                conclusion = Some(Conclusion::Resignation {
                    winner: actor.opponent(),
                });
            }
            GameCommand::AgreeDraw | GameCommand::DeclareImpasse => {
                return Err(GameError::UnsupportedCommand);
            }
            GameCommand::Stop { reason } => {
                if self.snapshot().incomplete.is_some() {
                    return Err(GameError::AlreadyStopped);
                }
                incomplete = Some(reason);
            }
        }
        let event = GameEvent {
            index: self.events.len(),
            board_ply: candidate.actions.len(),
            actor,
            command,
            effects,
            occurrences,
            obligation,
            conclusion,
            incomplete,
            replay_work: work,
        };
        candidate.events.push(event.clone());
        candidate.snapshots.push(HistorySnapshot {
            state: candidate.position().state().clone(),
            event_count: candidate.events.len(),
            board_ply: candidate.actions.len(),
            occurrences: event.occurrences.clone(),
            obligation: event.obligation.clone(),
            conclusion,
            incomplete,
        });
        let remaining = match command {
            GameCommand::Play { .. } if conclusion.is_none() => 2,
            GameCommand::Stop { .. } => 1,
            _ => 0,
        };
        let bytes = record::encode(&candidate)
            .map_err(|_| GameError::Capacity(Resource::EncodedBytes))?
            .len();
        check_capacity(
            CapacityUsage {
                encoded_bytes: bytes,
                events: candidate.events.len(),
                replay_work: work,
            },
            remaining,
        )?;
        candidate.runtime.revision = revision;
        candidate.node = revision;
        candidate.branch = branch;
        let undo = GameUndo {
            instance: self.runtime.instance,
            before_node: self.node,
            after_node: revision,
            before_len: self.events.len(),
            used: false,
        };
        *self = candidate;
        Ok(undo)
    }
    fn current_occurrences(&self) -> Vec<usize> {
        let key = self.keys.last().expect("known root");
        let current = self.keys.len() - 1;
        let mut occurrences: Vec<_> = self.keys[..current]
            .iter()
            .enumerate()
            .filter_map(|(i, k)| (k == key).then_some(i))
            .collect();
        occurrences.push(current);
        occurrences
    }
    /// Exact LIFO semantic undo. Monotonic runtime authority advances, never rewinds.
    pub fn undo(&mut self, undo: &mut GameUndo) -> Result<GameRedo, GameError> {
        if undo.used
            || undo.instance != self.runtime.instance
            || undo.after_node != self.node
            || self.events.len() != undo.before_len + 1
        {
            return Err(GameError::InvalidToken);
        }
        let revision = self.next_revision()?;
        let last = self.events.last().expect("undo post-state").clone();
        let before = self.event_prefix(undo.before_len)?;
        self.events.truncate(undo.before_len);
        self.snapshots.truncate(undo.before_len);
        self.actions.truncate(before.board_ply);
        self.keys.truncate(before.board_ply + 1);
        self.local =
            LocalPosition::admit(Position::install(before.state)).expect("native previous state");
        self.runtime.revision = revision;
        self.node = undo.before_node;
        undo.used = true;
        Ok(GameRedo {
            instance: self.runtime.instance,
            branch: self.branch,
            before: self.context(),
            actor: last.actor,
            command: last.command,
            used: false,
        })
    }
    /// Exact redo with newly issued undo authority; a new command retires old redo tokens.
    pub fn redo(&mut self, redo: &mut GameRedo) -> Result<GameUndo, GameError> {
        if redo.used
            || redo.instance != self.runtime.instance
            || redo.branch != self.branch
            || !redo.before.same_semantics(&self.context())
        {
            return Err(GameError::InvalidToken);
        }
        let undo = self.commit(redo.actor, redo.command, false)?;
        redo.used = true;
        Ok(undo)
    }
}
fn board_conclusion(status: LocalStatus, side: Side) -> Option<Conclusion> {
    match status {
        LocalStatus::Checkmate => Some(Conclusion::Checkmate {
            winner: side.opponent(),
        }),
        LocalStatus::QuietNoAction => Some(Conclusion::StalemateLoss { loser: side }),
        _ => None,
    }
}

fn check_capacity(usage: CapacityUsage, reserve: usize) -> Result<(), GameError> {
    if usage.events > MAX_GAME_EVENTS - reserve || usage.events > MAX_GAME_COMMANDS - reserve {
        return Err(GameError::Capacity(Resource::Events));
    }
    if usage.replay_work > MAX_REPLAY_WORK - reserve as u64 {
        return Err(GameError::Capacity(Resource::ReplayWork));
    }
    if usage.encoded_bytes > MAX_GAME_RECORD_BYTES - reserve * RESERVED_BYTES_PER_EVENT {
        return Err(GameError::Capacity(Resource::EncodedBytes));
    }
    Ok(())
}

#[cfg(test)]
mod capacity_tests {
    use super::*;
    #[test]
    fn exact_production_capacity_and_reservation_policy_boundaries() {
        for reserve in 0..=2 {
            let max = CapacityUsage {
                encoded_bytes: MAX_GAME_RECORD_BYTES - reserve * RESERVED_BYTES_PER_EVENT,
                events: MAX_GAME_EVENTS - reserve,
                replay_work: MAX_REPLAY_WORK - reserve as u64,
            };
            assert_eq!(check_capacity(max, reserve), Ok(()));
            for resource in [
                Resource::EncodedBytes,
                Resource::Events,
                Resource::ReplayWork,
            ] {
                let mut next = max;
                match resource {
                    Resource::EncodedBytes => next.encoded_bytes += 1,
                    Resource::Events => next.events += 1,
                    Resource::ReplayWork => next.replay_work += 1,
                    _ => unreachable!(),
                }
                assert_eq!(
                    check_capacity(next, reserve),
                    Err(GameError::Capacity(resource))
                );
            }
        }
    }
}
#[cfg(test)]
mod reservation_bound_test {
    use super::*;
    #[test]
    fn constructed_metadata_upper_bound_proves_each_reserved_nonboard_event_fits() {
        // A conservative bound, not a fabricated reachable game or conformance
        // position: every allowable index appears, and both departures are maximal.
        let move_a = Action::Move {
            from: "7g".parse().unwrap(),
            to: "1a".parse().unwrap(),
            result_face: Face::RightQuail,
        };
        let move_b = Action::Move {
            from: "6g".parse().unwrap(),
            to: "2a".parse().unwrap(),
            result_face: Face::LeftQuail,
        };
        let o = Obligation {
            side: Side::Sente,
            occurrences: [510, 511, 512],
            forbidden_departures: vec![move_a, move_b],
        };
        for command in [
            GameCommand::Stop {
                reason: StopReason::ResourceLimit,
            },
            GameCommand::Resign,
        ] {
            let event = GameEvent {
                index: MAX_GAME_EVENTS - 1,
                board_ply: MAX_GAME_COMMANDS,
                actor: Side::Sente,
                command,
                effects: None,
                occurrences: (0..=MAX_GAME_COMMANDS).collect(),
                obligation: Some(o.clone()),
                conclusion: Some(Conclusion::Resignation {
                    winner: Side::Sente,
                }),
                incomplete: Some(StopReason::ResourceLimit),
                replay_work: MAX_REPLAY_WORK,
            };
            let bytes = serde_json::to_vec(&event).unwrap().len();
            // Extra256 conservatively covers comma and changed current nullable
            // conclusion/incomplete fields. Root/current board do not change.
            assert!(
                bytes + 256 <= RESERVED_BYTES_PER_EVENT,
                "reserved metadata bound {bytes}"
            );
        }
    }
}
