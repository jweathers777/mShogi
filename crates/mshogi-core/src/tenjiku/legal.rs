//! Authoritative complete local turns. History, royal endings and game admission live above this layer.
use super::{Base, Face, Piece, Position, Profile, PromotionRight, RootError, Side, Square};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, error::Error, fmt, sync::Arc};

/// The two native complete-action shapes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionKind {
    /// One or more actual movement landings.
    Travel,
    /// Heavenly Tetrarch's stationary adjacent capture.
    Igui,
}
/// A whole player choice; victims and effects are never input authority.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Action {
    /// Native action shape; fields must agree with this tag.
    pub kind: ActionKind,
    /// Actual original square.
    pub from: Square,
    /// Actual visited landings, never crossed/skipped squares.
    pub route: Vec<Square>,
    /// Requested once-only promotion, resolved after survival.
    pub promote: bool,
    /// Required only for stationary igui; absent for travel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<Square>,
}
impl<'de> Deserialize<'de> for Action {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        fn present<'de, D: serde::Deserializer<'de>>(
            d: D,
        ) -> Result<Option<Option<Square>>, D::Error> {
            Option::<Square>::deserialize(d).map(Some)
        }
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            kind: ActionKind,
            from: Square,
            route: Vec<Square>,
            promote: bool,
            #[serde(default, deserialize_with = "present")]
            target: Option<Option<Square>>,
        }
        let w = Wire::deserialize(d)?;
        match w.kind {
            ActionKind::Travel if w.target.is_some() || w.route.is_empty() || w.route.len() > 3 => {
                return Err(serde::de::Error::custom("invalid Tenjiku travel fields"));
            }
            ActionKind::Igui
                if !w.route.is_empty() || w.promote || w.target.flatten().is_none() =>
            {
                return Err(serde::de::Error::custom("invalid Tenjiku igui fields"));
            }
            _ => {}
        }
        Ok(Self {
            kind: w.kind,
            from: w.from,
            route: w.route,
            promote: w.promote,
            target: w.target.flatten(),
        })
    }
}
impl Action {
    /// A complete nonpromoting one-landing move.
    pub fn single(from: Square, to: Square) -> Self {
        Self {
            kind: ActionKind::Travel,
            from,
            route: vec![to],
            promote: false,
            target: None,
        }
    }
    /// Complete stationary Tetrarch capture; admission still checks target/owner.
    pub fn igui(from: Square, target: Square) -> Self {
        Self {
            kind: ActionKind::Igui,
            from,
            route: vec![],
            promote: false,
            target: Some(target),
        }
    }
    /// Actual final placement; stationary actions remain at origin.
    pub fn to(&self) -> Option<Square> {
        match self.kind {
            ActionKind::Travel => self.route.last().copied(),
            ActionKind::Igui => Some(self.from),
        }
    }
}
/// An exact native victim; captures never change this original identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Capture {
    /// Actual square of removal.
    pub square: Square,
    /// Victim before removal.
    pub piece: Piece,
}
/// Ordered native phases; one Burn batch is simultaneous, irrespective of list order.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "phase", rename_all = "snake_case", deny_unknown_fields)]
pub enum EffectPhase {
    /// Direct landing/stationary capture, in route order.
    Capture {
        /// Native removed victim.
        victim: Capture,
    },
    /// Simultaneous bystanders in absolute rank/file canonical order.
    Burn {
        /// All victims from one phase snapshot.
        victims: Vec<Capture>,
    },
    /// Final-position enemy Demon hazard destroys the mover once.
    MoverDies {
        /// Actual final square.
        square: Square,
        /// Mover immediately before destruction.
        piece: Piece,
    },
    /// Surviving once-only promotion after all removals.
    Promotion {
        /// Actual final square.
        square: Square,
        /// Original mover before its flip.
        before: Piece,
        /// Same original after its flip.
        after: Piece,
    },
}
/// Complete native-derived effects from one accepted local turn.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Effects {
    /// Acting owner.
    pub actor: Side,
    /// Whole admitted choice.
    pub action: Action,
    /// Exact original mover and refusal right before resolution.
    pub mover_before: Piece,
    /// Exact surviving mover, or None after its hazard death.
    pub mover_after: Option<Piece>,
    /// Ordered atomic effects, including simultaneous batches.
    pub phases: Vec<EffectPhase>,
    /// Deterministic actual local work; not a game result or elapsed-time claim.
    pub work_units: u64,
}
impl Effects {
    /// Native enemy removals in phase order; mover suicide is separate.
    pub fn captures(&self) -> impl Iterator<Item = &Capture> {
        self.phases.iter().flat_map(|p| match p {
            EffectPhase::Capture { victim } => std::slice::from_ref(victim).iter(),
            EffectPhase::Burn { victims } => victims.iter(),
            _ => [].iter(),
        })
    }
}
/// A bounded native query and its actual deterministic work accounting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryResult<T> {
    /// Complete value; stopped queries never publish partial answers.
    pub value: T,
    /// Actual board-cell copies, generation attempts and resolution visits charged.
    pub work_units: u64,
}
/// Local refusal; all failures preserve the participant's live state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionError {
    /// Empty source.
    EmptySource,
    /// Actual source belongs to the other owner.
    WrongOwner,
    /// Incorrect shape/tag/route length/target.
    InvalidRoute,
    /// Native movement/blocking/capture rule does not admit the route.
    Geometry,
    /// An actual landing/stationary target has a friendly occupant.
    FriendlyDestination,
    /// Stationary capture lacks a victim.
    EmptyTarget,
    /// Unavailable/mandatory/deferred/second promotion choice.
    Promotion,
    /// Counter or local reversal authority is exhausted.
    Capacity,
    /// Caller requested cooperative stop; no partial action/query is published.
    Stopped,
}
impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tenjiku local action rejected: {self:?}")
    }
}
impl Error for ActionError {}
/// Invalid/stale/foreign local reversal, or exhausted authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReversalError {
    /// Token is used, foreign, out of order or belongs to a retired branch.
    InvalidToken,
    /// Monotonic local runtime authority exhausted.
    Capacity,
}
impl fmt::Display for ReversalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tenjiku local reversal rejected: {self:?}")
    }
}
impl Error for ReversalError {}
/// Instance-bound single-use LIFO local reversal, never canonical history.
#[derive(Debug)]
pub struct LocalUndo {
    owner: Arc<()>,
    before: Position,
    after: Position,
    before_node: u64,
    after_node: u64,
    used: bool,
    effects: Effects,
}
/// Single-use exact forward restoration; a newly accepted branch retires it.
#[derive(Debug)]
pub struct LocalRedo {
    owner: Arc<()>,
    before: Position,
    after: Position,
    before_node: u64,
    after_node: u64,
    branch: u64,
    used: bool,
    effects: Effects,
}
/// Explicit hypothetical complete choice; never live-game permission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InspectedChoice {
    /// Actual inspected origin.
    pub origin: Square,
    /// Complete local choice on this piece owner's hypothetical turn.
    pub action: Action,
    /// Actual hypothetical native effects.
    pub effects: Effects,
}
/// Local rules owner, independent of game history, repetition and endings.
#[derive(Debug)]
pub struct LocalPosition {
    position: Position,
    owner: Arc<()>,
    node: u64,
    next_node: u64,
    branch: u64,
    revision: u64,
}
impl Clone for LocalPosition {
    fn clone(&self) -> Self {
        Self::install(self.position.clone())
    }
}
impl PartialEq for LocalPosition {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}
impl Eq for LocalPosition {}
impl LocalPosition {
    fn install(position: Position) -> Self {
        Self {
            position,
            owner: Arc::new(()),
            node: 0,
            next_node: 1,
            branch: 0,
            revision: 0,
        }
    }
    /// Capture-based local admission: royals may be exposed, adjacent or missing.
    pub fn admit(position: Position) -> Result<Self, RootError> {
        Ok(Self::install(Position::install(position.state.clone())?))
    }
    /// Full native initial local board.
    pub fn standard(profile: Profile) -> Self {
        Self::install(Position::standard(profile))
    }
    /// Exact immutable structural authority.
    pub fn position(&self) -> &Position {
        &self.position
    }
    /// Complete local actual-turn actions, including promotion choices.
    pub fn legal_actions(&self) -> Vec<Action> {
        self.legal_actions_metered(|| false)
            .expect("finite native local query")
            .value
    }
    /// Cancellable complete local action set; no partial result after stop.
    pub fn legal_actions_with_stop(
        &self,
        stop: impl FnMut() -> bool,
    ) -> Result<Vec<Action>, ActionError> {
        self.legal_actions_metered(stop).map(|q| q.value)
    }
    /// Same query with actual deterministic work for native game admission.
    pub fn legal_actions_metered(
        &self,
        mut stop: impl FnMut() -> bool,
    ) -> Result<QueryResult<Vec<Action>>, ActionError> {
        let mut w = Work::new(&mut stop);
        let mut out = BTreeSet::new();
        for from in Square::all() {
            w.tick(1)?;
            if self
                .position
                .piece(from)
                .is_some_and(|p| p.side == self.position.side_to_move())
            {
                out.extend(self.choices(from, false, &mut w)?);
            }
        }
        Ok(QueryResult {
            value: out.into_iter().collect(),
            work_units: w.used,
        })
    }
    /// Actual-turn local choices for one owned source.
    pub fn choices_from(&self, from: Square) -> Vec<Action> {
        self.choices_from_metered(from, || false)
            .expect("finite native origin query")
            .value
    }
    /// Cancellable metered actual-turn origin query.
    pub fn choices_from_metered(
        &self,
        from: Square,
        mut stop: impl FnMut() -> bool,
    ) -> Result<QueryResult<Vec<Action>>, ActionError> {
        let mut w = Work::new(&mut stop);
        let value = self.choices(from, false, &mut w)?;
        Ok(QueryResult {
            value,
            work_units: w.used,
        })
    }
    /// Nonpromoting geometry on the actual piece owner's hypothetical turn.
    /// Iguis require an enemy target; promotion/turn/history/conclusions do not prune this set.
    pub fn geometry_actions_from(&self, from: Square) -> Vec<Action> {
        self.geometry_actions_from_metered(from, || false)
            .expect("finite native geometry query")
            .value
    }
    /// Cancellable exact geometry set with work; rights/counters are not permission filters.
    pub fn geometry_actions_from_metered(
        &self,
        from: Square,
        mut stop: impl FnMut() -> bool,
    ) -> Result<QueryResult<Vec<Action>>, ActionError> {
        let mut w = Work::new(&mut stop);
        let value = self.choices(from, true, &mut w)?;
        Ok(QueryResult {
            value,
            work_units: w.used,
        })
    }
    /// Read-only hypothetical full local choices/effects for either owner's piece.
    pub fn inspection(&self, from: Square) -> Vec<InspectedChoice> {
        self.inspection_metered(from, || false)
            .expect("finite native inspection")
            .value
    }
    /// Cancellable inspection without mutating counters, rights or local authority.
    pub fn inspection_metered(
        &self,
        from: Square,
        mut stop: impl FnMut() -> bool,
    ) -> Result<QueryResult<Vec<InspectedChoice>>, ActionError> {
        let mut w = Work::new(&mut stop);
        let mut copy = self.position.clone();
        w.tick(256)?;
        let Some(p) = copy.piece(from) else {
            return Ok(QueryResult {
                value: vec![],
                work_units: w.used,
            });
        };
        copy.state.side = p.side;
        let local = Self::install(copy);
        let mut value = vec![];
        for action in local.choices(from, false, &mut w)? {
            let (_, effects) = resolve(&local.position, &action, false, true, &mut w)?;
            value.push(InspectedChoice {
                origin: from,
                action,
                effects,
            });
        }
        Ok(QueryResult {
            value,
            work_units: w.used,
        })
    }
    fn choices(
        &self,
        from: Square,
        geometry: bool,
        w: &mut Work<'_>,
    ) -> Result<Vec<Action>, ActionError> {
        w.tick(1)?;
        let Some(p) = self.position.piece(from) else {
            return Ok(vec![]);
        };
        if !geometry && p.side != self.position.side_to_move() {
            return Ok(vec![]);
        }
        let mut base = self.position.clone();
        w.tick(256)?;
        base.state.side = p.side;
        let mut out = BTreeSet::new();
        for action in candidates(&base, from, p, w)? {
            if geometry {
                match resolve(&base, &action, false, false, w) {
                    Ok(_) => {
                        out.insert(action);
                    }
                    Err(e @ (ActionError::Stopped | ActionError::Capacity)) => return Err(e),
                    Err(_) => {}
                }
            } else {
                for promote in [false, true] {
                    let mut a = action.clone();
                    a.promote = promote;
                    match resolve(&base, &a, false, true, w) {
                        Ok(_) => {
                            out.insert(a);
                        }
                        Err(e @ (ActionError::Stopped | ActionError::Capacity)) => return Err(e),
                        Err(_) => {}
                    }
                }
            }
        }
        Ok(out.into_iter().collect())
    }
    /// Private semantic continuation without advancing representational counters.
    /// Turn, original identities and refusal rights are resolved normally.
    pub fn simulate(&self, action: &Action) -> Result<(Self, Effects), ActionError> {
        self.simulate_with_stop(action, || false)
    }
    /// Cancellable private continuation; no runtime authority or history is advanced.
    pub fn simulate_with_stop(
        &self,
        action: &Action,
        mut stop: impl FnMut() -> bool,
    ) -> Result<(Self, Effects), ActionError> {
        let mut w = Work::new(&mut stop);
        let (p, e) = resolve(&self.position, action, false, true, &mut w)?;
        Ok((Self::install(p), e))
    }
    /// Atomically apply a complete native local action.
    pub fn apply(&mut self, action: &Action) -> Result<(Effects, LocalUndo), ActionError> {
        self.apply_with_stop(action, || false)
    }
    /// Cancellable atomic apply; stops/refusals preserve every logical/runtime field.
    pub fn apply_with_stop(
        &mut self,
        action: &Action,
        mut stop: impl FnMut() -> bool,
    ) -> Result<(Effects, LocalUndo), ActionError> {
        let mut w = Work::new(&mut stop);
        let (after, mut effects) = resolve(&self.position, action, true, true, &mut w)?;
        let revision = self.revision.checked_add(1).ok_or(ActionError::Capacity)?;
        let next = self.next_node.checked_add(1).ok_or(ActionError::Capacity)?;
        let branch = self.branch.checked_add(1).ok_or(ActionError::Capacity)?;
        w.tick(512)?;
        effects.work_units = w.used;
        let token = LocalUndo {
            owner: self.owner.clone(),
            before: self.position.clone(),
            after: after.clone(),
            before_node: self.node,
            after_node: self.next_node,
            used: false,
            effects: effects.clone(),
        };
        self.position = after;
        self.node = self.next_node;
        self.next_node = next;
        self.branch = branch;
        self.revision = revision;
        Ok((effects, token))
    }
    /// Exact instance-bound LIFO restoration; runtime identity remains monotonic.
    pub fn undo(&mut self, t: &mut LocalUndo) -> Result<LocalRedo, ReversalError> {
        if t.used
            || !Arc::ptr_eq(&self.owner, &t.owner)
            || self.node != t.after_node
            || self.position != t.after
        {
            return Err(ReversalError::InvalidToken);
        }
        let revision = self
            .revision
            .checked_add(1)
            .ok_or(ReversalError::Capacity)?;
        let redo = LocalRedo {
            owner: self.owner.clone(),
            before: t.before.clone(),
            after: t.after.clone(),
            before_node: t.before_node,
            after_node: t.after_node,
            branch: self.branch,
            used: false,
            effects: t.effects.clone(),
        };
        self.position = t.before.clone();
        self.node = t.before_node;
        self.revision = revision;
        t.used = true;
        Ok(redo)
    }
    /// Exact semantic redo only on its original surviving branch.
    pub fn redo(&mut self, t: &mut LocalRedo) -> Result<(Effects, LocalUndo), ReversalError> {
        if t.used
            || !Arc::ptr_eq(&self.owner, &t.owner)
            || self.node != t.before_node
            || self.branch != t.branch
            || self.position != t.before
        {
            return Err(ReversalError::InvalidToken);
        }
        let revision = self
            .revision
            .checked_add(1)
            .ok_or(ReversalError::Capacity)?;
        let undo = LocalUndo {
            owner: self.owner.clone(),
            before: t.before.clone(),
            after: t.after.clone(),
            before_node: t.before_node,
            after_node: t.after_node,
            used: false,
            effects: t.effects.clone(),
        };
        self.position = t.after.clone();
        self.node = t.after_node;
        self.revision = revision;
        t.used = true;
        Ok((t.effects.clone(), undo))
    }
}
struct Work<'a> {
    used: u64,
    stop: &'a mut dyn FnMut() -> bool,
}
impl<'a> Work<'a> {
    fn new(stop: &'a mut dyn FnMut() -> bool) -> Self {
        Self { used: 0, stop }
    }
    fn tick(&mut self, n: u64) -> Result<(), ActionError> {
        if (self.stop)() {
            return Err(ActionError::Stopped);
        }
        self.used = self.used.checked_add(n).ok_or(ActionError::Capacity)?;
        Ok(())
    }
}

const O: &[(i8, i8)] = &[(0, -1), (0, 1), (-1, 0), (1, 0)];
const D: &[(i8, i8)] = &[(-1, -1), (1, -1), (-1, 1), (1, 1)];
const A: &[(i8, i8)] = &[
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (1, -1),
    (-1, 1),
    (1, 1),
];
#[derive(Clone, Copy)]
enum Special {
    None,
    Lion,
    Area,
    Ranked,
    RankedArea,
    ForwardLion,
    DiagonalLion,
    Tetrarch,
    DiagonalDouble,
}
struct Movement {
    steps: &'static [(i8, i8)],
    rays: &'static [(i8, i8)],
    jumps: &'static [(i8, i8)],
    limited: &'static [(i8, i8)],
    special: Special,
}
fn movement(face: Face, profile: Profile) -> Movement {
    use Face::*;
    use Special::*;
    type Directions = &'static [(i8, i8)];
    let (steps, rays, jumps, limited, special): (
        Directions,
        Directions,
        Directions,
        Directions,
        Special,
    ) = match face {
        K | CP => (A, &[], &[], &[], None),
        FiD => (
            &[],
            &[(0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            Area,
        ),
        GGn => (&[], A, &[], &[], Ranked),
        VGn => (&[], D, &[], &[], RankedArea),
        RGn => (&[], O, &[], &[], Ranked),
        BGn => (&[], D, &[], &[], Ranked),
        LHk => (&[], D, &[], &[], Lion),
        Ln => (&[], &[], &[], &[], Lion),
        WBf => (
            &[],
            &[(-1, 0), (1, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[(0, -1), (0, 1)],
            None,
        ),
        FEg => {
            if profile == Profile::AdamsModern {
                (&[], A, &[(0, -2), (0, 2), (-2, 0), (2, 0)], &[], None)
            } else {
                (&[], A, &[], &[], DiagonalDouble)
            }
        }
        FK => (&[], A, &[], &[], None),
        HT => (&[], &[], &[], &[], Tetrarch),
        SE => (
            &[],
            &[(0, -1), (0, 1), (-1, 0), (1, 0), (-1, 1), (1, 1)],
            &[],
            &[],
            DiagonalLion,
        ),
        HF => (
            &[],
            &[(0, 1), (-1, 0), (1, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            ForwardLion,
        ),
        ChS => (
            &[],
            &[(0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[(-1, 0), (1, 0)],
            None,
        ),
        DK => (D, O, &[], &[], None),
        DH => (O, D, &[], &[], None),
        FO => (
            &[],
            &[(0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            None,
        ),
        FB => (
            &[],
            &[(-1, 0), (1, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            None,
        ),
        R => (&[], O, &[], &[], None),
        B => (&[], D, &[], &[], None),
        Ph => (O, &[], &[(-2, -2), (2, -2), (-2, 2), (2, 2)], &[], None),
        VSo => (&[(0, 1)], &[(0, -1)], &[], &[(-1, 0), (1, 0)], None),
        VM => (&[(-1, 0), (1, 0)], &[(0, -1), (0, 1)], &[], &[], None),
        SSo => (&[(0, 1)], &[(-1, 0), (1, 0)], &[], &[(0, -1)], None),
        SM => (&[(0, -1), (0, 1)], &[(-1, 0), (1, 0)], &[], &[], None),
        WH => (&[], &[(0, -1), (0, 1), (-1, -1), (1, -1)], &[], &[], None),
        W => (&[], &[(0, -1), (0, 1), (-1, 1), (1, 1)], &[], &[], None),
        Ky => (D, &[], &[(0, -2), (0, 2), (-2, 0), (2, 0)], &[], None),
        FS => (
            &[(-1, 0), (1, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[(0, -1), (0, 1)],
            &[],
            &[],
            None,
        ),
        RC => (&[], &[(0, -1), (0, 1)], &[], &[], None),
        L => (&[], &[(0, -1)], &[], &[], None),
        MGn => (&[], &[(0, -1), (-1, 1), (1, 1)], &[], &[], None),
        G => (
            &[(0, -1), (0, 1), (-1, 0), (1, 0), (-1, -1), (1, -1)],
            &[],
            &[],
            &[],
            None,
        ),
        DE => (
            &[(0, -1), (-1, 0), (1, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            &[],
            None,
        ),
        BT => (
            &[(0, 1), (-1, 0), (1, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            &[],
            None,
        ),
        FL => (
            &[(0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            &[],
            None,
        ),
        S => (
            &[(0, -1), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            &[],
            None,
        ),
        C => (&[(0, -1), (0, 1), (-1, -1), (1, -1)], &[], &[], &[], None),
        I => (&[(0, -1), (-1, -1), (1, -1)], &[], &[], &[], None),
        N => (&[], &[], &[(-1, -2), (1, -2)], &[], None),
        Dg => (&[(0, -1), (-1, 1), (1, 1)], &[], &[], &[], None),
        P => (&[(0, -1)], &[], &[], &[], None),
    };
    Movement {
        steps,
        rays,
        jumps,
        limited,
        special,
    }
}
fn candidates(
    p: &Position,
    from: Square,
    piece: Piece,
    w: &mut Work<'_>,
) -> Result<Vec<Action>, ActionError> {
    let m = movement(piece.face(), p.profile());
    let mut out = BTreeSet::new();
    for &(df, dr) in m.steps.iter().chain(m.jumps) {
        w.tick(1)?;
        if let Some(to) = from.offset(piece.side, df, dr) {
            out.insert(Action::single(from, to));
        }
    }
    for &(df, dr) in m.limited {
        for n in 1..=2 {
            w.tick(1)?;
            let Some(to) = from.offset(piece.side, df * n, dr * n) else {
                break;
            };
            out.insert(Action::single(from, to));
            if p.piece(to).is_some() {
                break;
            }
        }
    }
    for &(df, dr) in m.rays {
        for n in 1..=15 {
            w.tick(1)?;
            let Some(to) = from.offset(piece.side, df * n, dr * n) else {
                break;
            };
            out.insert(Action::single(from, to));
            if let Some(q) = p.piece(to)
                && (!matches!(m.special, Special::Ranked | Special::RankedArea)
                    || q.face().rank() >= piece.face().rank())
            {
                break;
            }
        }
    }
    match m.special {
        Special::Lion => {
            for df in -2..=2 {
                for dr in -2..=2 {
                    w.tick(1)?;
                    if (df != 0 || dr != 0)
                        && let Some(to) = from.offset(piece.side, df, dr)
                    {
                        out.insert(Action::single(from, to));
                    }
                }
            }
            for &(df, dr) in A {
                if let Some(mid) = from.offset(piece.side, df, dr) {
                    for &(f, r) in A {
                        w.tick(1)?;
                        if let Some(to) = mid.offset(piece.side, f, r) {
                            let mut a = Action::single(from, mid);
                            a.route.push(to);
                            out.insert(a);
                        }
                    }
                }
            }
        }
        Special::Area | Special::RankedArea => {
            area_paths(from, from, piece.side, &mut vec![], &mut out, w)?
        }
        Special::ForwardLion | Special::DiagonalLion => {
            let dirs: &[(i8, i8)] = if matches!(m.special, Special::ForwardLion) {
                &[(0, -1)]
            } else {
                &[(-1, -1), (1, -1)]
            };
            for &(df, dr) in dirs {
                w.tick(1)?;
                if let Some(mid) = from.offset(piece.side, df, dr) {
                    out.insert(Action::single(from, mid));
                    let mut a = Action::single(from, mid);
                    a.route.push(from);
                    out.insert(a);
                    if let Some(to) = mid.offset(piece.side, df, dr) {
                        out.insert(Action::single(from, to));
                        let mut a = Action::single(from, mid);
                        a.route.push(to);
                        out.insert(a);
                    }
                }
            }
        }
        Special::DiagonalDouble => {
            for &(df, dr) in D {
                if let Some(mid) = from.offset(piece.side, df, dr) {
                    for &(f, r) in D {
                        w.tick(1)?;
                        if let Some(to) = mid.offset(piece.side, f, r) {
                            let mut a = Action::single(from, mid);
                            a.route.push(to);
                            out.insert(a);
                        }
                    }
                }
            }
        }
        Special::Tetrarch => {
            for &(df, dr) in A {
                w.tick(1)?;
                if let Some(target) = from.offset(piece.side, df, dr)
                    && p.piece(target).is_some_and(|q| q.side != piece.side)
                {
                    out.insert(Action::igui(from, target));
                }
                if dr == 0 {
                    for n in 2..=3 {
                        w.tick(1)?;
                        let Some(to) = from.offset(piece.side, df * n, 0) else {
                            break;
                        };
                        out.insert(Action::single(from, to));
                        if p.piece(to).is_some() {
                            break;
                        }
                    }
                } else if df != 0 || p.profile() == Profile::MullerCompleted {
                    for n in 2..=15 {
                        w.tick(1)?;
                        let Some(to) = from.offset(piece.side, df * n, dr * n) else {
                            break;
                        };
                        out.insert(Action::single(from, to));
                        if p.piece(to).is_some() {
                            break;
                        }
                    }
                }
            }
        }
        Special::None | Special::Ranked => {}
    }
    Ok(out.into_iter().collect())
}
fn area_paths(
    from: Square,
    at: Square,
    side: Side,
    path: &mut Vec<Square>,
    out: &mut BTreeSet<Action>,
    w: &mut Work<'_>,
) -> Result<(), ActionError> {
    if path.len() == 3 {
        return Ok(());
    }
    for &(df, dr) in A {
        w.tick(1)?;
        if let Some(to) = at.offset(side, df, dr) {
            path.push(to);
            out.insert(Action {
                kind: ActionKind::Travel,
                from,
                route: path.clone(),
                promote: false,
                target: None,
            });
            area_paths(from, to, side, path, out, w)?;
            path.pop();
        }
    }
    Ok(())
}
fn line_distance(from: Square, to: Square, side: Side, df: i8, dr: i8) -> Option<u8> {
    (1..=15u8).find(|&n| from.offset(side, df * n as i8, dr * n as i8) == Some(to))
}
#[derive(Clone, Copy)]
enum RayMode {
    Ordinary,
    Ranked,
    SkipFirst,
}
fn ray(
    p: &Position,
    from: Square,
    to: Square,
    piece: Piece,
    directions: &[(i8, i8)],
    mode: RayMode,
    w: &mut Work<'_>,
) -> Result<bool, ActionError> {
    for &(df, dr) in directions {
        w.tick(1)?;
        if let Some(n) = line_distance(from, to, piece.side, df, dr) {
            let mut crossed = false;
            for i in 1..n {
                w.tick(1)?;
                if matches!(mode, RayMode::SkipFirst) && i == 1 {
                    continue;
                }
                let s = from
                    .offset(piece.side, df * i as i8, dr * i as i8)
                    .expect("bounded aligned route");
                if let Some(q) = p.piece(s) {
                    if !matches!(mode, RayMode::Ranked) || q.face().rank() >= piece.face().rank() {
                        return Ok(false);
                    }
                    crossed = true;
                }
            }
            if !crossed {
                return Ok(true);
            }
            return Ok(p.piece(to).is_some_and(|q| {
                q.side != piece.side
                    && q.face().rank() != 4
                    && (p.profile() == Profile::MullerCompleted
                        || q.face().rank() < piece.face().rank())
            }));
        }
    }
    Ok(false)
}
fn geometry(p: &Position, piece: Piece, a: &Action, w: &mut Work<'_>) -> Result<bool, ActionError> {
    let m = movement(piece.face(), p.profile());
    if a.kind == ActionKind::Igui {
        return Ok(matches!(m.special, Special::Tetrarch)
            && a.route.is_empty()
            && !a.promote
            && a.target.is_some_and(|s| a.from.distance(s) == 1));
    }
    if a.target.is_some() || a.route.is_empty() || a.route.len() > 3 {
        return Ok(false);
    }
    if let [to] = a.route.as_slice() {
        if *to == a.from {
            return Ok(false);
        }
        for &(df, dr) in m.steps.iter().chain(m.jumps) {
            w.tick(1)?;
            if a.from.offset(piece.side, df, dr) == Some(*to) {
                return Ok(true);
            }
        }
        if ray(
            p,
            a.from,
            *to,
            piece,
            m.rays,
            if matches!(m.special, Special::Ranked | Special::RankedArea) {
                RayMode::Ranked
            } else {
                RayMode::Ordinary
            },
            w,
        )? {
            return Ok(true);
        }
        for &(df, dr) in m.limited {
            w.tick(1)?;
            if let Some(n) = line_distance(a.from, *to, piece.side, df, dr)
                && n <= 2
                && ray(p, a.from, *to, piece, &[(df, dr)], RayMode::Ordinary, w)?
            {
                return Ok(true);
            }
        }
        if matches!(m.special, Special::Lion) && a.from.distance(*to) <= 2 {
            return Ok(true);
        }
        if matches!(m.special, Special::ForwardLion | Special::DiagonalLion) {
            let dirs: &[(i8, i8)] = if matches!(m.special, Special::ForwardLion) {
                &[(0, -1)]
            } else {
                &[(-1, -1), (1, -1)]
            };
            for &(df, dr) in dirs {
                if (1..=2).any(|n| a.from.offset(piece.side, df * n, dr * n) == Some(*to)) {
                    return Ok(true);
                }
            }
        }
        if matches!(m.special, Special::Tetrarch) {
            if a.from.distance(*to) < 2 {
                return Ok(false);
            }
            if a.from.rank() == to.rank() {
                let n = a.from.file().abs_diff(to.file());
                return if n == 2 {
                    Ok(true)
                } else if n == 3 {
                    let df = if to.file() > a.from.file() { 1 } else { -1 };
                    let mid = Square::new((i16::from(a.from.file()) + 2 * df) as u8, a.from.rank())
                        .expect("bounded horizontal route");
                    Ok(p.piece(mid).is_none())
                } else {
                    Ok(false)
                };
            }
            let dirs = if p.profile() == Profile::AdamsModern {
                D
            } else {
                &[(0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)]
            };
            return ray(p, a.from, *to, piece, dirs, RayMode::SkipFirst, w);
        }
    }
    if matches!(m.special, Special::Area | Special::RankedArea) {
        let mut at = a.from;
        for &to in &a.route {
            w.tick(1)?;
            if at.distance(to) != 1 {
                return Ok(false);
            }
            at = to;
        }
        return Ok(true);
    }
    if let [mid, to] = a.route.as_slice() {
        return Ok(match m.special {
            Special::Lion => a.from.distance(*mid) == 1 && mid.distance(*to) == 1,
            Special::ForwardLion | Special::DiagonalLion => {
                let dirs: &[(i8, i8)] = if matches!(m.special, Special::ForwardLion) {
                    &[(0, -1)]
                } else {
                    &[(-1, -1), (1, -1)]
                };
                dirs.iter().any(|&(df, dr)| {
                    a.from.offset(piece.side, df, dr) == Some(*mid)
                        && (*to == a.from || mid.offset(piece.side, df, dr) == Some(*to))
                })
            }
            Special::DiagonalDouble => {
                a.from.file().abs_diff(mid.file()) == 1
                    && a.from.rank().abs_diff(mid.rank()) == 1
                    && mid.file().abs_diff(to.file()) == 1
                    && mid.rank().abs_diff(to.rank()) == 1
            }
            _ => false,
        });
    }
    Ok(false)
}
fn mandatory(p: Piece, to: Square) -> bool {
    if p.promoted {
        return false;
    }
    match p.base {
        Base::P | Base::L | Base::I => p.side.last_rank(to.rank()),
        Base::N => match p.side {
            Side::Sente => to.rank() <= 2,
            Side::Gote => to.rank() >= 15,
        },
        _ => false,
    }
}
fn resolve(
    p: &Position,
    a: &Action,
    advance: bool,
    promotion_rules: bool,
    w: &mut Work<'_>,
) -> Result<(Position, Effects), ActionError> {
    let start = w.used;
    w.tick(1)?;
    let before = p.piece(a.from).ok_or(ActionError::EmptySource)?;
    if before.side != p.side_to_move() {
        return Err(ActionError::WrongOwner);
    }
    if (a.kind == ActionKind::Travel
        && (a.route.is_empty() || a.route.len() > 3 || a.target.is_some()))
        || (a.kind == ActionKind::Igui && (!a.route.is_empty() || a.target.is_none()))
    {
        return Err(ActionError::InvalidRoute);
    }
    if !geometry(p, before, a, w)? {
        return Err(ActionError::Geometry);
    }
    w.tick(256)?;
    let mut next = p.clone();
    let mut phases = vec![];
    let mut captured = false;
    let to = a.to().ok_or(ActionError::InvalidRoute)?;
    if a.kind == ActionKind::Igui {
        let target = a.target.ok_or(ActionError::InvalidRoute)?;
        w.tick(1)?;
        let victim = next.piece(target).ok_or(ActionError::EmptyTarget)?;
        if victim.side == before.side {
            return Err(ActionError::FriendlyDestination);
        }
        next.state.board[target.index()] = None;
        phases.push(EffectPhase::Capture {
            victim: Capture {
                square: target,
                piece: victim,
            },
        });
        captured = true;
    } else {
        next.state.board[a.from.index()] = None;
        let mut at = a.from;
        let area = matches!(
            movement(before.face(), p.profile()).special,
            Special::Area | Special::RankedArea
        ) && a.route.len() > 1;
        for (i, &landing) in a.route.iter().enumerate() {
            w.tick(1)?;
            if let Some(victim) = next.piece(landing) {
                if victim.side == before.side {
                    return Err(ActionError::FriendlyDestination);
                }
                if area && i + 1 < a.route.len() {
                    return Err(ActionError::Geometry);
                }
                phases.push(EffectPhase::Capture {
                    victim: Capture {
                        square: landing,
                        piece: victim,
                    },
                });
                captured = true;
            }
            next.state.board[at.index()] = None;
            next.state.board[landing.index()] = Some(before);
            at = landing;
        }
    }
    let mut hazardous = false;
    for &(df, dr) in A {
        w.tick(1)?;
        if let Some(s) = to.offset(Side::Sente, df, dr)
            && next
                .piece(s)
                .is_some_and(|q| q.side != before.side && q.face() == Face::FiD)
        {
            hazardous = true;
        }
    }
    if before.face() == Face::FiD && (!hazardous || p.profile() == Profile::AdamsModern) {
        let mut victims = vec![];
        for &(df, dr) in A {
            w.tick(1)?;
            if let Some(s) = to.offset(Side::Sente, df, dr)
                && let Some(q) = next.piece(s)
                && q.side != before.side
                && q.face() != Face::FiD
            {
                victims.push(Capture {
                    square: s,
                    piece: q,
                });
            }
        }
        victims.sort_by_key(|v| (v.square.rank(), v.square.file()));
        if !victims.is_empty() {
            for v in &victims {
                next.state.board[v.square.index()] = None;
            }
            phases.push(EffectPhase::Burn { victims });
        }
    }
    let eligible = !before.promoted
        && before.base.promotable()
        && (before.side.in_zone(a.from.rank())
            || a.route.iter().any(|s| before.side.in_zone(s.rank())));
    let must = mandatory(before, to) && !hazardous;
    if promotion_rules {
        if a.promote
            && (!eligible
                || before.right == PromotionRight::FirstDeclinedNextMove && !captured && !must)
        {
            return Err(ActionError::Promotion);
        }
        if !a.promote && must {
            return Err(ActionError::Promotion);
        }
    }
    let mut after = before;
    if hazardous {
        next.state.board[to.index()] = None;
        phases.push(EffectPhase::MoverDies {
            square: to,
            piece: before,
        });
    } else if promotion_rules {
        if a.promote {
            after.promoted = true;
            after.right = PromotionRight::Consumed;
            phases.push(EffectPhase::Promotion {
                square: to,
                before,
                after,
            });
        } else if before.right == PromotionRight::FirstDeclinedNextMove {
            after.right = PromotionRight::Consumed;
        } else if eligible && before.right == PromotionRight::Unseen {
            after.right = PromotionRight::FirstDeclinedNextMove;
        }
        next.state.board[to.index()] = Some(after);
    }
    if advance {
        next.state.accepted_plies = next
            .state
            .accepted_plies
            .checked_add(1)
            .ok_or(ActionError::Capacity)?;
        next.state
            .root
            .declared_completed_plies()
            .checked_add(next.state.accepted_plies)
            .ok_or(ActionError::Capacity)?;
    }
    next.state.side = before.side.opponent();
    Ok((
        next,
        Effects {
            actor: before.side,
            action: a.clone(),
            mover_before: before,
            mover_after: (!hazardous).then_some(after),
            phases,
            work_units: w.used - start,
        },
    ))
}
