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
        table: vec![None; 4096],
        history: vec![0; 8192],
        continuation: vec![0; 4096],
        captures: vec![0; 768],
        killers: vec![[None; 2]; 129],
        stats: SearchStats::default(),
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
        key: key.clone(),
        hash,
        depth: 0,
        extensions: 4,
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
        key: foreign,
        hash,
        depth: 1,
        extensions: 4,
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
