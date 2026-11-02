//! Production-native fixture projections. Expected answers are only read by the
//! shared comparator; the adapter uses fixture inputs and admitted native APIs.
use super::*;
use mshogi_conformance::{
    assertions::Assertion,
    chu::{Fixture, NativeAdapter},
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn sq(v: &Value) -> Square {
    serde_json::from_value(v.clone()).expect("audited fixture square")
}
pub(super) fn side(v: &Value) -> Side {
    serde_json::from_value(v.clone()).expect("audited native owner")
}
pub(super) fn root(v: &Value) -> Result<Position, RootError> {
    if v == "standard_start" {
        return Ok(Position::standard());
    }
    let entries: Vec<_> = v["board"]
        .as_array()
        .expect("audited board")
        .iter()
        .map(|p| {
            (
                sq(&p["at"]),
                Piece::from_code(side(&p["side"]), p["piece"].as_str().unwrap())
                    .expect("audited original identity"),
            )
        })
        .collect();
    Position::from_diagram(
        &entries,
        side(&v["turn"]),
        RootMetadata::diagram(v["declared_board_plies"].as_u64().unwrap()),
    )
}
pub(super) fn action(v: &Value) -> Action {
    Action {
        from: sq(&v["from"]),
        route: v["route"].as_array().unwrap().iter().map(sq).collect(),
        promote: v["promote"].as_bool().unwrap(),
    }
}
pub(super) fn action_value(a: &Action) -> Value {
    json!({"kind":"move","from":a.from,"route":a.route,"promote":a.promote})
}
pub(super) fn board(p: &Position) -> Value {
    json!(
        p.pieces()
            .map(|(at, p)| json!({"at":at,"side":p.side,"piece":p.code()}))
            .collect::<Vec<_>>()
    )
}
pub(super) fn snapshot(p: &LocalPosition) -> Value {
    json!({"position":p.position().state(),"protection":p.protection()})
}

type ProjectionCache =
    std::cell::RefCell<BTreeMap<String, Result<BTreeMap<String, Value>, String>>>;

#[derive(Default)]
pub(super) struct LocalAdapter {
    cache: ProjectionCache,
}
impl NativeAdapter for LocalAdapter {
    fn observe(&self, f: &Fixture, a: &Assertion) -> Result<Option<Value>, String> {
        if !self.cache.borrow().contains_key(&f.id) {
            self.cache.borrow_mut().insert(f.id.clone(), project(f));
        }
        self.cache
            .borrow()
            .get(&f.id)
            .unwrap()
            .as_ref()
            .map(|m| m.get(&a.field).cloned())
            .map_err(Clone::clone)
    }
}
fn put(m: &mut BTreeMap<String, Value>, key: &str, value: impl serde::Serialize) {
    m.insert(
        key.into(),
        serde_json::to_value(value).expect("native observation serialization"),
    );
}
fn commands(f: &Fixture) -> Vec<Action> {
    if let Some(a) = f.given.get("action") {
        vec![action(a)]
    } else {
        f.given["commands"]
            .as_array()
            .map(|cs| {
                cs.iter()
                    .filter(|a| a["kind"] == "move")
                    .map(action)
                    .collect()
            })
            .unwrap_or_default()
    }
}
fn perft(p: &LocalPosition, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }
    p.legal_actions()
        .iter()
        .map(|a| {
            let mut next = p.clone();
            next.apply(a).expect("complete local choice admitted");
            perft(&next, depth - 1)
        })
        .sum()
}
fn project(f: &Fixture) -> Result<BTreeMap<String, Value>, String> {
    let mut out = BTreeMap::new();
    if !f.given["root"].is_object() && f.given["root"] != "standard_start" {
        return Ok(out);
    }
    // Game/history and record contracts are projected separately by GameAdapter.
    if f.id != "CHU-PERFT-REMOTE"
        && f.clauses
            .iter()
            .any(|c| matches!(c.as_str(), "C06" | "C07" | "C08" | "C09" | "C10" | "C11"))
    {
        return Ok(out);
    }
    let p = root(&f.given["root"]).map_err(|e| e.to_string())?;
    if f.id == "CHU-SETUP" {
        put(&mut out, "board", board(&p));
        put(&mut out, "side_to_move", p.side_to_move());
        put(&mut out, "piece_count", p.pieces().count());
        let counts =
            [Side::Sente, Side::Gote].map(|s| p.pieces().filter(|(_, v)| v.side == s).count());
        if counts[0] != counts[1] {
            return Err("native setup has unequal side inventories".into());
        }
        put(&mut out, "pieces_per_side", counts[0]);
        // Chu has no hand state, no hand choice and no drop action type.
        put(&mut out, "initial_hands", Value::Null);
        return Ok(out);
    }
    let local = LocalPosition::admit(p.clone());
    if f.id == "CHU-PERFT-REMOTE" {
        put(&mut out, "perft_depth1", perft(&local, 1));
        put(&mut out, "perft_depth2", perft(&local, 2));
        let divide: Vec<_> = local
            .legal_actions()
            .iter()
            .map(|a| {
                let mut next = local.clone();
                next.apply(a).expect("native divide choice");
                json!({"action":action_value(a),"nodes":perft(&next,1)})
            })
            .collect();
        put(&mut out, "divide_depth2", divide);
        return Ok(out);
    }
    if f.id.starts_with("CHU-MOVE-") {
        let at = sq(&f.given["actor"]);
        let piece = p.piece(at).ok_or("fixture actor missing")?;
        let descriptor = movement(piece.face());
        put(&mut out, "movement_face", piece.face().code());
        put(&mut out, "step_offsets", descriptor.steps);
        put(&mut out, "jump_offsets", descriptor.jumps);
        put(&mut out, "ray_directions", descriptor.rays);
        let empty = Position::from_diagram(&[(at, piece)], piece.side, RootMetadata::diagram(0))
            .map_err(|e| e.to_string())?;
        let endpoints: BTreeSet<_> = LocalPosition::admit(empty)
            .choices_from(at)
            .into_iter()
            .filter_map(|a| a.route.last().copied())
            .filter(|s| *s != at)
            .collect();
        put(
            &mut out,
            "empty_board_nonorigin_landing_count",
            endpoints.len(),
        );
        let mut valid = local.clone();
        put(
            &mut out,
            "legal_probe_admitted",
            valid.apply(&action(&f.given["legal_probe"])).is_ok(),
        );
        let mut invalid = local.clone();
        let before = snapshot(&invalid);
        let admitted = invalid.apply(&action(&f.given["illegal_probe"])).is_ok();
        put(&mut out, "illegal_probe_admitted", admitted);
        put(
            &mut out,
            "rejection_atomic",
            !admitted && snapshot(&invalid) == before,
        );
        put(
            &mut out,
            "special",
            match descriptor.special {
                Some(SpecialMovement::Lion) => "full_lion",
                Some(SpecialMovement::ForwardDiagonal) => "forward_diagonal_lion",
                Some(SpecialMovement::Forward) => "forward_lion",
                None => "none",
            },
        );
        return Ok(out);
    }
    if f.id.starts_with("CHU-COMPLETE-SET-") {
        let choices = local.choices_from(sq(&f.given["actor"]));
        let encoded: Vec<_> = choices.iter().map(action_value).collect();
        let unique: BTreeSet<_> = encoded.iter().map(Value::to_string).collect();
        put(&mut out, "actor_complete_actions", &encoded);
        put(&mut out, "actor_action_count", choices.len());
        // Comparison of the full observed array independently checks missing/extra.
        put(
            &mut out,
            "no_missing_extra_or_duplicate_actions",
            unique.len() == encoded.len(),
        );
        return Ok(out);
    }
    let actions = commands(f);
    let mut current = local.clone();
    let mut effects = Vec::new();
    let mut accepted = Vec::new();
    let mut atomic = Vec::new();
    let mut errors = Vec::new();
    for a in &actions {
        let before = snapshot(&current);
        match current.apply(a) {
            Ok((e, _)) => {
                effects.push(Some(e));
                accepted.push(true);
                atomic.push(true);
                errors.push(None);
            }
            Err(e) => {
                effects.push(None);
                accepted.push(false);
                atomic.push(before == snapshot(&current));
                errors.push(Some(e));
            }
        }
    }
    if let Some(first) = accepted.first() {
        put(&mut out, "admitted", first);
        put(&mut out, "first_admitted", first);
        put(&mut out, "rejection_atomic", atomic[0]);
        put(&mut out, "rejection_atomic_if_refused", atomic[0]);
    }
    put(
        &mut out,
        "all_admitted",
        !accepted.is_empty() && accepted.iter().all(|a| *a),
    );
    if accepted.len() > 1 {
        put(&mut out, "second_admitted", accepted[1]);
        put(&mut out, "second_rejection_atomic", atomic[1]);
    }
    if let Some(last) = actions.last() {
        let final_at = if accepted.last() == Some(&true) {
            *last.route.last().unwrap()
        } else {
            last.from
        };
        if let Some(piece) = current.position().piece(final_at) {
            put(&mut out, "result_piece", piece.code());
            put(&mut out, "movement_face", piece.face().code());
        }
        if f.id.starts_with("CHU-RESCUE-") {
            let before = p.piece(actions[0].from).unwrap();
            put(
                &mut out,
                "dead_if_unpromoted",
                before.dead(*actions[0].route.last().unwrap()),
            );
        }
    }
    if let Some(a) = actions.first() {
        let choices = local.choices_from(a.from);
        put(
            &mut out,
            "choice_optional",
            choices
                .iter()
                .any(|b| b.from == a.from && b.route == a.route && !b.promote)
                && choices
                    .iter()
                    .any(|b| b.from == a.from && b.route == a.route && b.promote),
        );
        let first = effects.first().and_then(Option::as_ref);
        put(
            &mut out,
            "capture_count",
            first.map_or(0, |e| e.captures.len()),
        );
        put(
            &mut out,
            "first_ordered_captures",
            first
                .map(|e| e.captures.iter().map(|c| c.square).collect::<Vec<_>>())
                .unwrap_or_default(),
        );
        put(
            &mut out,
            "ordered_captures",
            first
                .map(|e| e.captures.iter().map(|c| c.square).collect::<Vec<_>>())
                .unwrap_or_default(),
        );
        put(
            &mut out,
            "captures",
            first
                .map(|e| {
                    e.captures
                        .iter()
                        .map(|c| json!({"side":c.piece.side,"piece":c.piece.code(),"at":c.square}))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
        );
        put(
            &mut out,
            "quiet_pass",
            first.is_some_and(|e| e.quiet_return),
        );
        put(
            &mut out,
            "final_square",
            if accepted[0] {
                *a.route.last().unwrap()
            } else {
                a.from
            },
        );
        // A jump's midpoint is derived from submitted geometry, not expectations.
        if a.route.len() == 1 && a.from.distance(a.route[0]) == 2 {
            let middle = Square::new(
                (a.from.file() + a.route[0].file()) / 2,
                (a.from.rank() + a.route[0].rank()) / 2,
            )
            .unwrap();
            put(
                &mut out,
                "intermediate_piece_preserved",
                p.piece(middle).is_some() && p.piece(middle) == current.position().piece(middle),
            );
        }
        put(
            &mut out,
            "no_captures_committed",
            effects.iter().flatten().all(|e| e.captures.is_empty()),
        );
        if let Some(Some(ActionError::Promotion)) = errors.first()
            && p.piece(a.from).is_some_and(|p| p.promoted)
        {
            put(&mut out, "reason", "already_promoted");
        }
        if let Some(Some(ActionError::LionLaw)) = errors.first() {
            put(&mut out, "native_refusal_kind", "lion_law");
        }
        if let Some(first) = first {
            put(
                &mut out,
                "protected_lions_after_first",
                first.protection_after.protected_squares(),
            );
            put(
                &mut out,
                "transient_protection_after_first",
                first.protection_after.protected_squares(),
            );
            put(
                &mut out,
                "first_capture_creates_transient_protection",
                !first.protection_after.protected_squares().is_empty(),
            );
            put(
                &mut out,
                "extra_lion_is_qualifying_capture",
                first.captures.len() > 1 && first.captures.iter().any(|c| c.piece.is_lion()),
            );
            if first.mover_after.is_lion() {
                let new_lion = *a.route.last().unwrap();
                put(&mut out, "new_lion_at", new_lion);
                let mut after = local.clone();
                after.apply(a).map_err(|e| e.to_string())?;
                let defenders = after.geometric_defenders(first.actor, new_lion);
                if defenders.len() == 1 {
                    put(&mut out, "defender_at", defenders[0]);
                }
            }
        }
    }
    if let Some(Some(second)) = effects.get(1) {
        put(
            &mut out,
            "second_captures_lion",
            second.captures.iter().any(|c| c.piece.is_lion()),
        );
        put(
            &mut out,
            "second_extra_capture_overrides_protection",
            !second.protection_before.squares.is_empty()
                && second.captures.len() > 1
                && second
                    .captures
                    .iter()
                    .any(|c| !matches!(c.piece.face(), Face::Pawn | Face::GoBetween)),
        );
    }
    put(
        &mut out,
        "lion_count_final",
        current
            .position()
            .pieces()
            .filter(|(_, p)| p.is_lion())
            .count(),
    );
    if f.id == "CHU-LION-PROTECTION-EXPIRES" {
        let last = effects.last().and_then(Option::as_ref);
        put(
            &mut out,
            "immediate_protection_expired",
            last.is_some_and(|e| e.protection_before.squares.is_empty()),
        );
        put(
            &mut out,
            "final_capture_of_defended_lion_allowed",
            last.is_some_and(|e| e.captures.iter().any(|c| c.piece.is_lion())),
        );
    }
    if f.id.starts_with("CHU-RAY-BLOCK-") {
        let blockers: Vec<_> = p
            .pieces()
            .filter(|(at, piece)| {
                piece.base == Base::Pawn && piece.side == p.side_to_move() && *at != actions[0].from
            })
            .map(|(at, _)| at)
            .collect();
        let preserved = blockers
            .into_iter()
            .find(|at| p.piece(*at) == current.position().piece(*at))
            .ok_or("native blocker disappeared")?;
        put(&mut out, "blocker_preserved", preserved);
    }
    if f.id == "CHU-RAY-FIRST-CAPTURE" {
        let choices: Vec<_> = local
            .choices_from(actions[0].from)
            .into_iter()
            .filter(|a| {
                !a.promote
                    && a.route.len() == 1
                    && a.route[0].file() == actions[0].route[0].file()
                    && p.piece(a.route[0])
                        .is_some_and(|v| v.side != p.side_to_move())
            })
            .collect();
        if choices.len() != 1 {
            return Err(format!(
                "expected one native first-blocker capture, observed {}",
                choices.len()
            ));
        }
        put(&mut out, "alternative_capture", action_value(&choices[0]));
    }
    if f.id == "CHU-ROUTE-DISTINCT" {
        let a = &actions[0];
        let endpoint = *a.route.last().unwrap();
        let alternatives: Vec<_> = local
            .choices_from(a.from)
            .into_iter()
            .filter(|b| b.route.len() == 2 && b.route.last() == Some(&endpoint))
            .collect();
        let capture_route = alternatives
            .into_iter()
            .find(|b| p.piece(b.route[0]).is_some())
            .ok_or("native missing two-step capture route")?;
        let mut other = local.clone();
        let (e, _) = other.apply(&capture_route).map_err(|e| e.to_string())?;
        put(
            &mut out,
            "alternative_complete_action",
            action_value(&capture_route),
        );
        put(&mut out, "both_admitted", accepted[0]);
        let mid = capture_route.route[0];
        put(
            &mut out,
            "jump_preserves_intermediate",
            p.piece(mid) == current.position().piece(mid),
        );
        put(
            &mut out,
            "two_step_captures_intermediate",
            e.captures.iter().any(|c| c.square == mid),
        );
        put(
            &mut out,
            "same_endpoint_not_same_action",
            *a != capture_route && a.route.last() == capture_route.route.last(),
        );
    }
    Ok(out)
}

#[test]
fn local_independently_expected_corpus() {
    let corpus = mshogi_conformance::chu::Corpus::embedded().unwrap();
    let report = mshogi_conformance::chu::run_adapter(&corpus, &LocalAdapter::default());
    let failures: Vec<_> = report
        .assertions
        .iter()
        .filter(|a| a.status == mshogi_conformance::assertions::Status::Failed)
        .collect();
    assert!(
        failures.is_empty(),
        "{}",
        serde_json::to_string_pretty(&failures).unwrap()
    );
    eprintln!(
        "Chu local projections: {} passed fields, {} passed rows; remaining native history/record fields {}",
        report.passed, report.native_rows_passed, report.pending
    );
}

#[test]
fn every_geometry_choice_rotates_and_reverses_exactly() {
    let corpus = mshogi_conformance::chu::Corpus::embedded().unwrap();
    let mut checked = 0;
    for fixture in corpus
        .fixtures()
        .iter()
        .filter(|f| f.id.starts_with("CHU-MOVE-") && !f.id.ends_with("-ROT"))
    {
        let original = root(&fixture.given["root"]).unwrap();
        let entries: Vec<_> = original
            .pieces()
            .map(|(at, mut piece)| {
                piece.side = piece.side.opponent();
                (at.rotated(), piece)
            })
            .collect();
        let rotated = Position::from_diagram(
            &entries,
            original.side_to_move().opponent(),
            RootMetadata::diagram(original.root().declared_completed_plies()),
        )
        .unwrap();
        let actor = sq(&fixture.given["actor"]);
        let native = LocalPosition::admit(original);
        let rotated_native = LocalPosition::admit(rotated);
        let choices = native.choices_from(actor);
        let expected_rotation: BTreeSet<_> = choices
            .iter()
            .map(|a| Action {
                from: a.from.rotated(),
                route: a.route.iter().map(|s| s.rotated()).collect(),
                promote: a.promote,
            })
            .collect();
        let actual_rotation: BTreeSet<_> = rotated_native
            .choices_from(actor.rotated())
            .into_iter()
            .collect();
        assert_eq!(expected_rotation, actual_rotation, "{}", fixture.id);
        for action in choices {
            let mut subject = native.clone();
            let before = snapshot(&subject);
            let (effects, mut undo) = subject.apply(&action).unwrap();
            let after = snapshot(&subject);
            let mut redo = subject.undo(&mut undo).unwrap();
            assert_eq!(snapshot(&subject), before, "undo {} {action:?}", fixture.id);
            assert!(subject.undo(&mut undo).is_err(), "single-use token");
            let (replayed, _) = subject.redo(&mut redo).unwrap();
            assert_eq!(snapshot(&subject), after, "redo {} {action:?}", fixture.id);
            assert_eq!(effects, replayed);
            assert!(subject.redo(&mut redo).is_err(), "single-use redo token");
            checked += 1;
        }
    }
    assert!(
        checked > 500,
        "all29 movement families must exercise complete native choices"
    );
    eprintln!(
        "Chu rotation and exact reversal covered {checked} actual native choices across29 faces"
    );
}
