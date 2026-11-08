//! Hand-derived profile expectations plus state/record/undo properties.
use mshogi_core::chess::{
    Action, ActionError, ClaimPolicy, Game, GameRecord, HistoryOrigin, Position, RecordError,
    STANDARD_START_FEN,
};
use serde_json::Value;

fn action(text: &str) -> Action {
    text.parse().unwrap()
}
fn apply(game: &mut Game, texts: &[&str]) {
    for text in texts {
        game.apply(action(text)).unwrap();
    }
}
fn profile() -> Value {
    serde_json::from_str(include_str!(
        "../../../tests/fixtures/chess/match-profile.json"
    ))
    .unwrap()
}

#[test]
fn independent_history_counts_and_identical_fen_different_histories() {
    let data = profile();
    for c in data["cases"].as_array().unwrap() {
        let Some(expected) = c["expected"]["occurrences"].as_u64() else {
            continue;
        };
        let mut game =
            Game::from_fen(c["fen"].as_str().unwrap(), ClaimPolicy::ExplicitOnly).unwrap();
        for a in c["moves"].as_array().unwrap() {
            game.apply(action(a.as_str().unwrap())).unwrap();
        }
        assert_eq!(game.current_occurrences() as u64, expected, "{}", c["id"]);
    }
    let mut game = Game::new(ClaimPolicy::AutoClaim);
    let start = game.clone();
    for _ in 0..2 {
        apply(&mut game, &["g1f3", "g8f6", "f3g1", "f6g8"]);
    }
    let imported = Game::from_fen(&game.position().to_fen(), ClaimPolicy::AutoClaim).unwrap();
    assert_eq!(game.position(), imported.position());
    assert_eq!(game.repetition_identity(), imported.repetition_identity());
    assert_eq!(game.current_occurrences(), 3);
    assert_eq!(imported.current_occurrences(), 1);
    assert_ne!(game, imported);
    assert_eq!(game.history_origin(), HistoryOrigin::StandardStart);
    assert_eq!(imported.history_origin(), HistoryOrigin::ImportedFen);
    assert_eq!(imported.position().halfmove_clock(), 8);
    assert_eq!(imported.position().fullmove_number(), 5);
    assert_eq!(game.record().replay_history().unwrap().game, game);
    while game.undo().is_some() {}
    assert_eq!(game, start);
    let unchanged = game.clone();
    assert_eq!(game.undo(), None);
    assert_eq!(game, unchanged);
    assert_eq!(
        Game::from_fen(STANDARD_START_FEN, ClaimPolicy::ExplicitOnly)
            .unwrap()
            .history_origin(),
        HistoryOrigin::ImportedFen
    );
}

#[test]
fn independently_specified_repetition_equivalences() {
    let data = profile();
    for c in data["cases"].as_array().unwrap() {
        for (key, equal) in [
            ("same_repetition_as", true),
            ("different_repetition_from", false),
        ] {
            if let Some(other) = c["expected"][key].as_str() {
                let p = Position::from_fen(c["fen"].as_str().unwrap()).unwrap();
                let q = Position::from_fen(other).unwrap();
                assert_eq!(
                    p.repetition_identity() == q.repetition_identity(),
                    equal,
                    "{}",
                    c["id"]
                );
            }
        }
    }
    // Raw EP with no adjacent pawn and counters do not change equivalence.
    for (a, b) in [
        (
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 17 20",
        ),
        (
            "7k/8/8/5pP1/8/8/8/K7 w - f6 0 1",
            "7k/8/8/5pP1/8/8/8/K7 w - f6 0 4294967295",
        ),
    ] {
        let p = Position::from_fen(a).unwrap();
        let q = Position::from_fen(b).unwrap();
        assert_ne!(p, q);
        assert_eq!(p.repetition_identity(), q.repetition_identity());
    }
    let a = Position::from_fen("7k/8/8/8/8/8/8/K7 w - - 0 1").unwrap();
    let b = Position::from_fen("7k/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
    assert_ne!(a.repetition_identity(), b.repetition_identity());
}

#[test]
fn rejection_and_branch_undo_restore_all_history_fields() {
    let mut game = Game::new(ClaimPolicy::ExplicitOnly);
    apply(&mut game, &["g1f3", "g8f6", "f3g1", "f6g8"]);
    let before = game.clone();
    assert!(game.apply(action("e2e5")).is_err());
    assert_eq!(game, before);
    game.apply(action("e2e4")).unwrap();
    let abandoned = game.repetition_identity().clone();
    assert_eq!(game.undo(), Some(action("e2e4")));
    assert_eq!(game, before);
    assert_eq!(game.occurrences(&abandoned), 0);
    game.apply(action("d2d4")).unwrap();
    assert_eq!(game.occurrences(&abandoned), 0);
    game.undo();
    assert_eq!(game, before);
    let mut overflow = Game::from_fen(
        "7k/8/8/8/8/8/8/KR6 w - - 4294967295 1",
        ClaimPolicy::AutoClaim,
    )
    .unwrap();
    let before = overflow.clone();
    assert_eq!(
        overflow.apply(action("b1b2")),
        Err(ActionError::CounterOverflow)
    );
    assert_eq!(overflow, before);
    let mut record = before.record();
    record.push_event(mshogi_core::chess::RecordEvent {
        ply: 0,
        kind: mshogi_core::chess::RecordEventKind::Move(action("b1b2")),
    });
    assert!(matches!(
        record.replay_history(),
        Err(RecordError::Action(1, ActionError::CounterOverflow))
    ));
}

#[test]
fn independent_special_transition_fixtures_restore_game_and_records() {
    let data: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/chess/special-actions.json"
    ))
    .unwrap();
    for c in data["sequences"].as_array().unwrap() {
        let mut game =
            Game::from_fen(c["before_fen"].as_str().unwrap(), ClaimPolicy::ExplicitOnly).unwrap();
        let mut snapshots = Vec::new();
        for step in c["steps"].as_array().unwrap() {
            snapshots.push(game.clone());
            game.apply(action(step["action"].as_str().unwrap()))
                .unwrap();
            assert_eq!(
                game.position().to_fen(),
                step["after_fen"].as_str().unwrap(),
                "{}",
                c["id"]
            );
            assert_eq!(game.record().replay_history().unwrap().game, game);
        }
        for before in snapshots.into_iter().rev() {
            game.undo().unwrap();
            assert_eq!(game, before);
        }
    }
}

#[test]
fn fixed_seed_games_restore_positions_history_and_occurrence_tables() {
    let data: Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/chess/corpus.json")).unwrap();
    let mut seed = 0x6d53_686f_6769_0016u64;
    let mut total = 0;
    for c in data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["operation"] == "position")
    {
        let mut game =
            Game::from_fen(c["input"]["fen"].as_str().unwrap(), ClaimPolicy::AutoClaim).unwrap();
        let mut snapshots = Vec::new();
        let mut identities = vec![game.repetition_identity().clone()];
        for ply in 0..32 {
            let mut legal = game.position().legal_actions();
            legal.sort_by_key(|a| a.to_string());
            if legal.is_empty() {
                break;
            }
            seed = seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z = seed;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            z ^= z >> 31;
            snapshots.push(game.clone());
            game.apply(legal[(z % legal.len() as u64) as usize])
                .unwrap();
            identities.push(game.repetition_identity().clone());
            assert_eq!(
                game.current_occurrences(),
                identities
                    .iter()
                    .filter(|k| *k == game.repetition_identity())
                    .count()
            );
            assert_eq!(game.ply_count(), ply + 1);
            if ply % 5 == 0 {
                let json = game.record().to_json();
                let record = GameRecord::from_json(&json).unwrap();
                assert_eq!(record.to_json(), json);
                assert_eq!(record.replay_history().unwrap().game, game);
            }
            total += 1;
        }
        for before in snapshots.into_iter().rev() {
            game.undo().unwrap();
            assert_eq!(game, before);
        }
    }
    assert!(total > 700, "unexpected loss of sequence coverage: {total}");
    println!("MSH-16 fixed-seed plies: {total}");
}

#[test]
fn checked_game_transition_matches_full_position_errors_effects_and_atomicity() {
    use mshogi_core::chess::{Action, Promotion, Square};
    for fen in [
        STANDARD_START_FEN,
        "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
        "4kr2/8/8/8/8/8/8/4K2R w K - 0 1",        // transit attack
        "4k3/8/8/r4pPK/8/8/8/8 w - f6 0 9",       // EP pin
        "4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 9",      // legal EP
        "1r5k/2P5/8/8/8/8/8/K7 w - - 0 1",        // all promotions
        "7k/8/8/8/8/8/8/KR6 b - - 99 4294967295", // fullmove overflow
        "7k/8/8/8/8/8/8/KR6 w - - 4294967295 1",  // halfmove overflow
    ] {
        let original = Game::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap();
        for from in 0..64 {
            for to in 0..64 {
                for promotion in [
                    None,
                    Some(Promotion::Queen),
                    Some(Promotion::Rook),
                    Some(Promotion::Bishop),
                    Some(Promotion::Knight),
                ] {
                    let a = Action {
                        from: Square::new(from % 8, from / 8).unwrap(),
                        to: Square::new(to % 8, to / 8).unwrap(),
                        promotion,
                    };
                    let mut position = original.position().clone();
                    let expected = position.apply(a).map(|_| ());
                    let mut actual = original.clone();
                    assert_eq!(actual.apply(a), expected, "{fen}: {a:?}");
                    if expected.is_ok() {
                        assert_eq!(actual.position(), &position);
                        assert_eq!(
                            position,
                            Position::from_fen(&position.to_fen()).unwrap(),
                            "derived occupancy rebuild at {a:?}"
                        );
                        assert_eq!(actual.undo(), Some(a));
                    }
                    assert_eq!(actual, original, "exact rejection/undo at {a:?}");
                }
            }
        }
    }
}

#[test]
fn repetition_hash_collisions_retain_exact_identities_and_equivalent_ep_counts() {
    use std::{
        collections::HashMap,
        hash::{BuildHasherDefault, Hasher},
    };
    #[derive(Default)]
    struct Collision;
    impl Hasher for Collision {
        fn write(&mut self, _: &[u8]) {}
        fn finish(&self) -> u64 {
            0
        }
    }
    let mut counts = HashMap::<_, usize, BuildHasherDefault<Collision>>::default();
    for fen in [
        STANDARD_START_FEN,
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 8 2",
        "4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 9",
        "4k3/8/8/3pP3/8/8/8/4K3 w - - 0 9",
    ] {
        *counts
            .entry(Position::from_fen(fen).unwrap().repetition_identity())
            .or_default() += 1;
    }
    assert_eq!(
        counts.len(),
        4,
        "uncapturable raw EP alone is equivalent; legal EP is not"
    );
    assert_eq!(counts.values().copied().sum::<usize>(), 5);
    assert_eq!(counts.values().filter(|&&count| count == 2).count(), 1);
}
