//! One authoritative Tenjiku history owner. Local geometry is not live permission.
use super::*;
use crate::game::history::{GameHistory, Snapshot};
pub use crate::game::{
    GameContext,
    history::{CapacityUsage, GameUndo},
};
use crate::repetition::RepetitionIndex;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt,
    hash::{Hash, Hasher},
};

/// Game conclusions are separate from search, cancellation and operational failures.
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
/// Complete participant/administrative commands; no rule switches exist.
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
/// Typed atomic refusal.
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
/// A complete immutable committed prefix; it owns no execution authority.
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
pub type GameRequest = crate::game::history::Request<Side, GameCommand>;
/// Unscored no-action is not an invented mate, forfeit or draw.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum GameStatus {
    Ongoing,
    BareReply { side: Side },
    NoBoardActions,
    Concluded(Conclusion),
}
/// Authoritative Tenjiku history and adjudication.
pub type Game = GameHistory<HistorySnapshot>;
impl Snapshot for HistorySnapshot {
    type Side = Side;
    type Command = GameCommand;
    const STALE: GameError = GameError::StaleContext;
    const LARGE_ROOT: GameError = GameError::Capacity(Resource::EncodedBytes);
    type Local = LocalPosition;
    type Key = RepetitionKey;
    type Ply = ();
    type Event = GameEvent;
    type Error = GameError;
    const INVALID_UNDO: GameError = GameError::InvalidToken;
    const INVALID_PREFIX: GameError = GameError::Prefix;
    fn board_ply(&self) -> usize {
        self.board_ply
    }
    fn restore_local(&self) -> LocalPosition {
        self.local_position()
    }
    fn encoded_size(game: &Game) -> Result<usize, GameError> {
        crate::record::compact::encode(game)
            .map(|s| s.len())
            .map_err(|_| GameError::Record)
    }
    fn step(
        game: &mut Game,
        actor: Side,
        command: GameCommand,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), GameError> {
        check(stop)?;
        if game.conclusion().is_some() {
            return Err(GameError::Concluded);
        }
        let mut effects = None;
        let mut board_ply = game.snapshot().board_ply;
        let mut conclusion = game.conclusion();
        let mut pending = game.snapshot().pending_bare;
        let mut offer = game.snapshot().draw_offer;
        match &command {
            GameCommand::Play { action } => {
                if actor != game.position().side_to_move() {
                    return Err(GameError::WrongActor);
                }
                let (e, _) = game.local.apply_with_stop(action, &mut *stop)?;
                let key = RepetitionKey::from_position(game.position());
                check(stop)?;
                if game.keys.occurrences(&key).next().is_some() {
                    return Err(GameError::Repetition);
                }
                game.keys.push(key);
                board_ply += 1;
                effects = Some(e);
                offer = None;
                let (c, p) = adjudicate(&game.local, &game.keys, stop)?;
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
        game.push_event(
            actor, command, effects, board_ply, conclusion, pending, offer,
        );
        check(stop)
    }
}
impl Game {
    /// Admit a declared known-history root; no earlier history is invented.
    pub fn from_root(root: Position) -> Result<Self, GameError> {
        Self::from_root_with_stop(root, &mut || false)
    }
    /// Root admission with cancellation before finite bare-response queries.
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
        let keys = RepetitionIndex::new(RepetitionKey::from_position(local.position()));
        let mut snapshot = HistorySnapshot {
            state: local.position().state().clone(),
            event_count: 0,
            board_ply: 0,
            conclusion: None,
            pending_bare: None,
            draw_offer: None,
        };
        let (conclusion, pending) = adjudicate(&local, &keys, stop)?;
        snapshot.conclusion = conclusion;
        snapshot.pending_bare = pending;
        let game = Self::from_snapshot(local, keys, snapshot);
        game.check_root_size()?;
        check(stop)?;
        Ok(game)
    }
    /// Exact standard setup under an explicitly selected whole profile.
    pub fn standard(profile: Profile) -> Self {
        Self::from_root(Position::standard(profile)).expect("standard game root")
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
    pub fn game_root(&self) -> &StructuralState {
        self.root.state()
    }
    /// Exact completed game outcome only.
    pub fn conclusion(&self) -> Option<Conclusion> {
        self.snapshot().conclusion
    }
    /// All surviving royal identities and positions for one owner.
    pub fn royals(&self, side: Side) -> Vec<(Square, Piece)> {
        self.position()
            .pieces()
            .filter(|(_, p)| p.side == side && p.is_royal())
            .collect()
    }
    /// Canonical exact prefix reconstructed with fresh session authority.
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
            Position::install(self.root.state.clone()).map_err(|_| RecordError::Root)?,
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
    /// Complete current game choices, with repetition admission separate from geometry.
    pub fn legal_actions(&self) -> Vec<Action> {
        self.legal_actions_with_stop(&mut || false)
            .expect("finite admitted game query")
    }
    /// Cancellable Game choices; no process, frontend or implied outcome.
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
    /// Cancellable atomic admission; a stopped candidate never changes accepted authority.
    pub fn submit_with_stop(
        &mut self,
        request: GameRequest,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<GameUndo, GameError> {
        self.admit(request, stop)
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
    ) {
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
