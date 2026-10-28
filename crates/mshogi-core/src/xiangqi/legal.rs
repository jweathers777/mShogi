//! Local board rules only; no history permissions, claims or WXF outcomes.
use super::{
    Action, ActionError, Counters, Intersection, PieceKind, Position, ResolvedAction, Side, Undo,
};
use std::{error::Error, fmt};

/// Structural diagrams may be useful without being admissible local live roots.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalPositionError {
    /// A piece occupies an impossible home/geography point for its native kind.
    PieceGeography {
        /// Off-domain occupied intersection.
        at: Intersection,
    },
    /// The player to move already attacks the non-moving General, including facing Generals.
    NonMovingGeneralAttacked,
}
impl fmt::Display for LocalPositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "xiangqi local root rejected: {self:?}")
    }
}
impl Error for LocalPositionError {}

/// Local reason for having no action. Both lose under WXF; not a concluded game.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoActionReason {
    /// No evasion of a geometric check exists.
    Checkmate,
    /// No action exists although the General is not checked.
    Stalemate,
}
/// A board observation, not history-dependent permission to continue or an outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalStatus {
    /// At least one local action is available.
    Ongoing {
        /// Geometric check on the player to move.
        in_check: bool,
    },
    /// No local action; preserve the native reason for the later game owner.
    NoLegalAction {
        /// Checked versus unchecked local no-action reason.
        reason: NoActionReason,
    },
}

impl Position {
    /// Admit static local-rule geography and turn safety. This does not prove
    /// historical reachability, available history claims or a still-running game.
    pub fn validate_local(&self) -> Result<(), LocalPositionError> {
        for (at, piece) in self.pieces() {
            let home_rank = match piece.side {
                Side::Red => at.rank(),
                Side::Black => 9 - at.rank(),
            };
            let valid = match piece.kind {
                PieceKind::General => piece.side.in_palace(at),
                PieceKind::Advisor => piece.side.in_palace(at) && (at.file() + home_rank) % 2 == 1,
                PieceKind::Elephant => {
                    matches!((at.file(), home_rank), (2 | 6, 0 | 4) | (0 | 4 | 8, 2))
                }
                PieceKind::Soldier => home_rank >= 5 || (home_rank >= 3 && at.file() % 2 == 0),
                PieceKind::Horse | PieceKind::Rook | PieceKind::Cannon => true,
            };
            if !valid {
                return Err(LocalPositionError::PieceGeography { at });
            }
        }
        if self.is_in_check(self.side.opponent()) {
            return Err(LocalPositionError::NonMovingGeneralAttacked);
        }
        Ok(())
    }

    fn general(&self, side: Side) -> Intersection {
        self.pieces()
            .find_map(|(at, piece)| {
                (piece.side == side && piece.kind == PieceKind::General).then_some(at)
            })
            .expect("structural Position has exactly one General per side")
    }

    fn intervening(&self, from: Intersection, to: Intersection) -> Option<usize> {
        let dx = i16::from(to.file()) - i16::from(from.file());
        let dy = i16::from(to.rank()) - i16::from(from.rank());
        if (dx == 0) == (dy == 0) {
            return None;
        }
        let (sx, sy) = (dx.signum(), dy.signum());
        let mut x = i16::from(from.file()) + sx;
        let mut y = i16::from(from.rank()) + sy;
        let mut count = 0;
        while x != i16::from(to.file()) || y != i16::from(to.rank()) {
            let at =
                Intersection::new(x as u8, y as u8).expect("between bounded aligned endpoints");
            count += usize::from(self.piece(at).is_some());
            x += sx;
            y += sy;
        }
        Some(count)
    }

    // Capture-mode geometry is also used for geometric threats: pinned pieces
    // still threaten the General. No recursive legal-move definition of check.
    fn geometry(&self, action: Action, capture: bool) -> bool {
        let Some(piece) = self.piece(action.from) else {
            return false;
        };
        if action.from == action.to {
            return false;
        }
        let dx = i16::from(action.to.file()) - i16::from(action.from.file());
        let dy = i16::from(action.to.rank()) - i16::from(action.from.rank());
        let (ax, ay) = (dx.abs(), dy.abs());
        let midpoint_empty = |x: i16, y: i16| {
            Intersection::new(
                (i16::from(action.from.file()) + x) as u8,
                (i16::from(action.from.rank()) + y) as u8,
            )
            .is_some_and(|at| self.piece(at).is_none())
        };
        match piece.kind {
            PieceKind::Rook => self.intervening(action.from, action.to) == Some(0),
            PieceKind::Cannon => {
                self.intervening(action.from, action.to) == Some(usize::from(capture))
            }
            PieceKind::Horse => match (ax, ay) {
                (2, 1) => midpoint_empty(dx.signum(), 0),
                (1, 2) => midpoint_empty(0, dy.signum()),
                _ => false,
            },
            PieceKind::Elephant => {
                ax == 2
                    && ay == 2
                    && !piece.side.has_crossed_river(action.to)
                    && midpoint_empty(dx / 2, dy / 2)
            }
            PieceKind::Advisor => ax == 1 && ay == 1 && piece.side.in_palace(action.to),
            PieceKind::General => {
                (ax + ay == 1 && piece.side.in_palace(action.to))
                    || (capture
                        && dx == 0
                        && self.piece(action.to).is_some_and(|target| {
                            target.kind == PieceKind::General && target.side != piece.side
                        })
                        && self.intervening(action.from, action.to) == Some(0))
            }
            PieceKind::Soldier => {
                (dx == 0 && dy == i16::from(piece.side.forward()))
                    || (dy == 0 && ax == 1 && piece.side.has_crossed_river(action.from))
            }
        }
    }

    /// Geometric capture threat, including pinned attackers and flying Generals.
    /// Works on structural diagnostics without live admission. Friendly target
    /// occupancy does not suppress defense; empty targets use capture-mode
    /// geometry (not a cannon's quiet movement). Distant flying-General threats
    /// require an opposing General actually at the target; this is not a
    /// hypothetical General-placement query.
    pub fn is_attacked(&self, at: Intersection, by: Side) -> bool {
        self.pieces()
            .any(|(from, piece)| piece.side == by && self.geometry(Action { from, to: at }, true))
    }
    /// Geometric General check on a structural diagram, not game adjudication.
    pub fn is_in_check(&self, side: Side) -> bool {
        self.is_attacked(self.general(side), side.opponent())
    }

    /// Complete movement/capture choices before own-General safety filtering.
    /// General captures are never board actions. Requires static live admission.
    pub fn pseudo_legal_actions(&self) -> Result<Vec<Action>, LocalPositionError> {
        self.validate_local()?;
        Ok(self.pseudo_unchecked())
    }
    fn pseudo_unchecked(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        for (from, _) in self.pieces().filter(|(_, piece)| piece.side == self.side) {
            for to in Intersection::all() {
                let target = self.piece(to);
                if target.is_some_and(|target| {
                    target.side == self.side || target.kind == PieceKind::General
                }) {
                    continue;
                }
                let action = Action { from, to };
                if self.geometry(action, target.is_some()) {
                    actions.push(action);
                }
            }
        }
        actions
    }
    fn safe_after(&self, action: Action) -> bool {
        let mut after = self.clone();
        after.board[action.to.index()] = after.board[action.from.index()];
        after.board[action.from.index()] = None;
        !after.is_in_check(self.side)
    }
    /// Entire local set, independent of declared counters and all history policy.
    /// Invalid structural roots return an error, never a misleading empty set.
    pub fn legal_actions(&self) -> Result<Vec<Action>, LocalPositionError> {
        self.validate_local()?;
        Ok(self
            .pseudo_unchecked()
            .into_iter()
            .filter(|action| self.safe_after(*action))
            .collect())
    }
    /// Read-only complete movement inspection for either native participant.
    ///
    /// This is a hypothetical local movement query, not permission to submit an
    /// out-of-turn action or bypass game history/adjudication. The actual diagram
    /// must satisfy local admission first. Only then is the queried owner used
    /// for movement and own-General safety filtering; the hypothetical opponent
    /// may already be checked, which is not grounds to reject an inspection.
    /// The position, counters and actual side to move remain unchanged.
    pub fn inspection_actions(&self, side: Side) -> Result<Vec<Action>, LocalPositionError> {
        self.validate_local()?;
        let mut inspected = self.clone();
        inspected.side = side;
        Ok(inspected
            .pseudo_unchecked()
            .into_iter()
            .filter(|action| inspected.safe_after(*action))
            .collect())
    }
    /// Local no-action reasons are preserved for the future game owner.
    pub fn local_status(&self) -> Result<LocalStatus, LocalPositionError> {
        let checked = self.is_in_check(self.side);
        if self.legal_actions()?.is_empty() {
            Ok(LocalStatus::NoLegalAction {
                reason: if checked {
                    NoActionReason::Checkmate
                } else {
                    NoActionReason::Stalemate
                },
            })
        } else {
            Ok(LocalStatus::Ongoing { in_check: checked })
        }
    }
    /// Resolve a complete action against native rules and the exact current value.
    /// Representation exhaustion is separate from illegal movement or outcomes.
    pub fn resolve_action(&self, action: Action) -> Result<ResolvedAction, ActionError> {
        self.validate_local()
            .map_err(ActionError::InvalidPosition)?;
        // Run structural checks without allowing counters to mask rule errors.
        let mut query = self.clone();
        query.counters = Counters::new(Some(0), Some(1)).expect("known normalized counters");
        query.prepare_structural(action)?;
        if !self.geometry(action, self.piece(action.to).is_some()) {
            return Err(ActionError::Movement);
        }
        if !self.safe_after(action) {
            return Err(ActionError::GeneralExposed);
        }
        self.prepare_structural(action)
    }
    /// Apply only a rules-resolved action bound to its complete before-state.
    /// This local value guard is not a future game/request/process identity.
    pub fn apply_resolved(&mut self, action: ResolvedAction) -> Result<Undo, ActionError> {
        self.commit_resolved(action)
    }
    /// Atomically resolve and apply one complete local board choice.
    pub fn apply(&mut self, action: Action) -> Result<Undo, ActionError> {
        let resolved = self.resolve_action(action)?;
        self.apply_resolved(resolved)
    }
}
