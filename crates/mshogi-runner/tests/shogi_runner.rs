//! Actual USI child admission/failure/replay. Expected outcomes use accepted native fixtures.
use mshogi_runner::{
    shogi::{self, FailureKind, Stop},
    *,
};
fn engine(mode: &str) -> EngineConfig {
    EngineConfig {
        path: env!("CARGO_BIN_EXE_mshogi-usi-fixture").into(),
        args: vec![mode.into()],
        options: vec![],
        model_path: None,
        model_identity: Some("verification fixture".into()),
        deterministic: true,
    }
}
fn config(mode: &str) -> shogi::Config {
    shogi::Config {
        schema_version: 1,
        profile: "mshogi.shogi.csa36.v1".into(),
        id: "actual-fixture".into(),
        time_control: "untimed".into(),
        process_policy: "fresh_per_turn".into(),
        engines: [engine(mode), engine(mode)],
        openings: vec![shogi::Opening {
            id: "initial".into(),
            sfen: None,
            moves: vec![],
        }],
        paired: true,
        work: [WorkLimit {
            depth: Some(2),
            nodes: Some(5000),
            movetime_ms: None,
        }; 2],
        max_actions: 4,
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
fn resignation_and_false_declaration_are_native_conclusions_not_transport_failures() {
    for (mode, reason) in [("resign", "Resignation"), ("win", "FalseDeclaration")] {
        let report = shogi::run(&config(mode)).unwrap();
        assert_eq!(report.games.len(), 2);
        for game in &report.games {
            assert_eq!(game.stop, Stop::NativeConclusion);
            assert_eq!(game.score, Some([0, 2]));
            assert!(format!("{:?}", game.conclusion).contains(reason));
            assert!(game.failure.is_none());
            assert_eq!(game.turns.len(), 1);
            assert!(game.traffic_complete);
            assert!(game.traffic.iter().any(|e| e.text == "gameover lose"));
        }
        let games = report.replay().unwrap();
        for game in games {
            assert_eq!(game.position().completed_actions(), 0);
            assert_eq!(game.events().len(), 1);
        }
        assert_eq!(
            shogi::Report::from_json(&report.to_json().unwrap()).unwrap(),
            report
        );
    }
}
#[test]
fn valid_entering_king_declaration_uses_accepted_28point_fixture() {
    let mut c = config("win");
    c.openings[0].sfen = Some("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1".into());
    let report = shogi::run(&c).unwrap();
    for game in &report.games {
        assert_eq!(game.stop, Stop::NativeConclusion);
        assert_eq!(game.score, Some([2, 0]));
        assert!(format!("{:?}", game.conclusion).contains("EnteringKing"));
    }
    let games = report.replay().unwrap();
    for game in games {
        assert_eq!(game.position().completed_actions(), 0);
        assert!(!game.root().has_complete_initial_history());
    }
}
#[test]
fn malformed_illegal_missing_crashed_and_resource_replies_stay_unscored() {
    for (mode, kind) in [
        ("malformed", FailureKind::MalformedReply),
        ("illegal", FailureKind::IllegalAction),
        ("missing", FailureKind::MissingReply),
        ("crash", FailureKind::UnexpectedExit),
        ("oversize", FailureKind::ResourceLimit),
        ("duplicate", FailureKind::LateReply),
    ] {
        let report = shogi::run(&config(mode)).unwrap();
        for game in &report.games {
            assert_eq!(game.stop, Stop::Failure, "{mode}");
            assert_eq!(game.failure.as_ref().unwrap().kind, kind, "{mode}");
            assert!(game.score.is_none() && game.conclusion.is_none() && game.turns.is_empty());
            assert!(game.processes.iter().all(|p| p.observation.exit.is_some()));
        }
        for game in report.replay().unwrap() {
            assert!(game.events().is_empty());
        }
    }
}
#[test]
fn watchdog_and_late_reply_retirement_preserve_native_game_and_reap_children() {
    for mode in ["timeout", "late"] {
        let mut c = config(mode);
        c.watchdogs.response_ms = 20;
        c.watchdogs.cleanup_ms = 300;
        let report = shogi::run(&c).unwrap();
        for game in &report.games {
            assert_eq!(game.stop, Stop::Failure);
            assert_eq!(game.failure.as_ref().unwrap().kind, FailureKind::Watchdog);
            assert!(game.turns.is_empty() && game.score.is_none());
            assert!(game.processes.iter().all(|p| p.observation.exit.is_some()));
            if mode == "late" {
                assert!(
                    game.traffic
                        .iter()
                        .any(|e| e.phase == "retired" && e.text.starts_with("bestmove"))
                );
            }
        }
        report.replay().unwrap();
    }
}
#[test]
fn all_reported_options_must_be_advertised_with_supported_values() {
    for (name, value) in [
        ("Unknown", "1"),
        ("Nodes", "20001"),
        ("RulesProfile", "wrong"),
    ] {
        let mut c = config("resign");
        for e in &mut c.engines {
            e.options.push(UciOption {
                name: name.into(),
                value: value.into(),
            });
        }
        let report = shogi::run(&c).unwrap();
        for game in &report.games {
            assert_eq!(
                game.failure.as_ref().unwrap().kind,
                FailureKind::Configuration
            );
            assert!(game.turns.is_empty());
        }
    }
}
#[test]
fn imported_counter_and_complete_prefix_survive_terminal_precheck_and_replay() {
    let mut c = config("resign");
    c.openings[0].sfen = Some("4k4/9/5R3/9/9/9/9/9/K8 b - 1".into());
    c.openings[0].moves = ["4c3c", "5a4a", "3c4c", "4a5a"]
        .repeat(3)
        .iter()
        .map(|s| s.to_string())
        .collect();
    let report = shogi::run(&c).unwrap();
    for game in &report.games {
        assert_eq!(game.score, Some([1, 1]));
        assert!(game.turns.is_empty() && game.processes.is_empty());
    }
    assert_eq!(report.replay().unwrap()[0].events().len(), 12);
    c.openings[0].moves.clear();
    c.openings[0].sfen = Some("4k4/9/5R3/9/9/9/9/9/K8 b - 321".into());
    let report = shogi::run(&c).unwrap();
    assert!(report.games[0].turns.is_empty());
    assert!(format!("{:?}", report.games[0].conclusion).contains("Length"));
}
#[test]
fn actual_move_drop_promotion_and_action_cutoff_are_not_invented_outcomes() {
    for (root, action) in [
        (None, "7g7f"),
        (Some("8k/9/9/9/9/9/9/9/K8 b P 17"), "P*5e"),
        (Some("8k/4P4/9/9/9/9/9/9/K8 b - 1"), "5b5a+"),
    ] {
        let mut c = config("move");
        c.openings[0].sfen = root.map(str::to_owned);
        c.max_actions = 1;
        for e in &mut c.engines {
            e.args.push(action.into());
        }
        let report = shogi::run(&c).unwrap();
        for game in &report.games {
            assert_eq!(game.stop, Stop::ActionLimit);
            assert!(game.score.is_none() && game.conclusion.is_none());
            assert_eq!(game.turns.len(), 1);
        }
        report.replay().unwrap();
    }
    let mut c = config("move");
    c.openings[0].sfen = Some("8k/9/9/9/9/9/9/9/K8 b P 1".into());
    for e in &mut c.engines {
        e.args.push("P*5a".into());
    }
    let report = shogi::run(&c).unwrap();
    assert_eq!(
        report.games[0].failure.as_ref().unwrap().kind,
        FailureKind::IllegalAction
    );
}
#[test]
fn strict_replay_rejects_context_limits_traffic_and_native_payload_tampering() {
    let report = shogi::run(&config("resign")).unwrap();
    let mut bad = report.clone();
    bad.games[0].turns[0].version += 1;
    assert!(bad.replay().is_err());
    let mut bad = report.clone();
    bad.games[0].turns[0].go = "go depth 1 nodes 1".into();
    assert!(bad.replay().is_err());
    let mut bad = report.clone();
    bad.games[0].turns[0].generation = 0;
    assert!(bad.replay().is_err());
    let mut bad = report.clone();
    bad.games[0].traffic.retain(|e| e.text != "bestmove resign");
    assert!(bad.replay().is_err());
    let mut bad = report.clone();
    bad.games[0].processes[0].generation += 1;
    assert!(bad.replay().is_err());
    let mut bad = report.clone();
    bad.games[0].game = bad.games[0].game.replace(
        "\"schema_version\":1",
        "\"schema_version\":1,\"schema_version\":1",
    );
    assert!(bad.replay().is_err());
    let encoded = report.to_json().unwrap();
    assert!(
        shogi::Report::from_json(&encoded.replacen(
            "\"schema_version\":1",
            "\"schema_version\":1,\"schema_version\":1",
            1
        ))
        .is_err()
    );
}
#[test]
fn clocks_and_unsupported_profiles_reject_before_spawning() {
    let c = config("resign");
    for mutation in 0..3 {
        let mut c = c.clone();
        match mutation {
            0 => c.time_control = "increment".into(),
            1 => c.profile = "other".into(),
            _ => c.work[0].movetime_ms = Some(1),
        }
        assert!(shogi::run(&c).is_err());
    }
    let mut c = config("timeout");
    c.max_game_ms = 50;
    c.watchdogs.response_ms = 1000;
    let report = shogi::run(&c).unwrap();
    for game in &report.games {
        assert_eq!(game.stop, Stop::TimeLimit);
        assert!(game.score.is_none() && game.turns.is_empty());
    }
    report.replay().unwrap();
}
#[test]
fn bounded_traffic_failure_is_explicit_and_keeps_native_prefix() {
    let mut c = config("flood");
    c.io.transcript_bytes = 2048;
    let report = shogi::run(&c).unwrap();
    for game in &report.games {
        assert_eq!(game.stop, Stop::Failure);
        assert_eq!(
            game.failure.as_ref().unwrap().kind,
            FailureKind::ResourceLimit
        );
        assert!(!game.traffic_complete);
        assert!(game.turns.is_empty());
    }
    report.replay().unwrap();
}

#[test]
fn post_reply_process_and_cleanup_failures_cannot_commit_candidates() {
    for mode in ["post_crash", "hungquit"] {
        let mut c = config(mode);
        c.watchdogs.cleanup_ms = 100;
        let report = shogi::run(&c).unwrap();
        for game in &report.games {
            assert_eq!(game.stop, Stop::Failure);
            assert_eq!(
                game.failure.as_ref().unwrap().kind,
                if mode == "hungquit" {
                    FailureKind::ResourceLimit
                } else {
                    FailureKind::UnexpectedExit
                }
            );
            assert!(game.turns.is_empty() && game.conclusion.is_none() && game.score.is_none());
        }
        report.replay().unwrap();
    }
}
#[test]
fn exact_27point_false_declaration_retains_admission_evidence() {
    let mut c = config("win");
    c.openings[0].sfen = Some("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B4P 1".into());
    let report = shogi::run(&c).unwrap();
    for game in report.replay().unwrap() {
        assert!(matches!(
            game.conclusion(),
            Some(mshogi_core::shogi::Conclusion::Win {
                reason: mshogi_core::shogi::WinReason::FalseDeclaration,
                ..
            })
        ));
        assert_eq!(game.events()[0].declaration.unwrap().points, 27);
        assert_eq!(game.position().completed_actions(), 0);
    }
}

#[test]
fn accepted_ascii_whitespace_reply_roundtrips_raw_and_escaped_traffic() {
    let mut c = config("move");
    c.max_actions = 1;
    for e in &mut c.engines {
        e.args.push("7g7f\t".into());
    }
    let report = shogi::run(&c).unwrap();
    assert_eq!(report.games[0].stop, Stop::ActionLimit);
    assert!(report.games[0].turns[0].reply.ends_with('\t'));
    assert!(
        report.games[0]
            .traffic
            .iter()
            .any(|e| e.text == "bestmove 7g7f\\t")
    );
    assert_eq!(
        shogi::Report::from_json(&report.to_json().unwrap()).unwrap(),
        report
    );
}

#[test]
fn missing_retirement_eof_cannot_hide_a_delayed_duplicate_bestmove() {
    let mut c = config("delayed_duplicate");
    c.watchdogs.cleanup_ms = 500;
    c.max_actions = 1;
    let report = shogi::run(&c).unwrap();
    for game in &report.games {
        assert_eq!(game.stop, Stop::Failure);
        assert_eq!(
            game.failure.as_ref().unwrap().kind,
            FailureKind::ResourceLimit
        );
        assert!(!game.traffic_complete);
        assert!(game.turns.is_empty() && game.conclusion.is_none() && game.score.is_none());
        assert!(game.processes.iter().all(|p| p.observation.exit.is_some()));
    }
    report.replay().unwrap();
}
