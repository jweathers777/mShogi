//! Independently expected geometric attack queries for both colors.

use mshogi_core::{
    CHESS_POSITION_PROFILE_ID,
    chess::{Action, Color, Position, Square},
};
use serde_json::Value;

// Test-data adapter only; no production square notation is established here.
fn square(name: &str) -> Square {
    let bytes = name.as_bytes();
    assert_eq!(bytes.len(), 2, "{name}");
    Square::new(bytes[0] - b'a', bytes[1] - b'1').unwrap()
}

fn color(name: &Value) -> Color {
    match name.as_str().unwrap() {
        "white" => Color::White,
        "black" => Color::Black,
        other => panic!("unknown color {other}"),
    }
}

fn names(squares: impl Iterator<Item = Square>) -> Vec<String> {
    squares.map(|square| square.to_string()).collect()
}

fn all_squares() -> impl Iterator<Item = Square> {
    (0..8).flat_map(|rank| (0..8).map(move |file| Square::new(file, rank).unwrap()))
}

#[test]
fn independent_attack_fixtures_match_exact_attacker_sets_and_complete_maps() {
    let data: Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/chess/attacks.json")).unwrap();
    assert_eq!(data["format"], "mshogi.chess.attacks.v1");
    assert_eq!(data["schema_version"], 1);
    assert_eq!(data["rules_profile"], CHESS_POSITION_PROFILE_ID);
    let mut ids = std::collections::HashSet::new();
    let (mut queries, mut map_queries) = (0, 0);
    for case in data["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        assert!(ids.insert(id), "{id}: duplicate ID");
        assert_eq!(case["rules_profile"], CHESS_POSITION_PROFILE_ID, "{id}");
        let position = Position::from_fen(case["fen"].as_str().unwrap())
            .unwrap_or_else(|error| panic!("{id}: {error}"));
        let before = position.clone();
        for query in case["queries"].as_array().unwrap() {
            let (by, target) = (
                color(&query["by"]),
                square(query["square"].as_str().unwrap()),
            );
            let expected: Vec<&str> = query["attackers"]
                .as_array()
                .unwrap()
                .iter()
                .map(|name| name.as_str().unwrap())
                .collect();
            assert_eq!(
                names(position.attackers(target, by)),
                expected,
                "{id}: {by:?} attackers of {target}"
            );
            assert_eq!(
                position.is_square_attacked(target, by),
                !expected.is_empty(),
                "{id}: {by:?} attacks {target}"
            );
            queries += 1;
        }
        for (name, by) in [("white", Color::White), ("black", Color::Black)] {
            let expected: Vec<&str> = case["attacked_squares"][name]
                .as_array()
                .unwrap()
                .iter()
                .map(|name| name.as_str().unwrap())
                .collect();
            let actual: Vec<Square> = all_squares()
                .filter(|&target| position.is_square_attacked(target, by))
                .collect();
            assert_eq!(names(actual.into_iter()), expected, "{id}: {name} map");
            for target in all_squares() {
                assert_eq!(
                    position.attackers(target, by).next().is_some(),
                    position.is_square_attacked(target, by),
                    "{id}: {name} {target} query consistency"
                );
                map_queries += 1;
            }
        }
        assert_eq!(position, before, "{id}: queries mutated the position");
    }
    assert_eq!((ids.len(), queries, map_queries), (29, 155, 3712));
}

#[test]
fn original_corpus_attack_queries_are_satisfied() {
    let corpus: Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/chess/corpus.json")).unwrap();
    let mut queries = 0;
    for case in corpus["cases"].as_array().unwrap() {
        let Some(attacks) = case["expected"]["attacks"].as_array() else {
            continue;
        };
        let id = case["id"].as_str().unwrap();
        let position = Position::from_fen(case["input"]["fen"].as_str().unwrap()).unwrap();
        for query in attacks {
            let target = square(query["square"].as_str().unwrap());
            assert_eq!(
                Value::Bool(position.is_square_attacked(target, color(&query["by"]))),
                query["attacked"],
                "{id}: {target}"
            );
            queries += 1;
        }
    }
    assert_eq!(queries, 3);
}

#[test]
fn attacks_ignore_whether_moving_the_attacker_would_expose_its_king() {
    // The e7 knight is pinned to e8 by the e1 rook; it still attacks c6/d5,
    // and the white king may not step onto those squares.
    let mut position = Position::from_fen("4k3/4n3/8/2K5/8/8/8/4R3 w - - 0 1").unwrap();
    assert_eq!(
        names(position.attackers(square("d5"), Color::Black)),
        ["e7"]
    );
    assert!(position.is_square_attacked(square("c6"), Color::Black));
    let king_step = Action {
        from: square("c5"),
        to: square("d5"),
        promotion: None,
    };
    assert!(position.apply_ordinary(king_step).is_err());

    // The c3 pawn is pinned to c8 by the c1 rook, yet it still checks d2.
    let position = Position::from_fen("2k5/8/8/8/8/2p5/3K4/2R5 w - - 0 1").unwrap();
    assert_eq!(
        names(position.attackers(square("d2"), Color::Black)),
        ["c3"]
    );
}

#[test]
fn attack_maps_follow_occupancy_changes_from_checked_actions() {
    // Interposing on the checking line removes the attack on the king and the
    // squares beyond the blocker, while the blocker itself becomes attacked.
    let mut position = Position::from_fen("4r1k1/8/8/8/8/8/3N4/4K3 w - - 0 1").unwrap();
    assert_eq!(
        names(position.attackers(square("e1"), Color::Black)),
        ["e8"]
    );
    position
        .apply_ordinary(Action {
            from: square("d2"),
            to: square("e4"),
            promotion: None,
        })
        .unwrap();
    assert!(!position.is_square_attacked(square("e1"), Color::Black));
    assert!(!position.is_square_attacked(square("e3"), Color::Black));
    assert_eq!(
        names(position.attackers(square("e4"), Color::Black)),
        ["e8"]
    );

    // Vacating a square opens the slider's ray beyond it.
    let mut position = Position::from_fen("4k3/8/8/8/8/8/R2N3p/4K3 w - - 0 1").unwrap();
    assert!(!position.is_square_attacked(square("h2"), Color::White));
    position
        .apply_ordinary(Action {
            from: square("d2"),
            to: square("b3"),
            promotion: None,
        })
        .unwrap();
    assert_eq!(
        names(position.attackers(square("h2"), Color::White)),
        ["a2"]
    );
}
