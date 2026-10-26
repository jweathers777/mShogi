//! Complete choices and deliberately limited structural resolution.

use super::{Piece, PieceKind, Position, Square};
use std::{error::Error, fmt, str::FromStr};

/// One complete board choice; captures and hand transfers are derived from state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Action {
    /// Move a board piece, explicitly choosing whether to promote.
    Move {
        /// Source square.
        from: Square,
        /// Destination square.
        to: Square,
        /// Promote now; already promoted pieces use false.
        promote: bool,
    },
    /// Drop an unpromoted resource from the moving owner's hand.
    Drop {
        /// Base kind (king is rejected by resolution).
        kind: PieceKind,
        /// Empty destination square.
        to: Square,
    },
}
/// A string is not an exact USI board action (resign/win/null are not actions).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActionParseError;
impl fmt::Display for ActionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid USI board action")
    }
}
impl Error for ActionParseError {}
impl FromStr for Action {
    type Err = ActionParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let b = text.as_bytes();
        if !text.is_ascii() {
            return Err(ActionParseError);
        }
        if b.len() == 4 && b[1] == b'*' {
            let kind = PieceKind::from_letter(char::from(b[0])).ok_or(ActionParseError)?;
            if b[0] != kind.letter() as u8 || kind == PieceKind::King {
                return Err(ActionParseError);
            }
            return Ok(Self::Drop {
                kind,
                to: text[2..].parse()?,
            });
        }
        if b.len() != 4 && !(b.len() == 5 && b[4] == b'+') {
            return Err(ActionParseError);
        }
        Ok(Self::Move {
            from: text[..2].parse()?,
            to: text[2..4].parse()?,
            promote: b.len() == 5,
        })
    }
}
impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Move { from, to, promote } => {
                write!(f, "{from}{to}{}", if *promote { "+" } else { "" })
            }
            Self::Drop { kind, to } => write!(f, "{}*{to}", kind.letter()),
        }
    }
}
// The execution transport uses exactly the already scoped native USI primitive;
// this does not add declarations/resignation to the board-action parser.
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

/// Rejection leaves every position field unchanged.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionError {
    /// No source piece.
    EmptySource,
    /// Source is owned by the other player.
    WrongSide,
    /// Source equals destination.
    SameSquare,
    /// Destination is friendly.
    FriendlyDestination,
    /// King capture is never a board action.
    KingCapture,
    /// Invalid movement shape or blocked ray.
    IllegalGeometry,
    /// Promotion unavailable for kind, already promoted state, or zone.
    UnexpectedPromotion,
    /// Pawn/lance/knight must promote at the destination.
    MissingPromotion,
    /// A king cannot be dropped or requested resource is absent.
    MissingResource,
    /// A drop requires an empty square.
    OccupiedDrop,
    /// Drop would leave the piece without any future movement.
    DeadDrop,
    /// A second unpromoted friendly pawn on this file.
    Nifu,
    /// Next-ply increment exceeds u32; a representation failure.
    CounterOverflow,
    /// The moving king remains or becomes attacked (legal layer).
    SelfCheck,
    /// A pawn drop delivers immediate mate (legal layer).
    PawnDropMate,
}
impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "shogi action rejected: {self:?}")
    }
}
impl Error for ActionError {}

/// Exact local before/after snapshots, not a serialized history record.
/// Restore in LIFO order; an equal state in another context is not distinguished.
#[derive(Debug)]
pub struct Undo {
    before: Position,
    after: Position,
}
/// A restoration record does not match this complete post-state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UndoError {
    /// Wrong, stale or out-of-order record. Nothing was changed.
    PositionMismatch,
}
impl fmt::Display for UndoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("shogi undo post-state mismatch")
    }
}
impl Error for UndoError {}

impl Position {
    /// Resolve local geometry/resources only. King safety and uchifuzume belong
    /// to the legal layer; callers must never expose this as full legality.
    pub(crate) fn apply_structural(&mut self, action: Action) -> Result<Undo, ActionError> {
        let next_ply = self
            .next_ply
            .checked_add(1)
            .ok_or(ActionError::CounterOverflow)?;
        let mut after = self.clone();
        match action {
            Action::Move { from, to, promote } => {
                if from == to {
                    return Err(ActionError::SameSquare);
                }
                let piece = self.piece(from).ok_or(ActionError::EmptySource)?;
                if piece.side != self.side {
                    return Err(ActionError::WrongSide);
                }
                if let Some(victim) = self.piece(to) {
                    if victim.side == self.side {
                        return Err(ActionError::FriendlyDestination);
                    }
                    if victim.kind == PieceKind::King {
                        return Err(ActionError::KingCapture);
                    }
                }
                if !self.geometrically_reaches(from, to) {
                    return Err(ActionError::IllegalGeometry);
                }
                if promote
                    && (piece.promoted
                        || !piece.kind.can_promote()
                        || !(piece.side.in_camp(from.rank()) || piece.side.in_camp(to.rank())))
                {
                    return Err(ActionError::UnexpectedPromotion);
                }
                if !promote && !piece.promoted && piece.kind.dead_unpromoted(piece.side, to.rank())
                {
                    return Err(ActionError::MissingPromotion);
                }
                if let Some(victim) = self.piece(to) {
                    let i = victim
                        .kind
                        .hand_index()
                        .expect("king capture already rejected");
                    after.hands[self.side.index()].0[i] += 1;
                }
                after.board[from.index()] = None;
                after.board[to.index()] = Some(Piece {
                    promoted: piece.promoted || promote,
                    ..piece
                });
            }
            Action::Drop { kind, to } => {
                let i = kind.hand_index().ok_or(ActionError::MissingResource)?;
                if self.hands[self.side.index()].0[i] == 0 {
                    return Err(ActionError::MissingResource);
                }
                if self.piece(to).is_some() {
                    return Err(ActionError::OccupiedDrop);
                }
                if kind.dead_unpromoted(self.side, to.rank()) {
                    return Err(ActionError::DeadDrop);
                }
                if kind == PieceKind::Pawn
                    && Square::all().any(|s| {
                        s.file() == to.file()
                            && self.piece(s)
                                == Some(Piece {
                                    side: self.side,
                                    kind: PieceKind::Pawn,
                                    promoted: false,
                                })
                    })
                {
                    return Err(ActionError::Nifu);
                }
                after.hands[self.side.index()].0[i] -= 1;
                after.board[to.index()] = Some(Piece {
                    side: self.side,
                    kind,
                    promoted: false,
                });
            }
        }
        after.side = self.side.opponent();
        after.next_ply = next_ply;
        let undo = Undo {
            before: self.clone(),
            after: after.clone(),
        };
        *self = after;
        Ok(undo)
    }

    /// Restore every field only when this is the record's exact saved post-state.
    pub(crate) fn undo_structural(&mut self, undo: Undo) -> Result<(), UndoError> {
        if *self != undo.after {
            return Err(UndoError::PositionMismatch);
        }
        *self = undo.before;
        Ok(())
    }

    /// Geometric attack shape with path blocking, independent of destination
    /// ownership, king safety, promotion choice or whose turn it is.
    pub(crate) fn geometrically_reaches(&self, from: Square, to: Square) -> bool {
        let Some(piece) = self.piece(from) else {
            return false;
        };
        if from == to {
            return false;
        }
        let dx = to.file() as i8 - from.file() as i8;
        let dy = to.rank() as i8 - from.rank() as i8;
        let forward = dy * piece.side.forward();
        let gold = (forward == 1 && dx.abs() <= 1)
            || (forward == 0 && dx.abs() == 1)
            || (forward == -1 && dx == 0);
        let diagonal = dx.abs() == dy.abs();
        let orthogonal = dx == 0 || dy == 0;
        let adjacent = dx.abs() <= 1 && dy.abs() <= 1;
        let ray = match piece.kind {
            PieceKind::King => return adjacent,
            PieceKind::Gold => return gold,
            PieceKind::Silver | PieceKind::Knight | PieceKind::Lance | PieceKind::Pawn
                if piece.promoted =>
            {
                return gold;
            }
            PieceKind::Silver => {
                return (forward == 1 && dx.abs() <= 1) || (forward == -1 && dx.abs() == 1);
            }
            PieceKind::Knight => return forward == 2 && dx.abs() == 1,
            PieceKind::Pawn => return forward == 1 && dx == 0,
            PieceKind::Lance => dx == 0 && forward > 0,
            PieceKind::Rook => {
                if piece.promoted && diagonal && adjacent {
                    return true;
                }
                orthogonal
            }
            PieceKind::Bishop => {
                if piece.promoted && orthogonal && adjacent {
                    return true;
                }
                diagonal
            }
        };
        if !ray {
            return false;
        }
        let mut file = from.file() as i8 + dx.signum();
        let mut rank = from.rank() as i8 + dy.signum();
        while file != to.file() as i8 || rank != to.rank() as i8 {
            let square = Square::new(file as u8, rank as u8).expect("ray between bounded squares");
            if self.piece(square).is_some() {
                return false;
            }
            file += dx.signum();
            rank += dy.signum();
        }
        true
    }
}
