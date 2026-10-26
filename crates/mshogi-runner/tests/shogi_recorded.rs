//! Retained native USI evidence replays without external executables on every OS.
use mshogi_runner::shogi::{Report, Stop};
#[test]
fn recorded_games_and_intentional_retirement_regression_have_explicit_verdicts() {
    let records = [
        include_str!("../../../docs/m4/usi/selfplay.report.json"),
        include_str!("../../../docs/m4/usi/opponent.report.json"),
        include_str!("../../../docs/m4/usi/selfplay-repeat.report.json"),
        include_str!("../../../docs/m4/usi/no-result.report.json"),
        include_str!("../../../docs/m4/usi/late-fixture.report.json"),
        include_str!("../../../docs/m4/usi/retirement-fixture.report.json"),
        include_str!("../../../docs/m4/usi/exploratory/selfplay.report.json"),
        include_str!("../../../docs/m4/usi/exploratory/opponent.report.json"),
    ];
    let mut total = 0;
    for text in records {
        let report = Report::from_json(text).unwrap();
        let games = report.replay().unwrap();
        total += games.len();
        for (entry, game) in report.games.iter().zip(games) {
            assert_eq!(entry.conclusion, game.conclusion());
            if entry.stop != Stop::NativeConclusion {
                assert!(entry.score.is_none());
            }
        }
    }
    assert_eq!(total, 36);
    let before = include_str!("../../../docs/m4/usi/review-findings/cleanup-before-fix.json");
    assert!(
        Report::from_json(before).is_err(),
        "an incomplete retirement capture must never establish accepted work"
    );
    let value: serde_json::Value = serde_json::from_str(before).unwrap();
    let canonical = mshogi_core::native::CanonicalRecord::from_json(
        value["games"][0]["game"].as_str().unwrap(),
    )
    .unwrap();
    let mshogi_core::native::CanonicalRecord::Shogi(record) = canonical else {
        panic!()
    };
    assert_eq!(
        record.to_game().unwrap().events().len(),
        1,
        "the retained pre-fix native history itself remains inspectable"
    );
}
