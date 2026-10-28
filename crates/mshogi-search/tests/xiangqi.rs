//! Independent arithmetic, elementary trees and frozen MSH-94/105 history facts.
use mshogi_core::xiangqi as x;
use mshogi_search::xiangqi::*;

const CYCLE_ROOT: &str = "4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1";
const CYCLE: [&str; 4] = ["e9f9", "f8e8", "f9e9", "e8f8"];
const POSITIVE: &str = "5k3/R8/9/9/9/9/9/9/9/3K5 b - - 0 1";
fn game(fen: &str) -> x::Game {
    x::Game::from_root(
        x::PositionRoot::from_fen(fen).unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap()
}
fn record(game: &x::Game) -> x::GameRecord {
    x::GameRecord::from_game(game).unwrap()
}
fn submit(game: &mut x::Game, actor: x::Side, command: x::GameCommand) {
    game.submit(x::GameRequest {
        expected: game.context(),
        actor,
        command,
    })
    .unwrap();
}
fn play(game: &mut x::Game, action: &str) {
    submit(
        game,
        game.position().side_to_move(),
        x::GameCommand::Play {
            action: action.parse().unwrap(),
        },
    );
}
fn run(game: &x::Game, depth: u16) -> SearchResult {
    let before = record(game);
    let context = game.context();
    let r = search(
        game,
        Limits {
            depth,
            nodes: MAX_NODES,
        },
    )
    .unwrap();
    assert_eq!(record(game), before);
    assert_eq!(game.context(), context);
    r
}
fn choice(result: &SearchResult) -> Choice {
    let Some(Decision::Choose(c)) = result.decision else {
        panic!("{result:?}")
    };
    c
}
fn exact_limits() -> x::ExactLimits {
    x::ExactLimits {
        states: 5000,
        transitions: 50_000,
        millis: 60_000,
        traversal: x::ExactTraversal::BreadthFirst,
        ..x::ExactLimits::default()
    }
}

#[test]
fn independent_material_and_native_soldier_advancement_arithmetic() {
    for (kind, value) in [
        (x::PieceKind::General, 0),
        (x::PieceKind::Advisor, 200),
        (x::PieceKind::Elephant, 200),
        (x::PieceKind::Horse, 400),
        (x::PieceKind::Rook, 900),
        (x::PieceKind::Cannon, 450),
        (x::PieceKind::Soldier, 100),
    ] {
        assert_eq!(piece_value(kind), value);
    }
    // 2*(200+200+400+900+450)+5*100 per side; home soldiers have no bonus.
    let initial = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    assert_eq!(
        evaluate(initial.position()),
        Evaluation {
            red: 4800,
            black: 4800
        }
    );
    // Red c3 is unadvanced; Black e5 has forward rank4, worth100+20.
    let p = x::Position::from_fen("4k4/9/9/9/4p4/9/2P6/9/9/4K4 w - - 0 1").unwrap();
    assert_eq!(
        evaluate(&p),
        Evaluation {
            red: 100,
            black: 120
        }
    );
    assert_eq!(evaluate(&p).score_for(x::Side::Red), -20);
    assert_eq!(evaluate(&p).score_for(x::Side::Black), 20);
    assert_ne!(EVALUATOR_ID, SEARCH_ID);
}

#[test]
fn cannon_screen_and_horse_leg_tactics_have_independent_material_rewards() {
    for (fen, action, score) in [
        // Own soldier c5 is precisely one screen between cannon c3 and rook c7.
        ("5k3/9/2r6/9/2P6/9/2C6/9/9/3K5 w - - 0 1", "c3c7", 590),
        // Empty c4 is the horse leg for c3-d5; capturing the rook leaves400.
        ("5k3/9/9/9/3r5/9/2N6/9/9/3K5 w - - 0 1", "c3d5", 400),
        // Color/rank-reversed geometry and native Black score perspective.
        ("3k5/9/9/2c6/9/2p6/9/2R6/9/5K3 b - - 0 1", "c6c2", 590),
        ("3k5/9/9/2n6/9/3R5/9/9/9/5K3 b - - 0 1", "c6d4", 400),
    ] {
        let native = game(fen);
        let result = run(&native, 1);
        assert_eq!(
            choice(&result),
            Choice::Move(action.parse().unwrap()),
            "{fen}: {result:?}"
        );
        assert_eq!(result.score, Some(score), "{fen}");
        assert_eq!(result.perspective, native.position().side_to_move());
        assert_eq!(result.status, Status::Complete);
        assert!(matches!(result.assessment, AssessmentEvidence::Fast(_)));
    }
}

#[test]
fn immediate_stalemate_is_a_native_win_for_each_side() {
    for (fen, action, side) in [
        // MSH-105 P1: rook e8 controls e9 and f8, leaving Black f9 unchecked.
        ("5k3/9/4R4/9/9/9/9/9/9/3K5 w - - 0 1", "e7e8", x::Side::Red),
        (
            "3k5/9/9/9/9/9/9/4r4/9/5K3 b - - 0 1",
            "e2e1",
            x::Side::Black,
        ),
    ] {
        let native = game(fen);
        let result = run(&native, 1);
        assert_eq!(choice(&result), Choice::Move(action.parse().unwrap()));
        assert_eq!(result.score, Some(WIN_SCORE - 1));
        let mut checked = game(fen);
        play(&mut checked, action);
        assert_eq!(
            checked.conclusion(),
            Some(x::Conclusion::Win {
                winner: side,
                reason: x::WinReason::NoLegalAction {
                    reason: x::NoActionReason::Stalemate
                }
            })
        );
    }
}

#[test]
fn same_board_different_history_changes_score_without_a_board_only_cache() {
    let mut known = game(CYCLE_ROOT);
    for a in CYCLE.iter().cycle().take(11) {
        play(&mut known, a);
    }
    assert!(known.warnings().iter().any(|w| w.active));
    let fresh = game(&known.position().to_fen().unwrap());
    assert_eq!(known.position(), fresh.position());
    assert_ne!(record(&known), record(&fresh));
    // e8f8 closes the prohibited fourth Red checking cycle. In the imported
    // diagram Black can instead capture the undefended adjacent rook e9,
    // producing the independently proven bare-General draw.
    let actual = run(&known, 1);
    let imported = run(&fresh, 1);
    assert_eq!(choice(&actual), Choice::Move("e8f8".parse().unwrap()));
    assert_eq!(actual.score, Some(WIN_SCORE - 1));
    assert_eq!(choice(&imported), Choice::Move("e8e9".parse().unwrap()));
    assert_eq!(imported.score, Some(0));
    let mut capture = game(&fresh.position().to_fen().unwrap());
    play(&mut capture, "e8e9");
    assert_eq!(
        capture.conclusion(),
        Some(x::Conclusion::Draw {
            reason: x::DrawReason::NeitherCanWin
        })
    );
    assert_eq!(
        run(&known, 1),
        actual,
        "deterministic work does not cache the imported score"
    );
}

#[test]
fn warned_offender_avoids_locally_legal_repetition_loss() {
    let mut known = game(CYCLE_ROOT);
    for a in CYCLE.iter().cycle().take(10) {
        play(&mut known, a);
    }
    assert_eq!(known.position().side_to_move(), x::Side::Red);
    assert!(
        known
            .position()
            .legal_actions()
            .unwrap()
            .contains(&"f9e9".parse().unwrap())
    );
    let mut fatal = game(CYCLE_ROOT);
    for a in CYCLE.iter().cycle().take(12) {
        play(&mut fatal, a);
    }
    assert_eq!(
        fatal.conclusion(),
        Some(x::Conclusion::Win {
            winner: x::Side::Black,
            reason: x::WinReason::Repetition {
                fault: x::FaultKind::PerpetualCheck
            }
        })
    );
    let result = run(&known, 2);
    assert_ne!(choice(&result), Choice::Move("f9e9".parse().unwrap()));
    assert!(result.score.unwrap() >= 0);
}

#[test]
fn claimant_specific_history_and_policy_admit_only_valid_turn_claims() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/xiangqi/natural-count.json"
    ))
    .unwrap();
    let f = fixtures["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == "NATURAL-11-CHECKS")
        .unwrap();
    let root = f["root_fen"].as_str().unwrap();
    let mut native = game(root);
    let mut automatic = x::Game::from_root(
        x::PositionRoot::from_fen(root).unwrap(),
        x::ClaimPolicy::AutoValidClaims,
    )
    .unwrap();
    for action in f["moves"].as_array().unwrap().iter().take(100) {
        play(&mut native, action.as_str().unwrap());
        play(&mut automatic, action.as_str().unwrap());
    }
    // Frozen claimant arithmetic: Red has11 checks, so100-1=99. Black has100.
    assert_eq!(
        native
            .natural_count_eligibility(x::Side::Red)
            .effective_plies,
        99
    );
    assert_eq!(
        native
            .natural_count_eligibility(x::Side::Black)
            .effective_plies,
        100
    );
    assert_eq!(native.position().side_to_move(), x::Side::Red);
    assert_eq!(native.conclusion(), None);
    assert!(
        matches!(choice(&run(&native, 1)), Choice::Move(_)),
        "non-turn Black claim cannot be chosen on Red's behalf"
    );
    assert_eq!(
        automatic.conclusion(),
        Some(x::Conclusion::Draw {
            reason: x::DrawReason::NaturalCount
        })
    );
    assert_eq!(run(&automatic, 1).score, Some(0));
    play(&mut native, f["moves"][100].as_str().unwrap());
    assert_eq!(native.position().side_to_move(), x::Side::Black);
    assert!(native.natural_count_eligibility(x::Side::Black).eligible);
    // At this fixture prefix Red Rh8/Black Ra4 have no immediate capture;
    // all ordinary one-ply outcomes score at most0. The valid draw is the
    // documented first equal-valued choice, not an automatic conclusion.
    let result = run(&native, 1);
    assert_eq!(
        choice(&result),
        Choice::Claim {
            actor: x::Side::Black
        }
    );
    assert_eq!(result.score, Some(0));
    assert_eq!(native.conclusion(), None);
}

#[test]
fn native_conclusion_winner_is_not_inferred_from_turn() {
    for (actor, winner, score) in [
        (x::Side::Red, x::Side::Black, -WIN_SCORE),
        (x::Side::Black, x::Side::Red, WIN_SCORE),
    ] {
        let mut native = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
        submit(&mut native, actor, x::GameCommand::Resign);
        assert_eq!(native.position().side_to_move(), x::Side::Red);
        let result = run(&native, 1);
        assert_eq!(result.score, Some(score));
        assert_eq!(
            result.decision,
            Some(Decision::Finished(x::Conclusion::Win {
                winner,
                reason: x::WinReason::Resignation
            }))
        );
        assert!(result.principal_variation.is_empty());
    }
    let native = game("5k3/9/9/9/9/9/9/9/9/3K5 w - - 0 1");
    assert_eq!(
        run(&native, 1).decision,
        Some(Decision::Finished(x::Conclusion::Draw {
            reason: x::DrawReason::NeitherCanWin
        }))
    );
    assert_eq!(run(&native, 1).score, Some(0));
}

#[test]
fn zero_work_partial_iterations_and_cancellation_publish_only_completed_work() {
    let native = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    let before = record(&native);
    let context = native.context();
    for (nodes, expected_nodes) in [(0, 0), (1, 1)] {
        let r = search(&native, Limits { depth: 1, nodes }).unwrap();
        assert_eq!(r.nodes, expected_nodes);
        assert_eq!(r.status, Status::NodeLimit);
        assert_eq!(r.completed_depth, 0);
        assert_eq!(r.decision, None);
        assert_eq!(r.score, None);
        assert!(r.principal_variation.is_empty());
    }
    let complete = run(&native, 1);
    assert_eq!(complete.nodes, 45); // Root plus44 complete board alternatives.
    let partial = search(
        &native,
        Limits {
            depth: 2,
            nodes: 46,
        },
    )
    .unwrap();
    assert_eq!(partial.completed_depth, 1);
    assert_eq!(partial.nodes, 46);
    assert_eq!(partial.status, Status::NodeLimit);
    assert_eq!(partial.decision, complete.decision);
    assert_eq!(partial.score, complete.score);
    assert_eq!(partial.principal_variation, complete.principal_variation);
    let stopped = search_with_stop(
        &native,
        Limits {
            depth: MAX_DEPTH,
            nodes: MAX_NODES,
        },
        || true,
    )
    .unwrap();
    assert_eq!(stopped.status, Status::Stopped);
    assert_eq!(stopped.nodes, 0);
    assert_eq!(stopped.decision, None);
    assert_eq!(stopped.score, None);
    let mut polls = 0;
    let interrupted = search_with_stop(
        &native,
        Limits {
            depth: MAX_DEPTH,
            nodes: MAX_NODES,
        },
        || {
            polls += 1;
            polls == 8
        },
    )
    .unwrap();
    assert_eq!(interrupted.status, Status::Stopped);
    assert!(interrupted.nodes > 0);
    assert_eq!(interrupted.score, None);
    assert_eq!(record(&native), before);
    assert_eq!(native.context(), context);
    for limits in [
        Limits { depth: 0, nodes: 0 },
        Limits {
            depth: MAX_DEPTH + 1,
            nodes: MAX_NODES,
        },
        Limits {
            depth: 1,
            nodes: MAX_NODES + 1,
        },
    ] {
        assert!(matches!(
            search(&native, limits),
            Err(SearchError::InvalidLimits)
        ));
    }
}

#[test]
fn exact_negative_is_read_only_proof_not_an_installed_game_draw() {
    let native = game("5k3/9/9/9/9/9/9/9/4A4/3K5 w - - 0 1");
    let before = record(&native);
    let context = native.context();
    assert_eq!(native.conclusion(), None);
    assert_eq!(
        native.no_win_assessment(),
        x::NoWinAssessment::UnknownExactUnavailable
    );
    let r = search_requiring_root_exact(
        &native,
        Limits {
            depth: 1,
            nodes: MAX_NODES,
        },
        exact_limits(),
        || false,
    )
    .unwrap();
    assert_eq!(r.decision, Some(Decision::ExactNoWin));
    assert_eq!(r.score, None);
    assert_eq!(r.nodes, 0);
    let AssessmentEvidence::Exact(observation) = &r.assessment else {
        panic!("{r:?}")
    };
    assert_eq!(observation.report().verdict, x::ExactVerdict::NeitherCanWin);
    assert_eq!(observation.report().work.states, 255);
    assert_eq!(observation.request().source_record(), &before);
    assert_eq!(record(&native), before);
    assert_eq!(native.context(), context);
    assert_eq!(native.conclusion(), None);
}

#[test]
fn exact_positive_is_not_a_forced_win_score_and_retains_its_witness() {
    let native = game(POSITIVE);
    let before = record(&native);
    let ordinary = run(&native, 1);
    assert_eq!(ordinary.score, Some(-900));
    let r = search_requiring_root_exact(
        &native,
        Limits {
            depth: 1,
            nodes: MAX_NODES,
        },
        exact_limits(),
        || false,
    )
    .unwrap();
    assert_eq!(r.score, ordinary.score);
    assert_eq!(r.decision, ordinary.decision);
    let AssessmentEvidence::Exact(observation) = &r.assessment else {
        panic!("{r:?}")
    };
    let x::ExactVerdict::NaturalWin {
        winner,
        reason,
        witness,
    } = &observation.report().verdict
    else {
        panic!("{r:?}")
    };
    assert_eq!(*winner, x::Side::Red);
    assert_eq!(*reason, x::NoActionReason::Stalemate);
    assert_eq!(witness.len(), 2);
    let mut position = native.position().clone();
    for &action in witness {
        position.apply(action).unwrap();
    }
    assert_eq!(
        position.local_status().unwrap(),
        x::LocalStatus::NoLegalAction {
            reason: x::NoActionReason::Stalemate
        }
    );
    assert_eq!(observation.request().source_record(), &before);
    assert_eq!(record(&native), before);
}

#[test]
fn required_exact_unknown_cancel_and_failure_are_unscored_and_preserve_root() {
    let native = game(POSITIVE);
    let before = record(&native);
    let context = native.context();
    for (exact, stop, reason, status) in [
        (
            x::ExactLimits {
                states: 0,
                ..exact_limits()
            },
            false,
            x::ExactStop::States,
            Status::Unresolved,
        ),
        (
            exact_limits(),
            true,
            x::ExactStop::Cancelled,
            Status::Stopped,
        ),
    ] {
        let r = search_requiring_root_exact(
            &native,
            Limits {
                depth: 1,
                nodes: MAX_NODES,
            },
            exact,
            || stop,
        )
        .unwrap();
        assert_eq!(r.status, status);
        assert_eq!(r.unresolved, Some(Unresolved::Exact(reason)));
        assert_eq!(r.score, None);
        assert_eq!(r.decision, None);
        assert_eq!(r.nodes, 0);
        assert_eq!(r.completed_depth, 0);
        assert!(r.principal_variation.is_empty());
    }
    assert!(matches!(
        search_requiring_root_exact(
            &native,
            Limits {
                depth: 1,
                nodes: MAX_NODES
            },
            x::ExactLimits {
                states: x::MAX_EXACT_STATES + 1,
                ..exact_limits()
            },
            || false
        ),
        Err(SearchError::Exact(x::ExactFailure::UnsupportedLimits))
    ));
    assert_eq!(record(&native), before);
    assert_eq!(native.context(), context);
}

#[test]
fn native_capacity_failure_is_unscored_and_preserves_complete_source() {
    let mut native = game(CYCLE_ROOT);
    for _ in 0..511 {
        submit(
            &mut native,
            x::Side::Black,
            x::GameCommand::ClaimNaturalCount,
        );
    }
    let before = record(&native);
    let context = native.context();
    assert_eq!(
        search(&native, DEFAULT_LIMITS),
        Err(SearchError::Transition(x::GameError::Capacity))
    );
    assert_eq!(record(&native), before);
    assert_eq!(native.context(), context);
}

#[test]
fn deterministic_default_work_and_equal_capture_ties_are_reproducible() {
    let native = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    let before = record(&native);
    // Both Cannons can jump the enemy Cannon to capture a Horse on rank9.
    // Equal +400 leaves retain native order: b2b9 precedes h2h9. All44
    // alternatives plus the root must be visited in the full depth1 iteration.
    let expected = search(&native, DEFAULT_LIMITS).unwrap();
    assert_eq!(
        expected.decision,
        Some(Decision::Choose(Choice::Move("b2b9".parse().unwrap())))
    );
    assert_eq!(
        (
            expected.score,
            expected.completed_depth,
            expected.nodes,
            expected.status
        ),
        (Some(400), 1, 45, Status::Complete)
    );
    for _ in 0..2 {
        assert_eq!(search(&native, DEFAULT_LIMITS).unwrap(), expected);
    }
    assert_eq!(record(&native), before);
}
