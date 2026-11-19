//! Persistent, clocked UCI Xiangqi games: native results, unscored stops and replay.
use mshogi_runner::{
    shogi::FailureKind,
    shogi_match::Clock,
    xiangqi::Opening,
    xiangqi_match::{MatchConfig, MatchReport, MatchStop, run},
    *,
};

fn fixture(mode: &str) -> EngineConfig {
    EngineConfig {
        path: env!("CARGO_BIN_EXE_mshogi-xiangqi-fixture").into(),
        args: vec![mode.into()],
        options: vec![],
        model_path: None,
        model_identity: Some("verification fixture".into()),
        deterministic: true,
    }
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
fn illegal_move_null_move_and_silence_are_unscored() {
    let illegal = run(&config([fixture("illegal"), fixture("resign")], 0, 1000)).unwrap();
    assert_eq!(illegal.stop, MatchStop::Failure);
    assert_eq!(
        illegal.failure.as_ref().unwrap().kind,
        FailureKind::IllegalAction
    );
    assert_eq!(illegal.engine_score, None);
    let null = run(&config([fixture("null"), fixture("resign")], 0, 1000)).unwrap();
    assert_eq!(null.stop, MatchStop::Failure);
    assert_eq!(
        null.failure.as_ref().unwrap().kind,
        FailureKind::MalformedReply
    );
    let silent = run(&config([fixture("timeout"), fixture("resign")], 0, 1000)).unwrap();
    assert_eq!(silent.stop, MatchStop::ClockExhausted);
    let exhaustion = silent.exhaustion.as_ref().unwrap();
    assert_eq!((exhaustion.engine, exhaustion.remaining_ms), (0, 1000));
    assert!(exhaustion.elapsed_ms > 1050);
    assert_eq!(silent.engine_score, None);
    MatchReport::from_json(&silent.to_json().unwrap()).unwrap();
}

#[test]
fn the_game_watchdog_is_a_time_limit_not_a_clock_exhaustion() {
    let mut c = config([fixture("timeout"), fixture("resign")], 0, 60_000);
    c.max_game_ms = 1_000;
    let report = run(&c).unwrap();
    assert_eq!(report.stop, MatchStop::TimeLimit);
    assert!(report.exhaustion.is_none() && report.engine_score.is_none());
    MatchReport::from_json(&report.to_json().unwrap()).unwrap();
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
