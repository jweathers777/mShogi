//! All adapters use the same state and timing boundary. These tests establish
//! reset and deterministic isolation; elapsed wall-time measurements are separate.
use mshogi_search::alphabeta::timing::TimeBudget;

macro_rules! state_case {
    ($test:ident, $adapter:ident, $game:expr) => {
        state_case!($test, $adapter, $game, false);
    };
    ($test:ident, $adapter:ident, $game:expr, $fallback:expr) => {
        #[test]
        fn $test() {
            use mshogi_search::$adapter as adapter;
            let game = $game;
            let limits = adapter::Limits {
                depth: 2,
                nodes: 20_000,
            };
            let mut engine = adapter::new_engine();
            let stopped = adapter::search_with_state(
                &game,
                limits,
                Some(TimeBudget::from_hard_ms(0)),
                &mut engine,
                || false,
            )
            .unwrap();
            assert_eq!(stopped.nodes, 0);
            assert_eq!(stopped.completed_depth, 0);
            assert_eq!(stopped.decision.is_some(), $fallback);
            assert!(stopped.score.is_none());
            let fresh = adapter::search(&game, limits).unwrap();
            let timed = adapter::search_with_state(
                &game,
                limits,
                Some(TimeBudget::from_hard_ms(60_000)),
                &mut engine,
                || false,
            )
            .unwrap();
            assert_eq!(
                timed, fresh,
                "a nonbinding timer must preserve search output"
            );
            let warm = adapter::search_with_state(
                &game,
                limits,
                Some(TimeBudget::from_hard_ms(60_000)),
                &mut engine,
                || false,
            )
            .unwrap();
            assert_ne!(
                warm.nodes, fresh.nodes,
                "these roots must expose reuse of the caller-owned table"
            );
            assert_eq!(
                adapter::search(&game, limits).unwrap(),
                fresh,
                "a fixed request must not use another request's state"
            );
            engine.clear();
            let reset =
                adapter::search_with_state(&game, limits, None, &mut engine, || false).unwrap();
            assert_eq!(reset, fresh, "reset must discard both table and statistics");
        }
    };
}
state_case!(
    tori_state_and_time,
    tori,
    mshogi_core::tori::Game::standard()
);
state_case!(
    shogi_state_and_time,
    shogi,
    mshogi_core::shogi::Game::new(),
    true
);
state_case!(
    xiangqi_state_and_time,
    xiangqi,
    mshogi_core::xiangqi::Game::standard(mshogi_core::xiangqi::ClaimPolicy::Explicit).unwrap(),
    true
);
state_case!(chu_state_and_time, chu, mshogi_core::chu::Game::standard());
state_case!(
    tenjiku_adams_state_and_time,
    tenjiku,
    mshogi_core::tenjiku::Game::standard(mshogi_core::tenjiku::Profile::AdamsModern)
);
state_case!(
    tenjiku_muller_state_and_time,
    tenjiku,
    mshogi_core::tenjiku::Game::standard(mshogi_core::tenjiku::Profile::MullerCompleted)
);
