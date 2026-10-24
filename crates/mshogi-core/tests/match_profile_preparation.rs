//! Specification preparation only: future game tests must execute adjudication.
use mshogi_core::chess::{Action, Position, Promotion, Square};
use serde_json::Value;
use std::collections::HashSet;

fn action(text: &str) -> Action {
    let b = text.as_bytes();
    assert!(matches!(b.len(), 4 | 5));
    Action {
        from: Square::new(b[0] - b'a', b[1] - b'1').unwrap(),
        to: Square::new(b[2] - b'a', b[3] - b'1').unwrap(),
        promotion: b.get(4).map(|p| match p {
            b'q' => Promotion::Queen,
            b'r' => Promotion::Rook,
            b'b' => Promotion::Bishop,
            b'n' => Promotion::Knight,
            _ => panic!("fixture promotion"),
        }),
    }
}

#[test]
fn declared_profile_paths_are_valid_and_history_pair_is_identical() {
    let data: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/chess/match-profile.json"
    ))
    .unwrap();
    assert_eq!(data["schema_version"], 1);
    assert_eq!(data["profile"], "mshogi.chess.match.v1");
    let mut ids = HashSet::new();
    let mut repeated = None;
    let mut imported = None;
    for case in data["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        assert!(ids.insert(id));
        assert!(!case["rationale"].as_str().unwrap().is_empty());
        assert!(case["expected"].is_object());
        let mut position = Position::from_fen(case["fen"].as_str().unwrap()).unwrap();
        for text in case["moves"].as_array().unwrap() {
            position.apply(action(text.as_str().unwrap())).unwrap();
        }
        if let Some(text) = case["intended"].as_str() {
            let mut projected = position.clone();
            projected.apply(action(text)).unwrap();
            if let Some(count) = case["expected"]["after_halfmove"].as_u64() {
                assert_eq!(u64::from(projected.halfmove_clock()), count, "{id}");
            }
        }
        if let Some(witness) = case["expected"]["witness"].as_array() {
            let mut projected = position.clone();
            for text in witness {
                projected.apply(action(text.as_str().unwrap())).unwrap();
            }
            assert_eq!(projected.classify().to_string(), "checkmate", "{id}");
        }
        if let Some(class) = case["expected"]["classification"].as_str() {
            assert_eq!(position.classify().to_string(), class, "{id}");
        }
        if let Some(count) = case["expected"]["after_halfmove"].as_u64()
            && case["intended"].is_null()
        {
            assert_eq!(u64::from(position.halfmove_clock()), count, "{id}");
        }
        for key in ["same_repetition_as", "different_repetition_from"] {
            if let Some(fen) = case["expected"][key].as_str() {
                Position::from_fen(fen).unwrap();
            }
        }
        if id == "threefold-current" {
            repeated = Some(position.clone());
        }
        if id == "imported-same-current" {
            imported = Some(position);
        }
    }
    assert_eq!(repeated, imported);
    for case in data["clocks"].as_array().unwrap() {
        assert!(ids.insert(case["id"].as_str().unwrap()));
        assert!(case["expected"].is_object());
        assert!(!case["rationale"].as_str().unwrap().is_empty());
        let mut position = Position::from_fen(
            case["fen"]
                .as_str()
                .unwrap_or("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        )
        .unwrap();
        for event in case["events"].as_array().unwrap() {
            if let Some(text) = event["action"].as_str() {
                // Validate the declared path even when future clock semantics reject it.
                position.apply(action(text)).unwrap();
            }
        }
    }
}
