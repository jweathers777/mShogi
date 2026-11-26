//! Complete Chu game admission. Local geometry never grants live history permission.
use super::*;
use crate::native::history::{AppendGame, GameHistory, Snapshot};
pub use crate::native::{
    GameContext,
    history::{CapacityUsage, GameUndo},
};
use crate::repetition::RepetitionIndex;
use serde::{Deserialize, Serialize};

/// Repetition projection: native original/once-promoted placement, turn and Lion rights.
/// Counters, provenance, derived obligations, events and runtime identities are excluded.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
}
/// Immutable logical position/history view with no live command authority.
/// Untrusted structural data cannot overwrite a native historical view:
/// ```compile_fail,E0616
/// let mut view = mshogi_core::chu::Game::standard().event_prefix(0).unwrap();
/// view.state = serde_json::from_str("{}").unwrap();
/// ```
/// Nor can callers install arbitrary Lion rights:
/// ```compile_fail,E0616
/// let mut view = mshogi_core::chu::Game::standard().event_prefix(0).unwrap();
/// view.protection = Default::default();
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct HistorySnapshot {
    /// Complete structural state/provenance/counters.
    pub(super) state: StructuralState,
    /// Native next-turn Lion rights.
    pub(super) protection: LionProtection,
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
}
impl HistorySnapshot {
    /// Immutable native-validated structural state at this exact prefix.
    pub fn state(&self) -> &StructuralState {
        &self.state
    }
    /// Immutable native-derived next-turn Lion protection at this prefix.
    pub fn protection(&self) -> &LionProtection {
        &self.protection
    }
    /// Immutable native local inspection with this prefix's rights and no future history.
    pub fn local_position(&self) -> LocalPosition {
        LocalPosition::from_parts(
            Position::install(self.state.clone()).expect("native state"),
            self.protection.clone(),
        )
        .expect("native rights")
    }
}
/// Context-bound request; wrong actor/history/retired work is refused atomically.
pub type GameRequest = crate::native::history::Request<Side, GameCommand>;
/// Exhausted resource, distinct from illegal actions and game outcomes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resource {
    /// Encoded native root bytes at admission.
    EncodedBytes,
    /// Declared board counter.
    Counter,
}
/// Atomic native refusal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameError {
    /// Invalid structural/rights root.
    Root,
    /// Played positions cannot reset their known history.
    NotRoot,
    /// The request token is not the current one.
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
    /// Repeated draw offer without a response.
    DuplicateControl,
    /// Resource refusal, unscored.
    Capacity(Resource),
    /// Wrong/used reversal or prefix.
    InvalidToken,
    /// Caller explicitly stopped a cancellable query or reconstruction.
    Stopped,
}
impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chu native request refused: {self:?}")
    }
}
impl std::error::Error for GameError {}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
/// Chu root position and initial Lion protection.
pub struct NativeRoot {
    pub(super) state: StructuralState,
    pub(super) protection: LionProtection,
}
#[derive(Clone, Debug, Eq, PartialEq)]
/// Rules evidence for an accepted Chu board action.
#[doc(hidden)]
pub struct Ply {
    actor: Side,
    action: Action,
    quiet_return: bool,
    check: bool,
    attack: bool,
}
/// Authoritative Chu history and adjudication.
pub type Game = GameHistory<HistorySnapshot>;
impl Snapshot for HistorySnapshot {
    type Side = Side;
    type Command = GameCommand;
    const STALE: GameError = GameError::StaleContext;
    const LARGE_ROOT: GameError = GameError::Capacity(Resource::EncodedBytes);
    type Local = LocalPosition;
    type Key = RepetitionKey;
    type Ply = Ply;
    type Event = GameEvent;
    type Error = GameError;
    const INVALID_UNDO: GameError = GameError::InvalidToken;
    const INVALID_PREFIX: GameError = GameError::InvalidToken;
    fn board_ply(&self) -> usize {
        self.board_ply
    }
    fn restore_local(&self) -> LocalPosition {
        self.local_position()
    }
    fn encoded_size(game: &Game) -> Result<usize, GameError> {
        crate::record::compact::encode(game)
            .map(|text| text.len())
            .map_err(|_| GameError::Capacity(Resource::EncodedBytes))
    }
    fn step(
        game: &mut Game,
        actor: Side,
        command: GameCommand,
        _stop: &mut impl FnMut() -> bool,
    ) -> Result<(), GameError> {
        if game.conclusion().is_some() {
            return Err(GameError::Concluded);
        }
        let previous = game.snapshot().clone();
        let mut next = previous.clone();
        let mut effects = None;
        match &command {
            GameCommand::Play { action } => {
                if actor != game.position().side_to_move() {
                    return Err(GameError::WrongActor);
                }
                let influence_before = game.local.capture_influence(actor);
                let (e, _) = game.local.apply(action).map_err(|e| {
                    if e == ActionError::Capacity {
                        GameError::Capacity(Resource::Counter)
                    } else {
                        GameError::Action(e)
                    }
                })?;
                let key = RepetitionKey::from_local(&game.local);
                if !game.permits_next(&key, actor) {
                    return Err(GameError::RepetitionDeparture);
                }
                let influence_after = game.local.capture_influence(actor);
                let moved_to = *action.route.last().expect("native admitted route");
                let attack = influence_after
                    .iter()
                    .any(|pair| pair.0 == moved_to || !influence_before.contains(pair));
                let enemy_royals = royals(&game.local, actor.opponent());
                let check = enemy_royals.len() == 1
                    && influence_after.iter().any(|p| p.1 == enemy_royals[0]);
                game.plies.push(Ply {
                    actor,
                    action: action.clone(),
                    quiet_return: e.quiet_return,
                    check,
                    attack,
                });
                game.keys.push(key);
                next.occurrences = game.keys.current_occurrences();
                next.obligations =
                    game.derive_obligations(&previous.obligations, &next.occurrences);
                next.draw_offer = None;
                // Install history for finite reply checks, without recursive mate adjudication.
                next.state = game.position().state().clone();
                next.protection = game.local.protection().clone();
                next.event_count = game.events().len() + 1;
                next.board_ply = game.plies.len();
                game.snapshots.push(next.clone());
                next.conclusion = game.adjudicate(true)?;
                game.snapshots.pop();
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
        }
        next.state = game.position().state().clone();
        next.protection = game.local.protection().clone();
        next.event_count = game.events().len() + 1;
        next.board_ply = game.plies.len();
        game.events.push(GameEvent {
            index: game.events().len(),
            board_ply: next.board_ply,
            actor,
            command: command.clone(),
            effects,
            occurrences: next.occurrences.clone(),
            obligations: next.obligations.clone(),
            conclusion: next.conclusion,
            draw_offer: next.draw_offer,
        });
        game.snapshots.push(next);
        Ok(())
    }
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
        let snapshot = HistorySnapshot {
            state: local.position().state().clone(),
            protection: local.protection().clone(),
            event_count: 0,
            board_ply: 0,
            occurrences: vec![0],
            obligations: vec![],
            conclusion: None,
            draw_offer: None,
        };
        let keys = RepetitionIndex::new(RepetitionKey::from_local(&local));
        let mut game = Self::from_snapshot(local, keys, snapshot);
        game.root.conclusion = game.adjudicate(true)?;
        game.check_root_size()?;
        Ok(game)
    }
    /// Current native board/provenance.
    pub fn position(&self) -> &Position {
        self.local.position()
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
        royals(&self.local, side)
    }
    /// Actual complete board choices filtered by full native history admission.
    pub fn legal_actions(&self) -> Vec<Action> {
        self.legal_actions_with_stop(&mut || false)
            .expect("a query that is never stopped completes")
    }
    /// Cancellable native choices: the stop is checked between candidate actions.
    pub fn legal_actions_with_stop(
        &self,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Vec<Action>, GameError> {
        if stop() {
            return Err(GameError::Stopped);
        }
        if self.conclusion().is_some() {
            return Ok(vec![]);
        }
        let mut out = vec![];
        for action in self.local.legal_actions() {
            if stop() {
                return Err(GameError::Stopped);
            }
            if self.history_permits(&action) {
                out.push(action);
            }
        }
        Ok(out)
    }
    fn history_permits(&self, action: &Action) -> bool {
        if self.obligations().is_empty() {
            return true;
        }
        let Ok((local, _)) = self.local.simulate(action) else {
            return false;
        };
        let next = RepetitionKey::from_local(&local);
        self.permits_next(&next, self.position().side_to_move())
    }
    fn permits_next(&self, next: &RepetitionKey, actor: Side) -> bool {
        self.obligations().iter().all(|o| {
            let [_, b, c] = o.occurrences;
            let period = c - b;
            if o.progress >= period || next != &self.keys.keys()[b + o.progress + 1] {
                return true;
            }
            let last = (0..period).rfind(|i| self.keys.keys()[b + i].side == actor);
            !(o.responsible_sides.contains(&actor) && last == Some(o.progress))
        })
    }
    fn derive_obligations(
        &self,
        previous: &[Obligation],
        occurrences: &[usize],
    ) -> Vec<Obligation> {
        let keys = self.keys.keys();
        let n = self.plies.len();
        let mut out: Vec<_> = previous
            .iter()
            .filter_map(|o| {
                let [_, b, c] = o.occurrences;
                if o.progress + 1 < c - b && keys[n] == keys[b + o.progress + 1] {
                    let mut o = o.clone();
                    o.progress += 1;
                    Some(o)
                } else {
                    None
                }
            })
            .collect();
        // Each pair of equal cycles must return to the current key at its midpoint.
        // Walk nearest midpoints first to keep the shortest periods first.
        for &b in occurrences.iter().rev().skip(1) {
            let period = n - b;
            let Some(a) = b.checked_sub(period) else {
                break;
            };
            if period < 2 {
                continue;
            }
            if keys[a..=b] != keys[b..=n] {
                continue;
            }
            let (responsible_sides, category) = self.cycle_responsibility(b, n);
            let o = Obligation {
                occurrences: [a, b, n],
                progress: 0,
                responsible_sides,
                category,
                closing_mover: self.plies[n - 1].actor,
            };
            if !out.contains(&o) {
                out.push(o);
            }
        }
        out
    }
    pub(super) fn cycle_responsibility(&self, b: usize, n: usize) -> (Vec<Side>, Responsibility) {
        let segment = &self.plies[b..n];
        let both_pass = segment.iter().all(|p| p.quiet_return);
        let checks = [Side::Sente, Side::Gote]
            .map(|s| segment.iter().filter(|p| p.actor == s).all(|p| p.check));
        let attacks = [Side::Sente, Side::Gote]
            .map(|s| segment.iter().filter(|p| p.actor == s).all(|p| p.attack));
        if both_pass {
            let start = self.plies[..n]
                .iter()
                .rposition(|p| !p.quiet_return)
                .map_or(0, |i| i + 1);
            (
                vec![self.plies[start].actor],
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
                vec![self.keys.keys()[n].side],
                Responsibility::SideToMoveFallback,
            )
        }
    }
    /// `submit` behind a stop check: one Chu command is a bounded native step.
    pub fn submit_with_stop(
        &mut self,
        request: GameRequest,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<GameUndo, GameError> {
        if stop() {
            return Err(GameError::Stopped);
        }
        self.submit(request)
    }
    fn commit(&mut self, actor: Side, command: GameCommand) -> Result<GameUndo, GameError> {
        self.submit(GameRequest {
            expected: self.context(),
            actor,
            command,
        })
    }

    /// Canonical exact event prefix; reopening always receives fresh native authority.
    pub fn record_prefix(&self, count: usize) -> Result<GameRecord, RecordError> {
        self.record_prefix_with_stop(count, &mut || false)
    }
    /// Exact prefix with cooperative cancellation between replayed events.
    pub fn record_prefix_with_stop(
        &self,
        count: usize,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<GameRecord, RecordError> {
        self.event_prefix(count).map_err(|_| RecordError::Prefix)?;
        let mut game = Self::from_native_root(self.native_root()).map_err(|_| RecordError::Root)?;
        for e in &self.events()[..count] {
            if stop() {
                return Err(RecordError::Stopped);
            }
            game.commit(e.actor, e.command.clone())
                .map_err(|_| RecordError::Derived)?;
        }
        if stop() {
            return Err(RecordError::Stopped);
        }
        GameRecord::from_game(&game)
    }
    pub(super) fn from_native_root(root: NativeRoot) -> Result<Self, GameError> {
        let p = Position::install(root.state).map_err(|_| GameError::Root)?;
        let l = LocalPosition::from_parts(p, root.protection).map_err(|_| GameError::Root)?;
        Self::from_local_root(l)
    }
    pub(super) fn native_root(&self) -> NativeRoot {
        NativeRoot {
            state: self.root.state.clone(),
            protection: self.root.protection.clone(),
        }
    }
    /// Native turn-appropriate material exception replies, not general win search.
    pub fn immediate_material_replies(&self) -> Vec<Action> {
        let Some((_, bare)) = material_configuration(&self.local) else {
            return vec![];
        };
        if bare != self.position().side_to_move() {
            return vec![];
        }
        self.local
            .legal_actions()
            .into_iter()
            .filter(|a| {
                if !self.history_permits(a) {
                    return false;
                }
                let Ok((local, _)) = self.local.simulate(a) else {
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
            if royals(&self.local, loser).is_empty() {
                return Ok(Some(Conclusion::RoyalCapture {
                    winner: loser.opponent(),
                }));
            }
        }
        let threatened =
            mate && royals(&self.local, side).len() == 1 && self.has_capture_all(side)?;
        if threatened {
            let replies = self.local.legal_actions();
            let mut escaped = false;
            for action in replies {
                if !self.history_permits(&action) {
                    continue;
                }
                let before = self.checkpoint();
                let result = (|| {
                    self.apply_for_reply(&action)?;
                    let non_mate = self.adjudicate(false)?;
                    let captured = if non_mate.is_some() {
                        false
                    } else {
                        self.has_capture_all(side)?
                    };
                    Ok::<_, GameError>((non_mate, captured))
                })();
                self.restore(before);
                let (non_mate, captured) = result?;
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
        if let Some((winner, _)) = material_configuration(&self.local)
            && self.immediate_material_replies().is_empty()
        {
            return Ok(Some(Conclusion::Material { winner }));
        }
        Ok(None)
    }
    fn apply_for_reply(&mut self, action: &Action) -> Result<(), GameError> {
        let before = self.snapshot().clone();
        let actor = self.position().side_to_move();
        let influence_before = self.local.capture_influence(actor);
        let (next_local, e) = self.local.simulate(action).map_err(GameError::Action)?;
        self.local = next_local;
        let influence_after = self.local.capture_influence(actor);
        let moved_to = *action.route.last().expect("native route");
        let attack = influence_after
            .iter()
            .any(|p| p.0 == moved_to || !influence_before.contains(p));
        let enemy = royals(&self.local, actor.opponent());
        let check = enemy.len() == 1 && influence_after.iter().any(|p| p.1 == enemy[0]);
        self.plies.push(Ply {
            actor,
            action: action.clone(),
            quiet_return: e.quiet_return,
            check,
            attack,
        });
        self.keys.push(RepetitionKey::from_local(&self.local));
        let mut next = before.clone();
        next.state = self.position().state().clone();
        next.protection = self.local.protection().clone();
        next.board_ply += 1;
        next.occurrences = self.keys.current_occurrences();
        next.obligations = self.derive_obligations(&before.obligations, &next.occurrences);
        next.conclusion = None;
        next.draw_offer = None;
        self.snapshots.push(next);
        Ok(())
    }
    /// A single admitted opponent action capturing all royals, not separate attacks/minimax.
    pub fn has_capture_all(&self, victim: Side) -> Result<bool, GameError> {
        let targets = royals(&self.local, victim);
        if targets.is_empty() {
            return Ok(false);
        }
        let actions = self.local.legal_actions_for(victim.opponent());
        for action in actions {
            // A hypothetical future turn with the same placement must respect native history.
            // At actual replies the opponent is side-to-move; root threat is advisory geometry.
            let local = &self.local;
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
                self.position().state(),
                self.local.protection(),
                &self.events,
                self.keys.keys(),
                &self.plies,
                &self.snapshots,
                &self.root,
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
        assert!(discovered.plies[0].attack);
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
        assert!(!incidental.plies[0].attack);
        assert!(
            incidental
                .local_position()
                .capture_influence(Side::Sente)
                .contains(&(s(6, 8), s(6, 3)))
        );
    }
}
