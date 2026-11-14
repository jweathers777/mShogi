//! Complete position-local rules, including ordered compound routes and Lion law.
//! No Chess self-check restriction and no history-dependent game outcome occurs here.
use super::{Base, Face, Piece, Position, RootError, Side, Square};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, error::Error, fmt, sync::Arc};

/// A whole turn. The route contains actual landings, not overflown jump squares.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Action {
    /// Exact native origin.
    pub from: Square,
    /// One or two ordered actual landings.
    pub route: Vec<Square>,
    /// Optional once-only flip at the final landing.
    pub promote: bool,
}
impl Action {
    /// Final landing; malformed empty routes have no endpoint.
    pub fn to(&self) -> Option<Square> {
        self.route.last().copied()
    }
    /// Complete nonpromoting one-landing action.
    pub fn single(from: Square, to: Square) -> Self {
        Self {
            from,
            route: vec![to],
            promote: false,
        }
    }
}
/// Ordered discarded victim; ownership/base/promotion are never demoted/transferred.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Capture {
    /// Actual capture landing.
    pub square: Square,
    /// Original victim identity and current face.
    pub piece: Piece,
}
/// Next-opponent-turn protection created by a non-Lion taking a Lion.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LionProtection {
    /// Protected native owner, absent when no rights exist.
    pub owner: Option<Side>,
    /// Canonically sorted defended Lion squares.
    pub squares: Vec<Square>,
}
impl LionProtection {
    /// No active right.
    pub const fn empty() -> Self {
        Self {
            owner: None,
            squares: Vec::new(),
        }
    }
    /// Protected squares in canonical order.
    pub fn protected_squares(&self) -> &[Square] {
        &self.squares
    }
    /// Whether the active right names this square and owner.
    pub fn protects(&self, side: Side, square: Square) -> bool {
        self.owner == Some(side) && self.squares.contains(&square)
    }
}
/// Authoritative derived facts from one whole action.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Effects {
    /// Actual mover.
    pub actor: Side,
    /// Complete accepted action.
    pub action: Action,
    /// Original mover before the action.
    pub mover_before: Piece,
    /// Same original identity after an optional one-time flip.
    pub mover_after: Piece,
    /// Ordered actual victims, never supplied by the caller.
    pub captures: Vec<Capture>,
    /// A nonempty return route with no capture.
    pub quiet_return: bool,
    /// Rights before this action.
    pub protection_before: LionProtection,
    /// Rights after this action.
    pub protection_after: LionProtection,
}
/// Local refusal; no game conclusion or process failure is implied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionError {
    /// Empty origin.
    EmptySource,
    /// Origin is not owned by the current player.
    WrongOwner,
    /// Route must contain exactly one or two actual landings.
    InvalidRoute,
    /// Impossible geometry, obstructed ray or invalid compound turn.
    Geometry,
    /// An actual landing is occupied by a friendly piece.
    FriendlyDestination,
    /// Illegal, repeated or unavailable promotion.
    Promotion,
    /// A Lion capture violates defended-distance or next-turn law.
    LionLaw,
    /// Counter or reversal identity cannot advance.
    Capacity,
}
impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chu local action rejected: {self:?}")
    }
}
impl Error for ActionError {}
/// Single-use LIFO reversal authority, bound to one local-position instance.
#[derive(Debug)]
pub struct LocalUndo {
    owner: Arc<()>,
    before: LogicalState,
    after: LogicalState,
    before_node: u64,
    after_node: u64,
    used: bool,
    effects: Effects,
}
/// Single-use forward restoration, invalidated by any newly accepted branch.
#[derive(Debug)]
pub struct LocalRedo {
    owner: Arc<()>,
    before: LogicalState,
    after: LogicalState,
    before_node: u64,
    after_node: u64,
    branch: u64,
    used: bool,
    effects: Effects,
}
/// Reversal failure, with state and token unchanged.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReversalError {
    /// Wrong instance, order, retired branch, altered state or used token.
    InvalidToken,
    /// Monotonic runtime identity exhausted.
    Capacity,
}
impl fmt::Display for ReversalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chu local reversal rejected: {self:?}")
    }
}
impl Error for ReversalError {}
#[derive(Clone, Debug, Eq, PartialEq)]
struct LogicalState {
    position: Position,
    protection: LionProtection,
}
/// Read-only complete native movement choice, never game admission authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InspectedChoice {
    /// Exact inspected native origin.
    pub origin: Square,
    /// Whole native action.
    pub action: Action,
    /// Derived hypothetical effects.
    pub effects: Effects,
}
/// Local complete-action owner, independent of live-game history and permission.
#[derive(Debug)]
pub struct LocalPosition {
    position: Position,
    protection: LionProtection,
    owner: Arc<()>,
    node: u64,
    next_node: u64,
    branch: u64,
    revision: u64,
}
impl Clone for LocalPosition {
    fn clone(&self) -> Self {
        Self::install(self.position.clone(), self.protection.clone())
    }
}
impl PartialEq for LocalPosition {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.protection == other.protection
    }
}
impl Eq for LocalPosition {}
impl LocalPosition {
    fn install(position: Position, protection: LionProtection) -> Self {
        Self {
            position,
            protection,
            owner: Arc::new(()),
            node: 0,
            next_node: 1,
            branch: 0,
            revision: 0,
        }
    }
    /// Capture-based root admission: no check, adjacency or dead-rank filter.
    pub fn admit(position: Position) -> Self {
        Self::install(position, LionProtection::empty())
    }
    /// Standard complete local position.
    pub fn standard() -> Self {
        Self::admit(Position::standard())
    }
    /// Validate a structural snapshot plus explicit transient right. This does not
    /// prove its prior history; canonical games must replay to derive the right.
    pub fn from_parts(position: Position, protection: LionProtection) -> Result<Self, RootError> {
        if protection.squares.is_empty() {
            if protection.owner.is_some() {
                return Err(RootError::InvalidProtection);
            }
        } else {
            if protection.owner != Some(position.side_to_move().opponent())
                || protection.squares.windows(2).any(|w| w[0] >= w[1])
            {
                return Err(RootError::InvalidProtection);
            }
            let side = protection.owner.ok_or(RootError::InvalidProtection)?;
            if protection.squares.iter().any(|s| {
                !position
                    .piece(*s)
                    .is_some_and(|p| p.side == side && p.is_lion())
                    || !is_defended_on(&position, side, *s)
            }) {
                return Err(RootError::InvalidProtection);
            }
        }
        Ok(Self::install(position, protection))
    }
    /// Exact structural state.
    pub fn position(&self) -> &Position {
        &self.position
    }
    /// Active native next-turn right.
    pub fn protection(&self) -> &LionProtection {
        &self.protection
    }
    /// All complete locally admitted actions, including optional flip alternatives.
    /// This is not a history-dependent permission or a match conclusion.
    pub fn legal_actions(&self) -> Vec<Action> {
        self.position
            .pieces()
            .filter(|(_, p)| p.side == self.position.side_to_move())
            .flat_map(|(s, _)| self.choices_from(s))
            .collect()
    }
    /// Complete local choices for the actual turn's owned origin.
    pub fn choices_from(&self, from: Square) -> Vec<Action> {
        let Some(p) = self.position.piece(from) else {
            return Vec::new();
        };
        if p.side != self.position.side_to_move() {
            return Vec::new();
        }
        let mut out = BTreeSet::new();
        for route in candidate_routes(&self.position, from, p) {
            let mut action = Action {
                from,
                route,
                promote: false,
            };
            if self.resolve(&action, false).is_ok() {
                out.insert(action.clone());
                action.promote = true;
                if self.resolve(&action, false).is_ok() {
                    out.insert(action);
                }
            }
        }
        out.into_iter().collect()
    }
    /// Read-only local choices on either native player's hypothetical turn.
    /// A protection right constrains its opponent only, never its owning side.
    pub fn legal_actions_for(&self, side: Side) -> Vec<Action> {
        let mut copy = self.clone();
        copy.position.state.side = side;
        if copy.protection.owner == Some(side) {
            copy.protection = LionProtection::empty()
        }
        copy.legal_actions()
    }
    /// Resolve one complete action on a private native-side turn, without
    /// counters, game permission, history events or mutation of the live state.
    pub fn preview_for(&self, side: Side, action: &Action) -> Result<Effects, ActionError> {
        let mut copy = self.clone();
        copy.position.state.side = side;
        if copy.protection.owner == Some(side) {
            copy.protection = LionProtection::empty()
        }
        copy.resolve(action, false).map(|(_, effects)| effects)
    }
    // Private rules-only continuation for finite adjudication/history projection.
    // Counters and reversal authority never advance; this is not a playable or
    // persistable participant transition. Real admission always calls apply.
    pub(crate) fn simulate(&self, action: &Action) -> Result<(Self, Effects), ActionError> {
        let (next, effects) = self.resolve(action, false)?;
        Ok((Self::install(next.position, next.protection), effects))
    }
    /// Native actual capture relations under complete local action admission.
    /// Duplicate relations across routes/promotion alternatives are collapsed.
    pub fn capture_influence(&self, side: Side) -> Vec<(Square, Square)> {
        let mut copy = self.clone();
        copy.position.state.side = side;
        if copy.protection.owner == Some(side) {
            copy.protection = LionProtection::empty()
        }
        let mut out = BTreeSet::new();
        for a in copy.legal_actions() {
            if let Ok((_, effects)) = copy.resolve(&a, false) {
                for c in effects.captures {
                    out.insert((a.from, c.square));
                }
            }
        }
        out.into_iter().collect()
    }
    /// Source-geometric defenders, including friendly defended endpoints.
    /// This read-only influence observation never grants action permission.
    pub fn geometric_defenders(&self, side: Side, square: Square) -> Vec<Square> {
        geometric_defenders_on(&self.position, side, square)
    }
    /// Native read-only inspection of either player's actual piece.
    pub fn inspection(&self, origin: Square) -> Vec<InspectedChoice> {
        let Some(piece) = self.position.piece(origin) else {
            return Vec::new();
        };
        let mut copy = self.clone();
        copy.position.state.side = piece.side;
        if copy.protection.owner == Some(piece.side) {
            copy.protection = LionProtection::empty()
        }
        copy.choices_from(origin)
            .into_iter()
            .filter_map(|action| {
                copy.resolve(&action, false)
                    .ok()
                    .map(|(_, effects)| InspectedChoice {
                        origin,
                        action,
                        effects,
                    })
            })
            .collect()
    }
    /// Atomically apply one complete locally legal action and issue exact reversal.
    pub fn apply(&mut self, action: &Action) -> Result<(Effects, LocalUndo), ActionError> {
        let (after, effects) = self.resolve(action, true)?;
        let revision = self.revision.checked_add(1).ok_or(ActionError::Capacity)?;
        let next = self.next_node.checked_add(1).ok_or(ActionError::Capacity)?;
        let branch = self.branch.checked_add(1).ok_or(ActionError::Capacity)?;
        let before = self.logical();
        let before_node = self.node;
        let after_node = self.next_node;
        self.position = after.position.clone();
        self.protection = after.protection.clone();
        self.node = after_node;
        self.next_node = next;
        self.revision = revision;
        self.branch = branch;
        Ok((
            effects.clone(),
            LocalUndo {
                owner: self.owner.clone(),
                before,
                after,
                before_node,
                after_node,
                used: false,
                effects,
            },
        ))
    }
    /// Exact restoration, including route effects, counters, metadata and rights.
    pub fn undo(&mut self, token: &mut LocalUndo) -> Result<LocalRedo, ReversalError> {
        if token.used
            || !Arc::ptr_eq(&self.owner, &token.owner)
            || self.node != token.after_node
            || self.logical() != token.after
        {
            return Err(ReversalError::InvalidToken);
        }
        let revision = self
            .revision
            .checked_add(1)
            .ok_or(ReversalError::Capacity)?;
        self.position = token.before.position.clone();
        self.protection = token.before.protection.clone();
        self.node = token.before_node;
        self.revision = revision;
        token.used = true;
        Ok(LocalRedo {
            owner: self.owner.clone(),
            before: token.before.clone(),
            after: token.after.clone(),
            before_node: token.before_node,
            after_node: token.after_node,
            branch: self.branch,
            used: false,
            effects: token.effects.clone(),
        })
    }
    /// Restore a reversed action only while its exact branch remains current.
    pub fn redo(&mut self, token: &mut LocalRedo) -> Result<(Effects, LocalUndo), ReversalError> {
        if token.used
            || !Arc::ptr_eq(&self.owner, &token.owner)
            || self.branch != token.branch
            || self.node != token.before_node
            || self.logical() != token.before
        {
            return Err(ReversalError::InvalidToken);
        }
        let revision = self
            .revision
            .checked_add(1)
            .ok_or(ReversalError::Capacity)?;
        self.position = token.after.position.clone();
        self.protection = token.after.protection.clone();
        self.node = token.after_node;
        self.revision = revision;
        token.used = true;
        Ok((
            token.effects.clone(),
            LocalUndo {
                owner: self.owner.clone(),
                before: token.before.clone(),
                after: token.after.clone(),
                before_node: token.before_node,
                after_node: token.after_node,
                used: false,
                effects: token.effects.clone(),
            },
        ))
    }
    fn logical(&self) -> LogicalState {
        LogicalState {
            position: self.position.clone(),
            protection: self.protection.clone(),
        }
    }
    fn resolve(
        &self,
        action: &Action,
        advance: bool,
    ) -> Result<(LogicalState, Effects), ActionError> {
        let before = self
            .position
            .piece(action.from)
            .ok_or(ActionError::EmptySource)?;
        if before.side != self.position.side_to_move() {
            return Err(ActionError::WrongOwner);
        }
        if action.route.is_empty() || action.route.len() > 2 {
            return Err(ActionError::InvalidRoute);
        }
        if !route_geometry(&self.position, before, action) {
            return Err(ActionError::Geometry);
        }
        let mut position = self.position.clone();
        position.state.board[action.from.index()] = None;
        let mut previous = action.from;
        let mut captures = Vec::new();
        for &landing in &action.route {
            if let Some(victim) = position.piece(landing) {
                if victim.side == before.side {
                    return Err(ActionError::FriendlyDestination);
                }
                captures.push(Capture {
                    square: landing,
                    piece: victim,
                });
            }
            position.state.board[previous.index()] = None;
            position.state.board[landing.index()] = Some(before);
            previous = landing;
        }
        let to = previous;
        let mut after = before;
        if action.promote {
            if before.promoted || !before.base.promotable() {
                return Err(ActionError::Promotion);
            }
            let from_zone = before.side.in_zone(action.from.rank());
            let to_zone = before.side.in_zone(to.rank());
            let eligible = (!from_zone && to_zone)
                || (!captures.is_empty() && (from_zone || to_zone))
                || (before.base == Base::Pawn && before.side.last_rank(to.rank()));
            if !eligible {
                return Err(ActionError::Promotion);
            }
            after.promoted = true;
            position.state.board[to.index()] = Some(after);
        }
        for (i, capture) in captures
            .iter()
            .enumerate()
            .filter(|(_, c)| c.piece.is_lion())
        {
            let qualifying = captures[..i]
                .iter()
                .any(|c| c.piece.promoted || !matches!(c.piece.base, Base::Pawn | Base::GoBetween));
            if self.protection.protects(capture.piece.side, capture.square) && !qualifying {
                return Err(ActionError::LionLaw);
            }
            if before.is_lion() && action.from.distance(capture.square) > 1 && !qualifying {
                // Keep the initial defender even when an intermediate Pawn or
                // Go-between was removed, and uncover rays behind the origin.
                let mut vacated = self.position.clone();
                vacated.state.board[action.from.index()] = None;
                if is_defended_on(&vacated, capture.piece.side, capture.square)
                    || is_defended_on(&position, capture.piece.side, capture.square)
                {
                    return Err(ActionError::LionLaw);
                }
            }
        }
        let mut protection = LionProtection::empty();
        if !before.is_lion() && captures.iter().any(|c| c.piece.is_lion()) {
            let mut squares: Vec<_> = position
                .pieces()
                .filter(|(s, p)| {
                    p.side == before.side && p.is_lion() && is_defended_on(&position, p.side, *s)
                })
                .map(|(s, _)| s)
                .collect();
            squares.sort_unstable();
            if !squares.is_empty() {
                protection = LionProtection {
                    owner: Some(before.side),
                    squares,
                };
            }
        }
        if advance {
            let count = position
                .state
                .accepted_plies
                .checked_add(1)
                .ok_or(ActionError::Capacity)?;
            position
                .state
                .root
                .declared_completed_plies()
                .checked_add(count)
                .ok_or(ActionError::Capacity)?;
            position.state.accepted_plies = count;
        }
        position.state.side = before.side.opponent();
        let effects = Effects {
            actor: before.side,
            action: action.clone(),
            mover_before: before,
            mover_after: after,
            quiet_return: to == action.from && captures.is_empty(),
            captures,
            protection_before: self.protection.clone(),
            protection_after: protection.clone(),
        };
        Ok((
            LogicalState {
                position,
                protection,
            },
            effects,
        ))
    }
}

/// Compound category in the movement descriptor, not a second rules engine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpecialMovement {
    /// Lion's two King steps or outer-ring jump.
    Lion,
    /// Either single forward diagonal, no turn on leg2.
    ForwardDiagonal,
    /// Straight forward, no turn on leg2.
    Forward,
}
/// Native source movement geometry. Offsets are Sente file/rank deltas; Gote
/// rotates both. These are geometry facts, not complete legal-action choices.
#[derive(Clone, Copy, Debug)]
pub struct Movement {
    /// Unit step offsets.
    pub steps: &'static [(i8, i8)],
    /// Direct jump offsets.
    pub jumps: &'static [(i8, i8)],
    /// Unbounded ray directions until board edge/blocker.
    pub rays: &'static [(i8, i8)],
    /// Complete-action special geometry.
    pub special: Option<SpecialMovement>,
}
const ORTH: &[(i8, i8)] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];
const DIAG: &[(i8, i8)] = &[(-1, -1), (1, -1), (1, 1), (-1, 1)];
const KING: &[(i8, i8)] = &[
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
    (-1, -1),
    (1, -1),
    (1, 1),
    (-1, 1),
];
/// Exact movement table transcribed from JCSA C3, independent of presentation.
pub fn movement(face: Face) -> Movement {
    use Face::*;
    type Directions = &'static [(i8, i8)];
    let (steps, jumps, rays, special): (Directions, Directions, Directions, _) = match face {
        Pawn => (&[(0, -1)], &[], &[], None),
        GoBetween => (&[(0, -1), (0, 1)], &[], &[], None),
        Lance => (&[], &[], &[(0, -1)], None),
        ReverseChariot => (&[], &[], &[(0, -1), (0, 1)], None),
        Copper => (&[(-1, -1), (0, -1), (1, -1), (0, 1)], &[], &[], None),
        Silver => (
            &[(-1, -1), (0, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            None,
        ),
        Gold => (
            &[(0, -1), (1, 0), (0, 1), (-1, 0), (-1, -1), (1, -1)],
            &[],
            &[],
            None,
        ),
        Leopard => (
            &[(-1, -1), (1, -1), (1, 1), (-1, 1), (0, -1), (0, 1)],
            &[],
            &[],
            None,
        ),
        Tiger => (
            &[(1, 0), (0, 1), (-1, 0), (-1, -1), (1, -1), (1, 1), (-1, 1)],
            &[],
            &[],
            None,
        ),
        Elephant => (
            &[(0, -1), (1, 0), (-1, 0), (-1, -1), (1, -1), (1, 1), (-1, 1)],
            &[],
            &[],
            None,
        ),
        King | Prince => (KING, &[], &[], None),
        Kirin => (DIAG, &[(0, -2), (2, 0), (0, 2), (-2, 0)], &[], None),
        Phoenix => (ORTH, &[(-2, -2), (2, -2), (2, 2), (-2, 2)], &[], None),
        SideMover => (&[(0, -1), (0, 1)], &[], &[(1, 0), (-1, 0)], None),
        VerticalMover => (&[(1, 0), (-1, 0)], &[], &[(0, -1), (0, 1)], None),
        Rook => (&[], &[], ORTH, None),
        Bishop => (&[], &[], DIAG, None),
        DragonHorse => (ORTH, &[], DIAG, None),
        DragonKing => (DIAG, &[], ORTH, None),
        Lion => (&[], &[], &[], Some(SpecialMovement::Lion)),
        FreeKing => (&[], &[], KING, None),
        WhiteHorse => (&[], &[], &[(0, -1), (-1, -1), (1, -1), (0, 1)], None),
        Whale => (&[], &[], &[(0, -1), (0, 1), (-1, 1), (1, 1)], None),
        FlyingStag => (
            &[(-1, -1), (1, -1), (1, 1), (-1, 1), (1, 0), (-1, 0)],
            &[],
            &[(0, -1), (0, 1)],
            None,
        ),
        FlyingOx => (
            &[],
            &[],
            &[(-1, -1), (1, -1), (1, 1), (-1, 1), (0, -1), (0, 1)],
            None,
        ),
        FreeBoar => (
            &[],
            &[],
            &[(-1, -1), (1, -1), (1, 1), (-1, 1), (1, 0), (-1, 0)],
            None,
        ),
        SoaringEagle => (
            &[],
            &[],
            &[(0, -1), (1, 0), (0, 1), (-1, 0), (-1, 1), (1, 1)],
            Some(SpecialMovement::ForwardDiagonal),
        ),
        HornedFalcon => (
            &[],
            &[],
            &[(-1, -1), (1, -1), (1, 1), (-1, 1), (1, 0), (-1, 0), (0, 1)],
            Some(SpecialMovement::Forward),
        ),
    };
    Movement {
        steps,
        jumps,
        rays,
        special,
    }
}

fn ray_reaches(
    position: &Position,
    from: Square,
    side: Side,
    to: Square,
    directions: &[(i8, i8)],
) -> bool {
    for &(df, dr) in directions {
        for n in 1..12 {
            let Some(square) = from.offset(side, df * n, dr * n) else {
                break;
            };
            if square == to {
                return true;
            }
            if position.piece(square).is_some() {
                break;
            }
        }
    }
    false
}
fn route_geometry(position: &Position, piece: Piece, action: &Action) -> bool {
    let m = movement(piece.face());
    let from = action.from;
    match action.route.as_slice() {
        [to] => {
            if from == *to {
                return false;
            }
            if m.steps
                .iter()
                .chain(m.jumps)
                .any(|&(f, r)| from.offset(piece.side, f, r) == Some(*to))
                || ray_reaches(position, from, piece.side, *to, m.rays)
            {
                return true;
            }
            match m.special {
                Some(SpecialMovement::Lion) => from.distance(*to) <= 2,
                Some(SpecialMovement::ForwardDiagonal) => [-1, 1]
                    .into_iter()
                    .any(|x| (1..=2).any(|n| from.offset(piece.side, x * n, -n) == Some(*to))),
                Some(SpecialMovement::Forward) => {
                    (1..=2).any(|n| from.offset(piece.side, 0, -n) == Some(*to))
                }
                None => false,
            }
        }
        [middle, to] => match m.special {
            Some(SpecialMovement::Lion) => from.distance(*middle) == 1 && middle.distance(*to) == 1,
            Some(SpecialMovement::ForwardDiagonal) => [-1, 1].into_iter().any(|x| {
                from.offset(piece.side, x, -1) == Some(*middle)
                    && (*to == from || middle.offset(piece.side, x, -1) == Some(*to))
            }),
            Some(SpecialMovement::Forward) => {
                from.offset(piece.side, 0, -1) == Some(*middle)
                    && (*to == from || middle.offset(piece.side, 0, -1) == Some(*to))
            }
            None => false,
        },
        _ => false,
    }
}
fn candidate_routes(position: &Position, from: Square, piece: Piece) -> Vec<Vec<Square>> {
    let m = movement(piece.face());
    let mut routes = BTreeSet::new();
    for &(f, r) in m.steps.iter().chain(m.jumps) {
        if let Some(to) = from.offset(piece.side, f, r) {
            routes.insert(vec![to]);
        }
    }
    for &(f, r) in m.rays {
        for n in 1..12 {
            let Some(to) = from.offset(piece.side, f * n, r * n) else {
                break;
            };
            routes.insert(vec![to]);
            if position.piece(to).is_some() {
                break;
            }
        }
    }
    if let Some(special) = m.special {
        match special {
            SpecialMovement::Lion => {
                for df in -2i8..=2 {
                    for dr in -2i8..=2 {
                        if (df != 0 || dr != 0)
                            && let Some(to) = from.offset(piece.side, df, dr)
                        {
                            routes.insert(vec![to]);
                        }
                    }
                }
                for &(f, r) in KING {
                    if let Some(mid) = from.offset(piece.side, f, r) {
                        for &(f2, r2) in KING {
                            if let Some(to) = mid.offset(piece.side, f2, r2) {
                                routes.insert(vec![mid, to]);
                            }
                        }
                    }
                }
            }
            SpecialMovement::ForwardDiagonal | SpecialMovement::Forward => {
                let dirs: &[(i8, i8)] = if special == SpecialMovement::Forward {
                    &[(0, -1)]
                } else {
                    &[(-1, -1), (1, -1)]
                };
                for &(f, r) in dirs {
                    if let Some(mid) = from.offset(piece.side, f, r) {
                        routes.insert(vec![mid]);
                        routes.insert(vec![mid, from]);
                        if let Some(to) = from.offset(piece.side, f * 2, r * 2) {
                            routes.insert(vec![to]);
                            routes.insert(vec![mid, to]);
                        }
                    }
                }
            }
        }
    }
    routes.into_iter().collect()
}
/// Capture influence used for Lion defense. The endpoint may hold a friendly
/// defended piece; intervening occupancy still blocks ordinary rays/landings.
/// No recursive Lion-exchange test or royal self-check prohibition is imported.
fn is_defended_on(position: &Position, side: Side, target: Square) -> bool {
    !geometric_defenders_on(position, side, target).is_empty()
}
fn geometric_defenders_on(position: &Position, side: Side, target: Square) -> Vec<Square> {
    position
        .pieces()
        .filter_map(|(from, p)| {
            (p.side == side && from != target && {
                let m = movement(p.face());
                m.steps
                    .iter()
                    .chain(m.jumps)
                    .any(|&(f, r)| from.offset(p.side, f, r) == Some(target))
                    || ray_reaches(position, from, p.side, target, m.rays)
                    || match m.special {
                        Some(SpecialMovement::Lion) => from.distance(target) <= 2,
                        Some(SpecialMovement::ForwardDiagonal) => [-1, 1].into_iter().any(|x| {
                            (1..=2).any(|n| from.offset(p.side, x * n, -n) == Some(target))
                        }),
                        Some(SpecialMovement::Forward) => {
                            (1..=2).any(|n| from.offset(p.side, 0, -n) == Some(target))
                        }
                        None => false,
                    }
            })
            .then_some(from)
        })
        .collect()
}
