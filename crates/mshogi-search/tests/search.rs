//! Hand-solvable chess oracles. These assert chess facts, not perft counts.
use mshogi_core::chess::*;
use mshogi_search::search::*;

fn game(fen: &str, policy: ClaimPolicy) -> AdjudicatedGame {
    AdjudicatedGame::from_fen(fen, policy).unwrap()
}
fn run(game: &AdjudicatedGame, depth: u16) -> SearchResult {
    let before = game.clone();
    let result = search(game, SearchLimits { depth, nodes: None }).unwrap();
    assert_eq!(*game, before, "caller position, history, policy and result");
    assert_eq!(result.status, SearchStatus::Complete);
    // Independently replay every returned complete choice against core legality.
    let mut replay = game.clone();
    for choice in &result.principal_variation {
        match choice {
            SearchChoice::Move(action) => {
                assert!(replay.position().legal_actions().contains(action));
                replay.apply(*action).unwrap();
            }
            SearchChoice::Claim(claim) => replay.claim(*claim).unwrap(),
        }
    }
    result
}
fn chosen_move(result: &SearchResult) -> Action {
    let Some(SearchDecision::Choose(SearchChoice::Move(action))) = result.decision else {
        panic!("expected move: {result:?}")
    };
    action
}
#[test]
fn mate_in_one_for_both_colors_and_already_mated_root() {
    for (fen, expected) in [
        ("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", "g6g7"),
        ("8/8/8/8/8/5kq1/8/7K b - - 0 1", "g3g2"),
    ] {
        let g = game(fen, ClaimPolicy::ExplicitOnly);
        let result = run(&g, 1);
        assert_eq!(chosen_move(&result).to_string(), expected);
        assert_eq!(result.score, Some(MATE_SCORE - 1));
        let mut mate = g.clone();
        mate.apply(chosen_move(&result)).unwrap();
        let result = run(&mate, 4);
        assert_eq!(result.score, Some(-MATE_SCORE));
        assert_eq!(
            result.decision,
            Some(SearchDecision::Finished(ChessOutcome::Checkmate {
                winner: g.position().side_to_move(),
            }))
        );
        assert_eq!(result.nodes, 1);
    }
}
#[test]
fn forced_mate_in_two_becomes_visible_only_at_three_plies() {
    // Kf6/Qe6 vs Kh8: no immediate mate. Qe7 confines Kh8 to g8,
    // and Qg7 mates. Other equivalent three-ply mates are acceptable.
    let g = game("7k/8/4QK2/8/8/8/8/8 w - - 0 1", ClaimPolicy::ExplicitOnly);
    assert_eq!(run(&g, 1).score, Some(900));
    let result = run(&g, 3);
    assert_eq!(result.score, Some(MATE_SCORE - 3));
    assert_eq!(result.principal_variation.len(), 3);
}
#[test]
fn material_capture_perspective_and_deterministic_ties() {
    // White's rook can take an unprotected queen. Black's king is remote.
    let g = game("7k/8/8/8/8/8/q7/R6K w - - 0 1", ClaimPolicy::ExplicitOnly);
    let result = run(&g, 1);
    assert_eq!(chosen_move(&result).to_string(), "a1a2");
    assert_eq!(result.score, Some(500));
    assert_eq!(run(&g, 1), result);
    let mut after = g;
    after.apply(chosen_move(&result)).unwrap();
    assert_eq!(run(&after, 1).score, Some(-500));
}
#[test]
fn rules_draws_are_terminal_before_static_evaluation() {
    for (fen, reason) in [
        ("7k/5K2/6Q1/8/8/8/8/8 b - - 0 1", DrawReason::Stalemate),
        ("7k/8/8/8/8/8/8/K7 w - - 0 1", DrawReason::DeadPosition),
        (
            "7k/8/8/8/8/8/8/KR6 w - - 150 1",
            DrawReason::SeventyFiveMove,
        ),
    ] {
        let result = run(&game(fen, ClaimPolicy::ExplicitOnly), 2);
        assert_eq!(result.score, Some(0));
        assert_eq!(
            result.decision,
            Some(SearchDecision::Finished(ChessOutcome::Draw(reason)))
        );
    }
}
#[test]
fn optional_claim_is_a_choice_while_auto_claim_forces_draw_even_over_mate() {
    let fen = "7k/8/5KQ1/8/8/8/8/8 w - - 100 1";
    let explicit = game(fen, ClaimPolicy::ExplicitOnly);
    assert!(explicit.outcome().is_none());
    assert!(!explicit.available_claims().is_empty());
    assert_eq!(run(&explicit, 1).score, Some(MATE_SCORE - 1));
    let auto = game(fen, ClaimPolicy::AutoClaim);
    assert!(
        auto.outcome().is_none(),
        "stored policy is not an exercised claim"
    );
    let result = run(&auto, 1);
    assert_eq!(result.score, Some(0));
    assert!(matches!(
        result.decision,
        Some(SearchDecision::Choose(SearchChoice::Claim(ClaimRequest {
            reason: DrawClaim::FiftyMove,
            intended: None,
            ..
        })))
    ));
    // Losing Black can preserve zero by an optional intended-move claim at 99.
    let losing = game("7k/8/8/8/8/8/8/KR6 b - - 99 1", ClaimPolicy::ExplicitOnly);
    let result = run(&losing, 1);
    assert_eq!(result.score, Some(0));
    assert!(matches!(
        result.decision,
        Some(SearchDecision::Choose(SearchChoice::Claim(ClaimRequest {
            reason: DrawClaim::FiftyMove,
            intended: Some(_),
            ..
        })))
    ));
}
#[test]
fn auto_claim_is_respected_at_leaf_turn_boundaries() {
    // White could otherwise retain a +500 leaf, but every quiet move enables
    // Black's automatic claim (including intended-move claims from counter 99).
    let auto = game("7k/8/8/8/8/8/8/KR6 w - - 98 1", ClaimPolicy::AutoClaim);
    assert!(auto.available_claims().is_empty());
    assert_eq!(run(&auto, 1).score, Some(0));
}
#[test]
fn same_current_fen_differs_with_relevant_repetition_history() {
    let mut repeated = game("6nk/8/8/8/8/8/8/KR4N1 b - - 0 1", ClaimPolicy::ExplicitOnly);
    for _ in 0..2 {
        for text in ["g8f6", "g1f3", "f6g8", "f3g1"] {
            repeated.apply(text.parse().unwrap()).unwrap();
        }
    }
    let imported = game(&repeated.position().to_fen(), ClaimPolicy::ExplicitOnly);
    assert_eq!(imported.position(), repeated.position());
    assert_eq!(imported.history().current_occurrences(), 1);
    assert_eq!(repeated.history().current_occurrences(), 3);
    let repeated_result = run(&repeated, 1);
    assert_eq!(repeated_result.score, Some(0));
    assert!(matches!(
        repeated_result.decision,
        Some(SearchDecision::Choose(SearchChoice::Claim(_)))
    ));
    assert_eq!(run(&imported, 1).score, Some(-500));
    // A fifth occurrence is an actual automatic result with explicit claims.
    for _ in 0..2 {
        for text in ["g8f6", "g1f3", "f6g8", "f3g1"] {
            repeated.apply(text.parse().unwrap()).unwrap();
        }
    }
    assert_eq!(
        run(&repeated, 1).decision,
        Some(SearchDecision::Finished(ChessOutcome::Draw(
            DrawReason::Fivefold
        )))
    );
}
#[test]
fn work_limits_publish_only_complete_iterations_and_leave_caller_untouched() {
    let g = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let before = g.clone();
    let first = run(&g, 1);
    assert_eq!(first.nodes, 21);
    for nodes in [0, 1, 20] {
        let result = search(
            &g,
            SearchLimits {
                depth: 2,
                nodes: Some(nodes),
            },
        )
        .unwrap();
        assert_eq!(result.status, SearchStatus::NodeLimit);
        assert_eq!(result.nodes, nodes);
        assert_eq!(result.completed_depth, 0);
        assert!(result.decision.is_none());
        assert!(result.score.is_none());
        assert!(result.principal_variation.is_empty());
    }
    let limits = SearchLimits {
        depth: 2,
        nodes: Some(25),
    };
    let result = search(&g, limits).unwrap();
    assert_eq!(result.status, SearchStatus::NodeLimit);
    assert_eq!(result.nodes, 25);
    assert_eq!(result.completed_depth, 1);
    assert_eq!(result.decision, first.decision);
    assert_eq!(result.score, first.score);
    assert_eq!(result.principal_variation, first.principal_variation);
    assert_eq!(search(&g, limits).unwrap(), result);
    let mut polls = 0;
    let stopped = search_with_stop(
        &g,
        SearchLimits {
            depth: 2,
            nodes: None,
        },
        || {
            polls += 1;
            polls > 25
        },
    )
    .unwrap();
    assert_eq!(stopped.status, SearchStatus::Stopped);
    assert_eq!(stopped.completed_depth, 1);
    assert_eq!(stopped.nodes, 25);
    assert_eq!(stopped.decision, first.decision);
    assert_eq!(g, before);
}
#[test]
fn invalid_depth_and_unrepresentable_transition_are_explicit_errors() {
    let g = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    for depth in [0, MAX_DEPTH + 1] {
        assert_eq!(
            search(&g, SearchLimits { depth, nodes: None }),
            Err(SearchError::InvalidDepth(depth))
        );
    }
    let g = game(
        "7k/8/8/8/8/8/8/KR6 b - - 0 4294967295",
        ClaimPolicy::ExplicitOnly,
    );
    let before = g.clone();
    assert_eq!(
        search(
            &g,
            SearchLimits {
                depth: 1,
                nodes: None
            }
        ),
        Err(SearchError::Transition(PlayError::Action(
            ActionError::CounterOverflow
        )))
    );
    assert_eq!(g, before);
}
