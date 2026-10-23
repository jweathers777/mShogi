//! Independently expected castling and en passant execution, candidates,
//! atomic rejection and exact undo.

use std::collections::BTreeSet;

use mshogi_core::{
    CHESS_POSITION_PROFILE_ID,
    chess::{Action, ActionError, Position, Promotion, Square},
};
use serde_json::Value;

// Test-data adapter only; no production action notation is established here.
fn action(text: &str) -> Action {
    let bytes = text.as_bytes();
    assert!(matches!(bytes.len(), 4 | 5), "{text}");
    Action {
        from: Square::new(bytes[0] - b'a', bytes[1] - b'1').unwrap(),
        to: Square::new(bytes[2] - b'a', bytes[3] - b'1').unwrap(),
        promotion: bytes.get(4).map(|choice| match choice {
            b'q' => Promotion::Queen,
            b'r' => Promotion::Rook,
            b'b' => Promotion::Bishop,
            b'n' => Promotion::Knight,
            _ => panic!("unsupported fixture promotion"),
        }),
    }
}

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

fn sorted(actions: Vec<Action>) -> Vec<String> {
    let mut texts: Vec<String> = actions.into_iter().map(text).collect();
    texts.sort();
    texts
}

fn data() -> Value {
    serde_json::from_str(include_str!(
        "../../../tests/fixtures/chess/special-actions.json"
    ))
    .unwrap()
}

fn error_name(error: ActionError) -> &'static str {
    use ActionError::*;
    match error {
        EmptySource => "EmptySource",
        WrongSide => "WrongSide",
        SameSquare => "SameSquare",
        FriendlyDestination => "FriendlyDestination",
        KingCapture => "KingCapture",
        IllegalGeometry => "IllegalGeometry",
        MissingPromotion => "MissingPromotion",
        UnexpectedPromotion => "UnexpectedPromotion",
        CastlingNotOrdinary => "CastlingNotOrdinary",
        EnPassantNotOrdinary => "EnPassantNotOrdinary",
        CastlingUnavailable => "CastlingUnavailable",
        CastlingKingAttacked => "CastlingKingAttacked",
        CounterOverflow => "CounterOverflow",
        IllegalPosition(_) => "IllegalPosition",
    }
}

/// Check the documented candidate-versus-`apply` contract on every bounded
/// action and return the accepted (legal) actions, sorted.
fn assert_contract(id: &str, position: &Position) -> Vec<String> {
    let candidates = position.candidates();
    let unique: BTreeSet<String> = candidates.iter().copied().map(text).collect();
    assert_eq!(unique.len(), candidates.len(), "{id}: duplicate candidates");
    let choices = [
        None,
        Some(Promotion::Queen),
        Some(Promotion::Rook),
        Some(Promotion::Bishop),
        Some(Promotion::Knight),
    ];
    let mut accepted = Vec::new();
    for from in (0..64).map(|i| Square::new(i % 8, i / 8).unwrap()) {
        for to in (0..64).map(|i| Square::new(i % 8, i / 8).unwrap()) {
            for promotion in choices {
                let candidate = Action {
                    from,
                    to,
                    promotion,
                };
                let is_candidate = unique.contains(&text(candidate));
                let mut trial = position.clone();
                match trial.apply(candidate) {
                    Ok(record) => {
                        assert!(
                            is_candidate,
                            "{id}: accepted non-candidate {}",
                            text(candidate)
                        );
                        trial.undo(&record).unwrap();
                        assert_eq!(&trial, position, "{id}: undo {}", text(candidate));
                        accepted.push(text(candidate));
                    }
                    Err(error) => {
                        assert_eq!(&trial, position, "{id}: failed {} mutated", text(candidate));
                        let allowed = matches!(
                            error,
                            ActionError::CastlingKingAttacked
                                | ActionError::IllegalPosition(_)
                                | ActionError::CounterOverflow
                        );
                        assert!(
                            !is_candidate || allowed,
                            "{id}: candidate {} rejected as {error:?}",
                            text(candidate)
                        );
                    }
                }
            }
        }
    }
    accepted.sort();
    accepted
}

#[test]
fn independent_positions_match_special_candidates_and_full_legal_sets() {
    let data = data();
    assert_eq!(data["format"], "mshogi.chess.special-actions.v1");
    assert_eq!(data["schema_version"], 1);
    assert_eq!(data["rules_profile"], CHESS_POSITION_PROFILE_ID);
    let mut count = 0;
    for case in data["positions"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        assert_eq!(case["rules_profile"], CHESS_POSITION_PROFILE_ID, "{id}");
        let position = Position::from_fen(case["fen"].as_str().unwrap()).unwrap();
        assert_eq!(
            sorted(position.special_candidates()),
            strings(&case["special_candidates"]),
            "{id}: special candidates"
        );
        let legal = assert_contract(id, &position);
        assert_eq!(legal, strings(&case["legal"]), "{id}: legal set");
        let special: BTreeSet<String> = strings(&case["special_candidates"]).into_iter().collect();
        let legal_special: Vec<String> = legal
            .into_iter()
            .filter(|action| special.contains(action))
            .collect();
        assert_eq!(legal_special, strings(&case["legal_special"]), "{id}");
        count += 1;
    }
    assert_eq!(count, 21);
}

#[test]
fn independent_sequences_reach_exact_states_and_undo_in_reverse() {
    let data = data();
    let mut steps = 0;
    for case in data["sequences"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let start = Position::from_fen(case["before_fen"].as_str().unwrap()).unwrap();
        let mut position = start.clone();
        let mut history = vec![];
        for step in case["steps"].as_array().unwrap() {
            let before = position.clone();
            let record = position
                .apply(action(step["action"].as_str().unwrap()))
                .unwrap_or_else(|error| panic!("{id}: {error}"));
            assert_eq!(position.to_fen(), step["after_fen"], "{id}");
            assert_eq!(
                position,
                Position::from_fen(step["after_fen"].as_str().unwrap()).unwrap(),
                "{id}: full saved state"
            );
            history.push((before, record));
            steps += 1;
        }
        while let Some((before, record)) = history.pop() {
            position.undo(&record).unwrap();
            assert_eq!(position, before, "{id}: reverse step");
            assert_eq!(
                position.undo(&record),
                Err(mshogi_core::chess::UndoError::PositionMismatch)
            );
        }
        assert_eq!(position, start, "{id}");
    }
    assert_eq!(steps, 25);
}

#[test]
fn independent_rejections_report_the_expected_error_without_mutation() {
    let data = data();
    let mut count = 0;
    for case in data["rejections"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let mut position = Position::from_fen(case["fen"].as_str().unwrap()).unwrap();
        let before = position.clone();
        let error = position
            .apply(action(case["action"].as_str().unwrap()))
            .expect_err(id);
        assert_eq!(error_name(error), case["error"], "{id}: {error}");
        assert_eq!(position, before, "{id}: rejected action mutated state");
        count += 1;
    }
    assert_eq!(count, 18);
}

#[test]
fn corpus_legal_sets_equal_candidates_accepted_by_apply() {
    let corpus: Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/chess/corpus.json")).unwrap();
    let mut positions = 0;
    for case in corpus["cases"].as_array().unwrap() {
        if case["operation"] != "position" {
            continue;
        }
        let id = case["id"].as_str().unwrap();
        let position = Position::from_fen(case["input"]["fen"].as_str().unwrap()).unwrap();
        let mut expected = strings(&case["expected"]["legal_moves"]);
        expected.sort();
        assert_eq!(assert_contract(id, &position), expected, "{id}");
        positions += 1;
    }
    assert_eq!(positions, 35);
}

#[test]
fn ordinary_only_boundary_still_refuses_specials_that_apply_executes() {
    for (fen, special) in [
        ("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", "e1g1"),
        ("r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 0 1", "e8c8"),
        ("7k/8/8/3pP3/8/8/8/K7 w - d6 0 2", "e5d6"),
    ] {
        let mut position = Position::from_fen(fen).unwrap();
        let before = position.clone();
        let error = position.apply_ordinary(action(special)).unwrap_err();
        assert!(matches!(
            error,
            ActionError::CastlingNotOrdinary | ActionError::EnPassantNotOrdinary
        ));
        assert_eq!(position, before);
        assert!(position.apply(action(special)).is_ok(), "{fen} {special}");
        assert!(!position.ordinary_candidates().is_empty());
    }
    let position = Position::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
    assert!(
        position
            .ordinary_candidates()
            .iter()
            .all(|candidate| !position.special_candidates().contains(candidate))
    );
}

#[test]
fn castling_effects_and_rights_are_exact_for_each_side_and_color() {
    for (fen, king, after) in [
        (
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 5 9",
            "e1g1",
            "r3k2r/8/8/8/8/8/8/R4RK1 b kq - 6 9",
        ),
        (
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 5 9",
            "e1c1",
            "r3k2r/8/8/8/8/8/8/2KR3R b kq - 6 9",
        ),
        (
            "r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 5 9",
            "e8g8",
            "r4rk1/8/8/8/8/8/8/R3K2R w KQ - 6 10",
        ),
        (
            "r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 5 9",
            "e8c8",
            "2kr3r/8/8/8/8/8/8/R3K2R w KQ - 6 10",
        ),
    ] {
        let mut position = Position::from_fen(fen).unwrap();
        let record = position.apply(action(king)).unwrap();
        assert_eq!(position.to_fen(), after, "{king}");
        position.undo(&record).unwrap();
        assert_eq!(position.to_fen(), fen, "{king}");
    }
}
