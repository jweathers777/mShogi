//! Independently expected perft totals and divides, state preservation,
//! depth semantics and counter-independent traversal.

use mshogi_core::{
    CHESS_POSITION_PROFILE_ID,
    chess::{Action, DivideEntry, PerftError, Position, Promotion},
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

/// The first root entry where two divides disagree, in divide order: a
/// missing/extra action, a different action at that index, or a count.
fn first_disagreement(actual: &[(String, u64)], expected: &[(String, u64)]) -> Option<String> {
    for index in 0..actual.len().max(expected.len()) {
        match (actual.get(index), expected.get(index)) {
            (Some(a), Some(e)) if a == e => continue,
            (Some((a, an)), Some((e, en))) if a == e => {
                return Some(format!("{a}: {an} nodes, expected {en}"));
            }
            (a, e) => return Some(format!("entry {index}: actual {a:?}, expected {e:?}")),
        }
    }
    None
}

fn pairs(entries: &[DivideEntry]) -> Vec<(String, u64)> {
    entries
        .iter()
        .map(|entry| (text(entry.action), entry.nodes))
        .collect()
}

fn expected_pairs(case: &Value) -> Vec<(String, u64)> {
    case["divide"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            (
                entry["action"].as_str().unwrap().to_owned(),
                entry["nodes"].as_u64().unwrap(),
            )
        })
        .collect()
}

fn divide_data() -> Value {
    serde_json::from_str(include_str!("../../../tests/fixtures/chess/divide.json")).unwrap()
}

fn check_divide_case(case: &Value) {
    let id = case["id"].as_str().unwrap();
    assert_eq!(case["rules_profile"], CHESS_POSITION_PROFILE_ID, "{id}");
    let position = Position::from_fen(case["fen"].as_str().unwrap()).unwrap();
    let before = position.clone();
    let depth = case["depth"].as_u64().unwrap() as u32;
    let divide = position.divide(depth).unwrap();
    if let Some(problem) = first_disagreement(&pairs(&divide), &expected_pairs(case)) {
        panic!("{id} depth {depth}: first disagreement at {problem}");
    }
    let total: u64 = divide.iter().map(|entry| entry.nodes).sum();
    assert_eq!(total, case["total"].as_u64().unwrap(), "{id}");
    assert_eq!(
        position.perft(depth).unwrap(),
        total,
        "{id}: perft equals divide sum"
    );
    assert_eq!(position, before, "{id}: diagnostics changed the position");
}

#[test]
fn corpus_perft_totals_match_at_documented_depths() {
    let corpus: Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/chess/corpus.json")).unwrap();
    let mut checked = 0;
    for case in corpus["cases"].as_array().unwrap() {
        let Some(perft) = case["expected"]["perft"].as_object() else {
            continue;
        };
        let id = case["id"].as_str().unwrap();
        let position = Position::from_fen(case["input"]["fen"].as_str().unwrap()).unwrap();
        let before = position.clone();
        for (depth, expected) in perft {
            let depth: u32 = depth.parse().unwrap();
            assert_eq!(
                position.perft(depth).unwrap(),
                expected.as_u64().unwrap(),
                "{id} depth {depth}"
            );
            checked += 1;
        }
        assert_eq!(position, before, "{id}");
    }
    assert_eq!(checked, 38);
}

#[test]
fn independent_divides_match_in_order() {
    let data = divide_data();
    assert_eq!(data["format"], "mshogi.chess.divide.v1");
    assert_eq!(data["schema_version"], 1);
    assert_eq!(data["rules_profile"], CHESS_POSITION_PROFILE_ID);
    let mut count = 0;
    for case in data["cases"].as_array().unwrap() {
        if case["deep"] == false {
            check_divide_case(case);
            count += 1;
        }
    }
    assert_eq!(count, 11);
}

// Several million nodes; run with `cargo test --release -- --ignored`.
#[test]
#[ignore = "deep perft: run in release with --ignored"]
fn deep_independent_divides_match_in_order() {
    let data = divide_data();
    let mut count = 0;
    for case in data["cases"].as_array().unwrap() {
        if case["deep"] == true {
            check_divide_case(case);
            count += 1;
        }
    }
    assert_eq!(count, 3);
}

#[test]
fn the_disagreement_locator_names_the_first_differing_root_action() {
    let data = divide_data();
    let case = &data["cases"][0];
    let expected = expected_pairs(case);
    let mut wrong_count = expected.clone();
    wrong_count[5].1 += 1;
    assert_eq!(
        first_disagreement(&wrong_count, &expected),
        Some(format!(
            "{}: {} nodes, expected {}",
            expected[5].0,
            expected[5].1 + 1,
            expected[5].1
        ))
    );
    let mut missing = expected.clone();
    missing.remove(3);
    let problem = first_disagreement(&missing, &expected).unwrap();
    assert!(problem.starts_with("entry 3:"), "{problem}");
    assert_eq!(first_disagreement(&expected, &expected), None);
}

#[test]
fn depth_semantics_and_errors_are_explicit() {
    let initial =
        Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    assert_eq!(initial.perft(0), Ok(1));
    assert_eq!(initial.divide(0), Err(PerftError::ZeroDivideDepth));
    assert_eq!(
        PerftError::ZeroDivideDepth.to_string(),
        "perft: divide requires depth of at least 1"
    );
    // Mate and stalemate: depth zero is 1; every positive depth is 0.
    for fen in [
        "7k/6Q1/5K2/8/8/8/8/8 b - - 0 1",
        "7k/5Q2/6K1/8/8/8/8/8 b - - 0 1",
    ] {
        let position = Position::from_fen(fen).unwrap();
        assert_eq!(position.perft(0), Ok(1), "{fen}");
        for depth in 1..=4 {
            assert_eq!(position.perft(depth), Ok(0), "{fen}");
        }
        assert_eq!(position.divide(3), Ok(vec![]), "{fen}");
    }
    // Divide entries are complete actions in the documented order, and
    // depth-one divide counts each root action once.
    let divide = initial.divide(1).unwrap();
    assert_eq!(divide.len(), 20);
    assert!(divide.iter().all(|entry| entry.nodes == 1));
    assert_eq!(text(divide[0].action), "b1a3");
    assert_eq!(text(divide[19].action), "h2h4");
    // Malformed positions never reach perft: the FEN boundary rejects them.
    assert!(Position::from_fen("8/8/8/8/8/8/8/8 w - - 0 1").is_err());
}

#[test]
fn counters_never_limit_traversal() {
    let data = divide_data();
    for case in data["cases"].as_array().unwrap() {
        if case["deep"] == true {
            continue;
        }
        let fen = case["fen"].as_str().unwrap();
        let fields: Vec<&str> = fen.split(' ').collect();
        // A raw en passant target requires halfmove zero.
        let halfmove = if fields[3] == "-" { u32::MAX } else { 0 };
        let limit_fen = format!("{} {halfmove} {}", fields[..4].join(" "), u32::MAX);
        let normal = Position::from_fen(fen).unwrap();
        let limit = Position::from_fen(&limit_fen).unwrap();
        let depth = case["depth"].as_u64().unwrap() as u32;
        assert_eq!(limit.divide(depth), normal.divide(depth), "{limit_fen}");
        assert_eq!(limit.perft(depth), normal.perft(depth), "{limit_fen}");
        assert_eq!(limit.to_fen(), limit_fen, "{limit_fen}: caller state");
    }
}
