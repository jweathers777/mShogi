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

const MAX: u32 = u32::MAX;

// Replace the two FEN counter fields; everything else is unchanged.
fn with_counters(fen: &str, halfmove: u32, fullmove: u32) -> String {
    let fields: Vec<&str> = fen.split(' ').collect();
    format!("{} {halfmove} {fullmove}", fields[..4].join(" "))
}

fn every_fixture_fen() -> Vec<String> {
    let mut fens = Vec::new();
    let corpus: Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/chess/corpus.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        if case["operation"] == "position" {
            fens.push(case["input"]["fen"].as_str().unwrap().to_owned());
        }
    }
    for (file, list, key) in [
        (
            include_str!("../../../tests/fixtures/chess/classification.json"),
            "cases",
            "fen",
        ),
        (
            include_str!("../../../tests/fixtures/chess/special-actions.json"),
            "positions",
            "fen",
        ),
        (
            include_str!("../../../tests/fixtures/chess/ordinary-candidates.json"),
            "cases",
            "fen",
        ),
    ] {
        let data: Value = serde_json::from_str(file).unwrap();
        for case in data[list].as_array().unwrap() {
            fens.push(case[key].as_str().unwrap().to_owned());
        }
    }
    fens
}

#[test]
fn legal_sets_and_classification_are_identical_across_counter_values() {
    let (mut variants, mut overflowed) = (0, 0);
    for fen in every_fixture_fen() {
        let base = Position::from_fen(&fen).unwrap();
        let mut expected: Vec<String> = base.legal_actions().into_iter().map(text).collect();
        expected.sort();
        // A raw EP target requires halfmove zero, so only fullmove varies there.
        let mut counters = vec![(0, MAX)];
        if base.en_passant_target().is_none() {
            counters.extend([(17, 1), (MAX, 1), (MAX, MAX)]);
        }
        for (halfmove, fullmove) in counters {
            let variant_fen = with_counters(&fen, halfmove, fullmove);
            let variant = Position::from_fen(&variant_fen).unwrap();
            let legal = variant.legal_actions();
            let mut texts: Vec<String> = legal.iter().copied().map(text).collect();
            texts.sort();
            assert_eq!(texts, expected, "{variant_fen}: legal set");
            assert_eq!(variant.classify(), base.classify(), "{variant_fen}");
            assert_eq!(variant.in_check(), base.in_check(), "{variant_fen}");
            for action in legal {
                // Independently predict whether the resulting counters fit.
                let mover = variant.piece_at(action.from).unwrap();
                let resets = mover.kind == mshogi_core::chess::PieceKind::Pawn
                    || variant.piece_at(action.to).is_some();
                let overflows = (!resets && halfmove == MAX)
                    || (variant.side_to_move() == mshogi_core::chess::Color::Black
                        && fullmove == MAX);
                let mut trial = variant.clone();
                match trial.apply(action) {
                    Ok(record) => {
                        assert!(!overflows, "{variant_fen}: {} applied", text(action));
                        trial.undo(&record).unwrap();
                        assert_eq!(trial, variant);
                    }
                    Err(error) => {
                        assert!(overflows, "{variant_fen}: {} {error:?}", text(action));
                        assert_eq!(error, ActionError::CounterOverflow);
                        assert_eq!(trial, variant, "{variant_fen}: overflow mutated");
                        assert_eq!(trial.to_fen(), variant_fen);
                        overflowed += 1;
                    }
                }
            }
            variants += 1;
        }
    }
    assert_eq!((variants, overflowed), (276, 1659));
}

#[test]
fn counter_overflow_is_atomic_for_each_counter_side_and_action_kind() {
    // (FEN, action, applies): every rejection must be CounterOverflow and leave
    // board, side, rights, raw target and counters unchanged (there is no
    // derived state). Pawn moves and captures reset the halfmove clock.
    for (fen, action_text, applies) in [
        // Halfmove limit, White: quiet and castling fail; pawn and capture reset.
        (
            "r3k2r/8/8/8/8/8/P7/R3K2R w KQkq - 4294967295 7",
            "e1f1",
            false,
        ),
        (
            "r3k2r/8/8/8/8/8/P7/R3K2R w KQkq - 4294967295 7",
            "e1g1",
            false,
        ),
        (
            "r3k2r/8/8/8/8/8/P7/R3K2R w KQkq - 4294967295 7",
            "a2a4",
            true,
        ),
        (
            "r3k2r/8/8/8/8/8/P7/R3K2R w KQkq - 4294967295 7",
            "h1h8",
            true,
        ),
        // Halfmove limit, Black: quiet and castling fail; pawn resets.
        (
            "r3k2r/p7/8/8/8/8/8/R3K2R b KQkq - 4294967295 7",
            "e8d8",
            false,
        ),
        (
            "r3k2r/p7/8/8/8/8/8/R3K2R b KQkq - 4294967295 7",
            "e8c8",
            false,
        ),
        (
            "r3k2r/p7/8/8/8/8/8/R3K2R b KQkq - 4294967295 7",
            "a7a5",
            true,
        ),
        // Fullmove limit: White never increments it; every Black action does.
        (
            "r3k2r/p7/8/8/8/8/8/R3K2R w KQkq - 3 4294967295",
            "e1f1",
            true,
        ),
        (
            "r3k2r/p7/8/8/8/8/8/R3K2R w KQkq - 3 4294967295",
            "e1c1",
            true,
        ),
        (
            "r3k2r/p7/8/8/8/8/8/R3K2R b KQkq - 3 4294967295",
            "a7a5",
            false,
        ),
        (
            "r3k2r/p7/8/8/8/8/8/R3K2R b KQkq - 3 4294967295",
            "h8h1",
            false,
        ),
        (
            "r3k2r/p7/8/8/8/8/8/R3K2R b KQkq - 3 4294967295",
            "e8g8",
            false,
        ),
        // En passant keeps halfmove zero by FEN rule; Black's fails at fullmove.
        ("7k/8/8/8/3Pp3/8/8/K7 b - d3 0 4294967295", "e4d3", false),
        ("7k/8/8/3pP3/8/8/8/K7 w - d6 0 4294967295", "e5d6", true),
    ] {
        let mut position = Position::from_fen(fen).unwrap();
        let before = position.clone();
        let action = position
            .legal_actions()
            .into_iter()
            .find(|candidate| text(*candidate) == action_text)
            .unwrap_or_else(|| panic!("{fen}: {action_text} must be legal"));
        let result = position.apply(action);
        if applies {
            assert!(result.is_ok(), "{fen}: {action_text}");
        } else {
            assert_eq!(result.unwrap_err(), ActionError::CounterOverflow, "{fen}");
            assert_eq!(position, before, "{fen}: {action_text}");
            assert_eq!(position.to_fen(), fen, "{fen}: {action_text}");
        }
    }
}

#[test]
fn overflow_never_turns_a_live_position_into_mate_or_stalemate() {
    // Every Black reply would overflow the fullmove number, and every reply
    // to the check is a king move; the position is still `check`, not mate.
    let position = Position::from_fen("4k3/8/8/8/8/8/8/4RK2 b - - 0 4294967295").unwrap();
    let legal = position.legal_actions();
    assert_eq!(legal.len(), 4);
    assert_eq!(position.classify(), Classification::Check);
    for action in legal {
        assert_eq!(
            position.clone().apply(action).unwrap_err(),
            ActionError::CounterOverflow
        );
    }
    // Only quiet king moves exist at the halfmove limit: `quiet`, not stalemate.
    let position = Position::from_fen("7k/8/8/8/8/8/8/K7 b - - 4294967295 9").unwrap();
    assert_eq!(position.legal_actions().len(), 3);
    assert_eq!(position.classify(), Classification::Quiet);
}
