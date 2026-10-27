//! Complete local board legality, independent of game history and conclusions.

use super::{Action, ActionError, Hand, Piece, PieceKind, Position, Side, Square, Undo, UndoError};

/// Exact board/hand/turn repetition identity, without counters or root provenance.
///
/// Equality alone counts no occurrences and concludes no match. This is an
/// opaque native key, not a portable record encoding or a promised hash format.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RepetitionIdentity {
    board: [Option<Piece>; 81],
    hands: [Hand; 2],
    side: Side,
}

impl Position {
    /// Read-only complete local movement/drop choices for an owner's hypothetical
    /// turn. Uses native king safety, promotion and drop restrictions on a private
    /// copy; does not establish a game outcome or permission to commit a command.
    pub fn inspection_actions(&self, side: Side) -> Vec<Action> {
        let mut copy = self.clone();
        copy.side = side;
        copy.legal_actions()
    }

    /// Geometric attackers of a square in SFEN square order.
    ///
    /// Occupancy stops rays, including at a friendly defended target. Pinned
    /// pieces still influence king safety. This query is distinct from whether
    /// moving an attacker would preserve its own king, or from legal choices.
    pub fn attackers(&self, target: Square, by: Side) -> impl Iterator<Item = Square> + '_ {
        Square::all().filter(move |&from| {
            self.piece(from).is_some_and(|p| p.side == by)
                && self.geometrically_reaches(from, target)
        })
    }

    /// Whether `by` geometrically attacks `target` under current occupancy.
    pub fn is_square_attacked(&self, target: Square, by: Side) -> bool {
        self.attackers(target, by).next().is_some()
    }

    /// Whether this side's king is attacked; does not conclude a game.
    pub fn is_in_check(&self, side: Side) -> bool {
        self.is_square_attacked(self.king_square(side), side.opponent())
    }

    fn king_square(&self, side: Side) -> Square {
        Square::all()
            .find(|&s| {
                self.piece(s)
                    .is_some_and(|p| p.side == side && p.kind == PieceKind::King)
            })
            .expect("admitted positions contain exactly one king per side")
    }

    /// Complete structurally admissible choices in USI lexical order.
    ///
    /// These check geometry, ownership, resources, promotion, nifu and dead
    /// ranks, but omit king safety and pawn-drop mate. They are deliberately
    /// named candidates, never an alternative action-acceptance boundary.
    /// Counters and game/history conclusions do not prune these choices.
    pub fn pseudo_legal_actions(&self) -> Vec<Action> {
        let working = self.normalized_for_board_queries();
        let mut actions = Vec::new();
        for from in Square::all() {
            if !working.piece(from).is_some_and(|p| p.side == working.side) {
                continue;
            }
            for to in Square::all() {
                if !working.geometrically_reaches(from, to) {
                    continue;
                }
                for promote in [false, true] {
                    let action = Action::Move { from, to, promote };
                    if working.clone().apply_structural(action).is_ok() {
                        actions.push(action);
                    }
                }
            }
        }
        for kind in PieceKind::HAND_KINDS {
            if working.hand(working.side).count(kind) == 0 {
                continue;
            }
            for to in Square::all() {
                let action = Action::Drop { kind, to };
                if working.clone().apply_structural(action).is_ok() {
                    actions.push(action);
                }
            }
        }
        actions.sort_by_cached_key(ToString::to_string);
        actions
    }

    /// Every complete local legal Move/Drop, including every legal nonpromotion.
    ///
    /// Deterministic USI lexical order; no search pruning or duplicate choices
    /// for identical hand resources. This includes king safety and uchifuzume,
    /// but does not adjudicate repetition, declaration or the 320-ply policy.
    /// A diagram can have choices even when a future match classifies it ended.
    /// Counter overflow never makes this list spuriously empty: only committing
    /// an action at that representation boundary returns CounterOverflow.
    pub fn legal_actions(&self) -> Vec<Action> {
        let working = self.normalized_for_board_queries();
        working
            .pseudo_legal_actions()
            .into_iter()
            .filter(|&action| working.clone().apply(action).is_ok())
            .collect()
    }

    /// Complete local rule membership, independent of counter representability.
    pub fn is_legal(&self, action: Action) -> bool {
        self.normalized_for_board_queries().apply(action).is_ok()
    }

    /// Atomically commit one complete local legal board action.
    ///
    /// All capture/promotion/hand effects are native. Rejection preserves every
    /// field, including counter and root provenance. Match/session request
    /// admission and history-dependent outcomes belong to the game layer.
    pub fn apply(&mut self, action: Action) -> Result<Undo, ActionError> {
        let mover = self.side;
        let mut candidate = self.clone();
        let undo = candidate.apply_structural(action)?;
        if candidate.is_in_check(mover) {
            return Err(ActionError::SelfCheck);
        }
        if let Action::Drop {
            kind: PieceKind::Pawn,
            to,
        } = action
        {
            // Require the newly dropped pawn itself to check. Structural
            // diagrams may already attack the opposing king before this turn.
            if candidate.geometrically_reaches(to, candidate.king_square(mover.opponent()))
                && !candidate.has_king_safe_reply()
            {
                return Err(ActionError::PawnDropMate);
            }
        }
        *self = candidate;
        Ok(undo)
    }

    // A checking dropped pawn is adjacent to the king: it cannot be blocked.
    // A response must move the king or capture that pawn, with full king safety.
    // Thus no answering drop can evade it, and no recursive pawn-drop-mate
    // test is needed. All promotion variants of defender captures remain here.
    fn has_king_safe_reply(&self) -> bool {
        let working = self.normalized_for_board_queries();
        working.pseudo_legal_actions().into_iter().any(|action| {
            let mut candidate = working.clone();
            candidate.apply_structural(action).is_ok() && !candidate.is_in_check(working.side)
        })
    }

    /// Restore an exact local apply record in LIFO order.
    /// Wrong post-state rejection is atomic; this is not a canonical game undo.
    pub fn undo(&mut self, undo: Undo) -> Result<(), UndoError> {
        self.undo_structural(undo)
    }

    /// Board, piece owners/promotion, both hands and turn, excluding counters.
    pub fn repetition_identity(&self) -> RepetitionIdentity {
        RepetitionIdentity {
            board: self.board,
            hands: self.hands,
            side: self.side,
        }
    }

    pub(super) fn normalized_for_board_queries(&self) -> Self {
        let mut copy = self.clone();
        copy.next_ply = 1;
        copy
    }
}

#[cfg(test)]
mod inspection_tests {
    use super::*;
    fn actions(p: &Position, side: Side, from: &str) -> Vec<String> {
        p.inspection_actions(side)
            .into_iter()
            .map(|a| a.to_string())
            .filter(|a| a.starts_with(from))
            .collect()
    }
    #[test]
    fn opposite_side_movement_promotions_capture_and_pin_preserve_position() {
        let p = Position::from_sfen("4k4/4r4/6s2/9/6P2/9/9/9/K3R4 b - 23").unwrap();
        let before = p.clone();
        assert_eq!(
            actions(&p, Side::Gote, "3c"),
            ["3c2b", "3c2d", "3c3d", "3c4b", "3c4d"]
        );
        let rook = actions(&p, Side::Gote, "5b");
        assert!(!rook.iter().any(|a| a.starts_with("5b4b")));
        assert!(rook.contains(&"5b5i".into()));
        assert!(rook.contains(&"5b5i+".into()));
        assert_eq!(p, before);
    }
    #[test]
    fn hypothetical_drops_retain_nifu_dead_ranks_and_resources() {
        let p = Position::from_sfen("4k4/9/9/9/4p4/9/9/9/4K4 b pn 7").unwrap();
        let before = p.clone();
        let pawn = actions(&p, Side::Gote, "P*");
        assert!(pawn.contains(&"P*4f".into()));
        assert!(
            !pawn
                .iter()
                .any(|a| a.as_bytes()[2] == b'5' || a.ends_with('i'))
        );
        let knight = actions(&p, Side::Gote, "N*");
        assert!(!knight.iter().any(|a| a.ends_with('h') || a.ends_with('i')));
        assert_eq!(p, before);
    }
    #[test]
    fn hypothetical_inspection_still_excludes_pawn_drop_mate() {
        // The gold protects 5b; the gote king's other exits are occupied by
        // its lances/pawns. A dropped pawn on 5b gives undefendable check.
        let p = Position::from_sfen("3lkl3/3p1p3/4G4/9/9/9/9/9/K8 w P 1").unwrap();
        let before = p.clone();
        assert!(!actions(&p, Side::Sente, "P*").contains(&"P*5b".into()));
        assert_eq!(p, before);
    }
}
