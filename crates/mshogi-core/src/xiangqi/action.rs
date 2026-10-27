//! Complete native actions and private structural transitions, not a legality API.
use super::{Counters, Intersection, Piece, PieceKind, Position, Side};
use std::{error::Error, fmt, str::FromStr};

/// One complete native board action. Effects are derived, never client-supplied.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Action {
    /// Origin intersection.
    pub from: Intersection,
    /// Destination intersection.
    pub to: Intersection,
}
/// Input is not exactly four a0–i9 coordinate bytes; no drop/promotion/null tokens.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActionParseError;
impl fmt::Display for ActionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid Xiangqi coordinate board action")
    }
}
impl Error for ActionParseError {}
impl FromStr for Action {
    type Err = ActionParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if !text.is_ascii() || text.len() != 4 {
            return Err(ActionParseError);
        }
        Ok(Self {
            from: text[..2].parse()?,
            to: text[2..].parse()?,
        })
    }
}
impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.from, self.to)
    }
}
impl serde::Serialize for Action {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for Action {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(d)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}
/// Native derived effect of one complete action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActionEffects {
    /// Applied complete player choice.
    pub action: Action,
    /// Moving piece, with its native identity preserved.
    pub moving: Piece,
    /// Destination victim if captured. No hand resource is created.
    pub captured: Option<Piece>,
}
/// Atomic native action rejection, distinct from outcomes and representation limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionError {
    /// Origin equals destination.
    SameIntersection,
    /// No piece at origin.
    EmptySource,
    /// The source belongs to the other native owner.
    WrongSide,
    /// Destination has a friendly piece.
    FriendlyDestination,
    /// General capture is not a playable continuation.
    GeneralCapture,
    /// A declared counter cannot represent the next state; not board illegality.
    CounterOverflow,
    /// A prepared action belongs to a different complete position.
    StalePosition,
}
impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "xiangqi action rejected: {self:?}")
    }
}
impl Error for ActionError {}
/// A private native resolution tied to its exact before-state.
/// No public unchecked preparation/commit path exists in the state-only layer.
#[derive(Debug)]
pub struct ResolvedAction {
    pub(crate) before: Position,
    pub(crate) after: Position,
    pub(crate) effects: ActionEffects,
}
impl ResolvedAction {
    /// Rules-derived action/capture details; not input authority.
    pub const fn effects(&self) -> ActionEffects {
        self.effects
    }
}
/// Exact complete before/after values for local LIFO restoration, not game history.
#[derive(Debug)]
pub struct Undo {
    before: Position,
    after: Position,
    effects: ActionEffects,
}
impl Undo {
    /// Effects of the action that produced this local restoration record.
    pub const fn effects(&self) -> ActionEffects {
        self.effects
    }
}
/// A local restore was stale/out-of-order and changed nothing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UndoError {
    /// Complete current state does not equal this record's saved post-state.
    PositionMismatch,
}
impl fmt::Display for UndoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "xiangqi undo rejected: {self:?}")
    }
}
impl Error for UndoError {}

impl Position {
    // Deliberately private: ownership/capture/counter structure only. Movement,
    // blockers and General safety belong to the separately verified legal layer.
    #[allow(dead_code)] // State-only foundation: exercised by tests; legal layer is MSH-93.
    pub(crate) fn prepare_structural(&self, action: Action) -> Result<ResolvedAction, ActionError> {
        if action.from == action.to {
            return Err(ActionError::SameIntersection);
        }
        let moving = self.piece(action.from).ok_or(ActionError::EmptySource)?;
        if moving.side != self.side {
            return Err(ActionError::WrongSide);
        }
        let captured = self.piece(action.to);
        if captured.is_some_and(|piece| piece.side == self.side) {
            return Err(ActionError::FriendlyDestination);
        }
        if captured.is_some_and(|piece| piece.kind == PieceKind::General) {
            return Err(ActionError::GeneralCapture);
        }
        let no_capture = if captured.is_some() {
            Some(0)
        } else {
            self.counters
                .no_capture_plies()
                .map(|count| count.checked_add(1).ok_or(ActionError::CounterOverflow))
                .transpose()?
        };
        let fullmove = if self.side == Side::Black {
            self.counters
                .fullmove_number()
                .map(|count| count.checked_add(1).ok_or(ActionError::CounterOverflow))
                .transpose()?
        } else {
            self.counters.fullmove_number()
        };
        let mut after = self.clone();
        after.board[action.from.index()] = None;
        after.board[action.to.index()] = Some(moving);
        after.side = self.side.opponent();
        after.counters =
            Counters::new(no_capture, fullmove).expect("incremented or preserved positive counter");
        Ok(ResolvedAction {
            before: self.clone(),
            after,
            effects: ActionEffects {
                action,
                moving,
                captured,
            },
        })
    }
    #[allow(dead_code)] // No public unchecked action-acceptance path in MSH-92.
    pub(crate) fn commit_resolved(
        &mut self,
        resolved: ResolvedAction,
    ) -> Result<Undo, ActionError> {
        if *self != resolved.before {
            return Err(ActionError::StalePosition);
        }
        let undo = Undo {
            before: resolved.before,
            after: resolved.after.clone(),
            effects: resolved.effects,
        };
        *self = resolved.after;
        Ok(undo)
    }
    /// Restore only this record's exact post-state. This is local value safety,
    /// not a game/request/process identity or a way to revive retired engine work.
    pub fn undo(&mut self, undo: Undo) -> Result<(), UndoError> {
        if *self != undo.after {
            return Err(UndoError::PositionMismatch);
        }
        *self = undo.before;
        Ok(())
    }
}
