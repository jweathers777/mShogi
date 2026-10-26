//! Independent board fixtures plus exact native state/reversal checks.
use mshogi_core::shogi::{Action, ActionError, PerftError, PieceKind, Position, Side, Square};
use serde_json::Value;
use std::collections::BTreeSet;

fn names(position: &Position) -> Vec<String> {
    position
        .legal_actions()
        .iter()
        .map(ToString::to_string)
        .collect()
}
fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_owned())
        .collect()
}
fn corpus(text: &str) {
    let data: Value = serde_json::from_str(text).unwrap();
    for case in data["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        assert_eq!(case["rules_profile"], "mshogi.shogi.csa36.v1", "{id}");
        let mut position = Position::from_sfen(case["root_sfen"].as_str().unwrap()).unwrap();
        let root = position.clone();
        let mut prefixes = Vec::new();
        let mut undo_stack = Vec::new();
        for text in strings(&case["moves"]) {
            prefixes.push(position.clone());
            undo_stack.push(
                position
                    .apply(text.parse().unwrap())
                    .unwrap_or_else(|e| panic!("{id} {text}: {e}")),
            );
        }
        let before = position.clone();
        let expected = &case["expected"];
        let actual = names(&position);
        let set: BTreeSet<_> = actual.iter().cloned().collect();
        assert_eq!(actual.len(), set.len(), "{id}: duplicate complete choice");
        assert_eq!(actual, names(&position), "{id}: nondeterministic query");
        if expected.get("legal_moves").is_some() {
            let mut want = strings(&expected["legal_moves"]);
            want.sort();
            assert_eq!(actual, want, "{id}: complete independently expected set");
        }
        if let Some(contains) = expected.get("contains") {
            for text in strings(contains) {
                assert!(set.contains(&text), "{id}: missing {text}");
            }
        }
        if let Some(absent) = expected.get("absent") {
            for text in strings(absent) {
                assert!(!set.contains(&text), "{id}: forbidden {text}");
                if let Ok(action) = text.parse::<Action>() {
                    assert!(position.apply(action).is_err(), "{id}: accepted {text}");
                    assert_eq!(position, before, "{id}: rejection mutated state");
                }
            }
        }
        if let Some(check) = expected.get("in_check") {
            assert_eq!(
                position.is_in_check(position.side_to_move()),
                check.as_bool().unwrap(),
                "{id}"
            );
        }
        if let Some(sfen) = expected.get("sfen") {
            assert_eq!(position.to_sfen(), sfen.as_str().unwrap(), "{id}");
        }
        // Every complete choice restores all state, origin, check and exact
        // repetition identity. Expected transitions remain separate facts.
        for text in &actual {
            let key = position.repetition_identity();
            let record = position.apply(text.parse().unwrap()).unwrap();
            assert!(
                !position.is_in_check(before.side_to_move()),
                "{id}: unsafe {text}"
            );
            if let Some(want) = expected.get("after_sfen").and_then(|v| v.get(text)) {
                assert_eq!(position.to_sfen(), want.as_str().unwrap(), "{id}: {text}");
            }
            position.undo(record).unwrap();
            assert_eq!(position, before, "{id}: {text} did not restore");
            assert_eq!(position.repetition_identity(), key);
        }
        if let Some(perft) = expected.get("perft").and_then(Value::as_object) {
            for (depth, want) in perft {
                assert_eq!(
                    position.perft(depth.parse().unwrap()).unwrap(),
                    want.as_u64().unwrap(),
                    "{id}: perft{depth}"
                );
            }
        }
        if let Some(divide) = expected.get("divide").and_then(Value::as_object) {
            for (depth, want) in divide {
                let got: std::collections::BTreeMap<_, _> = position
                    .divide(depth.parse().unwrap())
                    .unwrap()
                    .into_iter()
                    .map(|e| (e.action.to_string(), e.nodes))
                    .collect();
                let want: std::collections::BTreeMap<_, _> = want
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| (k.to_owned(), v.as_u64().unwrap()))
                    .collect();
                assert_eq!(got, want, "{id}: divide{depth}");
            }
        }
        assert_eq!(position, before, "{id}: query mutated state");
        while let Some(record) = undo_stack.pop() {
            position.undo(record).unwrap();
            assert_eq!(
                position,
                prefixes.pop().unwrap(),
                "{id}: prefix restoration"
            );
        }
        assert_eq!(position, root);
    }
}
#[test]
fn independently_reviewed_board_fixture_contract() {
    corpus(include_str!(
        "../../../tests/fixtures/shogi/board-rules.json"
    ));
}

#[test]
fn independently_reviewed_complete_movement_and_diagnostic_contract() {
    corpus(include_str!(
        "../../../tests/fixtures/shogi/legal-completeness.json"
    ));
}

#[test]
fn counter_domain_never_changes_local_legality_or_identity() {
    let initial = Position::initial();
    let mut max = Position::from_sfen(&initial.to_sfen().replace(" - 1", " - 4294967295")).unwrap();
    assert_eq!(initial.repetition_identity(), max.repetition_identity());
    assert_ne!(initial, max);
    assert_eq!(initial.legal_actions(), max.legal_actions());
    assert_eq!(initial.pseudo_legal_actions(), max.pseudo_legal_actions());
    assert_eq!(max.perft(1).unwrap(), 30);
    for action in max.legal_actions() {
        let before = max.clone();
        assert!(max.is_legal(action));
        assert_eq!(max.apply(action).unwrap_err(), ActionError::CounterOverflow);
        assert_eq!(max, before);
    }
    assert_eq!(initial.perft(0).unwrap(), 1);
    assert_eq!(initial.divide(0), Err(PerftError::ZeroDivideDepth));
}

#[test]
fn attacks_candidates_and_legal_choices_are_distinct() {
    let position = Position::from_sfen("4r3k/9/9/9/9/9/9/4G4/4K4 b - 1").unwrap();
    let leave_pin: Action = "5h4h".parse().unwrap();
    assert!(position.pseudo_legal_actions().contains(&leave_pin));
    assert!(!position.legal_actions().contains(&leave_pin));
    let mut copy = position.clone();
    assert_eq!(copy.apply(leave_pin).unwrap_err(), ActionError::SelfCheck);
    assert_eq!(copy, position);
    let target: Square = "5h".parse().unwrap();
    assert!(position.is_square_attacked(target, Side::Gote));
    let mate = Position::from_sfen("3lkl3/3p1p3/4G4/9/9/9/9/9/K8 b P 1").unwrap();
    let drop = "P*5b".parse().unwrap();
    assert!(mate.pseudo_legal_actions().contains(&drop));
    let mut copy = mate.clone();
    assert_eq!(copy.apply(drop).unwrap_err(), ActionError::PawnDropMate);
    assert_eq!(copy, mate);
    // Even structurally imported adjacent kings have no king-capture action.
    let adjacent = Position::from_sfen("4k4/4K4/9/9/9/9/9/9/9 b - 1").unwrap();
    assert!(!adjacent.legal_actions().contains(&"5b5a".parse().unwrap()));
}

#[test]
fn deterministic_sequences_restore_every_prefix_and_material() {
    for mut seed in [1_u64, 20261026, 0x5eed] {
        let mut position = Position::initial();
        let root = position.clone();
        let mut prefixes = Vec::new();
        let mut records = Vec::new();
        for _ in 0..96 {
            let choices = position.legal_actions();
            if choices.is_empty() {
                break;
            }
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let action = choices[(seed as usize) % choices.len()];
            prefixes.push(position.clone());
            records.push(position.apply(action).unwrap());
            for kind in PieceKind::HAND_KINDS {
                let sum = position
                    .board()
                    .iter()
                    .flatten()
                    .filter(|p| p.kind == kind)
                    .count()
                    + position.hand(Side::Sente).count(kind) as usize
                    + position.hand(Side::Gote).count(kind) as usize;
                let original = root
                    .board()
                    .iter()
                    .flatten()
                    .filter(|p| p.kind == kind)
                    .count();
                assert_eq!(sum, original, "{kind:?}");
            }
            let roundtrip = Position::from_sfen(&position.to_sfen()).unwrap();
            assert_eq!(
                roundtrip.repetition_identity(),
                position.repetition_identity()
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
fn structural_diagrams_outside_reference_domain_still_use_native_king_safety() {
    // Original SH-F022 v1 had both kings checked. It is a structural diagram,
    // but cshogi's incremental check state assumes the nonmoving king is safe.
    // Keep its independently derived native behavior; never match that helper's
    // unsupported result. Reference fixtures v2 relocate the far king instead.
    let mut p = Position::from_sfen("8k/9/4b4/5B3/9/9/K8/9/9 b - 1").unwrap();
    let root = p.clone();
    assert!(p.is_in_check(Side::Sente));
    assert!(p.is_in_check(Side::Gote));
    assert!(!p.pseudo_legal_actions().contains(&"4d1a".parse().unwrap()));
    let record = p.apply("9g8g".parse().unwrap()).unwrap();
    assert!(p.is_in_check(Side::Gote));
    assert_eq!(names(&p), ["1a1b", "1a2a", "5c4d"]);
    p.undo(record).unwrap();
    assert_eq!(p, root);
}

#[test]
fn pawn_drop_defense_keeps_compulsory_knight_promotion_and_preexisting_check_distinct() {
    // Independently drawn by the separate reviewer: the knight on4f is the
    // only legal capture of the dropped checking pawn, and reaching5h for Gote
    // requires promotion. King escape/capture squares are occupied or defended.
    let mut p = Position::from_sfen("9/9/9/9/9/3ppn3/3lkl3/3b1p3/4G3K b P 1").unwrap();
    let root = p.clone();
    let drop = p.apply("P*5h".parse().unwrap()).unwrap();
    assert_eq!(names(&p), ["4f5h+"]);
    let capture = p.apply("4f5h+".parse().unwrap()).unwrap();
    p.undo(capture).unwrap();
    p.undo(drop).unwrap();
    assert_eq!(p, root);
    // A remote pawn drop does not deliver mate merely because an imported
    // diagram was already checking the other king with a rook.
    let mut prechecked = Position::from_sfen("8k/9/9/9/9/9/9/9/K7R b P 1").unwrap();
    assert!(prechecked.is_in_check(Side::Gote));
    prechecked.apply("P*5e".parse().unwrap()).unwrap();
}
