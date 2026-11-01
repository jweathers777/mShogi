//! Position-local board rules. No repetition, match commands or game permission.
use super::{Action, Face, Piece, Position, Side, Square, transition};
use std::{error::Error, fmt};

/// Native effects from a checked local move/drop; captures are derived, never supplied.
pub use super::transition::Effects;
/// Failed complete local action; no outcome or execution failure is implied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionError {
    /// Source has no piece.
    EmptySource,
    /// Source belongs to the other side.
    WrongOwner,
    /// Source and endpoint are identical.
    SameSquare,
    /// Endpoint holds a friendly piece.
    FriendlyDestination,
    /// Phoenix capture is not a board action.
    RoyalCapture,
    /// Impossible movement or an obstructed ray.
    Geometry,
    /// Requested resulting face contradicts compulsory promotion/native identity.
    ResultFace,
    /// Royal/promoted face cannot be dropped.
    InvalidDropFace,
    /// Requested hand resource is absent.
    MissingHand,
    /// Drop endpoint is occupied.
    OccupiedDrop,
    /// Mover's Phoenix is attacked in the resulting position.
    SelfCheck,
    /// Third own unpromoted Swallow on this file.
    SwallowFile,
    /// A Swallow cannot drop on the mover's last rank.
    SwallowLastRank,
    /// The dropped Swallow itself gives immediate mate.
    SwallowDropMate,
    /// Counter or reversal authority cannot represent another committed action.
    Capacity,
}
impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tori local action rejected: {self:?}")
    }
}
impl Error for ActionError {}
impl From<transition::TransitionError> for ActionError {
    fn from(e: transition::TransitionError) -> Self {
        use transition::TransitionError as T;
        match e {
            T::EmptySource => Self::EmptySource,
            T::WrongOwner => Self::WrongOwner,
            T::SameSquare => Self::SameSquare,
            T::FriendlyDestination => Self::FriendlyDestination,
            T::RoyalCapture => Self::RoyalCapture,
            T::Geometry => Self::Geometry,
            T::ResultFace => Self::ResultFace,
            T::InvalidDropFace => Self::InvalidDropFace,
            T::MissingHand => Self::MissingHand,
            T::OccupiedDrop => Self::OccupiedDrop,
            T::Capacity => Self::Capacity,
            T::InvalidToken => unreachable!("not an action-resolution error"),
        }
    }
}
/// Local root rejection, separate from structural validity and historical reachability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdmissionError {
    /// Phoenixes are adjacent.
    AdjacentRoyals,
    /// The side which is not to move is in check.
    NonmovingRoyalAttacked,
    /// Unpromoted Swallow stranded on its last rank.
    StrandedSwallow,
    /// More than two own unpromoted Swallows on a file.
    SwallowFile,
}
impl fmt::Display for AdmissionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tori local root rejected: {self:?}")
    }
}
impl Error for AdmissionError {}
/// Position observation, not a concluded match, repetition outcome or score.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalStatus {
    /// At least one legal action and no check.
    Ongoing,
    /// In check with a legal defense.
    Check,
    /// In check with no board-legal action;129 installs the corresponding loss.
    Checkmate,
    /// No check and no legal action; selected T05 is stalemate loss, never Chess draw.
    QuietNoAction,
}
/// Exact board or captured-resource origin for advisory read-only inspection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InspectionOrigin {
    /// Ownership comes from the native piece on this square.
    Board(Square),
    /// Inspect this owner's resource on a private hypothetical turn.
    Hand {
        /// Native owner, not the actual turn.
        side: Side,
        /// Exact base hand identity; no promoted/royal resources.
        face: Face,
    },
}
/// One complete inspected action with authoritative effects; not admission authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InspectedChoice {
    /// Exact supplied inspection origin.
    pub origin: InspectionOrigin,
    /// Native complete move/drop including compulsory resulting face.
    pub action: Action,
    /// Native effects on the hypothetical turn (no game/history events).
    pub effects: Effects,
}
/// Single-use local reversal token; fields are private and no structural caller can forge it.
#[derive(Debug)]
pub struct LocalUndo(transition::Undo);
/// Single-use local forward restoration retired by newly applied branches.
#[derive(Debug)]
pub struct LocalRedo(transition::Redo);
/// Failed reversal preserves position and token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReversalError {
    /// Wrong instance, order, used token, state or retired branch.
    InvalidToken,
    /// Non-rewound revision/node cannot advance.
    Capacity,
}
impl fmt::Display for ReversalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tori local reversal rejected: {self:?}")
    }
}
impl Error for ReversalError {}
impl From<transition::TransitionError> for ReversalError {
    fn from(e: transition::TransitionError) -> Self {
        match e {
            transition::TransitionError::InvalidToken => Self::InvalidToken,
            transition::TransitionError::Capacity => Self::Capacity,
            _ => unreachable!("not a reversal error"),
        }
    }
}
/// The single complete position-local legality boundary. No full-game permission.
///
/// ```
/// use mshogi_core::tori::LocalPosition;
/// let mut position = LocalPosition::standard();
/// let action = position.legal_actions()[0];
/// let original = position.position().state().clone();
/// let (_, mut undo) = position.apply(action).unwrap();
/// let mut redo = position.undo(&mut undo).unwrap();
/// assert_eq!(position.position().state(), &original);
/// position.redo(&mut redo).unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct LocalPosition {
    position: Position,
}
impl LocalPosition {
    /// Admit a structural root under T07. Moving-side check and terminal roots are
    /// permitted; neither custom nor imported roots gain a reachability claim.
    pub fn admit(position: Position) -> Result<Self, AdmissionError> {
        for side in [Side::Sente, Side::Gote] {
            for file in 1..=7 {
                let swallows: Vec<_> = Square::all()
                    .filter(|s| {
                        s.file() == file
                            && position.piece(*s)
                                == Some(Piece {
                                    side,
                                    face: Face::Swallow,
                                })
                    })
                    .collect();
                if swallows.iter().any(|s| last_rank(side, s.rank())) {
                    return Err(AdmissionError::StrandedSwallow);
                }
                if swallows.len() > 2 {
                    return Err(AdmissionError::SwallowFile);
                }
            }
        }
        let s = royal(&position, Side::Sente);
        let g = royal(&position, Side::Gote);
        if s.file().abs_diff(g.file()) <= 1 && s.rank().abs_diff(g.rank()) <= 1 {
            return Err(AdmissionError::AdjacentRoyals);
        }
        if in_check(&position, position.side_to_move().opponent()) {
            return Err(AdmissionError::NonmovingRoyalAttacked);
        }
        Ok(Self { position })
    }
    pub(super) fn board_probe(&self) -> Self {
        Self {
            position: rule_probe(&self.position, self.position.side_to_move()),
        }
    }
    /// Standard admitted32-piece setup.
    pub fn standard() -> Self {
        Self::admit(Position::standard()).expect("selected standard root")
    }
    /// Immutable complete structural state/provenance; no mutation escape hatch.
    pub fn position(&self) -> &Position {
        &self.position
    }
    /// Geometric native attackers, including pinned pieces and defended endpoints.
    /// This is a threat observation, not permission to move the attacker.
    pub fn attackers(&self, target: Square, by: Side) -> Vec<Square> {
        attackers(&self.position, target, by)
    }
    /// Check observation for either side, never a match conclusion.
    pub fn is_in_check(&self, side: Side) -> bool {
        in_check(&self.position, side)
    }
    /// All complete legal board choices in canonical square/face iteration order.
    /// Identical hand copies produce one choice. Board rules do not depend on
    /// counters, history or whether a future Game has already concluded.
    pub fn legal_actions(&self) -> Vec<Action> {
        legal_actions(&self.position)
    }
    /// Complete board-rule membership, independent of counter representability.
    pub fn is_legal(&self, action: Action) -> bool {
        resolve_legal(
            &rule_probe(&self.position, self.position.side_to_move()),
            action,
        )
        .is_ok()
    }
    /// Atomically commit one complete local choice using127's native transition.
    /// Requires separate129 permission in a real Game. Capacity failure is explicit.
    pub fn apply(&mut self, action: Action) -> Result<(Effects, LocalUndo), ActionError> {
        resolve_legal(
            &rule_probe(&self.position, self.position.side_to_move()),
            action,
        )?;
        let (effects, undo) = self.position.apply_structural(action)?;
        Ok((effects, LocalUndo(undo)))
    }
    /// Exact position-bound LIFO reversal, not semantic full-game undo.
    pub fn undo(&mut self, undo: &mut LocalUndo) -> Result<LocalRedo, ReversalError> {
        Ok(LocalRedo(self.position.undo_structural(&mut undo.0)?))
    }
    /// Exact forward restoration with fresh local Undo authority.
    pub fn redo(&mut self, redo: &mut LocalRedo) -> Result<(Effects, LocalUndo), ReversalError> {
        let (e, u) = self.position.redo_structural(&mut redo.0)?;
        Ok((e, LocalUndo(u)))
    }
    /// Exact local check/no-action observation. Counters cannot manufacture a mate.
    pub fn status(&self) -> LocalStatus {
        match (
            self.is_in_check(self.position.side_to_move()),
            !self.legal_actions().is_empty(),
        ) {
            (false, true) => LocalStatus::Ongoing,
            (true, true) => LocalStatus::Check,
            (true, false) => LocalStatus::Checkmate,
            (false, false) => LocalStatus::QuietNoAction,
        }
    }
    /// Read-only complete board/hand inspection for either native owner. Empty or
    /// unavailable/invalid origins have no choices. The actual turn, metadata,
    /// counters and reversal identities are untouched. Inspection of an opponent
    /// never authorizes a live action and never re-admits a hypothetical root.
    pub fn inspect(&self, origin: InspectionOrigin) -> Vec<InspectedChoice> {
        let owner = match origin {
            InspectionOrigin::Board(sq) => match self.position.piece(sq) {
                Some(p) => p.side,
                None => return Vec::new(),
            },
            InspectionOrigin::Hand { side, face } => {
                if self.position.hand(side).count(face) == 0 {
                    return Vec::new();
                }
                side
            }
        };
        let p = rule_probe(&self.position, owner);
        legal_actions(&p)
            .into_iter()
            .filter(|a| match (origin, a) {
                (InspectionOrigin::Board(s), Action::Move { from, .. }) => s == *from,
                (InspectionOrigin::Hand { face, .. }, Action::Drop { face: f, .. }) => face == *f,
                _ => false,
            })
            .map(|action| InspectedChoice {
                origin,
                action,
                effects: resolve_legal(&p, action).expect("already enumerated local choice"),
            })
            .collect()
    }
}
fn last_rank(side: Side, rank: u8) -> bool {
    match side {
        Side::Sente => rank == 1,
        Side::Gote => rank == 7,
    }
}
fn royal(position: &Position, side: Side) -> Square {
    Square::all()
        .find(|s| {
            position.piece(*s)
                == Some(Piece {
                    side,
                    face: Face::Phoenix,
                })
        })
        .expect("structural invariant: exactly one Phoenix per side")
}
fn attackers(position: &Position, target: Square, by: Side) -> Vec<Square> {
    Square::all()
        .filter(|s| {
            position.piece(*s).is_some_and(|p| p.side == by)
                && position.geometrically_reaches(*s, target)
        })
        .collect()
}
fn in_check(position: &Position, side: Side) -> bool {
    !attackers(position, royal(position, side), side.opponent()).is_empty()
}
// Board queries are finite position-local facts, independent of representational
// counters. This private copy is never installed or returned as actual history.
fn rule_probe(position: &Position, side: Side) -> Position {
    let mut p = position.clone();
    p.state.accepted_plies = 0;
    p.state.root.declared_completed_plies = 0;
    p.state.side = side;
    p
}
fn candidates(p: &Position) -> Vec<Action> {
    let side = p.side_to_move();
    let mut actions = Vec::new();
    for from in Square::all() {
        if let Some(piece) = p.piece(from).filter(|piece| piece.side == side) {
            for to in Square::all() {
                if p.geometrically_reaches(from, to) {
                    actions.push(Action::Move {
                        from,
                        to,
                        result_face: piece.face.moved_face(side, from, to),
                    });
                }
            }
        }
    }
    for face in Face::HANDS {
        if p.hand(side).count(face) > 0 {
            for to in Square::all() {
                actions.push(Action::Drop { face, to });
            }
        }
    }
    actions
}
// All local rules except the checking-Swallow-drop mate test. Used for the finite
// defense proof below, never as public legality or history-aware admission.
fn resolve_safety(p: &Position, action: Action) -> Result<(Position, Effects), ActionError> {
    let (state, effects) = p.resolve_structural(action)?;
    let mover = p.side_to_move();
    if let Action::Drop {
        face: Face::Swallow,
        to,
    } = action
    {
        if last_rank(mover, to.rank()) {
            return Err(ActionError::SwallowLastRank);
        }
        let count = Square::all()
            .filter(|s| {
                s.file() == to.file()
                    && p.piece(*s)
                        == Some(Piece {
                            side: mover,
                            face: Face::Swallow,
                        })
            })
            .count();
        if count >= 2 {
            return Err(ActionError::SwallowFile);
        }
    }
    let after = Position::install(state);
    if in_check(&after, mover) {
        return Err(ActionError::SelfCheck);
    }
    Ok((after, effects))
}
fn resolve_legal(p: &Position, action: Action) -> Result<Effects, ActionError> {
    let (after, effects) = resolve_safety(p, action)?;
    if let Action::Drop {
        face: Face::Swallow,
        to,
    } = action
    {
        let defender = after.side_to_move();
        if after.geometrically_reaches(to, royal(&after, defender)) {
            // The direct check is adjacent and cannot be interposed. A drop
            // neither moves the Phoenix nor captures the checker, so EVERY
            // defending drop fails self-check before its own mate rule matters.
            // Consequently all safety-valid replies here are board moves and
            // are fully legal; no recursive mate predicate or repetition input.
            let probe = rule_probe(&after, defender);
            if !candidates(&probe)
                .into_iter()
                .any(|a| resolve_safety(&probe, a).is_ok())
            {
                return Err(ActionError::SwallowDropMate);
            }
        }
    }
    Ok(effects)
}
fn legal_actions(p: &Position) -> Vec<Action> {
    let probe = rule_probe(p, p.side_to_move());
    candidates(&probe)
        .into_iter()
        .filter(|a| resolve_legal(&probe, *a).is_ok())
        .collect()
}
