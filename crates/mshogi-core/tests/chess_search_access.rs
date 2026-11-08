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
