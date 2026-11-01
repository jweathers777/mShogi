//! Crate-private structural resolution; no safety, drop-file/mate or history admission.
use super::{Action, Face, Piece, Position, Side, Square, StructuralState};
use std::sync::Arc;

/// One Tori face's exact local geometry; rays carry a maximum distance (6=board edge).
#[derive(Debug)]
pub(crate) struct Movement {
    pub(crate) steps: &'static [(i8, i8)],
    pub(crate) jumps: &'static [(i8, i8)],
    pub(crate) rays: &'static [(i8, i8, u8)],
}
pub(crate) fn movement(face: Face) -> Movement {
    use Face::*;
    let (steps, jumps, rays): (_, _, _) = match face {
        Phoenix => (
            &[
                (-1, 1),
                (0, 1),
                (1, 1),
                (-1, 0),
                (1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ][..],
            &[][..],
            &[][..],
        ),
        Falcon => (
            &[(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (1, -1)][..],
            &[][..],
            &[][..],
        ),
        Crane => (
            &[(-1, 1), (0, 1), (1, 1), (-1, -1), (0, -1), (1, -1)][..],
            &[][..],
            &[][..],
        ),
        Pheasant => (&[(-1, -1), (1, -1)][..], &[(0, 2)][..], &[][..]),
        LeftQuail => (&[(-1, -1)][..], &[][..], &[(0, 1, 6), (1, -1, 6)][..]),
        RightQuail => (&[(1, -1)][..], &[][..], &[(0, 1, 6), (-1, -1, 6)][..]),
        Swallow => (&[(0, 1)][..], &[][..], &[][..]),
        Eagle => (
            &[(0, 1), (-1, 0), (1, 0)][..],
            &[][..],
            &[(-1, 1, 6), (1, 1, 6), (0, -1, 6), (-1, -1, 2), (1, -1, 2)][..],
        ),
        Goose => (&[][..], &[(-2, 2), (2, 2), (0, -2)][..], &[][..]),
    };
    Movement { steps, jumps, rays }
}
/// Structural failure, never a game outcome or an empty legal-action list.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TransitionError {
    EmptySource,
    WrongOwner,
    SameSquare,
    FriendlyDestination,
    RoyalCapture,
    Geometry,
    ResultFace,
    InvalidDropFace,
    MissingHand,
    OccupiedDrop,
    Capacity,
    InvalidToken,
}
/// Native derived facts, with the actual captured promoted face preserved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Effects {
    pub(crate) actor: Side,
    pub(crate) action: Action,
    pub(crate) resulting_piece: Piece,
    pub(crate) captured: Option<Piece>,
    pub(crate) hand_change: Option<(Face, i8)>,
}
/// Position-bound, single-use, LIFO reversal authority; not a canonical record.
#[derive(Debug)]
pub(crate) struct Undo {
    owner: Arc<()>,
    before: StructuralState,
    after: StructuralState,
    before_node: u64,
    after_node: u64,
    used: bool,
    effects: Effects,
}
/// A single-use forward restoration, invalidated by any newly applied branch.
#[derive(Debug)]
pub(crate) struct Redo {
    owner: Arc<()>,
    before: StructuralState,
    after: StructuralState,
    branch: u64,
    used: bool,
    effects: Effects,
}
impl Position {
    /// Face geometry and blocking only. Includes friendly endpoints/royals so it
    /// can later support attack maps; never a list of legal board actions.
    pub(crate) fn geometrically_reaches(&self, from: Square, to: Square) -> bool {
        let Some(piece) = self.piece(from) else {
            return false;
        };
        if from == to {
            return false;
        }
        let m = movement(piece.face);
        if m.steps
            .iter()
            .chain(m.jumps)
            .any(|&(x, f)| from.offset(piece.side, x, f) == Some(to))
        {
            return true;
        }
        for &(x, f, max) in m.rays {
            for n in 1..=max {
                let Some(sq) = from.offset(piece.side, x * n as i8, f * n as i8) else {
                    break;
                };
                if sq == to {
                    return true;
                }
                if self.piece(sq).is_some() {
                    break;
                }
            }
        }
        false
    }
    /// Structural effect resolution on a private candidate. The caller must
    /// additionally enforce128 local legality and129 game permission.
    pub(crate) fn resolve_structural(
        &self,
        action: Action,
    ) -> Result<(StructuralState, Effects), TransitionError> {
        use TransitionError::*;
        let accepted = self.state.accepted_plies.checked_add(1).ok_or(Capacity)?;
        self.state
            .root
            .declared_completed_plies
            .checked_add(accepted)
            .ok_or(Capacity)?;
        let actor = self.side_to_move();
        let mut after = self.state.clone();
        let resulting_piece;
        let mut captured = None;
        let hand_change;
        match action {
            Action::Move {
                from,
                to,
                result_face,
            } => {
                if from == to {
                    return Err(SameSquare);
                }
                let p = self.piece(from).ok_or(EmptySource)?;
                if p.side != actor {
                    return Err(WrongOwner);
                }
                if let Some(v) = self.piece(to) {
                    if v.side == actor {
                        return Err(FriendlyDestination);
                    }
                    if v.face == Face::Phoenix {
                        return Err(RoyalCapture);
                    }
                    captured = Some(v);
                }
                if !self.geometrically_reaches(from, to) {
                    return Err(Geometry);
                }
                if p.face.moved_face(actor, from, to) != result_face {
                    return Err(ResultFace);
                }
                hand_change = if let Some(v) = captured {
                    let base = v.face.base();
                    let i = base.hand_index().ok_or(RoyalCapture)?;
                    let n = after.hands[actor.index()].0[i]
                        .checked_add(1)
                        .ok_or(Capacity)?;
                    if n > base.cap() {
                        return Err(Capacity);
                    }
                    after.hands[actor.index()].0[i] = n;
                    Some((base, 1))
                } else {
                    None
                };
                resulting_piece = Piece {
                    side: actor,
                    face: result_face,
                };
                after.board[from.index()] = None;
                after.board[to.index()] = Some(resulting_piece);
            }
            Action::Drop { face, to } => {
                let i = face.hand_index().ok_or(InvalidDropFace)?;
                if self.piece(to).is_some() {
                    return Err(OccupiedDrop);
                }
                if after.hands[actor.index()].0[i] == 0 {
                    return Err(MissingHand);
                }
                after.hands[actor.index()].0[i] -= 1;
                resulting_piece = Piece { side: actor, face };
                after.board[to.index()] = Some(resulting_piece);
                hand_change = Some((face, -1));
            }
        }
        after.side = actor.opponent();
        after.accepted_plies = accepted;
        Ok((
            after,
            Effects {
                actor,
                action,
                resulting_piece,
                captured,
                hand_change,
            },
        ))
    }
    /// NOT full legality. All counters/token capacity are checked before commit.
    pub(crate) fn apply_structural(
        &mut self,
        action: Action,
    ) -> Result<(Effects, Undo), TransitionError> {
        let (after, effects) = self.resolve_structural(action)?;
        let next = self
            .next_node
            .checked_add(1)
            .ok_or(TransitionError::Capacity)?;
        let branch = self
            .branch
            .checked_add(1)
            .ok_or(TransitionError::Capacity)?;
        let revision = self
            .revision
            .checked_add(1)
            .ok_or(TransitionError::Capacity)?;
        let undo = Undo {
            owner: Arc::clone(&self.owner),
            before: self.state.clone(),
            after: after.clone(),
            before_node: self.node,
            after_node: self.next_node,
            used: false,
            effects,
        };
        self.state = after;
        self.node = self.next_node;
        self.next_node = next;
        self.branch = branch;
        self.revision = revision;
        Ok((effects, undo))
    }
    /// Exact logical restoration. Tokens are single-use, position-bound and LIFO.
    /// Node identities allow unwinding earlier valid tokens; runtime revision is
    /// never rewound and old consumed tokens cannot revive on equal diagrams.
    pub(crate) fn undo_structural(&mut self, undo: &mut Undo) -> Result<Redo, TransitionError> {
        if undo.used
            || !Arc::ptr_eq(&self.owner, &undo.owner)
            || self.node != undo.after_node
            || self.state != undo.after
        {
            return Err(TransitionError::InvalidToken);
        }
        let revision = self
            .revision
            .checked_add(1)
            .ok_or(TransitionError::Capacity)?;
        let redo = Redo {
            owner: Arc::clone(&self.owner),
            before: undo.before.clone(),
            after: undo.after.clone(),
            branch: self.branch,
            used: false,
            effects: undo.effects,
        };
        self.state = undo.before.clone();
        self.node = undo.before_node;
        self.revision = revision;
        undo.used = true;
        Ok(redo)
    }
    /// Exact accepted post-state restoration, never reusing old reversal authority.
    /// A newly applied action retires all outstanding redo tokens; a chain of
    /// undo/redo alone can restore a sequence in order, each with fresh Undo.
    pub(crate) fn redo_structural(
        &mut self,
        redo: &mut Redo,
    ) -> Result<(Effects, Undo), TransitionError> {
        if redo.used
            || !Arc::ptr_eq(&self.owner, &redo.owner)
            || self.branch != redo.branch
            || self.state != redo.before
        {
            return Err(TransitionError::InvalidToken);
        }
        let next = self
            .next_node
            .checked_add(1)
            .ok_or(TransitionError::Capacity)?;
        let revision = self
            .revision
            .checked_add(1)
            .ok_or(TransitionError::Capacity)?;
        let undo = Undo {
            owner: Arc::clone(&self.owner),
            before: self.state.clone(),
            after: redo.after.clone(),
            before_node: self.node,
            after_node: self.next_node,
            used: false,
            effects: redo.effects,
        };
        self.state = redo.after.clone();
        self.node = self.next_node;
        self.next_node = next;
        self.revision = revision;
        redo.used = true;
        Ok((redo.effects, undo))
    }
}
