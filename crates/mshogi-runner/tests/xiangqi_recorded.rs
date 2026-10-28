//! Actual retained native Xiangqi matches replay without opponent artifacts on every OS.
use mshogi_runner::xiangqi::{Report, Stop};
#[test]
fn actual_selfplay_independent_opponent_and_repeat_have_native_replay_evidence() {
    let records = [
        include_str!("../../../docs/m4/xiangqi/protocol/selfplay.report.json"),
        include_str!("../../../docs/m4/xiangqi/protocol/opponent.report.json"),
        include_str!("../../../docs/m4/xiangqi/protocol/selfplay-repeat.report.json"),
    ];
    let mut conclusions = 0;
    let mut accepted = 0;
    for text in records {
        let report = Report::from_json(text).unwrap();
        assert_eq!(report.games.len(), 4);
        for (entry, game) in report.games.iter().zip(report.replay().unwrap()) {
            assert!(entry.failure.is_none() && entry.traffic_complete);
            assert_eq!(entry.conclusion, game.conclusion());
            assert_eq!(entry.pending, game.pending_adjudication().cloned());
            accepted += entry.turns.len();
            if entry.stop == Stop::NativeConclusion {
                conclusions += 1;
                assert_eq!(entry.score, Some([2, 0]));
            } else {
                assert_eq!(entry.stop, Stop::ActionLimit);
                assert!(entry.score.is_none());
            }
            assert!(
                entry
                    .processes
                    .iter()
                    .all(|p| p.observation.exit.as_ref().unwrap().success
                        && p.observation.cleanup_failures.is_empty())
            );
        }
    }
    assert_eq!(accepted, 33);
    assert_eq!(conclusions, 5);
    let first = Report::from_json(records[0]).unwrap();
    let repeated = Report::from_json(records[2]).unwrap();
    for (a, b) in first.games.iter().zip(repeated.games) {
        assert_eq!(a.game, b.game);
        assert_eq!(a.conclusion, b.conclusion);
        assert_eq!(a.score, b.score);
        let context = |entry: &mshogi_runner::xiangqi::GameReport| {
            entry
                .turns
                .iter()
                .map(|t| {
                    (
                        t.actor,
                        t.reply.clone(),
                        t.context_sha256.clone(),
                        t.position.clone(),
                        t.go.clone(),
                    )
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(context(a), context(&b));
    }
}
#[test]
fn failed_incompatible_network_is_preserved_as_unscored_failure_not_admission() {
    let report = Report::from_json(include_str!(
        "../../../docs/m4/xiangqi/protocol/exploratory/incompatible-network.report.json"
    ))
    .unwrap();
    assert_eq!(
        report
            .games
            .iter()
            .filter(|g| g.stop == Stop::Failure)
            .count(),
        3
    );
    assert!(
        report
            .games
            .iter()
            .filter(|g| g.stop == Stop::Failure)
            .all(|g| g.score.is_none() && g.conclusion.is_none() && g.failure.is_some())
    );
    assert!(
        report
            .games
            .iter()
            .flat_map(|g| &g.traffic)
            .any(|e| e.text.contains("not loaded successfully"))
    );
    report.replay().unwrap();
    Report::from_json(include_str!(
        "../../../docs/m4/xiangqi/protocol/exploratory/pre-review-selfplay.report.json"
    ))
    .unwrap()
    .replay()
    .unwrap();
}

#[test]
fn actual_own_engine_zero_work_remains_incomplete_and_unscored() {
    let report = Report::from_json(include_str!(
        "../../../docs/m4/xiangqi/protocol/no-work.report.json"
    ))
    .unwrap();
    assert_eq!(report.games.len(), 2);
    for entry in &report.games {
        assert_eq!(entry.stop, Stop::Failure);
        assert_eq!(
            entry.failure.as_ref().unwrap().kind,
            mshogi_runner::xiangqi::FailureKind::IncompleteWork
        );
        assert!(entry.turns.is_empty() && entry.score.is_none() && entry.conclusion.is_none());
    }
    assert!(
        report
            .replay()
            .unwrap()
            .iter()
            .all(|game| game.events().is_empty())
    );
}
