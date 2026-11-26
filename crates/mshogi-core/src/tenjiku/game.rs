//! One authoritative Tenjiku history owner. Local geometry is not live permission.
use super::*;
pub use crate::native::GameContext;
use crate::repetition::RepetitionIndex;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt,
    hash::{Hash, Hasher},
};

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
        }
        Ok(match Wire::deserialize(d)? {
            Wire::Play { action } => Self::Play { action },
            Wire::Resign {} => Self::Resign,
            Wire::OfferDraw {} => Self::OfferDraw,
            Wire::AcceptDraw {} => Self::AcceptDraw,
            Wire::DeclineDraw {} => Self::DeclineDraw,
        })
    }
}
/// Resource refusal leaves the accepted complete record intact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum Resource {
    EncodedBytes,
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RepetitionKey {
    profile: Profile,
    side: Side,
    board: Vec<Option<(Side, Base, bool)>>,
}
impl Hash for RepetitionKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.profile.hash(state);
        self.side.hash(state);
        // Hash the 16x16 board in one write. Each square fits in one byte:
        // 36 original types, two owners and two promotion states, plus empty.
        let mut board = [0; 256];
        for (code, piece) in board.iter_mut().zip(&self.board) {
            *code = piece.map_or(0, |(side, base, promoted)| {
                1 + 4 * base as u8 + 2 * side as u8 + u8::from(promoted)
            });
        }
        state.write(&board);
    }
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
/// Actual current native capacity facts plus bounded ending headroom.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub struct CapacityUsage {
    pub encoded_bytes: usize,
    pub events: usize,
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
    keys: RepetitionIndex<RepetitionKey>,
    snapshots: Vec<HistorySnapshot>,
    root_snapshot: HistorySnapshot,
    context: GameContext,
    node: u64,
    next_node: u64,
}
impl Game {
    /// Copy an admitted game for private analysis without replaying its history.
    /// The copy has a fresh runtime identity; existing reversal tokens cannot apply.
    pub fn fork(&self) -> Self {
        Self {
            root: self.root.clone(),
            local: self.local.clone(),
            events: self.events.clone(),
            keys: self.keys.clone(),
            snapshots: self.snapshots.clone(),
            root_snapshot: self.root_snapshot.clone(),
            context: GameContext::fresh(),
            node: self.node,
            next_node: self.next_node,
        }
    }

    #[cfg(test)]
    pub(super) fn set_test_authority(&mut self, revision: u64, next_node: u64) {
        self.context = self.context.at_revision(revision);
        self.next_node = next_node;
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
        let keys = RepetitionIndex::new(RepetitionKey::from_position(local.position()));
        let mut snapshot = HistorySnapshot {
            state: root.clone(),
            event_count: 0,
            board_ply: 0,
            conclusion: None,
            pending_bare: None,
            draw_offer: None,
        };
        let (conclusion, pending) = adjudicate(&local, &keys, stop)?;
        snapshot.conclusion = conclusion;
        snapshot.pending_bare = pending;
        let game = Self {
            root,
            local,
            events: vec![],
            keys,
            snapshots: vec![],
            root_snapshot: snapshot,
            context: GameContext::fresh(),
            node: 0,
            next_node: 1,
        };
        if record::encode(&game).map_err(|_| GameError::Record)?.len()
            > crate::record::MAX_COMPACT_ROOT_BYTES
        {
            return Err(GameError::Capacity(Resource::EncodedBytes));
        }
        check(stop)?;
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
        self.snapshots.last().unwrap_or(&self.root_snapshot)
    }
    /// Actual command/event ledger in native order.
    pub fn events(&self) -> &[GameEvent] {
        &self.events
    }
    /// Exact completed native outcome only.
    pub fn conclusion(&self) -> Option<Conclusion> {
        self.snapshot().conclusion
    }
    /// Current request token; it changes at each accepted command, undo and retirement.
    pub fn context(&self) -> GameContext {
        self.context
    }
    /// Repetition-only native identity, excluding serials/counters/refusal flags.
    pub fn repetition_key(&self) -> &RepetitionKey {
        self.keys.current()
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
            Ok(self.root_snapshot.clone())
        } else {
            self.snapshots
                .get(count - 1)
                .cloned()
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
        self.context = self.next_context(self.conclusion().is_none())?;
        Ok(())
    }
    /// Next request token; with `reserve`, one more revision stays free for an ending.
    fn next_context(&self, reserve: bool) -> Result<GameContext, GameError> {
        self.context
            .advanced()
            .filter(|next| !reserve || next.advanced().is_some())
            .ok_or(GameError::Capacity(Resource::Counter))
    }
    /// Canonical exact prefix reconstructed with fresh native authority.
    pub fn record_prefix(&self, count: usize) -> Result<GameRecord, RecordError> {
        self.record_prefix_with_stop(count, &mut || false)
    }
    /// Reconstruct an exact prefix with cooperative cancellation throughout replay.
    pub fn record_prefix_with_stop(
        &self,
        count: usize,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<GameRecord, RecordError> {
        if count > self.events.len() {
            return Err(RecordError::Prefix);
        }
        let mut game = Self::from_root_with_stop(
            Position::install(self.root.clone()).map_err(|_| RecordError::Root)?,
            stop,
        )
        .map_err(|e| match e {
            GameError::Capacity(_) => RecordError::Capacity,
            GameError::Stopped => RecordError::Stopped,
            _ => RecordError::Root,
        })?;
        for event in &self.events[..count] {
            game.submit_with_stop(
                GameRequest {
                    expected: game.context(),
                    actor: event.actor,
                    command: event.command.clone(),
                },
                stop,
            )
            .map_err(|e| match e {
                GameError::Capacity(_) => RecordError::Capacity,
                GameError::Stopped => RecordError::Stopped,
                _ => RecordError::Derived,
            })?;
        }
        if stop() {
            return Err(RecordError::Stopped);
        }
        let record = GameRecord::from_game(&game)?;
        if stop() {
            return Err(RecordError::Stopped);
        }
        Ok(record)
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
            check(stop)?;
            if self.keys.occurrences(&key).next().is_none() {
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
    /// Atomic token-bound admission; all effects/capacity validate before publication.
    pub fn submit(&mut self, request: GameRequest) -> Result<GameUndo, GameError> {
        self.submit_with_stop(request, &mut || false)
    }
    /// Cancellable atomic admission; a stopped candidate never changes accepted authority.
    pub fn submit_with_stop(
        &mut self,
        request: GameRequest,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<GameUndo, GameError> {
        if request.expected != self.context {
            return Err(GameError::StaleContext);
        }
        self.commit(request.actor, request.command, stop)
    }
    fn commit(
        &mut self,
        actor: Side,
        command: GameCommand,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<GameUndo, GameError> {
        let working = self.local.clone();
        let local = std::mem::replace(&mut self.local, working);
        let events = self.events.len();
        let keys = self.keys.len();
        let result = self.commit_inner(actor, command, stop);
        if result.is_err() {
            self.local = local;
            self.events.truncate(events);
            self.snapshots.truncate(events);
            self.keys.truncate(keys);
        }
        result
    }
    fn commit_inner(
        &mut self,
        actor: Side,
        command: GameCommand,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<GameUndo, GameError> {
        check(stop)?;
        if self.conclusion().is_some() {
            return Err(GameError::Concluded);
        }
        let mut effects = None;
        let mut board_ply = self.snapshot().board_ply;
        let mut conclusion = self.conclusion();
        let mut pending = self.snapshot().pending_bare;
        let mut offer = self.snapshot().draw_offer;
        match &command {
            GameCommand::Play { action } => {
                if actor != self.position().side_to_move() {
                    return Err(GameError::WrongActor);
                }
                let (e, _) = self.local.apply_with_stop(action, &mut *stop)?;
                let key = RepetitionKey::from_position(self.position());
                check(stop)?;
                if self.keys.occurrences(&key).next().is_some() {
                    return Err(GameError::Repetition);
                }
                self.keys.push(key);
                board_ply += 1;
                effects = Some(e);
                offer = None;
                let (c, p) = adjudicate(&self.local, &self.keys, stop)?;
                conclusion = c;
                pending = p;
            }
            GameCommand::Resign => {
                conclusion = Some(Conclusion::Resignation {
                    winner: actor.opponent(),
                });
                pending = None;
                offer = None;
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
            }
            GameCommand::DeclineDraw => {
                if offer != Some(actor.opponent()) {
                    return Err(GameError::DrawOffer);
                }
                offer = None;
            }
        }
        self.push_event(
            actor, command, effects, board_ply, conclusion, pending, offer,
        )?;
        check(stop)?;
        let reserve = self.conclusion().is_none();
        let context = self.next_context(reserve)?;
        let next_node = self
            .next_node
            .checked_add(1)
            .filter(|next| next.checked_add(u64::from(reserve)).is_some())
            .ok_or(GameError::Capacity(Resource::Counter))?;
        let token = GameUndo {
            instance: context.instance(),
            before_node: self.node,
            after_node: self.next_node,
            before_len: self.events.len() - 1,
            used: false,
        };
        self.context = context;
        self.node = self.next_node;
        self.next_node = next_node;
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
        };
        self.events.push(event);
        self.snapshots.push(HistorySnapshot {
            state: self.position().state().clone(),
            event_count: self.events.len(),
            board_ply,
            conclusion,
            pending_bare,
            draw_offer,
        });
        Ok(())
    }
    /// Actual record size and event count for an explicit diagnostic query.
    pub fn capacity_usage(&self) -> Result<CapacityUsage, GameError> {
        Ok(CapacityUsage {
            encoded_bytes: record::encoded_size(self).map_err(|_| GameError::Record)?,
            events: self.events.len(),
        })
    }

    /// LIFO exact logical undo; runtime revisions advance rather than rewind.
    pub fn undo(&mut self, undo: &mut GameUndo) -> Result<(), GameError> {
        if undo.used
            || undo.instance != self.context.instance()
            || undo.after_node != self.node
            || self.events.len() != undo.before_len + 1
        {
            return Err(GameError::InvalidToken);
        }
        let before = self.event_prefix(undo.before_len)?;
        let reserve = before.conclusion.is_none();
        let context = self.next_context(reserve)?;
        if self.next_node.checked_add(u64::from(reserve)).is_none() {
            return Err(GameError::Capacity(Resource::Counter));
        }
        let local = LocalPosition::admit(Position::install(before.state).map_err(GameError::Root)?)
            .map_err(GameError::Root)?;
        self.events.truncate(undo.before_len);
        self.snapshots.truncate(undo.before_len);
        self.keys.truncate(before.board_ply + 1);
        self.local = local;
        self.context = context;
        self.node = undo.before_node;
        undo.used = true;
        Ok(())
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
    keys: &RepetitionIndex<RepetitionKey>,
    stop: &mut impl FnMut() -> bool,
) -> Result<(Option<Conclusion>, Option<Side>), GameError> {
    check(stop)?;
    let p = local.position();
    if let Some(c) = terminal_royals(p) {
        return Ok((Some(c), None));
    }
    let b = [bare(p, Side::Sente), bare(p, Side::Gote)];
    if b == [true, true] {
        return Ok((Some(Conclusion::BothBareRoyals), None));
    }
    let Some(victim) = [Side::Sente, Side::Gote].into_iter().find(|s| b[s.index()]) else {
        return Ok((None, None));
    };
    if victim != p.side_to_move() {
        return Ok((
            Some(Conclusion::BareRoyalLoss {
                winner: victim.opponent(),
            }),
            None,
        ));
    }
    let actions = local.legal_actions_with_stop(&mut *stop)?;
    for action in actions {
        check(stop)?;
        let (next, _) = local.simulate_with_stop(&action, &mut *stop)?;
        let key = RepetitionKey::from_position(next.position());
        check(stop)?;
        if keys.occurrences(&key).next().is_some() {
            continue;
        }
        let escape = terminal_royals(next.position())
            .is_some_and(|c| c.winner() != Some(victim.opponent()))
            || (bare(next.position(), Side::Sente) && bare(next.position(), Side::Gote));
        if escape {
            return Ok((None, Some(victim)));
        }
    }
    Ok((
        Some(Conclusion::BareRoyalLoss {
            winner: victim.opponent(),
        }),
        None,
    ))
}

#[cfg(test)]
mod rollback_tests {
    use super::*;
    #[test]
    fn late_cancellation_restores_all_private_state() {
        for profile in Profile::ALL {
            let mut game = Game::standard(profile);
            let action = game.legal_actions()[0].clone();
            game.submit(GameRequest {
                expected: game.context(),
                actor: Side::Sente,
                command: GameCommand::Play { action },
            })
            .unwrap();
            // Checks include cancellation after a new control snapshot is appended.
            for stop_at in 1..=2 {
                let before = format!("{game:?}");
                let mut calls = 0;
                let result = game.submit_with_stop(
                    GameRequest {
                        expected: game.context(),
                        actor: Side::Sente,
                        command: GameCommand::OfferDraw,
                    },
                    &mut || {
                        calls += 1;
                        calls == stop_at
                    },
                );
                assert!(matches!(result, Err(GameError::Stopped)));
                assert_eq!(format!("{game:?}"), before);
            }
        }
    }
}
