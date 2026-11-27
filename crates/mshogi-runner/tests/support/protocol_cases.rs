// The same fresh-process failure contract is exercised through both adapters.
macro_rules! protocol_failure_cases {
    ($game:ident, [$($mode:literal => $kind:ident),* $(,)?], $check:expr) => {
        #[test]
        fn protocol_failures_preserve_the_prefix_and_reap_children() {
            for (mode, kind) in [
                ("malformed", FailureKind::MalformedReply),
                ("illegal", FailureKind::IllegalAction),
                ("missing", FailureKind::MissingReply),
                ("crash", FailureKind::UnexpectedExit),
                ("oversize", FailureKind::ResourceLimit),
                ("duplicate", FailureKind::LateReply),
                $(($mode, FailureKind::$kind),)*
            ] {
                let report = $game::run(&config(mode)).unwrap();
                for game in &report.games {
                    assert_eq!(game.stop, Stop::Failure, "{mode}");
                    assert_eq!(game.failure.as_ref().unwrap().kind, kind, "{mode}: {:?}", game.failure);
                    assert!(game.score.is_none() && game.conclusion.is_none() && game.turns.is_empty());
                    assert!(game.processes.iter().all(|p| p.observation.exit.is_some()));
                }
                for game in report.replay().unwrap() {
                    ($check)(&game);
                }
            }
        }
    };
}
