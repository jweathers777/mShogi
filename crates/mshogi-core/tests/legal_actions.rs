//! Independently expected legal action sets and position-level classification.

use std::collections::BTreeSet;

use mshogi_core::{
    CHESS_POSITION_PROFILE_ID,
    chess::{Action, ActionError, Classification, Position, Promotion, Square},
};
use serde_json::Value;

// Test-data adapter only; no production action notation is established here.
fn text(action: Action) -> String {
    let suffix = match action.promotion {
        None => "",
        Some(Promotion::Queen) => "q",
        Some(Promotion::Rook) => "r",
        Some(Promotion::Bishop) => "b",
        Some(Promotion::Knight) => "n",
    };
    format!("{}{}{suffix}", action.from, action.to)
}

fn strings(values: &Value) -> Vec<String> {
    values
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect()
}

fn class_name(class: Classification) -> String {
    class.to_string()
}

fn every_action() -> Vec<Action> {
    let squares: Vec<Square> = (0..64)
        .map(|i| Square::new(i % 8, i / 8).unwrap())
        .collect();
    let choices = [
        None,
        Some(Promotion::Queen),
        Some(Promotion::Rook),
        Some(Promotion::Bishop),
        Some(Promotion::Knight),
    ];
    let mut actions = Vec::new();
    for &from in &squares {
        for &to in &squares {
            for promotion in choices {
                actions.push(Action {
                    from,
                    to,
                    promotion,
                });
            }
        }
    }
    actions
}

/// Compare the public legal list with an expected set and check that every
/// listed action applies and undoes exactly while every other bounded action
/// is rejected atomically.
fn assert_legal(id: &str, position: &Position, expected: &[String]) {
    let legal = position.legal_actions();
    let texts: Vec<String> = legal.iter().copied().map(text).collect();
    let unique: BTreeSet<&String> = texts.iter().collect();
    assert_eq!(unique.len(), texts.len(), "{id}: duplicate legal actions");
    let mut sorted = texts.clone();
    sorted.sort();
    let mut expected = expected.to_vec();
    expected.sort();
    assert_eq!(sorted, expected, "{id}: legal set");
    for action in every_action() {
        let mut trial = position.clone();
        let listed = unique.contains(&text(action));
        match trial.apply(action) {
            Ok(record) => {
                assert!(listed, "{id}: apply accepted unlisted {}", text(action));
                trial.undo(&record).unwrap();
                assert_eq!(&trial, position, "{id}: undo {}", text(action));
            }
            Err(error) => {
                assert_eq!(&trial, position, "{id}: failed {} mutated", text(action));
                assert!(
                    !listed || error == ActionError::CounterOverflow,
                    "{id}: legal {} rejected as {error:?}",
                    text(action)
                );
            }
        }
    }
}

#[test]
fn corpus_legal_sets_and_classifications_match_exactly() {
    let corpus: Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/chess/corpus.json")).unwrap();
    let mut seen = BTreeSet::new();
    for case in corpus["cases"].as_array().unwrap() {
        if case["operation"] != "position" {
            continue;
        }
        let id = case["id"].as_str().unwrap();
        let position = Position::from_fen(case["input"]["fen"].as_str().unwrap()).unwrap();
        assert_legal(id, &position, &strings(&case["expected"]["legal_moves"]));
        let class = position.classify();
        assert_eq!(
            class_name(class),
            case["expected"]["classification"],
            "{id}"
        );
        assert_eq!(
            position.in_check(),
            matches!(class, Classification::Check | Classification::Checkmate),
            "{id}"
        );
        seen.insert(class_name(class));
    }
    assert_eq!(seen.len(), 4, "all four classifications occur");
}

#[test]
fn independent_classification_fixtures_match_exactly() {
    let data: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/chess/classification.json"
    ))
    .unwrap();
    assert_eq!(data["format"], "mshogi.chess.classification.v1");
    assert_eq!(data["schema_version"], 1);
    assert_eq!(data["rules_profile"], CHESS_POSITION_PROFILE_ID);
    let mut count = 0;
    for case in data["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        assert_eq!(case["rules_profile"], CHESS_POSITION_PROFILE_ID, "{id}");
        let position = Position::from_fen(case["fen"].as_str().unwrap()).unwrap();
        assert_legal(id, &position, &strings(&case["legal"]));
        assert_eq!(
            class_name(position.classify()),
            case["classification"],
            "{id}"
        );
        assert_eq!(
            case["hand_expectations"]["classification"],
            case["classification"]
        );
        count += 1;
    }
    assert_eq!(count, 8);
}

#[test]
fn legal_sets_match_the_special_and_candidate_fixture_families() {
    let special: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/chess/special-actions.json"
    ))
    .unwrap();
    for case in special["positions"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let position = Position::from_fen(case["fen"].as_str().unwrap()).unwrap();
        assert_legal(id, &position, &strings(&case["legal"]));
    }
    // The candidate family records legal ordinary actions only. Its legal sets
    // must add exactly the special actions its two exclusion cases describe.
    let candidates: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/chess/ordinary-candidates.json"
    ))
    .unwrap();
    for case in candidates["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let position = Position::from_fen(case["fen"].as_str().unwrap()).unwrap();
        let legal: BTreeSet<String> = position.legal_actions().into_iter().map(text).collect();
        let ordinary: BTreeSet<String> = strings(&case["legal_ordinary"]).into_iter().collect();
        assert!(ordinary.is_subset(&legal), "{id}");
        let extra: Vec<&String> = legal.difference(&ordinary).collect();
        match id {
            "en-passant-excluded" => assert_eq!(extra, ["e5f6"], "{id}"),
            "castling-excluded" => assert_eq!(extra, ["e1c1", "e1g1"], "{id}"),
            _ => assert!(extra.is_empty(), "{id}: {extra:?}"),
        }
    }
}

#[test]
fn empty_legal_sets_are_distinguished_by_check() {
    for (fen, class) in [
        (
            "7k/8/8/8/8/8/5PPP/r5K1 w - - 0 1",
            Classification::Checkmate,
        ),
        (
            "k7/1nK5/1P6/3B4/8/8/8/8 b - - 0 1",
            Classification::Stalemate,
        ),
    ] {
        let position = Position::from_fen(fen).unwrap();
        assert!(position.legal_actions().is_empty(), "{fen}");
        // Pseudo-legal candidates still exist; only the legal set is empty.
        assert!(!position.candidates().is_empty(), "{fen}");
        assert_eq!(position.in_check(), class == Classification::Checkmate);
        assert_eq!(position.classify(), class, "{fen}");
    }
}

#[test]
fn legality_and_classification_ignore_the_counter_domain() {
    let limit = Position::from_fen("7k/8/8/8/8/8/8/K7 w - - 4294967295 1").unwrap();
    let normal = Position::from_fen("7k/8/8/8/8/8/8/K7 w - - 0 1").unwrap();
    assert_eq!(limit.legal_actions(), normal.legal_actions());
    assert_eq!(limit.classify(), Classification::Quiet);
    for action in limit.legal_actions() {
        let mut trial = limit.clone();
        assert_eq!(
            trial.apply(action).unwrap_err(),
            ActionError::CounterOverflow
        );
        assert_eq!(trial, limit);
    }
    // A pawn move resets the halfmove clock and therefore still applies.
    let mut pawn = Position::from_fen("7k/8/8/8/8/8/P7/K7 w - - 4294967295 1").unwrap();
    let push = pawn
        .legal_actions()
        .into_iter()
        .find(|action| action.from == Square::new(0, 1).unwrap())
        .unwrap();
    assert!(pawn.apply(push).is_ok());
}

#[test]
fn queries_do_not_mutate_and_are_deterministic() {
    let position =
        Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();
    let before = position.clone();
    assert_eq!(position.legal_actions(), position.legal_actions());
    assert_eq!(position.legal_actions().len(), 48);
    assert_eq!(position.classify(), Classification::Quiet);
    assert!(!position.in_check());
    assert_eq!(position, before);
}
