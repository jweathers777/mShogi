use super::*;
use mshogi_core::chess::{DrawClaim, DrawReason, STANDARD_START_FEN};

fn game(fen: &str) -> AdjudicatedGame {
    AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap()
}
fn exact(g: &AdjudicatedGame, depth: u16, entries: usize) -> ConventionalResult {
    search_with_options(
        g,
        SearchLimits { depth, nodes: None },
        SearchOptions {
            mode: SearchMode::Exact,
            table_entries: entries,
            time: None,
            diagnostics: SearchDiagnostics::default(),
        },
        || false,
    )
    .unwrap()
}
fn replay(g: &AdjudicatedGame, r: &ConventionalResult) {
    let mut copy = g.clone();
    let mut moves = 0;
    for (i, choice) in r.result.principal_variation.iter().enumerate() {
        match choice {
            SearchChoice::Move(a) => {
                copy.apply(*a).unwrap();
                moves += 1;
            }
            SearchChoice::Claim(c) => {
                assert_eq!(i + 1, r.result.principal_variation.len());
                copy.claim(*c).unwrap();
            }
        }
    }
    assert!(moves <= r.selective_depth);
    assert!(r.selective_depth <= MAX_PLY);
    assert_eq!(r.result.nodes, r.stats.main_nodes + r.stats.q_nodes);
}
// Independent exhaustive backup: no PVS, TT, ordering or incremental evaluation.
// Static arithmetic is independently tested in the evaluator module. Outcomes,
// claims and every applied action use the accepted native application boundary.
fn oracle(g: &mut AdjudicatedGame, depth: u16, ply: u16) -> i32 {
    if let Some(outcome) = g.outcome() {
        return match outcome {
            ChessOutcome::Draw(_) => 0,
            ChessOutcome::Checkmate { winner } => {
                if winner == g.position().side_to_move() {
                    MATE_SCORE - i32::from(ply)
                } else {
                    -MATE_SCORE + i32::from(ply)
                }
            }
        };
    }
    let claims = g.available_claims();
    if !claims.is_empty() && g.history().claim_policy() == ClaimPolicy::AutoClaim {
        return 0;
    }
    if depth == 0 {
        let s = evaluation::evaluate(g.position()).score_for(g.position().side_to_move());
        return if claims.is_empty() { s } else { s.max(0) };
    }
    let mut best = if claims.is_empty() { -INF } else { 0 };
    for a in g.position().legal_actions() {
        g.apply(a).unwrap();
        let score = -oracle(g, depth - 1, ply + 1);
        g.undo().unwrap();
        best = best.max(score);
    }
    best
}
#[test]
fn exact_pvs_and_history_checked_table_match_independent_exhaustive_oracle() {
    for (fen, depth) in [
        ("7k/8/8/8/8/8/q7/R6K w - - 0 1", 2),
        ("7k/8/4QK2/8/8/8/8/8 w - - 0 1", 3),
        ("7k/8/8/8/8/8/8/KR6 w - - 98 1", 2),
        ("6nk/8/8/8/8/8/8/KR4N1 b - - 0 1", 2),
    ] {
        let g = game(fen);
        let before = g.clone();
        let mut native = g.clone();
        let want = oracle(&mut native, depth, 0);
        let a = exact(&g, depth, 0);
        let b = exact(&g, depth, 4096);
        assert_eq!(a.result.score, Some(want), "{fen}");
        assert_eq!(b.result.score, Some(want), "{fen}");
        assert_eq!(g, before);
        assert_eq!(native, g);
        replay(&g, &a);
        replay(&g, &b);
        for entries in [1, 3, 4, 5] {
            let bounded = exact(&g, depth, entries);
            assert_eq!(
                bounded.result.score,
                Some(want),
                "{entries}-entry boundary: {fen}"
            );
            replay(&g, &bounded);
        }
    }
    // A hand-solvable three-ply mate forces aspiration widening from centipawns
    // into the terminal band; failed windows must never be published as complete.
    let g = game("7k/8/4QK2/8/8/8/8/8 w - - 0 1");
    let r = exact(&g, 3, 4096);
    assert_eq!(r.result.score, Some(MATE_SCORE - 3));
    assert!(r.stats.aspiration_researches > 0);
}
#[test]
fn native_mates_draws_and_claims_precede_evaluation_and_horizons() {
    for (fen, score) in [
        ("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", MATE_SCORE - 1),
        ("8/8/8/8/8/5kq1/8/7K b - - 0 1", MATE_SCORE - 1),
        ("7k/6Q1/5K2/8/8/8/8/8 b - - 0 1", -MATE_SCORE),
        ("7k/5K2/6Q1/8/8/8/8/8 b - - 0 1", 0),
        ("7k/8/8/8/3K4/8/8/8 w - - 0 1", 0),
        ("7k/8/8/8/8/8/8/KR6 w - - 150 1", 0),
    ] {
        let g = game(fen);
        let r = search(
            &g,
            SearchLimits {
                depth: 1,
                nodes: Some(2000),
            },
        )
        .unwrap();
        assert_eq!(r.result.score, Some(score), "{fen}");
        replay(&g, &r);
    }
    let forced =
        AdjudicatedGame::from_fen("7k/8/8/8/8/8/8/KR6 w - - 99 1", ClaimPolicy::AutoClaim).unwrap();
    let r = search(
        &forced,
        SearchLimits {
            depth: 1,
            nodes: None,
        },
    )
    .unwrap();
    assert!(matches!(
        r.result.decision,
        Some(SearchDecision::Choose(SearchChoice::Claim(_)))
    ));
    assert_eq!(r.result.score, Some(0));
    assert_eq!(r.selective_depth, 0);
    replay(&forced, &r);
    let leaf =
        AdjudicatedGame::from_fen("7k/8/8/8/8/8/8/KR6 w - - 98 1", ClaimPolicy::AutoClaim).unwrap();
    let r = search(
        &leaf,
        SearchLimits {
            depth: 1,
            nodes: None,
        },
    )
    .unwrap();
    assert_eq!(r.result.score, Some(0));
    assert_eq!(r.result.principal_variation.len(), 2);
    assert!(
        matches!(r.result.principal_variation[1],SearchChoice::Claim(c) if c.reason==DrawClaim::FiftyMove&&c.intended.is_some())
    );
    replay(&leaf, &r);
}
#[test]
fn quiescence_sees_native_recapture_and_quiet_check_evasions() {
    // Rxa2 wins a queen, but ...Ra8xa2 recaptures the rook. Static depth1 misses
    // that reply; quiescence must include it in its actual native continuation.
    let g = game("r6k/8/8/8/8/8/q7/R6K w - - 0 1");
    let shallow = exact(&g, 1, 0);
    let q = search(
        &g,
        SearchLimits {
            depth: 1,
            nodes: Some(5000),
        },
    )
    .unwrap();
    assert!(shallow.result.score.unwrap() - q.result.score.unwrap() > 300);
    assert_eq!(
        q.result.principal_variation[0],
        SearchChoice::Move("a1a2".parse().unwrap())
    );
    assert_eq!(
        q.result.principal_variation[1],
        SearchChoice::Move("a8a2".parse().unwrap())
    );
    assert!(q.selective_depth >= 2);
    replay(&g, &q);
    // After Ra1a8+ the king's evasions are quiet. Quiescence cannot stand pat.
    let mut child = game("7k/8/8/8/8/8/8/R5K1 w - - 0 1");
    child.apply("a1a8".parse().unwrap()).unwrap();
    let mut stop = || false;
    let mut e = engine(&child, &mut stop, SearchMode::Conventional);
    let v = e.quiescence(0, -INF, INF, 16, None).ok().unwrap();
    let Some(SearchChoice::Move(a)) = v.pv.first() else {
        panic!("missing quiet evasion")
    };
    assert!(child.position().piece_at(a.to).is_none());
    assert_eq!(e.cursor.game(), Some(&child));
}
#[test]
fn legal_exchange_estimates_cover_recaptures_ep_and_promotion() {
    for (fen, action, expected) in [
        ("7k/8/8/8/8/8/q7/R6K w - - 0 1", "a1a2", 900),
        ("r6k/8/8/8/8/8/q7/R6K w - - 0 1", "a1a2", 400),
        ("7k/8/8/3pP3/8/8/8/KR6 w - d6 0 1", "e5d6", 100),
        ("7k/P7/8/8/8/8/8/KR6 w - - 0 1", "a7a8q", 800),
    ] {
        let p = game(fen).position().clone();
        let original = p.clone();
        let mut stop = || false;
        let mut work = Work {
            nodes: 0,
            limit: None,
            start: Instant::now(),
            time: None,
            stop: &mut stop,
            seldepth: 0,
        };
        assert_eq!(
            exchange(&p, action.parse().unwrap(), 16, &mut work).ok(),
            Some(expected)
        );
        assert_eq!(p, original);
    }
}
fn engine<'a>(
    g: &AdjudicatedGame,
    stop: &'a mut dyn FnMut() -> bool,
    mode: SearchMode,
) -> Engine<'a> {
    Engine {
        cursor: SearchCursor::new(g),
        eval: EvalState::new(g.position()),
        work: Work {
            nodes: 0,
            limit: Some(30000),
            start: Instant::now(),
            time: None,
            stop,
            seldepth: 0,
        },
        mode,
        diagnostics: SearchDiagnostics::default(),
        table: vec![None; 4096],
        history: vec![0; 8192],
        continuation: vec![0; 4096],
        captures: vec![0; 768],
        killers: vec![[None; 2]; 129],
        stats: SearchStats::default(),
        generation: 0,
    }
}
#[test]
fn budgets_cancel_errors_publish_only_complete_iterations_and_restore_state() {
    let g = game(STANDARD_START_FEN);
    let before = g.clone();
    let first = search(
        &g,
        SearchLimits {
            depth: 1,
            nodes: None,
        },
    )
    .unwrap();
    assert_eq!(first.result.score, Some(61));
    assert_eq!(first.result.nodes, 21);
    assert_eq!(
        first.result.principal_variation[0],
        SearchChoice::Move("b1c3".parse().unwrap())
    );
    for n in [0, 1, 20, 25] {
        let r = search(
            &g,
            SearchLimits {
                depth: 8,
                nodes: Some(n),
            },
        )
        .unwrap();
        assert_eq!(r.result.nodes, n);
        assert_eq!(r.result.status, SearchStatus::NodeLimit);
        if n < 21 {
            assert_eq!(r.result.completed_depth, 0);
            assert_eq!(r.result.decision, None);
            assert_eq!(r.result.score, None);
        } else {
            assert_eq!(r.result.completed_depth, 1);
            assert_eq!(r.result.decision, first.result.decision);
            assert_eq!(r.result.score, first.result.score);
        }
        replay(&g, &r);
        assert_eq!(g, before);
    }
    let zero = search_with_time(
        &g,
        SearchLimits {
            depth: 8,
            nodes: Some(10000),
        },
        Some(TimeBudget::from_hard_ms(0)),
        || false,
    )
    .unwrap();
    assert_eq!(zero.result.completed_depth, 0);
    assert_eq!(zero.result.nodes, 0);
    assert_eq!(zero.result.status, SearchStatus::Stopped);
    assert_eq!(
        search_with_stop(
            &g,
            SearchLimits {
                depth: 8,
                nodes: None
            },
            || true
        )
        .unwrap()
        .result
        .nodes,
        0
    );
    let overflow = game("7k/8/8/8/8/8/8/KR6 b - - 0 4294967295");
    assert!(matches!(
        search(
            &overflow,
            SearchLimits {
                depth: 1,
                nodes: None
            }
        ),
        Err(SearchError::Transition(_))
    ));
    assert_eq!(g, before);
}
#[test]
fn actual_history_and_mate_distance_are_never_board_only_cached() {
    let mut known = game("6nk/8/8/8/8/8/8/KR4N1 b - - 0 1");
    for a in [
        "g8f6", "g1f3", "f6g8", "f3g1", "g8f6", "g1f3", "f6g8", "f3g1",
    ] {
        known.apply(a.parse().unwrap()).unwrap();
    }
    let imported = game(&known.position().to_fen());
    let a = search(
        &known,
        SearchLimits {
            depth: 2,
            nodes: Some(5000),
        },
    )
    .unwrap();
    let b = search(
        &imported,
        SearchLimits {
            depth: 2,
            nodes: Some(5000),
        },
    )
    .unwrap();
    assert_eq!(a.result.score, Some(0));
    assert!(b.result.score.unwrap() < 0);
    assert!(matches!(
        a.result.decision,
        Some(SearchDecision::Choose(SearchChoice::Claim(_)))
    ));
    let mut claim_cursor = SearchCursor::new(&known);
    assert!(!claim_cursor.push_null());
    let mut cursor = SearchCursor::new(&imported);
    let key = cursor.key();
    assert_ne!(key, claim_cursor.key());
    assert!(cursor.push_null());
    assert!(cursor.key().is_none());
    assert!(cursor.game().is_none());
    assert!(cursor.node().outcome.is_none());
    assert!(cursor.pop());
    assert_eq!(cursor.key(), key);
    for s in [0, 621, -621, MATE_SCORE - 7, -MATE_SCORE + 7] {
        assert_eq!(from_table(to_table(s, 4), 4), s);
    }
    assert_eq!(from_table(to_table(MATE_SCORE - 7, 4), 6), MATE_SCORE - 9);
}
#[test]
fn controlled_null_verification_and_low_material_guards_restore_real_state() {
    let g = game(STANDARD_START_FEN);
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    let eval = e.eval.clone();
    let key = e.cursor.key();
    let v = e
        .main(4, 1, -100001, -100000, false, true, 4, None)
        .ok()
        .unwrap();
    assert!(v.score >= -100000);
    assert_eq!(e.stats.null_probes, 1);
    assert_eq!(e.stats.null_verified, 1);
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.cursor.key(), key);
    assert_eq!(e.eval, eval);
    assert!(v.score.abs() < MATE_BAND);
    let low = game("7k/8/2K5/8/8/8/P7/8 w - - 0 1");
    assert!(!null_material(low.position()));
    let mut e = engine(&low, &mut stop, SearchMode::Conventional);
    e.main(4, 1, -100001, -100000, false, true, 4, None)
        .ok()
        .unwrap();
    assert_eq!(e.stats.null_probes, 0);
    let claim = AdjudicatedGame::from_fen(
        &STANDARD_START_FEN.replace("0 1", "99 1"),
        ClaimPolicy::AutoClaim,
    )
    .unwrap();
    let mut e = engine(&claim, &mut stop, SearchMode::Conventional);
    assert_eq!(
        e.main(4, 1, -100001, -100000, false, true, 4, None)
            .ok()
            .unwrap()
            .score,
        0
    );
    assert_eq!(e.stats.null_probes, 0);
    let r = search(
        &g,
        SearchLimits {
            depth: 4,
            nodes: Some(10000),
        },
    )
    .unwrap();
    replay(&g, &r);
    assert!(r.stats.reductions > 0);
    assert!(r.stats.researches > 0);
    let dead = game("7k/8/8/8/3K4/8/8/8 w - - 0 1");
    assert_eq!(
        dead.outcome(),
        Some(ChessOutcome::Draw(DrawReason::DeadPosition))
    );
}

#[test]
fn synthetic_counter_failure_abandons_probe_but_real_mate_and_cancellation_stay_honest() {
    // Qg7# is protected by Kf6; h7/g8 are covered and the distant Qa8 cannot
    // answer the adjacent check. A real White action does not increment the
    // fullmove counter. The hypothetical Black null turn cannot play any action.
    let g = game("q6k/8/5KQ1/8/8/8/8/5R2 w - - 0 4294967295");
    let mate: Action = "g6g7".parse().unwrap();
    let mut child = g.clone();
    child.apply(mate).unwrap();
    assert_eq!(
        child.outcome(),
        Some(ChessOutcome::Checkmate {
            winner: Color::White
        })
    );
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    let original_eval = e.eval.clone();
    let key = e.cursor.key().unwrap();
    let hash = e.cursor.position_hash();
    let index = e.table_index(hash).unwrap();
    e.table[index] = Some(Entry {
        generation: 0,
        key: key.clone(),
        hash,
        depth: 0,
        extensions: 4,
        mate_threat: false,
        score: 0,
        bound: Bound::Upper,
        value: Value {
            score: 0,
            pv: vec![SearchChoice::Move(mate)],
        },
    });
    let value = e.main(4, 1, -1, 0, false, true, 4, None).ok().unwrap();
    assert_eq!(value.score, MATE_SCORE - 2);
    assert_eq!(value.pv[0], SearchChoice::Move(mate));
    assert_eq!(e.stats.null_abandoned, 1);
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.eval, original_eval);
    assert_eq!(e.cursor.key(), Some(key.clone()));
    e.work.time = Some(TimeBudget::from_hard_ms(0));
    assert!(matches!(
        e.probe_null(1, 1, 0, 4, None),
        Err(Halt::Limit(SearchStatus::Stopped))
    ));
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.cursor.key(), Some(key));
    assert_eq!(e.eval, original_eval);
}

#[test]
fn controlled_late_quiet_pruning_and_absolute_check_horizon_are_explicit() {
    let g = game(STANDARD_START_FEN);
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    // Known depth1 best b1c3/+61 occurs before the late quiet threshold.
    // Static0+140 is above alpha100, so this specifically exercises the
    // move-count/history guard rather than static-margin futility pruning.
    let v = e.main(1, 1, 100, 1000, false, false, 4, None).ok().unwrap();
    assert_eq!(v.score, 61);
    assert!(e.stats.late_move_pruned > 0);
    assert_eq!(e.stats.futility_pruned, 0);
    assert_eq!(e.cursor.game(), Some(&g));
    let mut checked = game("7k/8/8/8/8/8/8/R5K1 w - - 0 1");
    checked.apply("a1a8".parse().unwrap()).unwrap();
    let mut e = engine(&checked, &mut stop, SearchMode::Conventional);
    // The absolute recursion ceiling is an explicitly approximate horizon,
    // including in check. It must not invent an evasion or native outcome.
    let v = e.quiescence(MAX_PLY, -INF, INF, 16, None).ok().unwrap();
    assert!(v.pv.is_empty());
    assert!(e.cursor.node().outcome.is_none());
    assert_eq!(e.cursor.game(), Some(&checked));
    assert_eq!(e.work.seldepth, MAX_PLY);
}

#[test]
fn timed_full_native_solver_is_explicitly_refused_without_mutating_its_context() {
    let g = AdjudicatedGame::from_fen_with_mode(
        STANDARD_START_FEN,
        ClaimPolicy::ExplicitOnly,
        AdjudicationMode::Full {
            limits: mshogi_core::chess::ReachabilityLimits {
                max_positions: Some(0),
                max_transitions: Some(0),
            },
        },
    )
    .unwrap();
    let before = g.clone();
    assert!(matches!(
        search_with_time(
            &g,
            SearchLimits {
                depth: 1,
                nodes: Some(1000)
            },
            Some(TimeBudget::from_hard_ms(10)),
            || false
        ),
        Err(SearchError::TimedFullUnsupported)
    ));
    assert_eq!(g, before);
}

#[test]
fn board_hash_collision_never_admits_a_foreign_history_bound_or_claim() {
    let mut known = game("6nk/8/8/8/8/8/8/KR4N1 b - - 0 1");
    for a in [
        "g8f6", "g1f3", "f6g8", "f3g1", "g8f6", "g1f3", "f6g8", "f3g1",
    ] {
        known.apply(a.parse().unwrap()).unwrap();
    }
    let imported = game(&known.position().to_fen());
    let mut stop = || false;
    let mut e = engine(&imported, &mut stop, SearchMode::Exact);
    let foreign = SearchCursor::new(&known).key().unwrap();
    let hash = e.cursor.position_hash();
    let index = e.table_index(hash).unwrap();
    e.table[index] = Some(Entry {
        generation: 0,
        key: foreign,
        hash,
        depth: 1,
        extensions: 4,
        mate_threat: false,
        score: MATE_SCORE - 1,
        bound: Bound::Exact,
        value: Value {
            score: MATE_SCORE - 1,
            pv: vec![SearchChoice::Claim(known.available_claims()[0])],
        },
    });
    let v = e.main(1, 0, -INF, INF, false, false, 4, None).ok().unwrap();
    let mut native = imported.clone();
    assert_eq!(v.score, oracle(&mut native, 1, 0));
    assert!(v.score < 0);
    assert_eq!(e.stats.tt_cutoffs, 0);
    assert!(matches!(v.pv.first(), Some(SearchChoice::Move(_))));
    assert_eq!(e.cursor.game(), Some(&imported));
}

#[test]
fn native_compatible_transposed_history_reuses_a_completed_exact_bound() {
    let g = game(STANDARD_START_FEN);
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Exact);
    for text in ["g1f3", "g8f6", "b1c3", "b8c6", "e2e4"] {
        e.push(text.parse().unwrap()).ok().unwrap();
    }
    let key = e.cursor.key();
    let value = e.main(1, 5, -INF, INF, false, false, 4, None).ok().unwrap();
    let mut native = e.cursor.game().unwrap().clone();
    assert_eq!(value.score, oracle(&mut native, 1, 5));
    for _ in 0..5 {
        e.pop();
    }
    for text in ["b1c3", "b8c6", "g1f3", "g8f6", "e2e4"] {
        e.push(text.parse().unwrap()).ok().unwrap();
    }
    assert_eq!(e.cursor.key(), key);
    let cutoffs = e.stats.tt_cutoffs;
    let before_nodes = e.work.nodes;
    let reused = e.main(1, 5, -INF, INF, false, false, 4, None).ok().unwrap();
    assert_eq!(reused.score, value.score);
    assert_eq!(reused.pv, value.pv);
    assert_eq!(e.stats.tt_cutoffs, cutoffs + 1);
    assert_eq!(e.work.nodes, before_nodes + 1);
    let mut replay = e.cursor.game().unwrap().clone();
    for choice in reused.pv {
        match choice {
            SearchChoice::Move(a) => replay.apply(a).unwrap(),
            SearchChoice::Claim(c) => replay.claim(c).unwrap(),
        }
    }
    for _ in 0..5 {
        e.pop();
    }
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.eval, EvalState::new(g.position()));
}

#[test]
fn quiescence_finds_bounded_quiet_mate_and_prunes_losing_or_hopeless_captures() {
    let mut stop = || false;
    for (fen, mate) in [
        ("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", "g6g7"),
        ("8/8/8/8/8/5kq1/8/7K b - - 0 1", "g3g2"),
    ] {
        let g = game(fen);
        let mut native = g.clone();
        native.apply(mate.parse().unwrap()).unwrap();
        assert_eq!(
            native.outcome(),
            Some(ChessOutcome::Checkmate {
                winner: g.position().side_to_move()
            })
        );
        let mut e = engine(&g, &mut stop, SearchMode::Conventional);
        let v = e.quiescence(0, -INF, INF, Q_PLIES, None).ok().unwrap();
        assert_eq!(v.score, MATE_SCORE - 1);
        let mut replay = g.clone();
        for choice in v.pv {
            if let SearchChoice::Move(a) = choice {
                replay.apply(a).unwrap();
            }
        }
        assert!(matches!(
            replay.outcome(),
            Some(ChessOutcome::Checkmate { .. })
        ));
        assert_eq!(e.cursor.game(), Some(&g));
        assert_eq!(e.eval, EvalState::new(g.position()));
    }
    // Rxa2 wins100 then loses500 to Ra8: a negative native exchange. It
    // neither checks nor concludes/claims, so no delta condition is needed.
    let g = game("r6k/8/8/8/8/8/p7/R6K w - - 0 1");
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    let v = e.quiescence(0, -INF, INF, Q_PLIES - 2, None).ok().unwrap();
    assert!(v.pv.is_empty());
    assert_eq!(e.stats.see_pruned, 1);
    assert_eq!(e.work.nodes, 1);
    // An unrecaptured pawn still cannot bridge a very high alpha. Delta is
    // independently useful even though the exchange estimate is positive.
    let g = game("7k/8/8/8/8/8/p7/R6K w - - 0 1");
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    let v = e.quiescence(0, 5000, 5001, Q_PLIES - 2, None).ok().unwrap();
    assert!(v.pv.is_empty());
    assert_eq!(e.stats.see_pruned, 1);
    assert_eq!(e.work.nodes, 1);
    assert_eq!(e.cursor.game(), Some(&g));
}

#[test]
fn admitted_hint_is_staged_before_unrelated_exchange_work_and_history_is_signed() {
    let g = game("r6k/8/8/8/8/8/p7/R6K w - - 0 1");
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Exact);
    let hint: Action = "h1g1".parse().unwrap();
    let mut picker = MovePicker::new(e.cursor.node().actions.clone(), Some(hint));
    assert_eq!(
        picker.next(&mut e, 0, None).ok().flatten().unwrap().action,
        hint
    );
    assert_eq!(e.stats.see_probes, 0, "no work for remaining capture yet");
    let mut actions = vec![hint];
    while let Some(m) = picker.next(&mut e, 0, None).ok().flatten() {
        actions.push(m.action);
    }
    assert!(e.stats.see_probes > 0);
    let expected = g.position().legal_actions();
    assert_eq!(actions.len(), expected.len());
    assert!(
        expected
            .iter()
            .all(|a| actions.iter().filter(|b| *b == a).count() == 1)
    );
    let illegal: Action = "h1h8".parse().unwrap();
    let picker = MovePicker::new(expected, Some(illegal));
    assert!(
        picker.hint.is_none(),
        "hash hint is never legality authority"
    );
    let mut slot = 0;
    history_update(&mut slot, 1000);
    assert_eq!(slot, 1000);
    history_update(&mut slot, -1000);
    assert_eq!(slot, -62);
    for _ in 0..1000 {
        history_update(&mut slot, -1000);
    }
    assert!((-16_000..0).contains(&slot));
}

#[test]
fn current_and_intended_claims_precede_mate_distance_window_pruning() {
    let mut stop = || false;
    for clock in [99, 100] {
        for policy in [ClaimPolicy::AutoClaim, ClaimPolicy::ExplicitOnly] {
            let g =
                AdjudicatedGame::from_fen(&format!("7k/8/8/8/8/8/8/KR6 w - - {clock} 1"), policy)
                    .unwrap();
            for (alpha, beta) in [
                (-MATE_SCORE, -MATE_SCORE + 1),
                (MATE_SCORE - 2, MATE_SCORE - 1),
            ] {
                if policy == ClaimPolicy::ExplicitOnly && beta > 0 {
                    continue;
                }
                let mut e = engine(&g, &mut stop, SearchMode::Conventional);
                let main = e
                    .main(1, 8, alpha, beta, false, false, 4, None)
                    .ok()
                    .unwrap();
                let quiet = e.quiescence(8, alpha, beta, Q_PLIES, None).ok().unwrap();
                for v in [main, quiet] {
                    assert_eq!(v.score, 0);
                    let SearchChoice::Claim(claim) = v.pv[0] else {
                        panic!("native claim required");
                    };
                    let mut native = g.clone();
                    native.claim(claim).unwrap();
                    assert!(matches!(
                        native.outcome(),
                        Some(ChessOutcome::Draw(DrawReason::Claimed(_)))
                    ));
                }
                assert_eq!(e.cursor.game(), Some(&g));
            }
        }
    }
}

#[test]
fn quiescence_table_reuses_only_matching_native_context_and_remaining_budget() {
    let g = game("6k1/5ppp/8/8/8/8/8/R5K1 w - - 0 1");
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    let first = e.quiescence(0, -INF, INF, Q_PLIES, None).ok().unwrap();
    assert_eq!(first.score, MATE_SCORE - 1);
    let nodes = e.work.nodes;
    let second = e.quiescence(0, -INF, INF, Q_PLIES, None).ok().unwrap();
    assert_eq!(second.score, first.score);
    assert_eq!(second.pv, first.pv);
    assert_eq!(e.work.nodes, nodes + 1);
    assert_eq!(e.stats.q_tt_cutoffs, 1);
    // This budget excludes quiet checks, so the earlier mate observation is
    // not eligible even at the identical complete native history.
    let limited = e.quiescence(0, -INF, INF, Q_PLIES - 2, None).ok().unwrap();
    assert!(limited.score < MATE_BAND);
    assert!(limited.pv.is_empty());
    assert_eq!(e.stats.q_tt_cutoffs, 1);
    e.mode = SearchMode::Exact;
    let before = e.work.nodes;
    let main = e.main(1, 0, -INF, INF, false, false, 4, None).ok().unwrap();
    assert_eq!(main.score, MATE_SCORE - 1);
    assert!(
        e.work.nodes > before + 1,
        "q bounds cannot satisfy main depth"
    );
    assert_eq!(e.stats.tt_hits, 0);
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.eval, EvalState::new(g.position()));
}

#[test]
fn bounded_buckets_retain_deep_current_entries_then_retire_old_generations() {
    let g = game(STANDARD_START_FEN);
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Exact);
    e.table = vec![None; 4];
    e.generation = 1;
    let mut entries = Vec::new();
    for (i, text) in ["e2e4", "d2d4", "g1f3", "b1c3", "c2c4"]
        .into_iter()
        .enumerate()
    {
        e.push(text.parse().unwrap()).ok().unwrap();
        entries.push(Entry {
            key: e.cursor.key().unwrap(),
            hash: e.cursor.position_hash(),
            depth: if i == 4 { 1 } else { 6 + i as i32 },
            extensions: 4,
            mate_threat: false,
            score: 0,
            bound: Bound::Upper,
            value: Value::leaf(0),
            generation: 1,
        });
        e.pop();
    }
    for entry in entries.iter().take(4) {
        e.store(entry.clone(), false);
    }
    assert!(e.table.iter().all(Option::is_some));
    e.store(entries[4].clone(), false);
    assert_eq!(e.stats.tt_store_refusals, 1);
    assert_eq!(e.stats.tt_replacements, 0);
    for entry in entries.iter().take(4) {
        assert!(e.entry_index(entry.hash, Some(&entry.key), None).is_some());
    }
    e.generation = 3;
    let mut fresh = entries[4].clone();
    fresh.generation = 3;
    e.store(fresh.clone(), false);
    assert_eq!(e.stats.tt_replacements, 1);
    assert!(e.entry_index(fresh.hash, Some(&fresh.key), None).is_some());
    assert_eq!(
        e.table.len(),
        4,
        "replacement never expands the admitted budget"
    );
    // Q observations may not flood out even shallow current main searches.
    e.table.fill(None);
    for mut entry in entries.iter().take(4).cloned() {
        entry.depth = 1;
        entry.generation = e.generation;
        e.store(entry, false);
    }
    let refusals = e.stats.tt_store_refusals;
    fresh.depth = -17;
    fresh.extensions = 0;
    e.store(fresh, true);
    assert_eq!(e.stats.tt_store_refusals, refusals + 1);
    assert!(
        e.table
            .iter()
            .all(|slot| slot.as_ref().is_some_and(|entry| entry.depth == 1))
    );
}

#[test]
fn diverse_hand_solved_tactics_preserve_complete_native_actions_and_conclusions() {
    // Original small mechanisms, not sampled strength/ Elo data. Each supplied
    // witness is independently explained below and natively verified; search
    // may choose another actual mate at the same distance.
    for (name, fen, witness) in [
        ("back rank", "6k1/5ppp/8/8/8/8/8/R5K1 w - - 0 1", "a1a8"),
        (
            "smothered knight",
            "6rk/6pp/8/6N1/8/8/8/K7 w - - 0 1",
            "g5f7",
        ),
        (
            "discovered rook",
            "3rkr2/3p1p2/8/8/8/8/4B3/4R1K1 w - - 0 1",
            "e2b5",
        ),
        (
            "EP discovered rook",
            "3rkr2/5p2/8/3pP3/6B1/8/8/4R1K1 w - d6 0 1",
            "e5d6",
        ),
        (
            "castling rook",
            "4rkr1/4p1p1/8/8/8/8/8/4K2R w K - 0 1",
            "e1g1",
        ),
        ("promotion", "7k/2P5/6K1/8/8/8/8/8 w - - 0 1", "c7c8r"),
        (
            "stalemate avoidance",
            "k7/8/1QK5/8/8/8/8/8 w - - 0 1",
            "b6b7",
        ),
    ] {
        // The trapped kings have no check evasion: home pawns/pieces obstruct
        // exits in the first five (Bg4 covers the vacant d7 after EP); Kg6 covers
        // both seventh-rank escapes in the
        // promotion case; Kc6 protects Qb7 in the final corner pattern.
        let g = game(fen);
        let mut native = g.clone();
        native.apply(witness.parse().unwrap()).unwrap();
        assert_eq!(
            native.outcome(),
            Some(ChessOutcome::Checkmate {
                winner: Color::White
            }),
            "witness {name}"
        );
        let r = search(
            &g,
            SearchLimits {
                depth: 4,
                nodes: Some(40_000),
            },
        )
        .unwrap();
        assert_eq!(r.result.score, Some(MATE_SCORE - 1), "{name}");
        assert!(r.result.completed_depth >= 1);
        let Some(SearchDecision::Choose(SearchChoice::Move(choice))) = r.result.decision else {
            panic!("{name}: complete move required");
        };
        let mut played = g.clone();
        played.apply(choice).unwrap();
        assert_eq!(
            played.outcome(),
            Some(ChessOutcome::Checkmate {
                winner: Color::White
            }),
            "selected {name}"
        );
        replay(&g, &r);
    }
    let mut stale = game("k7/8/1QK5/8/8/8/8/8 w - - 0 1");
    stale.apply("b6c7".parse().unwrap()).unwrap();
    assert_eq!(
        stale.outcome(),
        Some(ChessOutcome::Draw(DrawReason::Stalemate))
    );
    // Nf7+ attacks Kh8 and Qd8. A knight check cannot be blocked; every native
    // reply moves the king and leaves Nxd8 winning the queen. Pa2 keeps the
    // resulting material from being a native bare-minor dead-position draw.
    let g = game("3q3k/8/8/4N3/8/8/P7/K7 w - - 0 1");
    let fork: Action = "e5f7".parse().unwrap();
    let mut checked = g.clone();
    checked.apply(fork).unwrap();
    assert_eq!(checked.position().classify(), Classification::Check);
    for reply in checked.position().legal_actions() {
        assert_eq!(
            checked.position().piece_at(reply.from).unwrap().kind,
            PieceKind::King
        );
        let mut line = checked.clone();
        line.apply(reply).unwrap();
        line.apply("f7d8".parse().unwrap()).unwrap();
        assert_eq!(
            line.position()
                .piece_at(Square::new(3, 7).unwrap())
                .unwrap()
                .kind,
            PieceKind::Knight
        );
    }
    let r = search(
        &g,
        SearchLimits {
            depth: 3,
            nodes: Some(40_000),
        },
    )
    .unwrap();
    assert_eq!(
        r.result.decision,
        Some(SearchDecision::Choose(SearchChoice::Move(fork)))
    );
    assert!(r.result.score.unwrap() > 0);
    replay(&g, &r);
}

#[test]
fn forcing_three_ply_sacrifices_are_proved_against_every_native_defence() {
    for (name, fen, first, reply, mate) in [
        // After ...Bxd1 in the Legal pattern, Ne5 protects Bf7, coversd7;
        // the bishop checks and Ke7 is the sole evasion. Nd5 then coversf6.
        (
            "Legal minor coordination",
            "rn1qkbnr/ppp2p1p/3p2p1/4N3/2B1P3/2N5/PPPP1PPP/R1BbK2R w KQkq - 0 5",
            "c4f7",
            "e8e7",
            "c3d5",
        ),
        // Qxf8+ deflects Kg8 onto f8; Re8 is protected by Bb5, so none of
        // e8/e7/g8 is an escape and f7/g7 remain occupied by Black pawns.
        (
            "queen deflection",
            "5rk1/4Qppp/8/1B6/8/8/8/4R1K1 w - - 0 1",
            "e7f8",
            "g8f8",
            "e1e8",
        ),
        // Bc4 protects Qg8 so Kh8 cannot capture it. The forced rook capture
        // smothers Kh8 behind Rg8/g7/h7, letting Nf7 deliver the final check.
        (
            "queen sacrifice smother",
            "5r1k/6pp/4Q3/6N1/2B5/8/8/6K1 w - - 0 1",
            "e6g8",
            "f8g8",
            "g5f7",
        ),
    ] {
        let g = game(fen);
        // Independently rule out a faster one-ply mate before asserting the
        // exact mate-distance score of this forced three-ply witness.
        for a in g.position().legal_actions() {
            let mut line = g.clone();
            line.apply(a).unwrap();
            assert!(
                !matches!(line.outcome(), Some(ChessOutcome::Checkmate { .. })),
                "unexpected faster mate {name}: {a:?}"
            );
        }
        let mut line = g.clone();
        line.apply(first.parse().unwrap()).unwrap();
        let replies: Vec<_> = line
            .position()
            .legal_actions()
            .iter()
            .map(ToString::to_string)
            .collect();
        assert_eq!(replies, [reply], "all defences {name}");
        line.apply(reply.parse().unwrap()).unwrap();
        line.apply(mate.parse().unwrap()).unwrap();
        assert_eq!(
            line.outcome(),
            Some(ChessOutcome::Checkmate {
                winner: Color::White
            })
        );
        let r = search(
            &g,
            SearchLimits {
                depth: 4,
                nodes: Some(80_000),
            },
        )
        .unwrap();
        assert_eq!(r.result.score, Some(MATE_SCORE - 3), "{name}");
        assert!(r.result.principal_variation.len() >= 3);
        replay(&g, &r);
        let mut played = g.clone();
        for choice in r.result.principal_variation {
            if let SearchChoice::Move(a) = choice {
                played.apply(a).unwrap();
            }
        }
        assert_eq!(
            played.outcome(),
            Some(ChessOutcome::Checkmate {
                winner: Color::White
            })
        );
    }
}

#[test]
fn historical_attack_null_mate_threat_only_extends_real_search_within_shared_budget() {
    let mut g = game("rn3rk1/p4ppp/3bpP2/1Q6/3P4/2q2N2/5PPP/R1B2RK1 b - - 0 15");
    g.apply("c3a1".parse().unwrap()).unwrap();
    g.apply("b5g5".parse().unwrap()).unwrap();
    // This is a threat, not a claimed forced mate: if Black passed, Qxg7#
    // would be protected by Pf6, and Qg7 coversf8/h8 while f7/h7 are blocked.
    let mut cursor = SearchCursor::new(&g);
    assert!(cursor.push_null());
    assert!(cursor.game().is_none());
    let mut hypothesis = cursor.position().clone();
    hypothesis.apply("g5g7".parse().unwrap()).unwrap();
    assert_eq!(hypothesis.classify(), Classification::Checkmate);
    assert!(cursor.pop());
    assert_eq!(cursor.game(), Some(&g));
    // Capturing Pf6 does not save the position: ...gxf6 would vacate g7
    // and expose Kg8 directly on Qg5's file. Native admission rejects it.
    let mut defence = g.clone();
    assert!(matches!(
        defence.apply("g7f6".parse().unwrap()),
        Err(mshogi_core::chess::PlayError::Action(
            mshogi_core::chess::ActionError::IllegalPosition(_)
        ))
    ));
    assert_eq!(defence, g);
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    let probe = e.probe_null(1, 2, 0, 1, None).ok().flatten().unwrap();
    assert!(probe.score >= MATE_BAND);
    assert_eq!(e.cursor.game(), Some(&g));
    e.work.limit = Some(100_000);
    let real = e.main(4, 2, -1, 0, false, true, 1, None).ok().unwrap();
    assert!(e.stats.threat_extensions >= 1);
    assert!(e.work.seldepth <= MAX_PLY);
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.eval, EvalState::new(g.position()));
    if let Some(SearchChoice::Move(a)) = real.pv.first() {
        let mut native = g.clone();
        native.apply(*a).unwrap();
    }
    let mut limited = engine(&g, &mut stop, SearchMode::Conventional);
    let _ = limited.main(4, 2, -1, 0, false, true, 0, None);
    assert_eq!(
        limited.stats.threat_extensions, 0,
        "no credit means no threat extension"
    );
    assert_eq!(limited.cursor.game(), Some(&g));
}

#[test]
fn practical_pawn_promotion_chooses_material_without_fabricating_a_native_win() {
    let g = game("7k/P7/K7/8/8/8/8/8 w - - 0 1");
    let r = search(
        &g,
        SearchLimits {
            depth: 1,
            nodes: Some(10_000),
        },
    )
    .unwrap();
    let queen: Action = "a7a8q".parse().unwrap();
    assert_eq!(
        r.result.decision,
        Some(SearchDecision::Choose(SearchChoice::Move(queen)))
    );
    let mut played = g.clone();
    played.apply(queen).unwrap();
    assert!(
        played.outcome().is_none(),
        "a static conversion preference is not adjudication"
    );
    replay(&g, &r);
    let mut minor = g.clone();
    minor.apply("a7a8n".parse().unwrap()).unwrap();
    assert_eq!(
        minor.outcome(),
        Some(ChessOutcome::Draw(DrawReason::DeadPosition))
    );
}

#[test]
fn historical_quiet_mating_plan_is_proved_against_every_native_defence() {
    fn prove(g: &AdjudicatedGame, plan: &[Action], left: u8, visits: &mut u32) -> Result<bool, ()> {
        *visits += 1;
        if *visits > 20_000 {
            return Err(());
        }
        if let Some(outcome) = g.outcome() {
            return Ok(outcome
                == ChessOutcome::Checkmate {
                    winner: Color::White,
                });
        }
        if left == 0 {
            return Ok(false);
        }
        if g.position().side_to_move() == Color::White {
            for &a in plan {
                let mut child = g.clone();
                if child.apply(a).is_ok() && prove(&child, plan, left - 1, visits)? {
                    return Ok(true);
                }
            }
            Ok(false)
        } else {
            if !g.available_claims().is_empty() {
                return Ok(false);
            }
            for a in g.position().legal_actions() {
                let mut child = g.clone();
                child.apply(a).unwrap();
                if !prove(&child, plan, left - 1, visits)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
    }
    let mut g = game("rn3rk1/p4ppp/3bpP2/1Q6/3P4/2q2N2/5PPP/R1B2RK1 b - - 0 15");
    for a in ["c3a1", "b5g5"] {
        g.apply(a.parse().unwrap()).unwrap();
    }
    let plan: Vec<Action> = ["g5g7", "h6g7", "g5h6", "f3h2"]
        .iter()
        .map(|a| a.parse().unwrap())
        .collect();
    let mut visits = 0;
    let result = prove(&g, &plan, 6, &mut visits);
    println!("historical fixed White plan / all native Black actions: {result:?}, visits={visits}");
    assert_eq!(
        result,
        Ok(true),
        "explicit White plan covers every native Black reply"
    );
}

#[test]
fn quiet_contact_threat_search_finds_the_proved_historical_mate_without_a_material_fallback() {
    let mut g = game("rn3rk1/p4ppp/3bpP2/1Q6/3P4/2q2N2/5PPP/R1B2RK1 b - - 0 15");
    g.apply("c3a1".parse().unwrap()).unwrap();
    let original = g.clone();
    let r = search(
        &g,
        SearchLimits {
            depth: 4,
            nodes: Some(200_000),
        },
    )
    .unwrap();
    println!("proved historical attack: {:?}, {:?}", r.result, r.stats);
    assert!(r.result.score.is_some_and(|s| s >= MATE_SCORE - 7));
    assert!(r.stats.quiet_threat_moves > 0 || r.stats.threat_protected_moves > 0);
    replay(&g, &r);
    let mut played = g.clone();
    for choice in r.result.principal_variation {
        if let SearchChoice::Move(a) = choice {
            played.apply(a).unwrap();
        }
    }
    assert_eq!(
        played.outcome(),
        Some(ChessOutcome::Checkmate {
            winner: Color::White
        })
    );
    assert_eq!(g, original);
}

#[test]
fn contact_mate_candidates_preserve_child_cancellation_and_actual_counter_errors() {
    let g = game("7k/8/5KQ1/8/8/8/8/8 w - - 0 1");
    let mut polls = 0;
    let mut stop = || {
        polls += 1;
        polls >= 2
    };
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    let key = e.cursor.key();
    assert!(matches!(
        e.quiescence(0, -INF, INF, Q_PLIES, None),
        Err(Halt::Limit(SearchStatus::Stopped))
    ));
    assert_eq!(
        e.work.nodes, 1,
        "stop at child's poll after native action was admitted"
    );
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.cursor.key(), key);
    assert_eq!(e.eval, EvalState::new(g.position()));
    assert_eq!(e.cursor.depth(), 0);
    let g = game("8/8/8/8/8/5kq1/8/7K b - - 0 4294967295");
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    assert!(matches!(
        e.quiescence(0, -INF, INF, Q_PLIES, None),
        Err(Halt::Error(SearchError::Transition(
            mshogi_core::chess::PlayError::Action(mshogi_core::chess::ActionError::CounterOverflow)
        )))
    ));
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.eval, EvalState::new(g.position()));
    assert_eq!(e.cursor.depth(), 0);
}

#[test]
fn destination_filtered_exchange_matches_previous_full_query_on_native_special_tactics() {
    // Reference retains the prior full-list/filter implementation. Hand-solved
    // SEE cases above and the native target-query fixtures supply independent
    // expectations; this additionally checks unchanged ordinary selection/ties.
    // Promotion branches use the corrected policy for the same selected origin.
    fn previous(p: &Position, a: Action, left: u8) -> i32 {
        let immediate = gain(p, a);
        if left == 0 {
            return immediate;
        }
        let mut after = p.clone();
        if after.apply(a).is_err() {
            return 0;
        }
        let replies: Vec<_> = after
            .legal_actions()
            .into_iter()
            .filter(|r| r.to == a.to)
            .collect();
        let reply = replies.iter().copied().min_by_key(|r| {
            (
                after.piece_at(r.from).map_or(0, |p| {
                    if p.kind == PieceKind::King {
                        20_000
                    } else {
                        piece_value(p.kind)
                    }
                }),
                lexical(*r),
            )
        });
        immediate
            - reply.map_or(0, |r| {
                if r.promotion.is_some() {
                    replies
                        .into_iter()
                        .filter(|candidate| candidate.from == r.from && candidate.to == r.to)
                        .map(|candidate| previous(&after, candidate, left - 1))
                        .max()
                        .unwrap()
                        .max(0)
                } else {
                    previous(&after, r, left - 1).max(0)
                }
            })
    }
    for fen in [
        "r1bq1rk1/ppp2ppp/2np1n2/2b1p3/2B1P3/2NP1N2/PPP2PPP/R1BQ1RK1 w - - 0 1",
        "4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 9",
        "1r5k/2P5/8/8/8/8/8/K7 w - - 0 1",
        "4r2k/8/8/8/8/8/4Rp2/4K3 w - - 0 1",
    ] {
        let p = Position::from_fen(fen).unwrap();
        let before = p.clone();
        let mut stop = || false;
        let mut work = Work {
            nodes: 0,
            limit: None,
            start: Instant::now(),
            time: None,
            stop: &mut stop,
            seldepth: 0,
        };
        for a in p.legal_actions() {
            assert_eq!(
                exchange(&p, a, 16, &mut work).ok(),
                Some(previous(&p, a, 16)),
                "{fen}: {a}"
            );
        }
        assert_eq!(p, before);
    }
}

#[test]
fn cancellation_after_a_completed_iteration_retains_only_that_iteration() {
    let g = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let before = g.clone();
    let mut first_polls = 0;
    let first = search_with_stop(
        &g,
        SearchLimits {
            depth: 1,
            nodes: None,
        },
        || {
            first_polls += 1;
            false
        },
    )
    .unwrap();
    assert_eq!(first.result.completed_depth, 1);
    assert!(first.result.decision.is_some());
    let mut polls = 0;
    let stopped = search_with_stop(
        &g,
        SearchLimits {
            depth: MAX_DEPTH,
            nodes: None,
        },
        || {
            polls += 1;
            polls > first_polls
        },
    )
    .unwrap();
    assert_eq!(stopped.result.status, SearchStatus::Stopped);
    assert_eq!(stopped.result.completed_depth, 1);
    assert_eq!(stopped.result.decision, first.result.decision);
    assert_eq!(stopped.result.score, first.result.score);
    assert_eq!(
        stopped.result.principal_variation,
        first.result.principal_variation
    );
    replay(&g, &stopped);
    assert_eq!(g, before);
}

#[test]
fn full_history_king_attack_completes_and_recognizes_the_losing_mate() {
    // The preserved MSH-139 game is already lost here. Independently checked
    // line after Kh5: Qg4+ permits only Kh6, then Qg6# with revealed Rg1 support.
    let fixtures: serde_json::Value =
        serde_json::from_str(include_str!("../../../docs/m5/evidence/msh141/inputs.json")).unwrap();
    let input = fixtures
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["id"] == "goldfish-loss-v66")
        .unwrap();
    let mut g = AdjudicatedGame::from_fen(STANDARD_START_FEN, ClaimPolicy::AutoClaim).unwrap();
    for a in input["history"].as_array().unwrap() {
        g.apply(a.as_str().unwrap().parse().unwrap()).unwrap();
    }
    let original = g.clone();
    for a in ["g6h5", "c4g4"] {
        g.apply(a.parse().unwrap()).unwrap();
    }
    assert_eq!(g.position().legal_actions(), vec!["h5h6".parse().unwrap()]);
    for a in ["h5h6", "g4g6"] {
        g.apply(a.parse().unwrap()).unwrap();
    }
    assert_eq!(
        g.outcome(),
        Some(ChessOutcome::Checkmate {
            winner: Color::White
        })
    );
    let r = search(
        &original,
        SearchLimits {
            depth: 6,
            nodes: Some(50_000),
        },
    )
    .unwrap();
    assert_eq!(
        r.result.completed_depth, 6,
        "frontier must not monopolize the budget: {:?}",
        r.stats
    );
    assert!(r.result.score.is_some_and(|s| s <= -MATE_BAND));
    replay(&original, &r);
    assert_eq!(r.iterations.last().unwrap().stats, r.stats);
    assert!(r.iterations.iter().all(|i| i.completed));
    assert_eq!(r.result.nodes, r.stats.main_nodes + r.stats.q_nodes);
}

#[test]
fn quiescence_retains_mate_and_check_evasions_beyond_quiet_forcing_frontier() {
    let g = game("8/8/7k/8/6Q1/8/8/K5R1 w - - 0 1");
    for left in [Q_DEFENCE_MIN, Q_DEFENCE_MIN - 1, 0] {
        let mut stop = || false;
        let mut e = engine(&g, &mut stop, SearchMode::Conventional);
        let v = e
            .quiescence(0, -INF, INF, left, None)
            .unwrap_or_else(|_| panic!("bounded mate"));
        assert_eq!(v.score, MATE_SCORE - 1);
        assert_eq!(v.pv, vec![SearchChoice::Move("g4g6".parse().unwrap())]);
        assert_eq!(e.cursor.game(), Some(&g));
        assert_eq!(e.eval, EvalState::new(g.position()));
    }
    // King in check cannot stand pat at an exhausted capture horizon.
    let g = game("8/8/7k/8/7Q/8/8/K7 b - - 0 1");
    assert_eq!(g.position().classify(), Classification::Check);
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    let v = e
        .quiescence(0, -INF, INF, 0, None)
        .unwrap_or_else(|_| panic!("bounded evasion"));
    assert!(!v.pv.is_empty());
    let SearchChoice::Move(a) = v.pv[0] else {
        panic!("expected evasion")
    };
    let mut played = g.clone();
    played.apply(a).unwrap();
    assert!(e.stats.q_check_nodes > 0);
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.eval, EvalState::new(g.position()));
}

#[test]
fn independent_quiet_threat_gets_a_defensive_response_at_the_frontier_boundary() {
    // Qg4! Kh7 (sole reply), Qg7#, supported by Kf6. No immediate mate.
    for (fen, setup, reply, mate) in [
        ("7k/8/5K2/8/5Q2/8/8/8 w - - 0 1", "f4g4", "h8h7", "g4g7"),
        ("8/8/8/5q2/8/5k2/8/7K b - - 0 1", "f5g5", "h1h2", "g5g2"),
    ] {
        let g = game(fen);
        assert!(g.position().queen_contact_mate().is_none());
        let mut proof = g.clone();
        proof.apply(setup.parse().unwrap()).unwrap();
        assert_eq!(
            proof.position().legal_actions(),
            vec![reply.parse().unwrap()]
        );
        proof.apply(reply.parse().unwrap()).unwrap();
        proof.apply(mate.parse().unwrap()).unwrap();
        assert!(matches!(
            proof.outcome(),
            Some(ChessOutcome::Checkmate { .. })
        ));
        for left in [Q_PLIES, Q_FORCING_MIN] {
            let mut stop = || false;
            let mut e = engine(&g, &mut stop, SearchMode::Conventional);
            let v = e
                .quiescence(0, -INF, INF, left, None)
                .unwrap_or_else(|_| panic!("bounded quiet mate"));
            assert_eq!(v.score, MATE_SCORE - 3);
            let mut replay = g.clone();
            for c in v.pv {
                let SearchChoice::Move(a) = c else {
                    panic!("no claims")
                };
                replay.apply(a).unwrap();
            }
            assert!(matches!(
                replay.outcome(),
                Some(ChessOutcome::Checkmate { .. })
            ));
            assert!(e.stats.queen_threat_nodes > 0);
            assert_eq!(e.cursor.game(), Some(&g));
            assert_eq!(e.eval, EvalState::new(g.position()));
        }
    }
}

#[test]
fn frontier_retains_quiet_and_checking_defences_against_contact_mate() {
    // Independent exhaustive python-chess expectation: only Ra7!, Rf8+ and
    // Ra6+ prevent every immediate White mate. Ra7 defends g7 without checking;
    // Rg8 is insufficient because Qh5/Qh4/Qh3 still mate. No global draw claim.
    let g = game("r6k/8/5K2/8/6Q1/8/8/8 b - - 0 1");
    let expected: Vec<Action> = ["a8a7", "a8a6", "a8f8"].map(|a| a.parse().unwrap()).into();
    let mut safe = Vec::new();
    for a in g.position().legal_actions() {
        let mut child = g.clone();
        child.apply(a).unwrap();
        let mut immediate_mate = false;
        for reply in child.position().legal_actions() {
            child.apply(reply).unwrap();
            immediate_mate |= matches!(
                child.outcome(),
                Some(ChessOutcome::Checkmate {
                    winner: Color::White
                })
            );
            child.undo().unwrap();
        }
        if !immediate_mate {
            safe.push(a);
        }
    }
    assert_eq!(safe.len(), expected.len());
    assert!(expected.iter().all(|a| safe.contains(a)));
    // Both the initial frontier and its last defensive layer must reject
    // replacing Qg7# with a different immediate mate.
    for left in [Q_PLIES, Q_DEFENCE_MIN] {
        let mut stop = || false;
        let mut e = engine(&g, &mut stop, SearchMode::Conventional);
        let v = e
            .quiescence(0, -INF, INF, left, None)
            .unwrap_or_else(|_| panic!("bounded defence"));
        let SearchChoice::Move(a) = v.pv[0] else {
            panic!("expected defence")
        };
        assert!(safe.contains(&a), "{a:?} left={left}");
        assert!(e.stats.queen_threat_nodes > 0);
        if left == Q_DEFENCE_MIN {
            assert!(e.stats.defence_reply_probes > 0);
        }
        assert_eq!(e.cursor.game(), Some(&g));
        assert_eq!(e.eval, EvalState::new(g.position()));
    }
}

#[test]
fn defensive_mate_reply_proof_cancels_and_restores_native_state() {
    use std::cell::Cell;
    let g = game("r6k/8/5K2/8/6Q1/8/8/8 b - - 0 1");
    let polls = Cell::new(0);
    let enabled = Cell::new(false);
    let mut stop = || {
        if !enabled.get() {
            return false;
        }
        polls.set(polls.get() + 1);
        polls.get() >= 3
    };
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    e.push_with_key("a8g8".parse().unwrap(), false)
        .unwrap_or_else(|_| panic!("native defence"));
    let before = e.cursor.game().unwrap().clone();
    let eval = e.eval.clone();
    enabled.set(true);
    let r = e.quiescence_with_mate_guard(
        1,
        -INF,
        INF,
        Q_DEFENCE_MIN - 1,
        Some("a8g8".parse().unwrap()),
        true,
    );
    assert!(matches!(r, Err(Halt::Limit(SearchStatus::Stopped))));
    assert!(e.stats.defence_reply_probes > 0);
    assert_eq!(e.cursor.game(), Some(&before));
    assert_eq!(e.eval, eval);
    e.pop();
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.eval, EvalState::new(g.position()));

    // An automatic-policy fifty-move claim comes before this new board probe,
    // even though the side to move has an immediate mating continuation.
    let g = AdjudicatedGame::from_fen("r6k/8/5K2/8/6Q1/8/8/8 b - - 99 1", ClaimPolicy::AutoClaim)
        .unwrap();
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    e.push_with_key("a8g8".parse().unwrap(), false)
        .unwrap_or_else(|_| panic!("native defence"));
    let v = e
        .quiescence_with_mate_guard(
            1,
            -INF,
            INF,
            Q_DEFENCE_MIN - 1,
            Some("a8g8".parse().unwrap()),
            true,
        )
        .unwrap_or_else(|_| panic!("native claim"));
    assert_eq!(v.score, 0);
    assert!(matches!(v.pv.as_slice(), [SearchChoice::Claim(_)]));
    assert_eq!(e.stats.defence_reply_probes, 0);
    e.pop();
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.eval, EvalState::new(g.position()));
}

#[test]
fn endgame_knowledge_remains_below_native_history_claims_and_terminal_authority() {
    let draw = game("8/4k3/8/4K3/4P3/8/8/8 w - - 0 1");
    assert!(draw.outcome().is_none(), "static zero is not adjudication");
    let r = search_with_stop(
        &draw,
        SearchLimits {
            depth: 4,
            nodes: Some(20000),
        },
        || false,
    )
    .unwrap();
    assert_eq!(r.result.score, Some(0));
    assert!(matches!(
        r.result.decision,
        Some(SearchDecision::Choose(SearchChoice::Move(_)))
    ));
    replay(&draw, &r);
    let claim = game("8/4k3/8/4K3/4P3/8/8/8 b - - 100 1");
    assert!(evaluation::evaluate(claim.position()).score_for(Color::Black) < 0);
    let before = claim.clone();
    let r = search_with_stop(
        &claim,
        SearchLimits {
            depth: 4,
            nodes: Some(20000),
        },
        || false,
    )
    .unwrap();
    assert_eq!(r.result.score, Some(0));
    assert!(matches!(
        r.result.decision,
        Some(SearchDecision::Choose(SearchChoice::Claim(_)))
    ));
    assert_eq!(claim, before);
    replay(&claim, &r);
    let terminal = game("8/4k3/8/4K3/4P3/8/8/8 b - - 150 1");
    assert!(matches!(terminal.outcome(), Some(ChessOutcome::Draw(_))));
    let r = search_with_stop(
        &terminal,
        SearchLimits {
            depth: 4,
            nodes: Some(20000),
        },
        || false,
    )
    .unwrap();
    assert_eq!(r.result.score, Some(0));
    assert!(matches!(
        r.result.decision,
        Some(SearchDecision::Finished(ChessOutcome::Draw(_)))
    ));
    let r = search_with_stop(
        &draw,
        SearchLimits {
            depth: 4,
            nodes: Some(20000),
        },
        || true,
    )
    .unwrap();
    assert!(r.result.decision.is_none());
    assert_eq!(r.result.completed_depth, 0);
}

#[test]
fn drawish_conversion_scales_preserve_native_mate_claims_and_cancellation() {
    // Independent Syzygy: two knights are usually drawish, but this board has
    // exactly one winning action, Ng6#. All other legal actions draw. Static
    // material knowledge must not become native dead-position adjudication.
    let mating = game("7k/4NK2/5N2/8/8/8/8/8 w - - 0 1");
    assert_eq!(
        evaluation::evaluate(mating.position()).score_for(Color::White),
        0
    );
    assert!(mating.outcome().is_none());
    let mating_before = mating.clone();
    let r = search_with_stop(
        &mating,
        SearchLimits {
            depth: 2,
            nodes: Some(20000),
        },
        || false,
    )
    .unwrap();
    assert_eq!(
        r.result.decision,
        Some(SearchDecision::Choose(SearchChoice::Move(
            "e7g6".parse().unwrap()
        )))
    );
    assert_eq!(r.result.score, Some(MATE_SCORE - 1));
    replay(&mating, &r);
    assert_eq!(mating, mating_before);
    let mut terminal = mating.clone();
    terminal.apply("e7g6".parse().unwrap()).unwrap();
    assert!(matches!(
        terminal.outcome(),
        Some(ChessOutcome::Checkmate {
            winner: Color::White
        })
    ));
    let r = search(
        &terminal,
        SearchLimits {
            depth: 1,
            nodes: Some(100),
        },
    )
    .unwrap();
    assert_eq!(
        r.result.decision,
        terminal.outcome().map(SearchDecision::Finished)
    );
    assert_eq!(r.result.score, Some(-MATE_SCORE));

    // Full native repetition history still precedes static knowledge and TT.
    let mut repeated =
        AdjudicatedGame::from_fen("8/8/8/8/8/2K5/NN6/7k w - - 0 1", ClaimPolicy::AutoClaim)
            .unwrap();
    for _ in 0..2 {
        for action in ["a2c1", "h1g1", "c1a2", "g1h1"] {
            repeated.apply(action.parse().unwrap()).unwrap();
        }
    }
    let before = repeated.clone();
    let r = search(
        &repeated,
        SearchLimits {
            depth: 2,
            nodes: Some(100),
        },
    )
    .unwrap();
    assert!(matches!(
        r.result.decision,
        Some(SearchDecision::Choose(SearchChoice::Claim(_)))
    ));
    assert_eq!(r.result.score, Some(0));
    replay(&repeated, &r);
    assert_eq!(repeated, before);

    // Automatic fifty-move policy precedes a possible mate-in-one, but never
    // an already concluded checkmate. A time/owner stop fabricates no choice.
    let claimed =
        AdjudicatedGame::from_fen("7k/4NK2/5N2/8/8/8/8/8 w - - 100 1", ClaimPolicy::AutoClaim)
            .unwrap();
    let r = search(
        &claimed,
        SearchLimits {
            depth: 2,
            nodes: Some(100),
        },
    )
    .unwrap();
    assert!(matches!(
        r.result.decision,
        Some(SearchDecision::Choose(SearchChoice::Claim(_)))
    ));
    assert_eq!(r.result.score, Some(0));
    let stopped = search_with_stop(
        &mating,
        SearchLimits {
            depth: 2,
            nodes: None,
        },
        || true,
    )
    .unwrap();
    assert_eq!(stopped.result.completed_depth, 0);
    assert!(stopped.result.decision.is_none());
    assert_eq!(mating, mating_before);
}

#[test]
fn winning_pawn_endings_progress_without_discarding_the_board_win() {
    // Independently confirmed wins: preserve the WDL while both sides choose
    // actual native moves, then reach safe promotion in a bounded journey.
    // This tests conversion integration, not competitive playing strength.
    for fen in [
        "K7/P7/k7/8/8/8/8/8 w - - 0 1",
        "8/4k3/8/4K3/4P3/8/8/8 b - - 0 1",
        "8/k1P5/2K5/8/8/8/8/8 w - - 0 1",
    ] {
        let mut g = game(fen);
        let mut promoted = false;
        for _ in 0..48 {
            assert_eq!(crate::chess_kpk::probe(g.position()), Some(true));
            let before = g.clone();
            let r = search(
                &g,
                SearchLimits {
                    depth: 6,
                    nodes: Some(20000),
                },
            )
            .unwrap();
            assert_eq!(g, before);
            let Some(SearchDecision::Choose(SearchChoice::Move(a))) = r.result.decision else {
                panic!("winning pawn ending did not choose a move: {r:?}");
            };
            g.apply(a).unwrap();
            assert!(!matches!(g.outcome(), Some(ChessOutcome::Draw(_))), "{fen}");
            if a.promotion.is_some() {
                promoted = true;
                assert!(
                    g.position()
                        .legal_actions()
                        .iter()
                        .all(|reply| reply.to != a.to),
                    "promotion must not be immediately capturable: {fen}"
                );
                assert!(matches!(
                    a.promotion,
                    Some(
                        mshogi_core::chess::Promotion::Queen | mshogi_core::chess::Promotion::Rook
                    )
                ));
                break;
            }
        }
        assert!(promoted, "bounded conversion stalled: {fen}");
    }
}

#[test]
fn diagnostics_default_preserves_search_and_exact_mode_ignores_ablations() {
    let g = game(STANDARD_START_FEN);
    let limits = SearchLimits {
        depth: 2,
        nodes: Some(30000),
    };
    let implicit = search(&g, limits).unwrap();
    let explicit = search_with_options(&g, limits, SearchOptions::default(), || false).unwrap();
    assert_eq!(implicit.result, explicit.result);
    assert_eq!(implicit.stats, explicit.stats);
    assert_eq!(implicit.selective_depth, explicit.selective_depth);
    assert_eq!(
        implicit.stats.researches,
        implicit.stats.reduction_researches + implicit.stats.pv_researches
    );
    assert!(implicit.stats.main_first_move_cutoffs <= implicit.stats.main_beta_cutoffs);
    assert!(implicit.stats.q_first_move_cutoffs <= implicit.stats.q_beta_cutoffs);
    assert!(implicit.stats.q_quiet_projections > 0);
    let baseline = exact(&g, 2, 4096);
    let ablated = search_with_options(
        &g,
        limits,
        SearchOptions {
            mode: SearchMode::Exact,
            diagnostics: SearchDiagnostics {
                disable_null: true,
                disable_lmr: true,
                disable_quiet_omissions: true,
                disable_see_pruning: true,
                disable_delta_pruning: true,
                endgame_lmr: true,
            },
            ..SearchOptions::default()
        },
        || false,
    )
    .unwrap();
    assert_eq!(baseline.result, ablated.result);
    assert_eq!(baseline.stats, ablated.stats);
}

#[test]
fn diagnostic_omission_controls_are_independent_and_restore_state() {
    let mut stop = || false;
    let g = game(STANDARD_START_FEN);
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    e.diagnostics.disable_quiet_omissions = true;
    e.main(1, 1, 100, 1000, false, false, 4, None).ok().unwrap();
    assert_eq!(e.stats.late_move_pruned, 0);
    assert_eq!(e.stats.futility_pruned, 0);
    assert_eq!(e.cursor.game(), Some(&g));
    // The established Rxa2/Ra8 exchange and unrecaptured-pawn delta fixtures
    // isolate distinct rejection reasons without changing their native effects.
    for (fen, alpha, beta, see_case) in [
        ("r6k/8/8/8/8/8/p7/R6K w - - 0 1", -INF, INF, true),
        ("7k/8/8/8/8/8/p7/R6K w - - 0 1", 5000, 5001, false),
    ] {
        for disable_matching in [false, true] {
            let g = game(fen);
            let mut e = engine(&g, &mut stop, SearchMode::Conventional);
            e.diagnostics.disable_see_pruning = see_case == disable_matching;
            e.diagnostics.disable_delta_pruning = see_case != disable_matching;
            e.quiescence(0, alpha, beta, Q_PLIES - 2, None)
                .ok()
                .unwrap();
            if disable_matching {
                assert_eq!(e.stats.see_pruned, 0);
                assert!(e.work.nodes > 1);
            } else {
                assert_eq!(e.stats.see_pruned, 1);
                assert_eq!(e.work.nodes, 1);
                assert_eq!(e.stats.q_see_pruned, u64::from(see_case));
                assert_eq!(e.stats.q_delta_pruned, u64::from(!see_case));
            }
            assert_eq!(e.cursor.game(), Some(&g));
            assert_eq!(e.eval, EvalState::new(g.position()));
        }
    }
}

#[test]
fn diagnostic_endgame_lmr_does_not_enable_null_or_omissions() {
    let g = game("4k3/p6p/8/8/8/8/P6P/4K3 w - - 0 1");
    let mut stop = || false;
    for (endgame_lmr, disable_lmr) in [(false, false), (true, false), (true, true)] {
        let mut e = engine(&g, &mut stop, SearchMode::Conventional);
        e.diagnostics.endgame_lmr = endgame_lmr;
        e.diagnostics.disable_lmr = disable_lmr;
        // A non-PV node retains ordinary LMR; shallow PV alternatives now
        // intentionally receive their full depth.
        e.main(3, 0, -INF, INF, false, true, 4, None).ok().unwrap();
        assert_eq!(e.stats.reductions > 0, endgame_lmr && !disable_lmr);
        assert_eq!(e.stats.null_probes, 0);
        assert_eq!(e.stats.late_move_pruned, 0);
        assert_eq!(e.stats.futility_pruned, 0);
        assert_eq!(e.cursor.game(), Some(&g));
        assert_eq!(e.eval, EvalState::new(g.position()));
    }
}

#[test]
fn diagnostic_endgame_lmr_preserves_quiet_mating_threat_protection() {
    // The independently hand-proved Qg4, Kh7 (only legal reply), Qg7# motif
    // has no non-pawn material for Black, so ordinary LMR is disabled.
    let g = game("7k/8/5K2/8/5Q2/8/8/8 w - - 0 1");
    let mut proof = g.clone();
    proof.apply("f4g4".parse().unwrap()).unwrap();
    assert_eq!(
        proof.position().legal_actions(),
        vec!["h8h7".parse().unwrap()]
    );
    proof.apply("h8h7".parse().unwrap()).unwrap();
    proof.apply("g4g7".parse().unwrap()).unwrap();
    assert!(matches!(
        proof.outcome(),
        Some(ChessOutcome::Checkmate { .. })
    ));
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    e.diagnostics.endgame_lmr = true;
    let v = e.main(3, 0, -INF, INF, true, true, 4, None).ok().unwrap();
    assert_eq!(v.score, MATE_SCORE - 3);
    assert!(e.stats.threat_protected_moves > 0);
    assert_eq!(e.stats.null_probes, 0);
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.eval, EvalState::new(g.position()));
}

#[test]
fn deferred_frontier_capture_cutoff_skips_unreached_quiet_projections() {
    // Rxa2 wins an undefended queen with check; Black has only its king.
    // At the last quiet-forcing layer, child nodes have no quiet projections,
    // so these counters isolate work reached in this root's move picker.
    let g = game("k7/8/8/8/8/8/q7/R6K w - - 0 1");
    let capture: Action = "a1a2".parse().unwrap();
    let mut native = g.clone();
    native.apply(capture).unwrap();
    assert!(native.position().in_check());
    assert!(native.outcome().is_none());
    let mut stop = || false;
    for beta in [0, INF] {
        let mut e = engine(&g, &mut stop, SearchMode::Conventional);
        assert!(e.evaluate() < 0, "beta zero cannot be a stand-pat cutoff");
        let key = e.cursor.key();
        let v = e
            .quiescence(0, -INF, beta, Q_FORCING_MIN, None)
            .ok()
            .unwrap();
        assert!(v.score > 0);
        assert_eq!(v.pv.first(), Some(&SearchChoice::Move(capture)));
        if beta == 0 {
            assert_eq!(e.stats.q_quiet_projections, 0);
            assert_eq!(e.stats.q_first_move_cutoffs, 1);
        } else {
            assert!(e.stats.q_quiet_projections > 0);
        }
        assert_eq!(e.cursor.game(), Some(&g));
        assert_eq!(e.cursor.key(), key);
        assert_eq!(e.cursor.depth(), 0);
        assert_eq!(e.eval, EvalState::new(g.position()));
    }
}

#[test]
fn deferred_frontier_reached_quiet_projection_polls_and_preserves_typed_errors() {
    let g = game(STANDARD_START_FEN);
    let count = g.position().legal_actions().len();
    assert_eq!(count, 20);
    // One node poll, one ordering poll per initial quiet move, then one poll
    // before each reached native projection. The initial moves do not check.
    for completed_projections in [0, 1] {
        let mut polls = 0;
        let mut stop = || {
            polls += 1;
            polls == 1 + count + completed_projections + 1
        };
        let mut e = engine(&g, &mut stop, SearchMode::Conventional);
        let key = e.cursor.key();
        assert!(matches!(
            e.quiescence(0, -INF, INF, Q_FORCING_MIN, None),
            Err(Halt::Limit(SearchStatus::Stopped))
        ));
        assert_eq!(e.stats.q_quiet_projections, completed_projections as u64);
        assert_eq!(e.work.nodes, 1);
        assert_eq!(e.cursor.game(), Some(&g));
        assert_eq!(e.cursor.key(), key);
        assert_eq!(e.cursor.depth(), 0);
        assert_eq!(e.eval, EvalState::new(g.position()));
    }
    // Every Black action increments the fullmove counter. A reached quiet
    // candidate must propagate native overflow, never silently skip the action.
    let g = game("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 4294967295");
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    let key = e.cursor.key();
    assert!(matches!(
        e.quiescence(0, -INF, INF, Q_FORCING_MIN, None),
        Err(Halt::Error(SearchError::Transition(
            mshogi_core::chess::PlayError::Action(mshogi_core::chess::ActionError::CounterOverflow)
        )))
    ));
    assert_eq!(e.stats.q_quiet_projections, 1);
    assert_eq!(e.work.nodes, 1);
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.cursor.key(), key);
    assert_eq!(e.cursor.depth(), 0);
    assert_eq!(e.eval, EvalState::new(g.position()));
}

// A native replay plus hand material accounting supplies the expectation. This
// does not select recaptures with SEE's least-attacker/lexical implementation.
fn promotion_recapture_material_proof(
    fen: &str,
    first: &str,
    promotion_prefix: &str,
    recapture: Option<&str>,
) -> AdjudicatedGame {
    fn material(position: &Position, side: Color) -> i32 {
        (0..8)
            .flat_map(|rank| (0..8).map(move |file| Square::new(file, rank).unwrap()))
            .filter_map(|sq| position.piece_at(sq))
            .map(|piece| {
                let value = match piece.kind {
                    PieceKind::Pawn => 100,
                    PieceKind::Knight => 320,
                    PieceKind::Bishop => 330,
                    PieceKind::Rook => 500,
                    PieceKind::Queen => 900,
                    PieceKind::King => 0,
                };
                if piece.color == side { value } else { -value }
            })
            .sum()
    }
    let original = game(fen);
    let side = original.position().side_to_move();
    let before_material = material(original.position(), side);
    let mut child = original.clone();
    child.apply(first.parse().unwrap()).unwrap();
    let promotions: Vec<_> = child
        .position()
        .legal_actions()
        .into_iter()
        .filter(|a| a.to_string().starts_with(promotion_prefix))
        .collect();
    assert_eq!(promotions.len(), 4);
    for (suffix, delta) in [('q', -400), ('r', 0), ('b', 170), ('n', 180)] {
        let action: Action = format!("{promotion_prefix}{suffix}").parse().unwrap();
        assert!(promotions.contains(&action));
        let mut branch = child.clone();
        branch.apply(action).unwrap();
        if let Some(reply) = recapture {
            branch.apply(reply.parse().unwrap()).unwrap();
        }
        assert_eq!(
            material(branch.position(), side) - before_material,
            if recapture.is_some() { 500 } else { delta }
        );
        // All native branches undo exactly, including automatic dead-position
        // conclusions following an unrecaptured bishop/knight promotion.
        if recapture.is_some() {
            branch.undo().unwrap();
        }
        branch.undo().unwrap();
        branch.undo().unwrap();
        assert_eq!(branch, original);
    }
    original
}

#[test]
fn promotion_recapture_see_accounts_for_the_opponents_queen_choice() {
    let g =
        promotion_recapture_material_proof("7k/8/8/8/8/R6K/1p6/q7 w - - 0 1", "a3a1", "b2a1", None);
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    // Rxa1 gains900; bxa1=Q costs500 and creates800: 900-1300=-400.
    assert_eq!(
        exchange(g.position(), "a3a1".parse().unwrap(), 16, &mut e.work).ok(),
        Some(-400)
    );
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.eval, EvalState::new(g.position()));
}

#[test]
fn promotion_recapture_see_is_color_reflection_symmetric() {
    let g =
        promotion_recapture_material_proof("Q7/1P6/r6k/8/8/8/8/7K b - - 0 1", "a6a8", "b7a8", None);
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    assert_eq!(
        exchange(g.position(), "a6a8".parse().unwrap(), 16, &mut e.work).ok(),
        Some(-400)
    );
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.eval, EvalState::new(g.position()));
}

#[test]
fn promotion_recapture_see_does_not_double_count_a_recaptured_promotion() {
    for (fen, first, promotion, reply) in [
        ("7k/8/8/8/R7/R6K/1p6/q7 w - - 0 1", "a3a1", "b2a1", "a4a1"),
        ("Q7/1P6/r6k/r7/8/8/8/7K b - - 0 1", "a6a8", "b7a8", "a5a8"),
    ] {
        let g = promotion_recapture_material_proof(fen, first, promotion, Some(reply));
        let mut stop = || false;
        let mut e = engine(&g, &mut stop, SearchMode::Conventional);
        assert_eq!(
            exchange(g.position(), first.parse().unwrap(), 16, &mut e.work).ok(),
            Some(500)
        );
        assert_eq!(e.cursor.game(), Some(&g));
        assert_eq!(e.eval, EvalState::new(g.position()));
    }
}

#[test]
fn promotion_recapture_see_cancels_before_reply_projection_without_mutation() {
    for (fen, first) in [
        ("7k/8/8/8/8/R6K/1p6/q7 w - - 0 1", "a3a1"),
        ("Q7/1P6/r6k/8/8/8/8/7K b - - 0 1", "a6a8"),
    ] {
        for stop_at in [2, 3] {
            let g = game(fen);
            let original = g.clone();
            let mut polls = 0;
            let mut stop = || {
                polls += 1;
                polls >= stop_at
            };
            let mut e = engine(&g, &mut stop, SearchMode::Conventional);
            let key = e.cursor.key();
            assert!(matches!(
                exchange(g.position(), first.parse().unwrap(), 16, &mut e.work),
                Err(Halt::Limit(SearchStatus::Stopped))
            ));
            assert_eq!(g, original);
            assert_eq!(e.cursor.game(), Some(&g));
            assert_eq!(e.cursor.key(), key);
            assert_eq!(e.cursor.depth(), 0);
            assert_eq!(e.eval, EvalState::new(g.position()));
        }
    }
}
#[test]
fn promotion_recapture_see_negative_exchange_reaches_quiescent_pruning() {
    let g = game("7k/8/8/8/8/R6K/1p6/q7 w - - 0 1");
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    e.diagnostics.disable_delta_pruning = true;
    let key = e.cursor.key();
    e.quiescence(0, -INF, INF, Q_PLIES - 2, None).ok().unwrap();
    assert_eq!(e.stats.q_see_pruned, 1);
    assert_eq!(e.stats.q_delta_pruned, 0);
    assert_eq!(
        e.work.nodes, 1,
        "the losing capture is the only tactical candidate"
    );
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.cursor.key(), key);
    assert_eq!(e.cursor.depth(), 0);
    assert_eq!(e.eval, EvalState::new(g.position()));
}

#[test]
fn diagnostic_null_off_disables_genuine_probes_without_changing_frontier() {
    let g = game(STANDARD_START_FEN);
    let mut stop = || false;
    for disabled in [false, true] {
        let mut e = engine(&g, &mut stop, SearchMode::Conventional);
        let key = e.cursor.key();
        e.diagnostics.disable_null = disabled;
        let v = e
            .main(4, 1, -100001, -100000, false, true, 4, None)
            .ok()
            .unwrap();
        assert!(v.score >= -100000);
        assert_eq!(e.stats.null_probes > 0, !disabled);
        assert_eq!(e.stats.null_verified > 0, !disabled);
        assert!(
            e.stats.q_nodes > 0,
            "quiescence remains enabled in both controls"
        );
        assert_eq!(e.cursor.game(), Some(&g));
        assert_eq!(e.cursor.key(), key);
        assert_eq!(e.cursor.depth(), 0);
        assert_eq!(e.eval, EvalState::new(g.position()));
    }
}

#[test]
fn promotion_recapture_see_two_origins_keep_variant_work_bounded() {
    use std::cell::Cell;
    // Rxd1 wins900; either pawn can promote while capturing that rook.
    // White's second rook could recapture, but the remaining pawn promotes
    // in turn, making that continuation worse than declining it. Losing400
    // is therefore the best material exchange after the initial capture.
    let g = game("7k/8/8/8/3R4/3R3K/2p1p3/3q4 w - - 0 1");
    let polls = Cell::new(0);
    let mut stop = || {
        polls.set(polls.get() + 1);
        false
    };
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    assert_eq!(
        exchange(g.position(), "d3d1".parse().unwrap(), 16, &mut e.work).ok(),
        Some(-400)
    );
    // Root + four first-promotion branches, each with its single rook reply
    // and four second-promotion completions:1+4*(1+1+4)=25 inspected actions.
    assert_eq!(polls.get(), 25);
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.eval, EvalState::new(g.position()));
}

fn msh152_history_fixture(json: &str, id: &str) -> (AdjudicatedGame, serde_json::Value) {
    let inputs: serde_json::Value = serde_json::from_str(json).unwrap();
    let input = inputs
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["id"] == id)
        .unwrap();
    let mut g =
        AdjudicatedGame::from_fen(input["fen"].as_str().unwrap(), ClaimPolicy::AutoClaim).unwrap();
    for action in input["history"].as_array().unwrap() {
        g.apply(action.as_str().unwrap().parse().unwrap()).unwrap();
    }
    assert_eq!(
        g.position().to_fen(),
        input["expected_root_fen"].as_str().unwrap()
    );
    (g, input.clone())
}

#[test]
fn pv_quiet_alternatives_preserve_full_history_invasion_defence() {
    // MSH-152 controlled selectivity experiment: the old PV reduction chose
    // Ke8; full-depth alternatives and the revised PV policy choose Rxf5.
    // This is a reference-supported engineering regression, not a claim that
    // Rxf5 is a proven unique best move or an independent rules oracle.
    let (g, _) = msh152_history_fixture(
        include_str!("../../../docs/m5/evidence/msh152/positions.json"),
        "confirmation-004-1-31-d7e8",
    );
    let before = g.clone();
    let key = SearchCursor::new(&g).key();
    let r = search(
        &g,
        SearchLimits {
            depth: 6,
            nodes: Some(2_000_000),
        },
    )
    .unwrap();
    assert_eq!(r.result.status, SearchStatus::Complete);
    assert_eq!(r.result.completed_depth, 6);
    assert_eq!(
        r.result.principal_variation.first(),
        Some(&SearchChoice::Move("e5f5".parse().unwrap()))
    );
    assert!(
        r.stats.reductions > 0,
        "ordinary non-PV/deeper LMR remains active"
    );
    assert!(r.iterations.iter().all(|i| i.completed));
    replay(&g, &r);
    assert_eq!(g, before);
    assert_eq!(SearchCursor::new(&g).key(), key);
}

#[test]
fn pv_quiet_alternatives_find_independent_published_rook_mating_threat() {
    let (g, input) = msh152_history_fixture(
        include_str!("../../../docs/m5/evidence/msh152/independent-panel.json"),
        "MO2mm",
    );
    let before = g.clone();
    // The preserved independent puzzle publishes Rd5 and this mate-in-three
    // line. Native replay proves this line ends in mate, not that it exhausts
    // every defensive alternative or establishes a unique best move.
    let mut line = g.clone();
    for action in input["expected_line"].as_array().unwrap() {
        line.apply(action.as_str().unwrap().parse().unwrap())
            .unwrap();
    }
    assert_eq!(
        line.outcome(),
        Some(ChessOutcome::Checkmate {
            winner: Color::Black
        })
    );
    let r = search(
        &g,
        SearchLimits {
            depth: 3,
            nodes: Some(2_000_000),
        },
    )
    .unwrap();
    assert_eq!(r.result.status, SearchStatus::Complete);
    assert_eq!(r.result.completed_depth, 3);
    assert_eq!(
        r.result.principal_variation.first(),
        Some(&SearchChoice::Move("d8d5".parse().unwrap()))
    );
    assert!(r.result.score.is_some_and(|score| score >= MATE_BAND));
    replay(&g, &r);
    assert_eq!(g, before);
}

#[test]
fn non_pv_reductions_remain_active_and_cancellation_restores_full_history() {
    let mut g = game(STANDARD_START_FEN);
    for action in ["g1f3", "g8f6", "f3g1", "f6g8"] {
        g.apply(action.parse().unwrap()).unwrap();
    }
    let before = g.clone();
    let mut total_polls = 0;
    {
        let mut stop = || {
            total_polls += 1;
            false
        };
        let mut e = engine(&g, &mut stop, SearchMode::Conventional);
        let key = e.cursor.key();
        e.main(3, 0, -INF, INF, false, false, 4, None).ok().unwrap();
        assert!(e.stats.reductions > 0);
        assert_eq!(e.cursor.game(), Some(&g));
        assert_eq!(e.cursor.key(), key);
        assert_eq!(e.cursor.depth(), 0);
        assert_eq!(e.eval, EvalState::new(g.position()));
    }
    // Interrupt the same real-history search at several interior work points;
    // no partially searched branch may survive in the native cursor/evaluator.
    for cancel_at in [total_polls / 3, total_polls / 2, total_polls - 1] {
        let mut polls = 0;
        let mut stop = || {
            polls += 1;
            polls >= cancel_at
        };
        let mut e = engine(&g, &mut stop, SearchMode::Conventional);
        let key = e.cursor.key();
        assert!(matches!(
            e.main(3, 0, -INF, INF, false, false, 4, None),
            Err(Halt::Limit(SearchStatus::Stopped))
        ));
        assert_eq!(e.cursor.game(), Some(&g));
        assert_eq!(e.cursor.key(), key);
        assert_eq!(e.cursor.depth(), 0);
        assert_eq!(e.eval, EvalState::new(g.position()));
    }
    assert_eq!(g, before);
}

fn threat_cache_fixture() -> AdjudicatedGame {
    let mut g = game("rn3rk1/p4ppp/3bpP2/1Q6/3P4/2q2N2/5PPP/R1B2RK1 b - - 0 15");
    g.apply("c3a1".parse().unwrap()).unwrap();
    g.apply("b5g5".parse().unwrap()).unwrap();
    g
}

// Produce the evidence through an actual non-PV null probe and real search,
// rather than manufacturing the flag being tested.
fn observed_threat_entry(g: &AdjudicatedGame) -> Entry {
    // Independently check the tactical witness: if Black could pass, Qxg7 is
    // checkmate. It is NOT an adjudicated result at the real Black-to-move root.
    assert!(g.outcome().is_none());
    let mut cursor = SearchCursor::new(g);
    assert!(cursor.push_null());
    let mut hypothesis = cursor.position().clone();
    hypothesis.apply("g5g7".parse().unwrap()).unwrap();
    assert_eq!(hypothesis.classify(), Classification::Checkmate);
    assert!(cursor.pop());
    assert_eq!(cursor.game(), Some(g));

    let mut stop = || false;
    let mut e = engine(g, &mut stop, SearchMode::Conventional);
    e.work.limit = Some(100_000);
    assert!(e.main(4, 2, -1, 0, false, true, 1, None).is_ok());
    assert!(e.stats.null_probes > 0);
    assert!(e.stats.threat_extensions > 0);
    assert_eq!(e.cursor.game(), Some(g));
    assert_eq!(e.eval, EvalState::new(g.position()));
    let key = e.cursor.key().unwrap();
    let hash = e.cursor.position_hash();
    let index = e.entry_index(hash, Some(&key), None).unwrap();
    let entry = e.table[index].as_ref().unwrap().clone();
    assert!(entry.key == key && entry.depth == 4 && entry.extensions == 1);
    assert!(entry.mate_threat);
    entry
}

#[test]
fn detected_narrow_window_mate_threat_is_retained_for_compatible_pv_verification() {
    let g = threat_cache_fixture();
    let entry = observed_threat_entry(&g);
    let mut stop = || false;
    let mut e = engine(&g, &mut stop, SearchMode::Conventional);
    e.store(entry, false);
    // One root visit is enough to exercise actual cached-evidence admission.
    // The limit stops at a subsequent node; move ordering may poll/abort before
    // a child is pushed. This tests extension admission, not exact child depth.
    // No new null probe is allowed here.
    e.work.limit = Some(1);
    assert!(matches!(
        e.main(4, 2, -INF, INF, true, false, 1, None),
        Err(Halt::Limit(SearchStatus::NodeLimit))
    ));
    assert_eq!(
        e.stats.threat_extensions, 1,
        "PV verification must retain the compatible narrow search's extra ply"
    );
    assert_eq!(
        e.stats.null_probes, 0,
        "reuse must not manufacture another null probe"
    );
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.eval, EvalState::new(g.position()));
    assert!(
        g.outcome().is_none(),
        "hypothetical mate never becomes a native outcome"
    );
}

#[test]
fn cached_threat_research_rejects_incompatible_depth_credit_history_and_modes() {
    let g = threat_cache_fixture();
    let observed = observed_threat_entry(&g);
    // Same board/counters, different known-history origin. A bound/threat hint
    // cannot manufacture equivalence from board identity alone.
    let imported = game(&g.position().to_fen());
    assert_eq!(imported.position(), g.position());
    for case in [
        "depth",
        "credits",
        "q-entry",
        "history",
        "disabled-null",
        "exact",
        "no-credit",
    ] {
        let root = if case == "history" { &imported } else { &g };
        let mut stop = || false;
        let mode = if case == "exact" {
            SearchMode::Exact
        } else {
            SearchMode::Conventional
        };
        let mut e = engine(root, &mut stop, mode);
        let mut entry = observed.clone();
        if case == "history" {
            assert!(
                entry.key != e.cursor.key().unwrap(),
                "fixture must carry a genuinely different native history context"
            );
        }
        match case {
            "depth" => entry.depth = 5,
            "credits" => entry.extensions = 2,
            "q-entry" => entry.depth = -17,
            _ => {}
        }
        e.diagnostics.disable_null = case == "disabled-null";
        e.store(entry, case == "q-entry");
        e.work.limit = Some(1);
        let credits = if case == "no-credit" { 0 } else { 1 };
        assert!(
            matches!(
                e.main(4, 2, -INF, INF, true, false, credits, None),
                Err(Halt::Limit(SearchStatus::NodeLimit))
            ),
            "{case}"
        );
        assert_eq!(e.stats.threat_extensions, 0, "{case}");
        assert_eq!(e.stats.null_probes, 0, "{case}");
        assert_eq!(e.cursor.game(), Some(root), "{case}");
        assert_eq!(e.eval, EvalState::new(root.position()), "{case}");
    }
}

#[test]
fn cached_threat_admission_preserves_native_outcomes_claims_and_cancellation() {
    let g = threat_cache_fixture();
    let observed = observed_threat_entry(&g);
    for (fen, expected) in [
        ("7k/6Q1/5K2/8/8/8/8/8 b - - 0 1", -MATE_SCORE + 2),
        ("7k/5K2/6Q1/8/8/8/8/8 b - - 0 1", 0),
        ("7k/8/8/8/8/8/8/KR6 w - - 150 1", 0),
    ] {
        let root = game(fen);
        let mut stop = || false;
        let mut e = engine(&root, &mut stop, SearchMode::Conventional);
        let mut entry = observed.clone();
        entry.key = e.cursor.key().unwrap();
        entry.hash = e.cursor.position_hash();
        e.store(entry, false);
        let value = e.main(4, 2, -INF, INF, true, false, 1, None).ok().unwrap();
        assert_eq!(value.score, expected);
        assert_eq!(
            e.stats.threat_extensions, 0,
            "native conclusion precedes cached heuristic"
        );
        assert_eq!(e.cursor.game(), Some(&root));
    }
    let auto = AdjudicatedGame::from_fen("7k/8/8/8/8/8/8/KR6 w - - 100 1", ClaimPolicy::AutoClaim)
        .unwrap();
    let mut stop = || false;
    let mut e = engine(&auto, &mut stop, SearchMode::Conventional);
    let mut entry = observed.clone();
    entry.key = e.cursor.key().unwrap();
    entry.hash = e.cursor.position_hash();
    e.store(entry, false);
    let value = e.main(4, 2, -INF, INF, true, false, 1, None).ok().unwrap();
    assert_eq!(value.score, 0);
    assert!(matches!(value.pv.first(), Some(SearchChoice::Claim(_))));
    assert_eq!(e.stats.threat_extensions, 0);
    assert_eq!(e.cursor.game(), Some(&auto));

    let mut polls = 0;
    let mut cancel = || {
        polls += 1;
        polls >= 2
    };
    let mut e = engine(&g, &mut cancel, SearchMode::Conventional);
    e.store(observed, false);
    let key = e.cursor.key();
    assert!(matches!(
        e.main(4, 2, -INF, INF, true, false, 1, None),
        Err(Halt::Limit(SearchStatus::Stopped))
    ));
    assert_eq!(e.stats.threat_extensions, 1);
    assert_eq!(e.cursor.game(), Some(&g));
    assert_eq!(e.cursor.key(), key);
    assert_eq!(e.eval, EvalState::new(g.position()));
}
