//! Persistent, clocked USI games: engine results, unscored stops and replay.
#[path = "support/engine.rs"]
mod engine_fixture;
use mshogi_runner::{
    shogi::{FailureKind, Opening},
    shogi_match::{Clock, MatchConfig, MatchReport, MatchStop, run},
    *,
};

fn fixture(mode: &str) -> EngineConfig {
    engine_fixture::fixture(env!("CARGO_BIN_EXE_mshogi-usi-fixture"), mode)
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
        assert_eq!(report.stop, MatchStop::GameConclusion);
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
fn a_scripted_game_replays_and_tampering_is_rejected() {
    // Two pawn moves, then three four-ply King cycles: the opening position
    // occurs at plies 2, 6, 10 and 14. Every reply and the draw are fixed;
    // short-clock self-play cannot promise either a completed search or game.
    let script = [
        "7g7f", "3c3d", "5i6h", "5a6b", "6h5i", "6b5a", "5i6h", "5a6b", "6h5i", "6b5a", "5i6h",
        "5a6b", "6h5i", "6b5a",
    ];
    let mut engine = fixture("script");
    engine.args.extend(script.iter().map(|s| s.to_string()));
    let mut c = config([engine.clone(), engine], 1, 60_000);
    c.opening.moves = script[..2].iter().map(|s| s.to_string()).collect();
    let report = run(&c).unwrap();
    assert_eq!(report.stop, MatchStop::GameConclusion, "{report:#?}");
    assert_eq!(report.turns.len(), 12);
    assert_eq!(report.engine_score, Some([1, 1]));
    assert_eq!(
        report.conclusion,
        Some(mshogi_core::shogi::Conclusion::Draw(
            mshogi_core::shogi::DrawReason::Fourfold {
                first: 2,
                fourth: 14
            }
        ))
    );
    assert!(report.traffic_complete);
    assert_eq!(
        report
            .turns
            .iter()
            .map(|t| t.reply.as_str())
            .collect::<Vec<_>>(),
        script[2..]
            .iter()
            .map(|s| format!("bestmove {s}"))
            .collect::<Vec<_>>()
    );
    let text = report.to_json().unwrap();
    MatchReport::from_json(&text).unwrap();
    let mut tampered = report.clone();
    tampered.turns[3].elapsed_ms += 1;
    assert!(tampered.replay().is_err());
    let mut tampered = report.clone();
    // Swapping [1, 1] does not change a draw report. This must actually
    // contradict the reported score for both draws and decisive conclusions.
    tampered.engine_score = Some([0, 0]);
    assert!(tampered.replay().is_err());
    let mut tampered = report;
    tampered.turns.pop();
    assert!(tampered.replay().is_err());
}

#[test]
fn actual_mshogi_mate_in_one_has_a_conclusion() {
    let mshogi =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/release/mshogi");
    if !mshogi.exists() {
        eprintln!("release mshogi not built; skipping actual engine mate");
        return;
    }
    let engine = EngineConfig {
        path: mshogi.to_str().unwrap().into(),
        args: vec!["usi".into()],
        ..fixture("resign")
    };
    let mut c = config([engine.clone(), engine], 1, 5_000);
    // Independently established fixture also used by console USI/search tests:
    // G*5b checks the boxed-in King, and Sente's King protects the dropped Gold.
    c.opening.sfen = Some("3lkl3/3p1p3/4K4/9/9/9/9/9/9 b G 1".into());
    let report = run(&c).unwrap();
    assert_eq!(report.stop, MatchStop::GameConclusion, "{report:#?}");
    assert_eq!(report.turns.len(), 1);
    assert_eq!(report.turns[0].reply, "bestmove G*5b");
    assert_eq!(report.engine_score, Some([0, 2]));
    MatchReport::from_json(&report.to_json().unwrap()).unwrap();
}

include!("support/clocked_cases.rs");
clocked_failure_cases!("shogi-match");
