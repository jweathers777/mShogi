// Shared persistent-runner contracts. Each caller supplies its config,
// fixture, report and stop types; game notation and outcome tests stay local.
macro_rules! clocked_failure_cases {
    ($command:literal) => {
        #[test]
        fn report_file_survives_a_second_run() {
            use std::{fs, process::Command};
            let dir=std::env::temp_dir().join(format!("mshogi-{}-{}",$command,std::process::id()));
            fs::create_dir(&dir).unwrap();
            let input=dir.join("config.json");
            let output=dir.join("report.json");
            fs::write(&input, serde_json::to_vec(&config([fixture("resign"), fixture("resign")],0,1000)).unwrap()).unwrap();
            let run=|| Command::new(env!("CARGO_BIN_EXE_mshogi-match")).arg($command).arg(&input).arg(&output).output().unwrap();
            assert!(run().status.success());
            let original=fs::read(&output).unwrap();
            assert!(!run().status.success());
            assert_eq!(fs::read(&output).unwrap(),original);
            fs::remove_dir_all(dir).unwrap();
        }

        #[test]
        fn illegal_move_and_silence_are_unscored() {
            let illegal = run(&config([fixture("illegal"), fixture("resign")], 0, 1000)).unwrap();
            assert_eq!(illegal.stop, MatchStop::Failure);
            assert_eq!(illegal.failure.as_ref().unwrap().kind, FailureKind::IllegalAction);
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
        fn the_game_watchdog_is_a_time_limit_not_a_clock_exhaustion() {
            let mut c = config([fixture("timeout"), fixture("resign")], 0, 60_000);
            c.max_game_ms = 1_000;
            let report = run(&c).unwrap();
            assert_eq!(report.stop, MatchStop::TimeLimit);
            assert!(report.exhaustion.is_none() && report.engine_score.is_none());
            MatchReport::from_json(&report.to_json().unwrap()).unwrap();
        }
    };
}
