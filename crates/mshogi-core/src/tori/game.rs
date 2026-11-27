//! Authoritative Tori history and admission; local board rules remain in LocalPosition.
use super::*;
use crate::game::history::{GameHistory, Snapshot};
pub use crate::game::{
    GameContext,
    history::{CapacityUsage, GameUndo},
};
use crate::repetition::RepetitionIndex;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Identity for repetition only: every face/owner, both hands and turn.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepetitionKey {
    board: [Option<Piece>; 49],
    hands: [Hand; 2],
    side: Side,
}
impl Hash for RepetitionKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.digest());
    }
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
    /// 64-bit FNV-1a digest of the key (MSH-208). A filter only: equal keys
    /// have equal digests, and a match is always confirmed on the full key.
    pub fn digest(&self) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut eat = |b: u8| {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        };
        for square in &self.board {
            eat(square.map_or(0, |p| 1 + p.side as u8 * 32 + p.face as u8));
        }
        for hand in &self.hands {
            hand.0.iter().for_each(|&n| eat(n));
        }
        eat(self.side as u8);
        h
    }
}
/// T06 (MSH-208): the side to move may not play an action that would create
/// the third occurrence of a position (board, both hands, side to move) in the
/// known history. Derived from history after every board action; never a draw.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Repetition {
    /// Side to move, which may not play the listed actions.
    pub side: Side,
    /// Complete local actions whose resulting position already occurred twice.
    pub forbidden: Vec<Action>,
}
impl Repetition {
    /// History permission, separate from local board legality.
    pub fn permits(&self, action: Action) -> bool {
        !self.forbidden.contains(&action)
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
        /// Winner.
        winner: Side,
    },
    /// Quiet no-action is a loss under T05.
    StalemateLoss {
        /// Immobile game side.
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
/// Participant/administrative command. Unsupported declarations remain explicit refusals.
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
}
/// Exact ordered command plus all authoritative derived facts at its committed prefix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameEvent {
    /// Zero-based event index.
    pub index: usize,
    /// Accepted board actions (not the declared imported counter).
    pub board_ply: usize,
    /// Game command actor, even when not the side to move.
    pub actor: Side,
    /// Original complete submitted command.
    pub command: GameCommand,
    /// Game board effects; null for non-board events.
    pub effects: Option<Effects>,
    /// All known occurrences of the current board key (at most two: a third
    /// occurrence is refused). The restriction they imply is derived on replay,
    /// not stored.
    pub occurrences: Vec<usize>,
    /// Actual game conclusion only.
    pub conclusion: Option<Conclusion>,
}
/// Immutable historical data; no action application or playable branch method.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistorySnapshot {
    /// All board/hand/turn/root/counter semantics at this exact prefix.
    state: StructuralState,
    /// Sides whose position has already occurred twice in this prefix.
    seen_twice: [bool; 2],
    /// Exact event prefix length.
    pub event_count: usize,
    /// Known board-action prefix length.
    pub board_ply: usize,
    /// Current key's root-indexed occurrences, without future knowledge.
    pub occurrences: Vec<usize>,
    /// Current third-occurrence restriction.
    pub repetition: Option<Repetition>,
    /// Conclusion known at this prefix.
    pub conclusion: Option<Conclusion>,
}
impl HistorySnapshot {
    /// Exact logical state. External callers cannot replace this admitted
    /// state with an unvalidated diagram or construct forged history views.
    pub fn state(&self) -> &StructuralState {
        &self.state
    }
    /// Read-only local choices/inspection are available without creating a playable Game.
    pub fn local_position(&self) -> LocalPosition {
        // Already admitted game state; historical moving-side check is permitted.
        LocalPosition::admit(Position::install(self.state.clone()))
            .expect("accepted historical state")
    }
}
/// Context-checked complete request.
pub type GameRequest = crate::game::history::Request<Side, GameCommand>;
/// Exhausted resource, never an illegal move or a game outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resource {
    /// Encoded game root bytes at admission.
    EncodedBytes,
    /// Declared board counter.
    Counter,
}
/// Atomic game admission/reversal error; failures do not commit audit events.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameError {
    /// T07 local root failure.
    Root(AdmissionError),
    /// A live root cannot silently reset an already played position's known history.
    NotRoot,
    /// Wrong history or retired runtime.
    StaleContext,
    /// Board action actor differs from game turn.
    WrongActor,
    /// Selected profile has no agreement/impasse declaration.
    UnsupportedCommand,
    /// An actual game conclusion already exists.
    Concluded,
    /// This complete board action would create a third occurrence (T06).
    ThirdOccurrence,
    /// Complete local rule refusal.
    Action(ActionError),
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
/// Authoritative Tori history and adjudication.
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
        let mut seen_twice = game.snapshot().seen_twice;
        let mut effects = None;
        let mut occurrences = game.occurrences().to_vec();
        let mut repetition = game.repetition().cloned();
        let mut conclusion;
        match command {
            GameCommand::Play { action } => {
                if actor != game.position().side_to_move() {
                    return Err(GameError::WrongActor);
                }
                // Validate local rules first so history cannot excuse an illegal choice.
                if !game.local.is_legal(action) {
                    return Err(GameError::Action(
                        game.local
                            .clone()
                            .apply(action)
                            .expect_err("not locally legal"),
                    ));
                }
                if repetition.as_ref().is_some_and(|r| !r.permits(action)) {
                    return Err(GameError::ThirdOccurrence);
                }
                effects = Some(
                    game.local
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
                let key = RepetitionKey::from_position(game.position());
                game.keys.push(key);
                occurrences = game.keys.current_occurrences();
                if occurrences.len() >= 2 {
                    let side = game.position().side_to_move();
                    seen_twice[side.index()] = true;
                }
                let local_choices = game.local.legal_actions();
                let side = game.position().side_to_move();
                let status = match (game.local.is_in_check(side), local_choices.is_empty()) {
                    (true, true) => LocalStatus::Checkmate,
                    (false, true) => LocalStatus::QuietNoAction,
                    (true, false) => LocalStatus::Check,
                    (false, false) => LocalStatus::Ongoing,
                };
                conclusion = board_conclusion(status, side);
                repetition = None;
                if conclusion.is_none() {
                    repetition = game.third_occurrences(&local_choices, seen_twice);
                    if repetition
                        .as_ref()
                        .is_some_and(|r| r.forces_loss(&local_choices))
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
        }
        let event = GameEvent {
            index: game.events.len(),
            board_ply: game.keys.len() - 1,
            actor,
            command,
            effects,
            occurrences,
            conclusion,
        };
        debug_assert!(event.occurrences.len() <= 2, "third occurrence admitted");
        game.events.push(event.clone());
        game.snapshots.push(HistorySnapshot {
            seen_twice,
            state: game.position().state().clone(),
            event_count: game.events.len(),
            board_ply: game.keys.len() - 1,
            occurrences: event.occurrences.clone(),
            repetition,
            conclusion,
        });
        Ok(())
    }
}
impl Game {
    #[cfg(test)]
    pub(super) fn transaction_for_test(&self) -> Self {
        self.clone()
    }
    #[cfg(test)]
    pub(super) fn cache_snapshot(&self) -> String {
        format!(
            "{:?}",
            (
                self.position().state(),
                &self.events,
                self.keys.keys(),
                &self.snapshots,
                &self.root
            )
        )
    }
    /// Admit a complete known-history root, including immediate T05 terminal outcomes.
    pub fn from_root(root: Position) -> Result<Self, GameError> {
        if root.accepted_plies() != 0 {
            return Err(GameError::NotRoot);
        }
        let local = LocalPosition::admit(root).map_err(GameError::Root)?;
        let conclusion = board_conclusion(local.status(), local.position().side_to_move());
        let root_snapshot = HistorySnapshot {
            state: local.position().state().clone(),
            seen_twice: [false; 2],
            event_count: 0,
            board_ply: 0,
            occurrences: vec![0],
            repetition: None,
            conclusion,
        };
        let key = RepetitionKey::from_position(local.position());
        let game = Self::from_snapshot(local, RepetitionIndex::new(key), root_snapshot);
        game.check_root_size()?;
        Ok(game)
    }
    /// Full standard game with empty known history and the standard root as occurrence 1.
    pub fn standard() -> Self {
        Self::from_root(Position::standard()).expect("standard root")
    }
    /// Current position and provenance.
    pub fn position(&self) -> &Position {
        self.local.position()
    }
    /// Actual concluded engine result.
    pub fn conclusion(&self) -> Option<Conclusion> {
        self.snapshot().conclusion
    }
    /// Current third-occurrence restriction of the side to move.
    pub fn repetition(&self) -> Option<&Repetition> {
        self.snapshot().repetition.as_ref()
    }
    /// Current known key occurrences, including root where appropriate.
    pub fn occurrences(&self) -> &[usize] {
        &self.snapshot().occurrences
    }
    /// Complete permitted game choices; concluded games have none even if local choices exist.
    pub fn legal_actions(&self) -> Vec<Action> {
        if self.conclusion().is_some() {
            return vec![];
        }
        self.local
            .legal_actions()
            .into_iter()
            .filter(|a| self.repetition().is_none_or(|r| r.permits(*a)))
            .collect()
    }
    /// Complete canonical history at an exact committed event prefix, for read-only
    /// historical analysis/inspection. Future commands and obligations are absent.
    /// This does not branch, rewind or replace the live Game; restoring the record
    /// later always creates fresh session authority.
    pub fn record_prefix(&self, count: usize) -> Result<GameRecord, RecordError> {
        if count > self.events.len() {
            return Err(RecordError::Prefix);
        }
        let mut prefix = self.clone();
        prefix.truncate_to(count);
        GameRecord::from_game(&prefix)
    }

    /// Actions of the side to move that would create a third occurrence.
    fn third_occurrences(&self, local: &[Action], seen_twice: [bool; 2]) -> Option<Repetition> {
        let side = self.position().side_to_move();
        // Only a position that already occurred twice can occur a third time.
        if !seen_twice[side.opponent().index()] {
            return None;
        }
        let forbidden: Vec<Action> = local
            .iter()
            .copied()
            .filter(|&action| {
                let mut after = self.local.clone();
                after.apply(action).expect("locally legal action");
                let key = RepetitionKey::from_position(after.position());
                self.keys.occurrences(&key).nth(1).is_some()
            })
            .collect();
        (!forbidden.is_empty()).then_some(Repetition { side, forbidden })
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
