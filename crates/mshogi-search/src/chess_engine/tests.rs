use super::*;
use crate::chess_bitboard::tests::TestRng;
use mshogi_core::chess::{Action, ClaimPolicy, Color, DrawReason};

fn game(fen: &str) -> AdjudicatedGame {
    AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap()
}
fn depth(depth: u16) -> SearchLimits {
    SearchLimits { depth, nodes: None }
}
fn run(engine: &mut ChessEngine, game: &AdjudicatedGame, limits: SearchLimits) -> SearchResult {
    engine
        .search(game, limits, SearchOptions::default(), || false)
        .unwrap()
        .result
}
fn chosen(result: &SearchResult) -> Action {
    match result.decision {
        Some(SearchDecision::Choose(SearchChoice::Move(action))) => action,
        other => panic!("expected a move, got {other:?}"),
    }
}

#[test]
fn finds_forced_mates_with_exact_distance() {
    // Known problems: mate in one, two and three for the side to move.
    for (fen, plies, first) in [
        ("6k1/5ppp/8/8/8/8/8/R3K3 w Q - 0 1", 1, Some("a1a8")),
        (
            // Legall-type mate: Nf6+ gxf6, Bxf7#.
            "r2qkb1r/pp2nppp/3p4/2pNN1B1/2BnP3/3P4/PPP2PPP/R2QK2R w KQkq - 1 1",
            3,
            Some("d5f6"),
        ),
        ("r5rk/5p1p/5R2/4B3/8/8/7P/7K w - - 0 1", 5, Some("f6a6")),
    ] {
        let g = game(fen);
        let mut engine = ChessEngine::new(8);
        let result = run(&mut engine, &g, depth(8));
        assert_eq!(result.score, Some(MATE_SCORE - plies), "{fen}");
        if let Some(first) = first {
            assert_eq!(chosen(&result), first.parse().unwrap(), "{fen}");
        }
        // The principal variation is a legal native line that ends in mate.
        let mut replay = g.clone();
        for choice in &result.principal_variation {
            match choice {
                SearchChoice::Move(action) => replay.apply(*action).unwrap(),
                SearchChoice::Claim(_) => panic!("claim in a mating line"),
            }
        }
        assert_eq!(result.principal_variation.len(), plies as usize, "{fen}");
        assert!(
            matches!(replay.outcome(), Some(ChessOutcome::Checkmate { winner }) if winner == g.position().side_to_move()),
            "{fen}"
        );
    }
}

#[test]
fn concluded_roots_return_the_native_outcome() {
    let mut engine = ChessEngine::new(1);
    let mated = game("R3k3/8/4K3/8/8/8/8/8 b - - 0 1");
    let result = run(&mut engine, &mated, depth(4));
    assert_eq!(
        result.decision,
        Some(SearchDecision::Finished(ChessOutcome::Checkmate {
            winner: Color::White
        }))
    );
    assert_eq!(result.score, Some(-MATE_SCORE));
    let stalemate = game("7k/5Q2/6K1/8/8/8/8/8 b - - 0 1");
    let result = run(&mut engine, &stalemate, depth(4));
    assert_eq!(
        result.decision,
        Some(SearchDecision::Finished(ChessOutcome::Draw(
            DrawReason::Stalemate
        )))
    );
    assert_eq!(result.score, Some(0));
}

#[test]
fn does_not_stalemate_a_bare_king_and_does_not_hang_the_queen() {
    let mut engine = ChessEngine::new(8);
    // Qf7 stalemates; Qf8 mates.
    let g = game("7k/8/6K1/8/8/8/8/5Q2 w - - 0 1");
    let result = run(&mut engine, &g, depth(6));
    assert!(
        result.score.unwrap() >= MATE_SCORE - 3,
        "{:?}",
        result.score
    );
    let mut after = g.clone();
    after.apply(chosen(&result)).unwrap();
    assert!(!matches!(
        after.outcome(),
        Some(ChessOutcome::Draw(DrawReason::Stalemate))
    ));
}

/// Shuffle rooks so that the root position has occurred three times.
fn threefold(fen: &str, policy: ClaimPolicy, cycle: [&str; 4]) -> AdjudicatedGame {
    let mut g = AdjudicatedGame::from_fen(fen, policy).unwrap();
    for _ in 0..2 {
        for text in cycle {
            g.apply(text.parse().unwrap()).unwrap();
        }
    }
    assert!(!g.available_claims().is_empty());
    assert_eq!(g.outcome(), None);
    g
}

#[test]
fn draw_claims_follow_policy_score_and_consumer_ability() {
    let cycle = ["b1a1", "e8d8", "a1b1", "d8e8"];
    // White is a queen down: an optional claim is better than each move.
    let worse = "4r1k1/5ppp/8/8/8/8/3q1PPP/1R4K1 w - - 0 1";
    // White is a queen up: play on.
    let better = "4r1k1/5ppp/8/8/8/8/3Q1PPP/1R4K1 w - - 0 1";
    let mut engine = ChessEngine::new(8);

    let g = threefold(worse, ClaimPolicy::ExplicitOnly, cycle);
    let result = run(&mut engine, &g, depth(5));
    assert!(matches!(
        result.decision,
        Some(SearchDecision::Choose(SearchChoice::Claim(_)))
    ));
    assert_eq!(result.score, Some(0));
    // The consumer cannot claim: the engine must give a move.
    let result = engine
        .search(
            &g,
            depth(5),
            SearchOptions {
                allow_draw_claims: false,
                time: None,
            },
            || false,
        )
        .unwrap()
        .result;
    let action = chosen(&result);
    assert!(g.position().legal_actions().contains(&action));

    let g = threefold(better, ClaimPolicy::ExplicitOnly, cycle);
    let result = run(&mut engine, &g, depth(5));
    assert!(result.score.unwrap() > 500);
    chosen(&result);

    // AutoClaim: an available claim is taken at once, even when ahead.
    let g = threefold(better, ClaimPolicy::AutoClaim, cycle);
    let result = run(&mut engine, &g, depth(5));
    assert!(matches!(
        result.decision,
        Some(SearchDecision::Choose(SearchChoice::Claim(_)))
    ));
}

#[test]
fn the_side_ahead_avoids_a_repetition_and_the_side_behind_takes_it() {
    // Black is a queen up and to move. ...Rd8-e8 would make a position that
    // has occurred two times in the game.
    let mut g = game("4r1k1/5ppp/8/8/8/8/3q1PPP/1R4K1 w - - 0 1");
    for text in ["b1a1", "e8d8", "a1b1", "d8e8", "b1a1", "e8d8", "a1b1"] {
        g.apply(text.parse().unwrap()).unwrap();
    }
    let mut engine = ChessEngine::new(8);
    let result = run(&mut engine, &g, depth(6));
    assert!(result.score.unwrap() > 500);
    assert_ne!(chosen(&result), "d8e8".parse().unwrap());
}

#[test]
fn limits_validation_and_table_reuse() {
    let g = game("r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 3 3");
    let mut engine = ChessEngine::new(8);
    assert_eq!(
        engine
            .search(&g, depth(0), SearchOptions::default(), || false)
            .unwrap_err(),
        SearchError::InvalidDepth(0)
    );
    let none = run(
        &mut engine,
        &g,
        SearchLimits {
            depth: 5,
            nodes: Some(0),
        },
    );
    assert_eq!(
        (none.decision, none.status),
        (None, SearchStatus::NodeLimit)
    );
    let stopped = engine
        .search(&g, depth(5), SearchOptions::default(), || true)
        .unwrap()
        .result;
    assert_eq!(
        (stopped.decision, stopped.status),
        (None, SearchStatus::Stopped)
    );

    let limited = run(
        &mut engine,
        &g,
        SearchLimits {
            depth: 64,
            nodes: Some(20_000),
        },
    );
    assert_eq!(limited.status, SearchStatus::NodeLimit);
    assert_eq!(limited.nodes, 20_000);
    chosen(&limited);

    // A fresh engine gives the same result for the same work: no hidden state.
    let first = run(&mut ChessEngine::new(8), &g, depth(7));
    let again = run(&mut ChessEngine::new(8), &g, depth(7));
    assert_eq!(first, again);
    assert_eq!(first.status, SearchStatus::Complete);
    assert_eq!(first.completed_depth, 7);

    // The table lasts between searches of one engine, and a new game clears it.
    let mut engine = ChessEngine::new(8);
    let cold = run(&mut engine, &g, depth(7));
    let warm = run(&mut engine, &g, depth(7));
    assert!(warm.nodes < cold.nodes / 2, "{} {}", warm.nodes, cold.nodes);
    engine.new_game();
    assert_eq!(run(&mut engine, &g, depth(7)).nodes, cold.nodes);
    engine.set_table_megabytes(3);
    assert_eq!(engine.table_megabytes(), 3);
}

#[test]
fn each_returned_move_is_a_native_legal_action() {
    let mut rng = TestRng(0xC0FF_EE00_1234_0001);
    let mut engine = ChessEngine::new(8);
    let mut searched = 0;
    for _ in 0..12 {
        let mut g = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        for ply in 0..80 {
            if g.outcome().is_some() {
                break;
            }
            let legal = g.position().legal_actions();
            let action = if ply % 4 == 3 {
                let result = engine
                    .search(
                        &g,
                        SearchLimits {
                            depth: 4,
                            nodes: Some(4_000),
                        },
                        SearchOptions {
                            allow_draw_claims: false,
                            time: None,
                        },
                        || false,
                    )
                    .unwrap()
                    .result;
                searched += 1;
                let action = chosen(&result);
                assert!(legal.contains(&action));
                action
            } else {
                legal[(rng.next() % legal.len() as u64) as usize]
            };
            g.apply(action).unwrap();
        }
    }
    assert!(searched > 100);
}
