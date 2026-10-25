//! Replay actual macOS experiments on every native CI host without opponents.
use mshogi_core::chess::{ChessOutcome, DrawClaim, DrawReason, MatchConclusion};
use mshogi_runner::{ExperimentReport, FailureKind, GameStop, TransportDirection};

const SELF: &str = include_str!("../../../tests/fixtures/matches/selfplay.report.json");
const SELF_AGAIN: &str = include_str!("../../../tests/fixtures/matches/selfplay.rerun.report.json");
const STOCKFISH: &str = include_str!("../../../tests/fixtures/matches/stockfish19.report.json");
const STOCKFISH_AGAIN: &str =
    include_str!("../../../tests/fixtures/matches/stockfish19.rerun.report.json");
const TIMED: &str = include_str!("../../../tests/fixtures/matches/timed-selfplay.report.json");
const CLAIM: &str = include_str!("../../../tests/fixtures/matches/known-repetition.report.json");
const FAILURE: &str = include_str!("../../../tests/fixtures/matches/search-failure.report.json");
const CLOCK: &str = include_str!("../../../tests/fixtures/matches/clock-cancellation.report.json");

fn report(text: &str) -> ExperimentReport {
    let r = ExperimentReport::from_json(text).unwrap();
    r.replay().unwrap();
    r
}

#[test]
fn all_twenty_one_recorded_games_replay_with_complete_diagnostics() {
    let mut count = 0;
    for text in [
        SELF,
        SELF_AGAIN,
        STOCKFISH,
        STOCKFISH_AGAIN,
        TIMED,
        CLAIM,
        FAILURE,
        CLOCK,
    ] {
        let r = report(text);
        // These observations were captured on macOS; replay here is portable,
        // not a claim that Stockfish was executed on this test host.
        assert_eq!(r.os, "macos");
        assert_eq!(r.arch, "aarch64");
        count += r.games.len();
        for g in r.games {
            assert!(!g.transcript.truncated);
            assert!(g.engines.iter().all(|e| !e.identity_truncated
                && !e.stderr.truncated
                && e.cleanup_failures.is_empty()));
        }
    }
    assert_eq!(count, 21);
    let sf = report(STOCKFISH);
    assert_eq!(
        sf.engine_identities[1].executable.sha256.as_deref(),
        Some("8eed61129d1493c5d1f2fd9323f0c54c47ac49319911fbde18c6b9c87e8b13c5")
    );
    assert_eq!(
        sf.engine_identities[0].executable.sha256.as_deref(),
        Some("5bd9b9c1f71828cbedf2f2811e17e1533193524e02e5f3b8d7213d7856be85d6")
    );
    assert!(sf.games.iter().all(|g| g.stop == GameStop::ChessConclusion));
}

fn commands(r: &ExperimentReport) -> Vec<Vec<serde_json::Value>> {
    r.games
        .iter()
        .map(|g| {
            let record: serde_json::Value = serde_json::from_str(&g.match_record_json).unwrap();
            record["events"]
                .as_array()
                .unwrap()
                .iter()
                .map(|e| {
                    serde_json::json!([
                        e["request"]["actor"],
                        e["request"]["version"],
                        e["request"]["command"]
                    ])
                })
                .collect()
        })
        .collect()
}

#[test]
fn documented_work_limited_reruns_repeat_commands_and_conclusions() {
    for (first, second) in [(SELF, SELF_AGAIN), (STOCKFISH, STOCKFISH_AGAIN)] {
        let a = report(first);
        let b = report(second);
        assert!(a.deterministic_rerun_eligible && b.deterministic_rerun_eligible);
        assert_eq!(a.config, b.config);
        assert_eq!(a.engine_identities, b.engine_identities);
        assert_eq!(commands(&a), commands(&b));
        assert_eq!(a.summaries, b.summaries);
        for (a, b) in a.games.iter().zip(&b.games) {
            assert_eq!(a.conclusion, b.conclusion);
            assert_eq!(a.engines_by_color, b.engines_by_color);
        }
    }
    assert!(!report(TIMED).deterministic_rerun_eligible);
}

#[test]
fn actual_claim_and_engine_failure_have_distinct_meanings() {
    let claim = report(CLAIM);
    let games = claim.replay().unwrap();
    assert_eq!(games[0].game().history().ply_count(), 8);
    assert_eq!(
        claim.games[0].conclusion,
        Some(MatchConclusion::Chess(ChessOutcome::Draw(
            DrawReason::Claimed(DrawClaim::Threefold)
        )))
    );
    assert!(
        !claim.games[0]
            .transcript
            .entries
            .iter()
            .any(|e| e.direction == TransportDirection::Command && e.text.starts_with("go "))
    );
    let failed = report(FAILURE);
    assert_eq!(failed.games[0].stop, GameStop::Failure);
    assert_eq!(failed.games[0].conclusion, None);
    let failure = failed.games[0].failure.as_ref().unwrap();
    assert_eq!(failure.kind, FailureKind::UnexpectedExit);
    assert_eq!(failure.exit.as_ref().unwrap().code, Some(1));
    assert_eq!(failed.summaries[0].unscored, 1);
    assert_eq!(failed.summaries[0].draws, 0);
}

#[test]
fn runner_clock_expiry_shuts_down_outstanding_real_uci_search_unscored() {
    let r = report(CLOCK);
    let g = &r.games[0];
    assert_eq!(g.stop, GameStop::ChessConclusion);
    assert!(matches!(
        g.conclusion,
        Some(MatchConclusion::Unresolved { .. })
    ));
    assert_eq!(g.failure, None);
    assert_eq!(r.summaries[0].unscored, 1);
    assert_eq!(r.summaries[0].failures, 0);
    assert!(!r.deterministic_rerun_eligible);
    assert_eq!(r.replay().unwrap()[0].game().history().ply_count(), 0);
    let raw: serde_json::Value = serde_json::from_str(&g.match_record_json).unwrap();
    let events = raw["events"].as_array().unwrap();
    let fixed_deadline = events[0]["request"]["at_ns"].as_u64().unwrap() + 50_000_000;
    let detection = events.last().unwrap()["request"]["at_ns"].as_u64().unwrap();
    assert!(
        detection > fixed_deadline,
        "actual delayed detection was preserved"
    );
    assert!(
        g.transcript
            .entries
            .iter()
            .any(|e| e.direction == TransportDirection::Command
                && e.text.starts_with("go depth 128 movetime 10000 "))
    );
    assert!(!g.transcript.entries.iter().any(|e| e.direction == TransportDirection::Response
        && e.text.starts_with("bestmove ")));
    assert_eq!(
        g.transcript
            .entries
            .iter()
            .filter(|e| e.direction == TransportDirection::Command && e.text == "quit")
            .count(),
        2
    );
    assert!(g.engines.iter().all(|e| e.exit.as_ref().unwrap().success));
}
