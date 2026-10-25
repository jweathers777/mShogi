//! Real cross-platform child processes; fixtures are explicitly not opponents.
use mshogi_core::chess::*;
use mshogi_runner::*;
use serde_json::{Value, json};
use std::time::{Duration, Instant};
const OPENING: &str = "7k/8/6K1/5Q2/8/8/8/8 b - - 0 1";
fn config(mode: &str) -> ExperimentConfig {
    let engine = EngineConfig {
        path: env!("CARGO_BIN_EXE_mshogi-runner-fixture").into(),
        args: vec![mode.into()],
        options: vec![],
        model_path: None,
        model_identity: Some("scripted test fixture, no strength identity".into()),
        deterministic: true,
    };
    ExperimentConfig {
        schema_version: 1,
        id: "native-process-test".into(),
        engines: [engine.clone(), engine],
        openings: vec![OpeningConfig {
            id: "forced-mate".into(),
            fen: Some(OPENING.into()),
            moves: vec![],
        }],
        paired: false,
        claim_policy: ClaimPolicy::AutoClaim,
        time_control: TimeControl::Untimed,
        work: WorkLimit {
            depth: Some(2),
            nodes: Some(1000),
            movetime_ms: None,
        },
        max_plies: 8,
        watchdogs: Watchdogs {
            handshake_ms: 2000,
            ready_ms: 2000,
            response_ms: 2000,
            write_ms: 100,
            cleanup_ms: 200,
        },
        io: IoLimits::default(),
    }
}
fn run(c: &ExperimentConfig) -> ExperimentReport {
    let report = run_experiment(c).unwrap();
    let decoded = ExperimentReport::from_json(&report.to_json()).unwrap();
    assert_eq!(decoded, report);
    assert_eq!(decoded.replay().unwrap().len(), report.games.len());
    report
}
#[test]
fn complete_scripted_pair_records_commands_identity_colors_results_and_replays() {
    let mut c = config("mate");
    c.paired = true;
    let r = run(&c);
    assert!(r.deterministic_rerun_eligible);
    assert_eq!(r.games.len(), 2);
    assert_eq!(r.games[0].engines_by_color, [0, 1]);
    assert_eq!(r.games[1].engines_by_color, [1, 0]);
    assert_eq!(r.summaries[0].wins, [1, 1]);
    assert_eq!(r.summaries[0].unscored, 0);
    for game in &r.games {
        assert_eq!(game.stop, GameStop::ChessConclusion);
        assert_eq!(
            game.conclusion,
            Some(MatchConclusion::Chess(ChessOutcome::Checkmate {
                winner: Color::White
            }))
        );
        let m = MatchRecord::from_json(&game.match_record_json)
            .unwrap()
            .replay()
            .unwrap();
        assert_eq!(
            m.game()
                .history()
                .actions()
                .map(|a| a.to_string())
                .collect::<Vec<_>>(),
            ["h8g8", "f5c8"]
        );
        assert_eq!(game.engines.len(), 2);
        assert!(
            game.engines.iter().all(
                |e| e.exit.as_ref().is_some_and(|s| s.success) && e.cleanup_failures.is_empty()
            )
        );
        for required in [
            "uci",
            "ucinewgame",
            "isready",
            "go depth 2 nodes 1000",
            "quit",
        ] {
            assert!(
                game.transcript
                    .entries
                    .iter()
                    .any(|e| e.text == required && e.direction == TransportDirection::Command),
                "{required}"
            );
        }
    }
    assert!(
        r.engine_identities.iter().all(|e| e
            .executable
            .sha256
            .as_ref()
            .is_some_and(|s| s.len() == 64))
    );
    // A new run under declared deterministic settings has the same actions,
    // although observation timestamps and OS process IDs are not identical.
    let again = run(&c);
    for (a, b) in r.replay().unwrap().iter().zip(again.replay().unwrap()) {
        assert_eq!(a.game().history(), b.game().history());
    }
}
#[test]
fn explicit_auto_claim_uses_known_history_before_any_go() {
    let mut c = config("mate");
    c.openings[0] = OpeningConfig {
        id: "known-history".into(),
        fen: None,
        moves: [
            "g1f3", "g8f6", "f3g1", "f6g8", "g1f3", "g8f6", "f3g1", "f6g8",
        ]
        .map(str::to_owned)
        .into(),
    };
    c.max_plies = 32;
    let r = run(&c);
    let g = &r.games[0];
    assert_eq!(
        g.conclusion,
        Some(MatchConclusion::Chess(ChessOutcome::Draw(
            DrawReason::Claimed(DrawClaim::Threefold)
        )))
    );
    assert!(
        !g.transcript
            .entries
            .iter()
            .any(|e| e.text.starts_with("go"))
    );
    assert_eq!(
        r.replay().unwrap()[0]
            .game()
            .history()
            .current_occurrences(),
        3
    );
}
#[test]
fn process_start_nonzero_exit_clean_missing_and_invalid_move_are_unscored() {
    for (mode, expected, faulty_engine) in [
        ("exit", FailureKind::UnexpectedExit, 0),
        ("clean_eof", FailureKind::MissingResponse, 0),
        ("search_exit", FailureKind::UnexpectedExit, 1),
        ("search_eof", FailureKind::MissingResponse, 1),
        ("invalid", FailureKind::InvalidMove, 1),
    ] {
        // Fault only the awaited engine. Two independently exiting children
        // race between required-response EOF and the other child's idle exit,
        // both of which are valid but deliberately different classifications.
        let mut c = config("mate");
        c.engines[faulty_engine].args = vec![mode.into()];
        let r = run(&c);
        let g = &r.games[0];
        assert_eq!(g.stop, GameStop::Failure, "{mode}");
        assert_eq!(g.conclusion, None);
        let f = g.failure.as_ref().unwrap();
        assert_eq!(f.kind, expected, "{mode}: {f:?}");
        assert_eq!(f.engine, faulty_engine);
        if mode.contains("exit") {
            assert!(
                f.exit
                    .as_ref()
                    .is_some_and(|s| !s.success && s.code.is_some())
            );
        }
        assert_eq!(r.summaries[0].unscored, 1);
        assert_eq!(r.summaries[0].failures, 1);
        assert_eq!(r.replay().unwrap()[0].game().history().ply_count(), 0);
    }
    // An already-ready engine exits cleanly while the controller awaits the
    // other engine's handshake. This is an idle exit, not its requested EOF.
    let mut idle = config("silent");
    idle.engines[0].args = vec!["idle_exit".into()];
    let r = run(&idle);
    let f = r.games[0].failure.as_ref().unwrap();
    assert_eq!(f.kind, FailureKind::UnexpectedExit);
    assert_eq!(f.engine, 0);
    assert_eq!(f.phase, "handshake");
    assert!(f.exit.as_ref().unwrap().success);
    assert_eq!(r.games[0].conclusion, None);
    assert_eq!(r.summaries[0].unscored, 1);
    let mut c = config("mate");
    c.engines[0].path.push_str("-does-not-exist");
    let r = run(&c);
    assert_eq!(
        r.games[0].failure.as_ref().unwrap().kind,
        FailureKind::ProcessStartFailure
    );
    assert!(r.engine_identities[0].executable.error.is_some());
    assert!(!r.deterministic_rerun_eligible);
}
#[test]
fn separate_watchdog_and_chess_clock_expiry_preserve_observations() {
    let mut c = config("search_silent");
    c.watchdogs.response_ms = 30;
    let r = run(&c);
    assert_eq!(
        r.games[0].failure.as_ref().unwrap().kind,
        FailureKind::ResponseTimeout
    );
    assert_eq!(r.games[0].conclusion, None);
    c.watchdogs.response_ms = 1000;
    c.time_control = TimeControl::Increment {
        initial_ns: [20_000_000, 20_000_000],
        increment_ns: 0,
    };
    let r = run(&c);
    let g = &r.games[0];
    assert_eq!(g.stop, GameStop::ChessConclusion);
    assert!(g.failure.is_none());
    assert_eq!(
        g.conclusion,
        Some(MatchConclusion::Unresolved {
            actor: Color::Black,
            trigger: TerminationTrigger::FlagFall
        })
    );
    assert_eq!(g.conclusion.unwrap().white_score_twice(), None);
    assert!(!r.deterministic_rerun_eligible);
    assert_eq!(r.replay().unwrap()[0].remaining_ns(), Some([20_000_000, 0]));
    c.watchdogs.response_ms = 10;
    c.time_control = TimeControl::Increment {
        initial_ns: [1_000_000_000, 1_000_000_000],
        increment_ns: 0,
    };
    let r = run(&c);
    assert_eq!(r.games[0].stop, GameStop::Failure);
    assert_eq!(
        r.games[0].failure.as_ref().unwrap().kind,
        FailureKind::ResponseTimeout
    );
}
#[test]
fn silent_and_blocked_stdin_children_cannot_block_cleanup() {
    let mut c = config("silent");
    c.watchdogs.handshake_ms = 30;
    c.watchdogs.cleanup_ms = 100;
    let start = Instant::now();
    let r = run(&c);
    assert!(start.elapsed() < Duration::from_secs(5));
    assert_eq!(
        r.games[0].failure.as_ref().unwrap().kind,
        FailureKind::ResponseTimeout
    );
    let mut c = config("blocked_writer");
    c.engines[0].options = (0..64)
        .map(|n| UciOption {
            name: format!("option{n}"),
            value: "v".repeat(2048),
        })
        .collect();
    c.watchdogs.write_ms = 30;
    c.watchdogs.cleanup_ms = 100;
    let start = Instant::now();
    let r = run(&c);
    assert!(start.elapsed() < Duration::from_secs(5));
    let f = r.games[0].failure.as_ref().unwrap();
    assert_eq!(f.kind, FailureKind::ResponseTimeout);
    assert_eq!(f.phase, "options");
}
#[test]
fn all_stream_and_metadata_buffers_are_bounded_with_truncation_markers() {
    let mut c = config("id_flood");
    c.io.transcript_bytes = 1024;
    let r = run(&c);
    let g = &r.games[0];
    assert_eq!(g.stop, GameStop::ChessConclusion);
    assert!(g.transcript.truncated);
    assert!(g.transcript.entries.len() < 10);
    assert!(g.transcript.observed_entries > 2000);
    for e in &g.engines {
        assert!(e.identity_truncated);
        assert!(e.uci_identity.len() <= 32);
        assert!(e.uci_identity.iter().map(String::len).sum::<usize>() <= 4096);
    }
    let mut c = config("stderr_flood");
    c.io.stderr_bytes = 64;
    let r = run(&c);
    assert!(r.games[0].engines.iter().all(|e| e.stderr.truncated
        && e.stderr.observed_bytes >= 1_048_576
        && e.stderr.text.len() <= 256));
    let mut c = config("oversized");
    c.io.line_bytes = 128;
    let r = run(&c);
    assert_eq!(
        r.games[0].failure.as_ref().unwrap().kind,
        FailureKind::InvalidResponse
    );
    let mut c = config("mate");
    c.io.transcript_bytes = 0;
    let r = run(&c);
    assert!(r.games[0].transcript.entries.is_empty());
    assert!(r.games[0].transcript.truncated);
    let mut c = config("mate");
    c.io.command_bytes = 16;
    let r = run(&c);
    assert_eq!(
        r.games[0].failure.as_ref().unwrap().kind,
        FailureKind::InvalidResponse
    );
}
#[test]
fn ply_limit_is_a_distinct_unscored_experiment_stop() {
    let mut c = config("mate");
    c.max_plies = 1;
    let r = run(&c);
    assert_eq!(r.games[0].stop, GameStop::PlyLimit);
    assert!(r.games[0].failure.is_none());
    assert!(r.games[0].conclusion.is_none());
    assert_eq!(r.summaries[0].unscored, 1);
}
#[test]
fn strict_configuration_and_replay_reject_inconsistent_or_oversized_evidence() {
    let c = config("mate");
    let base = serde_json::to_value(&c).unwrap();
    for (key, value) in [
        ("schema_version", json!(2)),
        ("claim_policy", json!("explicit_only")),
        ("max_plies", json!(513)),
        ("unknown", json!(true)),
    ] {
        let mut v = base.clone();
        v[key] = value;
        assert!(
            ExperimentConfig::from_json(&v.to_string()).is_err(),
            "{key}"
        );
    }
    let text = serde_json::to_string(&c).unwrap();
    assert!(
        ExperimentConfig::from_json(&text.replace(
            "\"schema_version\":1",
            "\"schema_version\":1,\"schema_version\":1"
        ))
        .is_err()
    );
    let mut oversized = c.clone();
    oversized.openings = (0..32)
        .map(|n| OpeningConfig {
            id: n.to_string(),
            fen: Some(OPENING.into()),
            moves: vec![],
        })
        .collect();
    oversized.paired = true;
    oversized.io.transcript_bytes = 4_194_304;
    assert!(oversized.validate().is_err());
    let r = run(&c);
    let base = serde_json::to_value(&r).unwrap();
    for path in [
        "summary",
        "conclusion",
        "color",
        "opening",
        "record",
        "transcript",
        "eligibility",
        "work",
        "path",
        "options",
        "model",
        "command",
    ] {
        let mut v = base.clone();
        match path {
            "summary" => v["summaries"][0]["wins"] = json!([99, 99]),
            "conclusion" => v["games"][0]["conclusion"] = Value::Null,
            "color" => v["games"][0]["engines_by_color"] = json!([1, 0]),
            "opening" => v["games"][0]["opening_id"] = json!("invented"),
            "record" => {
                let s = v["games"][0]["match_record_json"].as_str().unwrap();
                v["games"][0]["match_record_json"] = json!(s.replace("h8g8", "h8h7"));
            }
            "transcript" => v["games"][0]["transcript"]["entries"][1]["sequence"] = json!(0),
            "eligibility" => v["deterministic_rerun_eligible"] = json!(false),
            "work" => v["config"]["work"]["depth"] = json!(3),
            "path" => v["config"]["engines"][0]["path"] = json!("different-executable"),
            "options" => {
                v["config"]["engines"][0]["options"] = json!([{"name":"Threads","value":"2"}])
            }
            "model" => v["engine_identities"][0]["model_identity"] = json!("invented-model"),
            "command" => {
                let entries = v["games"][0]["transcript"]["entries"]
                    .as_array_mut()
                    .unwrap();
                let go = entries.iter_mut().find(|e| e["phase"] == "go").unwrap();
                go["text"] = json!("go depth 99");
            }
            _ => unreachable!(),
        }
        assert!(
            ExperimentReport::from_json(&v.to_string())
                .unwrap()
                .replay()
                .is_err(),
            "{path}"
        );
    }
}

#[cfg(unix)]
#[test]
fn nonregular_identity_paths_are_rejected_without_opening_a_fifo() {
    use std::process::{Command, Stdio};
    let dir = std::env::temp_dir().join(format!(
        "msh-runner-fifo-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&dir).unwrap();
    let fifo = dir.join("engine-fifo");
    assert!(
        Command::new("mkfifo")
            .arg(&fifo)
            .status()
            .unwrap()
            .success()
    );
    let mut c = config("mate");
    c.engines[0].path = fifo.to_string_lossy().into_owned();
    let config_path = dir.join("config.json");
    let output = dir.join("report.json");
    std::fs::write(&config_path, serde_json::to_string(&c).unwrap()).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_mshogi-match"))
        .arg("run")
        .arg(&config_path)
        .arg(&output)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            assert!(status.success());
            break;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("identity acquisition blocked on FIFO")
        }
        std::thread::sleep(Duration::from_millis(1));
    }
    let r = ExperimentReport::from_json(&std::fs::read_to_string(&output).unwrap()).unwrap();
    r.replay().unwrap();
    assert!(
        r.engine_identities[0]
            .executable
            .error
            .as_ref()
            .unwrap()
            .contains("not a regular file")
    );
    assert_eq!(
        r.games[0].failure.as_ref().unwrap().kind,
        FailureKind::ProcessStartFailure
    );
    std::fs::remove_dir_all(&dir).unwrap();
}

#[cfg(unix)]
#[test]
fn non_utf8_cli_arguments_fail_normally_without_panicking() {
    use std::os::unix::ffi::OsStringExt;
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_mshogi-match"))
        .arg("replay")
        .arg(std::ffi::OsString::from_vec(vec![0xff]))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let error = String::from_utf8(output.stderr).unwrap();
    assert!(error.contains("arguments must be valid UTF-8"));
    assert!(!error.contains("panicked"));
}
