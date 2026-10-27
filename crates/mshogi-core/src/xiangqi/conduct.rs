//! WXF conduct responsibility over an explicit, contiguous native history witness.
//!
//! This is not a second move generator or a mating-search implementation. Native
//! legal capture/recapture probes establish pursuit and real protection. The
//! permitted non-chase labels (mate threat, offer, block, exchange and idle) need
//! not be collapsed to "idle" to establish that no perpetual-check/chase fault
//! follows. Ambiguous pursuit must remain unresolved instead of manufacturing a
//! draw. Source/independent-fixture provenance is recorded with MSH-94.
use super::{Action, Counters, Intersection, Piece, PieceKind, Position, Side};
use std::{collections::BTreeSet, error::Error, fmt};

/// Native prohibited-conduct reason, separate from a locally legal board action.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FaultKind {
    /// WXF20.1: every move by the responsible player checks.
    PerpetualCheck,
    /// WXF19.10/20: persistent prohibited pursuit of the same native victim.
    PerpetualChase,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Step {
    pub(super) before: Position,
    pub(super) after: Position,
    pub(super) action: Action,
    pub(super) actor: Side,
    pub(super) ids_before: [Option<u8>; 90],
    pub(super) ids_after: [Option<u8>; 90],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Responsibility {
    None,
    OneSided { side: Side, kind: FaultKind },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum ConductError {
    Invariant(&'static str),
    Unresolved(&'static str),
}
impl fmt::Display for ConductError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invariant(reason) => write!(f, "WXF conduct invariant: {reason}"),
            Self::Unresolved(reason) => write!(f, "WXF conduct unresolved: {reason}"),
        }
    }
}
impl Error for ConductError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Capture {
    action: Action,
    attacker: u8,
    victim: u8,
    piece: Piece,
    target: Piece,
}

fn normalized(position: &Position, actor: Side) -> Position {
    let mut query = position.clone();
    query.side = actor;
    query.counters = Counters::new(Some(0), Some(1)).expect("constant counters");
    query
}

fn captures(
    position: &Position,
    ids: &[Option<u8>; 90],
    actor: Side,
) -> Result<Vec<Capture>, ConductError> {
    let query = normalized(position, actor);
    query
        .legal_actions()
        .map_err(|_| ConductError::Invariant("capture probe is not a legal native turn"))?
        .into_iter()
        .filter_map(|action| {
            let target = query.piece(action.to)?;
            let piece = query.piece(action.from).expect("native action source");
            Some(
                ids[action.from.index()]
                    .zip(ids[action.to.index()])
                    .map(|(attacker, victim)| Capture {
                        action,
                        attacker,
                        victim,
                        piece,
                        target,
                    })
                    .ok_or(ConductError::Invariant("missing native piece identity")),
            )
        })
        .collect()
}

fn matching_pair(captures: &[Capture], candidate: Capture) -> Option<Capture> {
    captures
        .iter()
        .find(|other| other.attacker == candidate.attacker && other.victim == candidate.victim)
        .copied()
}

fn real_root(position: &Position, capture: Capture) -> Result<bool, ConductError> {
    let mut after = normalized(position, capture.piece.side);
    after
        .apply(capture.action)
        .map_err(|_| ConductError::Invariant("admitted capture failed native application"))?;
    Ok(after
        .legal_actions()
        .map_err(|_| ConductError::Invariant("recapture probe is not a legal native turn"))?
        .into_iter()
        .any(|reply| reply.to == capture.action.to))
}

fn reciprocal_capture(position: &Position, capture: Capture) -> Result<bool, ConductError> {
    let query = normalized(position, capture.target.side);
    let reciprocal = Action {
        from: capture.action.to,
        to: capture.action.from,
    };
    Ok(query
        .legal_actions()
        .map_err(|_| ConductError::Invariant("reciprocal probe is not a legal native turn"))?
        .contains(&reciprocal))
}

fn prohibited_capture(position: &Position, capture: Capture) -> Result<bool, ConductError> {
    //20.9 belongs to the pursuing piece, not merely the piece which moved a
    // cannon mount; a Soldier moving a mount does not exempt its Cannon.
    if matches!(capture.piece.kind, PieceKind::General | PieceKind::Soldier)
        || (capture.target.kind == PieceKind::Soldier
            && !capture.target.side.has_crossed_river(capture.action.to))
    {
        return Ok(false);
    }
    //20.7: reciprocal legal capture distinguishes free same-type exchanges
    // from a pinned Rook/Cannon or a Horse whose return leg is blocked.
    if capture.piece.kind == capture.target.kind {
        if reciprocal_capture(position, capture)? {
            return Ok(false);
        }
        // Chapter5 protected pinned-Cannon/Rook examples (diagram54 and
        // section22) retain the real-root allowance even on a constrained target.
        return Ok(!real_root(position, capture)?);
    }
    //20.6 explicitly forbids Horse/Cannon pursuit of even a rooted Rook.
    if capture.target.kind == PieceKind::Rook
        && matches!(capture.piece.kind, PieceKind::Horse | PieceKind::Cannon)
    {
        return Ok(true);
    }
    Ok(!real_root(position, capture)?)
}

fn validate_steps(steps: &[Step], require_closed: bool) -> Result<(), ConductError> {
    if steps.is_empty() {
        return Err(ConductError::Invariant("empty repeating witness"));
    }
    let first = &steps[0];
    let last = steps.last().expect("nonempty witness");
    if require_closed
        && (first.before.board != last.after.board || first.before.side != last.after.side)
    {
        return Err(ConductError::Invariant(
            "repeating witness does not close at the same board and turn",
        ));
    }
    for (index, step) in steps.iter().enumerate() {
        if step.before.side_to_move() != step.actor {
            return Err(ConductError::Invariant(
                "step actor disagrees with native turn",
            ));
        }
        if index > 0
            && (steps[index - 1].after != step.before
                || steps[index - 1].ids_after != step.ids_before)
        {
            return Err(ConductError::Invariant("noncontiguous history witness"));
        }
        let mut replay = step.before.clone();
        replay
            .apply(step.action)
            .map_err(|_| ConductError::Invariant("illegal native witness action"))?;
        if replay != step.after {
            return Err(ConductError::Invariant(
                "native witness post-state disagreement",
            ));
        }
        let mut expected = step.ids_before;
        let moving = expected[step.action.from.index()]
            .ok_or(ConductError::Invariant("missing moving piece identity"))?;
        expected[step.action.from.index()] = None;
        expected[step.action.to.index()] = Some(moving);
        if expected != step.ids_after {
            return Err(ConductError::Invariant(
                "native witness identity transition disagreement",
            ));
        }
        let mut identities = BTreeSet::new();
        for at in Intersection::all() {
            if step.before.piece(at).is_some() != step.ids_before[at.index()].is_some() {
                return Err(ConductError::Invariant(
                    "piece/identity occupancy disagreement",
                ));
            }
            if let Some(id) = step.ids_before[at.index()]
                && !identities.insert(id)
            {
                return Err(ConductError::Invariant("duplicate native piece identity"));
            }
        }
    }
    Ok(())
}

fn chasing(steps: &[Step], side: Side, close_cycle: bool) -> Result<bool, ConductError> {
    let mut common: Option<BTreeSet<u8>> = None;
    let mut exempt_pursuers = BTreeSet::new();
    for (index, step) in steps
        .iter()
        .enumerate()
        .filter(|(_, step)| step.actor == side)
    {
        // A check/noncheck mixture is allowed by20.2; a simultaneous mating
        // threat does not neutralize an established chase (diagram109).
        if step.after.is_in_check(side.opponent()) {
            return Ok(false);
        }
        let before = captures(&step.before, &step.ids_before, side)?;
        let after = captures(&step.after, &step.ids_after, side)?;
        // At a closed board return, project the already observed first reply
        // through the current identity map. Identical pieces can have permuted
        // identities, so copying the old map would invent a victim. This is a
        // local cyclic diagnostic, never an accepted future game event.
        let (response_after, response_ids, unanswered) = if let Some(next) = steps.get(index + 1) {
            if next.actor != side.opponent() {
                return Err(ConductError::Invariant("missing alternating response"));
            }
            (next.after.clone(), next.ids_after, false)
        } else if !close_cycle {
            // No reply has occurred. Existing causal/prohibited threats supply
            // an upper bound: an empty persistent-victim intersection proves a
            // break already; a nonempty one has not yet been disproved. Do not
            // assert that an opponent actually resolved this pending pursuit.
            (step.after.clone(), step.ids_after, true)
        } else {
            let first = &steps[0];
            if first.actor == side {
                return Err(ConductError::Unresolved(
                    "no opposing response in repeating witness",
                ));
            }
            if first.before.board != step.after.board || first.before.side != step.after.side {
                return Err(ConductError::Unresolved(
                    "witness does not close at the same native board/turn",
                ));
            }
            let mut projected = normalized(&step.after, first.actor);
            projected
                .apply(first.action)
                .map_err(|_| ConductError::Invariant("closed-board reply projection is illegal"))?;
            let mut ids = step.ids_after;
            ids[first.action.to.index()] = ids[first.action.from.index()];
            ids[first.action.from.index()] = None;
            (projected, ids, false)
        };
        let replied = if unanswered {
            Vec::new()
        } else {
            captures(&response_after, &response_ids, side)?
        };
        let moving_id = step.ids_before[step.action.from.index()];
        let mut victims = BTreeSet::new();
        for capture in after {
            // Diagram110 excludes a stationary, continuously present attack.
            // Moving the pursuer, or changing a screen/leg/root to enable a
            // different native legal capture, supplies the causal link.
            let prohibited = prohibited_capture(&step.after, capture)?;
            let causal =
                moving_id == Some(capture.attacker) || matching_pair(&before, capture).is_none();
            // Moving a real protector into place can directly resolve pursuit
            // even though a geometrically/legal capture remains available.
            let resolved = unanswered
                || match matching_pair(&replied, capture) {
                    None => true,
                    Some(reply) => prohibited && !prohibited_capture(&response_after, reply)?,
                };
            if !causal || !resolved {
                continue;
            }
            // A pending reply can upper-bound a positive chase candidate, but
            // cannot establish a negative General/Soldier-combination exception:
            // the opponent may leave that incidental threat unresolved.
            if !unanswered && matches!(capture.piece.kind, PieceKind::General | PieceKind::Soldier)
            {
                exempt_pursuers.insert(capture.victim);
            }
            if prohibited {
                victims.insert(capture.victim);
            }
        }
        common = Some(match common {
            None => victims,
            Some(previous) => previous.intersection(&victims).copied().collect(),
        });
    }
    let Some(mut common) = common else {
        return Err(ConductError::Invariant("witness omits one native side"));
    };
    for victim in exempt_pursuers {
        common.remove(&victim);
    }
    //20.4 separates one/two pursuers of two or more victims from the same-victim
    // prohibition. Neither alternating victims (empty intersection), nor two
    // persistent victims, supplies a one-sided same-victim fault.
    Ok(common.len() == 1)
}

pub(super) fn responsibility(steps: &[Step]) -> Result<Responsibility, ConductError> {
    validate_steps(steps, true)?;
    let all_check = |side| {
        let mut seen = false;
        let all = steps.iter().filter(|step| step.actor == side).all(|step| {
            seen = true;
            step.after.is_in_check(side.opponent())
        });
        seen && all
    };
    match (all_check(Side::Red), all_check(Side::Black)) {
        (true, false) => {
            return Ok(Responsibility::OneSided {
                side: Side::Red,
                kind: FaultKind::PerpetualCheck,
            });
        }
        (false, true) => {
            return Ok(Responsibility::OneSided {
                side: Side::Black,
                kind: FaultKind::PerpetualCheck,
            });
        }
        (true, true) => return Ok(Responsibility::None), //20.1 equal perpetual check.
        (false, false) => {}
    }
    match (
        chasing(steps, Side::Red, true)?,
        chasing(steps, Side::Black, true)?,
    ) {
        (true, false) => Ok(Responsibility::OneSided {
            side: Side::Red,
            kind: FaultKind::PerpetualChase,
        }),
        (false, true) => Ok(Responsibility::OneSided {
            side: Side::Black,
            kind: FaultKind::PerpetualChase,
        }),
        _ => Ok(Responsibility::None), //Equal prohibited chase or neither.
    }
}

/// Reassess an already established chase obligation through actual new replies.
/// This is not complete-cycle classification. An unanswered latest pursuit uses
/// its possible causal threats as an upper bound; true means continuity is not
/// yet disproved. A check, absence/change of the persistent victim, or an actual
/// reply that ignores pursuit proves that continuity has ceased.
pub(super) fn continues_chase_prefix(steps: &[Step], side: Side) -> Result<bool, ConductError> {
    validate_steps(steps, false)?;
    chasing(steps, side, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn witness(fen: &str, actions: &str) -> Vec<Step> {
        let mut position = Position::from_fen(fen).unwrap();
        let mut ids = [None; 90];
        for (id, (at, _)) in position.pieces().enumerate() {
            ids[at.index()] = Some(id as u8);
        }
        actions
            .split_whitespace()
            .map(|token| {
                let action: Action = token.parse().unwrap();
                let before = position.clone();
                let ids_before = ids;
                position.apply(action).unwrap();
                ids[action.to.index()] = ids[action.from.index()];
                ids[action.from.index()] = None;
                let actor = before.side_to_move();
                Step {
                    before,
                    after: position.clone(),
                    action,
                    actor,
                    ids_before,
                    ids_after: ids,
                }
            })
            .collect()
    }

    #[test]
    fn primary_rule_independent_cycles() {
        let cases = [
            (
                "unrooted rook/horse",
                "5k3/9/9/9/2n6/R8/9/9/9/3K5 w - - 0 1",
                "a4c4 c5a6 c4a4 a6c5",
                Some(Side::Red),
            ),
            (
                "real-rooted horse",
                "5k3/9/9/7r1/2n4r1/R8/9/9/9/3K5 w - - 0 1",
                "a4c4 c5a6 c4a4 a6c5",
                None,
            ),
            (
                "fake root",
                "R1r2k3/9/9/9/2n6/4R4/9/9/9/3K5 w - - 0 1",
                "e4c4 c5e6 c4e4 e6c5",
                Some(Side::Red),
            ),
            (
                "free same-type",
                "5k3/9/9/9/2r6/R8/9/9/9/3K5 w - - 0 1",
                "a4c4 c5a5 c4a4 a5c5",
                None,
            ),
            (
                "pinned same-type",
                "R2r1k3/9/9/9/9/2R6/9/9/9/4K4 w - - 0 1",
                "c4d4 d9c9 d4c4 c9d9",
                Some(Side::Red),
            ),
            (
                "horse rooted-rook exception",
                "1r3k3/9/1r7/9/9/N8/9/9/9/3K5 w - - 0 1",
                "a4c5 b7b6 c5a4 b6b7",
                Some(Side::Red),
            ),
            (
                "rooted pinned same-type",
                "R2r1k3/4a4/9/9/9/2R6/9/9/9/4K4 w - - 0 1",
                "c4d4 d9c9 d4c4 c9d9",
                None,
            ),
            (
                "leg-blocked same-type Horse",
                "5k3/9/9/9/2Cn5/N8/1C7/9/9/3K5 w - - 0 1",
                "a4b6 d5c3 b6a4 c3d5",
                Some(Side::Red),
            ),
            (
                "General pursuer exemption",
                "5k3/9/9/9/9/9/9/4c4/3K5/9 w - - 0 1",
                "d1e1 e2d2 e1d1 d2e2",
                None,
            ),
            (
                "Soldier pursuer exemption",
                "5k3/9/9/1c7/P8/9/9/9/9/3K5 w - - 0 1",
                "a5b5 b6a6 b5a5 a6b6",
                None,
            ),
            (
                "General and Rook pursuer combination",
                "5k3/9/9/9/9/9/9/3K5/4p4/4R4 w - - 0 1",
                "d2e2 e1d1 e0d0 d1e1 d0e0 e1d1 e2d2 d1e1",
                None,
            ),
            (
                "alternating multiple victims",
                "5k3/9/9/nn7/9/9/r8/RR7/9/C2K5 w - - 0 1",
                "a0b0 a3b3 b0a0 b3a3",
                None,
            ),
            (
                "diagram37 check/chase",
                "2bak4/4a4/4b4/p8/2p3n2/9/P1P3R2/3AB4/c3N3N/1RB1KA3 w - - 0 1",
                "b0b1 a1a0 b1b0 a0a1",
                Some(Side::Red),
            ),
            (
                "diagram38 mate/chase",
                "2bakab2/9/2c6/7R1/p3p1p2/9/P1P1P1P1P/B1N6/1R2A4/2B1KA3 w - - 0 1",
                "h6c6 c7h7 c6h6 h7c7",
                Some(Side::Red),
            ),
            (
                "diagram108 equal prohibited chase",
                "3akab2/c2R1R2c/4b4/9/9/9/9/4B4/4K4/2B6 w - - 0 1",
                "f8f6 d9e8 f6f8 e8d9",
                None,
            ),
            (
                "diagram109 simultaneous mate/chase",
                "1r2kab2/4aR3/1cn5c/p5N1p/6b2/9/P7P/6C2/9/2BAKAB2 w - - 0 1",
                "g6h8 g5e7 h8g6 e7g5",
                Some(Side::Red),
            ),
            (
                "diagram110 persistent cannon excluded",
                "4kab2/4arNC1/4b1R2/4n4/2p3p2/2c6/4P3P/9/4A4/2BAK1B2 w - - 0 1",
                "g7g6 e6f4 g6g7 f4e6",
                Some(Side::Black),
            ),
            (
                "uncrossed stationary soldier",
                "5k3/9/9/p8/9/9/9/9/1Rn6/C2K5 w - - 0 1",
                "b1a1 c1a2 a1b1 a2c1",
                None,
            ),
            (
                "crossed stationary soldier",
                "5k3/9/9/9/9/p8/9/9/1Rn6/C2K5 w - - 0 1",
                "b1a1 c1a2 a1b1 a2c1",
                Some(Side::Red),
            ),
        ];
        for (name, fen, actions, expected) in cases {
            let result = responsibility(&witness(fen, actions)).unwrap();
            assert_eq!(
                result,
                expected.map_or(Responsibility::None, |side| Responsibility::OneSided {
                    side,
                    kind: FaultKind::PerpetualChase
                }),
                "{name}"
            );
        }
    }

    #[test]
    fn check_priority_and_contiguous_history_are_native() {
        let fen = "4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1";
        let steps = witness(fen, "e9f9 f8e8 f9e9 e8f8");
        assert_eq!(
            responsibility(&steps).unwrap(),
            Responsibility::OneSided {
                side: Side::Red,
                kind: FaultKind::PerpetualCheck
            }
        );
        let mut forged = steps.clone();
        forged[0].actor = Side::Black;
        assert!(matches!(
            responsibility(&forged),
            Err(ConductError::Invariant(_))
        ));
        let mut forged = steps.clone();
        forged[1].ids_before = [None; 90];
        assert!(matches!(
            responsibility(&forged),
            Err(ConductError::Invariant(_))
        ));
        let mut forged = steps.clone();
        forged[0].after = steps[0].before.clone();
        assert!(matches!(
            responsibility(&forged),
            Err(ConductError::Invariant(_))
        ));
        assert!(matches!(
            responsibility(&steps[..1]),
            Err(ConductError::Invariant(_))
        ));
    }

    #[test]
    fn check_and_idle_mixture_is_not_perpetual_chase_or_check() {
        let fen = "4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1";
        let steps = witness(fen, "e9f9 f8e8 f9e9 e8f8 e9a9 f8f7 a9e9 f7f8");
        assert_eq!(responsibility(&steps).unwrap(), Responsibility::None);
    }

    #[test]
    fn real_roots_require_actual_native_recapture() {
        let fake = witness(
            "R1r2k3/9/9/9/2n6/4R4/9/9/9/3K5 w - - 0 1",
            "e4c4 c5e6 c4e4 e6c5",
        );
        let capture = captures(&fake[0].after, &fake[0].ids_after, Side::Red)
            .unwrap()
            .into_iter()
            .find(|capture| capture.action.to == "c5".parse().unwrap())
            .unwrap();
        assert!(!real_root(&fake[0].after, capture).unwrap());

        let real = witness(
            "1r3k3/9/1r7/9/9/N8/9/9/9/3K5 w - - 0 1",
            "a4c5 b7b6 c5a4 b6b7",
        );
        for (index, victim) in [(0, "b7"), (2, "b6")] {
            let capture = captures(&real[index].after, &real[index].ids_after, Side::Red)
                .unwrap()
                .into_iter()
                .find(|capture| capture.action.to == victim.parse().unwrap())
                .unwrap();
            assert!(real_root(&real[index].after, capture).unwrap());
            assert!(prohibited_capture(&real[index].after, capture).unwrap());
        }
    }

    #[test]
    fn two_to_one_is_counter_chase_frequency_not_two_pursuing_pieces() {
        let steps = witness(
            "5k3/9/9/9/2n6/R8/9/9/9/3K5 w - - 0 1",
            "a4c4 c5a6 c4a4 a6c5",
        );
        let rook = steps[0].ids_before["a4".parse::<Intersection>().unwrap().index()].unwrap();
        let horse = steps[0].ids_before["c5".parse::<Intersection>().unwrap().index()].unwrap();
        for index in [0, 2] {
            assert!(
                captures(&steps[index].after, &steps[index].ids_after, Side::Red)
                    .unwrap()
                    .iter()
                    .any(|capture| capture.attacker == rook && capture.victim == horse)
            );
        }
        // a6->c4 is not a Horse move; c5->a4 is a legal Horse capture
        // through empty b5. Exactly one of Black's two replies counter-chases.
        assert!(
            !captures(&steps[1].after, &steps[1].ids_after, Side::Black)
                .unwrap()
                .iter()
                .any(|capture| capture.attacker == horse && capture.victim == rook)
        );
        assert!(
            captures(&steps[3].after, &steps[3].ids_after, Side::Black)
                .unwrap()
                .iter()
                .any(|capture| capture.attacker == horse && capture.victim == rook)
        );
        assert_eq!(
            responsibility(&steps).unwrap(),
            Responsibility::OneSided {
                side: Side::Red,
                kind: FaultKind::PerpetualChase,
            }
        );
    }

    #[test]
    fn board_repetition_may_permute_identical_piece_identities() {
        let steps = witness(
            "5k3/9/9/R8/9/R8/9/9/9/3K5 w - - 0 1",
            "a4b4 f9f8 a6a4 f8f9 b4b6 f9f8 b6a6 f8f9",
        );
        let first = &steps[0];
        let last = steps.last().unwrap();
        assert_eq!(first.before.board, last.after.board);
        assert_eq!(first.before.side, last.after.side);
        assert_ne!(first.ids_before, last.ids_after);
        let a4 = "a4".parse::<Intersection>().unwrap().index();
        let a6 = "a6".parse::<Intersection>().unwrap().index();
        assert_eq!(first.ids_before[a4], last.ids_after[a6]);
        assert_eq!(first.ids_before[a6], last.ids_after[a4]);
        let original = steps.clone();
        assert_eq!(responsibility(&steps).unwrap(), Responsibility::None);
        assert_eq!(steps, original); // Diagnostic projection never accepts a future event.
    }

    #[test]
    fn established_chase_retires_at_actual_break_not_only_at_anchor_return() {
        let fen = "4R4/5k3/c8/9/9/9/9/9/9/3K5 w - - 0 1";
        let tt = "e9a9 a7e7 a9e9 e7a7 e9a9 a7e7 a9e9 e7a7";
        let warning = witness(fen, tt);
        assert_eq!(
            responsibility(&warning).unwrap(),
            Responsibility::OneSided {
                side: Side::Red,
                kind: FaultKind::PerpetualChase,
            }
        );
        assert!(continues_chase_prefix(&warning, Side::Red).unwrap());
        // T+T+C: the new Red move checks instead of continuing the chase.
        let checking_break = witness(fen, &format!("{tt} e9f9"));
        assert!(!continues_chase_prefix(&checking_break, Side::Red).unwrap());
        let idle_break = witness(fen, &format!("{tt} e9d9"));
        assert!(!continues_chase_prefix(&idle_break, Side::Red).unwrap());
        // T+T+Q: Red still pursues Cannon a7 at ply9, but Black's King reply
        // f8f7 leaves that exact capture available. This is not an escape or
        // direct chase resolution under19.10, so continuity ceases at ply10.
        let unanswered = witness(fen, &format!("{tt} e9a9"));
        assert!(continues_chase_prefix(&unanswered, Side::Red).unwrap());
        let ignored = witness(fen, &format!("{tt} e9a9 f8f7"));
        assert!(!continues_chase_prefix(&ignored, Side::Red).unwrap());
    }

    #[test]
    fn pending_general_attack_cannot_invent_a_combination_exemption() {
        // Independently reviewed19.10/20.9 boundary: the Red General's direct
        // attack on d0 remains available after Black's reply, so the General is
        // not part of the resolved pursuit. Only Red Cannon d4 persistently
        // chases the Horse through alternating General/Cannon screens.
        let fen = "5k3/9/9/9/9/3C5/9/4c4/4K4/3n5 w - - 0 1";
        let cycle = "e1d1 e2d2 d1e1 d2e2";
        let twice = format!("{cycle} {cycle}");
        let warning = witness(fen, &twice);
        let fault = Responsibility::OneSided {
            side: Side::Red,
            kind: FaultKind::PerpetualChase,
        };
        assert_eq!(responsibility(&warning).unwrap(), fault);
        let unanswered = witness(fen, &format!("{twice} e1d1"));
        assert!(continues_chase_prefix(&unanswered, Side::Red).unwrap());
        let answered = witness(fen, &format!("{twice} e1d1 e2d2"));
        assert!(continues_chase_prefix(&answered, Side::Red).unwrap());
        let three = witness(fen, &format!("{twice} {cycle}"));
        assert_eq!(responsibility(&three).unwrap(), fault);
    }
}
