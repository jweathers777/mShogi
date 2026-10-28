//! Actual native-owned protocol admission, bounded failures and independent history expectations.
use mshogi_core::{native::CanonicalRecord, xiangqi as x};
use mshogi_runner::{
    xiangqi::{self, FailureKind, Stop},
    *,
};
fn engine(mode: &str) -> EngineConfig {
    EngineConfig {
        path: env!("CARGO_BIN_EXE_mshogi-xiangqi-fixture").into(),
        args: vec![mode.into()],
        options: vec![],
        model_path: None,
        model_identity: Some("verification fixture".into()),
        deterministic: true,
    }
}
fn config(mode: &str) -> xiangqi::Config {
    xiangqi::Config {
        schema_version: 1,
        profile: x::PROFILE_ID.into(),
        dialect: "pikafish_uci0_9".into(),
        claim_policy: x::ClaimPolicy::AutoValidClaims,
        id: "native-xiangqi-fixture".into(),
        time_control: "untimed".into(),
        process_policy: "fresh_per_turn".into(),
        engines: [engine(mode), engine(mode)],
        openings: vec![xiangqi::Opening {
            id: "initial".into(),
            fen: None,
            canonical: None,
            moves: vec![],
        }],
        paired: true,
        work: [WorkLimit {
            depth: Some(1),
            nodes: Some(128),
            movetime_ms: None,
        }; 2],
        max_actions: 1,
        max_game_ms: 5000,
        watchdogs: Watchdogs {
            handshake_ms: 1000,
            ready_ms: 1000,
            response_ms: 500,
            write_ms: 500,
            cleanup_ms: 500,
        },
        io: IoLimits {
            stderr_bytes: 4096,
            ..IoLimits::default()
        },
    }
}
#[test]
fn resignation_is_native_same_ply_and_clean_protocol_replays_without_processes() {
    let report = xiangqi::run(&config("resign")).unwrap();
    assert_eq!(report.games.len(), 2);
    for e in &report.games {
        assert_eq!(e.stop, Stop::NativeConclusion);
        assert_eq!(e.score, Some([0, 2]));
        assert_eq!(e.turns.len(), 1);
        assert!(e.failure.is_none());
        assert!(e.traffic_complete);
        assert!(
            e.processes
                .iter()
                .all(|p| p.observation.exit.as_ref().unwrap().success
                    && p.observation.cleanup_failures.is_empty())
        );
    }
    for game in report.replay().unwrap() {
        assert_eq!(
            x::GameRecord::from_game(&game)
                .unwrap()
                .known_board_actions(),
            0
        );
        assert_eq!(game.events().len(), 2);
        assert_eq!(game.position().side_to_move(), x::Side::Red);
    }
    assert_eq!(
        xiangqi::Report::from_json(&report.to_json().unwrap()).unwrap(),
        report
    );
}
#[test]
fn malformed_illegal_missing_crash_null_duplicate_and_incomplete_stay_unscored() {
    for (mode, kind) in [
        ("win", FailureKind::MalformedReply),
        ("malformed", FailureKind::MalformedReply),
        ("illegal", FailureKind::IllegalAction),
        ("missing", FailureKind::MissingReply),
        ("crash", FailureKind::UnexpectedExit),
        ("null", FailureKind::MalformedReply),
        ("duplicate", FailureKind::LateReply),
        ("oversize", FailureKind::ResourceLimit),
        ("incomplete", FailureKind::IncompleteWork),
        ("unresolved", FailureKind::UnresolvedWork),
    ] {
        let report = xiangqi::run(&config(mode)).unwrap();
        for e in &report.games {
            assert_eq!(e.stop, Stop::Failure, "{mode}");
            assert_eq!(
                e.failure.as_ref().unwrap().kind,
                kind,
                "{mode}: {:?}",
                e.failure
            );
            assert!(e.score.is_none() && e.conclusion.is_none() && e.turns.is_empty());
            assert!(e.processes.iter().all(|p| p.observation.exit.is_some()));
        }
        report.replay().unwrap();
    }
}
#[test]
fn watchdog_cancels_and_retires_late_result_then_reaps_child() {
    for mode in ["timeout", "late", "hungquit", "post_crash"] {
        let mut c = config(mode);
        c.watchdogs.response_ms = 20;
        c.watchdogs.cleanup_ms = 300;
        let report = xiangqi::run(&c).unwrap();
        for e in &report.games {
            assert!(e.turns.is_empty() && e.score.is_none());
            assert_eq!(e.stop, Stop::Failure);
            assert!(e.processes.iter().all(|p| p.observation.exit.is_some()));
            if mode == "late" {
                assert!(
                    e.traffic
                        .iter()
                        .any(|t| t.phase == "retired" && t.text == "bestmove a3a4")
                );
            }
        }
    }
}
#[test]
fn actual_action_preserves_full_warned_history_and_native_fourth_occurrence() {
    let mut c = config("move");
    for engine in &mut c.engines {
        engine.args.push("e8f8".into());
    }
    c.openings[0].fen = Some("4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1".into());
    c.openings[0].moves = ["e9f9", "f8e8", "f9e9", "e8f8"].repeat(3)[..11]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let report = xiangqi::run(&c).unwrap();
    for e in &report.games {
        assert_eq!(e.stop, Stop::NativeConclusion);
        assert_eq!(e.score, Some([0, 2]));
        assert!(
            e.turns[0]
                .position
                .ends_with(&c.openings[0].moves.join(" "))
        );
    }
    for game in report.replay().unwrap() {
        assert!(game.warnings().iter().any(|w| w.active));
        assert!(matches!(
            game.conclusion(),
            Some(x::Conclusion::Win {
                winner: x::Side::Black,
                reason: x::WinReason::Repetition { .. }
            })
        ));
    }
}
#[test]
fn claimant_specific_auto_claim_concludes_before_any_child_is_started() {
    let f: serde_json::Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/xiangqi/natural-count.json"
    ))
    .unwrap();
    let f = f["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == "NATURAL-11-CHECKS")
        .unwrap();
    let mut c = config("crash");
    c.openings[0].fen = Some(f["root_fen"].as_str().unwrap().into());
    c.openings[0].moves = f["moves"]
        .as_array()
        .unwrap()
        .iter()
        .take(100)
        .map(|v| v.as_str().unwrap().into())
        .collect();
    let report = xiangqi::run(&c).unwrap();
    for e in &report.games {
        assert_eq!(e.stop, Stop::NativeConclusion);
        assert_eq!(e.score, Some([1, 1]));
        assert!(e.processes.is_empty() && e.turns.is_empty());
    }
    let games = report.replay().unwrap();
    assert_eq!(
        games[0]
            .natural_count_eligibility(x::Side::Red)
            .effective_plies,
        99
    );
    assert_eq!(
        games[0]
            .natural_count_eligibility(x::Side::Black)
            .effective_plies,
        100
    );
}
#[test]
fn unsupported_policy_dialects_limits_and_live_nonboard_history_reject_losslessly() {
    let base = config("resign");
    for kind in 0..5 {
        let mut c = base.clone();
        match kind {
            0 => c.claim_policy = x::ClaimPolicy::Explicit,
            1 => c.dialect = "fairy_generic_uci1_10".into(),
            2 => c.work[0].nodes = Some(513),
            3 => c.work[1].movetime_ms = Some(1),
            _ => c.time_control = "timed".into(),
        };
        assert!(c.validate().is_err());
    }
    let mut game = x::Game::standard(x::ClaimPolicy::AutoValidClaims).unwrap();
    game.submit(x::GameRequest {
        expected: game.context(),
        actor: x::Side::Red,
        command: x::GameCommand::ClaimNaturalCount,
    })
    .unwrap();
    let canonical = CanonicalRecord::xiangqi(x::GameRecord::from_game(&game).unwrap())
        .to_json()
        .unwrap();
    let mut c = base;
    c.openings[0].canonical = Some(canonical.clone());
    assert!(
        c.validate()
            .unwrap_err()
            .to_string()
            .contains("non-board command history")
    );
    assert_eq!(
        x::GameRecord::from_game(&game)
            .unwrap()
            .known_board_actions(),
        0
    );
    // Terminal records retain those audit commands exactly and need no lossy dialect conversion.
    game.submit(x::GameRequest {
        expected: game.context(),
        actor: x::Side::Black,
        command: x::GameCommand::Resign,
    })
    .unwrap();
    c.openings[0].canonical = Some(
        CanonicalRecord::xiangqi(x::GameRecord::from_game(&game).unwrap())
            .to_json()
            .unwrap(),
    );
    let report = xiangqi::run(&c).unwrap();
    assert!(report.games.iter().all(|e| e.turns.is_empty()));
    assert_eq!(report.replay().unwrap()[0].events(), game.events());
    let clean_game = x::Game::standard(x::ClaimPolicy::AutoValidClaims).unwrap();
    let clean = CanonicalRecord::xiangqi(x::GameRecord::from_game(&clean_game).unwrap())
        .to_json()
        .unwrap();
    for token in [
        "\"game\":\"xiangqi\"",
        "\"no_win_mode\":\"sound_fast_only\"",
    ] {
        let duplicate = clean.replacen(token, &format!("{token},{token}"), 1);
        assert_ne!(duplicate, clean);
        c.openings[0].canonical = Some(duplicate);
        assert!(c.validate().is_err());
    }
    c.openings[0].canonical = None;
    for fen in [
        "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 120 1",
        "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 100001",
    ] {
        c.openings[0].fen = Some(fen.into());
        assert!(
            c.validate()
                .unwrap_err()
                .to_string()
                .contains("unsupported imported root counters")
        );
    }
}
#[test]
fn options_retention_and_transcript_or_native_record_tampering_are_rejected() {
    for (name, value) in [
        ("Unknown", "1"),
        ("Nodes", "513"),
        ("RulesProfile", "wrong"),
    ] {
        let mut c = config("resign");
        for e in &mut c.engines {
            e.options.push(UciOption {
                name: name.into(),
                value: value.into(),
            });
        }
        let report = xiangqi::run(&c).unwrap();
        assert!(report.games.iter().all(|e| e.failure.as_ref().unwrap().kind
            == FailureKind::Configuration
            && e.turns.is_empty()));
    }
    let report = xiangqi::run(&config("resign")).unwrap();
    for kind in 0..7 {
        let mut bad = report.clone();
        let e = &mut bad.games[0];
        match kind {
            0 => e.turns[0].context_sha256.push('0'),
            1 => e.turns[0].version += 1,
            2 => e.turns[0].generation = 0,
            3 => e.turns[0].go = "go depth 3 nodes 512".into(),
            4 => e.traffic.iter_mut().find(|t| t.text == "uci").unwrap().text = "usi".into(),
            5 => e.game = e.game.replace("mshogi.xiangqi.wxf2018.v1", "wrong"),
            _ => e.score = Some([2, 0]),
        };
        assert!(bad.replay().is_err(), "tampering {kind}");
    }
    let mut c = config("flood");
    c.io.transcript_bytes = 1000;
    let report = xiangqi::run(&c).unwrap();
    assert!(report.games.iter().all(|e| e.failure.as_ref().unwrap().kind
        == FailureKind::ResourceLimit
        && e.turns.is_empty()
        && !e.traffic_complete));
}

#[test]
fn accepted_generation_replay_rejects_retired_duplicate_and_reordered_responses() {
    let report = xiangqi::run(&config("resign")).unwrap();
    for kind in 0..6 {
        let mut bad = report.clone();
        let game = &mut bad.games[0];
        match kind {
            0 => {
                let mut late = game.traffic.last().unwrap().clone();
                late.direction = "response".into();
                late.phase = "retired".into();
                late.text = "bestmove a3a4".into();
                late.written = None;
                game.traffic.push(late);
            }
            1 => {
                let mut duplicate = game
                    .traffic
                    .iter()
                    .find(|e| e.text == "bestmove resign")
                    .unwrap()
                    .clone();
                duplicate.at_ns = game.traffic.last().unwrap().at_ns;
                game.traffic.push(duplicate);
            }
            2 => {
                game.traffic
                    .iter_mut()
                    .find(|e| e.text == "uciok")
                    .unwrap()
                    .phase = "search".into()
            }
            3 => {
                game.traffic
                    .iter_mut()
                    .find(|e| e.text == "readyok")
                    .unwrap()
                    .text = "uciok".into()
            }
            4 => {
                game.traffic
                    .iter_mut()
                    .find(|e| e.text == "uciok")
                    .unwrap()
                    .direction = "unknown".into()
            }
            _ => {
                game.traffic
                    .iter_mut()
                    .find(|e| e.text == "uciok")
                    .unwrap()
                    .phase = "unknown".into()
            }
        }
        assert!(bad.replay().is_err(), "contradictory transcript {kind}");
    }
}

#[test]
fn replay_requires_actual_supported_advertisement_for_each_configured_option() {
    let mut c = config("resign");
    for e in &mut c.engines {
        e.options.push(UciOption {
            name: "Nodes".into(),
            value: "128".into(),
        });
    }
    let report = xiangqi::run(&c).unwrap();
    for invalid in [false, true] {
        let mut bad = report.clone();
        let events = &mut bad.games[0].traffic;
        if invalid {
            events
                .iter_mut()
                .find(|e| e.text.starts_with("option name Nodes "))
                .unwrap()
                .text = "option name Nodes type spin default 0 min 0 max 127".into();
        } else {
            events.retain(|e| !e.text.starts_with("option name Nodes "));
        }
        assert!(
            bad.replay().is_err(),
            "missing/invalid advertisement must reject like runtime"
        );
    }
}
