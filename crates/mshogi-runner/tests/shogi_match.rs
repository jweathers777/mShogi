//! Persistent, clocked USI games: native results, unscored stops and replay.
use mshogi_runner::{
    shogi::{FailureKind, Opening},
    shogi_match::{Clock, MatchConfig, MatchReport, MatchStop, run},
    *,
};

fn fixture(mode: &str) -> EngineConfig {
    EngineConfig {
        path: env!("CARGO_BIN_EXE_mshogi-usi-fixture").into(),
        args: vec![mode.into()],
        options: vec![],
        model_path: None,
        model_identity: Some("verification fixture".into()),
        deterministic: true,
    }
}

fn config(engines: [EngineConfig; 2], sente: usize, base_ms: u64) -> MatchConfig {
    MatchConfig {
        schema_version: 1,
        profile: "mshogi.shogi.csa36.v1".into(),
        id: "clocked-fixture".into(),
        engines,
        opening: Opening {
            id: "initial".into(),
            sfen: None,
            moves: vec![],
        },
        sente,
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
fn resignation_and_false_declaration_score_for_the_opponent_by_engine_index() {
    for (mode, sente) in [("resign", 0), ("resign", 1), ("win", 0), ("win", 1)] {
        let report = run(&config([fixture(mode), fixture(mode)], sente, 1000)).unwrap();
        let reason = if mode == "win" {
            "FalseDeclaration"
        } else {
            "Resignation"
        };
        assert!(format!("{:?}", report.conclusion).contains(reason));
        assert_eq!(report.stop, MatchStop::NativeConclusion);
        assert_eq!(report.turns.len(), 1);
        // Sente resigns or declares falsely at once: the Gote engine wins.
        let expected = if sente == 0 { [0, 2] } else { [2, 0] };
        assert_eq!(report.engine_score, Some(expected));
        assert!(
            report.turns[0]
                .go
                .starts_with("go btime 1000 wtime 1000 binc 100 winc 100")
        );
        MatchReport::from_json(&report.to_json().unwrap()).unwrap();
    }
}

#[test]
fn illegal_move_and_silence_are_unscored() {
    let illegal = run(&config([fixture("illegal"), fixture("resign")], 0, 1000)).unwrap();
    assert_eq!(illegal.stop, MatchStop::Failure);
    assert_eq!(
        illegal.failure.as_ref().unwrap().kind,
        FailureKind::IllegalAction
    );
    assert_eq!(illegal.engine_score, None);
    let silent = run(&config([fixture("timeout"), fixture("resign")], 0, 1000)).unwrap();
    assert_eq!(silent.stop, MatchStop::ClockExhausted);
    let exhaustion = silent.exhaustion.as_ref().unwrap();
    assert_eq!((exhaustion.engine, exhaustion.remaining_ms), (0, 1000));
    assert!(exhaustion.elapsed_ms > 1050);
    assert_eq!(silent.engine_score, None);
    MatchReport::from_json(&silent.to_json().unwrap()).unwrap();
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
        args: vec!["usi".into()],
        ..fixture("resign")
    };
    let mut c = config([engine.clone(), engine], 1, 2000);
    c.opening.moves = vec!["7g7f".into(), "3c3d".into()];
    let report = run(&c).unwrap();
    assert_eq!(
        report.stop,
        MatchStop::NativeConclusion,
        "{:?}",
        report.failure
    );
    assert!(report.turns.len() > 10);
    let text = report.to_json().unwrap();
    MatchReport::from_json(&text).unwrap();
    let mut tampered = report.clone();
    tampered.turns[3].elapsed_ms += 1;
    assert!(tampered.replay().is_err());
    let mut tampered = report.clone();
    tampered.engine_score = tampered.engine_score.map(|[a, b]| [b, a]);
    assert!(tampered.replay().is_err());
    let mut tampered = report;
    tampered.turns.pop();
    assert!(tampered.replay().is_err());
}
