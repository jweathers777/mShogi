//! Offline command evidence integrity; protocol fixtures here are never strength opponents.
use mshogi_runner::experiments::{Config, Report};
use std::{fs, process::Command};
#[test]
fn strict_cli_replay_export_and_prior_file_integrity() {
    let dir = std::env::temp_dir().join(format!(
        "msh48-cli-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&dir).unwrap();
    let config = dir.join("config.json");
    let report = dir.join("report.json");
    let data = dir.join("data.jsonl");
    let cli = env!("CARGO_BIN_EXE_mshogi-match");
    let run = |args: Vec<&std::ffi::OsStr>| Command::new(cli).args(args).output().unwrap();
    assert!(
        run(vec![
            "experiment-template".as_ref(),
            env!("CARGO_BIN_EXE_mshogi-runner-fixture").as_ref(),
            config.as_os_str()
        ])
        .status
        .success()
    );
    let mut c = Config::from_json(&fs::read_to_string(&config).unwrap()).unwrap();
    c.openings.truncate(1);
    c.players.truncate(2);
    c.max_steps = 0;
    fs::write(&config, serde_json::to_vec(&c).unwrap()).unwrap();
    assert!(
        run(vec![
            "experiment-run".as_ref(),
            config.as_os_str(),
            report.as_os_str()
        ])
        .status
        .success()
    );
    let prior = fs::read(&report).unwrap();
    let r = Report::from_json(std::str::from_utf8(&prior).unwrap()).unwrap();
    assert!(
        r.games
            .iter()
            .all(|g| g.stop == "step_limit" && g.score_first_twice.is_none())
    );
    assert!(
        !run(vec![
            "experiment-run".as_ref(),
            config.as_os_str(),
            report.as_os_str()
        ])
        .status
        .success()
    );
    assert_eq!(fs::read(&report).unwrap(), prior);
    assert!(
        run(vec!["experiment-replay".as_ref(), report.as_os_str()])
            .status
            .success()
    );
    assert!(
        run(vec![
            "experiment-data".as_ref(),
            report.as_os_str(),
            data.as_os_str()
        ])
        .status
        .success()
    );
    assert!(fs::read(&data).unwrap().is_empty());
    fs::write(&data, b"prior-dataset").unwrap();
    assert!(
        !run(vec![
            "experiment-data".as_ref(),
            report.as_os_str(),
            data.as_os_str()
        ])
        .status
        .success()
    );
    assert_eq!(fs::read(&data).unwrap(), b"prior-dataset");
    let mut bad = serde_json::to_value(&c).unwrap();
    bad["players"][0]["model"] = serde_json::json!("unsupported");
    fs::write(&config, bad.to_string()).unwrap();
    assert!(
        !run(vec![
            "experiment-run".as_ref(),
            config.as_os_str(),
            report.as_os_str()
        ])
        .status
        .success()
    );
    assert_eq!(fs::read(&report).unwrap(), prior);
    fs::remove_dir_all(dir).unwrap();
}
#[test]
fn actual_incompatible_child_failure_is_retained_unscored() {
    use mshogi_runner::experiments::*;
    let mut c = template(env!("CARGO_BIN_EXE_mshogi-runner-fixture")).unwrap();
    c.players.truncate(2);
    c.openings.truncate(1);
    c.max_steps = 1;
    c.watchdog_ms = 100;
    let r = run(&c).unwrap();
    assert!(r.games.iter().all(|g| g.stop.starts_with("failure:")
        && g.steps.len() == 1
        && !g.steps[0].applied
        && g.steps[0].observation.is_some()
        && g.score_first_twice.is_none()
        && g.native_outcome.is_none()
        && g.cleanup.is_empty()));
    Report::from_json(&r.to_json().unwrap()).unwrap();
    for row in r.data_jsonl().unwrap().lines() {
        let row: serde_json::Value = serde_json::from_str(row).unwrap();
        assert_eq!(row["terminal_value_available"], false);
        assert_eq!(row["usable_action_target"], false);
    }
}
