//! Frozen independent WXF board facts, exact reversible actions and board-only diagnostics.
//! History-dependent WXF warnings, claims and game conclusions are deliberately absent.
use mshogi_core::xiangqi::{Action, LocalStatus, NoActionReason, PieceKind, Position, Side};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

fn fixtures() -> Vec<Value> {
    let data: Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/xiangqi/board.json")).unwrap();
    assert_eq!(data["format"], "mshogi.xiangqi.board-fixtures.v1");
    assert_eq!(data["profile"], "mshogi.xiangqi.wxf2018.v1");
    data["fixtures"].as_array().unwrap().clone()
}

fn strings(value: &Value) -> BTreeSet<String> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_owned())
        .collect()
}

fn choices(position: &Position) -> BTreeSet<String> {
    let actions = position.legal_actions().unwrap();
    let names: BTreeSet<_> = actions.iter().map(ToString::to_string).collect();
    assert_eq!(names.len(), actions.len(), "duplicate complete actions");
    assert_eq!(actions, position.legal_actions().unwrap(), "unstable query");
    names
}

fn check_every_reversal(position: &mut Position, label: &str) {
    let before = position.clone();
    let fen = before.to_fen().unwrap();
    let actions = position.legal_actions().unwrap();
    for action in actions {
        let moving = before.piece(action.from).unwrap();
        let captured = before.piece(action.to);
        assert_eq!(moving.side, before.side_to_move(), "{label}: {action}");
        assert!(captured.is_none_or(|p| p.side != moving.side));
        assert!(captured.is_none_or(|p| p.kind != PieceKind::General));
        let resolved = position.resolve_action(action).unwrap();
        assert_eq!(position, &before, "{label}: resolution changed state");
        let effects = resolved.effects();
        assert_eq!(effects.action, action);
        assert_eq!(effects.moving, moving);
        assert_eq!(effects.captured, captured);
        let undo = position.apply_resolved(resolved).unwrap();
        assert_eq!(undo.effects(), effects);
        assert_eq!(position.piece(action.from), None);
        assert_eq!(position.piece(action.to), Some(moving));
        assert_eq!(position.side_to_move(), before.side_to_move().opponent());
        assert!(
            !position.is_in_check(moving.side),
            "{label}: unsafe {action}"
        );
        assert_eq!(
            position.pieces().count(),
            before.pieces().count() - usize::from(captured.is_some())
        );
        let applied = position.clone();
        position.undo(undo).unwrap();
        assert_eq!(position, &before, "{label}: prepared {action} undo");
        assert_eq!(position.to_fen().unwrap(), fen);
        let direct = position.apply(action).unwrap();
        assert_eq!(position, &applied, "{label}: direct/prepared disagree");
        assert_eq!(direct.effects(), effects);
        position.undo(direct).unwrap();
        assert_eq!(position, &before, "{label}: direct {action} undo");
    }
}

#[test]
fn frozen_independent_board_facts_and_every_generated_action_restore_exactly() {
    let cases = fixtures();
    assert_eq!(cases.len(), 105);
    assert_eq!(
        cases
            .iter()
            .filter(|f| f["reference_eligible"] == true)
            .count(),
        101
    );
    for case in cases {
        let id = case["id"].as_str().unwrap();
        let mut position = Position::from_fen(case["fen"].as_str().unwrap()).unwrap();
        let before = position.clone();
        if case["domain"] == "structural_analysis" {
            assert_eq!(case["reference_eligible"], false, "{id}");
            assert!(position.validate_local().is_err(), "{id}: live admission");
            assert!(position.legal_actions().is_err(), "{id}: live legal set");
            assert!(
                position.pseudo_legal_actions().is_err(),
                "{id}: live pseudo set"
            );
            assert!(
                position.local_status().is_err(),
                "{id}: invented local status"
            );
            if let Some(absent) = case.get("must_exclude") {
                for name in strings(absent) {
                    assert!(
                        position.apply(name.parse().unwrap()).is_err(),
                        "{id}: {name}"
                    );
                    assert_eq!(position, before, "{id}: rejection changed diagram");
                }
            }
            continue;
        }
        position
            .validate_local()
            .unwrap_or_else(|e| panic!("{id}: {e}"));
        let actual = choices(&position);
        if let Some(expected) = case.get("legal_moves") {
            assert_eq!(
                actual,
                strings(expected),
                "{id}: exhaustive independent set"
            );
        }
        if let Some(expected) = case.get("moves_from").and_then(Value::as_object) {
            for (origin, names) in expected {
                let selected: BTreeSet<_> = actual
                    .iter()
                    .filter(|name| name.starts_with(origin))
                    .cloned()
                    .collect();
                assert_eq!(selected, strings(names), "{id}: complete origin {origin}");
            }
        }
        if let Some(expected) = case.get("must_include") {
            for name in strings(expected) {
                assert!(actual.contains(&name), "{id}: missing {name}");
            }
        }
        if let Some(expected) = case.get("must_exclude") {
            for name in strings(expected) {
                assert!(!actual.contains(&name), "{id}: forbidden {name}");
                assert!(
                    position.resolve_action(name.parse().unwrap()).is_err(),
                    "{id}: prepared {name}"
                );
                assert!(
                    position.apply(name.parse().unwrap()).is_err(),
                    "{id}: accepted {name}"
                );
                assert_eq!(position, before, "{id}: atomic rejection {name}");
            }
        }
        if let Some(check) = case.get("in_check") {
            assert_eq!(
                position.is_in_check(position.side_to_move()),
                check.as_bool().unwrap(),
                "{id}: independently expected threat state"
            );
        }
        if let Some(expected) = case.get("perft").and_then(Value::as_object) {
            for (depth, nodes) in expected {
                assert_eq!(
                    position.perft(depth.parse().unwrap()).unwrap(),
                    nodes.as_u64().unwrap(),
                    "{id}: hand-derived perft{depth}"
                );
            }
        }
        if let Some(expected) = case.get("divide").and_then(Value::as_object) {
            for (depth, branches) in expected {
                let actual: BTreeMap<_, _> = position
                    .divide(depth.parse().unwrap())
                    .unwrap()
                    .into_iter()
                    .map(|(action, nodes)| (action.to_string(), nodes))
                    .collect();
                let expected: BTreeMap<_, _> = branches
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(action, nodes)| (action.clone(), nodes.as_u64().unwrap()))
                    .collect();
                assert_eq!(actual, expected, "{id}: hand-derived divide{depth}");
            }
        }
        check_every_reversal(&mut position, id);
        assert_eq!(position, before, "{id}: read-only queries changed state");
    }
}

#[test]
fn opening_and_pinned_geometry_are_not_game_outcomes() {
    let initial = Position::initial();
    assert_eq!(initial.legal_actions().unwrap().len(), 44);
    assert_eq!(initial.perft(1).unwrap(), 44);
    assert_eq!(
        initial.local_status().unwrap(),
        LocalStatus::Ongoing { in_check: false }
    );
    let cases = fixtures();
    for (id, candidate) in [
        ("XQ-B29", "e4d6"),
        ("XQ-B32", "e1d1"),
        ("XQ-B33", "f1e1"),
        ("XQ-B34", "e2d2"),
    ] {
        let case = cases.iter().find(|f| f["id"] == id).unwrap();
        let mut position = Position::from_fen(case["fen"].as_str().unwrap()).unwrap();
        let before = position.clone();
        let action: Action = candidate.parse().unwrap();
        assert!(
            position.pseudo_legal_actions().unwrap().contains(&action),
            "{id}"
        );
        assert!(!position.legal_actions().unwrap().contains(&action), "{id}");
        assert!(position.apply(action).is_err(), "{id}: exposed General");
        assert_eq!(position, before);
    }
    for (id, reason) in [
        ("XQ-B42", NoActionReason::Stalemate),
        ("XQ-B43", NoActionReason::Checkmate),
        ("XQ-B42-black", NoActionReason::Stalemate),
        ("XQ-B43-black", NoActionReason::Checkmate),
    ] {
        let case = cases.iter().find(|f| f["id"] == id).unwrap();
        let position = Position::from_fen(case["fen"].as_str().unwrap()).unwrap();
        assert_eq!(
            position.local_status().unwrap(),
            LocalStatus::NoLegalAction { reason },
            "{id}"
        );
    }
}

#[test]
fn deterministic_sequences_restore_every_choice_and_every_recorded_prefix() {
    for mut seed in [1_u64, 20261027, 0x5eed] {
        let mut position = Position::initial();
        let root = position.clone();
        let mut prefixes = Vec::new();
        let mut records = Vec::new();
        for ply in 0..64 {
            let actions = position.legal_actions().unwrap();
            if actions.is_empty() {
                break;
            }
            check_every_reversal(&mut position, &format!("seed{seed}/ply{ply}"));
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let action = actions[(seed as usize) % actions.len()];
            prefixes.push(position.clone());
            records.push(position.apply(action).unwrap());
            assert!(position.validate_local().is_ok());
            assert_eq!(
                Position::from_fen(&position.to_fen().unwrap()).unwrap(),
                position
            );
        }
        while let Some(record) = records.pop() {
            position.undo(record).unwrap();
            assert_eq!(position, prefixes.pop().unwrap());
        }
        assert_eq!(position, root);
    }
}

#[test]
fn same_board_different_known_history_seeds_keep_identical_local_action_sets() {
    let cases = fixtures();
    let checking = cases.iter().find(|f| f["id"] == "XQ-F002-C").unwrap();
    let quiet = cases.iter().find(|f| f["id"] == "XQ-F002-Q").unwrap();
    assert_eq!(checking["fen"], quiet["fen"]);
    assert_ne!(checking["history_seed"], quiet["history_seed"]);
    let mut final_positions = Vec::new();
    for (case, gives_check) in [(checking, true), (quiet, false)] {
        let mut position = Position::from_fen(case["fen"].as_str().unwrap()).unwrap();
        let root = position.clone();
        let mut records = Vec::new();
        for name in case["history_seed"].as_array().unwrap() {
            let moving = position.side_to_move();
            records.push(
                position
                    .apply(name.as_str().unwrap().parse().unwrap())
                    .unwrap(),
            );
            if moving == Side::Red {
                assert_eq!(position.is_in_check(Side::Black), gives_check);
            }
        }
        assert_eq!(position.side_to_move(), root.side_to_move());
        assert_eq!(
            position.pieces().collect::<Vec<_>>(),
            root.pieces().collect::<Vec<_>>()
        );
        assert_eq!(choices(&position), choices(&root));
        final_positions.push(position.clone());
        while let Some(record) = records.pop() {
            position.undo(record).unwrap();
        }
        assert_eq!(position, root);
    }
    assert_eq!(final_positions[0], final_positions[1]);
    // No repetition identity, warning, claim or WXF adjudication is inferred.
}

#[test]
fn pinned_enemy_rook_still_geometrically_controls_general_destination() {
    // Independent diagram, WXF2.4/2.10: Red General e1, Red Rook d0,
    // Black General d9, Black Rook d2. The rook on d2 interposes on the
    // d0-d9 line, so it is pinned to its own General. Nonetheless it attacks
    // e2 geometrically; General safety must not recursively ask whether that
    // enemy rook could legally move d2-e2 and expose its own General. There is
    // no actual General-capture continuation in this test or native contract.
    let mut position = Position::from_fen("3k5/9/9/9/9/9/9/3r5/4K4/3R5 w - - 0 1").unwrap();
    let before = position.clone();
    let action: Action = "e1e2".parse().unwrap();
    position.validate_local().unwrap();
    assert!(!position.is_in_check(Side::Red));
    assert!(position.is_attacked("e2".parse().unwrap(), Side::Black));
    assert!(position.pseudo_legal_actions().unwrap().contains(&action));
    assert!(!position.legal_actions().unwrap().contains(&action));
    assert!(position.resolve_action(action).is_err());
    assert!(position.apply(action).is_err());
    assert_eq!(position, before);
}
