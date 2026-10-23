//! Independently expected pseudo-legal ordinary candidates and their contract
//! with the checked transition boundary.

use std::collections::BTreeSet;

use mshogi_core::{
    CHESS_POSITION_PROFILE_ID,
    chess::{Action, ActionError, PieceKind, Position, Promotion, Square},
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

fn all_squares() -> impl Iterator<Item = Square> {
    (0..8).flat_map(|rank| (0..8).map(move |file| Square::new(file, rank).unwrap()))
}

// Every bounded action shape, including invalid ones, for exhaustive comparison.
fn every_action() -> Vec<Action> {
    let choices = [
        None,
        Some(Promotion::Queen),
        Some(Promotion::Rook),
        Some(Promotion::Bishop),
        Some(Promotion::Knight),
    ];
    let mut actions = Vec::new();
    for from in all_squares() {
        for to in all_squares() {
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

fn strings(values: &Value) -> Vec<String> {
    values
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect()
}

fn sorted_texts(position: &Position) -> Vec<String> {
    let mut texts: Vec<String> = position
        .ordinary_candidates()
        .into_iter()
        .map(text)
        .collect();
    texts.sort();
    texts
}

/// Check the documented candidate-versus-checked-boundary contract on every
/// bounded action and return the accepted (legal ordinary) actions, sorted.
fn assert_contract(id: &str, position: &Position) -> Vec<String> {
    let candidates = position.ordinary_candidates();
    let unique: BTreeSet<String> = candidates.iter().copied().map(text).collect();
    assert_eq!(unique.len(), candidates.len(), "{id}: duplicate candidates");
    let mut accepted = Vec::new();
    for action in every_action() {
        let mut trial = position.clone();
        let result = trial.apply_ordinary(action);
        let is_candidate = unique.contains(&text(action));
        match result {
            Ok(record) => {
                assert!(
                    is_candidate,
                    "{id}: accepted non-candidate {}",
                    text(action)
                );
                trial.undo(&record).unwrap();
                assert_eq!(&trial, position, "{id}: undo {}", text(action));
                accepted.push(text(action));
            }
            Err(ActionError::IllegalPosition(_) | ActionError::CounterOverflow) => {}
            Err(error) => assert!(
                !is_candidate,
                "{id}: candidate {} rejected as {error:?}",
                text(action)
            ),
        }
        if !is_candidate {
            continue;
        }
        let mover = position.piece_at(action.from).unwrap();
        assert_eq!(mover.color, position.side_to_move(), "{id}");
        if let Some(victim) = position.piece_at(action.to) {
            assert_ne!(victim.color, mover.color, "{id}: own capture");
            assert_ne!(victim.kind, PieceKind::King, "{id}: king capture");
        }
    }
    accepted.sort();
    accepted
}

#[test]
fn independent_candidate_fixtures_match_exact_sets_and_contract() {
    let data: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/chess/ordinary-candidates.json"
    ))
    .unwrap();
    assert_eq!(data["format"], "mshogi.chess.ordinary-candidates.v1");
    assert_eq!(data["schema_version"], 1);
    assert_eq!(data["rules_profile"], CHESS_POSITION_PROFILE_ID);
    let mut ids = BTreeSet::new();
    let (mut candidates, mut legal) = (0, 0);
    for case in data["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        assert!(ids.insert(id), "{id}: duplicate ID");
        assert_eq!(case["rules_profile"], CHESS_POSITION_PROFILE_ID, "{id}");
        let position = Position::from_fen(case["fen"].as_str().unwrap())
            .unwrap_or_else(|error| panic!("{id}: {error}"));
        let expected = strings(&case["candidates"]);
        let actual = sorted_texts(&position);
        assert_eq!(actual, expected, "{id}: candidate set");
        let hand = &case["hand_expectations"];
        assert_eq!(
            actual.len() as u64,
            hand["candidate_count"],
            "{id}: hand count"
        );
        for required in strings(&hand["required"]) {
            assert!(actual.contains(&required), "{id}: missing {required}");
        }
        for excluded in strings(&hand["excluded"]) {
            assert!(!actual.contains(&excluded), "{id}: unexpected {excluded}");
        }
        assert_eq!(
            assert_contract(id, &position),
            strings(&case["legal_ordinary"]),
            "{id}: checked-boundary acceptance"
        );
        assert_eq!(
            case["legal_ordinary"].as_array().unwrap().len() as u64,
            hand["legal_ordinary_count"],
            "{id}: hand legal count"
        );
        candidates += expected.len();
        legal += case["legal_ordinary"].as_array().unwrap().len();
    }
    assert_eq!((ids.len(), candidates, legal), (17, 274, 241));
}

#[test]
fn original_corpus_ordinary_legal_moves_are_exactly_the_safe_candidates() {
    let corpus: Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/chess/corpus.json")).unwrap();
    let mut positions = 0;
    for case in corpus["cases"].as_array().unwrap() {
        if case["operation"] != "position" {
            continue;
        }
        let id = case["id"].as_str().unwrap();
        let position = Position::from_fen(case["input"]["fen"].as_str().unwrap()).unwrap();
        // Exclude the corpus's special actions by their board shape: a king moving
        // two files (castling) or a pawn moving diagonally onto an empty square.
        let expected: Vec<String> = strings(&case["expected"]["legal_moves"])
            .into_iter()
            .filter(|uci| {
                let bytes = uci.as_bytes();
                let from = Square::new(bytes[0] - b'a', bytes[1] - b'1').unwrap();
                let to = Square::new(bytes[2] - b'a', bytes[3] - b'1').unwrap();
                let kind = position.piece_at(from).unwrap().kind;
                let castling = kind == PieceKind::King && from.file().abs_diff(to.file()) == 2;
                let en_passant = kind == PieceKind::Pawn
                    && from.file() != to.file()
                    && position.piece_at(to).is_none();
                !castling && !en_passant
            })
            .collect();
        let mut expected = expected;
        expected.sort();
        assert_eq!(assert_contract(id, &position), expected, "{id}");
        positions += 1;
    }
    assert_eq!(positions, 35);
}

#[test]
fn every_promotion_choice_stays_a_distinct_action_through_transition() {
    for fen in [
        "1n2k2r/P6P/8/8/8/8/8/4K3 w k - 0 1",
        "4k3/8/8/8/8/8/6p1/4K2R b K - 0 1",
    ] {
        let mut position = Position::from_fen(fen).unwrap();
        let before = position.clone();
        let promotions: Vec<Action> = position
            .ordinary_candidates()
            .into_iter()
            .filter(|action| action.promotion.is_some())
            .collect();
        assert_eq!(promotions.len(), 8, "{fen}");
        let mut after_states = BTreeSet::new();
        for action in promotions {
            let record = position.apply_ordinary(action).unwrap();
            let placed = position.piece_at(action.to).unwrap().kind;
            let chosen = match action.promotion.unwrap() {
                Promotion::Queen => PieceKind::Queen,
                Promotion::Rook => PieceKind::Rook,
                Promotion::Bishop => PieceKind::Bishop,
                Promotion::Knight => PieceKind::Knight,
            };
            assert_eq!(placed, chosen, "{fen}: {}", text(action));
            assert!(
                after_states.insert(position.to_fen()),
                "{fen}: merged choice"
            );
            position.undo(&record).unwrap();
            assert_eq!(position, before, "{fen}: {}", text(action));
        }
    }
}

#[test]
fn generation_is_deterministic_and_does_not_mutate_the_position() {
    let position =
        Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
            .unwrap();
    let before = position.clone();
    assert_eq!(
        position.ordinary_candidates(),
        position.ordinary_candidates()
    );
    assert_eq!(position, before);
}

#[test]
fn candidates_ignore_counter_bounds_which_the_checked_boundary_enforces() {
    // At the maximum halfmove clock a quiet move cannot increment it, but it is
    // still a candidate; pawn moves reset the clock and remain applicable.
    let position = Position::from_fen("4k3/8/8/8/8/8/4P3/4K1N1 w - - 4294967295 1").unwrap();
    let candidates = position.ordinary_candidates();
    let quiet = Action {
        from: Square::new(6, 0).unwrap(),
        to: Square::new(5, 2).unwrap(),
        promotion: None,
    };
    let pawn = Action {
        from: Square::new(4, 1).unwrap(),
        to: Square::new(4, 3).unwrap(),
        promotion: None,
    };
    assert!(candidates.contains(&quiet) && candidates.contains(&pawn));
    assert_eq!(
        position.clone().apply_ordinary(quiet).unwrap_err(),
        ActionError::CounterOverflow
    );
    assert!(position.clone().apply_ordinary(pawn).is_ok());
    assert_contract("halfmove-max", &position);
}
