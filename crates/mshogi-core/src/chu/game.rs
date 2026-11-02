//! Complete Chu game admission. Local geometry never grants live history permission.
use super::*;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

/// Maximum native events, including two reserved concluding/stop events.
pub const MAX_GAME_EVENTS: usize = 512;
/// Native commands and events are one-to-one.
pub const MAX_GAME_COMMANDS: usize = MAX_GAME_EVENTS;
/// Deterministic history/reply work bound; resource refusal is never an outcome.
pub const MAX_REPLAY_WORK: u64 = 4_000_000;
const RESERVED_EVENTS: usize = 2;
static NEXT_INSTANCE: AtomicU64 = AtomicU64::new(1);

/// Repetition projection: native original/once-promoted placement, turn and Lion rights.
/// Counters, provenance, derived obligations, events and runtime identities are excluded.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepetitionKey {
    board: Vec<(Square, Piece)>,
    side: Side,
    protection: LionProtection,
}
impl RepetitionKey {
    /// Projection of actual native state, not a saved/search context.
    pub fn from_local(local: &LocalPosition) -> Self {
        Self {
            board: local.position().pieces().collect(),
            side: local.position().side_to_move(),
            protection: local.protection().clone(),
        }
    }
}
/// Native repetition responsibility category, not a forfeiture or draw.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Responsibility {
    /// Both sides make only quiet returns: first actor of the actual streak.
    FirstQuietReturnMover,
    /// Exactly one continuous sole-royal checker.
    SoleContinuousChecker,
    /// Both continuous sole-royal checkers.
    ContinuousCheckers,
    /// Only one side attacks during the cycle.
    SoleAttacker,
    /// Neither or both ordinary attackers: repeated boundary's side to move.
    SideToMoveFallback,
}
/// Witness for two matching completed cycles and an in-progress prohibited third.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Obligation {
    /// Root-indexed board boundaries, never event indices.
    pub occurrences: [usize; 3],
    /// Number of complete board actions matched since the third boundary.
    pub progress: usize,
    /// Native responsible players, canonical Sente/Gote order.
    pub responsible_sides: Vec<Side>,
    /// Sourced priority category.
    pub category: Responsibility,
    /// Actual prospective closing mover; may differ from the responsible side.
    pub closing_mover: Side,
}
/// Native rules conclusion. Process/resource failures never inhabit this type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Conclusion {
    /// Actual complete action removed the last opponent royal.
    RoyalCapture {
        /// Native winner.
        winner: Side,
    },
    /// Finite immediate-threat/complete-response predicate, not forced mate search.
    Mate {
        /// Native winner.
        winner: Side,
    },
    /// Exact narrow JCSA material-exhaustion award.
    Material {
        /// Native winner.
        winner: Side,
    },
    /// Actual live resignation, independent of board turn.
    Resignation {
        /// Native winner.
        winner: Side,
    },
    /// Actual opposing acceptance of a live recorded offer.
    AgreedDraw,
}
impl Conclusion {
    /// Native winner; agreed draws have none.
    pub fn winner(self) -> Option<Side> {
        match self {
            Self::RoyalCapture { winner }
            | Self::Mate { winner }
            | Self::Material { winner }
            | Self::Resignation { winner } => Some(winner),
            Self::AgreedDraw => None,
        }
    }
}
/// Current board status without inventing a quiet immobility conclusion.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameStatus {
    /// Admitted board actions remain.
    Ongoing,
    /// No admitted board action and no sourced conclusion; unscored.
    NoBoardActions,
    /// Actual native conclusion exists.
    Concluded,
}
/// Unscored operation annotation, independent of game outcomes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    /// Cancelled work.
    Cancelled,
    /// Resource exhaustion outside board rules.
    ResourceLimit,
    /// Process failure.
    EngineFailure,
    /// Bounded work incomplete.
    WorkLimit,
}
/// Complete native command. Actor is separately bound by GameRequest.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum GameCommand {
    /// One complete board action, never an intermediate subturn.
    Play {
        /// Native route and optional once-only flip.
        action: Action,
    },
    /// Live resignation by either participant.
    Resign,
    /// Actual participant material-exhaustion judgment; does not conclude.
    OfferDraw,
    /// Opposing acceptance of the current offer.
    AcceptDraw,
    /// Opposing refusal of the current offer.
    DeclineDraw,
    /// Unscored operational annotation; never authorizes work.
    Stop {
        /// Typed reason.
        reason: StopReason,
    },
}
/// Complete accepted event and native-derived facts at this exact prefix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameEvent {
    /// Zero-based event index.
    pub index: usize,
    /// Number of known accepted complete board actions.
    pub board_ply: usize,
    /// Actual command actor; only Play requires the board turn.
    pub actor: Side,
    /// Original complete command.
    pub command: GameCommand,
    /// Native ordered effects, absent for controls.
    pub effects: Option<Effects>,
    /// Current key's known board-boundary occurrences.
    pub occurrences: Vec<usize>,
    /// Complete active must-vary witnesses/progress.
    pub obligations: Vec<Obligation>,
    /// Actual native conclusion.
    pub conclusion: Option<Conclusion>,
    /// Current actual draw offer.
    pub draw_offer: Option<Side>,
    /// Current operational annotation.
    pub incomplete: Option<StopReason>,
    /// Accumulated deterministic native replay work.
    pub replay_work: u64,
}
/// Immutable logical position/history view with no live command authority.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HistorySnapshot {
    /// Complete structural state/provenance/counters.
    pub state: StructuralState,
    /// Native next-turn Lion rights.
    pub protection: LionProtection,
    /// Exact event-prefix length, independent of board ply.
    pub event_count: usize,
    /// Complete board-action count.
    pub board_ply: usize,
    /// Known occurrences at this prefix only.
    pub occurrences: Vec<usize>,
    /// Active native admission obligations.
    pub obligations: Vec<Obligation>,
    /// Actual native outcome only.
    pub conclusion: Option<Conclusion>,
    /// Recorded offer, never implied consent.
    pub draw_offer: Option<Side>,
    /// Unscored stopped work.
    pub incomplete: Option<StopReason>,
}
impl HistorySnapshot {
    /// Immutable native local inspection with this prefix's rights and no future history.
    pub fn local_position(&self) -> LocalPosition {
        LocalPosition::from_parts(
            Position::install(self.state.clone()).expect("native state"),
            self.protection.clone(),
        )
        .expect("native rights")
    }
}
/// Fresh never-rewound native execution authority, not serialized game history.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeToken {
    instance: u64,
    revision: u64,
}
/// Complete native context plus runtime authority. Outer purpose/process/limits belong78/43.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameContext {
    root: NativeRoot,
    events: Vec<GameEvent>,
    runtime: RuntimeToken,
}
impl GameContext {
    /// Compare canonical semantics independently of runtime retirement.
    pub fn same_semantics(&self, other: &Self) -> bool {
        self.root == other.root && self.events == other.events
    }
    /// Native runtime identity, for exact stale-result tests/technical details.
    pub fn runtime(&self) -> RuntimeToken {
        self.runtime
    }
}
/// Context-bound request; wrong actor/history/retired work is refused atomically.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRequest {
    /// Complete expected native context.
    pub expected: GameContext,
    /// Actual participant.
    pub actor: Side,
    /// Complete command.
    pub command: GameCommand,
}
/// Exhausted resource, distinct from illegal actions and game outcomes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resource {
    /// Encoded record bytes.
    EncodedBytes,
    /// Native events.
    Events,
    /// Bounded deterministic work.
    ReplayWork,
    /// Counter/runtime identity.
    Counter,
}
/// Atomic native refusal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameError {
    /// Invalid structural/rights root.
    Root,
    /// Played positions cannot reset their known history.
    NotRoot,
    /// Full submitted context does not match.
    StaleContext,
    /// Play actor differs from native turn.
    WrongActor,
    /// Native game is concluded.
    Concluded,
    /// Local complete-action refusal.
    Action(ActionError),
    /// Must vary before fourth.
    RepetitionDeparture,
    /// Missing/wrong/same-side draw acceptance or refusal.
    NoMatchingOffer,
    /// Repeated offer/stop without new work.
    DuplicateControl,
    /// Resource refusal, unscored.
    Capacity(Resource),
    /// Wrong/used reversal or prefix.
    InvalidToken,
}
impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chu native request refused: {self:?}")
    }
}
impl std::error::Error for GameError {}
/// Exact native resource usage; outer desktop limits remain78.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapacityUsage {
    /// Full encoded native record size.
    pub encoded_bytes: usize,
    /// Native event count.
    pub events: usize,
    /// Deterministic replay work.
    pub replay_work: u64,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct NativeRoot {
    pub state: StructuralState,
    pub protection: LionProtection,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Ply {
    actor: Side,
    action: Action,
    quiet_return: bool,
    check: bool,
    attack: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct GameData {
    root: NativeRoot,
    local: LocalPosition,
    events: Vec<GameEvent>,
    keys: Vec<RepetitionKey>,
    plies: Vec<Ply>,
    snapshots: Vec<HistorySnapshot>,
    root_snapshot: HistorySnapshot,
    work: u64,
}
/// Single-use LIFO reversal of every logical component/cache, never of runtime identity.
#[derive(Debug)]
pub struct GameUndo {
    instance: u64,
    before_node: u64,
    after_node: u64,
    before: GameData,
    used: bool,
}
/// Single-use semantic redo; intervening commands retire its branch.
#[derive(Debug)]
pub struct GameRedo {
    instance: u64,
    branch: u64,
    before_node: u64,
    after_node: u64,
    after: GameData,
    used: bool,
}
/// Sole authoritative native Chu game owner.
#[derive(Debug)]
pub struct Game {
    data: GameData,
    runtime: RuntimeToken,
    node: u64,
    next_node: u64,
    branch: u64,
}
impl Game {
    /// Standard game, empty known history.
    pub fn standard() -> Self {
        Self::from_root(Position::standard()).expect("standard native root")
    }
    /// Admit a diagram with no invented prior rights/occurrences. Root outcomes are derived.
    pub fn from_root(position: Position) -> Result<Self, GameError> {
        Self::from_local_root(LocalPosition::admit(position))
    }
    /// Admit explicit native root rights; imported rights must already pass native validation.
    pub fn from_local_root(local: LocalPosition) -> Result<Self, GameError> {
        if local.position().accepted_plies() != 0 {
            return Err(GameError::NotRoot);
        }
        if local.position().root().kind() == RootKind::Standard
            && (local.position() != &Position::standard()
                || local.protection() != &LionProtection::empty())
        {
            return Err(GameError::Root);
        }
        if royals(&local, Side::Sente).is_empty() && royals(&local, Side::Gote).is_empty() {
            return Err(GameError::Root);
        }
        let instance = NEXT_INSTANCE
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |v| v.checked_add(1))
            .map_err(|_| GameError::Capacity(Resource::Counter))?;
        let root = NativeRoot {
            state: local.position().state().clone(),
            protection: local.protection().clone(),
        };
        let snapshot = HistorySnapshot {
            state: root.state.clone(),
            protection: root.protection.clone(),
            event_count: 0,
            board_ply: 0,
            occurrences: vec![0],
            obligations: vec![],
            conclusion: None,
            draw_offer: None,
            incomplete: None,
        };
        let mut game = Self {
            data: GameData {
                root,
                keys: vec![RepetitionKey::from_local(&local)],
                local,
                events: vec![],
                plies: vec![],
                snapshots: vec![],
                root_snapshot: snapshot,
                work: 1,
            },
            runtime: RuntimeToken {
                instance,
                revision: 0,
            },
            node: 0,
            next_node: 1,
            branch: 0,
        };
        game.data.root_snapshot.conclusion = game.adjudicate(true)?;
        game.enforce_capacity(true)?;
        Ok(game)
    }
    /// Current native board/provenance.
    pub fn position(&self) -> &Position {
        self.data.local.position()
    }
    /// Current local owner; local choices are not live game admission.
    pub fn local_position(&self) -> &LocalPosition {
        &self.data.local
    }
    /// Exact accepted event history.
    pub fn events(&self) -> &[GameEvent] {
        &self.data.events
    }
    /// Current immutable full native snapshot.
    pub fn snapshot(&self) -> &HistorySnapshot {
        self.data
            .snapshots
            .last()
            .unwrap_or(&self.data.root_snapshot)
    }
    /// Current actual conclusion, never an operation failure.
    pub fn conclusion(&self) -> Option<Conclusion> {
        self.snapshot().conclusion
    }
    /// Current must-vary witnesses.
    pub fn obligations(&self) -> &[Obligation] {
        &self.snapshot().obligations
    }
    /// Current known repetition occurrences.
    pub fn occurrences(&self) -> &[usize] {
        &self.snapshot().occurrences
    }
    /// Current repetition projection, distinct from full saved/search context.
    pub fn repetition_key(&self) -> &RepetitionKey {
        self.data.keys.last().expect("root")
    }
    /// Native full-history and runtime identity.
    pub fn context(&self) -> GameContext {
        GameContext {
            root: self.data.root.clone(),
            events: self.data.events.clone(),
            runtime: self.runtime,
        }
    }
    /// Actual board status; no-action alone is unscored.
    pub fn status(&self) -> GameStatus {
        if self.conclusion().is_some() {
            GameStatus::Concluded
        } else if self.legal_actions().is_empty() {
            GameStatus::NoBoardActions
        } else {
            GameStatus::Ongoing
        }
    }
    /// Actual remaining royal squares for a native side.
    pub fn royal_squares(&self, side: Side) -> Vec<Square> {
        royals(&self.data.local, side)
    }
    /// Actual complete board choices filtered by full native history admission.
    pub fn legal_actions(&self) -> Vec<Action> {
        if self.conclusion().is_some() {
            return vec![];
        }
        self.data
            .local
            .legal_actions()
            .into_iter()
            .filter(|a| self.history_permits(a))
            .collect()
    }
    fn history_permits(&self, action: &Action) -> bool {
        if self.obligations().is_empty() {
            return true;
        }
        let Ok((local, _)) = self.data.local.simulate(action) else {
            return false;
        };
        let next = RepetitionKey::from_local(&local);
        self.permits_next(&next, self.position().side_to_move())
    }
    fn permits_next(&self, next: &RepetitionKey, actor: Side) -> bool {
        self.obligations().iter().all(|o| {
            let [_, b, c] = o.occurrences;
            let period = c - b;
            if o.progress >= period || next != &self.data.keys[b + o.progress + 1] {
                return true;
            }
            let last = (0..period).rfind(|i| self.data.keys[b + i].side == actor);
            !(o.responsible_sides.contains(&actor) && last == Some(o.progress))
        })
    }
    fn current_occurrences(&self) -> Vec<usize> {
        self.data
            .keys
            .iter()
            .enumerate()
            .filter_map(|(i, k)| (k == self.repetition_key()).then_some(i))
            .collect()
    }
    fn charge(&mut self, n: u64) -> Result<(), GameError> {
        self.data.work = self
            .data
            .work
            .checked_add(n)
            .filter(|w| *w <= MAX_REPLAY_WORK)
            .ok_or(GameError::Capacity(Resource::ReplayWork))?;
        Ok(())
    }
    fn derive_obligations(&self, previous: &[Obligation]) -> Vec<Obligation> {
        let n = self.data.plies.len();
        let mut out: Vec<_> = previous
            .iter()
            .filter_map(|o| {
                let [_, b, c] = o.occurrences;
                if o.progress + 1 < c - b && self.data.keys[n] == self.data.keys[b + o.progress + 1]
                {
                    let mut o = o.clone();
                    o.progress += 1;
                    Some(o)
                } else {
                    None
                }
            })
            .collect();
        for period in 2..=n / 2 {
            let a = n - 2 * period;
            let b = n - period;
            if self.data.keys[a..=b] != self.data.keys[b..=n] {
                continue;
            }
            let (responsible_sides, category) = self.cycle_responsibility(b, n);
            let o = Obligation {
                occurrences: [a, b, n],
                progress: 0,
                responsible_sides,
                category,
                closing_mover: self.data.plies[n - 1].actor,
            };
            if !out.contains(&o) {
                out.push(o);
            }
        }
        out
    }
    pub(super) fn cycle_responsibility(&self, b: usize, n: usize) -> (Vec<Side>, Responsibility) {
        let segment = &self.data.plies[b..n];
        let both_pass = segment.iter().all(|p| p.quiet_return);
        let checks = [Side::Sente, Side::Gote]
            .map(|s| segment.iter().filter(|p| p.actor == s).all(|p| p.check));
        let attacks = [Side::Sente, Side::Gote]
            .map(|s| segment.iter().filter(|p| p.actor == s).all(|p| p.attack));
        if both_pass {
            let start = self.data.plies[..n]
                .iter()
                .rposition(|p| !p.quiet_return)
                .map_or(0, |i| i + 1);
            (
                vec![self.data.plies[start].actor],
                Responsibility::FirstQuietReturnMover,
            )
        } else if checks[0] || checks[1] {
            (
                [Side::Sente, Side::Gote]
                    .into_iter()
                    .filter(|s| checks[s.index()])
                    .collect(),
                if checks[0] && checks[1] {
                    Responsibility::ContinuousCheckers
                } else {
                    Responsibility::SoleContinuousChecker
                },
            )
        } else if attacks[0] != attacks[1] {
            (
                vec![if attacks[0] { Side::Sente } else { Side::Gote }],
                Responsibility::SoleAttacker,
            )
        } else {
            (
                vec![self.data.keys[n].side],
                Responsibility::SideToMoveFallback,
            )
        }
    }
    /// Atomic context-bound participant command. Resource refusal preserves all state.
    pub fn submit(&mut self, request: GameRequest) -> Result<GameUndo, GameError> {
        if request.expected != self.context() {
            return Err(GameError::StaleContext);
        }
        self.commit(request.actor, request.command)
    }
    fn commit(&mut self, actor: Side, command: GameCommand) -> Result<GameUndo, GameError> {
        if self.conclusion().is_some() {
            return Err(GameError::Concluded);
        }
        let revision = self
            .runtime
            .revision
            .checked_add(1)
            .ok_or(GameError::Capacity(Resource::Counter))?;
        let node = self.next_node;
        let next_node = node
            .checked_add(1)
            .ok_or(GameError::Capacity(Resource::Counter))?;
        let branch = self
            .branch
            .checked_add(1)
            .ok_or(GameError::Capacity(Resource::Counter))?;
        let mut candidate = self.transaction();
        candidate.charge(1)?;
        let previous = self.snapshot().clone();
        let mut next = previous.clone();
        let mut effects = None;
        match &command {
            GameCommand::Play { action } => {
                if actor != self.position().side_to_move() {
                    return Err(GameError::WrongActor);
                }
                let influence_before = self.data.local.capture_influence(actor);
                let (e, _) = candidate.data.local.apply(action).map_err(|e| {
                    if e == ActionError::Capacity {
                        GameError::Capacity(Resource::Counter)
                    } else {
                        GameError::Action(e)
                    }
                })?;
                let key = RepetitionKey::from_local(&candidate.data.local);
                if !self.permits_next(&key, actor) {
                    return Err(GameError::RepetitionDeparture);
                }
                let influence_after = candidate.data.local.capture_influence(actor);
                let moved_to = *action.route.last().expect("native admitted route");
                let attack = influence_after
                    .iter()
                    .any(|pair| pair.0 == moved_to || !influence_before.contains(pair));
                let enemy_royals = royals(&candidate.data.local, actor.opponent());
                let check = enemy_royals.len() == 1
                    && influence_after.iter().any(|p| p.1 == enemy_royals[0]);
                candidate.data.plies.push(Ply {
                    actor,
                    action: action.clone(),
                    quiet_return: e.quiet_return,
                    check,
                    attack,
                });
                candidate.data.keys.push(key);
                let n = candidate.data.keys.len() as u64;
                candidate.charge(n * n)?;
                next.occurrences = candidate.current_occurrences();
                next.obligations = candidate.derive_obligations(&previous.obligations);
                next.draw_offer = None;
                next.incomplete = None;
                // Install history for finite reply checks, without recursive mate adjudication.
                next.state = candidate.position().state().clone();
                next.protection = candidate.data.local.protection().clone();
                next.event_count = self.events().len() + 1;
                next.board_ply = candidate.data.plies.len();
                candidate.data.snapshots.push(next.clone());
                next.conclusion = candidate.adjudicate(true)?;
                candidate.data.snapshots.pop();
                effects = Some(e);
            }
            GameCommand::Resign => {
                next.conclusion = Some(Conclusion::Resignation {
                    winner: actor.opponent(),
                })
            }
            GameCommand::OfferDraw => {
                if previous.draw_offer.is_some() {
                    return Err(GameError::DuplicateControl);
                }
                next.draw_offer = Some(actor);
            }
            GameCommand::AcceptDraw => {
                if previous.draw_offer != Some(actor.opponent()) {
                    return Err(GameError::NoMatchingOffer);
                }
                next.conclusion = Some(Conclusion::AgreedDraw);
                next.draw_offer = None;
            }
            GameCommand::DeclineDraw => {
                if previous.draw_offer != Some(actor.opponent()) {
                    return Err(GameError::NoMatchingOffer);
                }
                next.draw_offer = None;
            }
            GameCommand::Stop { reason } => {
                if previous.incomplete.is_some() {
                    return Err(GameError::DuplicateControl);
                }
                next.incomplete = Some(*reason);
            }
        }
        next.state = candidate.position().state().clone();
        next.protection = candidate.data.local.protection().clone();
        next.event_count = self.events().len() + 1;
        next.board_ply = candidate.data.plies.len();
        candidate.data.events.push(GameEvent {
            index: self.events().len(),
            board_ply: next.board_ply,
            actor,
            command: command.clone(),
            effects,
            occurrences: next.occurrences.clone(),
            obligations: next.obligations.clone(),
            conclusion: next.conclusion,
            draw_offer: next.draw_offer,
            incomplete: next.incomplete,
            replay_work: candidate.data.work,
        });
        candidate.data.snapshots.push(next);
        let reserved = matches!(
            command,
            GameCommand::Resign | GameCommand::AcceptDraw | GameCommand::Stop { .. }
        );
        candidate.enforce_capacity(!reserved)?;
        let undo = GameUndo {
            instance: self.runtime.instance,
            before_node: self.node,
            after_node: node,
            before: self.data.clone(),
            used: false,
        };
        self.data = candidate.data;
        self.runtime.revision = revision;
        self.node = node;
        self.next_node = next_node;
        self.branch = branch;
        Ok(undo)
    }
    fn transaction(&self) -> Self {
        Self {
            data: self.data.clone(),
            runtime: self.runtime,
            node: self.node,
            next_node: self.next_node,
            branch: self.branch,
        }
    }
    fn enforce_capacity(&self, reserve: bool) -> Result<(), GameError> {
        let limit = MAX_GAME_EVENTS - if reserve { RESERVED_EVENTS } else { 0 };
        if self.events().len() > limit {
            return Err(GameError::Capacity(Resource::Events));
        }
        self.check_encoded_capacity(reserve, MAX_GAME_RECORD_BYTES)?;
        if self.data.work > MAX_REPLAY_WORK - if reserve { RESERVED_EVENTS as u64 } else { 0 } {
            return Err(GameError::Capacity(Resource::ReplayWork));
        }
        Ok(())
    }
    fn check_encoded_capacity(&self, reserve: bool, limit: usize) -> Result<(), GameError> {
        let bytes = super::record::encode(self)
            .map_err(|_| GameError::Capacity(Resource::EncodedBytes))?
            .len();
        if bytes > limit || reserve && self.encoded_with_reservations()? > limit {
            return Err(GameError::Capacity(Resource::EncodedBytes));
        }
        Ok(())
    }
    fn encoded_with_reservations(&self) -> Result<usize, GameError> {
        // Encode the real complete candidate plus Stop+Resign. Obligations can be
        // variable-size; a fixed byte guess would not guarantee closing headroom.
        let mut reserved = self.transaction();
        for (actor, command) in [
            (
                Side::Sente,
                GameCommand::Stop {
                    reason: StopReason::ResourceLimit,
                },
            ),
            (Side::Gote, GameCommand::Resign),
        ] {
            let mut snapshot = reserved.snapshot().clone();
            snapshot.event_count += 1;
            match command {
                GameCommand::Stop { reason } => snapshot.incomplete = Some(reason),
                GameCommand::Resign => {
                    snapshot.conclusion = Some(Conclusion::Resignation {
                        winner: actor.opponent(),
                    })
                }
                _ => unreachable!(),
            }
            reserved.data.work += 1;
            reserved.data.events.push(GameEvent {
                index: reserved.events().len(),
                board_ply: snapshot.board_ply,
                actor,
                command,
                effects: None,
                occurrences: snapshot.occurrences.clone(),
                obligations: snapshot.obligations.clone(),
                conclusion: snapshot.conclusion,
                draw_offer: snapshot.draw_offer,
                incomplete: snapshot.incomplete,
                replay_work: reserved.data.work,
            });
            reserved.data.snapshots.push(snapshot);
        }
        Ok(super::record::encode(&reserved)
            .map_err(|_| GameError::Capacity(Resource::EncodedBytes))?
            .len())
    }
    /// Exact logical undo; runtime revision advances and retired work stays retired.
    pub fn undo(&mut self, token: &mut GameUndo) -> Result<GameRedo, GameError> {
        if token.used || token.instance != self.runtime.instance || token.after_node != self.node {
            return Err(GameError::InvalidToken);
        }
        let revision = self
            .runtime
            .revision
            .checked_add(1)
            .ok_or(GameError::Capacity(Resource::Counter))?;
        let node = self.next_node;
        let next = node
            .checked_add(1)
            .ok_or(GameError::Capacity(Resource::Counter))?;
        let redo = GameRedo {
            instance: self.runtime.instance,
            branch: self.branch,
            before_node: token.before_node,
            after_node: self.node,
            after: self.data.clone(),
            used: false,
        };
        self.data = token.before.clone();
        self.node = token.before_node;
        self.next_node = next;
        self.runtime.revision = revision;
        token.used = true;
        Ok(redo)
    }
    /// Restore exact semantic post-state while advancing authority; wrong branches reject.
    pub fn redo(&mut self, token: &mut GameRedo) -> Result<GameUndo, GameError> {
        if token.used
            || token.instance != self.runtime.instance
            || token.branch != self.branch
            || token.before_node != self.node
        {
            return Err(GameError::InvalidToken);
        }
        let revision = self
            .runtime
            .revision
            .checked_add(1)
            .ok_or(GameError::Capacity(Resource::Counter))?;
        let node = self.next_node;
        let next = node
            .checked_add(1)
            .ok_or(GameError::Capacity(Resource::Counter))?;
        let undo = GameUndo {
            instance: self.runtime.instance,
            before_node: self.node,
            after_node: token.after_node,
            before: self.data.clone(),
            used: false,
        };
        self.data = token.after.clone();
        self.node = token.after_node;
        self.next_node = next;
        self.runtime.revision = revision;
        token.used = true;
        Ok(undo)
    }
    /// Immutable exact event prefix; no future obligations or outcomes.
    pub fn event_prefix(&self, count: usize) -> Result<HistorySnapshot, GameError> {
        if count == 0 {
            Ok(self.data.root_snapshot.clone())
        } else {
            self.data
                .snapshots
                .get(count - 1)
                .cloned()
                .ok_or(GameError::InvalidToken)
        }
    }
    /// Explicit selection retires work without changing the live history.
    pub fn select_prefix(&mut self, count: usize) -> Result<HistorySnapshot, GameError> {
        let s = self.event_prefix(count)?;
        self.retire_context()?;
        Ok(s)
    }
    /// Return-to-live/cancellation retirement, never semantic rewind.
    pub fn retire_context(&mut self) -> Result<(), GameError> {
        self.runtime.revision = self
            .runtime
            .revision
            .checked_add(1)
            .ok_or(GameError::Capacity(Resource::Counter))?;
        Ok(())
    }
    /// Canonical exact event prefix; reopening always receives fresh native authority.
    pub fn record_prefix(&self, count: usize) -> Result<GameRecord, RecordError> {
        self.event_prefix(count).map_err(|_| RecordError::Prefix)?;
        let mut game =
            Self::from_native_root(self.data.root.clone()).map_err(|_| RecordError::Root)?;
        for e in &self.events()[..count] {
            game.commit(e.actor, e.command.clone())
                .map_err(|_| RecordError::Derived)?;
        }
        GameRecord::from_game(&game)
    }
    pub(super) fn from_native_root(root: NativeRoot) -> Result<Self, GameError> {
        let p = Position::install(root.state).map_err(|_| GameError::Root)?;
        let l = LocalPosition::from_parts(p, root.protection).map_err(|_| GameError::Root)?;
        Self::from_local_root(l)
    }
    pub(super) fn native_root(&self) -> &NativeRoot {
        &self.data.root
    }
    pub(super) fn root_snapshot(&self) -> &HistorySnapshot {
        &self.data.root_snapshot
    }
    /// Complete encoded native size, events and bounded work.
    pub fn capacity_usage(&self) -> Result<CapacityUsage, GameError> {
        Ok(CapacityUsage {
            encoded_bytes: super::record::encode(self)
                .map_err(|_| GameError::Capacity(Resource::EncodedBytes))?
                .len(),
            events: self.events().len(),
            replay_work: self.data.work,
        })
    }
    /// Native turn-appropriate material exception replies, not general win search.
    pub fn immediate_material_replies(&self) -> Vec<Action> {
        let Some((_, bare)) = material_configuration(&self.data.local) else {
            return vec![];
        };
        if bare != self.position().side_to_move() {
            return vec![];
        }
        self.data
            .local
            .legal_actions()
            .into_iter()
            .filter(|a| {
                if !self.history_permits(a) {
                    return false;
                }
                let Ok((local, _)) = self.data.local.simulate(a) else {
                    return false;
                };
                royals(&local, bare.opponent()).is_empty()
                    || counted_pieces(&local).iter().all(|(_, p)| p.is_royal())
                        && counted_pieces(&local).len() == 2
            })
            .collect()
    }
    fn adjudicate(&mut self, mate: bool) -> Result<Option<Conclusion>, GameError> {
        let side = self.position().side_to_move();
        for loser in [side, side.opponent()] {
            if royals(&self.data.local, loser).is_empty() {
                return Ok(Some(Conclusion::RoyalCapture {
                    winner: loser.opponent(),
                }));
            }
        }
        let mut threat_work = 0;
        let threatened = mate
            && royals(&self.data.local, side).len() == 1
            && self.capture_all_counted(side, &mut threat_work)?;
        self.charge(threat_work)?;
        if threatened {
            let replies = self.data.local.legal_actions();
            self.charge(replies.len() as u64)?;
            let mut escaped = false;
            for action in replies {
                if !self.history_permits(&action) {
                    continue;
                }
                let mut reply = self.transaction();
                reply.apply_for_reply(&action)?;
                let before = reply.data.work;
                let non_mate = reply.adjudicate(false)?;
                let mut work = 0;
                let captured = if non_mate.is_some() {
                    false
                } else {
                    reply.capture_all_counted(side, &mut work)?
                };
                self.charge(reply.data.work - before + work)?;
                if non_mate.is_some() || !captured {
                    escaped = true;
                    break;
                }
            }
            if !escaped {
                return Ok(Some(Conclusion::Mate {
                    winner: side.opponent(),
                }));
            }
        }
        if let Some((winner, _)) = material_configuration(&self.data.local)
            && self.immediate_material_replies().is_empty()
        {
            return Ok(Some(Conclusion::Material { winner }));
        }
        Ok(None)
    }
    fn apply_for_reply(&mut self, action: &Action) -> Result<(), GameError> {
        let before = self.snapshot().clone();
        let actor = self.position().side_to_move();
        let influence_before = self.data.local.capture_influence(actor);
        let (next_local, e) = self
            .data
            .local
            .simulate(action)
            .map_err(GameError::Action)?;
        self.data.local = next_local;
        let influence_after = self.data.local.capture_influence(actor);
        let moved_to = *action.route.last().expect("native route");
        let attack = influence_after
            .iter()
            .any(|p| p.0 == moved_to || !influence_before.contains(p));
        let enemy = royals(&self.data.local, actor.opponent());
        let check = enemy.len() == 1 && influence_after.iter().any(|p| p.1 == enemy[0]);
        self.data.plies.push(Ply {
            actor,
            action: action.clone(),
            quiet_return: e.quiet_return,
            check,
            attack,
        });
        self.data
            .keys
            .push(RepetitionKey::from_local(&self.data.local));
        let mut next = before.clone();
        next.state = self.position().state().clone();
        next.protection = self.data.local.protection().clone();
        next.board_ply += 1;
        next.occurrences = self.current_occurrences();
        next.obligations = self.derive_obligations(&before.obligations);
        next.conclusion = None;
        next.draw_offer = None;
        next.incomplete = None;
        self.data.snapshots.push(next);
        Ok(())
    }
    /// A single admitted opponent action capturing all royals, not separate attacks/minimax.
    pub fn has_capture_all(&self, victim: Side) -> Result<bool, GameError> {
        self.capture_all_counted(victim, &mut 0)
    }
    fn capture_all_counted(&self, victim: Side, work: &mut u64) -> Result<bool, GameError> {
        let targets = royals(&self.data.local, victim);
        if targets.is_empty() {
            return Ok(false);
        }
        let actions = self.data.local.legal_actions_for(victim.opponent());
        *work = work
            .checked_add(actions.len() as u64)
            .filter(|w| *w <= MAX_REPLAY_WORK)
            .ok_or(GameError::Capacity(Resource::ReplayWork))?;
        for action in actions {
            // A hypothetical future turn with the same placement must respect native history.
            // At actual replies the opponent is side-to-move; root threat is advisory geometry.
            let local = &self.data.local;
            let effects = local.preview_for(victim.opponent(), &action);
            if let Ok(e) = effects
                && targets
                    .iter()
                    .all(|s| e.captures.iter().any(|c| c.square == *s))
                && (victim.opponent() != self.position().side_to_move()
                    || self.history_permits(&action))
            {
                return Ok(true);
            }
        }
        Ok(false)
    }
    #[cfg(test)]
    pub(super) fn cache_snapshot(&self) -> String {
        format!(
            "{:?}",
            (
                &self.data.root,
                self.position().state(),
                self.data.local.protection(),
                &self.data.events,
                &self.data.keys,
                &self.data.plies,
                &self.data.snapshots,
                &self.data.root_snapshot,
                self.data.work
            )
        )
    }
}
fn royals(local: &LocalPosition, side: Side) -> Vec<Square> {
    local
        .position()
        .pieces()
        .filter_map(|(s, p)| (p.side == side && p.is_royal()).then_some(s))
        .collect()
}
/// Pieces ignored only for the narrow material count remain physically on the board.
pub fn dead_for_material(square: Square, piece: Piece) -> bool {
    !piece.promoted
        && matches!(piece.base, Base::Pawn | Base::Lance)
        && square.rank() == if piece.side == Side::Sente { 1 } else { 12 }
}
fn counted_pieces(local: &LocalPosition) -> Vec<(Square, Piece)> {
    local
        .position()
        .pieces()
        .filter(|(s, p)| !dead_for_material(*s, *p))
        .collect()
}
fn material_configuration(local: &LocalPosition) -> Option<(Side, Side)> {
    let pieces = counted_pieces(local);
    if pieces.len() != 3 {
        return None;
    }
    for bare in [Side::Sente, Side::Gote] {
        let one: Vec<_> = pieces.iter().filter(|(_, p)| p.side == bare).collect();
        let two: Vec<_> = pieces
            .iter()
            .filter(|(_, p)| p.side == bare.opponent())
            .collect();
        if one.len() == 1
            && one[0].1.is_royal()
            && two.len() == 2
            && two.iter().any(|(_, p)| p.is_royal())
            && two
                .iter()
                .all(|(_, p)| p.promoted || !matches!(p.base, Base::Pawn | Base::GoBetween))
        {
            return Some((bare.opponent(), bare));
        }
    }
    None
}

#[cfg(test)]
mod capacity_tests {
    use super::*;
    #[test]
    fn actual_encoded_reservations_cover_every_actor_and_exact_byte_boundary() {
        let f=super::super::conformance::root(&serde_json::json!({"board":[{"at":[1,12],"side":"sente","piece":"K"},{"at":[3,12],"side":"sente","piece":"G"},{"at":[12,1],"side":"gote","piece":"K"},{"at":[10,1],"side":"gote","piece":"G"}],"turn":"sente","declared_board_plies":0})).unwrap();
        let mut game = Game::from_root(f).unwrap();
        let square = |f, r| Square::new(f, r).unwrap();
        let cycle = [
            Action::single(square(1, 12), square(1, 11)),
            Action::single(square(12, 1), square(12, 2)),
            Action::single(square(1, 11), square(1, 12)),
            Action::single(square(12, 2), square(12, 1)),
        ];
        for a in cycle.iter().cycle().take(10) {
            game.commit(
                game.position().side_to_move(),
                GameCommand::Play { action: a.clone() },
            )
            .unwrap();
        }
        assert!(game.obligations().len() >= 3);
        let current = super::super::record::encode(&game).unwrap().len();
        let reserved = game.encoded_with_reservations().unwrap();
        let mut largest = 0;
        for stop_actor in [Side::Sente, Side::Gote] {
            for resign_actor in [Side::Sente, Side::Gote] {
                let mut actual = game.transaction();
                actual
                    .commit(
                        stop_actor,
                        GameCommand::Stop {
                            reason: StopReason::ResourceLimit,
                        },
                    )
                    .unwrap();
                actual.commit(resign_actor, GameCommand::Resign).unwrap();
                largest = largest.max(super::super::record::encode(&actual).unwrap().len());
            }
        }
        assert_eq!(reserved, largest);
        assert!(reserved > current);
        assert_eq!(game.check_encoded_capacity(false, current), Ok(()));
        assert_eq!(
            game.check_encoded_capacity(false, current - 1),
            Err(GameError::Capacity(Resource::EncodedBytes))
        );
        assert_eq!(game.check_encoded_capacity(true, reserved), Ok(()));
        assert_eq!(
            game.check_encoded_capacity(true, reserved - 1),
            Err(GameError::Capacity(Resource::EncodedBytes))
        );
        eprintln!(
            "Chu obligation headroom: {} witnesses, {} current bytes, {} closing bytes",
            game.obligations().len(),
            current,
            reserved - current
        );
    }
}

#[cfg(test)]
mod attack_metadata_tests {
    use super::*;
    fn position(blocker: Square) -> Game {
        let s = |f, r| Square::new(f, r).unwrap();
        let entries = [
            (s(1, 12), Piece::from_code(Side::Sente, "K").unwrap()),
            (s(6, 8), Piece::from_code(Side::Sente, "R").unwrap()),
            (blocker, Piece::from_code(Side::Sente, "G").unwrap()),
            (s(12, 1), Piece::from_code(Side::Gote, "K").unwrap()),
            (s(6, 3), Piece::from_code(Side::Gote, "Si").unwrap()),
        ];
        Game::from_root(
            Position::from_diagram(&entries, Side::Sente, RootMetadata::diagram(0)).unwrap(),
        )
        .unwrap()
    }
    #[test]
    fn discovered_attack_counts_but_unchanged_incidental_attack_does_not() {
        let s = |f, r| Square::new(f, r).unwrap();
        let mut discovered = position(s(6, 6));
        discovered
            .commit(
                Side::Sente,
                GameCommand::Play {
                    action: Action::single(s(6, 6), s(5, 5)),
                },
            )
            .unwrap();
        assert!(discovered.data.plies[0].attack);
        assert!(
            discovered
                .local_position()
                .capture_influence(Side::Sente)
                .contains(&(s(6, 8), s(6, 3)))
        );
        let mut incidental = position(s(2, 12));
        incidental
            .commit(
                Side::Sente,
                GameCommand::Play {
                    action: Action::single(s(2, 12), s(3, 11)),
                },
            )
            .unwrap();
        assert!(!incidental.data.plies[0].attack);
        assert!(
            incidental
                .local_position()
                .capture_influence(Side::Sente)
                .contains(&(s(6, 8), s(6, 3)))
        );
    }
}
