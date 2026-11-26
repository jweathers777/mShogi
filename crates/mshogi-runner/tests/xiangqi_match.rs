//! Persistent, clocked UCI Xiangqi games: native results, unscored stops and replay.
#[path = "support/engine.rs"]
mod engine_fixture;
use mshogi_runner::{
    FischerClock as Clock,
    xiangqi::Opening,
    xiangqi_match::{MatchConfig, MatchReport, MatchStop, run},
    *,
};

fn fixture(mode: &str) -> EngineConfig {
    engine_fixture::fixture(env!("CARGO_BIN_EXE_mshogi-xiangqi-fixture"), mode)
}

fn config(engines: [EngineConfig; 2], red: usize, base_ms: u64) -> MatchConfig {
    MatchConfig {
        schema_version: 1,
        profile: "mshogi.xiangqi.wxf2018.v1".into(),
        id: "clocked-fixture".into(),
        engines,
        opening: Opening {
            id: "initial".into(),
            fen: None,
            canonical: None,
            moves: vec![],
        },
        red,
        wire: Default::default(),
        odds: None,
        clock: Clock {
            base_ms,
            increment_ms: 100,
        },
        grace_ms: 50,
        max_game_ms: 120_000,
        watchdogs: Watchdogs {
            handshake_ms: 2000,
            ready_ms: 2000,
            response_ms: 2000,
            write_ms: 500,
            cleanup_ms: 1000,
        },
        io: IoLimits {
            stderr_bytes: 4096,
            command_bytes: 4096,
            ..IoLimits::default()
        },
    }
}

#[test]
fn resignation_scores_for_the_opponent_by_engine_index() {
    for red in [0, 1] {
        let report = run(&config([fixture("resign"), fixture("resign")], red, 1000)).unwrap();
        assert!(format!("{:?}", report.conclusion).contains("Resign"));
        assert_eq!(report.stop, MatchStop::NativeConclusion);
        assert_eq!(report.turns.len(), 1);
        // Red resigns at once: the Black engine wins.
        let expected = if red == 0 { [0, 2] } else { [2, 0] };
        assert_eq!(report.engine_score, Some(expected));
        assert_eq!(
            report.turns[0].go,
            "go wtime 1000 btime 1000 winc 100 binc 100"
        );
        MatchReport::from_json(&report.to_json().unwrap()).unwrap();
    }
}

#[test]
fn a_full_mshogi_game_replays_and_tampering_is_rejected() {
    let mshogi =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/release/mshogi");
    if !mshogi.exists() {
        eprintln!("release mshogi not built; skipping full game");
        return;
    }
    let engine = EngineConfig {
        path: mshogi.to_str().unwrap().into(),
        args: vec!["--game".into(), "xiangqi".into(), "uci".into()],
        ..fixture("resign")
    };
    let mut c = config([engine.clone(), engine], 1, 2000);
    c.opening.moves = vec!["h2e2".into(), "h9g7".into()];
    let report = run(&c).unwrap();
    assert!(
        matches!(
            report.stop,
            MatchStop::NativeConclusion | MatchStop::Unresolved
        ),
        "{:?} {:?} {:?} {:?}",
        report.stop,
        report.failure,
        report.exhaustion,
        report
            .turns
            .iter()
            .map(|t| (t.clock_before_ms, t.elapsed_ms))
            .collect::<Vec<_>>()
    );
    assert!(report.turns.len() > 10);
    let text = report.to_json().unwrap();
    MatchReport::from_json(&text).unwrap();
    let mut tampered = report.clone();
    tampered.turns[3].elapsed_ms += 1;
    assert!(tampered.replay().is_err());
    let mut tampered = report.clone();
    tampered.turns[2].reply = "bestmove a0a1".into();
    assert!(tampered.replay().is_err());
    let mut tampered = report;
    tampered.turns.pop();
    assert!(tampered.replay().is_err());
}

#[test]
fn time_odds_give_each_engine_its_own_clock_and_replay_checks_them() {
    // Engine 1 resigns at once; with engine 0 Red the first go carries Red's
    // (engine 0) and Black's (engine 1) own clocks and increments.
    let mut c = config([fixture("resign"), fixture("resign")], 1, 1000);
    c.odds = Some([
        Clock {
            base_ms: 2000,
            increment_ms: 50,
        },
        Clock {
            base_ms: 1000,
            increment_ms: 20,
        },
    ]);
    let report = run(&c).unwrap();
    // Engine 1 plays Red: Red has 1000 + 20, Black (engine 0) 2000 + 50.
    assert_eq!(
        report.turns[0].go,
        "go wtime 1000 btime 2000 winc 20 binc 50"
    );
    MatchReport::from_json(&report.to_json().unwrap()).unwrap();
    let mut tampered = report;
    tampered.config.odds = None;
    assert!(tampered.replay().is_err());
}

include!("support/clocked_cases.rs");
clocked_failure_cases!("xiangqi-match");

#[test]
fn null_move_is_unscored() {
    let null = run(&config([fixture("null"), fixture("resign")], 0, 1000)).unwrap();
    assert_eq!(null.stop, MatchStop::Failure);
    assert_eq!(
        null.failure.as_ref().unwrap().kind,
        FailureKind::MalformedReply
    );
}
