//! Search access preserves authoritative Chess outcomes/history and exact state.
use mshogi_core::chess::*;
fn action(text: &str) -> Action {
    text.parse().unwrap()
}
fn game(fen: &str) -> AdjudicatedGame {
    AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap()
}

#[test]
fn standard_queries_and_independent_special_transitions_restore_every_field() {
    let standard = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let mut cursor = SearchCursor::new(&standard);
    assert_eq!(cursor.node().classification, Classification::Quiet);
    assert_eq!(cursor.node().actions.len(), 20);
    assert!(cursor.node().claims.is_empty());
    for (fen, choice, after) in [
        (
            STANDARD_START_FEN,
            "e2e4",
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        ),
        (
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            "e1g1",
            "r3k2r/8/8/8/8/8/8/R4RK1 b kq - 1 1",
        ),
        (
            "7k/8/8/3pP3/8/8/8/KR6 w - d6 0 7",
            "e5d6",
            "7k/8/3P4/8/8/8/8/KR6 b - - 0 7",
        ),
        (
            "7k/P7/8/8/8/8/8/KR6 w - - 0 4",
            "a7a8n",
            "N6k/8/8/8/8/8/8/KR6 b - - 0 4",
        ),
    ] {
        let original = game(fen);
        let mut cursor = SearchCursor::new(&original);
        let before_node = cursor.node().clone();
        let before_key = cursor.key().unwrap();
        let before_hash = cursor.position_hash();
        assert!(before_node.actions.contains(&action(choice)));
        cursor.push(action(choice)).unwrap();
        assert_eq!(cursor.position().to_fen(), after);
        assert_eq!(
            cursor.position_hash(),
            SearchCursor::new(cursor.game().unwrap()).position_hash()
        );
        let mut checked = original.clone();
        checked.apply(action(choice)).unwrap();
        assert_eq!(cursor.game().unwrap(), &checked);
        assert_eq!(cursor.node().outcome, checked.outcome());
        assert_eq!(cursor.node().claims, checked.available_claims());
        assert!(cursor.pop());
        assert_eq!(cursor.game().unwrap(), &original);
        assert_eq!(cursor.position_hash(), before_hash);
        assert_eq!(cursor.key().unwrap(), before_key);
        assert_eq!(cursor.node(), &before_node);
        assert_eq!(cursor.depth(), 0);
        assert!(!cursor.pop());
    }
    assert_eq!(standard, AdjudicatedGame::new(ClaimPolicy::ExplicitOnly));
}

#[test]
fn actual_outcomes_claims_and_representation_errors_keep_native_semantics() {
    for fen in [
        "7k/6Q1/5K2/8/8/8/8/8 b - - 0 1",
        "7k/5K2/6Q1/8/8/8/8/8 b - - 0 1",
        "7k/8/8/8/8/8/8/K7 w - - 0 1",
        "7k/8/8/8/8/8/8/KR6 w - - 150 1",
        "7k/8/8/8/8/8/8/KR6 w - - 99 4294967295",
        "7k/8/8/8/8/8/8/KR6 b - - 99 4294967295",
    ] {
        let original = game(fen);
        let mut cursor = SearchCursor::new(&original);
        assert_eq!(cursor.node().outcome, original.outcome());
        assert_eq!(cursor.node().claims, original.available_claims());
        let key = cursor.key().unwrap();
        let hash = cursor.position_hash();
        for choice in cursor.node().actions.clone() {
            let mut checked = original.clone();
            let expected = checked.apply(choice);
            assert_eq!(cursor.push(choice), expected);
            if expected.is_ok() {
                assert!(cursor.pop());
            }
            assert_eq!(cursor.game().unwrap(), &original);
            assert_eq!(cursor.key().unwrap(), key);
            assert_eq!(cursor.position_hash(), hash);
        }
    }
    // Intended claims ignore the unplayed move's fullmove representability.
    let mut cursor = SearchCursor::new(&game("7k/8/8/8/8/8/8/KR6 b - - 99 4294967295"));
    assert!(
        cursor
            .node()
            .claims
            .iter()
            .any(|c| c.intended == Some(action("h8h7")))
    );
    assert_eq!(
        cursor.push(action("h8h7")),
        Err(PlayError::Action(ActionError::CounterOverflow))
    );
}

#[test]
fn keys_distinguish_imports_policies_counters_modes_and_exact_prefixes() {
    let original = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let a = SearchCursor::new(&original);
    let b = SearchCursor::new(&original.clone());
    assert_eq!(a.key(), b.key());
    assert_eq!(a.key().unwrap().hash64(), b.key().unwrap().hash64());
    for other in [
        game(STANDARD_START_FEN),
        AdjudicatedGame::new(ClaimPolicy::AutoClaim),
        game(&STANDARD_START_FEN.replace("0 1", "1 1")),
        game(&STANDARD_START_FEN.replace("0 1", "0 2")),
        AdjudicatedGame::from_fen_with_mode(
            STANDARD_START_FEN,
            ClaimPolicy::ExplicitOnly,
            AdjudicationMode::Full {
                limits: ReachabilityLimits {
                    max_positions: Some(0),
                    max_transitions: Some(0),
                },
            },
        )
        .unwrap(),
    ] {
        assert_ne!(a.key(), SearchCursor::new(&other).key());
    }
    let mut a = SearchCursor::new(&original);
    let mut b = SearchCursor::new(&original);
    for choice in ["g1f3", "g8f6", "b1c3", "b8c6"] {
        a.push(action(choice)).unwrap();
    }
    for choice in ["b1c3", "b8c6", "g1f3", "g8f6"] {
        b.push(action(choice)).unwrap();
    }
    assert_eq!(a.position(), b.position());
    assert_eq!(a.position_hash(), b.position_hash());
    assert_ne!(
        a.key(),
        b.key(),
        "different full histories never share cached bounds"
    );
}

#[test]
fn irreversible_boundaries_allow_exact_future_context_transpositions() {
    for (fen, first, second, reset) in [
        (
            STANDARD_START_FEN,
            ["g1f3", "g8f6", "b1c3", "b8c6"],
            ["b1c3", "b8c6", "g1f3", "g8f6"],
            "e2e4", // pawn move; earlier pawn ranks cannot recur
        ),
        (
            "rn2k1nr/8/8/8/8/8/8/RN2K1NR w KQkq - 0 1",
            ["g1f3", "g8f6", "b1c3", "b8c6"],
            ["b1c3", "b8c6", "g1f3", "g8f6"],
            "e1f1", // rights loss, despite a nonzero halfmove clock
        ),
        (
            "1n2k1n1/8/8/8/8/2p5/8/1N2K1N1 w - - 0 1",
            ["g1f3", "g8f6", "e1f1", "b8c6"],
            ["e1f1", "b8c6", "g1f3", "g8f6"],
            "b1c3", // capture; the missing pawn cannot be restored
        ),
    ] {
        let original = game(fen);
        let mut a = SearchCursor::new(&original);
        let mut b = SearchCursor::new(&original);
        for text in first {
            a.push(action(text)).unwrap();
        }
        for text in second {
            b.push(action(text)).unwrap();
        }
        assert_eq!(a.position(), b.position());
        assert_ne!(a.key(), b.key(), "different reversible count maps");
        a.push(action(reset)).unwrap();
        b.push(action(reset)).unwrap();
        assert_ne!(a.game(), b.game(), "canonical histories stay complete");
        assert_eq!(a.position(), b.position());
        assert_eq!(
            a.key(),
            b.key(),
            "{reset} makes prior identities unreachable"
        );
        assert_eq!(a.key().unwrap().hash64(), b.key().unwrap().hash64());
        assert_eq!(a.node(), b.node());
        let key = a.key();
        for next in a.node().actions.clone() {
            let prepared = a
                .preview(next)
                .unwrap()
                .expect("claim-free native transposition");
            a.push(next).unwrap();
            b.push_preview(prepared).unwrap();
            assert_eq!(a.key(), b.key());
            assert_eq!(a.node(), b.node());
            assert_ne!(
                a.game(),
                b.game(),
                "prepared effects do not replace canonical histories"
            );
            assert!(a.pop() && b.pop());
            assert_eq!(a.key(), key);
        }
        while a.pop() {}
        while b.pop() {}
        assert_eq!(a.game(), Some(&original));
        assert_eq!(b.game(), Some(&original));
        assert_eq!(a.key(), b.key());
    }
}

#[test]
fn full_mode_keeps_prefix_identity_even_after_an_irreversible_action() {
    let original = AdjudicatedGame::from_fen_with_mode(
        STANDARD_START_FEN,
        ClaimPolicy::ExplicitOnly,
        AdjudicationMode::Full {
            limits: ReachabilityLimits {
                max_positions: Some(0),
                max_transitions: Some(0),
            },
        },
    )
    .unwrap();
    let mut a = SearchCursor::new(&original);
    let mut b = SearchCursor::new(&original);
    for text in ["g1f3", "g8f6", "b1c3", "b8c6", "e2e4"] {
        a.push(action(text)).unwrap();
    }
    for text in ["b1c3", "b8c6", "g1f3", "g8f6", "e2e4"] {
        b.push(action(text)).unwrap();
    }
    assert_eq!(a.position(), b.position());
    assert_ne!(a.key(), b.key(), "Full proof provenance stays conservative");
}

#[test]
fn unkeyed_real_subtrees_preserve_native_history_and_restore_parent_identity() {
    let original = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let mut cursor = SearchCursor::new(&original);
    let key = cursor.key();
    let hash = cursor.position_hash();
    let node = cursor.node().clone();
    assert!(cursor.push_unkeyed(action("e2e5")).is_err());
    assert_eq!(cursor.key(), key, "failed push cannot disable parent key");
    cursor.push_unkeyed(action("e2e4")).unwrap();
    assert!(cursor.key().is_none());
    assert!(!cursor.is_synthetic());
    let mut native = original.clone();
    native.apply(action("e2e4")).unwrap();
    assert_eq!(cursor.game(), Some(&native));
    cursor.push(action("e7e5")).unwrap();
    native.apply(action("e7e5")).unwrap();
    assert!(
        cursor.key().is_none(),
        "ordinary push cannot revive stale key"
    );
    assert_eq!(cursor.game(), Some(&native));
    assert_eq!(cursor.node().claims, native.available_claims());
    assert_eq!(cursor.node().outcome, native.outcome());
    assert!(cursor.pop());
    assert!(cursor.key().is_none());
    assert!(cursor.pop());
    assert_eq!(cursor.game(), Some(&original));
    assert_eq!(cursor.key(), key);
    assert_eq!(cursor.position_hash(), hash);
    assert_eq!(cursor.node(), &node);
}

#[test]
fn lazy_turn_summaries_equal_full_native_semantics_at_sensitive_boundaries() {
    for fen in [
        STANDARD_START_FEN,
        "7k/6Q1/5K2/8/8/8/8/8 b - - 0 1", // mate before thresholds
        "7k/5K2/6Q1/8/8/8/8/8 b - - 0 1", // stalemate, no stand pat
        "7k/8/8/8/8/8/8/K7 w - - 0 1",    // native death
        "7k/8/8/8/8/8/8/KR6 w - - 99 1",  // intended fifty-move
        "7k/8/8/8/8/8/8/KR6 w - - 150 1", // automatic draw
        "7k/8/8/8/8/8/8/KR6 b - - 99 4294967295", // intended claim ignores unplayed overflow
        "4k3/8/8/r4pPK/8/8/8/8 w - f6 0 9", // pinned EP
    ] {
        let original = game(fen);
        let mut cursor = SearchCursor::new(&original);
        let summary = cursor.turn().clone();
        assert_eq!(summary.classification, original.position().classify());
        assert_eq!(summary.outcome, original.outcome());
        assert_eq!(summary.claims, original.available_claims());
        assert_eq!(&cursor.node().turn, &summary);
        assert_eq!(cursor.game(), Some(&original));
    }
    let mut cursor = SearchCursor::new(&game("6nk/8/8/8/8/8/8/KR4N1 w - - 0 1"));
    for text in ["g1f3", "g8f6", "f3g1", "f6g8", "g1f3", "g8h6", "f3g1"] {
        cursor.push(action(text)).unwrap();
        let expected = cursor.game().unwrap().available_claims();
        assert_eq!(cursor.turn().claims, expected);
    }
    assert!(
        cursor
            .turn()
            .claims
            .iter()
            .any(|c| c.intended == Some(action("h6g8")))
    );
}

#[test]
fn prepared_native_transitions_equal_regular_games_and_preserve_sensitive_fallbacks() {
    for fen in [
        STANDARD_START_FEN,
        "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
        "4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 9",
        "7k/2P5/6K1/8/8/8/8/8 w - - 0 1",
        "7k/8/8/8/8/8/8/KR6 w - - 97 1",
    ] {
        let original = game(fen);
        let mut cursor = SearchCursor::new(&original);
        let key = cursor.key();
        let hash = cursor.position_hash();
        for action in original.position().legal_actions() {
            let prepared = cursor.preview(action).unwrap().expect("safe M2 prefix");
            assert_eq!(cursor.game(), Some(&original));
            assert_eq!(cursor.depth(), 0);
            let mut native = original.clone();
            native.apply(action).unwrap();
            assert_eq!(prepared.position(), native.position());
            assert_eq!(prepared.turn().classification, native.position().classify());
            assert_eq!(prepared.turn().outcome, native.outcome());
            assert_eq!(prepared.turn().claims, native.available_claims());
            cursor.push_preview(prepared).unwrap();
            assert_eq!(cursor.game(), Some(&native));
            assert_eq!(cursor.turn().claims, native.available_claims());
            assert_eq!(cursor.turn().outcome, native.outcome());
            assert!(cursor.pop());
            assert_eq!(cursor.key(), key);
            assert_eq!(cursor.position_hash(), hash);
            assert_eq!(cursor.game(), Some(&original));
        }
    }
    let original = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let mut cursor = SearchCursor::new(&original);
    for a in ["g1f3", "g8f6", "f3g1"] {
        cursor.push(action(a)).unwrap();
    }
    // A child's second occurrence does not let its opposite-side intended
    // action claim threefold; exact native catalog confirms the side argument.
    let prepared = cursor.preview(action("f6g8")).unwrap().unwrap();
    cursor.push_preview(prepared).unwrap();
    assert_eq!(cursor.game().unwrap().history().current_occurrences(), 2);
    assert!(cursor.turn().claims.is_empty());
    assert!(cursor.preview(action("e2e4")).unwrap().is_none());
    for clock in [98, 99, 100] {
        let mut cursor = SearchCursor::new(&game(&format!("7k/8/8/8/8/8/8/KR6 w - - {clock} 1")));
        assert!(cursor.preview(action("b1b2")).unwrap().is_none());
    }
    let mut cursor = SearchCursor::new(&original);
    let token = cursor.preview(action("e2e4")).unwrap().unwrap();
    cursor.push(action("d2d4")).unwrap();
    let before = cursor.game().unwrap().clone();
    assert_eq!(
        cursor.push_preview(token),
        Err(PlayError::Action(ActionError::WrongSide))
    );
    assert_eq!(
        cursor.game(),
        Some(&before),
        "stale effects are never committed"
    );
}

#[test]
fn native_repetition_catalog_and_undo_cross_the_first_repeated_identity() {
    let original = game("6nk/8/8/8/8/8/8/KR4N1 w - - 0 1");
    let mut cursor = SearchCursor::new(&original);
    let root = cursor.key().unwrap();
    for text in ["g1f3", "g8f6", "f3g1", "f6g8", "g1f3", "g8h6", "f3g1"] {
        cursor.push(action(text)).unwrap();
        let native = cursor.game().unwrap();
        let claims = native.available_claims();
        let outcome = native.outcome();
        assert_eq!(cursor.node().claims, claims);
        assert_eq!(cursor.node().outcome, outcome);
    }
    assert_eq!(cursor.game().unwrap().history().current_occurrences(), 1);
    assert!(
        cursor
            .node()
            .claims
            .iter()
            .any(|c| c.reason == DrawClaim::Threefold && c.intended == Some(action("h6g8")))
    );
    assert!(
        !cursor.push_null(),
        "claim-bearing turn cannot start a null probe"
    );
    while cursor.pop() {}
    assert_eq!(cursor.game().unwrap(), &original);
    assert_eq!(cursor.key().unwrap(), root);
}

#[test]
fn null_is_a_detached_position_never_a_game_or_history_key() {
    let original = game("4k3/8/8/3pP3/8/8/8/R3K3 w - d6 0 9");
    let record = original.record();
    let mut cursor = SearchCursor::new(&original);
    let summary = cursor.turn().clone();
    assert_eq!(summary.outcome, original.outcome());
    assert_eq!(summary.classification, original.position().classify());
    let cache = cursor.node().clone();
    let key = cursor.key();
    let hash = cursor.position_hash();
    assert!(cursor.push_null());
    assert!(cursor.is_synthetic());
    assert_eq!(
        cursor.position().to_fen(),
        "4k3/8/8/3pP3/8/8/8/R3K3 b - - 0 9"
    );
    assert!(cursor.game().is_none());
    assert!(cursor.key().is_none());
    let expected_classification = cursor.position().classify();
    assert_eq!(cursor.turn().classification, expected_classification);
    assert!(cursor.turn().outcome.is_none());
    assert!(cursor.turn().claims.is_empty());
    assert!(cursor.node().outcome.is_none());
    assert!(cursor.node().claims.is_empty());
    assert!(!cursor.push_null());
    cursor.push(action("e8f8")).unwrap();
    assert_eq!(
        cursor.position().to_fen(),
        "5k2/8/8/3pP3/8/8/8/R3K3 w - - 1 10"
    );
    assert!(cursor.game().is_none());
    assert!(cursor.key().is_none());
    assert!(cursor.pop());
    assert!(cursor.is_synthetic());
    assert!(cursor.pop());
    assert!(!cursor.is_synthetic());
    assert_eq!(cursor.key(), key);
    assert_eq!(cursor.position_hash(), hash);
    assert_eq!(cursor.node(), &cache);
    assert_eq!(cursor.turn(), &summary);
    assert_eq!(cursor.game().unwrap(), &original);
    assert_eq!(
        original.record(),
        record,
        "hypothetical moves never enter canonical history"
    );
    let mut checked = SearchCursor::new(&game("4k3/8/8/8/8/8/4r3/4K3 w - - 0 1"));
    assert!(!checked.push_null());
}

#[test]
fn complete_mode_cached_proof_and_claimed_root_are_preserved() {
    let original = AdjudicatedGame::from_fen_with_mode(
        "7k/8/8/8/8/8/8/KR6 w - - 0 1",
        ClaimPolicy::ExplicitOnly,
        AdjudicationMode::Full {
            limits: ReachabilityLimits {
                max_positions: Some(0),
                max_transitions: Some(0),
            },
        },
    )
    .unwrap();
    let mut cursor = SearchCursor::new(&original);
    let summary = cursor.turn().clone();
    assert_eq!(summary.outcome, original.outcome());
    assert_eq!(summary.claims, original.available_claims());
    let before = cursor.node().clone();
    let key = cursor.key();
    assert!(
        !cursor.push_null(),
        "full graph-adjudication mode never becomes a detached probe"
    );
    cursor.push(action("b1b2")).unwrap();
    assert!(cursor.pop());
    assert_eq!(cursor.game(), Some(&original));
    assert_eq!(cursor.node(), &before);
    assert_eq!(cursor.turn(), &summary);
    assert_eq!(cursor.key(), key);
    let mut claimed = game("7k/8/8/8/8/8/8/KR6 w - - 100 1");
    let claim = claimed.available_claims()[0];
    claimed.claim(claim).unwrap();
    let mut cursor = SearchCursor::new(&claimed);
    assert_eq!(
        cursor.node().outcome,
        Some(ChessOutcome::Draw(DrawReason::Claimed(
            DrawClaim::FiftyMove
        )))
    );
    assert!(cursor.node().claims.is_empty());
    assert!(!cursor.push_null());
    assert_eq!(cursor.push(action("b1b2")), Err(PlayError::GameOver));
    assert_eq!(cursor.game(), Some(&claimed));
}

// Independently enumerate the public validation boundary rather than repeating
// the optimized catalog's projection or repetition prerequisite.
fn public_claim_oracle(game: &AdjudicatedGame) -> Vec<ClaimRequest> {
    let mut actions = game.position().legal_actions();
    actions.sort_by_key(|a| {
        (
            a.from.rank() * 8 + a.from.file(),
            a.to.rank() * 8 + a.to.file(),
            match a.promotion {
                None => 0,
                Some(Promotion::Queen) => 1,
                Some(Promotion::Rook) => 2,
                Some(Promotion::Bishop) => 3,
                Some(Promotion::Knight) => 4,
            },
        )
    });
    let actor = game.position().side_to_move();
    let mut candidates = Vec::new();
    for reason in [DrawClaim::Threefold, DrawClaim::FiftyMove] {
        candidates.push(ClaimRequest {
            actor,
            reason,
            intended: None,
        });
    }
    for reason in [DrawClaim::Threefold, DrawClaim::FiftyMove] {
        for &a in &actions {
            candidates.push(ClaimRequest {
                actor,
                reason,
                intended: Some(a),
            });
        }
    }
    candidates
        .into_iter()
        .filter(|&c| game.history().validate_claim(c).is_ok())
        .collect()
}

#[test]
fn projected_claim_catalog_matches_public_validator_at_boundaries_and_special_moves() {
    for clock in [98, 99, 100, 149, 150] {
        for board in [
            "7k/8/8/8/8/8/8/KR6 b - -",
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq -",
            "7k/P7/8/8/8/8/8/KR6 w - -",
        ] {
            let native = game(&format!("{board} {clock} 4294967295"));
            let expected = public_claim_oracle(&native);
            assert_eq!(native.available_claims(), expected);
            let mut cursor = SearchCursor::new(&native);
            assert_eq!(cursor.turn().claims, expected);
            assert_eq!(cursor.node().claims, expected);
        }
    }
    // Legal EP with an actual zeroing counter, and terminal precedence controls.
    for fen in [
        "7k/8/8/3pP3/8/8/8/KR6 w - d6 0 7",
        "7k/6Q1/5K2/8/8/8/8/8 b - - 100 1",
        "7k/5K2/6Q1/8/8/8/8/8 b - - 100 1",
        "7k/8/8/8/8/8/8/K7 w - - 100 1",
    ] {
        let native = game(fen);
        assert_eq!(native.available_claims(), public_claim_oracle(&native));
    }
    // A real live repeat and fifty-move threshold coexist; intended claims must
    // remain grouped by reason, not interleaved by action.
    let mut native = game("4k1n1/8/8/8/8/8/8/4K1NR w - - 92 1");
    for text in ["g1f3", "g8f6", "f3g1", "f6g8", "g1f3", "g8f6", "f3g1"] {
        native.apply(action(text)).unwrap();
    }
    let expected = public_claim_oracle(&native);
    assert!(
        expected
            .iter()
            .any(|c| c.reason == DrawClaim::Threefold && c.intended == Some(action("f6g8")))
    );
    assert!(expected.iter().any(|c| c.reason == DrawClaim::FiftyMove));
    assert_eq!(native.available_claims(), expected);
    let mut cursor = SearchCursor::new(&native);
    assert_eq!(cursor.turn().claims, expected);
}

#[test]
fn obsolete_repetitions_allow_preview_without_discarding_history_or_live_claims() {
    let mut native = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    for text in ["g1f3", "g8f6", "f3g1", "f6g8", "e2e4"] {
        native.apply(action(text)).unwrap();
    }
    let original = native.clone();
    let mut cursor = SearchCursor::new(&native);
    let token = cursor
        .preview(action("g8f6"))
        .unwrap()
        .expect("obsolete repeat is unreachable");
    native.apply(action("g8f6")).unwrap();
    cursor.push_preview(token).unwrap();
    assert_eq!(cursor.game(), Some(&native));
    assert_eq!(cursor.turn().claims, public_claim_oracle(&native));
    assert!(cursor.pop());
    assert_eq!(cursor.game(), Some(&original));
    // Restore a new live occurrence within the post-pawn-move segment.
    for text in ["g8f6", "g1f3", "f6g8", "f3g1"] {
        cursor.push(action(text)).unwrap();
    }
    assert!(cursor.preview(action("b8c6")).unwrap().is_none());
    for text in ["g8f6", "g1f3", "f6g8"] {
        cursor.push(action(text)).unwrap();
    }
    assert!(
        cursor
            .turn()
            .claims
            .iter()
            .any(|c| c.reason == DrawClaim::Threefold && c.intended == Some(action("f3g1")))
    );
}
